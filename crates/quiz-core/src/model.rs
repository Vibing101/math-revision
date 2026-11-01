//! Question model, the public (answer-free) JSON shape and answer checking.

use crate::util::PowMap;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub enum Kind {
    /// Natural number (0, 1, 2, …).
    Num,
    /// Integer, may be negative.
    Int,
    /// Fraction (numerator / denominator); must be given in lowest terms.
    Frac,
    YesNo {
        yes: &'static str,
        no: &'static str,
    },
    /// Multi-select among the given options.
    Set {
        options: Vec<i64>,
        /// Display text per option (e.g. letters); empty means show the numbers.
        labels: Vec<String>,
    },
    Select {
        options: Vec<(i64, String)>,
    },
    /// Exponent boxes over the given primes.
    Exp {
        prefix: String,
        primes: Vec<u64>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    Num(i64),
    /// Lowest terms, positive denominator.
    Frac(i64, i64),
    Bool(bool),
    Set(Vec<i64>),
    Choice(i64),
    Pow(PowMap),
}

pub struct Part {
    pub label: String,
    pub kind: Kind,
    pub answer: Answer,
}

pub struct Group {
    pub label: String,
    pub parts: Vec<Part>,
}

/// A pupil's value for one part, after parsing.
#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Blank,
    Invalid,
    Num(i64),
    Frac(i64, i64),
    Bool(bool),
    Set(Vec<i64>),
    Choice(i64),
    Pow(PowMap),
}

impl Val {
    pub fn num(&self) -> Option<i64> {
        match self {
            Val::Num(n) | Val::Choice(n) => Some(*n),
            _ => None,
        }
    }
    pub fn set(&self) -> Option<&Vec<i64>> {
        match self {
            Val::Set(s) => Some(s),
            _ => None,
        }
    }
    pub fn pow(&self) -> Option<&PowMap> {
        match self {
            Val::Pow(m) => Some(m),
            _ => None,
        }
    }
}

pub struct Diag {
    pub tag: Option<&'static str>,
    pub msg: Option<String>,
}

pub type DiagFn = Box<dyn Fn(&[Val]) -> Vec<Diag>>;

pub struct Question {
    pub cat: &'static str,
    pub prompt: String,
    pub groups: Vec<Group>,
    pub hints: [String; 4],
    pub solution: String,
    pub diagnose: DiagFn,
}

impl Question {
    pub fn parts(&self) -> impl Iterator<Item = &Part> {
        self.groups.iter().flat_map(|g| g.parts.iter())
    }
}

// ---------- public JSON ----------

#[derive(Serialize)]
pub struct PublicQuestion {
    pub cat: &'static str,
    pub seed: u32,
    pub prompt: String,
    pub groups: Vec<PublicGroup>,
}

#[derive(Serialize)]
pub struct PublicGroup {
    pub label: String,
    pub parts: Vec<PublicPart>,
}

