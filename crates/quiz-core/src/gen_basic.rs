//! Euclidean division, divisors and multiples, divisibility properties,
//! divisibility tests, primes and the enrichment tests (7, 11, 19).

use crate::mk::*;
use crate::model::*;
use crate::util::*;

// ---------------- Ευκλείδεια διαίρεση ----------------

pub fn eucl(r: &mut Rng) -> Question {
    match r.r(0, 3) {
        0 => {
            let d = r.r(7, 25);
            let big = loop {
                let x = r.r(120, 999);
                if !x.is_multiple_of(d) {
                    break x;
                }
            };
            let (q, rem) = (big / d, big % d);
            Question {
                cat: "eucl",
                prompt: format!("Κάνε την Ευκλείδεια διαίρεση <b>{big} : {d}</b>. Βρες το πηλίκο και το υπόλοιπο."),
                groups: one(vec![num("Πηλίκο π =", q), num("Υπόλοιπο υ =", rem)]),
                hints: hints(
                    format!("Πόσες φορές «χωράει» το {d} στο {big};"),
                    format!("{d} · {q} = {} και {d} · {} = {}. Ποιο είναι πιο κοντά χωρίς να ξεπερνά το {big};", d * q, q + 1, d * (q + 1)),
                    format!("Το πηλίκο είναι {q}. Το υπόλοιπο είναι {big} − {}.", d * q),
                    format!("π = {q}, υ = {rem}"),
                ),
                solution: format!("{big} = {d} · {q} + {rem}, και ισχύει 0 ≤ {rem} &lt; {d}. Άρα π = <b>{q}</b>, υ = <b>{rem}</b>."),
                diagnose: Box::new(move |v| match (v[0].num(), v[1].num()) {
                    (Some(uq), Some(ur)) if uq >= 0 && d as i64 * uq + ur == big as i64 && ur >= d as i64 => vec![dm(
                        "eucl-rem",
                        format!("Η ισότητα {big} = {d}·{uq} + {ur} ισχύει, αλλά το υπόλοιπο πρέπει να είναι <b>μικρότερο από {d}</b>."),
                    )],
                    _ => vec![],
                }),
            }
        }
        1 => {
            let (d, q) = (r.r(6, 20), r.r(8, 30));
            let big = d * q + d - 1;
            Question {
                cat: "eucl",
                prompt: format!("Σε μια διαίρεση ο διαιρέτης είναι <b>{d}</b>, το πηλίκο <b>{q}</b> και το υπόλοιπο είναι το <b>μεγαλύτερο δυνατό</b>. Ποιος είναι ο διαιρετέος;"),
                groups: one(vec![num("Διαιρετέος Δ =", big)]),
                hints: hints(
                    "Θυμήσου: το υπόλοιπο είναι πάντα μικρότερο από τον διαιρέτη.",
                    format!("Το μεγαλύτερο υπόλοιπο που γίνεται με διαιρέτη {d} είναι {}.", d - 1),
                    format!("Δ = δ · π + υ = {d} · {q} + {}", d - 1),
                    format!("Δ = {big}"),
                ),
                solution: format!("υ = {d} − 1 = {}. Δ = {d}·{q} + {} = {} + {} = <b>{big}</b>.", d - 1, d - 1, d * q, d - 1),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some((d * q + d) as i64) {
                        vec![dm("eucl-rem", format!("Πήρες υπόλοιπο {d}. Το υπόλοιπο πρέπει να είναι <b>μικρότερο</b> από τον διαιρέτη, άρα το πιο μεγάλο είναι {}.", d - 1))]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        2 => {
            // "at least how many ... are needed": round the quotient up
            let t = r.pick(&[
                ("Ένα σχολείο επέλεξε {n} μαθητές για να δουν μια θεατρική παράσταση. Κάθε καθηγητής μπορεί να συνοδεύει μέχρι {c} μαθητές. Ποιος είναι ο μικρότερος αριθμός καθηγητών που χρειάζονται;", "καθηγητές", "μαθητές"),
                ("Για μια εκδρομή δήλωσαν συμμετοχή {n} μαθητές. Κάθε λεωφορείο χωρά {c} επιβάτες. Πόσα λεωφορεία χρειάζονται τουλάχιστον;", "λεωφορεία", "μαθητές"),
                ("Ένας ζαχαροπλάστης έχει {n} κουλουράκια και κάθε κουτί χωρά {c}. Πόσα κουτιά χρειάζεται τουλάχιστον για να τα συσκευάσει όλα;", "κουτιά", "κουλουράκια"),
            ]);
            let c = r.r(12, 40);
            let n = loop {
                let x = r.r(60, 400);
                if !x.is_multiple_of(c) {
                    break x;
                }
            };
            let (q, rem) = (n / c, n % c);
            let (unit, items) = (t.1, t.2);
            Question {
                cat: "eucl",
                prompt: t.0.replace("{n}", &n.to_string()).replace("{c}", &c.to_string()),
                groups: one(vec![num(format!("{unit}:"), q + 1)]),
                hints: hints(
                    format!("Κάνε την Ευκλείδεια διαίρεση {n} : {c}."),
                    format!("{n} = {c} · {q} + {rem}."),
                    format!("Με {q} {unit} περισσεύουν {rem} {items}. Τι κάνουμε με αυτά;"),
                    format!("{} {unit}", q + 1),
                ),
                solution: format!("{n} = {c}·{q} + {rem}. Με {q} {unit} περισσεύουν {rem} {items}, άρα χρειάζεται ένα ακόμα: <b>{}</b> {unit}.", q + 1),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some(q as i64) {
                        vec![dm("ceil", format!("Με {q} {unit} μένουν {rem} {items} χωρίς θέση. Χρειάζεται <b>ένα ακόμα</b>."))]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let (d, n) = (r.r(0, 6) as usize, r.r(20, 200));
            let (w, rem) = (n / 7, n % 7);
            let ans = (d + rem as usize) % 7;
            Question {
                cat: "eucl",
                prompt: format!("Σήμερα είναι <b>{}</b>. Τι μέρα θα είναι μετά από <b>{n}</b> μέρες;", DAYS[d]),
                groups: one(vec![day_select(ans)]),
                hints: hints(
                    "Κάθε 7 μέρες ξαναέρχεται η ίδια μέρα της εβδομάδας.",
                    format!("Κάνε τη διαίρεση {n} : 7."),
                    format!("{n} = 7 · {w} + {rem}. Οι {w} εβδομάδες μάς φέρνουν πάλι σε {}. Μένουν {rem} μέρες ακόμα.", day_art(d)),
                    DAYS[ans].to_string(),
                ),
                solution: format!("{n} = 7·{w} + {rem}. Μετά από {w} ολόκληρες εβδομάδες είναι πάλι {}. Προχωράμε {rem} μέρες ακόμα: <b>{}</b>.", DAYS[d], DAYS[ans]),
                diagnose: Box::new(move |v| {
                    if rem != 0 && v[0].num() == Some(d as i64) {
                        vec![dm("eucl-week", format!("Οι ολόκληρες εβδομάδες σε φέρνουν στην ίδια μέρα, αλλά περισσεύουν <b>{rem}</b> μέρες."))]
                    } else {
                        vec![]
                    }
                }),
            }
        }
    }
}

// ---------------- Διαιρέτες και πολλαπλάσια ----------------

pub fn div(r: &mut Rng) -> Question {
    match r.r(0, 3) {
        0 => {
            let n = loop {
                let x = r.r(12, 48);
                if divisors(x).len() >= 4 {
                    break x;
                }
            };
            let ds = divisors(n);
            let ds2 = ds.clone();
            let pairs: Vec<String> = ds
                .iter()
                .filter(|a| *a * *a <= n)
                .map(|a| format!("{a}·{}", n / a))
                .collect();
            Question {
                cat: "div",
                prompt: format!("Διάλεξε <b>όλους</b> τους διαιρέτες του αριθμού <span class=\"big\">{n}</span>."),
                groups: one(vec![set((1..=n).collect(), ds.clone())]),
                hints: hints(
                    "Διαιρέτης του n είναι κάθε αριθμός που διαιρεί το n χωρίς υπόλοιπο.",
                    "Μην ξεχνάς: κάθε αριθμός έχει διαιρέτες το 1 και τον εαυτό του.",
                    format!("Ψάξε σε ζευγάρια που δίνουν γινόμενο {n}: {} …", pairs[..pairs.len().min(2)].join(", ")),
                    format!("Οι διαιρέτες του {n}: {}", list(&ds)),
                ),
                solution: format!("Ζευγάρια: {}.<br>Διαιρέτες του {n}: <b>{}</b>.", pairs.join(", "), list(&ds)),
                diagnose: Box::new(move |v| {
                    let s = match v[0].set() {
                        Some(s) => s.clone(),
                        None => return vec![],
                    };
                    let mut out = vec![];
                    if let Some(x) = s.iter().find(|x| !ds2.contains(&(**x as u64))) {
                        out.push(dm("div-wrong", format!("Το {x} δεν διαιρεί το {n}: {n} = {x}·{} + {}.", n / *x as u64, n % *x as u64)));
                    }
                    if !s.contains(&1) || !s.contains(&(n as i64)) {
                        out.push(dm("div-trivial", format!("Το 1 και το {n} είναι πάντα διαιρέτες του {n}.")));
                    } else if ds2.iter().any(|d| !s.contains(&(*d as i64))) {
                        out.push(dm("div-missing", "Σου λείπουν κάποιοι διαιρέτες. Ψάξε σε ζευγάρια που δίνουν γινόμενο τον αριθμό."));
                    }
                    out
                }),
            }
        }
        1 => {
            let a = r.r(2, 15);
            let truth = r.chance(0.5);
            let m = r.r(2, 12);
            let b = if truth { a * m } else { a * m + r.r(1, a - 1) };
            let form = r.r(0, 3);
            let (stmt, value) = match form {
                0 => (format!("{a} | {b}"), truth),
                1 => (format!("Το {b} είναι πολλαπλάσιο του {a}."), truth),
                2 => (format!("Το {a} είναι πολλαπλάσιο του {}.", a * m), false),
                _ => (format!("Ο {a} είναι διαιρέτης του {b}."), truth),
            };
            let explain = if form == 2 {
                format!("Το {} είναι πολλαπλάσιο του {a} ({} = {a}·{m}), όχι το αντίστροφο. Το {a} είναι μικρότερο από το {}, άρα δεν μπορεί να είναι πολλαπλάσιό του.", a * m, a * m, a * m)
            } else if truth {
                format!("{b} = {a}·{m}, άρα το {a} διαιρεί το {b} και το {b} είναι πολλαπλάσιο του {a}.")
            } else {
                format!(
                    "{b} = {a}·{} + {}, η διαίρεση αφήνει υπόλοιπο, άρα το {a} δεν διαιρεί το {b}.",
                    b / a,
                    b % a
                )
            };
            Question {
                cat: "div",
                prompt: format!("Σωστό ή λάθος;<br><span class=\"big\" style=\"font-size:1.3rem\">{stmt}</span>"),
                groups: one(vec![truefalse(value)]),
                hints: hints(
                    "Το «α | β» διαβάζεται «το α διαιρεί το β» και σημαίνει ότι β = α·κ για κάποιον φυσικό κ.",
                    "Αν το α διαιρεί το β, τότε το β είναι πολλαπλάσιο του α και το α διαιρέτης του β.",
                    "Κάνε τη διαίρεση και δες αν αφήνει υπόλοιπο.",
                    explain.clone(),
                ),
                solution: format!("{explain} Η πρόταση είναι <b>{}</b>.", if value { "σωστή" } else { "λάθος" }),
                diagnose: Box::new(|_| vec![tag("div-tf")]),
            }
        }
        2 => {
            let n = loop {
                let x = r.r(12, 100);
                if divisors(x).len() >= 4 {
                    break x;
                }
            };
            let ds = divisors(n);
            let c = ds.len() as u64;
            Question {
                cat: "div",
                prompt: format!("Πόσους διαιρέτες έχει ο αριθμός <span class=\"big\">{n}</span>;"),
                groups: one(vec![num("Πλήθος διαιρετών:", c)]),
                hints: hints(
                    "Γράψε τους διαιρέτες σε ζευγάρια που δίνουν γινόμενο τον αριθμό.",
                    format!("Ξεκίνα: 1·{n}, …"),
                    "Μέτρησε και το 1 και τον ίδιο τον αριθμό.",
                    format!("{}: {c} διαιρέτες", list(&ds)),
                ),
                solution: format!("Διαιρέτες του {n}: {}. Σύνολο <b>{c}</b>.", list(&ds)),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some(c as i64 - 2) {
                        vec![dm(
                            "div-trivial",
                            format!("Μέτρησε και το 1 και το {n}: είναι κι αυτοί διαιρέτες."),
                        )]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let a = r.r(6, 29);
            let lim = loop {
                let x = r.r(100, 600);
                if !x.is_multiple_of(a) {
                    break x;
                }
            };
            let ans = (lim - 1) / a * a;
            Question {
                cat: "div",
                prompt: format!("Ποιο είναι το <b>μεγαλύτερο</b> πολλαπλάσιο του {a} που είναι <b>μικρότερο</b> από το {lim};"),
                groups: one(vec![num("Απάντηση:", ans)]),
                hints: hints(
                    format!("Κάνε την Ευκλείδεια διαίρεση {lim} : {a}."),
                    format!("{lim} = {a}·{} + {}.", lim / a, lim % a),
                    format!("Το {a}·{} είναι πολλαπλάσιο του {a} και είναι μικρότερο από {lim}.", lim / a),
                    format!("{ans}"),
                ),
                solution: format!("{lim} = {a}·{} + {}. Άρα το μεγαλύτερο πολλαπλάσιο κάτω από {lim} είναι {a}·{} = <b>{ans}</b>.", lim / a, lim % a, lim / a),
                diagnose: Box::new(move |v| match v[0].num() {
                    Some(x) if x >= lim as i64 => vec![msg(format!("Ο αριθμός πρέπει να είναι <b>μικρότερος</b> από το {lim}."))],
                    Some(x) if x % a as i64 != 0 => vec![msg(format!("Το {x} δεν είναι πολλαπλάσιο του {a}."))],
                    _ => vec![],
                }),
            }
        }
    }
}

// ---------------- Ιδιότητες διαιρετότητας ----------------

pub fn prop(r: &mut Rng) -> Question {
    let d = r.pick(&[7u64, 11, 13, 17, 19, 23]);
    match r.r(0, 2) {
        0 => {
            let m = r.r(1, 9);
            let big = d * m * 100;
            let yes = r.chance(0.5);
            let k = r.r(1, 4);
            let small = if yes { d * k } else { d * k + r.r(1, d - 1) };
            let plus = r.chance(0.5);
            let op = if plus { "+" } else { "−" };
            let s_part = if yes {
                format!("Το {d} διαιρεί το {small}, γιατί {small} = {d}·{k}.")
            } else {
                format!(
                    "Το {d} <b>δεν</b> διαιρεί το {small} ({small} = {d}·{} + {}).",
                    small / d,
                    small % d
                )
            };
            let concl = if yes {
                format!("Αφού διαιρεί και τους δύο αριθμούς, διαιρεί και {} τους (Ιδιότητα 2). Άρα <b>ναι</b>.", if plus { "το άθροισμά" } else { "τη διαφορά" })
            } else {
                format!(
                    "Αφού διαιρεί τον έναν αλλά όχι τον άλλον, <b>δεν</b> διαιρεί {} τους.",
                    if plus {
                        "το άθροισμά"
                    } else {
                        "τη διαφορά"
                    }
                )
            };
            let p1 = format!("Το {d} διαιρεί το {big}, γιατί {big} = {d}·{}.", m * 100);
            Question {
                cat: "prop",
                prompt: format!("Χωρίς να κάνεις τις πράξεις, εξέτασε αν το <b>{d}</b> διαιρεί τον αριθμό <span class=\"big\">{big} {op} {small}</span>."),
                groups: one(vec![yesno(yes)]),
                hints: hints(
                    "Ιδιότητα 2: αν ένας αριθμός διαιρεί δύο αριθμούς, διαιρεί και το άθροισμα και τη διαφορά τους.",
                    p1.clone(),
                    format!("Διαιρεί το {d} και το {small};"),
                    concl.clone(),
                ),
                solution: format!("{p1}<br>{s_part}<br>{concl}"),
                diagnose: Box::new(|_| vec![tag("prop")]),
            }
        }
        1 => {
            let m = r.r(1, 7);
            let base = d * m * 100;
            let yes = r.chance(0.5);
            let s = loop {
                let x = r.r(10, 99);
                if x.is_multiple_of(d) == yes {
                    break x;
                }
            };
            let n = base + s;
            let concl = format!("Άρα το {d} {}διαιρεί το {n}.", yn(yes));
            Question {
                cat: "prop",
                prompt: format!("Με χρήση των ιδιοτήτων διαιρετότητας, εξέτασε αν ο αριθμός <b>{d}</b> διαιρεί τον αριθμό <span class=\"big\">{n}</span>."),
                groups: one(vec![yesno(yes)]),
                hints: hints(
                    format!("Γράψε το {n} ως άθροισμα: ένα μεγάλο κομμάτι που ξέρεις ότι διαιρείται με το {d}, συν ένα μικρό."),
                    format!("{n} = {base} + {s}"),
                    format!("Το {d} διαιρεί το {} (= {d}·{m}), άρα και το {base}. Διαιρεί και το {s};", d * m),
                    concl.clone(),
                ),
                solution: format!(
                    "{n} = {base} + {s}.<br>Το {d} διαιρεί το {}, άρα και το πολλαπλάσιό του {base} (Ιδιότητα 1).<br>Το {d} {}διαιρεί το {s}.<br>{concl}",
                    d * m,
                    yn(yes)
                ),
                diagnose: Box::new(|_| vec![tag("prop")]),
            }
        }
        _ => {
            let a = r.pick(&[6u64, 8, 9, 10, 12, 14, 15, 18, 20, 21, 24, 30]);
            let kind = r.r(0, 2);
            let b = match kind {
                0 => r.pick(
                    &divisors(a)
                        .into_iter()
                        .filter(|x| *x > 1 && *x < a)
                        .collect::<Vec<_>>(),
                ),
                1 => loop {
                    let x = r.r(2, 12);
                    if !a.is_multiple_of(x) {
                        break x;
                    }
                },
                _ => a * r.r(2, 3),
            };
            let truth = a.is_multiple_of(b);
            let why = if truth {
                format!("{b} | {a} και {a} | α, άρα {b} | α (μεταβατική ιδιότητα).")
            } else {
                format!(
                    "Αντιπαράδειγμα: για α = {a} ισχύει {a} | {a}, αλλά το {b} δεν διαιρεί το {a}."
                )
            };
            Question {
                cat: "prop",
                prompt: format!("Σωστό ή λάθος;<br><span class=\"big\" style=\"font-size:1.3rem\">Αν {a} | α, τότε {b} | α.</span>"),
                groups: one(vec![truefalse(truth)]),
                hints: hints(
                    "Μεταβατική ιδιότητα: αν α | β και β | γ, τότε α | γ.",
                    format!("Διαιρεί το {b} το {a};"),
                    "Για να δείξεις ότι κάτι είναι λάθος, αρκεί ένα αντιπαράδειγμα. Δοκίμασε α = τον ίδιο τον αριθμό.",
                    why.clone(),
                ),
                solution: format!("{why} Η πρόταση είναι <b>{}</b>.", if truth { "σωστή" } else { "λάθος" }),
                diagnose: Box::new(|_| vec![tag("prop")]),
            }
        }
    }
}

// ---------------- Κριτήρια διαιρετότητας ----------------

fn crit_rule(k: u64) -> &'static str {
    match k {
        2 => "Ένας αριθμός διαιρείται με το 2 όταν το τελευταίο ψηφίο του είναι 0, 2, 4, 6 ή 8.",
        5 => "Ένας αριθμός διαιρείται με το 5 όταν το τελευταίο ψηφίο του είναι 0 ή 5.",
        10 => "Ένας αριθμός διαιρείται με το 10 όταν το τελευταίο ψηφίο του είναι 0.",
        4 => "Ένας αριθμός διαιρείται με το 4 όταν ο αριθμός που σχηματίζουν τα δύο τελευταία ψηφία του διαιρείται με το 4.",
        25 => "Ένας αριθμός διαιρείται με το 25 όταν τα δύο τελευταία ψηφία του είναι 00, 25, 50 ή 75.",
        3 => "Ένας αριθμός διαιρείται με το 3 όταν το άθροισμα των ψηφίων του διαιρείται με το 3.",
        _ => "Ένας αριθμός διαιρείται με το 9 όταν το άθροισμα των ψηφίων του διαιρείται με το 9.",
    }
}

fn digit_sum_text(n: u64) -> (String, u64) {
    let s = n.to_string();
    let sum = s.chars().map(|c| c.to_digit(10).unwrap() as u64).sum();
    (
        s.chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" + "),
        sum,
    )
}

fn crit_calc(n: u64, k: u64) -> String {
    let s = n.to_string();
    let y = yn(n.is_multiple_of(k));
    match k {
        2 | 5 | 10 => format!("Το τελευταίο ψηφίο είναι {}.", &s[s.len() - 1..]),
        4 | 25 => format!(
            "Τα δύο τελευταία ψηφία σχηματίζουν το {}, που {y}διαιρείται με το {k}.",
            n % 100
        ),
        _ => {
            let (t, sum) = digit_sum_text(n);
            format!("Άθροισμα ψηφίων: {t} = {sum}, που {y}διαιρείται με το {k}.")
        }
    }
}

pub fn crit(r: &mut Rng) -> Question {
    if r.chance(0.5) {
        let k = r.pick(&[2u64, 3, 4, 5, 9, 10, 25, 3, 9, 4]);
        let want = r.chance(0.5);
        let n = loop {
            let x = r.r(100, 99_999);
            if x.is_multiple_of(k) == want {
                break x;
            }
        };
        let concl = format!(
            "Άρα ο {} {} με το {k}.",
            thousands(n),
            if n % k == 0 {
                "<b>διαιρείται</b>"
            } else {
                "<b>δεν διαιρείται</b>"
            }
        );
        return Question {
            cat: "crit",
            prompt: format!("Διαιρείται ο αριθμός <span class=\"big\">{}</span> με το <b>{k}</b>;<br><span class=\"muted\">Χρησιμοποίησε κριτήριο, χωρίς να κάνεις τη διαίρεση.</span>", thousands(n)),
            groups: one(vec![yesno(n % k == 0)]),
            hints: hints(format!("Ποιο είναι το κριτήριο διαιρετότητας με το {k};"), crit_rule(k), crit_calc(n, k), concl.clone()),
            solution: format!("{}<br>{}<br>{concl}", crit_rule(k), crit_calc(n, k)),
            diagnose: Box::new(|_| vec![tag("crit")]),
        };
    }
    struct K {
        txt: &'static str,
        f: fn(u64) -> bool,
        rule: &'static str,
        tens: bool,
        last: &'static [u64],
    }
    let kinds = [
        K { txt: "να διαιρείται με το 3", f: |n| n % 3 == 0, rule: crit_rule(3), tens: false, last: &[] },
        K { txt: "να διαιρείται με το 9", f: |n| n % 9 == 0, rule: crit_rule(9), tens: false, last: &[] },
        K { txt: "να διαιρείται με το 4", f: |n| n % 4 == 0, rule: crit_rule(4), tens: true, last: &[] },
        K { txt: "να διαιρείται με το 3 <b>και</b> το 5", f: |n| n % 15 == 0, rule: "Για το 5: τελευταίο ψηφίο 0 ή 5. Για το 3: άθροισμα ψηφίων πολλαπλάσιο του 3.", tens: false, last: &[0, 5] },
        K { txt: "να διαιρείται με το 5 <b>και</b> το 9, <b>αλλά όχι</b> με το 2", f: |n| n % 45 == 0 && n % 2 == 1, rule: "Για το 5 αλλά όχι το 2: τελευταίο ψηφίο 5. Για το 9: άθροισμα ψηφίων πολλαπλάσιο του 9.", tens: false, last: &[5] },
        K { txt: "να διαιρείται με το 2 <b>και</b> το 3", f: |n| n % 6 == 0, rule: "Για το 2: τελευταίο ψηφίο άρτιο. Για το 3: άθροισμα ψηφίων πολλαπλάσιο του 3.", tens: false, last: &[0, 2, 4, 6, 8] },
    ];
    let (k, ds, pos, sols) = loop {
        let k = &kinds[r.r(0, kinds.len() as u64 - 1) as usize];
        let len = r.r(4, 5) as usize;
        let mut ds = vec![r.r(1, 9)];
        for _ in 1..len {
            ds.push(r.r(0, 9));
        }
        if !k.last.is_empty() {
            ds[len - 1] = r.pick(k.last);
        }
        let pos = if k.tens {
            len - 2
        } else {
            r.r(1, len as u64 - 2) as usize
        };
        let with = |d: u64| {
            let mut c = ds.clone();
            c[pos] = d;
            c.iter().fold(0, |a, x| a * 10 + x)
        };
        let sols: Vec<u64> = (0..10).filter(|d| (k.f)(with(*d))).collect();
        if !sols.is_empty() && sols.len() < 10 {
            break (k, ds, pos, sols);
        }
    };
    let shown: String = ds
        .iter()
        .enumerate()
        .map(|(i, d)| {
            if i == pos {
                "<span class=\"gap\">?</span>".into()
            } else {
                d.to_string()
            }
        })
        .collect();
    let known: u64 = ds
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != pos)
        .map(|(_, d)| d)
        .sum();
    let last = ds[ds.len() - 1];
    let calc = if k.tens {
        format!(
            "Τα δύο τελευταία ψηφία είναι «?{last}». Δοκίμασε: {}. Ποια διαιρούνται με το 4;",
            (0..10)
                .map(|d| format!("{d}{last}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    } else {
        format!("Τα γνωστά ψηφία έχουν άθροισμα {known}. Πρόσθεσε κάθε ψηφίο 0–9 και δες ποιο άθροισμα ταιριάζει.")
    };
    let nums: Vec<String> = sols
        .iter()
        .map(|d| {
            ds.iter()
                .enumerate()
                .map(|(i, x)| {
                    if i == pos {
                        d.to_string()
                    } else {
                        x.to_string()
                    }
                })
                .collect()
        })
        .collect();
    let ans = format!(
        "Τα ψηφία που ταιριάζουν: <b>{}</b> ({}).",
        list(&sols),
        nums.join(", ")
    );
    let sols2 = sols.clone();
    Question {
        cat: "crit",
        prompt: format!("Ποιο ψηφίο μπορεί να μπει στη θέση του <b>?</b> ώστε ο αριθμός <span class=\"big\">{shown}</span> {};<br><span class=\"muted\">Διάλεξε <b>όλα</b> τα ψηφία που ταιριάζουν.</span>", k.txt),
        groups: one(vec![set((0..10).collect(), sols.clone())]),
        hints: hints("Θυμήσου τον κανόνα για κάθε διαιρέτη που ζητείται.", k.rule, calc.clone(), ans.clone()),
        solution: format!("{}<br>{}<br>{ans}", k.rule, calc.replace(" Ποια διαιρούνται με το 4;", "")),
        diagnose: Box::new(move |v| {
            let s = v[0].set().cloned().unwrap_or_default();
            if !s.is_empty() && s.iter().all(|d| sols2.contains(&(*d as u64))) && s.len() < sols2.len() {
                vec![dm("digits-partial", format!("Όσα διάλεξες είναι σωστά, αλλά υπάρχουν <b>{}</b> λύσεις συνολικά. Βρες και τις υπόλοιπες!", sols2.len()))]
            } else {
                vec![tag("crit")]
            }
        }),
    }
}

// ---------------- Πρώτοι και σύνθετοι ----------------

const TRAPS: [u64; 16] = [
    51, 57, 87, 91, 111, 119, 121, 133, 143, 161, 169, 187, 203, 209, 221, 247,
];

fn prime_reason(n: u64) -> String {
    if is_prime(n) {
        let lim = (n as f64).sqrt() as u64;
        let ps: Vec<u64> = (2..=lim).filter(|p| is_prime(*p)).collect();
        format!("Κανένας από τους πρώτους {} (μέχρι το {lim}, αφού {lim}² ≤ {n}) δεν διαιρεί το {n}. Άρα ο {n} είναι <b>πρώτος</b>.", list(&ps), lim = lim)
    } else {
        let p = (2..n).find(|p| n.is_multiple_of(*p)).unwrap();
        format!("{n} = {p} · {}. Άρα ο {n} είναι <b>σύνθετος</b>.", n / p)
    }
}

pub fn prime(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let n = if r.chance(0.5) {
                r.pick(&TRAPS)
            } else {
                loop {
                    let x = r.r(23, 199);
                    if is_prime(x) {
                        break x;
                    }
                }
            };
            let p = is_prime(n);
            Question {
                cat: "prime",
                prompt: format!("Είναι ο αριθμός <span class=\"big\">{n}</span> πρώτος;"),
                groups: one(vec![yesno(p)]),
                hints: hints(
                    "Πρώτος είναι ο αριθμός που έχει μόνο δύο διαιρέτες: το 1 και τον εαυτό του.",
                    "Δοκίμασε να διαιρέσεις με τους πρώτους 2, 3, 5, 7, 11, 13 …",
                    "Αρκεί να δοκιμάσεις πρώτους των οποίων το τετράγωνο δεν ξεπερνά τον αριθμό.",
                    prime_reason(n),
                ),
                solution: prime_reason(n),
                diagnose: Box::new(move |_| {
                    if p {
                        vec![tag("prime-comp")]
                    } else {
                        vec![dm(
                            "prime-comp",
                            "Μην σταματάς στο 2, 3 και 5: δοκίμασε και το 7, 11, 13, 17.",
                        )]
                    }
                }),
            }
        }
        1 => {
            let want = r.chance(0.5);
            let (a, b) = loop {
                let a = r.r(6, 60);
                let b = r.r(6, 60);
                if a != b && !is_prime(a) && !is_prime(b) && (gcd(a, b) == 1) == want {
                    break (a.min(b), a.max(b));
                }
            };
            let g = gcd(a, b);
            Question {
                cat: "prime",
                prompt: format!("Είναι οι αριθμοί <b>{a}</b> και <b>{b}</b> πρώτοι μεταξύ τους;"),
                groups: one(vec![yesno(g == 1)]),
                hints: hints(
                    "Δύο αριθμοί είναι πρώτοι μεταξύ τους όταν ο ΜΚΔ τους είναι 1.",
                    fac_lines(&[a, b]),
                    "Έχουν κοινό πρώτο παράγοντα;",
                    format!("ΜΚΔ({a}, {b}) = {g}"),
                ),
                solution: format!(
                    "{}<br>ΜΚΔ({a}, {b}) = {g}, άρα {}είναι πρώτοι μεταξύ τους.",
                    fac_lines(&[a, b]),
                    yn(g == 1)
                ),
                diagnose: Box::new(|_| vec![tag("coprime")]),
            }
        }
        _ => {
            let mut pool: Vec<u64> = vec![1, 2];
            pool.extend(TRAPS.iter().filter(|x| **x < 100));
            pool.extend([9, 15, 21, 27, 33, 39, 49, 63, 77]);
            let primes_pool: Vec<u64> = (3..100).filter(|x| is_prime(*x)).collect();
            let (opts, ans) = loop {
                let mut o: Vec<u64> = r.shuffle(&pool)[..4].to_vec();
                o.extend(&r.shuffle(&primes_pool)[..4]);
                o.sort_unstable();
                o.dedup();
                let ans: Vec<u64> = o.iter().copied().filter(|x| is_prime(*x)).collect();
                if o.len() == 8 && ans.len() >= 3 {
                    break (o, ans);
                }
            };
            let lines: Vec<String> = opts
                .iter()
                .map(|x| match *x {
                    1 => "1: δεν είναι ούτε πρώτος ούτε σύνθετος".into(),
                    x if is_prime(x) => format!("{x}: πρώτος"),
                    x => format!("{x} = {}", fmt_pow(&factor(x))),
                })
                .collect();
            let ans2 = ans.clone();
            Question {
                cat: "prime",
                prompt: "Διάλεξε <b>όλους</b> τους πρώτους αριθμούς.".into(),
                groups: one(vec![set(opts.clone(), ans.clone())]),
                hints: hints(
                    "Πρώτος είναι ο αριθμός που έχει ακριβώς δύο διαιρέτες: το 1 και τον εαυτό του.",
                    "Το 1 δεν είναι πρώτος. Το 2 είναι ο μόνος άρτιος πρώτος.",
                    "Πρόσεξε αριθμούς σαν το 51 = 3·17 ή το 91 = 7·13: μοιάζουν πρώτοι αλλά δεν είναι.",
                    format!("Πρώτοι: {}", list(&ans)),
                ),
                solution: lines.join("<br>"),
                diagnose: Box::new(move |v| {
                    let s = v[0].set().cloned().unwrap_or_default();
                    let mut out = vec![];
                    if s.contains(&1) {
                        out.push(dm("prime-one", "Το 1 <b>δεν</b> είναι πρώτος: έχει μόνο έναν διαιρέτη."));
                    }
                    if let Some(x) = s.iter().find(|x| **x > 1 && !is_prime(**x as u64)) {
                        out.push(dm("prime-comp", format!("Το {x} είναι σύνθετος: {x} = {}.", fmt_pow(&factor(*x as u64)))));
                    }
                    if ans2.iter().any(|p| !s.contains(&(*p as i64))) {
                        out.push(dm("prime-missed", "Σου ξέφυγε κάποιος πρώτος."));
                    }
                    out
                }),
            }
        }
    }
}

// ---------------- Δραστηριότητες εμπλουτισμού: κριτήρια 7, 11, 19 ----------------

pub fn enrich(r: &mut Rng) -> Question {
    let k = r.pick(&[7u64, 11, 19]);
    let want = r.chance(0.5);
    let n = loop {
        let x = if k == 11 {
            r.r(1_000, 999_999)
        } else {
            r.r(300, 99_999)
        };
        if (x % k == 0) == want {
            break x;
        }
    };
    let (rule, steps, last): (&str, Vec<String>, i64) = match k {
        11 => {
            let ds: Vec<i64> = n
                .to_string()
                .chars()
                .rev()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect();
            let mut t = ds[0].to_string();
            let mut s = ds[0];
            for (i, d) in ds.iter().enumerate().skip(1) {
                if i % 2 == 1 {
                    t += &format!(" − {d}");
                    s -= d;
                } else {
                    t += &format!(" + {d}");
                    s += d;
                }
            }
            (
                "Ξεκινούμε από το τελευταίο ψηφίο και αφαιρούμε και προσθέτουμε εναλλάξ τα ψηφία. Αν το αποτέλεσμα διαιρείται με το 11, διαιρείται και ο αριθμός.",
                vec![format!("{t} = {s}")],
                s,
            )
        }
        7 => {
            let (mut x, mut st) = (n, vec![]);
            while x >= 100 {
                let (h, t) = (x / 100, x % 100);
                st.push(format!("{x}: 2·{h} + {t} = {}", 2 * h + t));
                x = 2 * h + t;
            }
            ("Διπλασιάζουμε το πλήθος των εκατοντάδων και προσθέτουμε το διψήφιο μέρος που μένει. Επαναλαμβάνουμε μέχρι να μείνει διψήφιος. Αν αυτός διαιρείται με το 7, διαιρείται και ο αρχικός.", st, x as i64)
        }
        _ => {
            let (mut x, mut st) = (n, vec![]);
            while x >= 100 {
                let (t, u) = (x / 10, x % 10);
                st.push(format!("{x}: {t} + 2·{u} = {}", t + 2 * u));
                x = t + 2 * u;
            }
            ("Προσθέτουμε στο πλήθος των δεκάδων το διπλάσιο του ψηφίου των μονάδων. Επαναλαμβάνουμε μέχρι να μείνει διψήφιος. Αν αυτός διαιρείται με το 19, διαιρείται και ο αρχικός.", st, x as i64)
        }
    };
    let yes = n % k == 0;
    let concl = format!(
        "Το {last} {}διαιρείται με το {k}, άρα ο {} {}διαιρείται με το {k}.",
        yn(last % k as i64 == 0),
        thousands(n),
        yn(yes)
    );
    Question {
        cat: "enrich",
        prompt: format!("Με το κριτήριο διαιρετότητας με το <b>{k}</b>, εξέτασε αν ο αριθμός <span class=\"big\">{}</span> διαιρείται με το {k}.", thousands(n)),
        groups: one(vec![yesno(yes)]),
        hints: hints(rule, steps.first().cloned().unwrap_or_default(), steps.join("<br>"), concl.clone()),
        solution: format!("{rule}<br>{}<br>{concl}", steps.join("<br>")),
        diagnose: Box::new(|_| vec![tag("enrich")]),
    }
}
