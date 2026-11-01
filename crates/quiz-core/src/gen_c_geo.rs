//! Γ΄ Γυμνασίου: Κεφάλαιο 3 (Γεωμετρία: ανισοτικές σχέσεις, ισότητα τριγώνων),
//! Κεφάλαιο 4 (Τριγωνομετρία), Κεφάλαιο 9 (Παραλληλόγραμμα – Τραπέζια).

use crate::mk::*;
use crate::model::*;
use crate::util::*;

pub(crate) const TRIPLES: [(i64, i64, i64); 9] = [
    (3, 4, 5),
    (5, 12, 13),
    (6, 8, 10),
    (8, 15, 17),
    (7, 24, 25),
    (9, 12, 15),
    (20, 21, 29),
    (12, 16, 20),
    (9, 40, 41),
];

fn tf(r: &mut Rng, cat: &'static str, items: &[(&str, bool)], tagname: &'static str) -> Question {
    let (s, t) = r.pick(items);
    Question {
        cat,
        prompt: format!("Σωστό ή λάθος;<br><b>{s}</b>"),
        groups: one(vec![truefalse(t)]),
        hints: hints(
            "Θυμήσου τους ορισμούς και τις ιδιότητες.",
            "Για να δείξεις ότι κάτι είναι λάθος, αρκεί ένα αντιπαράδειγμα.",
            "Σκέψου ένα σχήμα που ικανοποιεί την υπόθεση.",
            if t { "Σωστό." } else { "Λάθος." },
        ),
        solution: format!(
            "Η πρόταση είναι <b>{}</b>.",
            if t { "σωστή" } else { "λάθος" }
        ),
        diagnose: Box::new(move |_| vec![tag(tagname)]),
    }
}

// ---------------- Κεφάλαιο 3 ----------------

pub fn c_tri_ineq(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let (a, b) = (r.r(3, 15), r.r(3, 15));
            let c = if r.chance(0.5) {
                r.r(a.abs_diff(b) + 1, a + b - 1)
            } else {
                a + b + r.r(0, 4)
            };
            let ok = c < a + b && c > a.abs_diff(b);
            Question {
                cat: "c-tri-ineq",
                prompt: format!("Υπάρχει τρίγωνο με πλευρές <b>{a} cm, {b} cm και {c} cm</b>;"),
                groups: one(vec![yesno(ok)]),
                hints: hints("Τριγωνική ανισότητα: κάθε πλευρά είναι μικρότερη από το άθροισμα των άλλων δύο.", "Αρκεί να ελέγξεις τη μεγαλύτερη πλευρά.", format!("Μεγαλύτερη: {}. Άθροισμα των άλλων: {}.", a.max(b).max(c), a + b + c - a.max(b).max(c)), if ok { "Ναι." } else { "Όχι." }),
                solution: format!("Η μεγαλύτερη πλευρά είναι {} και το άθροισμα των άλλων δύο {}. {}", a.max(b).max(c), a + b + c - a.max(b).max(c), if ok { "Αφού είναι μικρότερη, <b>υπάρχει</b> τρίγωνο." } else { "Αφού δεν είναι μικρότερη, <b>δεν υπάρχει</b> τρίγωνο." }),
                diagnose: Box::new(|_| vec![tag("tri-ineq")]),
            }
        }
        1 => {
            let (a, b) = (r.r(3, 15), r.r(3, 15));
            let (lo, hi) = (a.abs_diff(b), a + b);
            Question {
                cat: "c-tri-ineq",
                prompt: format!("Δύο πλευρές ενός τριγώνου είναι {a} cm και {b} cm. Μεταξύ ποιων τιμών βρίσκεται το μήκος x της τρίτης πλευράς;"),
                groups: one(vec![slots(&["", " &lt; <i>x</i> &lt; ", ""], vec![lo as i64, hi as i64], false)]),
                hints: hints("Η τρίτη πλευρά είναι μεγαλύτερη από τη διαφορά και μικρότερη από το άθροισμα των άλλων δύο.", format!("Διαφορά: |{a} − {b}| = {lo}"), format!("Άθροισμα: {a} + {b} = {hi}"), format!("{lo} &lt; x &lt; {hi}")),
                solution: format!("|{a} − {b}| &lt; x &lt; {a} + {b}, δηλαδή <b>{lo} &lt; x &lt; {hi}</b>."),
                diagnose: no_diag(),
            }
        }
        _ => {
            let (a, b, c) = loop {
                let (a, b, c) = (r.r(4, 15), r.r(4, 15), r.r(4, 15));
                if a != b && b != c && a != c && a < b + c && b < a + c && c < a + b {
                    break (a, b, c);
                }
            };
            // ΒΓ = a (opposite Â), ΑΓ = b (opposite B̂), ΑΒ = c (opposite Γ̂)
            let big = if a > b && a > c {
                0
            } else if b > c {
                1
            } else {
                2
            };
            let names = ["Â", "B̂", "Γ̂"];
            Question {
                cat: "c-tri-ineq",
                prompt: format!("Σε τρίγωνο ΑΒΓ είναι ΒΓ = {a} cm, ΑΓ = {b} cm και ΑΒ = {c} cm. Ποια είναι η <b>μεγαλύτερη</b> γωνία του;"),
                groups: one(vec![select("Μεγαλύτερη γωνία:", names.iter().enumerate().map(|(i, n)| (i as i64, n.to_string())).collect(), big)]),
                hints: hints("Απέναντι από τη μεγαλύτερη πλευρά βρίσκεται η μεγαλύτερη γωνία.", "Η γωνία Â είναι απέναντι από τη ΒΓ, η B̂ από την ΑΓ, η Γ̂ από την ΑΒ.", format!("Μεγαλύτερη πλευρά: {}", ["ΒΓ", "ΑΓ", "ΑΒ"][big as usize]), names[big as usize]),
                solution: format!("Η μεγαλύτερη πλευρά είναι η {}, άρα η μεγαλύτερη γωνία είναι η απέναντί της, <b>{}</b>.", ["ΒΓ", "ΑΓ", "ΑΒ"][big as usize], names[big as usize]),
                diagnose: Box::new(|_| vec![tag("tri-ineq")]),
            }
        }
    }
}