#[derive(Serialize)]
pub struct Opt {
    pub v: i64,
    pub l: String,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PublicPart {
    Num {
        label: String,
    },
    Int {
        label: String,
    },
    Frac {
        label: String,
    },
    Yesno {
        label: String,
        yes: &'static str,
        no: &'static str,
    },
    Set {
        label: String,
        options: Vec<i64>,
        labels: Vec<String>,
    },
    Select {
        label: String,
        options: Vec<Opt>,
    },
    Exp {
        label: String,
        prefix: String,
        primes: Vec<u64>,
    },
}

pub fn public(q: &Question, seed: u32) -> PublicQuestion {
    PublicQuestion {
        cat: q.cat,
        seed,
        prompt: q.prompt.clone(),
        groups: q
            .groups
            .iter()
            .map(|g| PublicGroup {
                label: g.label.clone(),
                parts: g
                    .parts
                    .iter()
                    .map(|p| {
                        let label = p.label.clone();
                        match &p.kind {
                            Kind::Num => PublicPart::Num { label },
                            Kind::Int => PublicPart::Int { label },
                            Kind::Frac => PublicPart::Frac { label },
                            Kind::YesNo { yes, no } => PublicPart::Yesno { label, yes, no },
                            Kind::Set { options, labels } => PublicPart::Set {
                                label,
                                options: options.clone(),
                                labels: labels.clone(),
                            },
                            Kind::Select { options } => PublicPart::Select {
                                label,
                                options: options
                                    .iter()
                                    .map(|(v, l)| Opt {
                                        v: *v,
                                        l: l.clone(),
                                    })
                                    .collect(),
                            },
                            Kind::Exp { prefix, primes } => PublicPart::Exp {
                                label,
                                prefix: prefix.clone(),
                                primes: primes.clone(),
                            },
                        }
                    })
                    .collect(),
            })
            .collect(),
    }
}

// ---------- checking ----------

fn parse_int(v: &Value) -> Val {
    match v {
        Value::Null => Val::Blank,
        Value::Number(n) => n
            .as_i64()
            .filter(|x| *x >= 0)
            .map(Val::Num)
            .unwrap_or(Val::Invalid),
        Value::String(s) => {
            let t: String = s
                .chars()
                .filter(|c| !c.is_whitespace() && *c != '.')
                .collect();
            if t.is_empty() {
                Val::Blank
            } else {
                t.parse::<i64>()
                    .ok()
                    .filter(|x| *x >= 0)
                    .map(Val::Num)
                    .unwrap_or(Val::Invalid)
            }
        }
        _ => Val::Invalid,
    }
}

/// Signed integer; accepts "-", the typographic minus "−" and a leading "+".
fn parse_signed(v: &Value) -> Val {
    match v {
        Value::Null => Val::Blank,
        Value::Number(n) => n.as_i64().map(Val::Num).unwrap_or(Val::Invalid),
        Value::String(s) => {
            let t: String = s
                .chars()
                .filter(|c| !c.is_whitespace() && *c != '.')
                .map(|c| if c == '−' || c == '–' { '-' } else { c })
                .collect();
            let t = t.strip_prefix('+').unwrap_or(&t).to_string();
            if t.is_empty() {
                Val::Blank
            } else {
                t.parse::<i64>()
                    .ok()
                    .filter(|x| x.abs() < 1_000_000_000_000)
                    .map(Val::Num)
                    .unwrap_or(Val::Invalid)
            }
        }
        _ => Val::Invalid,
    }
}

pub fn parse_val(kind: &Kind, v: &Value) -> Val {
    match kind {
        Kind::Num => parse_int(v),
        Kind::Int => parse_signed(v),
        Kind::Frac => match v {
            Value::Object(o) => {
                let n = parse_signed(o.get("n").unwrap_or(&Value::Null));
                let d = parse_signed(o.get("d").unwrap_or(&Value::Null));
                match (n, d) {
                    (Val::Num(n), Val::Num(d)) if d != 0 => Val::Frac(n, d),
                    (Val::Num(n), Val::Blank) => Val::Frac(n, 1),
                    (Val::Blank, Val::Blank) => Val::Blank,
                    _ => Val::Invalid,
                }
            }
            Value::Null => Val::Blank,
            other => match parse_signed(other) {
                Val::Num(n) => Val::Frac(n, 1),
                x => x,
            },
        },
        Kind::YesNo { .. } => match v {
            Value::Bool(b) => Val::Bool(*b),
            Value::Null => Val::Blank,
            _ => Val::Invalid,
        },
        Kind::Set { .. } => match v {
            Value::Array(a) if a.is_empty() => Val::Blank,
            Value::Array(a) => {
                let mut s: Vec<i64> = a.iter().filter_map(|x| x.as_i64()).collect();
                s.sort_unstable();
                s.dedup();
                Val::Set(s)
            }
            Value::Null => Val::Blank,
            _ => Val::Invalid,
        },
        Kind::Select { .. } => match parse_int(v) {
            Val::Num(n) => Val::Choice(n),
            other => other,
        },
        Kind::Exp { primes, .. } => match v {
            Value::Object(o) => {
                let mut any = false;
                let mut m = PowMap::new();
                for p in primes {
                    match parse_int(o.get(&p.to_string()).unwrap_or(&Value::Null)) {
                        Val::Blank => {}
                        Val::Num(e) => {
                            any = true;
                            if e > 0 {
                                m.insert(*p, e.min(40) as u32);
                            }
                        }
                        _ => return Val::Invalid,
                    }
                }
                if any {
                    Val::Pow(m)
                } else {
                    Val::Blank
                }
            }
            Value::Null => Val::Blank,
            _ => Val::Invalid,
        },
    }
}

pub fn is_right(a: &Answer, v: &Val) -> bool {
    match (a, v) {
        (Answer::Num(x), Val::Num(y)) => x == y,
        (Answer::Frac(an, ad), Val::Frac(n, d)) => {
            let (n, d) = if *d < 0 { (-n, -d) } else { (*n, *d) };
            n == *an && d == *ad
        }
        (Answer::Bool(x), Val::Bool(y)) => x == y,
        (Answer::Choice(x), Val::Choice(y)) => x == y,
        (Answer::Set(x), Val::Set(y)) => {
            let mut x = x.clone();
            x.sort_unstable();
            &x == y
        }
        (Answer::Pow(x), Val::Pow(y)) => {
            let clean = |m: &PowMap| {
                m.iter()
                    .filter(|(_, e)| **e > 0)
                    .map(|(p, e)| (*p, *e))
                    .collect::<Vec<_>>()
            };
            clean(x) == clean(y)
        }
        _ => false,
    }
}

#[derive(Serialize)]
pub struct PubDiag {
    pub tag: Option<&'static str>,
    pub msg: Option<String>,
}

#[derive(Serialize)]
pub struct CheckResult {
    pub blank: Vec<bool>,
    pub ok: Vec<bool>,
    pub correct: bool,
    pub diag: Vec<PubDiag>,
    pub solution: Option<String>,
}

/// Check the pupil's values. `reveal` returns the worked solution even when wrong.
pub fn check(q: &Question, values: &[Value], reveal: bool) -> CheckResult {
    let parts: Vec<&Part> = q.parts().collect();
    let vals: Vec<Val> = parts
        .iter()
        .enumerate()
        .map(|(i, p)| parse_val(&p.kind, values.get(i).unwrap_or(&Value::Null)))
        .collect();
    let blank: Vec<bool> = vals
        .iter()
        .map(|v| matches!(v, Val::Blank | Val::Invalid))
        .collect();
    if blank.iter().any(|b| *b) {
        return CheckResult {
            ok: vec![false; parts.len()],
            blank,
            correct: false,
            diag: vec![PubDiag {
                tag: Some("blank"),
                msg: None,
            }],
            solution: None,
        };
    }
    let ok: Vec<bool> = parts
        .iter()
        .zip(&vals)
        .map(|(p, v)| is_right(&p.answer, v))
        .collect();
    let correct = ok.iter().all(|b| *b);
    let diag = if correct {
        vec![]
    } else {
        let mut seen = std::collections::HashSet::new();
        // Equal value but not in lowest terms: say so for every fraction part.
        let unreduced = parts.iter().zip(&vals).any(|(p, v)| match (&p.answer, v) {
            (Answer::Frac(an, ad), Val::Frac(n, d)) => {
                (*n as i128) * (*ad as i128) == (*an as i128) * (*d as i128) && (n, d) != (an, ad)
            }
            _ => false,
        });
        let extra = unreduced.then(|| Diag {
            tag: Some("frac-reduce"),
            msg: Some("Η τιμή είναι σωστή, αλλά γράψε το κλάσμα <b>ανάγωγο</b> (απλοποίησε όσο γίνεται, με θετικό παρονομαστή).".into()),
        });
        extra
            .into_iter()
            .chain((q.diagnose)(&vals))
            .filter(|d| d.tag.is_none_or(|t| seen.insert(t)))
            .map(|d| PubDiag {
                tag: d.tag,
                msg: d.msg,
            })
            .collect()
    };
    CheckResult {
        blank,
        ok,
        correct,
        diag,
        solution: if correct || reveal {
            Some(q.solution.clone())
        } else {
            None
        },
    }
}

/// Correct answers in the JSON shape the client sends (used by tests and the worksheet).
pub fn answer_json(a: &Answer) -> Value {
    match a {
        Answer::Num(n) | Answer::Choice(n) => Value::from(*n),
        Answer::Frac(n, d) => serde_json::json!({ "n": n, "d": d }),
        Answer::Bool(b) => Value::from(*b),
        Answer::Set(s) => Value::from(s.clone()),
        Answer::Pow(m) => Value::Object(
            m.iter()
                .map(|(p, e)| (p.to_string(), Value::from(*e)))
                .collect(),
        ),
    }
}

/// Human-readable answer for printed answer keys.
pub fn answer_text(p: &Part) -> String {
    match (&p.kind, &p.answer) {
        (Kind::YesNo { yes, no }, Answer::Bool(b)) => (if *b { *yes } else { *no }).to_string(),
        (Kind::Select { options }, Answer::Choice(c)) => options
            .iter()
            .find(|(v, _)| v == c)
            .map(|(_, l)| l.clone())
            .unwrap_or_default(),
        (_, Answer::Num(n)) => crate::util::signed(*n),
        (_, Answer::Frac(n, d)) => crate::util::frac_text(*n, *d),
        (Kind::Set { labels, options }, Answer::Set(s)) if !labels.is_empty() => s
            .iter()
            .filter_map(|x| {
                options
                    .iter()
                    .position(|o| o == x)
                    .map(|i| labels[i].clone())
            })
            .collect::<Vec<_>>()
            .join(", "),
        (_, Answer::Set(s)) => s
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        (_, Answer::Pow(m)) => crate::util::fmt_pow(m),
        _ => String::new(),
    }
}
