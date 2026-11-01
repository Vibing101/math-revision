//! Κεφάλαιο 5 (Ακολουθίες), Κεφάλαιο 7 (Διανύσματα), Κεφάλαιο 8 (Συναρτήσεις).

use crate::mk::*;
use crate::model::*;
use crate::util::*;

fn nz(r: &mut Rng, lo: i64, hi: i64) -> i64 {
    loop {
        let x = lo + r.r(0, (hi - lo) as u64) as i64;
        if x != 0 {
            return x;
        }
    }
}
/// First four distinct candidates (the correct one first); `more(i)` fills any gap.
fn four(cands: Vec<String>, more: impl Fn(usize) -> String) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for c in cands {
        if out.len() < 4 && !out.contains(&c) {
            out.push(c);
        }
    }
    let mut i = 0;
    while out.len() < 4 {
        let c = more(i);
        if !out.contains(&c) {
            out.push(c);
        }
        i += 1;
    }
    out
}

fn terms(v: &[i64]) -> String {
    v.iter().map(|x| signed(*x)).collect::<Vec<_>>().join(", ")
}
/// "3ν + 2", "−2ν − 1", "ν"
fn lin(k: i64, m: i64, var: &str) -> String {
    let head = match k {
        1 => var.to_string(),
        -1 => format!("−{var}"),
        k => format!("{}{var}", signed(k)),
    };
    match m {
        0 => head,
        m if m > 0 => format!("{head} + {m}"),
        m => format!("{head} − {}", -m),
    }
}

// ---------------- Κεφάλαιο 5: Ακολουθίες ----------------

pub fn seq_next(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let a1 = r.r(1, 30) as i64;
            let d = nz(r, -6, 9);
            let n = r.r(4, 5) as usize;
            let seq: Vec<i64> = (0..=n).map(|i| a1 + d * i as i64).collect();
            let missing = r.chance(0.4);
            let (shown, ans, which) = if missing {
                let k = r.r(1, n as u64 - 1) as usize;
                let shown: Vec<String> = seq[..n]
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if i == k {
                            "<span class=\"gap\">?</span>".into()
                        } else {
                            signed(*x)
                        }
                    })
                    .collect();
                (shown.join(", ") + ", …", seq[k], "ο όρος που λείπει")
            } else {
                (
                    format!("{}, …", terms(&seq[..n])),
                    seq[n],
                    "ο επόμενος όρος",
                )
            };
            Question {
                cat: "seq-next",
                prompt: format!("Δίνεται η ακολουθία <span class=\"big\">{shown}</span><br>Ποιος είναι {which};"),
                groups: one(vec![int("Όρος:", ans)]),
                hints: hints(
                    "Βρες τον κανόνα: τι αλλάζει από κάθε όρο στον επόμενο;",
                    "Βρες τη διαφορά δύο διαδοχικών όρων.",
                    format!("Κάθε όρος {} κατά {}.", if d > 0 { "αυξάνεται" } else { "μειώνεται" }, d.abs()),
                    signed(ans),
                ),
                solution: format!("Ο πρώτος όρος είναι {} και κάθε επόμενος {} κατά {}: {}. Άρα <b>{}</b>.", signed(a1), if d > 0 { "αυξάνεται" } else { "μειώνεται" }, d.abs(), terms(&seq), signed(ans)),
                diagnose: no_diag(),
            }
        }
        1 => {
            let (desc, seq): (&str, Vec<i64>) = match r.r(0, 3) {
                0 => {
                    let a = r.r(1, 5) as i64;
                    let q = r.pick(&[2i64, 3]);
                    (
                        "κάθε όρος πολλαπλασιάζεται με τον ίδιο αριθμό",
                        (0..6).map(|i| a * q.pow(i)).collect(),
                    )
                }
                1 => (
                    "οι όροι είναι τα τετράγωνα των θέσεων: ν²",
                    (1..=6).map(|i| i * i).collect(),
                ),
                2 => {
                    let k = r.r(1, 9) as i64;
                    (
                        "οι όροι είναι ν² συν έναν σταθερό αριθμό",
                        (1..=6).map(|i| i * i + k).collect(),
                    )
                }
                _ => (
                    "η διαφορά των διαδοχικών όρων αυξάνεται κατά 1 κάθε φορά (τρίγωνοι αριθμοί)",
                    (1..=6).map(|i| i * (i + 1) / 2).collect(),
                ),
            };
            Question {
                cat: "seq-next",
                prompt: format!("Δίνεται η ακολουθία <span class=\"big\">{}, …</span><br>Ποιος είναι ο επόμενος όρος;", terms(&seq[..5])),
                groups: one(vec![int("Όρος:", seq[5])]),
                hints: hints(
                    "Δεν αυξάνεται πάντα κατά τον ίδιο αριθμό. Δοκίμασε διαφορές, πηλίκα, τετράγωνα.",
                    format!("Διαφορές: {}", (1..5).map(|i| signed(seq[i] - seq[i - 1])).collect::<Vec<_>>().join(", ")),
                    format!("Κανόνας: {desc}."),
                    signed(seq[5]),
                ),
                solution: format!("Κανόνας: {desc}. Ο επόμενος όρος είναι <b>{}</b>.", signed(seq[5])),
                diagnose: no_diag(),
            }
        }
        _ => {
            let (a1, d) = (r.r(1, 20) as i64, r.r(2, 9) as i64);
            let n = r.r(6, 15) as i64;
            let ans = a1 + (n - 1) * d;
            Question {
                cat: "seq-next",
                prompt: format!("Μια ακολουθία έχει πρώτο όρο το <b>{a1}</b> και κάθε επόμενος όρος αυξάνεται κατά <b>{d}</b>. Ποιος είναι ο {n}ος όρος της;"),
                groups: one(vec![int(format!("α<sub>{n}</sub> ="), ans)]),
                hints: hints(
                    "Γράψε μερικούς όρους: α₁, α₂, α₃, …",
                    format!("{}, {}, {}, …", a1, a1 + d, a1 + 2 * d),
                    format!("Από τον 1ο ως τον {n}ο όρο κάνουμε {} βήματα των {d}.", n - 1),
                    format!("{a1} + {}·{d} = {ans}", n - 1),
                ),
                solution: format!("α<sub>{n}</sub> = {a1} + ({n} − 1)·{d} = {a1} + {} = <b>{ans}</b>", (n - 1) * d),
                diagnose: Box::new(move |v| if v[0].num() == Some(a1 + n * d) { vec![dm("seq-off", format!("Από τον 1ο ως τον {n}ο όρο υπάρχουν <b>{}</b> βήματα, όχι {n}.", n - 1))] } else { vec![] }),
            }
        }
    }
}

