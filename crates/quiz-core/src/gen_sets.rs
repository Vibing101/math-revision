//! Κεφάλαιο 1: Σύνολα (έννοια συνόλου, σχέσεις, πράξεις).

use crate::mk::*;
use crate::model::*;
use crate::util::*;
use std::collections::BTreeSet;

const ALPHABET: [&str; 24] = [
    "Α", "Β", "Γ", "Δ", "Ε", "Ζ", "Η", "Θ", "Ι", "Κ", "Λ", "Μ", "Ν", "Ξ", "Ο", "Π", "Ρ", "Σ", "Τ",
    "Υ", "Φ", "Χ", "Ψ", "Ω",
];
const WORDS: [&str; 16] = [
    "ΠΟΡΤΟΓΑΛΙΑ",
    "ΜΑΘΗΜΑΤΙΚΑ",
    "ΘΑΛΑΣΣΑ",
    "ΚΥΠΡΟΣ",
    "ΛΕΜΕΣΟΣ",
    "ΑΡΙΘΜΟΣ",
    "ΣΥΝΟΛΟ",
    "ΛΕΥΚΩΣΙΑ",
    "ΑΜΜΟΧΩΣΤΟΣ",
    "ΠΑΠΑΓΑΛΟΣ",
    "ΑΓΓΛΙΑ",
    "ΓΑΛΛΙΑ",
    "ΛΑΡΝΑΚΑ",
    "ΠΑΦΟΣ",
    "ΚΕΡΥΝΕΙΑ",
    "ΤΡΙΓΩΝΟ",
];

fn letters(word: &str) -> Vec<usize> {
    let set: BTreeSet<usize> = word
        .chars()
        .filter_map(|c| ALPHABET.iter().position(|a| a.starts_with(c)))
        .collect();
    set.into_iter().collect()
}

fn braces(items: &[String]) -> String {
    if items.is_empty() {
        "∅".into()
    } else {
        format!("{{{}}}", items.join(", "))
    }
}
fn braces_n(v: &[u64]) -> String {
    braces(&v.iter().map(|x| x.to_string()).collect::<Vec<_>>())
}
fn letter_list(v: &[usize]) -> String {
    braces(
        &v.iter()
            .map(|i| ALPHABET[*i].to_string())
            .collect::<Vec<_>>(),
    )
}

/// Numeric sets defined by a property.
fn described(r: &mut Rng) -> (String, Vec<u64>) {
    match r.r(0, 5) {
        0 => {
            let k = r.r(9, 20);
            (
                format!("οι άρτιοι φυσικοί αριθμοί που είναι μικρότεροι του {k}"),
                (1..k).filter(|x| x % 2 == 0).collect(),
            )
        }
        1 => {
            let n = r.pick(&[12u64, 16, 18, 20, 24, 28, 30]);
            (format!("οι διαιρέτες του {n}"), divisors(n))
        }
        2 => {
            let a = r.r(3, 7);
            let b = a * r.r(3, 5) + r.r(1, a - 1);
            (
                format!("τα μη μηδενικά πολλαπλάσια του {a} που είναι μικρότερα του {b}"),
                (1..b).filter(|x| x % a == 0).collect(),
            )
        }
        3 => {
            let k = r.r(12, 30);
            (
                format!("οι πρώτοι αριθμοί που είναι μικρότεροι του {k}"),
                (2..k).filter(|x| is_prime(*x)).collect(),
            )
        }
        4 => ("οι μονοψήφιοι περιττοί αριθμοί".into(), vec![1, 3, 5, 7, 9]),
        _ => {
            let a = r.r(0, 8);
            let b = a + r.r(3, 7);
            (
                format!("{{x | x ∈ ℕ και {a} &lt; x ≤ {b}}}"),
                (a + 1..=b).filter(|x| *x >= 1).collect(),
            )
        }
    }
}

