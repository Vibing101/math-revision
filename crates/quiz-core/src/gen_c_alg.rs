//! Γ΄ Γυμνασίου, Κεφάλαια 1–2: Αξιοσημείωτες ταυτότητες, Παραγοντοποίηση,
//! εξισώσεις 2ου και ανώτερου βαθμού, ρητές αλγεβρικές παραστάσεις.

use crate::mk::*;
use crate::model::*;
use crate::util::*;

pub(crate) fn nz(r: &mut Rng, lo: i64, hi: i64) -> i64 {
    loop {
        let x = lo + r.r(0, (hi - lo) as u64) as i64;
        if x != 0 {
            return x;
        }
    }
}

/// Polynomial from (coefficient, power) terms, highest power first: "2x² − 3x + 1".
pub(crate) fn poly(terms: &[(i64, u32)], v: &str) -> String {
    let mut out = String::new();
    for (c, p) in terms.iter().copied().filter(|(c, _)| *c != 0) {
        let sign = if c < 0 { "−" } else { "+" };
        let a = c.abs();
        let var = match p {
            0 => String::new(),
            1 => format!("<i>{v}</i>"),
            p => format!("<i>{v}</i>{}", sup(p)),
        };
        let body = if p > 0 && a == 1 {
            var
        } else {
            format!("{a}{var}")
        };
        if out.is_empty() {
            out = if c < 0 { format!("−{body}") } else { body };
        } else {
            out += &format!(" {sign} {body}");
        }
    }
    if out.is_empty() {
        "0".into()
    } else {
        out
    }
}
fn same_bag(u: &[i64], v: &[i64]) -> bool {
    let (mut a, mut b) = (u.to_vec(), v.to_vec());
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

/// Plain-text version for <select> options.
pub(crate) fn ptext(s: &str) -> String {
    strip_tags(s)
}
/// "(x + 3)", "(x − 3)", "(2x + 1)"
fn lin(a: i64, b: i64, v: &str) -> String {
    format!("({})", poly(&[(a, 1), (b, 0)], v))
}
fn quad_pieces(v: &str) -> Vec<String> {
    vec![
        String::new(),
        format!("<i>{v}</i>² + "),
        format!("<i>{v}</i> + "),
        String::new(),
    ]
}

const VARS: [&str; 4] = ["x", "y", "α", "β"];

pub fn c_ident(r: &mut Rng) -> Question {
    let v = r.pick(&VARS);
    let vv = format!("<i>{v}</i>");
    match r.r(0, 3) {
        0 | 1 => {
            let a = if r.chance(0.6) { 1 } else { r.r(2, 3) as i64 };
            let b = nz(r, -9, 9);
            let kind = r.r(0, 2); // 0: (a v + b)², 1: (a v + b)(a v − b), 2: (v + b)³
            let (expr, res, steps, wrong): (String, Vec<i64>, String, Option<Vec<i64>>) = match kind
            {
                0 => (
                    format!("{}²", lin(a, b, v)),
                    vec![a * a, 2 * a * b, b * b],
                    format!(
                        "(α {} β)² = α² {} 2αβ + β², με α = {}, β = {}",
                        if b < 0 { "−" } else { "+" },
                        if b < 0 { "−" } else { "+" },
                        poly(&[(a, 1)], v),
                        b.abs()
                    ),
                    Some(vec![a * a, 0, b * b]),
                ),
                1 => (
                    format!("{}{}", lin(a, b, v), lin(a, -b, v)),
                    vec![a * a, 0, -b * b],
                    format!(
                        "(α + β)(α − β) = α² − β², με α = {}, β = {}",
                        poly(&[(a, 1)], v),
                        b.abs()
                    ),
                    Some(vec![a * a, 0, b * b]),
                ),
                _ => {
                    let b = b.signum() * b.abs().min(5);
                    (
                        format!("{}³", lin(1, b, v)),
                        vec![1, 3 * b, 3 * b * b, b * b * b],
                        format!(
                            "(α {} β)³ = α³ {} 3α²β + 3αβ² {} β³, με α = {vv}, β = {}",
                            if b < 0 { "−" } else { "+" },
                            if b < 0 { "−" } else { "+" },
                            if b < 0 { "−" } else { "+" },
                            b.abs()
                        ),
                        Some(vec![1, 0, 0, b * b * b]),
                    )
                }
            };
            let pieces: Vec<String> = if kind == 2 {
                vec![
                    String::new(),
                    format!("{vv}³ + "),
                    format!("{vv}² + "),
                    format!("{vv} + "),
                    String::new(),
                ]
            } else {
                quad_pieces(v)
            };
            let terms: Vec<(i64, u32)> = res
                .iter()
                .enumerate()
                .map(|(i, c)| (*c, (res.len() - 1 - i) as u32))
                .collect();
            let result = poly(&terms, v);
            let p = Part {
                label: String::new(),
                kind: Kind::Slots {
                    pieces,
                    unordered: false,
                },
                answer: Answer::Ints(res.clone()),
            };
            Question {
                cat: "c-ident",
                prompt: format!("Να βρεις το ανάπτυγμα: <span class=\"big\">{expr}</span><br><span class=\"muted\">Γράψε τους συντελεστές (μπορεί να είναι αρνητικοί ή 0).</span>"),
                groups: one(vec![p]),
                hints: hints(
                    "Ταυτότητες: (α ± β)² = α² ± 2αβ + β², (α + β)(α − β) = α² − β², (α ± β)³ = α³ ± 3α²β + 3αβ² ± β³.",
                    steps.clone(),
                    "Μην ξεχάσεις το «διπλάσιο γινόμενο» 2αβ στο τετράγωνο αθροίσματος.",
                    format!("{expr} = {result}"),
                ),
                solution: format!("{steps}.<br><b>{expr} = {result}</b>"),
                diagnose: Box::new(move |vals| match (&wrong, &vals[0]) {
                    (Some(w), Val::Ints(u)) if u == w && *w != res => vec![dm("ident-middle", "Ξέχασες τους «ενδιάμεσους» όρους. Το (α + β)² <b>δεν</b> είναι α² + β²: λείπει το 2αβ.")],
                    _ => vec![],
                }),
            }
        }
        2 => {
            // fill the missing term: (v + □)² = v² + 2b v + b²
            let b = r.r(2, 12) as i64;
            let minus = r.chance(0.4);
            let sb = if minus { -b } else { b };
            Question {
                cat: "c-ident",
                prompt: format!(
                    "Συμπλήρωσε την ταυτότητα: <span class=\"big\">({vv} {} □)² = {}</span>",
                    if minus { "−" } else { "+" },
                    poly(&[(1, 2), (2 * sb, 1), (b * b, 0)], v)
                ),
                groups: one(vec![num("□ =", b as u64)]),
                hints: hints(
                    "(α ± β)² = α² ± 2αβ + β².",
                    format!("Το β² είναι {}.", b * b),
                    format!(
                        "Ποιος θετικός αριθμός στο τετράγωνο δίνει {}; Έλεγξε και το 2αβ = {}.",
                        b * b,
                        2 * b
                    ),
                    b.to_string(),
                ),
                solution: format!(
                    "β² = {} ⇒ β = {b}, και 2·{vv}·{b} = {}{vv} ✓. Άρα <b>□ = {b}</b>.",
                    b * b,
                    2 * b
                ),
                diagnose: no_diag(),
            }
        }
        _ => {
            // mental arithmetic with identities
            let base = r.pick(&[10i64, 20, 30, 40, 50, 100]);
            let d = r.r(1, 3) as i64;
            let k = r.r(0, 2);
            let (expr, val, how) = match k {
                0 => (
                    format!("{}²", base + d),
                    (base + d).pow(2),
                    format!(
                        "({base} + {d})² = {} + {} + {} = {}",
                        base * base,
                        2 * base * d,
                        d * d,
                        (base + d).pow(2)
                    ),
                ),
                1 => (
                    format!("{}²", base - d),
                    (base - d).pow(2),
                    format!(
                        "({base} − {d})² = {} − {} + {} = {}",
                        base * base,
                        2 * base * d,
                        d * d,
                        (base - d).pow(2)
                    ),
                ),
                _ => (
                    format!("{}·{}", base + d, base - d),
                    base * base - d * d,
                    format!(
                        "({base} + {d})({base} − {d}) = {} − {} = {}",
                        base * base,
                        d * d,
                        base * base - d * d
                    ),
                ),
            };
            Question {
                cat: "c-ident",
                prompt: format!("Υπολόγισε <b>με τη βοήθεια ταυτότητας</b> (χωρίς υπολογιστική μηχανή): <span class=\"big\">{expr}</span>"),
                groups: one(vec![num("Αποτέλεσμα:", val as u64)]),
                hints: hints(format!("Γράψε τους αριθμούς ως {base} ± {d}."), if k == 2 { "(α + β)(α − β) = α² − β²" } else { "(α ± β)² = α² ± 2αβ + β²" }, how.split(" = ").nth(1).unwrap_or("").to_string(), val.to_string()),
                solution: how.to_string(),
                diagnose: no_diag(),
            }
        }
    }
}

pub fn c_factor(r: &mut Rng) -> Question {
    let v = r.pick(&["x", "y", "α"]);
    let vv = format!("<i>{v}</i>");
    match r.r(0, 5) {
        0 => {
            // trinomial with integer roots
            let (p, q) = (nz(r, -9, 9), nz(r, -9, 9));
            let s = p + q;
            let pr = p * q;
            let expr = poly(&[(1, 2), (s, 1), (pr, 0)], v);
            Question {
                cat: "c-factor",
                prompt: format!("Να παραγοντοποιήσεις το τριώνυμο <span class=\"big\">{expr}</span>."),
                groups: one(vec![slots(&[&format!("({vv} + "), &format!(")({vv} + "), ")"], vec![p, q], true)]),
                hints: hints(
                    "Ψάχνουμε δύο αριθμούς με γινόμενο τον σταθερό όρο και άθροισμα τον συντελεστή του x.",
                    format!("Γινόμενο {}, άθροισμα {}.", signed(pr), signed(s)),
                    "Δοκίμασε ζευγάρια διαιρετών του σταθερού όρου, με τα σωστά πρόσημα.",
                    format!("{expr} = {}{}", lin(1, p, v), lin(1, q, v)),
                ),
                solution: format!("{} · {} = {} και {} + {} = {}, άρα <b>{expr} = {}{}</b>.", signed(p), signed(q), signed(pr), signed(p), signed(q), signed(s), lin(1, p, v), lin(1, q, v)),
                diagnose: Box::new(move |vals| match &vals[0] {
                    Val::Ints(u) if same_bag(u, &[-p, -q]) && p + q != 0 => vec![dm("root-sign", "Τα πρόσημα είναι ανάποδα. Έλεγξε: οι αριθμοί πρέπει να έχουν άθροισμα ίσο με τον συντελεστή του x.")],
                    _ => vec![],
                }),
            }
        }
        1 => {
            let b = r.r(2, 12) as i64;
            let expr = poly(&[(1, 2), (-b * b, 0)], v);
            Question {
                cat: "c-factor",
                prompt: format!("Να παραγοντοποιήσεις: <span class=\"big\">{expr}</span>"),
                groups: one(vec![slots(
                    &[&format!("({vv} + "), &format!(")({vv} + "), ")"],
                    vec![b, -b],
                    true,
                )]),
                hints: hints(
                    "Διαφορά τετραγώνων: α² − β² = (α + β)(α − β).",
                    format!("{} = {b}²", b * b),
                    format!("α = {vv}, β = {b}"),
                    format!("({vv} + {b})({vv} − {b})"),
                ),
                solution: format!("{expr} = {vv}² − {b}² = <b>({vv} + {b})({vv} − {b})</b>"),
                diagnose: no_diag(),
            }
        }
        2 => {
            let b = nz(r, -9, 9);
            let expr = poly(&[(1, 2), (2 * b, 1), (b * b, 0)], v);
            Question {
                cat: "c-factor",
                prompt: format!(
                    "Να γράψεις ως τέλειο τετράγωνο: <span class=\"big\">{expr}</span>"
                ),
                groups: one(vec![slots(&[&format!("({vv} + "), ")²"], vec![b], false)]),
                hints: hints(
                    "α² ± 2αβ + β² = (α ± β)².",
                    format!("{} = {}²", b * b, b.abs()),
                    format!(
                        "Το πρόσημο του μεσαίου όρου είναι «{}».",
                        if b < 0 { "−" } else { "+" }
                    ),
                    format!("{}²", lin(1, b, v)),
                ),
                solution: format!("{expr} = <b>{}²</b>", lin(1, b, v)),
                diagnose: no_diag(),
            }
        }
        _ => {
            // multiple choice: common factor, grouping, two squares, sum/difference of cubes
            let (expr, right, wrongs, how): (String, String, Vec<String>, &str) = match r.r(0, 3) {
                0 => {
                    let (k, b) = (r.r(2, 9) as i64, r.r(1, 9) as i64);
                    let e = poly(&[(1, 3), (k, 2)], v);
                    (
                        e,
                        format!("{v}²({v} + {k})"),
                        vec![
                            format!("{v}({v}² + {k})"),
                            format!("{v}³({k} + 1)"),
                            format!("{k}{v}²({v} + 1)"),
                        ],
                        {
                            let _ = b;
                            "Κοινός παράγοντας: η μικρότερη δύναμη της μεταβλητής."
                        },
                    )
                }
                1 => {
                    let (a, b) = (r.r(2, 5) as i64, r.r(2, 9) as i64);
                    let e = poly(&[(a * a, 0), (-(b * b), 2)], v);
                    (
                        e,
                        format!("({a} − {b}{v})({a} + {b}{v})"),
                        vec![
                            format!("({a} − {b}{v})²"),
                            format!("({b}{v} − {a})({b}{v} + {a})"),
                            format!("({a} − {b}{v})({a} − {b}{v})"),
                        ],
                        "Διαφορά τετραγώνων: α² − β² = (α − β)(α + β).",
                    )
                }
                2 => {
                    let b = r.pick(&[1i64, 2, 3, 4, 5]);
                    let plus = r.chance(0.5);
                    let e = poly(&[(1, 3), (if plus { b.pow(3) } else { -b.pow(3) }, 0)], v);
                    let (s1, s2) = if plus { ("+", "−") } else { ("−", "+") };
                    (
                        e,
                        format!(
                            "({v} {s1} {b})({v}² {s2} {}{v} + {})",
                            if b == 1 { String::new() } else { b.to_string() },
                            b * b
                        ),
                        vec![
                            format!("({v} {s1} {b})³"),
                            format!(
                                "({v} {s1} {b})({v}² {s1} {}{v} + {})",
                                if b == 1 { String::new() } else { b.to_string() },
                                b * b
                            ),
                            format!(
                                "({v} {s2} {b})({v}² {s2} {}{v} + {})",
                                if b == 1 { String::new() } else { b.to_string() },
                                b * b
                            ),
                        ],
                        if plus {
                            "Άθροισμα κύβων: α³ + β³ = (α + β)(α² − αβ + β²)."
                        } else {
                            "Διαφορά κύβων: α³ − β³ = (α − β)(α² + αβ + β²)."
                        },
                    )
                }
                _ => {
                    let (a, b) = (r.r(2, 7) as i64, r.r(2, 7) as i64);
                    // v² + a v + b v + a b = (v + a)(v + b) written as v(v + a) + b(v + a)
                    let e = format!("{v}² + {a}{v} + {b}{v} + {}", a * b);
                    (
                        e,
                        format!("({v} + {a})({v} + {b})"),
                        vec![
                            format!("({v} + {a}{b})({v} + 1)"),
                            format!("{v}({v} + {a}) + {b}"),
                            format!("({v} + {})²", a + b),
                        ],
                        "Ομαδοποίηση: x² + αx + βx + αβ = x(x + α) + β(x + α) = (x + α)(x + β).",
                    )
                }
            };
            let mut opts = vec![right.clone()];
            for w in wrongs {
                if !opts.contains(&w) {
                    opts.push(w);
                }
            }
            Question {
                cat: "c-factor",
                prompt: format!("Ποια είναι η πλήρης παραγοντοποίηση του <span class=\"big\">{}</span>;", expr),
                groups: one(vec![choice(r, "Παραγοντοποίηση:", &opts)]),
                hints: hints("Ψάξε πρώτα για κοινό παράγοντα.", "Μετά δες αν είναι διαφορά τετραγώνων, άθροισμα/διαφορά κύβων, τριώνυμο ή αν χρειάζεται ομαδοποίηση.", how, right.clone()),
                solution: format!("{how}<br><b>{} = {right}</b>", ptext(&expr)),
                diagnose: Box::new(|_| vec![tag("factor")]),
            }
        }
    }
}

pub fn c_eq2(r: &mut Rng) -> Question {
    let v = "x";
    let vv = "<i>x</i>";
    match r.r(0, 3) {
        0 | 1 => {
            let (p, q) = loop {
                let (p, q) = (r.r(0, 18) as i64 - 9, r.r(0, 18) as i64 - 9);
                if p != q {
                    break (p, q);
                }
            };
            let a = if r.chance(0.3) { r.r(2, 3) as i64 } else { 1 };
            let (b, c) = (-a * (p + q), a * p * q);
            let eq = if r.chance(0.3) && c != 0 {
                // move the constant to the right
                format!("{} = {}", poly(&[(a, 2), (b, 1)], v), signed(-c))
            } else {
                format!("{} = 0", poly(&[(a, 2), (b, 1), (c, 0)], v))
            };
            let fact = if c == 0 {
                format!("{vv}{}", lin(a, b, v))
            } else {
                format!(
                    "{}{}{}",
                    if a == 1 { String::new() } else { a.to_string() },
                    lin(1, -p, v),
                    lin(1, -q, v)
                )
            };
            Question {
                cat: "c-eq2",
                prompt: format!("Λύσε την εξίσωση <span class=\"big\">{eq}</span>."),
                groups: one(vec![slots(
                    &[&format!("{vv} = "), &format!(" ή {vv} = "), ""],
                    vec![p, q],
                    true,
                )]),
                hints: hints(
                    "Φέρε όλους τους όρους στο πρώτο μέλος και παραγοντοποίησε.",
                    format!("Παραγοντοποίηση: {fact} = 0"),
                    "Ένα γινόμενο είναι 0 όταν ένας τουλάχιστον παράγοντας είναι 0.",
                    format!("{vv} = {} ή {vv} = {}", signed(p), signed(q)),
                ),
                solution: format!(
                    "{eq} ⇔ {fact} = 0 ⇔ <b>{vv} = {} ή {vv} = {}</b>",
                    signed(p),
                    signed(q)
                ),
                diagnose: Box::new(move |vals| {
                    match &vals[0] {
                    Val::Ints(u) if same_bag(u, &[-p, -q]) && p + q != 0 => vec![dm("root-sign", format!("Από το ({vv} − {}) = 0 προκύπτει {vv} = {}, όχι {}. Πρόσεξε τα πρόσημα.", p.abs(), signed(p), signed(-p)))],
                    Val::Ints(u) if c == 0 && !u.contains(&0) => vec![dm("lost-root", "Μην διαιρείς με το x: χάνεις τη λύση x = 0. Βγάλε κοινό παράγοντα.")],
                    _ => vec![],
                }
                }),
            }
        }
        2 => {
            let b = r.r(1, 6) as i64;
            let eq = format!("{} = 0", poly(&[(1, 3), (-b * b, 1)], v));
            Question {
                cat: "c-eq2",
                prompt: format!("Λύσε την εξίσωση <span class=\"big\">{eq}</span> (έχει τρεις λύσεις)."),
                groups: one(vec![slots(&[&format!("{vv} = "), &format!(", {vv} = "), &format!(", {vv} = "), ""], vec![0, b, -b], true)]),
                hints: hints("Βγάλε κοινό παράγοντα το x.", format!("{vv}({vv}² − {}) = 0", b * b), format!("{vv}({vv} − {b})({vv} + {b}) = 0"), format!("{vv} = 0, {b}, −{b}")),
                solution: format!("{eq} ⇔ {vv}({vv}² − {}) = 0 ⇔ {vv}({vv} − {b})({vv} + {b}) = 0 ⇔ <b>{vv} = 0 ή {vv} = {b} ή {vv} = −{b}</b>", b * b),
                diagnose: Box::new(|vals| match &vals[0] {
                    Val::Ints(u) if !u.contains(&0) => vec![dm("lost-root", "Μην διαιρείς με το x: χάνεις τη λύση x = 0.")],
                    _ => vec![],
                }),
            }
        }
        _ => {
            let (p, q) = (nz(r, -9, 9), nz(r, -9, 9));
            let (a, b) = (r.r(1, 4) as i64, r.r(1, 4) as i64);
            // (a x − a p)(b x − b q) = 0 shown in factored form
            Question {
                cat: "c-eq2",
                prompt: format!(
                    "Λύσε την εξίσωση <span class=\"big\">{}{} = 0</span>.",
                    lin(a, -a * p, v),
                    lin(b, -b * q, v)
                ),
                groups: one(vec![slots(
                    &[&format!("{vv} = "), &format!(" ή {vv} = "), ""],
                    vec![p, q],
                    true,
                )]),
                hints: hints(
                    "Ένα γινόμενο είναι 0 όταν ένας τουλάχιστον παράγοντας είναι 0.",
                    format!(
                        "{} = 0 ή {} = 0",
                        poly(&[(a, 1), (-a * p, 0)], v),
                        poly(&[(b, 1), (-b * q, 0)], v)
                    ),
                    "Λύσε κάθε πρωτοβάθμια εξίσωση χωριστά.",
                    format!("{vv} = {} ή {vv} = {}", signed(p), signed(q)),
                ),
                solution: format!(
                    "{} = 0 ⇔ {vv} = {} και {} = 0 ⇔ {vv} = {}. <b>{vv} = {} ή {vv} = {}</b>",
                    poly(&[(a, 1), (-a * p, 0)], v),
                    signed(p),
                    poly(&[(b, 1), (-b * q, 0)], v),
                    signed(q),
                    signed(p),
                    signed(q)
                ),
                diagnose: no_diag(),
            }
        }
    }
}

pub fn c_rational(r: &mut Rng) -> Question {
    let v = "x";
    let vv = "<i>x</i>";
    match r.r(0, 2) {
        0 => {
            let (p, q) = loop {
                let (p, q) = (nz(r, -8, 8), nz(r, -8, 8));
                if p != q {
                    break (p, q);
                }
            };
            let num_ = poly(&[(r.r(1, 5) as i64, 1), (nz(r, -9, 9), 0)], v);
            let den = poly(&[(1, 2), (-(p + q), 1), (p * q, 0)], v);
            Question {
                cat: "c-rational",
                prompt: format!("Για ποιες τιμές του {vv} <b>δεν ορίζεται</b> η ρητή παράσταση <span class=\"big\"><span class=\"fr\"><span>{num_}</span><span>{den}</span></span></span>;"),
                groups: one(vec![slots(&[&format!("{vv} ≠ "), &format!(" και {vv} ≠ "), ""], vec![p, q], true)]),
                hints: hints("Μια ρητή παράσταση δεν ορίζεται όταν ο παρονομαστής είναι 0.", format!("Λύσε {den} = 0."), format!("Παραγοντοποίηση: {}{}", lin(1, -p, v), lin(1, -q, v)), format!("{vv} ≠ {} και {vv} ≠ {}", signed(p), signed(q))),
                solution: format!("{den} = {}{} = 0 ⇔ {vv} = {} ή {vv} = {}. Άρα πρέπει <b>{vv} ≠ {} και {vv} ≠ {}</b>.", lin(1, -p, v), lin(1, -q, v), signed(p), signed(q), signed(p), signed(q)),
                diagnose: Box::new(|_| vec![tag("rational-domain")]),
            }
        }
        1 => {
            // simplify (x² − b²)/(x² + b x) = (x − b)/x
            let b = r.r(1, 9) as i64;
            let (n_, d_, right, w1, w2, w3) = match r.r(0, 2) {
                0 => (
                    poly(&[(1, 2), (-b * b, 0)], v),
                    poly(&[(1, 2), (b, 1)], v),
                    format!("(x − {b})/x"),
                    format!("(x − {b})/(x + {b})"),
                    format!("−{b}/x"),
                    format!("(x + {b})/x"),
                ),
                1 => (
                    poly(&[(1, 2), (2 * b, 1), (b * b, 0)], v),
                    poly(&[(1, 2), (-b * b, 0)], v),
                    format!("(x + {b})/(x − {b})"),
                    format!("(x − {b})/(x + {b})"),
                    format!("(x + {b})/x"),
                    "1".into(),
                ),
                _ => {
                    let k = r.r(2, 6) as i64;
                    (
                        poly(&[(k, 2), (k * b, 1)], v),
                        poly(&[(k, 1)], v),
                        format!("x + {b}"),
                        format!("x + {}", k * b),
                        format!("{k}x + {b}"),
                        format!("x² + {b}"),
                    )
                }
            };
            let opts = vec![right.clone(), w1, w2, w3];
            let mut uniq: Vec<String> = vec![];
            for o in opts {
                if !uniq.contains(&o) {
                    uniq.push(o);
                }
            }
            Question {
                cat: "c-rational",
                prompt: format!("Απλοποίησε τη ρητή παράσταση <span class=\"big\"><span class=\"fr\"><span>{n_}</span><span>{d_}</span></span></span>."),
                groups: one(vec![choice(r, "Αποτέλεσμα:", &uniq)]),
                hints: hints("Παραγοντοποίησε αριθμητή και παρονομαστή.", "Διαγράφουμε μόνο κοινούς <b>παράγοντες</b>, ποτέ όρους αθροίσματος.", "Χρησιμοποίησε κοινό παράγοντα, διαφορά τετραγώνων ή τέλειο τετράγωνο.", right.clone()),
                solution: format!("Παραγοντοποιούμε και διαγράφουμε τον κοινό παράγοντα: <b>{right}</b>."),
                diagnose: Box::new(|_| vec![tag("rational-simplify")]),
            }
        }
        _ => {
            // a/(x − p) = b/(x − q) with an integer solution
            let (sol, a, b) = loop {
                let (p, q) = (nz(r, -6, 6), nz(r, -6, 6));
                let (a, b) = (r.r(1, 6) as i64, r.r(1, 6) as i64);
                if a == b || p == q {
                    continue;
                }
                // a(x − q) = b(x − p) ⇒ (a − b)x = a q − b p
                let num_ = a * q - b * p;
                if num_ % (a - b) != 0 {
                    continue;
                }
                let x = num_ / (a - b);
                if x == p || x == q {
                    continue;
                }
                break (x, (a, p), (b, q));
            };
            let ((a, p), (b, q)) = (a, b);
            let fr1 = format!(
                "<span class=\"fr\"><span>{a}</span><span>{}</span></span>",
                poly(&[(1, 1), (-p, 0)], v)
            );
            let fr2 = format!(
                "<span class=\"fr\"><span>{b}</span><span>{}</span></span>",
                poly(&[(1, 1), (-q, 0)], v)
            );
            Question {
                cat: "c-rational",
                prompt: format!("Λύσε την εξίσωση <span class=\"big\">{fr1} = {fr2}</span>."),
                groups: one(vec![int("x =", sol)]),
                hints: hints(format!("Περιορισμοί: {vv} ≠ {} και {vv} ≠ {}.", signed(p), signed(q)), "Πολλαπλασίασε χιαστί (ή με το ΕΚΠ των παρονομαστών).", format!("{a}({}) = {b}({})", poly(&[(1, 1), (-q, 0)], v), poly(&[(1, 1), (-p, 0)], v)), format!("{vv} = {}, που είναι δεκτή.", signed(sol))),
                solution: format!("Περιορισμοί: {vv} ≠ {}, {}. {a}({}) = {b}({}) ⇔ {}{vv} = {} ⇔ <b>{vv} = {}</b> (δεκτή).", signed(p), signed(q), poly(&[(1, 1), (-q, 0)], v), poly(&[(1, 1), (-p, 0)], v), a - b, a * q - b * p, signed(sol)),
                diagnose: no_diag(),
            }
        }
    }
}