pub fn seq_gen(r: &mut Rng) -> Question {
    let nu = "ν";
    match r.r(0, 3) {
        0 => {
            let (k, m) = (nz(r, -5, 9), r.r(0, 20) as i64 - 6);
            let sq = r.chance(0.3);
            let n = r.r(5, 25) as i64;
            let (formula, val) = if sq {
                (
                    format!("ν² {} {}", if m < 0 { "−" } else { "+" }, m.abs()),
                    n * n + m,
                )
            } else {
                (lin(k, m, nu), k * n + m)
            };
            Question {
                cat: "seq-gen",
                prompt: format!("Ο γενικός όρος μιας ακολουθίας είναι <span class=\"big\">α<sub>ν</sub> = {formula}</span>. Να βρεις τον όρο α<sub>{n}</sub>."),
                groups: one(vec![int(format!("α<sub>{n}</sub> ="), val)]),
                hints: hints("Στον γενικό όρο αντικαθιστούμε το ν με τη θέση του όρου.", format!("Βάλε ν = {n}."), if sq { format!("{n}² = {}", n * n) } else { format!("{}·{n} = {}", signed(k), signed(k * n)) }, signed(val)),
                solution: if sq {
                    format!("α<sub>{n}</sub> = {n}² {} {} = {} {} {} = <b>{}</b>", if m < 0 { "−" } else { "+" }, m.abs(), n * n, if m < 0 { "−" } else { "+" }, m.abs(), signed(val))
                } else {
                    format!("α<sub>{n}</sub> = {}·{n} {} {} = <b>{}</b>", signed(k), if m < 0 { "−" } else { "+" }, m.abs(), signed(val))
                },
                diagnose: no_diag(),
            }
        }
        1 => {
            let (k, m) = (r.r(2, 7) as i64, r.r(0, 12) as i64 - 4);
            let seq: Vec<i64> = (1..=5).map(|i| k * i + m).collect();
            let correct = lin(k, m, nu);
            let opts = four(
                vec![
                    correct.clone(),
                    lin(m.max(1), k, nu),
                    lin(k, m + k, nu),
                    lin(k + 1, m - 1, nu),
                    lin(k, m - k, nu),
                ],
                |i| lin(k + 2 + i as i64, m, nu),
            );
            let order = r.shuffle(&[0usize, 1, 2, 3]);
            let options: Vec<(i64, String)> = order
                .iter()
                .enumerate()
                .map(|(i, j)| (i as i64, format!("αν = {}", opts[*j])))
                .collect();
            let ans = order.iter().position(|j| *j == 0).unwrap() as i64;
            Question {
                cat: "seq-gen",
                prompt: format!("Ποιος είναι ο γενικός όρος της ακολουθίας <span class=\"big\">{}, …</span>;", terms(&seq)),
                groups: one(vec![select("Γενικός όρος:", options, ans)]),
                hints: hints(
                    "Αν κάθε όρος αυξάνεται κατά τον ίδιο αριθμό κ, ο γενικός όρος περιέχει το κ·ν.",
                    format!("Κάθε όρος αυξάνεται κατά {k}, άρα έχουμε {k}ν."),
                    format!("Για ν = 1: {k}·1 = {k}, αλλά ο πρώτος όρος είναι {}. Τι πρέπει να προσθέσουμε;", seq[0]),
                    format!("αν = {correct}"),
                ),
                solution: format!("Η διαφορά είναι σταθερή, {k}, άρα αν = {k}ν + β. Από α₁ = {} βρίσκουμε β = {}. <b>αν = {correct}</b>. Έλεγχος: α₂ = {}.", seq[0], signed(m), seq[1]),
                diagnose: Box::new(|_| vec![tag("seq-formula")]),
            }
        }
        2 => {
            let (k, m) = (r.r(2, 9) as i64, r.r(0, 15) as i64 - 5);
            let n = r.r(5, 40) as i64;
            let v = k * n + m;
            Question {
                cat: "seq-gen",
                prompt: format!("Ο γενικός όρος μιας ακολουθίας είναι <span class=\"big\">α<sub>ν</sub> = {}</span>. Ποιος όρος (σε ποια θέση ν) είναι ίσος με {v};", lin(k, m, nu)),
                groups: one(vec![num("ν =", n as u64)]),
                hints: hints("Γράψε την εξίσωση α<sub>ν</sub> = τιμή και λύσε ως προς ν.", format!("{} = {v}", lin(k, m, nu)), format!("{k}ν = {}", v - m), format!("ν = {n}")),
                solution: format!("{} = {v} ⇔ {k}ν = {} ⇔ <b>ν = {n}</b>. Δηλαδή ο {n}ος όρος.", lin(k, m, nu), v - m),
                diagnose: no_diag(),
            }
        }
        _ => {
            // the book's beam: ν metres need 4ν − 1 rods (3, 7, 11, …)
            let n = r.r(6, 30) as i64;
            let ans = 4 * n - 1;
            Question {
                cat: "seq-gen",
                prompt: format!("Μια μεταλλική δοκός μήκους 1 m φτιάχνεται με 3 ράβδους, μήκους 2 m με 7 ράβδους, μήκους 3 m με 11 ράβδους, κ.ο.κ.<br>Πόσες ράβδους χρειάζεται μια δοκός μήκους <b>{n} m</b>;"),
                groups: one(vec![num("Ράβδοι:", ans as u64)]),
                hints: hints("Οι όροι είναι 3, 7, 11, … Πόσο αυξάνονται κάθε φορά;", "Αυξάνονται κατά 4, άρα ο γενικός όρος περιέχει 4ν.", "Για ν = 1: 4·1 = 4, αλλά χρειάζονται 3. Άρα αν = 4ν − 1.", format!("4·{n} − 1 = {ans}")),
                solution: format!("α<sub>ν</sub> = 4ν − 1, άρα α<sub>{n}</sub> = 4·{n} − 1 = <b>{ans}</b>."),
                diagnose: no_diag(),
            }
        }
    }
}

