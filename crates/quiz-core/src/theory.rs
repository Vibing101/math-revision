//! The spoken form of the theory pages (`public/theory/<chapter>.json`).
//!
//! The Worker never speaks text sent by a browser: the browser names a piece of a theory page
//! with a *key* and this module says what is read aloud for it. The audio is stored under a hash
//! of the voice and that exact text, so a sentence is synthesised once, and only when its words
//! change is it synthesised again.
//!
//! Keys (`s` = section, `c` = card, both counted from 0, `r` = fading rung, `n` = step):
//!
//! | key          | spoken                                                   |
//! |--------------|----------------------------------------------------------|
//! | `s:c`        | the card: its label, title and text                      |
//! | `s:c:n`      | step `n` of an example card                              |
//! | `s:c:rR`     | the statement of rung `R` of the card's «Τώρα δοκίμασε εσύ» |
//! | `s:c:rR:n`   | step `n` of that rung                                    |
//! | `s:x`        | the section's «Σκέψου πρώτα» situation                   |
//! | `s:x:qN`     | its question `N`                                         |
//! | `s:x:aN`     | the answer to question `N`                               |
//! | `s:x:b`      | the sentence that links it to the mathematics            |

use crate::lesson::{sentences, speak};
use serde_json::Value;

const LABELS: &[(&str, &str)] = &[
    ("goal", "Τι θα μάθουμε"),
    ("def", "Ορισμός"),
    ("rule", "Κανόνας"),
    ("note", "Να θυμάσαι"),
    ("warn", "Πρόσεξε"),
    ("example", "Παράδειγμα"),
];

/// Card HTML as sentences a Greek voice reads well.
fn html_parts(html: &str) -> Vec<String> {
    let h = html
        .replace("</p>", ". ")
        .replace("</li>", ". ")
        .replace("<span class=\"vec\">", " διάνυσμα ")
        .replace("Π.χ.", "Για παράδειγμα,")
        .replace("π.χ.", "για παράδειγμα,");
    sentences(&speak(&h))
}

fn step_parts(steps: &Value, n: usize) -> Option<Vec<String>> {
    let step = steps.get(n)?.as_str()?;
    let mut out = vec![format!("Βήμα {}.", n + 1)];
    out.extend(html_parts(step));
    Some(out)
}

/// The «Σκέψου πρώτα» block of a section: a situation, questions with answers, a link to the maths.
fn explore_parts(ex: &Value, rest: &[&str]) -> Option<Vec<String>> {
    let out = match rest {
        [] => {
            let mut v = vec!["Σκέψου πρώτα.".to_string()];
            v.extend(html_parts(ex.get("html")?.as_str()?));
            v
        }
        ["b"] => html_parts(ex.get("bridge")?.as_str()?),
        [t] if t.starts_with('q') || t.starts_with('a') => {
            let n: usize = t[1..].parse().ok()?;
            let item = ex.get("qs")?.get(n)?;
            let (head, field) = if t.starts_with('q') {
                (format!("Ερώτηση {}.", n + 1), "q")
            } else {
                ("Μια απάντηση.".to_string(), "a")
            };
            let mut v = vec![head];
            v.extend(html_parts(item.get(field)?.as_str()?));
            v
        }
        _ => return None,
    };
    let out: Vec<String> = out.into_iter().filter(|p| !p.trim().is_empty()).collect();
    (!out.is_empty()).then_some(out)
}

/// The sentences read aloud for `key` of a theory page given as JSON text.
pub fn parts(theory_json: &str, key: &str) -> Option<Vec<String>> {
    let theory: Value = serde_json::from_str(theory_json).ok()?;
    let mut it = key.split(':');
    let s: usize = it.next()?.parse().ok()?;
    let second = it.next()?;
    if second == "x" {
        let rest: Vec<&str> = it.collect();
        return explore_parts(theory.get("sections")?.get(s)?.get("explore")?, &rest);
    }
    let c: usize = second.parse().ok()?;
    let card = theory.get("sections")?.get(s)?.get("cards")?.get(c)?;
    let rest: Vec<&str> = it.collect();
    let out = match rest.as_slice() {
        [] => {
            let ty = card.get("type")?.as_str()?;
            let label = LABELS.iter().find(|(t, _)| *t == ty)?.1;
            let head = match card.get("title").and_then(Value::as_str) {
                Some(t) => format!(
                    "{label}: {}.",
                    html_parts(t).join(" ").trim_end_matches('.')
                ),
                None => format!("{label}."),
            };
            let mut v = vec![head];
            v.extend(html_parts(card.get("html")?.as_str()?));
            v
        }
        [n] if !n.starts_with('r') => step_parts(card.get("steps")?, n.parse().ok()?)?,
        [r] | [r, _] if r.starts_with('r') => {
            let rung = card.get("fade")?.get(r[1..].parse::<usize>().ok()?)?;
            match rest.get(1) {
                None => {
                    let k: usize = r[1..].parse().ok()?;
                    let mut v = vec![format!("Δοκίμασε εσύ, {}.", k + 1)];
                    v.extend(html_parts(rung.get("html")?.as_str()?));
                    v
                }
                Some(n) => step_parts(rung.get("steps")?, n.parse().ok()?)?,
            }
        }
        _ => return None,
    };
    let out: Vec<String> = out.into_iter().filter(|p| !p.trim().is_empty()).collect();
    (!out.is_empty()).then_some(out)
}

