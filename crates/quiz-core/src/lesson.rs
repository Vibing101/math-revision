//! Short narrated "videos" (step-by-step lessons) for pupils who need instructions
//! broken into small pieces (e.g. ADHD).
//!
//! * `instructions`: what the exercise asks and how to fill in each answer box.
//!   Built for every category from the question itself.
//! * `method`: the solving method, shown on a *similar* example with other numbers,
//!   so it never gives away the pupil's own answer. Available for `METHOD_CATS`.
//!
//! Every step has a short title (shown as a checklist), `show` (HTML on screen) and
//! `say` (plain text for the speech synthesiser and for the subtitles).

use crate::model::{Kind, Question};
use crate::util::*;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Step {
    pub title: String,
    pub say: String,
    pub show: String,
    /// `say` split into sentences: the player speaks (and highlights) one at a time.
    pub parts: Vec<String>,
}

/// Split spoken text into sentences, keeping the closing punctuation.
pub fn sentences(t: &str) -> Vec<String> {
    let mut out = vec![];
    let mut cur = String::new();
    let mut ending = false;
    for c in t.chars() {
        let end = matches!(c, '.' | '!' | ';' | '?');
        if ending && !end {
            if !cur.trim().is_empty() {
                out.push(cur.trim().to_string());
            }
            cur.clear();
        }
        ending = end;
        cur.push(c);
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

#[derive(Serialize)]
pub struct Lesson {
    pub title: String,
    pub steps: Vec<Step>,
}

fn st(title: &str, show: impl Into<String>, say: impl Into<String>) -> Step {
    let say: String = say.into();
    Step {
        title: title.into(),
        show: show.into(),
        parts: sentences(&say),
        say,
    }
}

/// Categories with a method lesson (pilot).
pub const METHOD_CATS: &[&str] = &["num", "c-factor", "c-eq2"];

// ---------------- speech ----------------

/// Remove every `<tag …>…</tag>` block (e.g. drawings, which cannot be read aloud).
fn drop_blocks(s: &str, tag: &str) -> String {
    let (open, close) = (format!("<{tag}"), format!("</{tag}>"));
    let mut s = s.to_string();
    while let Some(i) = s.find(&open) {
        let end = s[i..]
            .find(&close)
            .map(|j| i + j + close.len())
            .unwrap_or(s.len());
        s.replace_range(i..end, " ");
    }
    s
}

/// How an exponent is read: ² "στο τετράγωνο", ⁵ "στην πέμπτη", ¹⁴ "στη δύναμη 14".
fn power_words(e: &str) -> String {
    let w = match e {
        "1" => "στην πρώτη",
        "2" => "στο τετράγωνο",
        "3" => "στον κύβο",
        "4" => "στην τετάρτη",
        "5" => "στην πέμπτη",
        "6" => "στην έκτη",
        "7" => "στην έβδομη",
        "8" => "στην όγδοη",
        "9" => "στην ένατη",
        "10" => "στη δέκατη",
        "?" | "□" => "στη δύναμη κουτάκι",
        e => return format!(" στη δύναμη {e} "),
    };
    format!(" {w} ")
}

/// `<sup>…</sup>` and ²³… characters become words.
fn powers(s: &str) -> String {
    const SUP: &str = "⁰¹²³⁴⁵⁶⁷⁸⁹";
    let mut s = s.to_string();
    while let Some(i) = s.find("<sup>") {
        let Some(j) = s[i..].find("</sup>") else {
            break;
        };
        let e = strip_tags(&s[i + 5..i + j]);
        s.replace_range(i..i + j + 6, &power_words(e.trim()));
    }
    let mut out = String::new();
    let mut run = String::new();
    for c in s.chars().chain(std::iter::once('\0')) {
        if let Some(d) = SUP.chars().position(|x| x == c) {
            run.push(char::from(b'0' + d as u8));
            continue;
        }
        if !run.is_empty() {
            out.push_str(&power_words(&run));
            run.clear();
        }
        if c != '\0' {
            out.push(c);
        }
    }
    out
}

/// ημ / συν / εφ written in front of an angle are read in full.
fn trig_words(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    'outer: while i < chars.len() {
        for (short, long) in [
            ("ημ", "ημίτονο"),
            ("συν", "συνημίτονο"),
            ("εφ", "εφαπτομένη"),
        ] {
            let n = short.chars().count();
            let word_start = i == 0 || !chars[i - 1].is_alphabetic();
            let matches = chars[i..].iter().take(n).copied().eq(short.chars());
            let next = chars.get(i + n);
            if word_start && matches && next.is_some_and(|c| c.is_ascii_digit() || c.is_uppercase())
            {
                out.push_str(long);
                out.push(' ');
                i += n;
                continue 'outer;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// Turn question HTML into something a Greek speech synthesiser reads sensibly.
pub fn speak(html: &str) -> String {
    let mut s = drop_blocks(html, "svg");
    // fractions: <span class="fr"><span>A</span><span>B</span></span> → "A προς B"
    while let Some(i) = s.find("<span class=\"fr\"><span>") {
        let rest = &s[i + 23..];
        let Some(a_end) = rest.find("</span><span>") else {
            break;
        };
        let a = rest[..a_end].to_string();
        let rest2 = &rest[a_end + 13..];
        let Some(b_end) = rest2.find("</span></span>") else {
            break;
        };
        let b = rest2[..b_end].to_string();
        let end = i + 23 + a_end + 13 + b_end + 14;
        s.replace_range(i..end, &format!(" {a} προς {b} "));
    }
    let s = s
        .replace("<span class=\"ov\">", " και περίοδος ")
        .replace("<br>", ". ")
        .replace("<table", ". <table")
        .replace("</td>", ", ")
        .replace("</th>", ", ")
        .replace("</tr>", ". ")
        .replace("<sub>", " ")
        .replace("</sub>", " ");
    let s = s
        .replace("cm²", " τετραγωνικά εκατοστά ")
        .replace("cm³", " κυβικά εκατοστά ")
        .replace("m²", " τετραγωνικά μέτρα ");
    let s = powers(&s);
    let mut s = trig_words(&strip_tags(&s));
    for (unit, word) in [
        ("km", " χιλιόμετρα "),
        ("cm", " εκατοστά "),
        ("°", " μοίρες "),
        ("€", " ευρώ "),
        ("Â", "A\u{302}"),
        ("Ê", "E\u{302}"),
    ] {
        s = s.replace(unit, word);
    }
    // "1 : 6" is a ratio in ratio exercises, a division elsewhere.
    let colon = if s.contains("λόγο") {
        " προς "
    } else {
        " διά "
    };
    let s = s
        .replace(" : ", colon)
        .replace(")(", ") · (")
        .replace("][", "] · [");
    let chars: Vec<char> = s.chars().collect();
    // Is the bracket opened at `i` around math (not around words)?
    let math_inside = |i: usize| {
        let inner: String = chars[i + 1..]
            .iter()
            .take_while(|c| **c != ')' && **c != ']')
            .collect();
        let wordy = inner
            .split(|c: char| !c.is_alphabetic())
            .any(|w| w.chars().count() >= 3 && w != "προς");
        !wordy && inner.chars().any(|c| c == 'x' || c.is_ascii_digit())
    };
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        // A Latin letter on its own is a variable; inside a word it is just a letter.
        let alone = (i == 0 || !chars[i - 1].is_alphabetic())
            && chars.get(i + 1).is_none_or(|n| !n.is_alphabetic());
        if c.is_alphabetic() && chars.get(i + 1) == Some(&'\u{302}') {
            out.push_str(&format!(" γωνία {c} "));
            i += 2;
            continue;
        }
        let w = match c {
            'x' if alone => " χι ",
            'y' if alone => " ψι ",
            '₀'..='₉' => {
                out.push(' ');
                out.push(char::from(b'0' + (c as u32 - '₀' as u32) as u8));
                i += 1;
                continue;
            }
            '−' | '–' => " μείον ",
            '+' => " συν ",
            '·' => " επί ",
            '=' => " ίσον ",
            '√' => " ρίζα ",
            '≠' => " διάφορο του ",
            '≤' => " μικρότερο ή ίσο του ",
            '≥' => " μεγαλύτερο ή ίσο του ",
            '<' => " μικρότερο από ",
            '>' => " μεγαλύτερο από ",
            '|' => " διαιρεί ",
            '∩' => " τομή ",
            '∪' => " ένωση ",
            '′' => " συμπλήρωμα ",
            '∅' => " κενό σύνολο ",
            '∈' => " ανήκει στο ",
            '∉' => " δεν ανήκει στο ",
            '⊆' => " υποσύνολο του ",
            '{' | '}' => " ",
            '/' => " προς ",
            // Math in brackets is read "παρένθεση … ," so that the grouping is heard.
            '(' | '[' if math_inside(i) => " παρένθεση ",
            '(' | '[' => ", ",
            ')' | ']' => ", ",
            '□' => " κουτάκι ",
            '\u{302}' => " ",
            _ => {
                out.push(c);
                i += 1;
                continue;
            }
        };
        out.push_str(w);
        i += 1;
    }
    let mut r = out.split_whitespace().collect::<Vec<_>>().join(" ");
    for _ in 0..2 {
        for (a, b) in [
            (" ,", ","),
            (" .", "."),
            (" ;", ";"),
            (" :", ":"),
            (",,", ","),
            (",.", "."),
            (",;", ";"),
            (",:", ":"),
            (";.", ";"),
            (":.", ":"),
            ("..", "."),
            ("?.", "?"),
        ] {
            r = r.replace(a, b);
        }
    }
    r.trim_end_matches(',').to_string()
}

// ---------------- instructions (every category) ----------------

fn kind_help(k: &Kind) -> (&'static str, String, String) {
    match k {
        Kind::Num => ("Γράφω αριθμό", "Γράψε τον αριθμό στο κουτάκι.".into(), "Γράψε τον αριθμό στο κουτάκι.".into()),
        Kind::Int => (
            "Γράφω αριθμό με πρόσημο",
            "Γράψε τον αριθμό. Αν είναι <b>αρνητικός</b>, πάτα το κουμπί <b>±</b>.".into(),
            "Γράψε τον αριθμό. Αν είναι αρνητικός, πάτα το κουμπί συν πλην.".into(),
        ),
        Kind::Frac => (
            "Γράφω κλάσμα",
            "<b>Πάνω</b> κουτί: ο αριθμητής. <b>Κάτω</b> κουτί: ο παρονομαστής.<br>Απλοποίησε το κλάσμα. Αν η απάντηση είναι ακέραιος, άφησε το κάτω κουτί κενό.".into(),
            "Στο πάνω κουτί γράφεις τον αριθμητή και στο κάτω τον παρονομαστή. Απλοποίησε το κλάσμα. Αν η απάντηση είναι ακέραιος, άφησε το κάτω κουτί κενό.".into(),
        ),
        Kind::Slots { unordered, .. } => (
            "Συμπληρώνω τα κουτάκια",
            format!("Γράψε έναν αριθμό σε <b>κάθε</b> κουτάκι. Για αρνητικό, πάτα <b>±</b>.{}", if *unordered { "<br>Η σειρά <b>δεν</b> μετράει." } else { "" }),
            format!("Γράψε έναν αριθμό σε κάθε κουτάκι. Για αρνητικό, πάτα συν πλην.{}", if *unordered { " Η σειρά δεν μετράει." } else { "" }),
        ),
        Kind::Surd => (
            "Γράφω ρίζα",
            "Πρώτο κουτί: ο αριθμός <b>μπροστά</b> από τη ρίζα. Δεύτερο κουτί: ο αριθμός <b>μέσα</b> στη ρίζα.<br>Βγάλε έξω από τη ρίζα ό,τι μπορείς. Για ακέραιο, άφησε κενή τη ρίζα.".into(),
            "Στο πρώτο κουτί γράφεις τον αριθμό μπροστά από τη ρίζα, στο δεύτερο τον αριθμό μέσα στη ρίζα. Αν η απάντηση είναι ακέραιος, άφησε κενή τη ρίζα.".into(),
        ),
        Kind::Dec { .. } => (
            "Γράφω δεκαδικό",
            "Γράψε τον αριθμό με <b>κόμμα</b>, π.χ. 3,5. Στρογγύλεψε όπως λέει η άσκηση.".into(),
            "Γράψε τον αριθμό με κόμμα, για παράδειγμα τρία κόμμα πέντε. Στρογγύλεψε όπως λέει η άσκηση.".into(),
        ),
        Kind::Select { .. } => (
            "Διαλέγω από λίστα",
            "Πάτα τη <b>λίστα</b> και διάλεξε <b>μία</b> απάντηση.".into(),
            "Πάτα τη λίστα και διάλεξε μία απάντηση.".into(),
        ),
        Kind::YesNo { yes, no } => (
            "Πατάω ένα κουμπί",
            format!("Πάτα <b>{yes}</b> ή <b>{no}</b>."),
            format!("Πάτα {yes} ή {no}."),
        ),
        Kind::Set { .. } => (
            "Διαλέγω όσα ταιριάζουν",
            "Πάτα <b>όλα</b> τα κουμπιά που ταιριάζουν. Μπορεί να είναι πολλά. Πάτα ξανά για να ακυρώσεις.".into(),
            "Πάτα όλα τα κουμπιά που ταιριάζουν. Μπορεί να είναι πολλά. Αν πατήσεις λάθος, πάτα ξανά για να το ακυρώσεις.".into(),
        ),
        Kind::Exp { .. } => (
            "Γράφω εκθέτες",
            "Πάνω από κάθε πρώτο αριθμό γράψε τον <b>εκθέτη</b> του. Αν ο αριθμός δεν χρειάζεται, άφησε κενό.".into(),
            "Πάνω από κάθε πρώτο αριθμό γράψε τον εκθέτη του. Αν κάποιος πρώτος δεν χρειάζεται, άφησε το κουτάκι κενό.".into(),
        ),
    }
}

pub fn instructions(q: &Question) -> Lesson {
    let mut steps = vec![st(
        "Διαβάζουμε την άσκηση",
        format!("<div class=\"lp\">{}</div>", q.prompt),
        format!("Ας διαβάσουμε μαζί την άσκηση. {}", speak(&q.prompt)),
    )];
    // what is asked: one line per question part
    let asks: Vec<&str> = q
        .groups
        .iter()
        .map(|g| g.label.as_str())
        .filter(|l| !l.is_empty())
        .collect();
    let n_parts = q.parts().count();
    if !asks.is_empty() {
        steps.push(st(
            "Τι μου ζητά",
            format!(
                "<ol class=\"lask\">{}</ol>",
                asks.iter()
                    .map(|a| format!("<li>{a}</li>"))
                    .collect::<String>()
            ),
            format!(
                "Η άσκηση έχει {} ερωτήματα. {} Πρέπει να απαντήσεις σε όλα.",
                asks.len(),
                asks.iter().map(|a| speak(a)).collect::<Vec<_>>().join(". ")
            ),
        ));
    } else if let Some(boxes) = q
        .parts()
        .map(|p| match &p.kind {
            Kind::Slots { pieces, .. } => Some(pieces.len().saturating_sub(1)),
            _ => None,
        })
        .sum::<Option<usize>>()
        .filter(|b| *b > 1)
    {
        steps.push(st(
            "Τι μου ζητά",
            format!("Θα συμπληρώσεις <b>{boxes}</b> κουτάκια."),
            format!("Θα συμπληρώσεις {boxes} κουτάκια. Συμπλήρωσε όλα τα κουτάκια."),
        ));
    } else {
        steps.push(st(
            "Τι μου ζητά",
            format!(
                "Θα δώσεις <b>{n_parts}</b> {}.",
                if n_parts == 1 {
                    "απάντηση"
                } else {
                    "απαντήσεις"
                }
            ),
            if n_parts == 1 {
                "Θα δώσεις μία απάντηση.".to_string()
            } else {
                format!("Θα δώσεις {n_parts} απαντήσεις. Απάντησε σε όλες.")
            },
        ));
    }
    // how to type each kind of answer (once per kind)
    let mut seen: Vec<&'static str> = vec![];
    for p in q.parts() {
        let (t, show, say) = kind_help(&p.kind);
        if !seen.contains(&t) {
            seen.push(t);
            steps.push(st(t, show, say));
        }
    }
    let has_method = METHOD_CATS.contains(&q.cat);
    steps.push(st(
        "Όταν τελειώσω",
        if has_method {
            "1. Πάτα <b>Έλεγχος</b>.<br>2. Αν κάτι είναι κόκκινο, διόρθωσέ το και πάτα ξανά Έλεγχος.<br>3. Αν κολλήσεις: πάτα <b>Βοήθεια</b>, ή δες το βίντεο <b>Μέθοδος</b>."
        } else {
            "1. Πάτα <b>Έλεγχος</b>.<br>2. Αν κάτι είναι κόκκινο, διόρθωσέ το και πάτα ξανά Έλεγχος.<br>3. Αν κολλήσεις: πάτα <b>Βοήθεια</b>."
        },
        if has_method {
            "Όταν τελειώσεις, πάτα Έλεγχος. Αν κάτι γίνει κόκκινο, διόρθωσέ το και πάτα ξανά Έλεγχος. Αν κολλήσεις, πάτα Βοήθεια ή δες το βίντεο Μέθοδος. Καλή επιτυχία!"
        } else {
            "Όταν τελειώσεις, πάτα Έλεγχος. Αν κάτι γίνει κόκκινο, διόρθωσέ το και πάτα ξανά Έλεγχος. Αν κολλήσεις, πάτα Βοήθεια. Καλή επιτυχία!"
        },
    ));
    Lesson {
        title: "Οδηγίες".into(),
        steps,
    }
}

// ---------------- method lessons (pilot categories) ----------------
//
// Every example is built from a few numbers, and `method` tries the variants in
// order until it finds one that does not appear in the pupil's own question.

const X: &str = "<i>x</i>";

fn your_turn() -> Step {
    st("Τώρα εσύ", "Κάνε τα <b>ίδια βήματα</b> με τους αριθμούς της δικής σου άσκησης.", "Τώρα κάνε τα ίδια βήματα με τους αριθμούς της δικής σου άσκησης. Μπορείς να ξαναδείς όποιο βήμα θέλεις.")
}

fn check_step() -> Step {
    st("Έλεγχος", "Βάλε κάθε λύση στην αρχική εξίσωση. Πρέπει να βγαίνει ίσο και στα δύο μέλη.", "Στο τέλος βάλε κάθε λύση στην αρχική εξίσωση. Αν βγαίνει ίσο και στα δύο μέλη, η λύση είναι σωστή.")
}

/// ` + 5` / ` − 5` (nothing for 0), to append a term.
fn tail(c: i64) -> String {
    match c {
        0 => String::new(),
        c if c < 0 => format!(" − {}", -c),
        c => format!(" + {c}"),
    }
}

/// `3x`, `x`, `−x`, `−2x` (coefficient 1 hidden).
fn xterm(a: i64) -> String {
    match a {
        1 => X.into(),
        -1 => format!("−{X}"),
        a => format!("{}{X}", signed(a)),
    }
}

/// `ax + b` (or just `b` when a = 0).
fn lin(a: i64, b: i64) -> String {
    if a == 0 {
        signed(b)
    } else {
        format!("{}{}", xterm(a), tail(b))
    }
}

/// `x² + bx + c` with zero and unit coefficients tidied.
fn quad(a: i64, b: i64, c: i64) -> String {
    let lead = match a {
        1 => format!("{X}²"),
        -1 => format!("−{X}²"),
        a => format!("{}{X}²", signed(a)),
    };
    let mid = match b {
        0 => String::new(),
        1 => format!(" + {X}"),
        -1 => format!(" − {X}"),
        b if b < 0 => format!(" − {}{X}", -b),
        b => format!(" + {b}{X}"),
    };
    format!("{lead}{mid}{}", tail(c))
}

/// The factor that is zero at `r`: (x − r).
fn root(r: i64) -> String {
    format!("({})", lin(1, -r))
}

/// `x = r` in bold.
fn sol(r: &str) -> String {
    format!("<b>{X} = {r}</b>")
}

fn m_num(v: usize) -> Option<Lesson> {
    let (a, b) = *[(24u64, 36u64), (18, 30), (20, 50), (28, 42)].get(v)?;
    let (fa, fb) = (factor(a), factor(b));
    let (g, l) = (
        min_map(&[fa.clone(), fb.clone()]),
        max_map(&[fa.clone(), fb.clone()]),
    );
    let (pa, pb, pg, pl) = (fmt_pow(&fa), fmt_pow(&fb), fmt_pow(&g), fmt_pow(&l));
    let (gv, lv) = (map_val(&g), map_val(&l));
    Some(Lesson {
        title: "Μέθοδος: ΜΚΔ και ΕΚΠ".into(),
        steps: vec![
            st("Τι ψάχνουμε", format!("<b>ΜΚΔ</b>: ο <b>μεγαλύτερος</b> αριθμός που διαιρεί όλους.<br><b>ΕΚΠ</b>: το <b>μικρότερο</b> κοινό πολλαπλάσιο.<br>Παράδειγμα: <b>{a}</b> και <b>{b}</b>."), format!("Ο ΜΚΔ είναι ο μεγαλύτερος αριθμός που διαιρεί όλους τους αριθμούς. Το ΕΚΠ είναι το μικρότερο κοινό τους πολλαπλάσιο. Θα δούμε ένα παράδειγμα με το {a} και το {b}.")),
            st("Βήμα 1: Η σκάλα", format!("{}{}", ladders(&[a, b]), fac_lines(&[a, b])), format!("Αναλύουμε κάθε αριθμό με τη σκάλα. Διαιρούμε με τον μικρότερο πρώτο αριθμό που χωράει, ξανά και ξανά, μέχρι να φτάσουμε στο ένα. {a} ίσον {}. {b} ίσον {}.", speak(&pa), speak(&pb))),
            st("Βήμα 2: ΜΚΔ", format!("{a} = {pa}<br>{b} = {pb}<br>Μόνο οι <b>κοινοί</b> πρώτοι, με τον <b>μικρότερο</b> εκθέτη:<br><b>ΜΚΔ = {pg} = {gv}</b>"), format!("Για τον ΜΚΔ παίρνουμε μόνο τους κοινούς πρώτους, με τον μικρότερο εκθέτη. ΜΚΔ ίσον {}, δηλαδή {gv}.", speak(&pg))),
            st("Βήμα 3: ΕΚΠ", format!("{a} = {pa}<br>{b} = {pb}<br><b>Όλοι</b> οι πρώτοι, με τον <b>μεγαλύτερο</b> εκθέτη:<br><b>ΕΚΠ = {pl} = {lv}</b>"), format!("Για το ΕΚΠ παίρνουμε όλους τους πρώτους, με τον μεγαλύτερο εκθέτη. Προσοχή, εδώ γίνονται τα περισσότερα λάθη. ΕΚΠ ίσον {}, δηλαδή {lv}.", speak(&pl))),
            st("Βήμα 4: Έλεγχος", format!("{a} : {gv} = {} ✓ &nbsp; {b} : {gv} = {} ✓<br>{lv} : {a} = {} ✓ &nbsp; {lv} : {b} = {} ✓", a / gv, b / gv, lv / a, lv / b), format!("Ελέγχουμε. Το {gv} διαιρεί το {a} και το {b}. Το {lv} διαιρείται και από τους δύο. Σωστά!")),
            your_turn(),
        ],
    })
}

fn m_factor(q: &Question, v: usize) -> Option<Lesson> {
    let kind = match q.parts().next().map(|p| &p.kind) {
        Some(Kind::Select { .. }) => "mc",
        Some(Kind::Slots { pieces, .. }) if pieces.len() == 2 => "square",
        _ if q.prompt.contains("τριώνυμο") => "tri",
        _ => "diff",
    };
    let steps = match kind {
        "tri" => {
            let (p, r) = *[(2i64, 5i64), (3, 4), (1, 6), (2, 7)].get(v)?;
            let (s, m) = (p + r, p * r);
            let e = quad(1, s, m);
            let tries: Vec<String> = (1..=p)
                .filter(|d| m % d == 0)
                .map(|d| {
                    let ok = d == p;
                    let line = format!(
                        "{d}·{} → άθροισμα {} {}",
                        m / d,
                        d + m / d,
                        if ok { "✓" } else { "✗" }
                    );
                    if ok {
                        format!("<mark>{line}</mark>")
                    } else {
                        line
                    }
                })
                .collect();
            let neg = quad(1, -(r - p), -m);
            vec![
                st("Τι βλέπω", format!("Παράδειγμα: <b>{e}</b><br>3 όροι, το {X}² χωρίς αριθμό μπροστά → <b>τριώνυμο</b>."), format!("Θα δούμε ένα παράδειγμα: {}. Έχει τρεις όρους, άρα είναι τριώνυμο.", speak(&e))),
                st("Βήμα 1: Γινόμενο και άθροισμα", format!("Ψάχνω <b>δύο αριθμούς</b>:<br>γινόμενο = <mark>{m}</mark> (ο σταθερός όρος)<br>άθροισμα = <mark>{s}</mark> (ο αριθμός μπροστά από το {X})"), format!("Ψάχνουμε δύο αριθμούς. Το γινόμενό τους πρέπει να είναι {m}, όσο ο σταθερός όρος. Το άθροισμά τους πρέπει να είναι {s}, όσο ο αριθμός μπροστά από το χι.")),
                st("Βήμα 2: Δοκιμάζω ζευγάρια", tries.join("<br>"), format!("Δοκιμάζουμε ζευγάρια που κάνουν γινόμενο {m}, μέχρι να βρούμε άθροισμα {s}. Το {p} και το {r}. Αυτοί είναι!")),
                st("Βήμα 3: Γράφω τους παράγοντες", format!("{e} = <b>{}{}</b>", root(-p), root(-r)), format!("Γράφουμε: παρένθεση {}, επί παρένθεση {}.", speak(&lin(1, p)), speak(&lin(1, r)))),
                st("Βήμα 4: Πρόσημα", format!("Αν ο σταθερός όρος είναι <b>αρνητικός</b>, οι αριθμοί έχουν <b>διαφορετικά</b> πρόσημα.<br>π.χ. {neg} = {}{}", root(r), root(-p)), "Προσοχή στα πρόσημα. Αν ο σταθερός όρος είναι αρνητικός, ο ένας αριθμός είναι θετικός και ο άλλος αρνητικός. Στην εφαρμογή γράφεις τους αριθμούς με το πρόσημό τους στα κουτάκια."),
                st("Βήμα 5: Έλεγχος", format!("{}{} = {X}² + {}{X} + {}{X} + {m} = {e} ✓", root(-p), root(-r), r, if p == 1 { String::new() } else { p.to_string() }), "Ελέγχουμε κάνοντας τον πολλαπλασιασμό. Βγαίνει ξανά η αρχική παράσταση. Σωστά!"),
                your_turn(),
            ]
        }
        "diff" => {
            let b = *[7i64, 6, 8, 5, 9].get(v)?;
            let e = format!("{X}² − {}", b * b);
            vec![
                st("Τι βλέπω", format!("Παράδειγμα: <b>{e}</b><br>2 όροι, και οι δύο <b>τετράγωνα</b>, με <b>μείον</b> ανάμεσα."), format!("Παράδειγμα: {}. Δύο όροι, και οι δύο είναι τετράγωνα, και ανάμεσα έχουν μείον. Αυτό είναι διαφορά τετραγώνων.", speak(&e))),
                st("Βήμα 1: Γράφω τα τετράγωνα", format!("{e} = {X}² − <mark>{b}²</mark>"), format!("Γράφουμε το {} ως {b} στο τετράγωνο.", b * b)),
                st("Βήμα 2: Ο κανόνας", "α² − β² = <b>(α − β)(α + β)</b>", "Ο κανόνας λέει: α στο τετράγωνο μείον β στο τετράγωνο ίσον α μείον β επί α συν β."),
                st("Βήμα 3: Εφαρμόζω", format!("{X}² − {b}² = <b>{}{}</b>", root(b), root(-b)), format!("Άρα: χι μείον {b} επί χι συν {b}. Στην εφαρμογή γράφεις τα δύο κουτάκια: μείον {b} και {b}.")),
                st("Προσοχή", format!("Το {X}² <b>+</b> {} <b>δεν</b> αναλύεται έτσι.<br>Χρειάζεται <b>μείον</b>.", b * b), "Προσοχή: αν ανάμεσα έχει συν, δεν είναι διαφορά τετραγώνων."),
                your_turn(),
            ]
        }
        "square" => {
            let b = *[3i64, 5, 4, 6].get(v)?;
            let e = quad(1, 2 * b, b * b);
            vec![
                st("Τι βλέπω", format!("Παράδειγμα: <b>{e}</b><br>Πρώτος και τελευταίος όρος είναι <b>τετράγωνα</b>."), format!("Παράδειγμα: {}. Ο πρώτος όρος είναι χι στο τετράγωνο και ο τελευταίος {}, που είναι {b} στο τετράγωνο.", speak(&e), b * b)),
                st("Βήμα 1: Βρίσκω α και β", format!("α² = {X}² → α = {X}<br>β² = {} → <mark>β = {b}</mark>", b * b), format!("Το α είναι χι και το β είναι {b}.")),
                st("Βήμα 2: Ελέγχω τον μεσαίο", format!("2·α·β = 2·{X}·{b} = <mark>{}{X}</mark> ✓", 2 * b), format!("Ελέγχουμε τον μεσαίο όρο: δύο επί χι επί {b} ίσον {} χι. Ταιριάζει!", 2 * b)),
                st("Βήμα 3: Γράφω το τετράγωνο", format!("{e} = <b>{}²</b><br>Αν ο μεσαίος έχει μείον: {}²", root(-b), root(b)), format!("Άρα είναι χι συν {b} στο τετράγωνο. Αν ο μεσαίος όρος έχει μείον, γράφουμε χι μείον {b} στο τετράγωνο. Στην εφαρμογή γράφεις στο κουτάκι τον αριθμό με το πρόσημό του.")),
                your_turn(),
            ]
        }
        _ => {
            let (k, b) = *[(2i64, 3i64), (3, 2), (5, 2), (2, 5)].get(v)?;
            let e = format!("{k}{X}² − {}", k * b * b);
            vec![
                st("Η σειρά που ακολουθώ", "1. <b>Κοινός παράγοντας;</b><br>2. Μετρώ τους όρους:<br>&nbsp;&nbsp;2 όροι → διαφορά τετραγώνων ή κύβοι<br>&nbsp;&nbsp;3 όροι → τριώνυμο ή τέλειο τετράγωνο<br>&nbsp;&nbsp;4 όροι → ομαδοποίηση<br>3. Συνεχίζω μέχρι να μην αναλύεται τίποτα.", "Για κάθε παραγοντοποίηση ακολουθούμε την ίδια σειρά. Πρώτα ψάχνουμε κοινό παράγοντα. Μετά μετράμε τους όρους. Δύο όροι: διαφορά τετραγώνων ή κύβοι. Τρεις όροι: τριώνυμο ή τέλειο τετράγωνο. Τέσσερις όροι: ομαδοποίηση. Και συνεχίζουμε μέχρι να μην αναλύεται τίποτα άλλο."),
                st("Παράδειγμα: κοινός παράγοντας", format!("<b>{e}</b><br>Κοινός παράγοντας <mark>{k}</mark>: {k}({X}² − {})", b * b), format!("Παράδειγμα: {}. Βγάζουμε κοινό παράγοντα το {k}. Μένει χι στο τετράγωνο μείον {}.", speak(&e), b * b)),
                st("Συνεχίζω: διαφορά τετραγώνων", format!("{X}² − {} = {X}² − {b}² → διαφορά τετραγώνων<br>= <b>{k}{}{}</b>", b * b, root(b), root(-b)), format!("Το χι στο τετράγωνο μείον {} είναι διαφορά τετραγώνων. Άρα όλο γίνεται {k} επί χι μείον {b} επί χι συν {b}.", b * b)),
                st("Προσοχή στο πρόσημο", "(α − β) = <b>−</b>(β − α)<br>π.χ. 16(1 − 3<i>y</i>) = −16(3<i>y</i> − 1)", "Αν δύο παρενθέσεις διαφέρουν μόνο στη σειρά, βγάζουμε ένα μείον. Για παράδειγμα, δεκαέξι επί ένα μείον τρία ψι, ισούται με μείον δεκαέξι επί τρία ψι μείον ένα."),
                st("Πώς διαλέγω απάντηση", "Στη λίστα, διάλεξε την παραγοντοποίηση που:<br>• <b>δεν</b> αναλύεται άλλο<br>• αν την πολλαπλασιάσεις, δίνει την αρχική παράσταση", "Στη λίστα, διάλεξε την παραγοντοποίηση που δεν αναλύεται άλλο, και που αν την πολλαπλασιάσεις δίνει την αρχική παράσταση."),
                your_turn(),
            ]
        }
    };
    Some(Lesson {
        title: "Μέθοδος: Παραγοντοποίηση".into(),
        steps,
    })
}

fn m_eq2(q: &Question, v: usize) -> Option<Lesson> {
    let p = &q.prompt;
    let kind = if p.contains("τρεις λύσεις") {
        "cubic"
    } else if p.contains("Τι ρίζες έχει") {
        "kind"
    } else if p.contains("τιμές του κ") {
        "param"
    } else if p.contains("ορθογώνι") {
        "word"
    } else if p.contains(")² = (") {
        "squares"
    } else if q.parts().any(|pt| pt.label.contains("Μικρότερη")) {
        "split"
    } else if p.contains(")(") && !p.contains(") = 0") {
        "prodconst"
    } else if p.contains(")(") {
        "product"
    } else {
        "standard"
    };
    let steps = match kind {
        "standard" => {
            let (r1, r2) = *[(2i64, 3i64), (1, 4), (2, 5), (3, 4)].get(v)?;
            let (s, m) = (r1 + r2, r1 * r2);
            let e = format!("{X}² − {s}{X} = −{m}");
            let f = format!("{}{}", root(r1), root(r2));
            vec![
                st("Παράδειγμα", format!("<b>{e}</b>"), format!("Παράδειγμα: {}.", speak(&e))),
                st("Βήμα 1: Όλα αριστερά", format!("{X}² − {s}{X} <mark>+ {m}</mark> = <mark>0</mark>"), format!("Πρώτα φέρνουμε όλους τους όρους στο αριστερό μέλος, ώστε δεξιά να μείνει μηδέν. Το μείον {m} περνάει αριστερά ως συν {m}.")),
                st("Βήμα 2: Παραγοντοποιώ", format!("Γινόμενο {m}, άθροισμα −{s} → −{r1} και −{r2}<br>{f} = 0"), format!("Παραγοντοποιούμε το τριώνυμο. Ψάχνουμε δύο αριθμούς με γινόμενο {m} και άθροισμα μείον {s}: μείον {r1} και μείον {r2}.")),
                st("Βήμα 3: Κάθε παράγοντας = 0", format!("{} = 0 → {}<br>{} = 0 → {}", lin(1, -r1), sol(&r1.to_string()), lin(1, -r2), sol(&r2.to_string())), format!("Ένα γινόμενο είναι μηδέν όταν ένας παράγοντας είναι μηδέν. Άρα χι μείον {r1} ίσον μηδέν, δηλαδή χι ίσον {r1}, ή χι μείον {r2} ίσον μηδέν, δηλαδή χι ίσον {r2}.")),
                st("Προσοχή στα πρόσημα", format!("{} = 0 → {X} = <b>+{r1}</b>, όχι −{r1}", root(r1)), format!("Προσοχή: από το χι μείον {r1} βγαίνει χι ίσον συν {r1}. Το πρόσημο αλλάζει.")),
                check_step(),
                your_turn(),
            ]
        }
        "product" => {
            let (a, b) = *[(2i64, 3i64), (3, 1), (1, 4), (4, 2)].get(v)?;
            let e = format!("({})({}) = 0", lin(2, -2 * a), lin(1, b));
            vec![
                st("Παράδειγμα", format!("<b>{e}</b>"), format!("Παράδειγμα: {}.", speak(&e))),
                st("Βήμα 1: Ήδη γινόμενο = 0", "Η εξίσωση είναι ήδη <b>γινόμενο = 0</b>. Δεν χρειάζεται να κάνω πράξεις.", "Η εξίσωση είναι ήδη γινόμενο ίσον μηδέν. Δεν κάνουμε πράξεις."),
                st("Βήμα 2: Κάθε παράγοντας = 0", format!("{} = 0 → 2{X} = {} → {}<br>{} = 0 → {}", lin(2, -2 * a), 2 * a, sol(&a.to_string()), lin(1, b), sol(&signed(-b))), format!("Βάζουμε κάθε παράγοντα ίσον μηδέν. Δύο χι μείον {} ίσον μηδέν, άρα χι ίσον {a}. Χι συν {b} ίσον μηδέν, άρα χι ίσον μείον {b}.", 2 * a)),
                check_step(),
                your_turn(),
            ]
        }
        "prodconst" => {
            // (x − a)(x − b) = k  →  roots r1, r2
            let (a, b, k, r1, r2) = *[
                (2i64, 5i64, 4i64, 1i64, 6i64),
                (1, 4, -2, 2, 3),
                (3, 4, 6, 1, 6),
                (1, 6, -4, 2, 5),
            ]
            .get(v)?;
            let e = format!("{}{} = {}", root(a), root(b), signed(k));
            let ex = quad(1, -(a + b), a * b);
            let z = quad(1, -(a + b), a * b - k);
            vec![
                st("Παράδειγμα", format!("<b>{e}</b>"), format!("Παράδειγμα: {}.", speak(&e))),
                st("Παγίδα!", format!("Δεξιά <b>δεν</b> είναι 0. Δεν μπορώ να πω «κάθε παράγοντας = {}».", signed(k)), format!("Προσοχή, παγίδα! Δεξιά δεν έχουμε μηδέν αλλά {}. Δεν μπορούμε να βάλουμε κάθε παράγοντα ίσο με κάτι.", speak(&signed(k)))),
                st("Βήμα 1: Κάνω τις πράξεις", format!("{ex} = {}", signed(k)), format!("Κάνουμε τον πολλαπλασιασμό: {}.", speak(&format!("{ex} = {}", signed(k))))),
                st("Βήμα 2: Όλα αριστερά", format!("{z} = 0"), format!("Φέρνουμε το {} αριστερά: {}.", speak(&signed(k)), speak(&format!("{z} = 0")))),
                st("Βήμα 3: Παραγοντοποιώ και λύνω", format!("{}{} = 0 → {} ή {}", root(r1), root(r2), sol(&r1.to_string()), sol(&r2.to_string())), format!("Παραγοντοποιούμε: {}. Άρα χι ίσον {r1} ή χι ίσον {r2}.", speak(&format!("{}{}", root(r1), root(r2))))),
                check_step(),
                your_turn(),
            ]
        }
        "squares" => {
            // (x + c)² = (dx + e)²
            let (c, d, e) = *[(-1i64, 2i64, 1i64), (2, 3, -2), (-3, 2, 0)].get(v)?;
            let (a1, b1) = (1 - d, c - e);
            let (a2, b2) = (1 + d, c + e);
            let s1 = frac_text(-b1, a1).replace("−−", "");
            let r = |a: i64, b: i64| {
                let (n, den) = reduce(-b, a);
                frac_text(n, den)
            };
            let (x1, x2) = (r(a1, b1), r(a2, b2));
            let _ = s1;
            let eq = format!("({})² = ({})²", lin(1, c), lin(d, e));
            vec![
                st("Παράδειγμα", format!("<b>{eq}</b>"), format!("Παράδειγμα: {}.", speak(&eq))),
                st("Βήμα 1: Όλα αριστερά", format!("({})² − ({})² = 0", lin(1, c), lin(d, e)), "Φέρνουμε το δεξί μέλος αριστερά."),
                st("Βήμα 2: Διαφορά τετραγώνων", "Α² − Β² = (Α − Β)(Α + Β)", "Αυτό είναι διαφορά τετραγώνων: Α στο τετράγωνο μείον Β στο τετράγωνο ίσον Α μείον Β επί Α συν Β."),
                st("Βήμα 3: Απλοποιώ τις παρενθέσεις", format!("[{} − {}{}][{} + {}] = 0<br>({})({}) = 0", lin(1, c), xterm(d), tail(-e), lin(1, c), lin(d, e), lin(a1, b1), lin(a2, b2)), "Στην πρώτη αγκύλη, το μείον αλλάζει τα πρόσημα της δεύτερης παρένθεσης. Μετά κάνουμε αναγωγή ομοίων όρων σε κάθε αγκύλη."),
                st("Βήμα 4: Λύνω", format!("{} = 0 → {}<br>{} = 0 → {}", lin(a1, b1), sol(&x1), lin(a2, b2), sol(&x2)), format!("Κάθε παράγοντας ίσον μηδέν. Βρίσκουμε χι ίσον {} και χι ίσον {}. Στην εφαρμογή γράφεις πρώτα τη μικρότερη ρίζα.", speak(&x1), speak(&x2))),
                check_step(),
                your_turn(),
            ]
        }
        "split" => {
            // (x + p)(ax + 1) = ax² + (ap + 1)x + p
            let (a, pp) = *[(2i64, 3i64), (2, 2), (3, 2), (2, 5)].get(v)?;
            let e = quad(a, a * pp + 1, pp);
            vec![
                st("Παράδειγμα", format!("<b>{e} = 0</b>"), format!("Παράδειγμα: {} ίσον μηδέν. Μπροστά από το χι στο τετράγωνο έχει {a}, όχι ένα.", speak(&e))),
                st("Βήμα 1: Πολλαπλασιάζω", format!("{a} · {pp} = <mark>{}</mark><br>Ψάχνω αριθμούς με γινόμενο {} και άθροισμα <mark>{}</mark> → {} και 1", a * pp, a * pp, a * pp + 1, a * pp), format!("Πολλαπλασιάζουμε τον πρώτο αριθμό με τον τελευταίο: {a} επί {pp}, {}. Ψάχνουμε δύο αριθμούς με γινόμενο {} και άθροισμα {}: {} και ένα.", a * pp, a * pp, a * pp + 1, a * pp)),
                st("Βήμα 2: Σπάω τον μεσαίο όρο", format!("{a}{X}² + <mark>{}{X} + {X}</mark> + {pp} = 0", a * pp), format!("Γράφουμε το {} χι ως {} χι συν χι.", a * pp + 1, a * pp)),
                st("Βήμα 3: Ομαδοποίηση", format!("{a}{X}({}) + 1({}) = ({})({}) = 0", lin(1, pp), lin(1, pp), lin(1, pp), lin(a, 1)), format!("Ομαδοποιούμε. Κοινός παράγοντας το {}.", speak(&lin(1, pp)))),
                st("Βήμα 4: Λύνω", format!("{} = 0 → {}<br>{} = 0 → {}", lin(1, pp), sol(&signed(-pp)), lin(a, 1), sol(&format!("−1/{a}"))), format!("Χι συν {pp} ίσον μηδέν, άρα χι ίσον μείον {pp}. {a} χι συν ένα ίσον μηδέν, άρα χι ίσον μείον ένα προς {a}. Στην εφαρμογή γράφεις το κλάσμα: πάνω μείον ένα, κάτω {a}.")),
                check_step(),
                your_turn(),
            ]
        }
        "kind" => {
            let ((r1, r2), d, (h, k)) = *[
                ((2i64, 3i64), 3i64, (2i64, 1i64)),
                ((1, 4), 5, (3, 2)),
                ((2, 5), 4, (1, 3)),
            ]
            .get(v)?;
            vec![
                st("Τρεις περιπτώσεις", "Μια εξίσωση 2ου βαθμού μπορεί να έχει:<br>• <b>δύο</b> διαφορετικές ρίζες<br>• <b>μία διπλή</b> ρίζα<br>• <b>καμία</b> ρίζα (αδύνατη)", "Μια εξίσωση δευτέρου βαθμού μπορεί να έχει δύο διαφορετικές ρίζες, μία διπλή ρίζα, ή καμία ρίζα."),
                st("Δύο ρίζες", format!("{} = {}{} → <b>δύο</b> ρίζες", quad(1, -(r1 + r2), r1 * r2), root(r1), root(r2)), "Αν παραγοντοποιείται σε δύο διαφορετικούς παράγοντες, έχει δύο ρίζες."),
                st("Διπλή ρίζα", format!("{} = {}² → <b>μία διπλή</b>", quad(1, -2 * d, d * d), root(d)), "Αν είναι τέλειο τετράγωνο, έχει μία διπλή ρίζα."),
                st("Αδύνατη", format!("{} = {}² + {k}<br>τετράγωνο + θετικό &gt; 0 → <b>αδύνατη</b>", quad(1, 2 * h, h * h + k), root(-h)), "Αν γράφεται ως τετράγωνο συν θετικός αριθμός, δεν γίνεται ποτέ μηδέν. Η εξίσωση είναι αδύνατη."),
                st("Πώς το ελέγχω", format!("Συμπληρώνω τετράγωνο: {X}² + β{X} = ({X} + β/2)² − (β/2)²"), "Για να το ελέγξεις, συμπλήρωσε το τετράγωνο με το μισό του αριθμού μπροστά από το χι."),
                your_turn(),
            ]
        }
        "word" => {
            // sides (x + a), (x + b), positive root r
            let (a, b, r) = *[(1i64, 4i64, 4i64), (2, 3, 3), (2, 5, 3), (1, 2, 6)].get(v)?;
            // the pupil's rectangle must not be this one
            if p.contains(&lin(1, a)) && p.contains(&lin(1, b)) {
                return None;
            }
            let area = (r + a) * (r + b);
            let other = -(a + b) - r;
            vec![
                st("Παράδειγμα", format!("Ορθογώνιο με πλευρές ({}) και ({}), εμβαδόν {area}.", lin(1, a), lin(1, b)), format!("Παράδειγμα: ένα ορθογώνιο έχει πλευρές χι συν {a} και χι συν {b}, και εμβαδόν {area}.")),
                st("Βήμα 1: Γράφω την εξίσωση", format!("μήκος · πλάτος = εμβαδόν<br>({})({}) = {area}", lin(1, a), lin(1, b)), format!("Εμβαδόν ορθογωνίου είναι μήκος επί πλάτος. Γράφουμε: χι συν {a}, επί χι συν {b}, ίσον {area}.")),
                st("Βήμα 2: Πράξεις και όλα αριστερά", format!("{} − {area} = 0<br>{} = 0", quad(1, a + b, a * b), quad(1, a + b, a * b - area)), format!("Κάνουμε τις πράξεις και φέρνουμε το {area} αριστερά.")),
                st("Βήμα 3: Λύνω", format!("{}{} = 0 → {X} = {} ή {X} = {r}", root(other), root(r), signed(other), ), format!("Παραγοντοποιούμε και βρίσκουμε χι ίσον μείον {} ή χι ίσον {r}.", -other)),
                st("Βήμα 4: Ποια κρατώ;", format!("Μήκος δεν γίνεται αρνητικό: {X} = {} → πλευρά {} ✗<br>{} → πλευρές {} και {} ✓", signed(other), signed(other + a), sol(&r.to_string()), r + a, r + b), format!("Ένα μήκος δεν μπορεί να είναι αρνητικό. Η λύση μείον {} απορρίπτεται. Κρατάμε χι ίσον {r}.", -other)),
                your_turn(),
            ]
        }
        "param" => {
            let (c, r1, r2) = *[(2i64, 2i64, 3i64), (1, 3, 4), (3, 4, 5), (2, 1, 5)].get(v)?;
            let e = format!("{} = 0", quad(1, -(r1 + r2), r1 * r2));
            vec![
                st("Παράδειγμα", format!("{X} = κ + {c} είναι λύση της {e}"), format!("Παράδειγμα: το χι ίσον κάπα συν {c} είναι λύση της εξίσωσης {}.", speak(&e))),
                st("Βήμα 1: Λύνω την εξίσωση", format!("{}{} = 0 → {X} = {r1} ή {X} = {r2}", root(r1), root(r2)), format!("Πρώτα λύνουμε την εξίσωση: χι ίσον {r1} ή χι ίσον {r2}.")),
                st("Βήμα 2: Βρίσκω το κ", format!("κ + {c} = {r1} → <b>κ = {}</b><br>κ + {c} = {r2} → <b>κ = {}</b>", signed(r1 - c), signed(r2 - c)), format!("Μετά βάζουμε κάπα συν {c} ίσον με κάθε λύση. Βρίσκουμε κάπα ίσον {} ή κάπα ίσον {}.", speak(&signed(r1 - c)), speak(&signed(r2 - c)))),
                your_turn(),
            ]
        }
        _ => {
            let b = *[3i64, 2, 4, 5].get(v)?;
            let e = format!("{X}³ − {}{X} = 0", b * b);
            vec![
                st("Παράδειγμα", format!("<b>{e}</b>"), format!("Παράδειγμα: {}.", speak(&e))),
                st("Βήμα 1: Κοινός παράγοντας", format!("<mark>{X}</mark>({X}² − {}) = 0", b * b), "Βγάζουμε κοινό παράγοντα το χι. Μην διαιρέσεις με το χι, γιατί χάνεις τη λύση μηδέν."),
                st("Βήμα 2: Συνεχίζω την ανάλυση", format!("{X}{}{} = 0", root(b), root(-b)), format!("Το χι στο τετράγωνο μείον {} είναι διαφορά τετραγώνων.", b * b)),
                st("Βήμα 3: Τρεις λύσεις", format!("<b>{X} = 0</b>, <b>{X} = {b}</b>, <b>{X} = −{b}</b>"), format!("Κάθε παράγοντας ίσον μηδέν: χι ίσον μηδέν, {b} ή μείον {b}.")),
                check_step(),
                your_turn(),
            ]
        }
    };
    Some(Lesson {
        title: "Μέθοδος: Εξισώσεις 2ου βαθμού".into(),
        steps,
    })
}

/// Math fragments (with x and a number) of a lesson's example, used to make sure
/// the example is not the pupil's own question.
fn fragments(l: &Lesson) -> Vec<String> {
    let mut out = vec![];
    for s in &l.steps {
        for line in s.show.split("<br>") {
            let t = strip_tags(line).replace("&gt;", ">").replace("&nbsp;", " ");
            let mut cur = String::new();
            for c in t.chars().chain(std::iter::once('#')) {
                if c.is_ascii_digit() || "x+−=()²³ ·".contains(c) {
                    cur.push(c);
                } else {
                    let f = cur.trim().trim_end_matches('=').trim().to_string();
                    // A lone binomial such as (x + 1) is too common to give anything away.
                    let lone =
                        f.starts_with('(') && f.ends_with(')') && f.matches('(').count() == 1;
                    if !lone
                        && f.contains('x')
                        && f.chars().any(|c| c.is_ascii_digit())
                        && f.chars().count() >= 6
                    {
                        out.push(f);
                    }
                    cur.clear();
                }
            }
        }
    }
    out
}

fn numbers(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(String::from)
        .collect()
}

/// Does the lesson's example give away (part of) the question?
fn collides(q: &Question, l: &Lesson) -> bool {
    let plain = strip_tags(&q.prompt);
    if fragments(l).iter().any(|f| plain.contains(f.as_str())) {
        return true;
    }
    if q.cat == "num" {
        let qn = numbers(&plain);
        let ex = numbers(&l.steps[0].show);
        return qn.iter().filter(|n| ex.contains(n)).count() >= 2;
    }
    false
}

pub fn method(q: &Question) -> Option<Lesson> {
    if !METHOD_CATS.contains(&q.cat) {
        return None;
    }
    let build = |v: usize| -> Option<Option<Lesson>> {
        // Some(None): this variant is unusable (skipped); None: no more variants.
        let max = 4;
        if v >= max {
            return None;
        }
        Some(match q.cat {
            "num" => m_num(v),
            "c-factor" => m_factor(q, v),
            _ => m_eq2(q, v),
        })
    };
    let mut first = None;
    let mut v = 0;
    while let Some(l) = build(v) {
        if let Some(mut l) = l {
            // The player numbers the steps itself: "Βήμα 2: Ο κανόνας" → "Ο κανόνας".
            for st in &mut l.steps {
                if let Some((head, rest)) = st.title.split_once(": ") {
                    if head.starts_with("Βήμα ") {
                        st.title = rest.to_string();
                    } else if rest.starts_with("Βήμα ") {
                        st.title = head.to_string();
                    }
                }
            }
            if !collides(q, &l) {
                return Some(l);
            }
            first.get_or_insert(l);
        }
        v += 1;
    }
    first
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sentences_split() {
        assert_eq!(sentences("Α. Β; Γ!"), vec!["Α.", "Β;", "Γ!"]);
        assert_eq!(sentences("3,5 και 0,6."), vec!["3,5 και 0,6."]);
    }

    #[test]
    fn speech_is_readable() {
        assert_eq!(
            speak("<i>x</i>² − 9 = 0"),
            "χι στο τετράγωνο μείον 9 ίσον 0"
        );
        assert_eq!(speak("έχει 48 cm²"), "έχει 48 τετραγωνικά εκατοστά");
        assert_eq!(
            speak("(2<i>x</i> − 4)(<i>x</i> + 3) = 0"),
            "παρένθεση 2 χι μείον 4, επί παρένθεση χι συν 3, ίσον 0"
        );
        assert_eq!(
            speak("(<i>x</i> + 3)²"),
            "παρένθεση χι συν 3, στο τετράγωνο"
        );
        assert_eq!(
            speak("<span class=\"fr\"><span>3</span><span>4</span></span>"),
            "3 προς 4"
        );
    }
    #[test]
    fn method_example_is_not_the_question() {
        for cat in METHOD_CATS {
            for seed in 0..1500 {
                let q = crate::generate(cat, seed).unwrap();
                let l = method(&q).unwrap();
                assert!(!collides(&q, &l), "{cat} {seed}: {}", q.prompt);
            }
        }
    }

    #[test]
    fn lessons_exist() {
        for c in crate::CATEGORIES {
            for seed in 0..60 {
                let q = crate::generate(c.id, seed).unwrap();
                let l = instructions(&q);
                assert!(
                    l.steps.len() >= 3 && l.steps.iter().all(|s| !s.say.is_empty()),
                    "{}",
                    c.id
                );
                assert_eq!(method(&q).is_some(), METHOD_CATS.contains(&c.id));
            }
        }
    }
}