// ---------------- Κεφάλαιο 7: Διανύσματα ----------------

pub fn vec_concept(r: &mut Rng) -> Question {
    if r.chance(0.5) {
        let items = [
            ("η μάζα ενός σώματος", false),
            ("η ταχύτητα ενός αυτοκινήτου", true),
            ("η δύναμη που ασκούμε σε ένα κουτί", true),
            ("η θερμοκρασία ενός δωματίου", false),
            ("ο χρόνος", false),
            ("η μετατόπιση ενός σημείου", true),
            ("το μήκος ενός τμήματος", false),
            ("το βάρος ενός σώματος", true),
            ("ο όγκος ενός δοχείου", false),
            ("το εμβαδόν ενός οικοπέδου", false),
        ];
        let (s, vecq) = r.pick(&items);
        return Question {
            cat: "vec-concept",
            prompt: format!("Τι είδους μέγεθος είναι <b>{s}</b>;"),
            groups: one(vec![select(
                "Είδος:",
                vec![(0, "μονόμετρο".into()), (1, "διανυσματικό".into())],
                vecq as i64,
            )]),
            hints: hints(
                "Μονόμετρο: χαρακτηρίζεται μόνο από το μέτρο του (έναν αριθμό με μονάδα).",
                "Διανυσματικό: έχει μέτρο <b>και</b> κατεύθυνση.",
                "Έχει νόημα να ρωτήσουμε «προς τα πού;»",
                if vecq {
                    "διανυσματικό"
                } else {
                    "μονόμετρο"
                },
            ),
            solution: format!(
                "{} είναι <b>{}</b> μέγεθος{}.",
                s,
                if vecq {
                    "διανυσματικό"
                } else {
                    "μονόμετρο"
                },
                if vecq {
                    ": έχει μέτρο και κατεύθυνση"
                } else {
                    ": αρκεί ένας αριθμός με μονάδα"
                }
            ),
            diagnose: Box::new(|_| vec![tag("vec-kind")]),
        };
    }
    let items = [
        (
            "Δύο ίσα διανύσματα έχουν ίσα μέτρα και την ίδια κατεύθυνση.",
            true,
        ),
        (
            "Δύο αντίθετα διανύσματα έχουν ίσα μέτρα και αντίθετη κατεύθυνση.",
            true,
        ),
        ("Τα διανύσματα ΑΒ και ΒΑ είναι ίσα.", false),
        ("Τα διανύσματα ΑΒ και ΒΑ είναι αντίθετα.", true),
        ("Το μέτρο ενός διανύσματος μπορεί να είναι αρνητικό.", false),
        ("Το μηδενικό διάνυσμα έχει μέτρο 0.", true),
        ("Δύο διανύσματα με ίσα μέτρα είναι πάντα ίσα.", false),
        ("Ομόρροπα διανύσματα έχουν την ίδια φορά.", true),
        ("Αντίρροπα διανύσματα έχουν την ίδια φορά.", false),
    ];
    let (s, t) = r.pick(&items);
    Question {
        cat: "vec-concept",
        prompt: format!("Σωστό ή λάθος;<br><b>{s}</b>"),
        groups: one(vec![truefalse(t)]),
        hints: hints(
            "Ένα διάνυσμα έχει μέτρο (μήκος), διεύθυνση και φορά.",
            "Ίσα: ίδιο μέτρο και ίδια κατεύθυνση. Αντίθετα: ίδιο μέτρο, αντίθετη φορά.",
            "Το μέτρο είναι μήκος, άρα ποτέ αρνητικό.",
            if t { "Σωστό." } else { "Λάθος." },
        ),
        solution: format!(
            "Η πρόταση είναι <b>{}</b>.",
            if t { "σωστή" } else { "λάθος" }
        ),
        diagnose: Box::new(|_| vec![tag("vec-kind")]),
    }
}

