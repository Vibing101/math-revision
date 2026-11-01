//! Question engine for the «Διαιρετότητα» practice app (Μαθηματικά Α΄ Γυμνασίου).
//!
//! Every question is rebuilt from `(category, seed)`, so the server can check
//! answers without storing anything and without sending answers to the browser.

pub mod gen_basic;
pub mod gen_mkd;
pub mod mk;
pub mod model;
pub mod util;

pub use model::{answer_json, answer_text, check, public, CheckResult, PublicQuestion, Question};
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Category {
    pub id: &'static str,
    /// Chapter (book unit) this category belongs to.
    pub chapter: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    /// Book section this practises.
    pub section: &'static str,
    /// Known weak spot from the class test: offered with a highlight and more often.
    pub focus: bool,
}

#[derive(Serialize, Clone, Copy)]
pub struct Chapter {
    pub id: &'static str,
    /// Number of the unit in the book.
    pub number: u32,
    pub title: &'static str,
    pub desc: &'static str,
    /// The unit's sections, as the book lists them.
    pub sections: &'static [&'static str],
}

/// Book units, in book order. A new unit needs an entry here and its categories below.
pub const CHAPTERS: &[Chapter] = &[Chapter {
    id: "diairetotita",
    number: 1,
    title: "Διαιρετότητα",
    desc: "Ευκλείδεια διαίρεση, κριτήρια, πρώτοι αριθμοί, ΜΚΔ και ΕΚΠ",
    sections: &[
        "Ευκλείδεια Διαίρεση",
        "Ιδιότητες Διαιρετότητας",
        "Κριτήρια Διαιρετότητας",
        "Πρώτοι και Σύνθετοι Αριθμοί",
        "ΜΚΔ και ΕΚΠ",
        "Δραστηριότητες Ενότητας και Εμπλουτισμού",
    ],
}];

pub fn chapter_categories(chapter: &str) -> Vec<&'static str> {
    CATEGORIES
        .iter()
        .filter(|c| c.chapter == chapter)
        .map(|c| c.id)
        .collect()
}

pub const CATEGORIES: &[Category] = &[
    Category {
        id: "eucl",
        chapter: "diairetotita",
        title: "Ευκλείδεια διαίρεση",
        desc: "Δ = δ·π + υ, μέρες και ομάδες",
        section: "Ευκλείδεια Διαίρεση",
        focus: false,
    },
    Category {
        id: "div",
        chapter: "diairetotita",
        title: "Διαιρέτες και πολλαπλάσια",
        desc: "α | β, διαιρέτες ενός αριθμού",
        section: "Ευκλείδεια Διαίρεση",
        focus: false,
    },
    Category {
        id: "prop",
        chapter: "diairetotita",
        title: "Ιδιότητες διαιρετότητας",
        desc: "Άθροισμα, διαφορά, μεταβατική",
        section: "Ιδιότητες Διαιρετότητας",
        focus: false,
    },
    Category {
        id: "crit",
        chapter: "diairetotita",
        title: "Κριτήρια διαιρετότητας",
        desc: "Διαιρείται; Ποιο ψηφίο λείπει;",
        section: "Κριτήρια Διαιρετότητας",
        focus: false,
    },
    Category {
        id: "prime",
        chapter: "diairetotita",
        title: "Πρώτοι και σύνθετοι",
        desc: "Πρώτος ή όχι; Πρώτοι μεταξύ τους;",
        section: "Πρώτοι και Σύνθετοι Αριθμοί",
        focus: false,
    },
    Category {
        id: "fact",
        chapter: "diairetotita",
        title: "Πρώτοι παράγοντες",
        desc: "Ανάλυση με τη «σκάλα»",
        section: "Ανάλυση σε γινόμενο πρώτων παραγόντων",
        focus: false,
    },
    Category {
        id: "pow",
        chapter: "diairetotita",
        title: "ΜΚΔ & ΕΚΠ με δυνάμεις",
        desc: "Μικρότερος ή μεγαλύτερος εκθέτης;",
        section: "ΜΚΔ και ΕΚΠ",
        focus: true,
    },
    Category {
        id: "num",
        chapter: "diairetotita",
        title: "ΜΚΔ & ΕΚΠ αριθμών",
        desc: "Από την ανάλυση στο αποτέλεσμα",
        section: "ΜΚΔ και ΕΚΠ",
        focus: false,
    },
    Category {
        id: "gcdp",
        chapter: "diairetotita",
        title: "Προβλήματα ΜΚΔ",
        desc: "Ομάδες, δέματα, κομμάτια",
        section: "ΜΚΔ και ΕΚΠ",
        focus: false,
    },
    Category {
        id: "time",
        chapter: "diairetotita",
        title: "Προβλήματα ΕΚΠ: ώρες & μέρες",
        desc: "Τρένα, πλοία, δορυφόροι",
        section: "ΜΚΔ και ΕΚΠ",
        focus: true,
    },
    Category {
        id: "years",
        chapter: "diairetotita",
        title: "Προβλήματα ΕΚΠ: χρονιές",
        desc: "Κομήτες και επέτειοι",
        section: "ΜΚΔ και ΕΚΠ",
        focus: false,
    },
    Category {
        id: "chal",
        chapter: "diairetotita",
        title: "Προκλήσεις",
        desc: "Αριθμός ανάμεσα σε δύο όρια",
        section: "Δραστηριότητες Ενότητας",
        focus: false,
    },
    Category {
        id: "enrich",
        chapter: "diairetotita",
        title: "Εμπλουτισμός: κριτήρια 7, 11, 19",
        desc: "Κριτήρια πέρα από το βασικό βιβλίο",
        section: "Δραστηριότητες Εμπλουτισμού",
        focus: false,
    },
];

