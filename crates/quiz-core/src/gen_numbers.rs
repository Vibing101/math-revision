//! Κεφάλαιο 2: Αριθμοί (δυνάμεις, δυαδικό σύστημα, αλγεβρικές παραστάσεις,
//! αριθμητική τιμή, ισότητες και εξισώσεις στους φυσικούς).

use crate::mk::*;
use crate::model::*;
use crate::util::*;

const VARS: [&str; 5] = ["x", "y", "α", "β", "ω"];

fn v(name: &str) -> String {
    format!("<i>{name}</i>")
}

pub fn powers(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let (a, n) = loop {
                let a = r.r(2, 12);
                let n = if r.chance(0.15) { r.r(0, 1) } else { r.r(2, 5) };
                if a.pow(n as u32) <= 100_000 {
                    break (a, n);
                }
            };
            let val = a.pow(n as u32);
            let expl = match n {
                0 => format!("Ορίζεται ότι κάθε αριθμός (≠ 0) στη μηδενική είναι 1: {a}⁰ = 1."),
                1 => format!("Ορίζεται ότι {a}¹ = {a}."),
                _ => format!(
                    "{a}{} = {} = {val}",
                    sup(n as u32),
                    vec![a.to_string(); n as usize].join("·")
                ),
            };
            Question {
                cat: "powers",
                prompt: format!(
                    "Υπολόγισε τη δύναμη <span class=\"big\">{a}{}</span>.",
                    sup(n as u32)
                ),
                groups: one(vec![num("Αποτέλεσμα:", val)]),
                hints: hints(
                    "Η δύναμη αⁿ είναι το γινόμενο n παραγόντων ίσων με α.",
                    format!("Η βάση είναι {a} και ο εκθέτης {n}."),
                    if n >= 2 {
                        format!(
                            "Γράψε το γινόμενο: {}",
                            vec![a.to_string(); n as usize].join("·")
                        )
                    } else {
                        "Θυμήσου: α¹ = α και α⁰ = 1.".into()
                    },
                    expl.clone(),
                ),
                solution: expl,
                diagnose: Box::new(move |vv| {
                    if n >= 2 && vv[0].num() == Some((a * n) as i64) && a * n != val {
                        vec![dm("pow-mult", format!("Πολλαπλασίασες τη βάση με τον εκθέτη ({a}·{n}). Η δύναμη σημαίνει {} φορές το {a} πολλαπλασιασμένο με τον εαυτό του.", n))]
                    } else if n == 0 && vv[0].num() == Some(0) {
                        vec![dm(
                            "pow-zero",
                            "Κάθε αριθμός διάφορος του μηδενός στη μηδενική δύναμη είναι <b>1</b>.",
                        )]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        1 => {
            // order of operations with powers
            let (a, b, c) = (r.r(2, 9), r.r(2, 6), r.r(2, 5));
            let (expr, val, wrong, steps) = match r.r(0, 4) {
                0 => (
                    format!("{a} + {b}·{c}²"),
                    a + b * c * c,
                    (a + b) * c * c,
                    format!(
                        "{c}² = {}, {b}·{} = {}, {a} + {} = {}",
                        c * c,
                        c * c,
                        b * c * c,
                        b * c * c,
                        a + b * c * c
                    ),
                ),
                1 => (
                    format!("({a} + {b})² − {c}"),
                    (a + b).pow(2) - c,
                    a * a + b * b - c,
                    format!(
                        "{a} + {b} = {}, {}² = {}, {} − {c} = {}",
                        a + b,
                        a + b,
                        (a + b).pow(2),
                        (a + b).pow(2),
                        (a + b).pow(2) - c
                    ),
                ),
                2 => (
                    format!("{b}·{c}²"),
                    b * c * c,
                    (b * c).pow(2),
                    format!(
                        "Πρώτα η δύναμη: {c}² = {}, μετά {b}·{} = {}",
                        c * c,
                        c * c,
                        b * c * c
                    ),
                ),
                3 => (
                    format!("{a}² + {b}² − {c}"),
                    a * a + b * b - c,
                    (a + b).pow(2) - c,
                    format!(
                        "{a}² = {}, {b}² = {}, {} + {} − {c} = {}",
                        a * a,
                        b * b,
                        a * a,
                        b * b,
                        a * a + b * b - c
                    ),
                ),
                _ => {
                    let d = c * r.r(1, 4);
                    let m = 2u64.pow(3);
                    (
                        format!("2³·{d} : {c} + {a}"),
                        m * d / c + a,
                        m * d / (c + a),
                        format!(
                            "2³ = 8, 8·{d} = {}, {} : {c} = {}, {} + {a} = {}",
                            8 * d,
                            8 * d,
                            8 * d / c,
                            8 * d / c,
                            8 * d / c + a
                        ),
                    )
                }
            };
            Question {
                cat: "powers",
                prompt: format!("Υπολόγισε την τιμή της παράστασης <span class=\"big\">{expr}</span>."),
                groups: one(vec![num("Τιμή:", val)]),
                hints: hints(
                    "Σειρά πράξεων: παρενθέσεις, μετά δυνάμεις, μετά πολλαπλασιασμοί και διαιρέσεις, τέλος προσθέσεις και αφαιρέσεις.",
                    "Ξεκίνα από ό,τι είναι μέσα σε παρενθέσεις και από τις δυνάμεις.",
                    steps.clone(),
                    format!("{expr} = {val}"),
                ),
                solution: format!("{steps}<br><b>{expr} = {val}</b>"),
                diagnose: Box::new(move |vv| {
                    if vv[0].num() == Some(wrong as i64) && wrong != val {
                        vec![dm("order", "Πρόσεξε τη σειρά των πράξεων: οι δυνάμεις γίνονται <b>πριν</b> από πολλαπλασιασμούς και προσθέσεις, εκτός αν υπάρχει παρένθεση.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let (a, n) = loop {
                let a = r.r(2, 7);
                let n = r.r(2, 6);
                if a.pow(n as u32) <= 20_000 {
                    break (a, n);
                }
            };
            let val = a.pow(n as u32);
            let as_product = r.chance(0.5);
            let prompt = if as_product {
                format!(
                    "Γράψε το γινόμενο <span class=\"big\">{}</span> ως δύναμη: {a}<sup>?</sup>",
                    vec![a.to_string(); n as usize].join("·")
                )
            } else {
                format!("Γράψε τον αριθμό <b>{val}</b> ως δύναμη του {a}: {val} = {a}<sup>?</sup>")
            };
            Question {
                cat: "powers",
                prompt,
                groups: one(vec![num("Εκθέτης:", n)]),
                hints: hints(
                    "Ο εκθέτης λέει πόσες φορές πολλαπλασιάζεται η βάση με τον εαυτό της.",
                    if as_product {
                        "Μέτρησε τους παράγοντες.".into()
                    } else {
                        format!("Διαίρεσε το {val} με το {a} ξανά και ξανά μέχρι να φτάσεις στο 1.")
                    },
                    format!("{a}² = {}, {a}³ = {}, …", a * a, a * a * a),
                    format!("{val} = {a}{}", sup(n as u32)),
                ),
                solution: format!(
                    "{} = {a}{}",
                    vec![a.to_string(); n as usize].join("·"),
                    sup(n as u32)
                ),
                diagnose: no_diag(),
            }
        }
    }
}

pub fn base2(r: &mut Rng) -> Question {
    if r.chance(0.5) {
        let n = r.r(5, 255);
        let bin = format!("{n:b}");
        let bits: Vec<u32> = bin.chars().map(|c| c.to_digit(2).unwrap()).collect();
        let len = bits.len();
        let terms: Vec<String> = bits
            .iter()
            .enumerate()
            .filter(|(_, b)| **b == 1)
            .map(|(i, _)| format!("2{}", sup((len - 1 - i) as u32)))
            .collect();
        let vals: Vec<String> = bits
            .iter()
            .enumerate()
            .filter(|(_, b)| **b == 1)
            .map(|(i, _)| (1u64 << (len - 1 - i)).to_string())
            .collect();
        Question {
            cat: "base2",
            prompt: format!("Μετάτρεψε τον αριθμό <span class=\"big\">{bin}<sub>2</sub></span> του δυαδικού συστήματος στο δεκαδικό σύστημα."),
            groups: one(vec![num("Στο δεκαδικό:", n)]),
            hints: hints(
                "Κάθε ψηφίο έχει αξία θέσης μια δύναμη του 2: από δεξιά 1, 2, 4, 8, 16, 32, 64, 128, …",
                format!("Οι θέσεις με 1 είναι: {}", terms.join(", ")),
                format!("Πρόσθεσε: {}", vals.join(" + ")),
                format!("{bin}<sub>2</sub> = {n}"),
            ),
            solution: format!("{bin}<sub>2</sub> = {} = {} = <b>{n}</b>", terms.join(" + "), vals.join(" + ")),
            diagnose: no_diag(),
        }
    } else {
        let n = r.r(5, 200);
        let bin = format!("{n:b}");
        let ans: u64 = bin.parse().unwrap();
        let mut steps = vec![];
        let mut x = n;
        while x > 0 {
            let p = 63 - x.leading_zeros();
            steps.push(format!(
                "η μεγαλύτερη δύναμη του 2 μέχρι το {x} είναι 2{} = {}",
                sup(p),
                1u64 << p
            ));
            x -= 1u64 << p;
        }
        let rev: u64 = bin.chars().rev().collect::<String>().parse().unwrap_or(0);
        Question {
            cat: "base2",
            prompt: format!("Γράψε τον αριθμό <span class=\"big\">{n}</span> στο <b>δυαδικό</b> σύστημα."),
            groups: one(vec![num("Στο δυαδικό (μόνο 0 και 1):", ans)]),
            hints: hints(
                "Γράψε τον αριθμό ως άθροισμα δυνάμεων του 2, κάθε δύναμη το πολύ μία φορά.",
                "Δυνάμεις του 2: 1, 2, 4, 8, 16, 32, 64, 128, 256.",
                format!("{}.", steps[0]),
                format!("{n} = {bin}<sub>2</sub>"),
            ),
            solution: format!("{}.<br>Βάζουμε 1 στις θέσεις αυτών των δυνάμεων και 0 στις υπόλοιπες: <b>{n} = {bin}<sub>2</sub></b>.", steps.join(",<br>")),
            diagnose: Box::new(move |vv| {
                let mut out = vec![];
                if let Some(u) = vv[0].num() {
                    if u.to_string().chars().any(|c| c != '0' && c != '1') {
                        out.push(dm("bin-digit", "Στο δυαδικό σύστημα χρησιμοποιούμε μόνο τα ψηφία 0 και 1."));
                    } else if u as u64 == rev && rev != ans {
                        out.push(dm("bin-rev", "Τα ψηφία σου είναι σωστά αλλά ανάποδα. Η μεγαλύτερη δύναμη του 2 γράφεται <b>αριστερά</b>."));
                    }
                }
                out
            }),
        }
    }
}

pub fn alg(r: &mut Rng) -> Question {
    let x = r.pick(&VARS);
    let xv = v(x);
    if r.chance(0.65) {
        let (expr, co, k, steps, wrong_k): (String, u64, u64, String, Option<u64>) = match r.r(0, 2)
        {
            0 => {
                let (a, b, c) = (r.r(3, 12), r.r(2, 9), r.r(1, 4));
                let co = a + b - c;
                (
                    format!("{a}{xv} + {b}{xv} − {c}{xv}"),
                    co,
                    0,
                    format!("({a} + {b} − {c}){xv} = {co}{xv}"),
                    None,
                )
            }
            1 => {
                let (a, b, c) = (r.r(2, 6), r.r(1, 9), r.r(1, 7));
                (
                    format!("{a}({xv} + {b}) + {c}{xv}"),
                    a + c,
                    a * b,
                    format!("Επιμεριστική: {a}({xv} + {b}) = {a}{xv} + {}. Μετά {a}{xv} + {c}{xv} = {}{xv}.", a * b, a + c),
                    Some(b),
                )
            }
            _ => {
                let (a, b, c, d) = (r.r(2, 9), r.r(1, 9), r.r(1, 9), r.r(1, 9));
                (
                    format!("{a}{xv} + {b} + {c}{xv} + {d}"),
                    a + c,
                    b + d,
                    format!(
                        "Ομαδοποιούμε: ({a} + {c}){xv} + ({b} + {d}) = {}{xv} + {}",
                        a + c,
                        b + d
                    ),
                    None,
                )
            }
        };
        let res = if k > 0 {
            format!("{co}{xv} + {k}")
        } else {
            format!("{co}{xv}")
        };
        return Question {
            cat: "alg",
            prompt: format!("Γράψε την αλγεβρική παράσταση <span class=\"big\">{expr}</span> στην πιο απλή μορφή της, δηλαδή ως <b>□{xv} + □</b>."),
            groups: one(vec![num(format!("Αριθμός μπροστά από το {x}:"), co), num("Σταθερός όρος (0 αν δεν υπάρχει):", k)]),
            hints: hints(
                format!("Πρόσθεσε μεταξύ τους τους όρους με {xv} και χωριστά τους σκέτους αριθμούς."),
                "Αν υπάρχει παρένθεση, εφάρμοσε πρώτα την επιμεριστική ιδιότητα: α(β + γ) = αβ + αγ.",
                steps.clone(),
                format!("{expr} = {res}"),
            ),
            solution: format!("{steps}<br><b>{expr} = {res}</b>"),
            diagnose: Box::new(move |vv| match (wrong_k, vv[1].num()) {
                (Some(b), Some(u)) if u == b as i64 && b != k => vec![dm("distrib", "Στην επιμεριστική ιδιότητα πολλαπλασιάζουμε τον αριθμό έξω από την παρένθεση με <b>κάθε</b> όρο μέσα σε αυτήν.")],
                _ => vec![],
            }),
        };
    }
    // words -> expression
    let k = r.r(2, 9);
    let items: Vec<(String, [String; 4])> = vec![
        (
            format!("Το διπλάσιο ενός αριθμού {xv} αυξημένο κατά {k}"),
            [
                format!("2{xv} + {k}"),
                format!("2({xv} + {k})"),
                format!("{xv}² + {k}"),
                format!("{k}{xv} + 2"),
            ],
        ),
        (
            format!("Το τριπλάσιο του αθροίσματος ενός αριθμού {xv} και του {k}"),
            [
                format!("3({xv} + {k})"),
                format!("3{xv} + {k}"),
                format!("{xv} + 3·{k}"),
                format!("3 + {xv} + {k}"),
            ],
        ),
        (
            format!("Το τετράγωνο ενός αριθμού {xv} μειωμένο κατά {k}"),
            [
                format!("{xv}² − {k}"),
                format!("2{xv} − {k}"),
                format!("({xv} − {k})²"),
                format!("{k} − {xv}²"),
            ],
        ),
        (
            format!("Ο αριθμός που είναι κατά {k} μεγαλύτερος από το μισό του {xv}"),
            [
                format!("{xv} : 2 + {k}"),
                format!("({xv} + {k}) : 2"),
                format!("2{xv} + {k}"),
                format!("{xv} : 2 − {k}"),
            ],
        ),
        (
            format!("Η διαφορά του {k} από το πενταπλάσιο ενός αριθμού {xv}"),
            [
                format!("5{xv} − {k}"),
                format!("{k} − 5{xv}"),
                format!("5({xv} − {k})"),
                format!("{xv} − 5·{k}"),
            ],
        ),
    ];
    let (text, opts) = r.pick(&items);
    let order = r.shuffle(&[0usize, 1, 2, 3]);
    // <select> options are plain text: drop the italics markup
    let options: Vec<(i64, String)> = order
        .iter()
        .enumerate()
        .map(|(i, j)| (i as i64, opts[*j].replace("<i>", "").replace("</i>", "")))
        .collect();
    let ans = order.iter().position(|j| *j == 0).unwrap() as i64;
    Question {
        cat: "alg",
        prompt: format!("Ποια αλγεβρική παράσταση περιγράφει τη φράση:<br><b>«{text}»</b>;"),
        groups: one(vec![select("Παράσταση:", options, ans)]),
        hints: hints(
            "Διάβασε τη φράση κομμάτι-κομμάτι και γράψε κάθε κομμάτι με σύμβολα.",
            "Διπλάσιο = 2·, τριπλάσιο = 3·, τετράγωνο = ², μισό = : 2.",
            "Πρόσεξε αν η πράξη γίνεται σε ένα άθροισμα: τότε χρειάζεται παρένθεση.",
            format!("{} → {}", text, opts[0]),
        ),
        solution: format!("«{text}» γράφεται <b>{}</b>.", opts[0]),
        diagnose: Box::new(|_| vec![tag("alg-words")]),
    }
}

pub fn val(r: &mut Rng) -> Question {
    let (x, y) = (r.r(1, 9), r.r(1, 9));
    let (xv, yv) = (v("x"), v("y"));
    let (a, b) = (r.r(2, 9), r.r(1, 9));
    let (expr, value, steps, wrong): (String, u64, String, Option<u64>) = match r.r(0, 4) {
        0 => (
            format!("{a}{xv} + {b}"),
            a * x + b,
            format!("{a}·{x} + {b} = {} + {b} = {}", a * x, a * x + b),
            None,
        ),
        1 => {
            let b = b.min(a * x * x);
            (
                format!("{a}{xv}² − {b}"),
                a * x * x - b,
                format!(
                    "{a}·{x}² − {b} = {a}·{} − {b} = {} − {b} = {}",
                    x * x,
                    a * x * x,
                    a * x * x - b
                ),
                Some((a * 2 * x).saturating_sub(b)),
            )
        }
        2 => (
            format!("{xv}² + {yv}²"),
            x * x + y * y,
            format!("{x}² + {y}² = {} + {} = {}", x * x, y * y, x * x + y * y),
            Some(2 * x + 2 * y),
        ),
        3 => {
            let b = b.min(a * (x + y));
            (
                format!("{a}({xv} + {yv}) − {b}"),
                a * (x + y) - b,
                format!(
                    "{a}·({x} + {y}) − {b} = {a}·{} − {b} = {} − {b} = {}",
                    x + y,
                    a * (x + y),
                    a * (x + y) - b
                ),
                None,
            )
        }
        _ => (
            format!("{xv}³ + {a}{yv}"),
            x.pow(3) + a * y,
            format!(
                "{x}³ + {a}·{y} = {} + {} = {}",
                x.pow(3),
                a * y,
                x.pow(3) + a * y
            ),
            Some(3 * x + a * y),
        ),
    };
    let uses_y = expr.contains("<i>y</i>");
    let given = if uses_y {
        format!("{xv} = {x} και {yv} = {y}")
    } else {
        format!("{xv} = {x}")
    };
    Question {
        cat: "val",
        prompt: format!("Να βρεις την αριθμητική τιμή της παράστασης <span class=\"big\">{expr}</span> για {given}."),
        groups: one(vec![num("Τιμή:", value)]),
        hints: hints(
            "Αντικατέστησε κάθε μεταβλητή με τον αριθμό της.",
            "Μετά κάνε τις πράξεις με τη σωστή σειρά: δυνάμεις, πολλαπλασιασμοί, προσθέσεις.",
            format!("Θυμήσου: {a}{xv} σημαίνει {a}·{xv}, και {xv}² σημαίνει {xv}·{xv}."),
            steps.clone(),
        ),
        solution: steps.to_string(),
        diagnose: Box::new(move |vv| match (wrong, vv[0].num()) {
            (Some(w), Some(u)) if u == w as i64 && w != value => vec![dm("val-sq", format!("Πρόσεξε: {}² σημαίνει {x}·{x}, όχι 2·{x}.", x))],
            _ => vec![],
        }),
    }
}

pub fn eq(r: &mut Rng) -> Question {
    let x = r.pick(&VARS);
    let xv = v(x);
    match r.r(0, 3) {
        0 | 1 => {
            let sol = r.r(2, 30);
            let (a, b) = (r.r(2, 12), r.r(1, 40));
            // (left side, right side, solution, steps, a typical wrong answer)
            let (lhs, rhs, answer, steps, wrong): (String, u64, u64, String, Option<u64>) =
                match r.r(0, 6) {
                    0 => (
                        format!("{xv} + {b}"),
                        sol + b,
                        sol,
                        format!("{xv} = {} − {b} = {sol}", sol + b),
                        Some(sol + 2 * b),
                    ),
                    1 => (
                        format!("{xv} − {b}"),
                        sol,
                        sol + b,
                        format!("{xv} = {sol} + {b} = {}", sol + b),
                        Some(sol.saturating_sub(b)),
                    ),
                    2 => {
                        let big = sol + b;
                        (
                            format!("{big} − {xv}"),
                            b,
                            sol,
                            format!("{xv} = {big} − {b} = {sol}"),
                            Some(big + b),
                        )
                    }
                    3 => (
                        format!("{a}{xv}"),
                        a * sol,
                        sol,
                        format!("{xv} = {} : {a} = {sol}", a * sol),
                        Some(a * sol - a),
                    ),
                    4 => (
                        format!("{xv} : {a}"),
                        sol,
                        sol * a,
                        format!("{xv} = {sol}·{a} = {}", sol * a),
                        Some(sol / a),
                    ),
                    5 => (
                        format!("{a}{xv} + {b}"),
                        a * sol + b,
                        sol,
                        format!(
                            "{a}{xv} = {} − {b} = {}, άρα {xv} = {} : {a} = {sol}",
                            a * sol + b,
                            a * sol,
                            a * sol
                        ),
                        Some((a * sol + 2 * b) / a),
                    ),
                    _ => {
                        let b = b.min(a * sol - 1);
                        (
                            format!("{a}{xv} − {b}"),
                            a * sol - b,
                            sol,
                            format!(
                                "{a}{xv} = {} + {b} = {}, άρα {xv} = {} : {a} = {sol}",
                                a * sol - b,
                                a * sol,
                                a * sol
                            ),
                            None,
                        )
                    }
                };
            let equation = format!("{lhs} = {rhs}");
            Question {
                cat: "eq",
                prompt: format!("Λύσε την εξίσωση <span class=\"big\">{equation}</span>."),
                groups: one(vec![num(format!("{x} ="), answer)]),
                hints: hints(
                    "Θέλουμε να μείνει ο άγνωστος μόνος του. Κάνουμε την αντίστροφη πράξη και στα δύο μέλη.",
                    "Η πρόσθεση «ξεκάνει» την αφαίρεση και ο πολλαπλασιασμός τη διαίρεση (και αντίστροφα).",
                    steps.clone(),
                    format!("{x} = {answer}. Επαλήθευση: βάλε {answer} στη θέση του {x}."),
                ),
                solution: format!("{steps}.<br>Επαλήθευση: αντικαθιστούμε {x} = {answer} και βγαίνει αληθής ισότητα. <b>{x} = {answer}</b>"),
                diagnose: Box::new(move |vv| match (wrong, vv[0].num()) {
                    (Some(w), Some(u)) if u == w as i64 && w != answer => vec![dm("eq-inverse", "Πρόσεξε ποια είναι η αντίστροφη πράξη. Κάνε επαλήθευση: βάλε τη λύση σου στην εξίσωση και δες αν βγαίνει ίδιο και στα δύο μέλη.")],
                    _ => vec![],
                }),
            }
        }
        2 => {
            let sol = r.r(2, 15);
            let (a, b) = (r.r(2, 9), r.r(1, 20));
            let rhs = a * sol + b;
            let test = if r.chance(0.5) { sol } else { sol + r.r(1, 3) };
            let lhs_val = a * test + b;
            Question {
                cat: "eq",
                prompt: format!("Είναι ο αριθμός <b>{test}</b> λύση της εξίσωσης <span class=\"big\">{a}{xv} + {b} = {rhs}</span>;"),
                groups: one(vec![yesno(test == sol)]),
                hints: hints(
                    "Ένας αριθμός είναι λύση όταν, αν τον βάλουμε στη θέση του αγνώστου, προκύπτει αληθής ισότητα.",
                    format!("Αντικατέστησε: {a}·{test} + {b}"),
                    format!("{a}·{test} + {b} = {lhs_val}"),
                    format!("{lhs_val} {} {rhs}", if lhs_val == rhs { "=" } else { "≠" }),
                ),
                solution: format!("{a}·{test} + {b} = {lhs_val}, και {lhs_val} {} {rhs}. Άρα το {test} <b>{}είναι</b> λύση.", if lhs_val == rhs { "=" } else { "≠" }, yn(test == sol)),
                diagnose: no_diag(),
            }
        }
        _ => {
            let sol = r.r(3, 25);
            let (a, b) = (r.r(2, 9), r.r(2, 30));
            let res = a * sol + b;
            let t = r.pick(&[
                format!("Σκέφτηκα έναν αριθμό. Τον πολλαπλασίασα με {a} και στο αποτέλεσμα πρόσθεσα {b}. Βρήκα {res}. Ποιον αριθμό σκέφτηκα;"),
                format!("Ο Ανδρέας αγόρασε {a} τετράδια ίδιας τιμής και ένα στυλό που κόστιζε {b} λεπτά. Πλήρωσε συνολικά {res} λεπτά. Πόσα λεπτά κόστιζε κάθε τετράδιο;"),
                format!("Σε μια αίθουσα υπάρχουν {a} σειρές με τον ίδιο αριθμό καρεκλών και άλλες {b} καρέκλες στο πλάι. Συνολικά υπάρχουν {res} καρέκλες. Πόσες καρέκλες έχει κάθε σειρά;"),
            ]);
            Question {
                cat: "eq",
                prompt: t,
                groups: one(vec![num("Απάντηση:", sol)]),
                hints: hints(
                    format!("Ονόμασε τον άγνωστο {xv} και γράψε μια εξίσωση."),
                    format!("Η εξίσωση είναι {a}{xv} + {b} = {res}."),
                    format!("{a}{xv} = {res} − {b} = {}", res - b),
                    format!("{xv} = {} : {a} = {sol}", res - b),
                ),
                solution: format!("{a}{xv} + {b} = {res} ⇔ {a}{xv} = {} ⇔ {xv} = <b>{sol}</b>.<br>Επαλήθευση: {a}·{sol} + {b} = {res}.", res - b),
                diagnose: Box::new(move |vv| {
                    if vv[0].num() == Some(((res + b) / a) as i64) && (res + b).is_multiple_of(a) {
                        vec![dm("eq-inverse", format!("Για να φύγει το «+ {b}», <b>αφαιρούμε</b> {b} και από τα δύο μέλη."))]
                    } else {
                        vec![]
                    }
                }),
            }
        }
    }
}