fn vv(a: &str, b: &str) -> String {
    format!("<span class=\"vec\">{a}{b}</span>")
}

pub fn vec_ops(r: &mut Rng) -> Question {
    let pool = ["Α", "Β", "Γ", "Δ", "Ε", "Κ", "Λ", "Μ", "Ν", "Ρ"];
    let p: Vec<&str> = r.shuffle(&pool)[..4].to_vec();
    let (a, b, c, d) = (p[0], p[1], p[2], p[3]);
    let (expr, ans, why, para): (String, String, String, bool) = match r.r(0, 5) {
        0 => (format!("{} + {}", vv(a, b), vv(b, c)), format!("{a}{c}"), format!("Τα διανύσματα είναι διαδοχικά: το άθροισμα έχει αρχή την αρχή του πρώτου ({a}) και τέλος το τέλος του δεύτερου ({c})."), false),
        1 => (format!("{} + {} + {}", vv(a, b), vv(b, c), vv(c, d)), format!("{a}{d}"), format!("Διαδοχικά διανύσματα: από το {a} στο {d}."), false),
        2 => (format!("{} + {}", vv(b, c), vv(a, b)), format!("{a}{c}"), format!("Η πρόσθεση είναι αντιμεταθετική: {} + {} = {}.", vv(a, b), vv(b, c), vv(a, c)), false),
        3 => (format!("{} − {}", vv(a, b), vv(a, c)), format!("{c}{b}"), format!("α − β = α + (−β): {} − {} = {} + {} = {} + {} = {}.", vv(a, b), vv(a, c), vv(a, b), vv(c, a), vv(c, a), vv(a, b), vv(c, b)), false),
        4 => (format!("{} + {}", vv(a, b), vv(b, a)), "0".into(), "Αντίθετα διανύσματα έχουν άθροισμα το μηδενικό διάνυσμα.".into(), false),
        _ => (format!("{} + {}", vv(a, b), vv(a, d)), format!("{a}{c}"), format!("Κανόνας του παραλληλογράμμου: το άθροισμα δύο διανυσμάτων με κοινή αρχή είναι η διαγώνιος από την κοινή αρχή, δηλαδή {}.", vv(a, c)), true),
    };
    // options: correct, its reverse, and two other pairs
    let mut opts: Vec<String> = vec![ans.clone()];
    let rev = if ans == "0" {
        format!("{a}{a}")
    } else {
        ans.chars().rev().collect()
    };
    for cand in [
        rev,
        format!("{a}{b}"),
        format!("{b}{c}"),
        format!("{a}{d}"),
        format!("{c}{a}"),
        "0".into(),
    ] {
        if opts.len() < 4 && !opts.contains(&cand) && cand != format!("{a}{a}") {
            opts.push(cand);
        }
    }
    let label = |s: &String| {
        if s == "0" {
            "0 (μηδενικό διάνυσμα)".to_string()
        } else {
            format!("διάνυσμα {s}")
        }
    };
    let order = r.shuffle(&[0usize, 1, 2, 3]);
    let options: Vec<(i64, String)> = order
        .iter()
        .enumerate()
        .map(|(i, j)| (i as i64, label(&opts[*j])))
        .collect();
    let pos = order.iter().position(|j| *j == 0).unwrap() as i64;
    let rev_idx = order.iter().position(|j| *j == 1).unwrap() as i64;
    let shown_ans = if ans == "0" {
        "0⃗".to_string()
    } else {
        vv(
            &ans[..ans.char_indices().nth(1).unwrap().0],
            &ans[ans.char_indices().nth(1).unwrap().0..],
        )
    };
    Question {
        cat: "vec-ops",
        prompt: format!(
            "{}Ποιο διάνυσμα είναι ίσο με <span class=\"big\">{expr}</span>;",
            if para { format!("Το {a}{b}{c}{d} είναι παραλληλόγραμμο. ") } else { String::new() }
        ),
        groups: one(vec![select("Αποτέλεσμα:", options, pos)]),
        hints: hints(
            "Διαδοχικά διανύσματα: το τέλος του ενός είναι η αρχή του επόμενου. Το άθροισμα πάει από την πρώτη αρχή στο τελευταίο τέλος.",
            "Η διαφορά α − β είναι το άθροισμα του α με το αντίθετο του β. Το αντίθετο του ΑΒ είναι το ΒΑ.",
            "Κάνε ένα πρόχειρο σχήμα με τα σημεία.",
            format!("{expr} = {shown_ans}"),
        ),
        solution: format!("{why}<br><b>{expr} = {shown_ans}</b>"),
        diagnose: Box::new(move |v| if v[0].num() == Some(rev_idx) { vec![dm("vec-dir", "Το διάνυσμα που διάλεξες έχει την αντίθετη φορά. Πρόσεξε ποια είναι η αρχή και ποιο το τέλος.")] } else { vec![] }),
    }
}