/// Teacher-facing labels for the mistake tags returned by `check`.
pub const TAGS: &[(&str, &str)] = &[
    (
        "lcm-min",
        "ΕΚΠ: πήρε τον μικρότερο εκθέτη αντί για τον μεγαλύτερο",
    ),
    (
        "lcm-missing",
        "ΕΚΠ: ξέχασε πρώτο παράγοντα που υπάρχει μόνο σε έναν αριθμό",
    ),
    (
        "gcd-noncommon",
        "ΜΚΔ: έβαλε πρώτο παράγοντα που δεν είναι κοινός",
    ),
    (
        "gcd-max",
        "ΜΚΔ: πήρε τον μεγαλύτερο εκθέτη αντί για τον μικρότερο",
    ),
    (
        "pow-mult",
        "Υπολόγισε δύναμη σαν πολλαπλασιασμό (π.χ. 5² = 10)",
    ),
    ("not-least", "Βρήκε κοινό πολλαπλάσιο αλλά όχι το ελάχιστο"),
    ("not-greatest", "Βρήκε κοινό διαιρέτη αλλά όχι τον μέγιστο"),
    ("gcd-vs-lcm", "Μπέρδεψε ΜΚΔ με ΕΚΠ"),
    (
        "time-remainder",
        "Ώρες σε μέρες: ξέχασε τις ώρες που περισσεύουν",
    ),
    ("time-12", "Μπέρδεψε π.μ. με μ.μ."),
    ("time-day", "Λάθος στη μέτρηση των ημερών"),
    (
        "blank",
        "Πήγε να κάνει έλεγχο χωρίς να απαντήσει σε όλα τα ερωτήματα",
    ),
    ("eucl-rem", "Υπόλοιπο ίσο ή μεγαλύτερο από τον διαιρέτη"),
    (
        "eucl-week",
        "Μέρες της εβδομάδας: ξέχασε τις μέρες που περισσεύουν",
    ),
    ("ceil", "«Πόσα χρειάζονται τουλάχιστον»: ξέχασε το υπόλοιπο"),
    (
        "div-trivial",
        "Διαιρέτες: ξέχασε το 1 ή τον ίδιο τον αριθμό",
    ),
    (
        "div-wrong",
        "Διαιρέτες: διάλεξε αριθμό που δεν είναι διαιρέτης",
    ),
    ("div-missing", "Διαιρέτες: δεν βρήκε όλους τους διαιρέτες"),
    ("div-tf", "Μπέρδεψε «διαιρέτης» με «πολλαπλάσιο»"),
    ("prop", "Ιδιότητες διαιρετότητας"),
    (
        "fact-product",
        "Ανάλυση σε παράγοντες που δεν δίνει τον αρχικό αριθμό",
    ),
    (
        "digits-partial",
        "Συμπλήρωση ψηφίου: δεν βρήκε όλες τις λύσεις",
    ),
    ("crit", "Λάθος σε κριτήριο διαιρετότητας"),
    ("prime-one", "Θεώρησε το 1 πρώτο αριθμό"),
    (
        "prime-comp",
        "Θεώρησε σύνθετο αριθμό πρώτο (ή το αντίστροφο)",
    ),
    ("prime-missed", "Δεν αναγνώρισε κάποιον πρώτο"),
    ("coprime", "Πρώτοι μεταξύ τους"),
    ("enrich", "Κριτήρια εμπλουτισμού (7, 11, 19)"),
    ("range-lcm", "Πρόβλημα με όρια: σταμάτησε στο ΕΚΠ"),
    (
        "range-rem",
        "Πρόβλημα με υπόλοιπο: ξέχασε να προσθέσει το υπόλοιπο",
    ),
    ("years-count", "Μέτρηση φορών: στρογγύλεψε προς τα πάνω"),
];