/// Every key a page has, in reading order (to warm the audio cache and to test).
pub fn keys(theory_json: &str) -> Vec<String> {
    let Ok(theory) = serde_json::from_str::<Value>(theory_json) else {
        return vec![];
    };
    let mut out = vec![];
    let sections = theory.get("sections").and_then(Value::as_array);
    for (s, sec) in sections.into_iter().flatten().enumerate() {
        if let Some(ex) = sec.get("explore") {
            out.push(format!("{s}:x"));
            let n = ex.get("qs").and_then(Value::as_array).map_or(0, Vec::len);
            for q in 0..n {
                out.push(format!("{s}:x:q{q}"));
                out.push(format!("{s}:x:a{q}"));
            }
            if ex.get("bridge").is_some() {
                out.push(format!("{s}:x:b"));
            }
        }
        let cards = sec.get("cards").and_then(Value::as_array);
        for (c, card) in cards.into_iter().flatten().enumerate() {
            out.push(format!("{s}:{c}"));
            let n_steps = card
                .get("steps")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            out.extend((0..n_steps).map(|n| format!("{s}:{c}:{n}")));
            let rungs = card.get("fade").and_then(Value::as_array);
            for (r, rung) in rungs.into_iter().flatten().enumerate() {
                out.push(format!("{s}:{c}:r{r}"));
                let n = rung
                    .get("steps")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len);
                out.extend((0..n).map(|n| format!("{s}:{c}:r{r}:{n}")));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{"sections":[{"title":"T","explore":{"html":"<p>Κατάσταση.</p>","qs":[{"q":"<p>Ερώτηση;</p>","a":"<p>Απάντηση.</p>"}],"bridge":"<p>Άρα σύνολο.</p>"},"cards":[
      {"type":"def","title":"Σύνολο","html":"<p>Το <b>σύνολο</b> έχει ν(Α) = 5. Π.χ. Α ∪ Β.</p>"},
      {"type":"example","html":"<p>Βρες το x² − 4.</p>","steps":["Γράφουμε x² − 4 = 0.","Άρα x = 2."],
       "fade":[{"html":"<p>Βρες το x² − 9.</p>","steps":["α","β","γ"]}]}]}]}"#;

    #[test]
    fn card_text_has_label_and_spoken_math() {
        let p = parts(SAMPLE, "0:0").unwrap();
        assert_eq!(p[0], "Ορισμός: Σύνολο.");
        let all = p.join(" ");
        assert!(all.contains("ίσον"), "{all}");
        assert!(all.contains("ένωση"), "{all}");
        assert!(all.contains("Για παράδειγμα"), "{all}");
        assert!(!all.contains('<'));
    }

    #[test]
    fn steps_and_rungs() {
        assert_eq!(parts(SAMPLE, "0:1:0").unwrap()[0], "Βήμα 1.");
        assert!(parts(SAMPLE, "0:1:1").unwrap().join(" ").contains("χι"));
        assert_eq!(parts(SAMPLE, "0:1:r0").unwrap()[0], "Δοκίμασε εσύ, 1.");
        assert_eq!(parts(SAMPLE, "0:1:r0:2").unwrap(), vec!["Βήμα 3.", "γ"]);
    }

    #[test]
    fn explore_block_is_spoken() {
        assert_eq!(parts(SAMPLE, "0:x").unwrap()[0], "Σκέψου πρώτα.");
        assert_eq!(parts(SAMPLE, "0:x:q0").unwrap()[0], "Ερώτηση 1.");
        assert_eq!(parts(SAMPLE, "0:x:a0").unwrap()[0], "Μια απάντηση.");
        assert_eq!(parts(SAMPLE, "0:x:b").unwrap(), vec!["Άρα σύνολο."]);
        for k in ["0:x:q1", "0:x:a9", "0:x:z", "0:x:q", "1:x", "0:x:b:1"] {
            assert!(parts(SAMPLE, k).is_none(), "{k}");
        }
    }

    #[test]
    fn unknown_keys_are_refused() {
        for k in [
            "",
            "x",
            "0",
            "9:0",
            "0:9",
            "0:0:0",
            "0:1:5",
            "0:1:r1",
            "0:1:r0:7",
            "0:1:x",
            "0:1:r0:1:1",
            "-1:0",
        ] {
            assert!(parts(SAMPLE, k).is_none(), "{k}");
        }
    }

    #[test]
    fn keys_list_matches_parts() {
        let ks = keys(SAMPLE);
        assert_eq!(
            ks,
            [
                "0:x", "0:x:q0", "0:x:a0", "0:x:b", "0:0", "0:1", "0:1:0", "0:1:1", "0:1:r0",
                "0:1:r0:0", "0:1:r0:1", "0:1:r0:2"
            ]
        );
        assert!(ks.iter().all(|k| parts(SAMPLE, k).is_some()));
    }

    /// Every card, step and rung of every real theory page can be spoken.
    #[test]
    fn all_theory_pages_can_be_spoken() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../public/theory");
        let mut n = 0;
        for f in std::fs::read_dir(dir).unwrap() {
            let f = f.unwrap().path();
            if f.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let text = std::fs::read_to_string(&f).unwrap();
            for k in keys(&text) {
                let p = parts(&text, &k).unwrap_or_else(|| panic!("{}: {k}", f.display()));
                for s in &p {
                    assert!(
                        !s.contains('<') && !s.contains('>'),
                        "{}: {k}: {s}",
                        f.display()
                    );
                    assert!(
                        s.chars().count() < 600,
                        "{}: {k}: sentence too long",
                        f.display()
                    );
                }
                n += 1;
            }
        }
        assert!(n > 500, "only {n} keys found");
    }
}