// ---------------- Κεφάλαιο 8: Συναρτήσεις ----------------

fn grid_svg(pts: &[(i64, i64, &str)]) -> String {
    let (s, o) = (22.0, 132.0);
    let mut g = String::new();
    for i in -5..=5 {
        let p = o + i as f64 * s;
        g += &format!("<line x1=\"{p}\" y1=\"22\" x2=\"{p}\" y2=\"242\" stroke=\"currentColor\" stroke-opacity=\".18\"/><line x1=\"22\" y1=\"{p}\" x2=\"242\" y2=\"{p}\" stroke=\"currentColor\" stroke-opacity=\".18\"/>");
        if i != 0 {
            g += &format!("<text x=\"{}\" y=\"{}\" font-size=\"10\" text-anchor=\"middle\" fill-opacity=\".7\">{}</text>", p, o + 14.0, signed(i));
            g += &format!("<text x=\"{}\" y=\"{}\" font-size=\"10\" text-anchor=\"end\" fill-opacity=\".7\">{}</text>", o - 5.0, o - i as f64 * s + 4.0, signed(i));
        }
    }
    let mut dots = String::new();
    for (x, y, name) in pts {
        let (px, py) = (o + *x as f64 * s, o - *y as f64 * s);
        dots += &format!("<circle cx=\"{px}\" cy=\"{py}\" r=\"5\" style=\"fill:var(--accent)\"/><text x=\"{}\" y=\"{}\" font-size=\"14\" font-weight=\"700\">{name}</text>", px + 7.0, py - 7.0);
    }
    format!(
        "<svg class=\"fig\" viewBox=\"0 0 264 264\" width=\"264\" role=\"img\" aria-label=\"Σύστημα αξόνων\" fill=\"currentColor\">{g}\
<line x1=\"14\" y1=\"{o}\" x2=\"252\" y2=\"{o}\" stroke=\"currentColor\" stroke-width=\"1.6\"/><line x1=\"{o}\" y1=\"252\" x2=\"{o}\" y2=\"12\" stroke=\"currentColor\" stroke-width=\"1.6\"/>\
<text x=\"254\" y=\"{}\" font-size=\"12\" font-style=\"italic\">x</text><text x=\"{}\" y=\"12\" font-size=\"12\" font-style=\"italic\">y</text>{dots}</svg>",
        o - 4.0,
        o + 6.0
    )
}