pub fn c_congr(r: &mut Rng) -> Question {
    let opts = [
        "Π – Π – Π",
        "Π – Γ – Π",
        "Γ – Π – Γ",
        "Ορθογώνια: υποτείνουσα και μία κάθετη πλευρά",
        "Δεν είναι απαραίτητα ίσα",
    ];
    let cases: [(&str, usize, &str); 7] = [
        ("ΑΒ = ΔΕ, ΒΓ = ΕΖ και ΑΓ = ΔΖ", 0, "Τρεις πλευρές ίσες μία προς μία: κριτήριο Π – Π – Π."),
        ("ΑΒ = ΔΕ, B̂ = Ê και ΒΓ = ΕΖ", 1, "Δύο πλευρές και η <b>περιεχόμενη</b> γωνία τους: κριτήριο Π – Γ – Π."),
        ("B̂ = Ê, ΒΓ = ΕΖ και Γ̂ = Ζ̂", 2, "Μία πλευρά και οι δύο <b>προσκείμενες</b> γωνίες της: κριτήριο Γ – Π – Γ."),
        ("ΑΒ = ΔΕ, ΑΓ = ΔΖ και B̂ = Ê", 4, "Η γωνία B̂ <b>δεν</b> είναι η περιεχόμενη των ΑΒ και ΑΓ (περιεχόμενη είναι η Â). Αυτά δεν αρκούν."),
        ("Â = Δ̂, B̂ = Ê και Γ̂ = Ζ̂", 4, "Τρεις ίσες γωνίες δεν αρκούν: τα τρίγωνα μπορεί να έχουν διαφορετικό μέγεθος."),
        ("Â = Δ̂ = 90°, ΒΓ = ΕΖ και ΑΒ = ΔΕ", 3, "Ορθογώνια τρίγωνα με ίσες υποτείνουσες και μία κάθετη πλευρά ίση."),
        ("Â = Δ̂, ΑΒ = ΔΕ και ΑΓ = ΔΖ", 1, "Δύο πλευρές και η περιεχόμενη γωνία τους Â = Δ̂: κριτήριο Π – Γ – Π."),
    ];
    let (given, k, why) = r.pick(&cases);
    let options: Vec<(i64, String)> = opts
        .iter()
        .enumerate()
        .map(|(i, o)| (i as i64, o.to_string()))
        .collect();
    Question {
        cat: "c-congr",
        prompt: format!("Για τα τρίγωνα ΑΒΓ και ΔΕΖ ισχύει: <b>{given}</b>.<br>Με ποιο κριτήριο είναι ίσα;"),
        groups: one(vec![select("Κριτήριο:", options, k as i64)]),
        hints: hints(
            "Κριτήρια: Π – Π – Π (τρεις πλευρές), Π – Γ – Π (δύο πλευρές και η περιεχόμενη γωνία), Γ – Π – Γ (μία πλευρά και οι προσκείμενες γωνίες).",
            "Περιεχόμενη γωνία δύο πλευρών είναι αυτή που σχηματίζουν οι δύο πλευρές μεταξύ τους.",
            "Κάνε ένα πρόχειρο σχήμα και σημείωσε τα ίσα στοιχεία.",
            opts[k].to_string(),
        ),
        solution: format!("{why}<br><b>{}</b>", opts[k]),
        diagnose: Box::new(|_| vec![tag("congr")]),
    }
}

