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
    match r.r(0, 9) {
        4 => eq_product_const(r),
        5 => eq_equal_squares(r),
        6 => eq_kind(r),
        7 => eq_split(r),
        8 => eq_word(r),
        9 => eq_param(r),
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

// ---------- exam-style equations (from the school tests) ----------

fn ordered_roots(a: (i64, i64), b: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    if a.0 * b.1 <= b.0 * a.1 {
        (a, b)
    } else {
        (b, a)
    }
}

/// (x − p)(x − q) = c, which becomes a trinomial with integer roots.
fn eq_product_const(r: &mut Rng) -> Question {
    let vv = "<i>x</i>";
    let (r1, r2) = loop {
        let (a, b) = (r.r(0, 18) as i64 - 9, r.r(0, 18) as i64 - 9);
        if a != b {
            break (a, b);
        }
    };
    let p = nz(r, -9, 9);
    let q = r1 + r2 - p;
    let c = p * q - r1 * r2;
    if c == 0 {
        return eq_product_const(r);
    }
    let eq = format!("{}{} = {}", lin(1, -p, "x"), lin(1, -q, "x"), signed(c));
    let tri = poly(&[(1, 2), (-(r1 + r2), 1), (r1 * r2, 0)], "x");
    Question {
        cat: "c-eq2",
        prompt: format!("Λύσε την εξίσωση <span class=\"big\">{eq}</span>."),
        groups: one(vec![slots(&[&format!("{vv} = "), &format!(" ή {vv} = "), ""], vec![r1, r2], true)]),
        hints: hints(
            "Δεν μπορείς να πεις «ένας παράγοντας ίσος με τον αριθμό»: αυτό ισχύει μόνο όταν το δεύτερο μέλος είναι 0.",
            "Κάνε τις πράξεις στο πρώτο μέλος και φέρε όλους τους όρους στο ένα μέλος.",
            format!("{tri} = 0"),
            format!("{vv} = {} ή {vv} = {}", signed(r1), signed(r2)),
        ),
        solution: format!(
            "{} = {} ⇔ {tri} = 0 ⇔ {}{} = 0 ⇔ <b>{vv} = {} ή {vv} = {}</b>",
            poly(&[(1, 2), (-(p + q), 1), (p * q, 0)], "x"),
            signed(c),
            lin(1, -r1, "x"),
            lin(1, -r2, "x"),
            signed(r1),
            signed(r2)
        ),
        diagnose: Box::new(move |v| match &v[0] {
            Val::Ints(u) if same_bag(u, &[p, q]) => vec![dm("eq-product-const", format!("Τα {} και {} μηδενίζουν τους παράγοντες, αλλά εδώ το δεύτερο μέλος είναι {}, όχι 0. Κάνε πρώτα τις πράξεις.", signed(p), signed(q), signed(c)))],
            _ => vec![],
        }),
    }
}

/// (x − a)² = (b x + c)²  ⇒  difference of squares.
fn eq_equal_squares(r: &mut Rng) -> Question {
    let vv = "<i>x</i>";
    let (a, b, c) = loop {
        let (a, b, c) = (nz(r, -6, 6), r.r(2, 5) as i64, nz(r, -6, 6));
        if a + c != 0 && a != c {
            break (a, b, c);
        }
    };
    // x − a = b x + c  ⇒ x = (a + c)/(1 − b);  x − a = −(b x + c) ⇒ x = (a − c)/(1 + b)
    let x1 = reduce(a + c, 1 - b);
    let x2 = reduce(a - c, 1 + b);
    let (lo, hi) = ordered_roots(x1, x2);
    let eq = format!("{}² = {}²", lin(1, -a, "x"), lin(b, c, "x"));
    Question {
        cat: "c-eq2",
        prompt: format!("Λύσε την εξίσωση <span class=\"big\">{eq}</span>.<br><span class=\"muted\">Γράψε τις ρίζες ως ανάγωγα κλάσματα (ή ακέραιους).</span>"),
        groups: one(vec![frac("Μικρότερη ρίζα:", lo.0, lo.1), frac("Μεγαλύτερη ρίζα:", hi.0, hi.1)]),
        hints: hints(
            "Φέρε τα πάντα στο πρώτο μέλος: Α² − Β² = 0.",
            "Διαφορά τετραγώνων: (Α − Β)(Α + Β) = 0.",
            format!("({vv} − {a} − ({}))·({vv} − {a} + {}) = 0", poly(&[(b, 1), (c, 0)], "x"), poly(&[(b, 1), (c, 0)], "x")).replace("− -", "+ ").replace("− −", "+ "),
            format!("{vv} = {} ή {vv} = {}", fr(x1.0, x1.1), fr(x2.0, x2.1)),
        ),
        solution: format!(
            "{eq} ⇔ {}² − {}² = 0 ⇔ [{} − {}][{} + {}] = 0 ⇔ ({})·({}) = 0 ⇔ <b>{vv} = {} ή {vv} = {}</b>",
            lin(1, -a, "x"),
            lin(b, c, "x"),
            lin(1, -a, "x"),
            lin(b, c, "x"),
            lin(1, -a, "x"),
            lin(b, c, "x"),
            poly(&[(1 - b, 1), (-a - c, 0)], "x"),
            poly(&[(1 + b, 1), (c - a, 0)], "x"),
            fr(x1.0, x1.1),
            fr(x2.0, x2.1)
        ),
        diagnose: Box::new(|_| vec![tag("eq-squares")]),
    }
}

/// How many roots: two, one double, or none.
fn eq_kind(r: &mut Rng) -> Question {
    let vv = "<i>x</i>";
    let kind = r.r(0, 2);
    let (bco, cco, how) = match kind {
        0 => {
            let (p, q) = loop {
                let (p, q) = (nz(r, -8, 8), nz(r, -8, 8));
                if p != q {
                    break (p, q);
                }
            };
            (
                -(p + q),
                p * q,
                format!(
                    "{}{} = 0 ⇔ {vv} = {} ή {vv} = {}: δύο άνισες ρίζες.",
                    lin(1, -p, "x"),
                    lin(1, -q, "x"),
                    signed(p),
                    signed(q)
                ),
            )
        }
        1 => {
            let h = nz(r, -8, 8);
            (
                -2 * h,
                h * h,
                format!(
                    "{}² = 0 ⇔ {vv} = {}: μία διπλή ρίζα.",
                    lin(1, -h, "x"),
                    signed(h)
                ),
            )
        }
        _ => {
            let h = nz(r, -6, 6);
            let k = r.r(1, 9) as i64;
            (-2 * h, h * h + k, format!("{}² + {k} &gt; 0 για κάθε {vv}, άρα η εξίσωση είναι αδύνατη: δεν έχει πραγματικές ρίζες.", lin(1, -h, "x")))
        }
    };
    let eq = format!("{} = 0", poly(&[(1, 2), (bco, 1), (cco, 0)], "x"));
    let opts = [
        "Δύο άνισες πραγματικές ρίζες",
        "Μία διπλή ρίζα",
        "Καμία πραγματική ρίζα (αδύνατη)",
    ];
    Question {
        cat: "c-eq2",
        prompt: format!("Τι ρίζες έχει η εξίσωση <span class=\"big\">{eq}</span>;"),
        groups: one(vec![select(
            "Είδος ριζών:",
            opts.iter()
                .enumerate()
                .map(|(i, o)| (i as i64, o.to_string()))
                .collect(),
            kind as i64,
        )]),
        hints: hints(
            "Προσπάθησε να την παραγοντοποιήσεις (τριώνυμο ή τέλειο τετράγωνο).",
            format!(
                "Συμπλήρωσε τετράγωνο: {vv}² {} {}{vv} = ({vv} {} {})² − {}.",
                if bco < 0 { "−" } else { "+" },
                bco.abs(),
                if bco < 0 { "−" } else { "+" },
                fr(bco.abs(), 2),
                fr(bco * bco, 4)
            ),
            "Αν καταλήξεις σε «τετράγωνο + θετικός αριθμός = 0», η εξίσωση είναι αδύνατη.",
            opts[kind as usize].to_string(),
        ),
        solution: format!("{eq}: {how}"),
        diagnose: Box::new(|_| vec![tag("eq-kind")]),
    }
}

/// p x² + b x + c with one fractional root: split the middle term.
fn eq_split(r: &mut Rng) -> Question {
    let vv = "<i>x</i>";
    let (pp, qq, s) = loop {
        let (pp, qq, s) = (r.r(2, 3) as i64, nz(r, -5, 5), nz(r, -6, 6));
        if qq % pp != 0 {
            break (pp, qq, s);
        }
    };
    // (pp x + qq)(x + s) = pp x² + (pp s + qq) x + qq s
    let eq = format!(
        "{} = 0",
        poly(&[(pp, 2), (pp * s + qq, 1), (qq * s, 0)], "x")
    );
    let (lo, hi) = ordered_roots(reduce(-qq, pp), (-s, 1));
    Question {
        cat: "c-eq2",
        prompt: format!("Λύσε την εξίσωση <span class=\"big\">{eq}</span>.<br><span class=\"muted\">Γράψε τις ρίζες ως ανάγωγα κλάσματα (ή ακέραιους).</span>"),
        groups: one(vec![frac("Μικρότερη ρίζα:", lo.0, lo.1), frac("Μεγαλύτερη ρίζα:", hi.0, hi.1)]),
        hints: hints(
            format!("Σπάσε τον μεσαίο όρο σε δύο όρους με γινόμενο {}·{} = {}.", pp, signed(qq * s), signed(pp * qq * s)),
            format!("{} = {}{vv}² {} {}{vv} {} {}{vv} {} {}", poly(&[(pp, 2), (pp * s + qq, 1), (qq * s, 0)], "x"), pp, if pp * s < 0 { "−" } else { "+" }, (pp * s).abs(), if qq < 0 { "−" } else { "+" }, qq.abs(), if qq * s < 0 { "−" } else { "+" }, (qq * s).abs()),
            format!("Ομαδοποίηση: {}{}", lin(pp, qq, "x"), lin(1, s, "x")),
            format!("{vv} = {} ή {vv} = {}", fr(-qq, pp), signed(-s)),
        ),
        solution: format!("{eq} ⇔ {}{} = 0 ⇔ <b>{vv} = {} ή {vv} = {}</b>", lin(pp, qq, "x"), lin(1, s, "x"), fr(-qq, pp), signed(-s)),
        diagnose: Box::new(|_| vec![tag("eq-split")]),
    }
}

/// Rectangle area problems that lead to a quadratic equation.
fn eq_word(r: &mut Rng) -> Question {
    let vv = "<i>x</i>";
    let x0 = r.r(1, 9) as i64;
    let (a, b) = (r.r(1, 9) as i64, r.r(1, 9) as i64);
    let area = (x0 + a) * (x0 + b);
    // a smaller rectangle whose area is exactly half
    let small = (2..=area / 2)
        .find(|w| (area / 2) % w == 0 && area % 2 == 0 && *w > 1 && area / 2 / w > 1)
        .filter(|_| area % 2 == 0);
    let (ctx, sol_t) = match small {
        Some(w) if r.chance(0.6) => (
            format!("Το εμβαδόν ορθογωνίου με διαστάσεις ({vv} + {a}) cm και ({vv} + {b}) cm είναι <b>διπλάσιο</b> από το εμβαδόν ορθογωνίου με διαστάσεις {w} cm και {} cm. Να βρεις το {vv}.", area / 2 / w),
            format!("({vv} + {a})({vv} + {b}) = 2·{w}·{} = {area}", area / 2 / w),
        ),
        _ => (
            format!("Ένα ορθογώνιο έχει διαστάσεις ({vv} + {a}) cm και ({vv} + {b}) cm και εμβαδόν {area} cm². Να βρεις το {vv}."),
            format!("({vv} + {a})({vv} + {b}) = {area}"),
        ),
    };
    let other = -(a + b) - x0;
    Question {
        cat: "c-eq2",
        prompt: ctx,
        groups: one(vec![num(format!("{vv} ="), x0 as u64)]),
        hints: hints(
            "Γράψε την εξίσωση του εμβαδού: μήκος · πλάτος = εμβαδόν.",
            sol_t.clone(),
            format!("{} = 0", poly(&[(1, 2), (a + b, 1), (a * b - area, 0)], "x")),
            format!("Ρίζες {} και {}. Οι διαστάσεις πρέπει να είναι θετικές, άρα {vv} = {x0}.", signed(x0), signed(other)),
        ),
        solution: format!(
            "{sol_t} ⇔ {} = 0 ⇔ {}{} = 0 ⇔ {vv} = {x0} ή {vv} = {}. Η αρνητική λύση απορρίπτεται (μήκος θετικό). <b>{vv} = {x0}</b>, διαστάσεις {} cm και {} cm.",
            poly(&[(1, 2), (a + b, 1), (a * b - area, 0)], "x"),
            lin(1, -x0, "x"),
            lin(1, -other, "x"),
            signed(other),
            x0 + a,
            x0 + b
        ),
        diagnose: no_diag(),
    }
}

/// x = κ + t is a root of a trinomial: find κ.
fn eq_param(r: &mut Rng) -> Question {
    let vv = "<i>x</i>";
    let (r1, r2) = loop {
        let (a, b) = (r.r(1, 9) as i64, r.r(1, 9) as i64);
        if a != b {
            break (a, b);
        }
    };
    let t = nz(r, -4, 5);
    let eq = format!(
        "{} = 0",
        poly(&[(1, 2), (-(r1 + r2), 1), (r1 * r2, 0)], "x")
    );
    Question {
        cat: "c-eq2",
        prompt: format!("Να βρεις τις τιμές του κ, αν {vv} = κ {} {} είναι λύση της εξίσωσης <span class=\"big\">{eq}</span>.", if t < 0 { "−" } else { "+" }, t.abs()),
        groups: one(vec![slots(&["κ = ", " ή κ = ", ""], vec![r1 - t, r2 - t], true)]),
        hints: hints("Λύσε πρώτα την εξίσωση.", format!("{}{} = 0", lin(1, -r1, "x"), lin(1, -r2, "x")), format!("Οι λύσεις είναι {r1} και {r2}. Βάλε κ {} {} ίσο με καθεμία.", if t < 0 { "−" } else { "+" }, t.abs()), format!("κ = {} ή κ = {}", signed(r1 - t), signed(r2 - t))),
        solution: format!("{eq} ⇔ {vv} = {r1} ή {vv} = {r2}. Άρα κ {} {} = {r1} ή κ {} {} = {r2}, δηλαδή <b>κ = {} ή κ = {}</b>.", if t < 0 { "−" } else { "+" }, t.abs(), if t < 0 { "−" } else { "+" }, t.abs(), signed(r1 - t), signed(r2 - t)),
        diagnose: Box::new(move |v| match &v[0] {
            Val::Ints(u) if same_bag(u, &[r1, r2]) => vec![msg("Αυτές είναι οι λύσεις για το x. Ζητείται το κ: λύσε κ + t = λύση.")],
            _ => vec![],
        }),
    }
}

// ---------- composite factorizations and method names (from the school tests) ----------

pub fn c_factor2(r: &mut Rng) -> Question {
    let methods = [
        "Κοινός παράγοντας",
        "Ομαδοποίηση",
        "Διαφορά τετραγώνων",
        "Άθροισμα κύβων",
        "Διαφορά κύβων",
        "Τριώνυμο",
        "Ανάπτυγμα τετραγώνου (τέλειο τετράγωνο)",
    ];
    if r.chance(0.35) {
        let (k, m) = (r.r(2, 9) as i64, r.r(2, 6) as i64);
        let (txt, idx): (String, usize) = match r.r(0, 6) {
            0 => (format!("{k}αx + {k}αy = {k}α(x + y)"), 0),
            1 => (format!("αx + αy + {k}x + {k}y = (x + y)(α + {k})"), 1),
            2 => (
                format!("{}x² − {} = ({m}x − {k})({m}x + {k})", m * m, k * k),
                2,
            ),
            3 => (
                format!("y³ + {} = (y + {m})(y² − {m}y + {})", m.pow(3), m * m),
                3,
            ),
            4 => (
                format!("α³ − {} = (α − {m})(α² + {m}α + {})", m.pow(3), m * m),
                4,
            ),
            5 => (
                format!("x² + {}x + {} = (x + {k})(x + {m})", k + m, k * m),
                5,
            ),
            _ => (format!("{}x² − {}xy + y² = ({m}x − y)²", m * m, 2 * m), 6),
        };
        return Question {
            cat: "c-factor2",
            prompt: format!("Ποια μέθοδος παραγοντοποίησης εφαρμόστηκε;<br><span class=\"big\" style=\"font-size:1.2rem\">{txt}</span>"),
            groups: one(vec![select("Μέθοδος:", methods.iter().enumerate().map(|(i, m)| (i as i64, m.to_string())).collect(), idx as i64)]),
            hints: hints(
                "Μέτρησε τους όρους: 2 όροι → κοινός παράγοντας, διαφορά τετραγώνων ή κύβοι. 3 όροι → τριώνυμο ή τέλειο τετράγωνο. 4 όροι → ομαδοποίηση.",
                "Δες αν οι όροι είναι τετράγωνα ή κύβοι.",
                "Τέλειο τετράγωνο: ο μεσαίος όρος είναι το διπλάσιο γινόμενο.",
                methods[idx].to_string(),
            ),
            solution: format!("<b>{}</b>", methods[idx]),
            diagnose: Box::new(|_| vec![tag("factor-method")]),
        };
    }
    let (expr, right, wrongs, how): (String, String, Vec<String>, String) = match r.r(0, 4) {
        0 => {
            // k²(p − q y) + m² x²(q y − p)
            // coprime pairs, so that nothing more can be factored out
            let (k, m, p, q) = loop {
                let (k, m, p, q) = (
                    r.r(2, 5) as i64,
                    r.r(2, 5) as i64,
                    r.r(1, 5) as i64,
                    r.r(2, 5) as i64,
                );
                if gcd(k as u64, m as u64) == 1 && gcd(p as u64, q as u64) == 1 {
                    break (k, m, p, q);
                }
            };
            (
                format!("{}({p} − {q}y) + {}x²({q}y − {p})", k * k, m * m),
                format!("({q}y − {p})({m}x − {k})({m}x + {k})"),
                vec![format!("({p} − {q}y)({m}x − {k})({m}x + {k})"), format!("({q}y − {p})({m}x − {k})²"), format!("({q}y − {p})({}x² + {})", m * m, k * k)],
                format!("{}({p} − {q}y) = −{}({q}y − {p}). Κοινός παράγοντας ({q}y − {p}): ({q}y − {p})({}x² − {}) και μετά διαφορά τετραγώνων.", k * k, k * k, m * m, k * k),
            )
        }
        1 => {
            // k x³ − k c³ + d x − d c
            let (k, c, d) = loop {
                let (k, c, d) = (r.r(1, 3) as i64, r.r(1, 3) as i64, r.r(1, 9) as i64);
                // gcd(k, d) = 1, otherwise the quadratic factor still has a common factor
                if gcd(k as u64, d as u64) == 1 {
                    break (k, c, d);
                }
            };
            (
                format!("{}x³ − {} + {d}x − {}", if k == 1 { String::new() } else { k.to_string() }, k * c.pow(3), d * c),
                format!("(x − {c})({}x² + {}x + {})", if k == 1 { String::new() } else { k.to_string() }, k * c, k * c * c + d),
                vec![format!("(x − {c})({}x² + {}x + {})", if k == 1 { String::new() } else { k.to_string() }, k * c, k * c * c), format!("(x + {c})({}x² − {}x + {})", if k == 1 { String::new() } else { k.to_string() }, k * c, k * c * c + d), format!("(x − {c})²({}x + {d})", if k == 1 { String::new() } else { k.to_string() })],
                format!("Ομαδοποίηση: {k}(x³ − {}) + {d}(x − {c}) = {k}(x − {c})(x² + {c}x + {}) + {d}(x − {c}) = (x − {c})[{k}(x² + {c}x + {}) + {d}]", c.pow(3), c * c, c * c),
            )
        }
        2 => {
            // x²(x − a) + B x(x − a) + C(a − x) with x² + Bx − C = (x + m)(x + n)
            let (a, m, n) = (r.r(1, 5) as i64, nz(r, -6, 6), nz(r, -6, 6));
            let (bb, cc) = (m + n, -m * n);
            if cc == 0 || bb == 0 {
                return c_factor2(r);
            }
            let bterm = if bb < 0 {
                format!(" − {}x(x − {a})", -bb)
            } else {
                format!(" + {bb}x(x − {a})")
            };
            let cterm = if cc < 0 {
                format!(" − {}({a} − x)", -cc)
            } else {
                format!(" + {cc}({a} − x)")
            };
            (
                format!("x²(x − {a}){bterm}{cterm}"),
                format!("(x − {a}){}{}", lin(1, m, "x"), lin(1, n, "x")).replace("<i>", "").replace("</i>", ""),
                vec![format!("(x − {a}){}{}", lin(1, -m, "x"), lin(1, -n, "x")).replace("<i>", "").replace("</i>", ""), format!("(x − {a})(x² {} {}x {} {})", if bb < 0 { "−" } else { "+" }, bb.abs(), if cc < 0 { "−" } else { "+" }, cc.abs()), format!("(x − {a})²{}", lin(1, m + n, "x")).replace("<i>", "").replace("</i>", "")],
                format!("Ο όρος με ({a} − x) γίνεται −(x − {a}). Κοινός παράγοντας (x − {a}): (x − {a})(x² {} {}x {} {}) και μετά τριώνυμο.", if bb < 0 { "−" } else { "+" }, bb.abs(), if cc < 0 { "+" } else { "−" }, cc.abs()),
            )
        }
        3 => {
            // (p x + 1)² − ψ²  written expanded
            let p = r.r(2, 5) as i64;
            (
                format!("{}x² − ψ² + {}x + 1", p * p, 2 * p),
                format!("({p}x + 1 − ψ)({p}x + 1 + ψ)"),
                vec![
                    format!("({p}x + 1)² − ψ²"),
                    format!("({p}x − ψ)({p}x + ψ) + {}x + 1", 2 * p),
                    format!("({p}x − 1 − ψ)({p}x − 1 + ψ)"),
                ],
                format!(
                    "Ομαδοποίηση: ({}x² + {}x + 1) − ψ² = ({p}x + 1)² − ψ², διαφορά τετραγώνων.",
                    p * p,
                    2 * p
                ),
            )
        }
        _ => {
            // k² α⁶ − m² β⁴ / n²
            let (k, m, n) = (r.r(2, 9) as i64, r.r(2, 12) as i64, r.r(3, 11) as i64);
            if gcd(m as u64, n as u64) != 1 {
                return c_factor2(r);
            }
            (
                format!("{}α⁶ − {}β⁴/{}", k * k, m * m, n * n),
                format!("({k}α³ − {m}β²/{n})({k}α³ + {m}β²/{n})"),
                vec![
                    format!("({k}α³ − {m}β²/{n})²"),
                    format!("({k}α² − {m}β/{n})({k}α² + {m}β/{n})"),
                    format!(
                        "({}α³ − {}β²/{})({}α³ + {}β²/{})",
                        k * k,
                        m * m,
                        n * n,
                        k * k,
                        m * m,
                        n * n
                    ),
                ],
                format!(
                    "{}α⁶ = ({k}α³)² και {}β⁴/{} = ({m}β²/{n})². Διαφορά τετραγώνων.",
                    k * k,
                    m * m,
                    n * n
                ),
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
        cat: "c-factor2",
        prompt: format!("Να αναλύσεις πλήρως σε γινόμενο πρώτων παραγόντων:<br><span class=\"big\" style=\"font-size:1.2rem\">{expr}</span>"),
        groups: one(vec![choice(r, "Παραγοντοποίηση:", &opts)]),
        hints: hints(
            "Ψάξε πρώτα κοινό παράγοντα. Αν δύο παρενθέσεις διαφέρουν μόνο στο πρόσημο, βγάλε ένα −: (α − β) = −(β − α).",
            "Με 4 όρους δοκίμασε ομαδοποίηση. Μετά δες αν κάτι αναλύεται κι άλλο (διαφορά τετραγώνων, τριώνυμο).",
            how.clone(),
            right.clone(),
        ),
        solution: format!("{how}<br><b>{expr} = {right}</b>"),
        diagnose: Box::new(|_| vec![tag("factor")]),
    }
}