fn quadrant(x: i64, y: i64) -> i64 {
    match (x.signum(), y.signum()) {
        (1, 1) => 0,
        (-1, 1) => 1,
        (-1, -1) => 2,
        (1, -1) => 3,
        _ => 4,
    }
}

pub fn fn_coord(r: &mut Rng) -> Question {
    let (x, y) = (r.r(0, 10) as i64 - 5, r.r(0, 10) as i64 - 5);
    match r.r(0, 2) {
        0 => {
            let (x, y) = if x == 0 && y == 0 { (3, -2) } else { (x, y) };
            Question {
                cat: "fn-coord",
                prompt: format!("Ποιες είναι οι συντεταγμένες του σημείου Α;<br>{}", grid_svg(&[(x, y, "Α")])),
                groups: one(vec![int("τετμημένη x =", x), int("τεταγμένη y =", y)]),
                hints: hints("Η τετμημένη (x) διαβάζεται στον οριζόντιο άξονα, η τεταγμένη (y) στον κατακόρυφο.", "Από το Α κατέβα (ή ανέβα) κάθετα στον άξονα x και πήγαινε οριζόντια στον άξονα y.", "Δεξιά και πάνω είναι θετικά, αριστερά και κάτω αρνητικά.", format!("Α({}, {})", signed(x), signed(y))),
                solution: format!("<b>Α({}, {})</b>", signed(x), signed(y)),
                diagnose: Box::new(move |v| if v[0].num() == Some(y) && v[1].num() == Some(x) && x != y { vec![dm("coord-swap", "Αντέστρεψες τις συντεταγμένες. Πρώτα γράφουμε την τετμημένη (x), μετά την τεταγμένη (y).")] } else { vec![] }),
            }
        }
        1 => {
            let (x, y) = (nz(r, -9, 9), if r.chance(0.15) { 0 } else { nz(r, -9, 9) });
            let q = quadrant(x, y);
            let names = [
                "1ο τεταρτημόριο",
                "2ο τεταρτημόριο",
                "3ο τεταρτημόριο",
                "4ο τεταρτημόριο",
                "πάνω σε άξονα",
            ];
            Question {
                cat: "fn-coord",
                prompt: format!("Πού βρίσκεται το σημείο <span class=\"big\">Β({}, {})</span>;", signed(x), signed(y)),
                groups: one(vec![select("Θέση:", names.iter().enumerate().map(|(i, n)| (i as i64, n.to_string())).collect(), q)]),
                hints: hints("1ο: x &gt; 0, y &gt; 0. 2ο: x &lt; 0, y &gt; 0. 3ο: x &lt; 0, y &lt; 0. 4ο: x &gt; 0, y &lt; 0.", "Αν μια συντεταγμένη είναι 0, το σημείο είναι πάνω σε άξονα.", format!("x {} 0 και y {} 0", if x > 0 { "&gt;" } else { "&lt;" }, if y > 0 { "&gt;" } else if y < 0 { "&lt;" } else { "=" }), names[q as usize]),
                solution: format!("Β({}, {}): <b>{}</b>.", signed(x), signed(y), names[q as usize]),
                diagnose: Box::new(|_| vec![tag("quadrant")]),
            }
        }
        _ => {
            let (x, y) = (nz(r, -9, 9), nz(r, -9, 9));
            let ask_x = r.chance(0.5);
            Question {
                cat: "fn-coord",
                prompt: format!("Ποια είναι η <b>{}</b> του σημείου <span class=\"big\">Γ({}, {})</span>;", if ask_x { "τετμημένη" } else { "τεταγμένη" }, signed(x), signed(y)),
                groups: one(vec![int("Απάντηση:", if ask_x { x } else { y })]),
                hints: hints("Σε ένα σημείο (x, y) η πρώτη συντεταγμένη είναι η τετμημένη και η δεύτερη η τεταγμένη.", "Τετμημένη → x, τεταγμένη → y.", if ask_x { "Πάρε τον πρώτο αριθμό." } else { "Πάρε τον δεύτερο αριθμό." }, signed(if ask_x { x } else { y })),
                solution: format!("Στο Γ({}, {}) η τετμημένη είναι {} και η τεταγμένη {}.", signed(x), signed(y), signed(x), signed(y)),
                diagnose: Box::new(|_| vec![tag("coord-swap")]),
            }
        }
    }
}