// ---------------- Κεφάλαιο 4 ----------------

pub fn c_trig_ratio(r: &mut Rng) -> Question {
    let (a, b, c) = r.pick(&TRIPLES);
    let k = if c <= 13 { r.r(1, 2) as i64 } else { 1 };
    let (ab, ag, bg) = (a * k, b * k, c * k); // Â = 90°, ΒΓ hypotenuse
    match r.r(0, 2) {
        0 | 1 => {
            let hide_hyp = r.chance(0.5);
            let given = if hide_hyp {
                format!("ΑΒ = {ab} cm και ΑΓ = {ag} cm")
            } else {
                format!("ΑΒ = {ab} cm και ΒΓ = {bg} cm")
            };
            let (fn_, n, d, name) = match r.r(0, 2) {
                0 => (
                    "ημ",
                    ag,
                    bg,
                    "ημB̂ = απέναντι κάθετη / υποτείνουσα = ΑΓ / ΒΓ",
                ),
                1 => (
                    "συν",
                    ab,
                    bg,
                    "συνB̂ = προσκείμενη κάθετη / υποτείνουσα = ΑΒ / ΒΓ",
                ),
                _ => (
                    "εφ",
                    ag,
                    ab,
                    "εφB̂ = απέναντι κάθετη / προσκείμενη κάθετη = ΑΓ / ΑΒ",
                ),
            };
            let missing = if hide_hyp {
                format!(
                    "ΒΓ² = ΑΒ² + ΑΓ² = {} + {} = {}, ΒΓ = {bg}",
                    ab * ab,
                    ag * ag,
                    bg * bg
                )
            } else {
                format!(
                    "ΑΓ² = ΒΓ² − ΑΒ² = {} − {} = {}, ΑΓ = {ag}",
                    bg * bg,
                    ab * ab,
                    ag * ag
                )
            };
            let (rn, rd) = reduce(n, d);
            Question {
                cat: "c-trig-ratio",
                prompt: format!("Σε ορθογώνιο τρίγωνο ΑΒΓ (Â = 90°) είναι {given}. Να βρεις το <b>{fn_}B̂</b> (ανάγωγο κλάσμα)."),
                groups: one(vec![frac(format!("{fn_}B̂ ="), rn, rd)]),
                hints: hints("Βρες πρώτα την πλευρά που λείπει με το Πυθαγόρειο θεώρημα.", missing.clone(), name, fr(rn, rd)),
                solution: format!("{missing}.<br>{name} = {} = <b>{}</b>", fr_raw(n, d), fr(rn, rd)),
                diagnose: Box::new(move |v| {
                    let u = match v[0] { Val::Frac(x, y) => Some(reduce(x, y)), _ => None };
                    let swapped = [reduce(ab, bg), reduce(ag, bg), reduce(ag, ab), reduce(ab, ag)];
                    if u.is_some() && u != Some((rn, rd)) && swapped.contains(&u.unwrap()) {
                        vec![dm("trig-sides", "Μπέρδεψες τις πλευρές. Για τη γωνία B̂: απέναντι κάθετη είναι η ΑΓ, προσκείμενη κάθετη η ΑΒ, υποτείνουσα η ΒΓ.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let (sn, sd) = reduce(a, c);
            let (cn, cd) = reduce(b, c);
            Question {
                cat: "c-trig-ratio",
                prompt: format!(
                    "Για μια οξεία γωνία ω ισχύει ημω = {}. Να βρεις το <b>συνω</b>.",
                    fr(sn, sd)
                ),
                groups: one(vec![frac("συνω =", cn, cd)]),
                hints: hints(
                    "Ισχύει ημ²ω + συν²ω = 1.",
                    format!("συν²ω = 1 − {}", fr_raw(sn * sn, sd * sd)),
                    format!("συν²ω = {}", fr_raw(cn * cn, cd * cd)),
                    format!("Αφού ω οξεία, συνω &gt; 0: συνω = {}", fr(cn, cd)),
                ),
                solution: format!(
                    "συν²ω = 1 − ημ²ω = 1 − {} = {}, και αφού η ω είναι οξεία, <b>συνω = {}</b>.",
                    fr_raw(sn * sn, sd * sd),
                    fr_raw(cn * cn, cd * cd),
                    fr(cn, cd)
                ),
                diagnose: no_diag(),
            }
        }
    }
}

pub fn c_trig_solve(r: &mut Rng) -> Question {
    let th = r.r(20, 70) as f64;
    let t = th.to_radians();
    match r.r(0, 3) {
        0 => {
            let h = r.r(5, 30) as f64;
            let (which, val, f, fname) = if r.chance(0.5) {
                ("ΑΓ", h * t.sin(), t.sin(), "ημ")
            } else {
                ("ΑΒ", h * t.cos(), t.cos(), "συν")
            };
            Question {
                cat: "c-trig-solve",
                prompt: format!("Σε ορθογώνιο τρίγωνο ΑΒΓ (Â = 90°) είναι ΒΓ = {} cm και B̂ = {}°. Να υπολογίσεις την πλευρά <b>{which}</b> (με ένα δεκαδικό ψηφίο).", h, th),
                groups: one(vec![dec(format!("{which} (cm) ≈"), (val * 10.0).round() / 10.0, 0.11)]),
                hints: hints("Γνωρίζουμε την υποτείνουσα και μία οξεία γωνία.", format!("{fname}B̂ = {which} / ΒΓ"), format!("{which} = ΒΓ·{fname}{}° = {}·{}", th, h, dec_text((f * 10000.0).round() / 10000.0)), format!("{which} ≈ {}", dec_text((val * 10.0).round() / 10.0))),
                solution: format!("{fname}B̂ = {which}/ΒΓ ⇒ {which} = {}·{fname}{}° ≈ {}·{} ≈ <b>{} cm</b>", h, th, h, dec_text((f * 10000.0).round() / 10000.0), dec_text((val * 10.0).round() / 10.0)),
                diagnose: no_diag(),
            }
        }
        1 => {
            let ab = r.r(4, 25) as f64;
            let val = ab * t.tan();
            Question {
                cat: "c-trig-solve",
                prompt: format!("Σε ορθογώνιο τρίγωνο ΑΒΓ (Â = 90°) είναι ΑΒ = {} cm και B̂ = {}°. Να υπολογίσεις την <b>ΑΓ</b> (με ένα δεκαδικό ψηφίο).", ab, th),
                groups: one(vec![dec("ΑΓ (cm) ≈", (val * 10.0).round() / 10.0, 0.11)]),
                hints: hints("Η ΑΓ είναι απέναντι και η ΑΒ προσκείμενη κάθετη της B̂.", "εφB̂ = ΑΓ / ΑΒ", format!("ΑΓ = {}·εφ{}°", ab, th), format!("ΑΓ ≈ {}", dec_text((val * 10.0).round() / 10.0))),
                solution: format!("εφB̂ = ΑΓ/ΑΒ ⇒ ΑΓ = {}·εφ{}° ≈ {}·{} ≈ <b>{} cm</b>", ab, th, ab, dec_text((t.tan() * 10000.0).round() / 10000.0), dec_text((val * 10.0).round() / 10.0)),
                diagnose: no_diag(),
            }
        }
        2 => {
            let (a, b) = (r.r(3, 20) as f64, r.r(3, 20) as f64);
            let ang = (b / a).atan().to_degrees();
            Question {
                cat: "c-trig-solve",
                prompt: format!("Σε ορθογώνιο τρίγωνο ΑΒΓ (Â = 90°) είναι ΑΒ = {} cm και ΑΓ = {} cm. Να βρεις τη γωνία <b>B̂</b> (σε μοίρες, στρογγυλεμένη σε ακέραιο).", a, b),
                groups: one(vec![dec("B̂ ≈", ang.round(), 0.6)]),
                hints: hints("Για τη γωνία B̂: η ΑΓ είναι απέναντι και η ΑΒ προσκείμενη κάθετη.", format!("εφB̂ = {} / {}", b, a), format!("εφB̂ ≈ {}", dec_text(((b / a) * 1000.0).round() / 1000.0)), format!("Με την αντίστροφη εφαπτομένη (tan⁻¹): B̂ ≈ {}°", ang.round())),
                solution: format!("εφB̂ = ΑΓ/ΑΒ = {}/{} ≈ {}, άρα <b>B̂ ≈ {}°</b>.", b, a, dec_text(((b / a) * 1000.0).round() / 1000.0), ang.round()),
                diagnose: no_diag(),
            }
        }
        _ => {
            let (x, y) = (r.r(10, 80), r.r(10, 80));
            let (f1, f2, a1, a2) = match r.r(0, 2) {
                0 => ("ημ", "ημ", x, y),
                1 => ("συν", "συν", x, y),
                _ => ("ημ", "συν", x, 90 - x),
            };
            let v = |f: &str, a: u64| {
                let t = (a as f64).to_radians();
                if f == "ημ" {
                    t.sin()
                } else {
                    t.cos()
                }
            };
            let (l, rr) = (v(f1, a1), v(f2, a2));
            let ord = if (l - rr).abs() < 1e-9 {
                std::cmp::Ordering::Equal
            } else if l < rr {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            };
            let sym = match ord {
                std::cmp::Ordering::Less => "&lt;",
                std::cmp::Ordering::Equal => "=",
                _ => "&gt;",
            };
            Question {
                cat: "c-trig-solve",
                prompt: format!("Βάλε το σωστό σύμβολο (&lt;, = ή &gt;) χωρίς υπολογιστική μηχανή:<br><span class=\"big\">{f1}{a1}° &nbsp;□&nbsp; {f2}{a2}°</span>"),
                groups: one(vec![sign_select("Σύμβολο:", ord)]),
                hints: hints("Στις οξείες γωνίες: όσο μεγαλώνει η γωνία, το ημίτονο μεγαλώνει και το συνημίτονο μικραίνει.", "Για συμπληρωματικές γωνίες: ημω = συν(90° − ω).", "Σύγκρινε τις γωνίες.", format!("{f1}{a1}° {sym} {f2}{a2}°")),
                solution: format!("<b>{f1}{a1}° {sym} {f2}{a2}°</b>. {}", if f1 != f2 { "Οι γωνίες είναι συμπληρωματικές, άρα ημω = συν(90° − ω)." } else if f1 == "ημ" { "Το ημίτονο αυξάνεται όσο αυξάνεται η οξεία γωνία." } else { "Το συνημίτονο μειώνεται όσο αυξάνεται η οξεία γωνία." }),
                diagnose: Box::new(|_| vec![tag("trig-monotone")]),
            }
        }
    }
}

// ---------------- Κεφάλαιο 9 ----------------

pub fn c_quad(r: &mut Rng) -> Question {
    match r.r(0, 3) {
        0 => {
            let x = r.r(35, 145);
            Question {
                cat: "c-quad",
                prompt: format!(
                    "Σε παραλληλόγραμμο ΑΒΓΔ είναι Â = {x}°. Να βρεις τις γωνίες B̂ και Γ̂."
                ),
                groups: vec![group("", vec![num("B̂ =", 180 - x), num("Γ̂ =", x)])],
                hints: hints(
                    "Στο παραλληλόγραμμο οι απέναντι γωνίες είναι ίσες.",
                    "Δύο διαδοχικές γωνίες είναι παραπληρωματικές (άθροισμα 180°).",
                    format!("B̂ = 180° − {x}°"),
                    format!("B̂ = {}°, Γ̂ = {x}°", 180 - x),
                ),
                solution: format!(
                    "Â + B̂ = 180° ⇒ <b>B̂ = {}°</b>. Η Γ̂ είναι απέναντι της Â: <b>Γ̂ = {x}°</b>.",
                    180 - x
                ),
                diagnose: no_diag(),
            }
        }
        1 => {
            let (a, b, c) = r.pick(&TRIPLES);
            Question {
                cat: "c-quad",
                prompt: format!("Ένα ορθογώνιο ΑΒΓΔ έχει πλευρές ΑΒ = {a} cm και ΒΓ = {b} cm. Πόσο είναι η διαγώνιός του ΑΓ;"),
                groups: one(vec![num("ΑΓ (cm) =", c as u64)]),
                hints: hints("Η διαγώνιος χωρίζει το ορθογώνιο σε δύο ορθογώνια τρίγωνα.", "Πυθαγόρειο θεώρημα: ΑΓ² = ΑΒ² + ΒΓ².", format!("ΑΓ² = {} + {} = {}", a * a, b * b, c * c), format!("ΑΓ = {c}")),
                solution: format!("ΑΓ² = {a}² + {b}² = {} ⇒ <b>ΑΓ = {c} cm</b>.", c * c),
                diagnose: no_diag(),
            }
        }
        2 => {
            let (a, b, c) = r.pick(&TRIPLES);
            let (d1, d2) = (2 * a, 2 * b);
            Question {
                cat: "c-quad",
                prompt: format!("Οι διαγώνιοι ενός ρόμβου είναι {d1} cm και {d2} cm. Να βρεις την πλευρά και το εμβαδόν του."),
                groups: vec![group("α) Πλευρά", vec![num("cm:", c as u64)]), group("β) Εμβαδόν", vec![num("cm²:", (d1 * d2 / 2) as u64)])],
                hints: hints("Οι διαγώνιοι του ρόμβου διχοτομούνται και τέμνονται κάθετα.", format!("Μισές διαγώνιοι: {a} και {b}. Πυθαγόρειο: πλευρά² = {a}² + {b}²."), "Εμβαδόν ρόμβου = (δ₁·δ₂)/2.", format!("πλευρά = {c}, Ε = {}", d1 * d2 / 2)),
                solution: format!("Πλευρά² = {a}² + {b}² = {} ⇒ <b>{c} cm</b>. Ε = ({d1}·{d2})/2 = <b>{} cm²</b>.", c * c, d1 * d2 / 2),
                diagnose: Box::new(move |v| if v[1].num() == Some(d1 * d2) { vec![dm("rhombus-area", "Το εμβαδόν του ρόμβου είναι το <b>μισό</b> του γινομένου των διαγωνίων.")] } else { vec![] }),
            }
        }
        _ => tf(
            r,
            "c-quad",
            &[
                (
                    "Κάθε τετράπλευρο που έχει τις διαγώνιούς του ίσες είναι ορθογώνιο.",
                    false,
                ),
                ("Οι διαγώνιοι κάθε παραλληλογράμμου διχοτομούνται.", true),
                ("Οι διαγώνιοι του ρόμβου τέμνονται κάθετα.", true),
                ("Κάθε ρόμβος είναι τετράγωνο.", false),
                ("Κάθε τετράγωνο είναι και ρόμβος.", true),
                ("Οι διαγώνιοι του ορθογωνίου είναι ίσες.", true),
                (
                    "Οι απέναντι γωνίες ενός παραλληλογράμμου είναι παραπληρωματικές.",
                    false,
                ),
                (
                    "Ένα παραλληλόγραμμο με μία ορθή γωνία είναι ορθογώνιο.",
                    true,
                ),
                (
                    "Ένα τετράπλευρο με δύο πλευρές παράλληλες είναι πάντα παραλληλόγραμμο.",
                    false,
                ),
            ],
            "quad-props",
        ),
    }
}

pub fn c_theorems(r: &mut Rng) -> Question {
    let k = r.r(3, 20);
    let (prompt, ans, sol): (String, u64, String) = match r.r(0, 4) {
        0 => (format!("Στο τρίγωνο ΑΒΓ τα Δ και Ε είναι τα μέσα των πλευρών ΑΒ και ΑΓ, και ΒΓ = {} cm. Πόσο είναι το ΔΕ;", 2 * k), k, format!("Το τμήμα που ενώνει τα μέσα δύο πλευρών είναι παράλληλο με την τρίτη και ίσο με το μισό της: ΔΕ = {} : 2 = <b>{k} cm</b>.", 2 * k)),
        1 => (format!("Σε ορθογώνιο τρίγωνο ΑΒΓ (Â = 90°) η υποτείνουσα είναι ΒΓ = {} cm. Πόσο είναι η διάμεσος ΑΜ;", 2 * k), k, format!("Η διάμεσος προς την υποτείνουσα είναι ίση με το μισό της: ΑΜ = {} : 2 = <b>{k} cm</b>.", 2 * k)),
        2 => (format!("Σε ορθογώνιο τρίγωνο ΑΒΓ (Â = 90°) είναι B̂ = 30° και ΒΓ = {} cm. Πόσο είναι η ΑΓ;", 2 * k), k, format!("Απέναντι από γωνία 30° σε ορθογώνιο τρίγωνο η κάθετη πλευρά είναι το μισό της υποτείνουσας: ΑΓ = <b>{k} cm</b>.")),
        3 => {
            let small = r.r(2, 15);
            let big = small + 2 * r.r(1, 8);
            (format!("Ένα τραπέζιο έχει βάσεις {big} cm και {small} cm. Πόσο είναι η διάμεσός του;"), (big + small) / 2, format!("Διάμεσος τραπεζίου = (Β + β)/2 = ({big} + {small})/2 = <b>{} cm</b>.", (big + small) / 2))
        }
        _ => {
            let small = r.r(2, 15);
            let med = small + r.r(1, 8);
            (format!("Η διάμεσος ενός τραπεζίου είναι {med} cm και η μικρή του βάση {small} cm. Πόσο είναι η μεγάλη βάση;"), 2 * med - small, format!("(Β + {small})/2 = {med} ⇒ Β = 2·{med} − {small} = <b>{} cm</b>.", 2 * med - small))
        }
    };
    Question {
        cat: "c-theorems",
        prompt,
        groups: one(vec![num("cm:", ans)]),
        hints: hints(
            "Θεωρήματα: μέσα δύο πλευρών → μισό της τρίτης. Διάμεσος προς υποτείνουσα → μισό της υποτείνουσας.",
            "Απέναντι από 30° σε ορθογώνιο τρίγωνο → μισό της υποτείνουσας. Διάμεσος τραπεζίου → ημιάθροισμα βάσεων.",
            "Ποιο θεώρημα ταιριάζει εδώ;",
            sol.clone(),
        ),
        solution: sol,
        diagnose: no_diag(),
    }
}