pub fn generate(cat: &str, seed: u32) -> Option<Question> {
    let mut r = util::Rng::new(
        seed as u64
            ^ ((cat.len() as u64) << 40)
            ^ cat
                .bytes()
                .fold(0u64, |a, b| a.wrapping_mul(131).wrapping_add(b as u64))
                << 8,
    );
    let q = match cat {
        "eucl" => gen_basic::eucl(&mut r),
        "div" => gen_basic::div(&mut r),
        "prop" => gen_basic::prop(&mut r),
        "crit" => gen_basic::crit(&mut r),
        "prime" => gen_basic::prime(&mut r),
        "enrich" => gen_basic::enrich(&mut r),
        "fact" => gen_mkd::fact(&mut r),
        "pow" => gen_mkd::pow(&mut r),
        "num" => gen_mkd::num_q(&mut r),
        "gcdp" => gen_mkd::gcdp(&mut r),
        "time" => gen_mkd::time(&mut r),
        "years" => gen_mkd::years(&mut r),
        "chal" => gen_mkd::chal(&mut r),
        _ => return None,
    };
    Some(q)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn correct_values(q: &Question) -> Vec<Value> {
        q.parts().map(|p| answer_json(&p.answer)).collect()
    }

    #[test]
    fn chapters_and_categories_match() {
        for c in CATEGORIES {
            assert!(
                CHAPTERS.iter().any(|ch| ch.id == c.chapter),
                "{}: unknown chapter",
                c.id
            );
            assert!(generate(c.id, 1).is_some(), "{}: no generator", c.id);
        }
        for ch in CHAPTERS {
            assert!(
                !chapter_categories(ch.id).is_empty(),
                "{}: empty chapter",
                ch.id
            );
        }
    }

    #[test]
    fn every_generator_is_consistent() {
        for c in CATEGORIES {
            for seed in 0..1500u32 {
                let q = generate(c.id, seed).unwrap();
                assert_eq!(q.cat, c.id);
                assert!(q.parts().count() > 0);
                assert!(
                    q.hints.iter().all(|h| !h.is_empty()),
                    "{} {seed}: empty hint",
                    c.id
                );
                // correct answers pass
                let res = check(&q, &correct_values(&q), false);
                assert!(res.correct, "{} seed {seed}: own answer rejected", c.id);
                assert!(res.solution.is_some());
                // deterministic
                let again = generate(c.id, seed).unwrap();
                assert_eq!(q.prompt, again.prompt);
                // wrong answers never panic and never pass
                let junk: Vec<Value> = q.parts().map(|_| Value::from(7)).collect();
                let _ = check(&q, &junk, true);
                let blanks: Vec<Value> = q.parts().map(|_| Value::Null).collect();
                let b = check(&q, &blanks, false);
                assert!(!b.correct && b.solution.is_none());
                // set/select options contain the answer
                for p in q.parts() {
                    match (&p.kind, &p.answer) {
                        (model::Kind::Set { options }, model::Answer::Set(a)) => {
                            assert!(a.iter().all(|x| options.contains(x)))
                        }
                        (model::Kind::Select { options }, model::Answer::Choice(a)) => {
                            assert!(options.iter().any(|(v, _)| v == a))
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[test]
    fn known_mistakes_are_diagnosed() {
        // find a pow question and give the classic "min exponent in LCM" answer
        let mut hit = false;
        for seed in 0..200 {
            let q = generate("pow", seed).unwrap();
            let parts: Vec<_> = q.parts().collect();
            let (model::Answer::Pow(g), model::Answer::Pow(_)) =
                (&parts[0].answer, &parts[2].answer)
            else {
                panic!()
            };
            let mut vals = correct_values(&q);
            vals[2] = answer_json(&model::Answer::Pow(g.clone()));
            let res = check(&q, &vals, false);
            if res.diag.iter().any(|d| d.tag == Some("lcm-min")) {
                hit = true;
                break;
            }
        }
        assert!(hit, "lcm-min never diagnosed");

        let mut hit = false;
        for seed in 0..300 {
            let q = generate("time", seed).unwrap();
            if !q.prompt.contains("ώρες") {
                continue;
            }
            let parts: Vec<_> = q.parts().collect();
            let n = parts.len();
            let mut vals = correct_values(&q);
            // keep the start hour: pupil forgot the leftover hours
            let start: i64 = (6..=11)
                .find(|h| q.prompt.contains(&format!("<b>{h}:00 π.μ.")))
                .unwrap();
            let model::Answer::Num(l) = parts[0].answer else {
                panic!()
            };
            if l % 24 == 0 {
                continue;
            }
            let start_day = (0..7)
                .find(|d| q.prompt.contains(&format!("{} στις", util::DAYS[*d])))
                .unwrap() as i64;
            vals[n - 2] = Value::from((start_day + l / 24) % 7);
            vals[n - 1] = Value::from(start);
            let res = check(&q, &vals, false);
            if res.diag.iter().any(|d| d.tag == Some("time-remainder")) {
                hit = true;
                break;
            }
        }
        assert!(hit, "time-remainder never diagnosed");
    }
}