pub fn fn_func(r: &mut Rng) -> Question {
    let (a, b) = (nz(r, -5, 6), r.r(0, 16) as i64 - 8);
    let f = lin(a, b, "x");
    match r.r(0, 3) {
        0 => {
            let x = r.r(0, 12) as i64 - 6;
            let y = a * x + b;
            Question {
                cat: "fn-func",
                prompt: format!("Δίνεται η συνάρτηση <span class=\"big\">y = {f}</span>. Ποια τιμή του y αντιστοιχεί στο x = {}", signed(x)) + ";",
                groups: one(vec![int("y =", y)]),
                hints: hints("Αντικατέστησε το x με τον αριθμό.", format!("y = {}·{} {} {}", signed(a), par(x), if b < 0 { "−" } else { "+" }, b.abs()), "Πρόσεξε τα πρόσημα.", signed(y)),
                solution: format!("y = {}·{} {} {} = {} {} {} = <b>{}</b>", signed(a), par(x), if b < 0 { "−" } else { "+" }, b.abs(), signed(a * x), if b < 0 { "−" } else { "+" }, b.abs(), signed(y)),
                diagnose: Box::new(move |v| if v[0].num() == Some(-y) && y != 0 { vec![dm("sign", "Πρόσεξε τα πρόσημα στον πολλαπλασιασμό.")] } else { vec![] }),
            }
        }
        1 => {
            let n = 4;
            let xs: Vec<i64> = r.shuffle(&[-3i64, -2, -1, 0, 1, 2, 3, 4])[..n].to_vec();
            let is_fn = r.chance(0.5);
            let mut pairs: Vec<(i64, i64)> =
                xs.iter().map(|x| (*x, r.r(0, 12) as i64 - 4)).collect();
            if !is_fn {
                let x0 = pairs[0].0;
                pairs.push((x0, pairs[0].1 + nz(r, -3, 3)));
            }
            let shown = r.shuffle(&pairs);
            let list = shown
                .iter()
                .map(|(x, y)| format!("({}, {})", signed(*x), signed(*y)))
                .collect::<Vec<_>>()
                .join(", ");
            Question {
                cat: "fn-func",
                prompt: format!("Μια αντιστοιχία από το x στο y δίνεται από τα ζεύγη<br><span class=\"big\" style=\"font-size:1.2rem\">{list}</span><br>Είναι <b>συνάρτηση</b>;"),
                groups: one(vec![yesno(is_fn)]),
                hints: hints("Συνάρτηση είναι η αντιστοιχία όπου κάθε x αντιστοιχεί σε <b>ένα μόνο</b> y.", "Έλεγξε αν κάποιο x εμφανίζεται δύο φορές με διαφορετικό y.", "Κοίταξε τους πρώτους αριθμούς των ζευγών.", if is_fn { "Κάθε x εμφανίζεται μία φορά: είναι συνάρτηση." } else { "Ένα x έχει δύο διαφορετικά y: δεν είναι συνάρτηση." }),
                solution: if is_fn { "Κάθε x αντιστοιχεί σε ένα μόνο y, άρα <b>είναι συνάρτηση</b>.".to_string() } else { format!("Το x = {} αντιστοιχεί σε δύο διαφορετικές τιμές του y, άρα <b>δεν είναι συνάρτηση</b>.", signed(pairs[0].0)) },
                diagnose: Box::new(|_| vec![tag("fn-def")]),
            }
        }
        2 => {
            let x = r.r(0, 12) as i64 - 6;
            let y = a * x + b;
            Question {
                cat: "fn-func",
                prompt: format!("Δίνεται η συνάρτηση <span class=\"big\">y = {f}</span>. Για ποια τιμή του x είναι y = {}", signed(y)) + ";",
                groups: one(vec![int("x =", x)]),
                hints: hints("Γράψε την εξίσωση και λύσε ως προς x.", format!("{f} = {}", signed(y)), format!("{}x = {}", signed(a), signed(y - b)), format!("x = {}", signed(x))),
                solution: format!("{f} = {} ⇔ {}x = {} ⇔ <b>x = {}</b>", signed(y), signed(a), signed(y - b), signed(x)),
                diagnose: no_diag(),
            }
        }
        _ => {
            let sq = r.chance(0.5);
            let k = r.r(0, 6) as i64 - 3;
            let xs: Vec<i64> = vec![-2, 0, 3];
            let fx = |x: i64| if sq { x * x + k } else { a * x + b };
            let formula = if sq {
                format!("x² {} {}", if k < 0 { "−" } else { "+" }, k.abs())
            } else {
                f.clone()
            };
            Question {
                cat: "fn-func",
                prompt: format!("Συμπλήρωσε τον πίνακα τιμών της συνάρτησης <span class=\"big\">y = {formula}</span>."),
                groups: xs.iter().map(|x| group(format!("x = {}", signed(*x)), vec![int("y =", fx(*x))])).collect(),
                hints: hints("Για κάθε x αντικατέστησε στον τύπο.", "Πρόσεξε: (−2)² = 4.", format!("x = −2: y = {}", signed(fx(-2))), xs.iter().map(|x| format!("x = {} → y = {}", signed(*x), signed(fx(*x)))).collect::<Vec<_>>().join(", ")),
                solution: xs.iter().map(|x| format!("x = {} → y = <b>{}</b>", signed(*x), signed(fx(*x)))).collect::<Vec<_>>().join("<br>"),
                diagnose: Box::new(move |v| if sq && v[0].num() == Some(-4 + k) { vec![dm("neg-pow", "(−2)² = (−2)·(−2) = +4.")] } else { vec![] }),
            }
        }
    }
}