pub fn set_def(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let (desc, el) = described(r);
            let max = (*el.iter().max().unwrap()).max(20);
            let n = el.len() as u64;
            let el2 = el.clone();
            Question {
                cat: "set-def",
                prompt: format!("Δίνεται το σύνολο <b>Α: {desc}</b>.<br>Γράψε το σύνολο με <b>αναγραφή</b> των στοιχείων του και βρες τον πληθικό του αριθμό."),
                groups: vec![
                    group("α) Διάλεξε όλα τα στοιχεία του Α.", vec![set((0..=max).collect(), el.clone())]),
                    group("β) ν(Α) =", vec![num("", n)]),
                ],
                hints: hints(
                    "Γράψε έναν-έναν τους αριθμούς που έχουν την ιδιότητα της περιγραφής.",
                    "Πρόσεξε τα όρια: «μικρότερος του» σημαίνει ότι ο ίδιος ο αριθμός δεν μετράει. Και ℕ = {1, 2, 3, …}.",
                    format!("Ξεκινά έτσι: {} …", list(&el[..el.len().min(2)])),
                    format!("Α = {}, ν(Α) = {n}", braces_n(&el)),
                ),
                solution: format!("Α = <b>{}</b><br>Το Α έχει {n} στοιχεία, άρα <b>ν(Α) = {n}</b>.", braces_n(&el)),
                diagnose: Box::new(move |v| {
                    let mut out = vec![];
                    if let Some(s) = v[0].set() {
                        if s.len() as u64 != n && v[1].num() == Some(s.len() as i64) {
                            out.push(msg("Ο πληθικός αριθμός συμφωνεί με όσα διάλεξες. Διόρθωσε πρώτα τα στοιχεία."));
                        }
                        if let Some(x) = s.iter().find(|x| !el2.contains(&(**x as u64))) {
                            out.push(dm("set-elem", format!("Το {x} δεν έχει την ιδιότητα της περιγραφής.")));
                        }
                    }
                    out
                }),
            }
        }
        1 => {
            let w = r.pick(&WORDS);
            let ls = letters(w);
            let n = ls.len() as u64;
            Question {
                cat: "set-def",
                prompt: format!("Δίνεται το σύνολο <b>Α: τα γράμματα της λέξης «{w}»</b>.<br>Γράψε το σύνολο με αναγραφή και βρες τον πληθικό του αριθμό."),
                groups: vec![
                    group("α) Διάλεξε όλα τα στοιχεία του Α.", vec![set_labeled(ALPHABET.iter().map(|s| s.to_string()).collect(), ls.clone())]),
                    group("β) ν(Α) =", vec![num("", n)]),
                ],
                hints: hints(
                    "Σε ένα σύνολο κάθε στοιχείο γράφεται <b>μία φορά</b>, ακόμα κι αν εμφανίζεται πολλές φορές.",
                    format!("Η λέξη έχει {} γράμματα, αλλά κάποια επαναλαμβάνονται.", w.chars().count()),
                    format!("Τα διαφορετικά γράμματα είναι {n}."),
                    format!("Α = {}, ν(Α) = {n}", letter_list(&ls)),
                ),
                solution: format!("Α = <b>{}</b>. Κάθε γράμμα γράφεται μία φορά, άρα <b>ν(Α) = {n}</b>.", letter_list(&ls)),
                diagnose: Box::new(move |v| {
                    if v[1].num() == Some(w.chars().count() as i64) {
                        vec![dm("set-dup", "Μέτρησες και τα γράμματα που επαναλαμβάνονται. Σε σύνολο κάθε στοιχείο μετράει <b>μία</b> φορά.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let (desc, el) = described(r);
            let max = (*el.iter().max().unwrap()).max(12);
            let x = if r.chance(0.5) {
                r.pick(&el)
            } else {
                loop {
                    let x = r.r(0, max + 3);
                    if !el.contains(&x) {
                        break x;
                    }
                }
            };
            let inside = el.contains(&x);
            let use_in = r.chance(0.5);
            let stmt = format!("{x} {} Α", if use_in { "∈" } else { "∉" });
            let truth = inside == use_in;
            Question {
                cat: "set-def",
                prompt: format!("Δίνεται το σύνολο <b>Α: {desc}</b>.<br>Σωστό ή λάθος;<br><span class=\"big\" style=\"font-size:1.3rem\">{stmt}</span>"),
                groups: one(vec![truefalse(truth)]),
                hints: hints(
                    "Το «∈» διαβάζεται «ανήκει» και το «∉» «δεν ανήκει».",
                    "Γράψε πρώτα το Α με αναγραφή.",
                    format!("Α = {}", braces_n(&el)),
                    format!("Το {x} {}ανήκει στο Α, άρα η πρόταση είναι {}.", yn(inside), if truth { "σωστή" } else { "λάθος" }),
                ),
                solution: format!("Α = {}. Το {x} {}ανήκει στο Α, άρα η πρόταση «{stmt}» είναι <b>{}</b>.", braces_n(&el), yn(inside), if truth { "σωστή" } else { "λάθος" }),
                diagnose: Box::new(|_| vec![tag("set-elem")]),
            }
        }
    }
}

pub fn set_rel(r: &mut Rng) -> Question {
    if r.chance(0.4) {
        // two words: which relation holds?
        let (a, b, rel) = loop {
            let a = r.pick(&WORDS);
            let b = r.pick(&WORDS);
            if a == b {
                continue;
            }
            let (la, lb) = (letters(a), letters(b));
            let sub_ab = la.iter().all(|x| lb.contains(x));
            let sub_ba = lb.iter().all(|x| la.contains(x));
            let rel = match (sub_ab, sub_ba) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                _ => 3,
            };
            // relations other than "none" are rare: keep them more often
            if rel != 3 || r.chance(0.25) {
                break (a, b, rel);
            }
        };
        let (la, lb) = (letters(a), letters(b));
        let opts: Vec<(i64, String)> = vec![
            (0, "Α = Β".into()),
            (1, "Α ⊆ Β και Α ≠ Β".into()),
            (2, "Β ⊆ Α και Α ≠ Β".into()),
            (3, "Κανένα δεν είναι υποσύνολο του άλλου".into()),
        ];
        let why = match rel {
            0 => "Έχουν ακριβώς τα ίδια στοιχεία, άρα Α = Β.".to_string(),
            1 => "Κάθε στοιχείο του Α είναι και στοιχείο του Β, αλλά το Β έχει και άλλα. Άρα Α ⊆ Β.".to_string(),
            2 => "Κάθε στοιχείο του Β είναι και στοιχείο του Α, αλλά το Α έχει και άλλα. Άρα Β ⊆ Α.".to_string(),
            _ => "Κάθε σύνολο έχει στοιχείο που δεν υπάρχει στο άλλο, άρα κανένα δεν είναι υποσύνολο του άλλου.".to_string(),
        };
        return Question {
            cat: "set-rel",
            prompt: format!("Δίνονται τα σύνολα<br><b>Α: τα γράμματα της λέξης «{a}»</b><br><b>Β: τα γράμματα της λέξης «{b}»</b>.<br>Ποια σχέση ισχύει;"),
            groups: one(vec![select("Σχέση:", opts, rel)]),
            hints: hints(
                "Γράψε πρώτα τα δύο σύνολα με αναγραφή (κάθε γράμμα μία φορά).",
                format!("Α = {}<br>Β = {}", letter_list(&la), letter_list(&lb)),
                "Το Α ⊆ Β σημαίνει ότι <b>κάθε</b> στοιχείο του Α είναι και στοιχείο του Β.",
                why.clone(),
            ),
            solution: format!("Α = {}<br>Β = {}<br>{why}", letter_list(&la), letter_list(&lb)),
            diagnose: Box::new(|_| vec![tag("set-sub")]),
        };
    }
    let pool: Vec<u64> = (1..=12).collect();
    let b: Vec<u64> = {
        let mut v = r.shuffle(&pool)[..r.r(4, 6) as usize].to_vec();
        v.sort_unstable();
        v
    };
    let kind = r.r(0, 2);
    let a: Vec<u64> = match kind {
        0 => {
            let mut v = r.shuffle(&b)[..r.r(2, 3) as usize].to_vec();
            v.sort_unstable();
            v
        }
        1 => b.clone(),
        _ => loop {
            let mut v = r.shuffle(&pool)[..r.r(3, 4) as usize].to_vec();
            v.sort_unstable();
            if v.iter().any(|x| !b.contains(x)) && v.iter().any(|x| b.contains(x)) {
                break v;
            }
        },
    };
    let shown_a = r.shuffle(&a);
    let shown_b = r.shuffle(&b);
    let sub = |x: &[u64], y: &[u64]| x.iter().all(|e| y.contains(e));
    let st = r.r(0, 4);
    let (stmt, truth, why) = match st {
        0 => (
            "Α ⊆ Β".to_string(),
            sub(&a, &b),
            if sub(&a, &b) {
                "Κάθε στοιχείο του Α ανήκει και στο Β.".to_string()
            } else {
                format!(
                    "Το {} ανήκει στο Α αλλά όχι στο Β.",
                    a.iter().find(|x| !b.contains(x)).unwrap()
                )
            },
        ),
        1 => (
            "Β ⊆ Α".to_string(),
            sub(&b, &a),
            if sub(&b, &a) {
                "Κάθε στοιχείο του Β ανήκει και στο Α.".to_string()
            } else {
                format!(
                    "Το {} ανήκει στο Β αλλά όχι στο Α.",
                    b.iter().find(|x| !a.contains(x)).unwrap()
                )
            },
        ),
        2 => (
            "Α = Β".to_string(),
            a == b,
            if a == b {
                "Έχουν ακριβώς τα ίδια στοιχεία (η σειρά δεν μετράει).".to_string()
            } else {
                "Δεν έχουν ακριβώς τα ίδια στοιχεία.".to_string()
            },
        ),
        3 => (
            "∅ ⊆ Α".to_string(),
            true,
            "Το κενό σύνολο είναι υποσύνολο κάθε συνόλου.".to_string(),
        ),
        _ => (
            format!(
                "ν(Α) {} ν(Β)",
                if a.len() < b.len() {
                    "&lt;"
                } else if a.len() == b.len() {
                    "="
                } else {
                    "&gt;"
                }
            ),
            true,
            format!("ν(Α) = {} και ν(Β) = {}.", a.len(), b.len()),
        ),
    };
    Question {
        cat: "set-rel",
        prompt: format!("Δίνονται τα σύνολα <b>Α = {}</b> και <b>Β = {}</b>.<br>Σωστό ή λάθος;<br><span class=\"big\" style=\"font-size:1.3rem\">{stmt}</span>", braces_n(&shown_a), braces_n(&shown_b)),
        groups: one(vec![truefalse(truth)]),
        hints: hints(
            "Το Α ⊆ Β σημαίνει ότι κάθε στοιχείο του Α είναι και στοιχείο του Β.",
            "Δύο σύνολα είναι ίσα όταν έχουν ακριβώς τα ίδια στοιχεία. Η σειρά με την οποία τα γράφουμε δεν μετράει.",
            "Έλεγξε ένα-ένα τα στοιχεία.",
            format!("{why} Άρα η πρόταση είναι {}.", if truth { "σωστή" } else { "λάθος" }),
        ),
        solution: format!("{why} Η πρόταση «{stmt}» είναι <b>{}</b>.", if truth { "σωστή" } else { "λάθος" }),
        diagnose: Box::new(|_| vec![tag("set-sub")]),
    }
}

pub fn set_ops(r: &mut Rng) -> Question {
    if r.chance(0.3) {
        // counting problem with ν(Α ∪ Β) = ν(Α) + ν(Β) − ν(Α ∩ Β)
        let t = r.pick(&[
            ("Σε ένα τμήμα {t} μαθητών, {a} παίζουν ποδόσφαιρο, {b} παίζουν μπάσκετ και {c} παίζουν και τα δύο.", "παίζουν τουλάχιστον ένα από τα δύο αθλήματα", "δεν παίζουν κανένα από τα δύο"),
            ("Από τους {t} μαθητές μιας τάξης, {a} πήγαν στη χορωδία, {b} στο θεατρικό και {c} και στα δύο.", "συμμετέχουν σε τουλάχιστον ένα όμιλο", "δεν συμμετέχουν σε κανένα από τους δύο ομίλους"),
            ("Σε μια έρευνα με {t} άτομα, {a} έχουν σκύλο, {b} έχουν γάτα και {c} έχουν και τα δύο.", "έχουν τουλάχιστον ένα από τα δύο ζώα", "δεν έχουν ούτε σκύλο ούτε γάτα"),
        ]);
        let c = r.r(2, 8);
        let a = c + r.r(4, 12);
        let b = c + r.r(4, 12);
        let u = a + b - c;
        let tot = u + r.r(1, 8);
        let p =
            t.0.replace("{t}", &tot.to_string())
                .replace("{a}", &a.to_string())
                .replace("{b}", &b.to_string())
                .replace("{c}", &c.to_string());
        return Question {
            cat: "set-ops",
            prompt: p,
            groups: vec![
                group(format!("α) Πόσοι {};", t.1), vec![num("ν(Α ∪ Β) =", u)]),
                group(format!("β) Πόσοι {};", t.2), vec![num("", tot - u)]),
            ],
            hints: hints(
                "Ζωγράφισε βέννειο διάγραμμα: δύο κύκλοι που τέμνονται. Στην τομή βάζεις όσους κάνουν και τα δύο.",
                format!("Μόνο το πρώτο: {a} − {c} = {}. Μόνο το δεύτερο: {b} − {c} = {}.", a - c, b - c),
                format!("ν(Α ∪ Β) = ν(Α) + ν(Β) − ν(Α ∩ Β) = {a} + {b} − {c}"),
                format!("ν(Α ∪ Β) = {u}, και {tot} − {u} = {}", tot - u),
            ),
            solution: format!(
                "Όσοι κάνουν και τα δύο μετρήθηκαν δύο φορές, άρα:<br>ν(Α ∪ Β) = {a} + {b} − {c} = <b>{u}</b>.<br>Κανένα από τα δύο: {tot} − {u} = <b>{}</b>.",
                tot - u
            ),
            diagnose: Box::new(move |v| {
                if v[0].num() == Some((a + b) as i64) {
                    vec![dm("set-double", format!("Πρόσθεσες {a} + {b}, αλλά οι {c} που κάνουν και τα δύο μετρήθηκαν <b>δύο φορές</b>. Αφαίρεσέ τους μία φορά."))]
                } else {
                    vec![]
                }
            }),
        };
    }
    let n = r.r(8, 12);
    let omega: Vec<u64> = (1..=n).collect();
    let pick_sub = |r: &mut Rng| {
        let mut v = r.shuffle(&omega)[..r.r(3, 6) as usize].to_vec();
        v.sort_unstable();
        v
    };
    let a = pick_sub(r);
    let b = pick_sub(r);
    let inter: Vec<u64> = a.iter().copied().filter(|x| b.contains(x)).collect();
    let uni: Vec<u64> = omega
        .iter()
        .copied()
        .filter(|x| a.contains(x) || b.contains(x))
        .collect();
    let comp = |s: &[u64]| {
        omega
            .iter()
            .copied()
            .filter(|x| !s.contains(x))
            .collect::<Vec<u64>>()
    };
    let (expr, res, how): (&str, Vec<u64>, String) = match r.r(0, 5) {
        0 => (
            "Α ∪ Β",
            uni.clone(),
            "Η ένωση έχει όλα τα στοιχεία που ανήκουν <b>σε ένα τουλάχιστον</b> από τα δύο σύνολα."
                .into(),
        ),
        1 => (
            "Α ∩ Β",
            inter.clone(),
            "Η τομή έχει μόνο τα <b>κοινά</b> στοιχεία.".into(),
        ),
        2 => (
            "Α′",
            comp(&a),
            "Το συμπλήρωμα Α′ έχει τα στοιχεία του Ω που <b>δεν</b> ανήκουν στο Α.".into(),
        ),
        3 => (
            "(Α ∪ Β)′",
            comp(&uni),
            format!(
                "Πρώτα Α ∪ Β = {}. Μετά παίρνουμε όσα στοιχεία του Ω λείπουν.",
                braces_n(&uni)
            ),
        ),
        4 => (
            "Α ∩ Β′",
            a.iter().copied().filter(|x| !b.contains(x)).collect(),
            format!(
                "Β′ = {}. Η τομή του με το Α έχει τα στοιχεία του Α που δεν είναι στο Β.",
                braces_n(&comp(&b))
            ),
        ),
        _ => (
            "Α′ ∩ Β",
            b.iter().copied().filter(|x| !a.contains(x)).collect(),
            format!(
                "Α′ = {}. Η τομή του με το Β έχει τα στοιχεία του Β που δεν είναι στο Α.",
                braces_n(&comp(&a))
            ),
        ),
    };
    // options: the elements of Ω and the empty set
    let mut labels: Vec<String> = omega.iter().map(|x| x.to_string()).collect();
    labels.push("∅".into());
    let ans: Vec<usize> = if res.is_empty() {
        vec![omega.len()]
    } else {
        res.iter().map(|x| (*x - 1) as usize).collect()
    };
    let (res2, a2, b2, uni2, inter2) = (
        res.clone(),
        a.clone(),
        b.clone(),
        uni.clone(),
        inter.clone(),
    );
    let empty_idx = omega.len() as i64;
    Question {
        cat: "set-ops",
        prompt: format!(
            "Δίνονται τα σύνολα <b>Ω = {}</b>, <b>Α = {}</b> και <b>Β = {}</b>.<br>Βρες το σύνολο <span class=\"big\" style=\"font-size:1.3rem\">{expr}</span>.<br><span class=\"muted\">Διάλεξε τα στοιχεία του. Αν είναι κενό, διάλεξε μόνο το ∅.</span>",
            braces_n(&omega),
            braces_n(&a),
            braces_n(&b)
        ),
        groups: one(vec![set_labeled(labels, ans)]),
        hints: hints(
            "Ένωση ∪: στοιχεία που ανήκουν σε ένα τουλάχιστον από τα σύνολα. Τομή ∩: τα κοινά στοιχεία. Συμπλήρωμα ′: ό,τι λείπει από το Ω.",
            "Ζωγράφισε βέννειο διάγραμμα με το Ω ως ορθογώνιο και τα Α, Β ως κύκλους.",
            how.clone(),
            format!("{expr} = {}", braces_n(&res)),
        ),
        solution: format!("{how}<br><b>{expr} = {}</b>", braces_n(&res)),
        diagnose: Box::new(move |v| {
            let s: Vec<u64> = v[0].set().map(|s| s.iter().filter(|x| **x != empty_idx).map(|x| (*x + 1) as u64).collect()).unwrap_or_default();
            let has_empty = v[0].set().is_some_and(|s| s.contains(&empty_idx));
            let mut out = vec![];
            if has_empty && !s.is_empty() {
                out.push(dm("set-empty", "Το ∅ σημαίνει «κανένα στοιχείο». Δεν μπαίνει μαζί με άλλα στοιχεία."));
            }
            if expr == "Α ∩ Β" && s == uni2 && uni2 != res2 {
                out.push(dm("set-union-inter", "Έγραψες την ένωση. Η τομή έχει μόνο τα <b>κοινά</b> στοιχεία."));
            } else if expr == "Α ∪ Β" && s == inter2 && uni2 != inter2 {
                out.push(dm("set-union-inter", "Έγραψες την τομή. Η ένωση έχει <b>όλα</b> τα στοιχεία και των δύο συνόλων."));
            } else if expr == "Α′" && s == a2 {
                out.push(msg("Αυτό είναι το ίδιο το Α. Το συμπλήρωμα έχει τα στοιχεία του Ω που <b>λείπουν</b> από το Α."));
            }
            let _ = &b2;
            out
        }),
    }
}