pub fn fn_graph(r: &mut Rng) -> Question {
    let (a, b) = (nz(r, -4, 5), r.r(0, 12) as i64 - 6);
    let f = lin(a, b, "x");
    match r.r(0, 2) {
        0 => {
            let x = r.r(0, 8) as i64 - 4;
            let on = r.chance(0.5);
            let y = a * x + b + if on { 0 } else { nz(r, -3, 3) };
            Question {
                cat: "fn-graph",
                prompt: format!("Το σημείο <span class=\"big\">Α({}, {})</span> ανήκει στη γραφική παράσταση της συνάρτησης <span class=\"big\">y = {f}</span>;", signed(x), signed(y)),
                groups: one(vec![yesno(on)]),
                hints: hints("Ένα σημείο ανήκει στη γραφική παράσταση αν οι συντεταγμένες του επαληθεύουν τον τύπο.", format!("Βάλε x = {} στον τύπο.", signed(x)), format!("y = {}", signed(a * x + b)), format!("{} {} {}", signed(a * x + b), if on { "=" } else { "≠" }, signed(y))),
                solution: format!("Για x = {}: y = {}. {} το Α ανήκει στη γραφική παράσταση.", signed(x), signed(a * x + b), if on { "Είναι ίσο με την τεταγμένη του Α, άρα <b>ναι</b>," } else { "Δεν είναι ίσο με την τεταγμένη του Α, άρα <b>όχι</b>, δεν" }),
                diagnose: no_diag(),
            }
        }
        1 => Question {
            cat: "fn-graph",
            prompt: format!("Σε ποιο σημείο η γραφική παράσταση της <span class=\"big\">y = {f}</span> τέμνει τον άξονα <b>y</b>; Γράψε την τεταγμένη του."),
            groups: one(vec![int("y =", b)]),
            hints: hints("Τα σημεία του άξονα y έχουν τετμημένη x = 0.", "Βάλε x = 0 στον τύπο.", format!("y = {}·0 {} {}", signed(a), if b < 0 { "−" } else { "+" }, b.abs()), format!("(0, {})", signed(b))),
            solution: format!("Για x = 0: y = {}. Το σημείο είναι <b>(0, {})</b>.", signed(b), signed(b)),
            diagnose: no_diag(),
        },
        _ => {
            // read a point of y = a x + b from the grid
            let pts: Vec<(i64, i64)> = (-5..=5).map(|x| (x, a * x + b)).filter(|(_, y)| y.abs() <= 5).collect();
            if pts.len() < 2 {
                return fn_graph(r);
            }
            let (p, q) = (pts[0], pts[pts.len() - 1]);
            let names = [(p.0, p.1, "Α"), (q.0, q.1, "Β")];
            let ok_formula = lin(a, b, "x");
            let opts = four(vec![ok_formula.clone(), lin(-a, b, "x"), lin(a, -b, "x"), lin(b.max(1), a, "x"), lin(a, b + 1, "x")], |i| lin(a + 1 + i as i64, b, "x"));
            let order = r.shuffle(&[0usize, 1, 2, 3]);
            let options: Vec<(i64, String)> = order.iter().enumerate().map(|(i, j)| (i as i64, format!("y = {}", opts[*j]))).collect();
            let ans = order.iter().position(|j| *j == 0).unwrap() as i64;
            Question {
                cat: "fn-graph",
                prompt: format!("Τα σημεία Α και Β ανήκουν στη γραφική παράσταση μιας συνάρτησης. Ποιος είναι ο τύπος της;<br>{}", grid_svg(&names)),
                groups: one(vec![select("Τύπος:", options, ans)]),
                hints: hints("Διάβασε τις συντεταγμένες των Α και Β.", format!("Α({}, {}) και Β({}, {})", signed(p.0), signed(p.1), signed(q.0), signed(q.1)), "Έλεγξε ποιος τύπος επαληθεύεται και από τα δύο σημεία.", format!("y = {ok_formula}")),
                solution: format!("Α({}, {}): {} = {}·{} {} {} ✓<br>Β({}, {}): {} = {}·{} {} {} ✓<br><b>y = {ok_formula}</b>", signed(p.0), signed(p.1), signed(p.1), signed(a), par(p.0), if b < 0 { "−" } else { "+" }, b.abs(), signed(q.0), signed(q.1), signed(q.1), signed(a), par(q.0), if b < 0 { "−" } else { "+" }, b.abs()),
                diagnose: no_diag(),
            }
        }
    }
}
