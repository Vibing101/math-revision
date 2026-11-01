//! Κεφάλαιο 6 (Βασικές Γεωμετρικές Έννοιες) και Κεφάλαιο 9 (Γεωμετρία ΙΙ).

use crate::mk::*;
use crate::model::*;
use crate::util::*;

fn deg(x: u64) -> String {
    format!("{x}°")
}

// ---------------- Κεφάλαιο 6 ----------------

pub fn geo_len(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let half = r.r(2, 40);
            let (ab, am) = (2 * half, half);
            let ask_whole = r.chance(0.5);
            let (prompt, ans, sol) = if ask_whole {
                (format!("Το σημείο Μ είναι το <b>μέσο</b> του ευθύγραμμου τμήματος ΑΒ και ΑΜ = {am} cm. Πόσο είναι το ΑΒ;"), ab, format!("Το μέσο χωρίζει το τμήμα σε δύο ίσα μέρη: ΑΒ = 2·ΑΜ = 2·{am} = <b>{ab} cm</b>."))
            } else {
                (format!("Το σημείο Μ είναι το <b>μέσο</b> του ευθύγραμμου τμήματος ΑΒ και ΑΒ = {ab} cm. Πόσο είναι το ΜΒ;"), am, format!("Το μέσο χωρίζει το τμήμα σε δύο ίσα μέρη: ΜΒ = ΑΒ : 2 = {ab} : 2 = <b>{am} cm</b>."))
            };
            Question {
                cat: "geo-len",
                prompt,
                groups: one(vec![num("cm:", ans)]),
                hints: hints(
                    "Το μέσο Μ ενός τμήματος ΑΒ ισαπέχει από τα άκρα: ΑΜ = ΜΒ.",
                    "Άρα ΑΒ = ΑΜ + ΜΒ = 2·ΑΜ.",
                    if ask_whole {
                        "Διπλασίασε.".to_string()
                    } else {
                        "Μοίρασε στα δύο.".to_string()
                    },
                    sol.clone(),
                ),
                solution: sol,
                diagnose: no_diag(),
            }
        }
        1 => {
            // midpoint with algebra: AM = a x − b, MB = c x + d
            let x = r.r(3, 15);
            let (a, c) = loop {
                let a = r.r(3, 7);
                let c = r.r(1, 5);
                if a > c {
                    break (a, c);
                }
            };
            let d = r.r(1, 12);
            let am = c * x + d;
            // a x − b = c x + d  ⇒  b = (a − c) x − d, which must be positive
            if (a - c) * x <= d {
                return geo_len(r);
            }
            let b = (a - c) * x - d;
            let ab = 2 * am;
            Question {
                cat: "geo-len",
                prompt: format!("Το Μ είναι το μέσο του ΑΒ. Αν ΑΜ = ({a}<i>x</i> − {b}) cm και ΜΒ = ({c}<i>x</i> + {d}) cm, να βρεις το <i>x</i> και το μήκος του ΑΒ."),
                groups: vec![group("α)", vec![num("x =", x)]), group("β)", vec![num("ΑΒ (cm) =", ab)])],
                hints: hints(
                    "Αφού το Μ είναι μέσο, ΑΜ = ΜΒ. Γράψε την εξίσωση.",
                    format!("{a}<i>x</i> − {b} = {c}<i>x</i> + {d}"),
                    format!("{}<i>x</i> = {}", a - c, b + d),
                    format!("x = {x}, ΑΜ = {am}, ΑΒ = 2·{am} = {ab}"),
                ),
                solution: format!("ΑΜ = ΜΒ ⇔ {a}x − {b} = {c}x + {d} ⇔ {}x = {} ⇔ <b>x = {x}</b>.<br>ΑΜ = {a}·{x} − {b} = {am} cm, άρα <b>ΑΒ = {ab} cm</b>.", a - c, b + d),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some(x as i64) && v[1].num() == Some(am as i64) {
                        vec![msg("Βρήκες το ΑΜ. Το ΑΒ είναι διπλάσιο, αφού ΑΒ = ΑΜ + ΜΒ.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            // unit conversions that give whole numbers
            let (q, a, b, f): (u64, &str, &str, u64) = match r.r(0, 5) {
                0 => (r.r(2, 90), "cm", "mm", 10),
                1 => (r.r(2, 40), "m", "cm", 100),
                2 => (r.r(2, 30), "km", "m", 1000),
                3 => (r.r(2, 90) * 10, "mm", "cm", 10),
                4 => (r.r(2, 40) * 100, "cm", "m", 100),
                _ => (r.r(2, 30) * 1000, "m", "km", 1000),
            };
            let up = matches!((a, b), ("cm", "mm") | ("m", "cm") | ("km", "m"));
            let ans = if up { q * f } else { q / f };
            Question {
                cat: "geo-len",
                prompt: format!("Μετάτρεψε: <span class=\"big\">{q} {a} = ? {b}</span>"),
                groups: one(vec![num(format!("{b}:"), ans)]),
                hints: hints(
                    "1 km = 1000 m, 1 m = 100 cm, 1 cm = 10 mm.",
                    if up {
                        "Από μεγαλύτερη σε μικρότερη μονάδα πολλαπλασιάζουμε."
                    } else {
                        "Από μικρότερη σε μεγαλύτερη μονάδα διαιρούμε."
                    },
                    format!(
                        "1 {} = {f} {}",
                        if up { a } else { b },
                        if up { b } else { a }
                    ),
                    format!("{q} {a} = {ans} {b}"),
                ),
                solution: format!("{q} {} {f} = <b>{ans} {b}</b>", if up { "·" } else { ":" }),
                diagnose: Box::new(move |v| {
                    let wrong = if up { q / f } else { q * f };
                    if v[0].num() == Some(wrong as i64) {
                        vec![dm("units", "Πρόσεξε την κατεύθυνση: σε μικρότερη μονάδα ο αριθμός μεγαλώνει, σε μεγαλύτερη μικραίνει.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
    }
}

pub fn geo_angle(r: &mut Rng) -> Question {
    match r.r(0, 3) {
        0 => {
            let x = match r.r(0, 5) {
                0 => r.r(1, 89),
                1 => 90,
                2 => r.r(91, 179),
                3 => 180,
                4 => 360,
                _ => r.r(10, 170),
            };
            let kind = match x {
                90 => 1,
                180 => 3,
                360 => 4,
                x if x < 90 => 0,
                _ => 2,
            };
            let names = ["οξεία", "ορθή", "αμβλεία", "ευθεία", "πλήρης"];
            Question {
                cat: "geo-angle",
                prompt: format!(
                    "Τι είδους γωνία είναι μια γωνία <span class=\"big\">{}</span>;",
                    deg(x)
                ),
                groups: one(vec![select(
                    "Είδος:",
                    names
                        .iter()
                        .enumerate()
                        .map(|(i, n)| (i as i64, n.to_string()))
                        .collect(),
                    kind,
                )]),
                hints: hints(
                    "Οξεία: μικρότερη από 90°. Ορθή: 90°. Αμβλεία: από 90° έως 180°.",
                    "Ευθεία: 180°. Πλήρης: 360°.",
                    format!("Σύγκρινε το {} με το 90° και το 180°.", deg(x)),
                    format!("{} → {}", deg(x), names[kind as usize]),
                ),
                solution: format!("Η γωνία {} είναι <b>{}</b>.", deg(x), names[kind as usize]),
                diagnose: Box::new(|_| vec![tag("angle-kind")]),
            }
        }
        1 => {
            let comp = r.chance(0.5);
            let total = if comp { 90 } else { 180 };
            let x = r.r(5, total - 5);
            let word = if comp {
                "συμπληρωματική"
            } else {
                "παραπληρωματική"
            };
            Question {
                cat: "geo-angle",
                prompt: format!("Να βρεις την <b>{word}</b> μιας γωνίας {}.", deg(x)),
                groups: one(vec![num("Μοίρες:", total - x)]),
                hints: hints(
                    "Συμπληρωματικές: άθροισμα 90°. Παραπληρωματικές: άθροισμα 180°.",
                    format!("Ψάχνουμε γωνία ω με {} + ω = {}.", deg(x), deg(total)),
                    format!("ω = {total}° − {x}°"),
                    deg(total - x),
                ),
                solution: format!(
                    "{word} γωνίες έχουν άθροισμα {}: {total}° − {x}° = <b>{}</b>.",
                    deg(total),
                    deg(total - x)
                ),
                diagnose: Box::new(move |v| {
                    let other = if comp {
                        180 - x
                    } else {
                        90u64.saturating_sub(x)
                    };
                    if v[0].num() == Some(other as i64) && other != total - x {
                        vec![dm("comp-supp", "Μπέρδεψες συμπληρωματικές (άθροισμα 90°) με παραπληρωματικές (άθροισμα 180°).")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        2 => {
            // x = k · (complement or supplement of x)
            let (comp, k, x) = r.pick(&[
                (true, 2u64, 60u64),
                (true, 5, 75),
                (true, 8, 80),
                (true, 9, 81),
                (true, 4, 72),
                (false, 2, 120),
                (false, 3, 135),
                (false, 4, 144),
                (false, 5, 150),
                (false, 8, 160),
                (false, 9, 162),
            ]);
            let total = if comp { 90 } else { 180 };
            let word = if comp {
                "συμπληρωματικής"
            } else {
                "παραπληρωματικής"
            };
            let times = [
                "",
                "",
                "διπλάσια",
                "τριπλάσια",
                "τετραπλάσια",
                "πενταπλάσια",
                "",
                "",
                "οκταπλάσια",
                "εννιαπλάσια",
            ][k as usize];
            Question {
                cat: "geo-angle",
                prompt: format!("Μια γωνία ω είναι <b>{times}</b> της {word} της. Πόσες μοίρες είναι η ω;"),
                groups: one(vec![num("ω (μοίρες) =", x)]),
                hints: hints(
                    format!("Η {} της ω είναι {total}° − ω.", if comp { "συμπληρωματική" } else { "παραπληρωματική" }),
                    format!("Η εξίσωση είναι ω = {k}·({total}° − ω)."),
                    format!("ω + {k}ω = {}°, δηλαδή {}ω = {}°.", k * total, k + 1, k * total),
                    deg(x),
                ),
                solution: format!("ω = {k}({total} − ω) ⇔ ω = {} − {k}ω ⇔ {}ω = {} ⇔ <b>ω = {}</b>. Έλεγχος: {total}° − {x}° = {}°, και {k}·{} = {}.", k * total, k + 1, k * total, deg(x), total - x, total - x, x),
                diagnose: no_diag(),
            }
        }
        _ => {
            let x = r.r(10, 85) * 2;
            let bis = r.chance(0.5);
            if bis {
                Question {
                    cat: "geo-angle",
                    prompt: format!("Η ημιευθεία Οδ είναι η <b>διχοτόμος</b> της γωνίας xÔy = {}. Πόσες μοίρες είναι η γωνία xÔδ;", deg(x)),
                    groups: one(vec![num("Μοίρες:", x / 2)]),
                    hints: hints("Η διχοτόμος χωρίζει τη γωνία σε δύο ίσες γωνίες.", format!("{} : 2", deg(x)), "Μοίρασε στα δύο.", deg(x / 2)),
                    solution: format!("xÔδ = {} : 2 = <b>{}</b>.", deg(x), deg(x / 2)),
                    diagnose: no_diag(),
                }
            } else {
                let y = r.r(20, 160);
                let ask_vert = r.chance(0.5);
                let ans = if ask_vert { y } else { 180 - y };
                Question {
                    cat: "geo-angle",
                    prompt: format!("Δύο ευθείες τέμνονται και σχηματίζουν τις γωνίες α̂, β̂, γ̂, δ̂ (με αυτή τη σειρά γύρω από το σημείο τομής). Αν α̂ = {}, πόσο είναι η {};", deg(y), if ask_vert { "γ̂ (κατακορυφήν της α̂)" } else { "β̂ (εφεξής της α̂)" }),
                    groups: one(vec![num("Μοίρες:", ans)]),
                    hints: hints("Οι κατακορυφήν γωνίες είναι ίσες.", "Δύο εφεξής γωνίες πάνω σε ευθεία είναι παραπληρωματικές (άθροισμα 180°).", if ask_vert { "Η γ̂ είναι απέναντι από την α̂.".to_string() } else { format!("β̂ = 180° − {}", deg(y)) }, deg(ans)),
                    solution: if ask_vert { format!("Οι κατακορυφήν γωνίες είναι ίσες: <b>γ̂ = {}</b>.", deg(y)) } else { format!("α̂ + β̂ = 180°, άρα <b>β̂ = {}</b>.", deg(ans)) },
                    diagnose: no_diag(),
                }
            }
        }
    }
}

pub fn geo_circle(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let rad = r.r(2, 40);
            let ask_d = r.chance(0.5);
            Question {
                cat: "geo-circle",
                prompt: if ask_d {
                    format!(
                        "Ένας κύκλος (Κ, ρ) έχει ακτίνα ρ = {rad} cm. Πόσο είναι η διάμετρός του;"
                    )
                } else {
                    format!(
                        "Η διάμετρος ενός κύκλου είναι {} cm. Πόση είναι η ακτίνα του;",
                        2 * rad
                    )
                },
                groups: one(vec![num("cm:", if ask_d { 2 * rad } else { rad })]),
                hints: hints(
                    "Η διάμετρος είναι η χορδή που περνά από το κέντρο.",
                    "Η διάμετρος είναι διπλάσια από την ακτίνα: δ = 2ρ.",
                    if ask_d {
                        "Διπλασίασε."
                    } else {
                        "Μοίρασε στα δύο."
                    },
                    if ask_d {
                        format!("{} cm", 2 * rad)
                    } else {
                        format!("{rad} cm")
                    },
                ),
                solution: format!(
                    "δ = 2ρ, άρα {}.",
                    if ask_d {
                        format!("δ = 2·{rad} = <b>{} cm</b>", 2 * rad)
                    } else {
                        format!("ρ = {} : 2 = <b>{rad} cm</b>", 2 * rad)
                    }
                ),
                diagnose: no_diag(),
            }
        }
        1 => {
            let n = r.pick(&[3u64, 4, 5, 6, 8, 9, 10, 12, 18, 20, 24, 36]);
            Question {
                cat: "geo-circle",
                prompt: format!("Ένας κύκλος χωρίζεται σε <b>{n}</b> ίσα τόξα. Πόσων μοιρών είναι η επίκεντρη γωνία που βαίνει σε κάθε τόξο;"),
                groups: one(vec![num("Μοίρες:", 360 / n)]),
                hints: hints("Ολόκληρος ο κύκλος είναι 360°.", "Η επίκεντρη γωνία έχει τόσες μοίρες όσες και το τόξο στο οποίο βαίνει.", format!("360° : {n}"), deg(360 / n)),
                solution: format!("Κάθε τόξο είναι 360° : {n} = {}, άρα και η επίκεντρη γωνία είναι <b>{}</b>.", deg(360 / n), deg(360 / n)),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some((180 / n) as i64) {
                        vec![dm("circle-360", "Ολόκληρος ο κύκλος είναι <b>360°</b>, όχι 180°.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let items = [
                ("Η διάμετρος είναι η μεγαλύτερη χορδή ενός κύκλου.", true),
                ("Κάθε χορδή περνά από το κέντρο του κύκλου.", false),
                ("Όλες οι ακτίνες ενός κύκλου είναι ίσες.", true),
                (
                    "Η επίκεντρη γωνία έχει την κορυφή της πάνω στον κύκλο.",
                    false,
                ),
                ("Ένα τόξο 90° αντιστοιχεί σε επίκεντρη γωνία 90°.", true),
                ("Η διάμετρος είναι ίση με την ακτίνα.", false),
                ("Η επίκεντρη γωνία έχει κορυφή το κέντρο του κύκλου.", true),
                ("Ένα ημικύκλιο αντιστοιχεί σε τόξο 180°.", true),
            ];
            let (s, t) = r.pick(&items);
            Question {
                cat: "geo-circle",
                prompt: format!("Σωστό ή λάθος;<br><b>{s}</b>"),
                groups: one(vec![truefalse(t)]),
                hints: hints("Χορδή: τμήμα που ενώνει δύο σημεία του κύκλου. Διάμετρος: χορδή που περνά από το κέντρο.", "Επίκεντρη γωνία: γωνία με κορυφή το κέντρο του κύκλου.", "Ολόκληρος ο κύκλος είναι 360°.", if t { "Σωστό." } else { "Λάθος." }),
                solution: format!("Η πρόταση είναι <b>{}</b>.", if t { "σωστή" } else { "λάθος" }),
                diagnose: Box::new(|_| vec![tag("circle-terms")]),
            }
        }
    }
}

// ---------------- Κεφάλαιο 9 ----------------

/// Names and positions of the 8 angles, as in the book: α, β, γ, δ at Κ (top line), ε, ζ, η, θ at Μ.
/// Position: (at K?, above the line?, right of the transversal?)
const ANG: [(&str, bool, bool, bool); 8] = [
    ("α", true, true, false),
    ("β", true, true, true),
    ("γ", true, false, false),
    ("δ", true, false, true),
    ("ζ", false, true, false),
    ("ε", false, true, true),
    ("η", false, false, false),
    ("θ", false, false, true),
];

/// The transversal goes up to the right, so above-right and below-left angles are acute.
fn ang_val(i: usize, acute: u64) -> u64 {
    let (_, _, above, right) = ANG[i];
    if above == right {
        acute
    } else {
        180 - acute
    }
}

fn relation(i: usize, j: usize) -> Option<(usize, &'static str)> {
    let (_, ki, ai, ri) = ANG[i];
    let (_, kj, aj, rj) = ANG[j];
    if ki == kj {
        if ai != aj && ri != rj {
            return Some((3, "κατακορυφήν"));
        }
        return Some((4, "εφεξής (παραπληρωματικές)"));
    }
    // interior: below K's line or above M's line
    let inner = |k: bool, a: bool| if k { !a } else { a };
    let (ini, inj) = (inner(ki, ai), inner(kj, aj));
    if ai == aj && ri == rj {
        return Some((2, "εντός–εκτός και επί τα αυτά"));
    }
    if ini && inj && ri != rj {
        return Some((0, "εντός εναλλάξ"));
    }
    if ini && inj && ri == rj {
        return Some((1, "εντός και επί τα αυτά"));
    }
    None
}

fn par_svg(acute: u64) -> String {
    // lines ε1 (y=50) and ε2 (y=140); transversal through K and M
    let t = (acute as f64).to_radians();
    let (ky, my) = (50.0, 140.0);
    let dx = (my - ky) / t.tan();
    let kx = 170.0 + dx / 2.0;
    let mx = 170.0 - dx / 2.0;
    let ext = 45.0;
    let (x1, y1) = (mx - ext / t.tan(), my + ext);
    let (x2, y2) = (kx + ext / t.tan(), ky - ext);
    let mut labels = String::new();
    for (name, at_k, above, right) in ANG {
        let (cx, cy) = if at_k { (kx, ky) } else { (mx, my) };
        let ox = if right { 16.0 } else { -24.0 };
        let oy = if above { -8.0 } else { 20.0 };
        labels += &format!(
            "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"15\" font-style=\"italic\">{name}</text>",
            cx + ox,
            cy + oy
        );
    }
    format!(
        "<svg class=\"fig\" viewBox=\"0 0 340 200\" width=\"340\" role=\"img\" aria-label=\"Δύο παράλληλες ευθείες που τέμνονται από τρίτη ευθεία\" fill=\"currentColor\" stroke-linecap=\"round\">\
<line x1=\"10\" y1=\"50\" x2=\"330\" y2=\"50\" stroke=\"currentColor\" stroke-width=\"2\"/>\
<line x1=\"10\" y1=\"140\" x2=\"330\" y2=\"140\" stroke=\"currentColor\" stroke-width=\"2\"/>\
<line x1=\"{x1:.0}\" y1=\"{y1:.0}\" x2=\"{x2:.0}\" y2=\"{y2:.0}\" stroke=\"var(--accent)\" stroke-width=\"2\"/>\
<text x=\"312\" y=\"42\" font-size=\"14\">ε₁</text><text x=\"312\" y=\"132\" font-size=\"14\">ε₂</text><text x=\"{:.0}\" y=\"{:.0}\" font-size=\"14\">ε₃</text>\
<circle cx=\"{kx:.0}\" cy=\"{ky:.0}\" r=\"3\"/><circle cx=\"{mx:.0}\" cy=\"{my:.0}\" r=\"3\"/>\
<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"13\">Κ</text><text x=\"{:.0}\" y=\"{:.0}\" font-size=\"13\">Μ</text>{labels}</svg>",
        x2 + 6.0,
        y2 + 14.0,
        kx - 4.0,
        ky + 36.0,
        mx - 4.0,
        my - 26.0
    )
}

pub fn geo_par(r: &mut Rng) -> Question {
    let acute = r.r(35, 80);
    let rel_names = [
        "εντός εναλλάξ",
        "εντός και επί τα αυτά",
        "εντός–εκτός και επί τα αυτά",
        "κατακορυφήν",
        "εφεξής (παραπληρωματικές)",
    ];
    let (i, j, rel) = loop {
        let i = r.r(0, 7) as usize;
        let j = r.r(0, 7) as usize;
        if i == j {
            continue;
        }
        if let Some(rel) = relation(i, j) {
            // prefer the relations that need parallel lines
            if rel.0 <= 2 || r.chance(0.3) {
                break (i, j, rel);
            }
        }
    };
    let (gi, gj) = (ANG[i].0, ANG[j].0);
    let given = ang_val(i, acute);
    let ans = ang_val(j, acute);
    let fig = par_svg(acute);
    let rule = match rel.0 {
        0 | 2 | 3 => "είναι ίσες",
        _ => "είναι παραπληρωματικές (άθροισμα 180°)",
    };
    Question {
        cat: "geo-par",
        prompt: format!("Οι ευθείες ε₁ και ε₂ είναι <b>παράλληλες</b> και τέμνονται από την ευθεία ε₃.<br>{fig}<br>Αν <b>{gi}̂ = {}</b>:", deg(given)),
        groups: vec![
            group(format!("α) Πώς λέγονται οι γωνίες {gi}̂ και {gj}̂;"), vec![select("Ονομάζονται:", rel_names.iter().enumerate().map(|(k, n)| (k as i64, n.to_string())).collect(), rel.0 as i64)]),
            group(format!("β) Πόσες μοίρες είναι η {gj}̂;"), vec![num("Μοίρες:", ans)]),
        ],
        hints: hints(
            "«Εντός» είναι οι γωνίες ανάμεσα στις ε₁ και ε₂. «Εναλλάξ» σημαίνει σε διαφορετική πλευρά της ε₃, «επί τα αυτά» στην ίδια πλευρά.",
            "Με παράλληλες ευθείες: οι εντός εναλλάξ και οι εντός–εκτός και επί τα αυτά είναι ίσες, οι εντός και επί τα αυτά είναι παραπληρωματικές.",
            format!("Οι {gi}̂ και {gj}̂ είναι {}.", rel.1),
            format!("Οι {} γωνίες {rule}: {gj}̂ = {}.", rel.1, deg(ans)),
        ),
        solution: format!("Οι γωνίες {gi}̂ και {gj}̂ είναι <b>{}</b>, άρα {rule}. Επομένως <b>{gj}̂ = {}</b>.", rel.1, deg(ans)),
        diagnose: Box::new(move |v| {
            if v[1].num() == Some((180 - ans) as i64) && ans != 90 {
                vec![dm("par-angle", "Οι γωνίες που βρίσκεις είναι ή ίσες ή παραπληρωματικές. Δες στο σχήμα αν η ζητούμενη είναι οξεία ή αμβλεία.")]
            } else {
                vec![]
            }
        }),
    }
}

pub fn geo_tri(r: &mut Rng) -> Question {
    match r.r(0, 4) {
        0 => {
            let a = r.r(20, 110);
            let b = r.r(15, 160 - a);
            let c = 180 - a - b;
            Question {
                cat: "geo-tri",
                prompt: format!(
                    "Σε ένα τρίγωνο ΑΒΓ είναι Â = {} και B̂ = {}. Πόσες μοίρες είναι η Γ̂;",
                    deg(a),
                    deg(b)
                ),
                groups: one(vec![num("Γ̂ =", c)]),
                hints: hints(
                    "Το άθροισμα των γωνιών κάθε τριγώνου είναι 180°.",
                    format!("Γ̂ = 180° − ({a}° + {b}°)"),
                    format!("{a}° + {b}° = {}°", a + b),
                    deg(c),
                ),
                solution: format!(
                    "Â + B̂ + Γ̂ = 180°, άρα Γ̂ = 180° − {}° = <b>{}</b>.",
                    a + b,
                    deg(c)
                ),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some((360 - a - b) as i64) {
                        vec![dm(
                            "tri-180",
                            "Το άθροισμα των γωνιών τριγώνου είναι <b>180°</b>, όχι 360°.",
                        )]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        1 => {
            let a = r.r(25, 90);
            let b = r.r(20, 150 - a);
            let ext = a + b;
            Question {
                cat: "geo-tri",
                prompt: format!("Σε τρίγωνο ΑΒΓ είναι Â = {} και B̂ = {}. Πόσες μοίρες είναι η <b>εξωτερική</b> γωνία στην κορυφή Γ;", deg(a), deg(b)),
                groups: one(vec![num("Μοίρες:", ext)]),
                hints: hints(
                    "Η εξωτερική γωνία είναι παραπληρωματική της εσωτερικής γωνίας στην ίδια κορυφή.",
                    "Η εξωτερική γωνία ενός τριγώνου είναι ίση με το άθροισμα των δύο απέναντι εσωτερικών γωνιών.",
                    format!("{a}° + {b}°"),
                    deg(ext),
                ),
                solution: format!("Εξωτερική γωνία = Â + B̂ = {a}° + {b}° = <b>{}</b>. (Έλεγχος: Γ̂ = {}° και {}° + {}° = 180°.)", deg(ext), 180 - ext, 180 - ext, ext),
                diagnose: Box::new(move |v| if v[0].num() == Some((180 - ext) as i64) { vec![dm("tri-ext", "Βρήκες την εσωτερική γωνία Γ̂. Η εξωτερική είναι η παραπληρωματική της.")] } else { vec![] }),
            }
        }
        2 => {
            let base = r.r(20, 85);
            let top = 180 - 2 * base;
            let ask_base = r.chance(0.5);
            Question {
                cat: "geo-tri",
                prompt: if ask_base {
                    format!("Σε <b>ισοσκελές</b> τρίγωνο ΑΒΓ με ΑΒ = ΑΓ η γωνία της κορυφής είναι Â = {}. Πόσες μοίρες είναι κάθε γωνία της βάσης;", deg(top))
                } else {
                    format!("Σε <b>ισοσκελές</b> τρίγωνο ΑΒΓ με ΑΒ = ΑΓ είναι B̂ = {}. Πόσες μοίρες είναι η Â;", deg(base))
                },
                groups: one(vec![num("Μοίρες:", if ask_base { base } else { top })]),
                hints: hints(
                    "Στο ισοσκελές τρίγωνο οι γωνίες της βάσης είναι ίσες: B̂ = Γ̂.",
                    "Το άθροισμα των γωνιών είναι 180°.",
                    if ask_base {
                        format!("B̂ + Γ̂ = 180° − {top}° = {}°", 180 - top)
                    } else {
                        format!("Â = 180° − 2·{base}°")
                    },
                    deg(if ask_base { base } else { top }),
                ),
                solution: if ask_base {
                    format!("B̂ = Γ̂ = (180° − {top}°) : 2 = <b>{}</b>.", deg(base))
                } else {
                    format!(
                        "B̂ = Γ̂ = {}, άρα Â = 180° − 2·{base}° = <b>{}</b>.",
                        deg(base),
                        deg(top)
                    )
                },
                diagnose: no_diag(),
            }
        }
        3 => {
            let a = r.r(20, 120);
            let b = r.r(10, 160 - a);
            let c = 180 - a - b;
            let m = a.max(b).max(c);
            let kind = if m < 90 {
                0
            } else if m == 90 {
                1
            } else {
                2
            };
            let names = ["οξυγώνιο", "ορθογώνιο", "αμβλυγώνιο"];
            Question {
                cat: "geo-tri",
                prompt: format!("Ένα τρίγωνο έχει δύο γωνίες {} και {}. Τι είδους τρίγωνο είναι ως προς τις γωνίες του;", deg(a), deg(b)),
                groups: one(vec![select("Είδος:", names.iter().enumerate().map(|(i, n)| (i as i64, n.to_string())).collect(), kind)]),
                hints: hints("Βρες πρώτα την τρίτη γωνία.", format!("Τρίτη γωνία: 180° − {a}° − {b}° = {c}°"), "Οξυγώνιο: όλες < 90°. Ορθογώνιο: μία = 90°. Αμβλυγώνιο: μία > 90°.", names[kind as usize]),
                solution: format!("Τρίτη γωνία = {}. Η μεγαλύτερη γωνία είναι {}, άρα το τρίγωνο είναι <b>{}</b>.", deg(c), deg(m), names[kind as usize]),
                diagnose: Box::new(|_| vec![tag("tri-kind")]),
            }
        }
        _ => {
            let kind = r.r(0, 2);
            let (a, b, c) = match kind {
                0 => {
                    let s = r.r(3, 15);
                    (s, s, s)
                }
                1 => {
                    let s = r.r(4, 15);
                    (s, s, r.r(2, 2 * s - 1).max(1))
                }
                _ => loop {
                    let (a, b, c) = (r.r(3, 15), r.r(3, 15), r.r(3, 15));
                    if a != b && b != c && a != c && a + b > c && a + c > b && b + c > a {
                        break (a, b, c);
                    }
                },
            };
            let kind = if a == b && b == c {
                0
            } else if a == b || b == c || a == c {
                1
            } else {
                2
            };
            let names = ["ισόπλευρο", "ισοσκελές", "σκαληνό"];
            Question {
                cat: "geo-tri",
                prompt: format!("Ένα τρίγωνο έχει πλευρές {a} cm, {b} cm και {c} cm. Τι είδους τρίγωνο είναι ως προς τις πλευρές του;"),
                groups: one(vec![select("Είδος:", names.iter().enumerate().map(|(i, n)| (i as i64, n.to_string())).collect(), kind)]),
                hints: hints("Ισόπλευρο: τρεις ίσες πλευρές. Ισοσκελές: δύο ίσες. Σκαληνό: όλες διαφορετικές.", "Σύγκρινε τα μήκη.", format!("{a}, {b}, {c}"), names[kind as usize]),
                solution: format!("Οι πλευρές είναι {a}, {b}, {c} cm, άρα το τρίγωνο είναι <b>{}</b>.", names[kind as usize]),
                diagnose: Box::new(|_| vec![tag("tri-kind")]),
            }
        }
    }
}

pub fn geo_tri2(r: &mut Rng) -> Question {
    let names = ["διάμεσος", "ύψος", "διχοτόμος", "μεσοκάθετος"];
    match r.r(0, 1) {
        0 => {
            let defs = [
                ("Το ευθύγραμμο τμήμα που ενώνει μια κορυφή του τριγώνου με το μέσο της απέναντι πλευράς λέγεται:", 0),
                ("Το κάθετο ευθύγραμμο τμήμα από μια κορυφή του τριγώνου προς την ευθεία της απέναντι πλευράς λέγεται:", 1),
                ("Το τμήμα της ημιευθείας που χωρίζει μια γωνία του τριγώνου σε δύο ίσες γωνίες, από την κορυφή μέχρι την απέναντι πλευρά, λέγεται:", 2),
                ("Η ευθεία που είναι κάθετη σε μια πλευρά του τριγώνου και περνά από το μέσο της λέγεται:", 3),
            ];
            let (d, k) = r.pick(&defs);
            Question {
                cat: "geo-tri2",
                prompt: d.to_string(),
                groups: one(vec![select(
                    "Απάντηση:",
                    names
                        .iter()
                        .enumerate()
                        .map(|(i, n)| (i as i64, n.to_string()))
                        .collect(),
                    k,
                )]),
                hints: hints(
                    "Διάμεσος → μέσο απέναντι πλευράς.",
                    "Ύψος → κάθετο στην απέναντι πλευρά.",
                    "Διχοτόμος → μισή γωνία. Μεσοκάθετος → κάθετη στο μέσο πλευράς.",
                    names[k as usize],
                ),
                solution: format!("<b>{}</b>", names[k as usize]),
                diagnose: Box::new(|_| vec![tag("tri-lines")]),
            }
        }
        _ => {
            let (q, ans, sol) = match r.r(0, 2) {
                0 => {
                    let s = r.r(3, 20) * 2;
                    (format!("Στο τρίγωνο ΑΒΓ η ΑΜ είναι <b>διάμεσος</b> και ΒΓ = {s} cm. Πόσο είναι το ΒΜ (σε cm);"), s / 2, format!("Η διάμεσος ΑΜ καταλήγει στο μέσο Μ της ΒΓ, άρα ΒΜ = {s} : 2 = <b>{} cm</b>.", s / 2))
                }
                1 => {
                    let a = r.r(20, 70) * 2;
                    (format!("Στο τρίγωνο ΑΒΓ η ΑΔ είναι <b>διχοτόμος</b> της γωνίας Â = {}. Πόσες μοίρες είναι η γωνία ΒÂΔ;", deg(a)), a / 2, format!("Η διχοτόμος χωρίζει τη γωνία σε δύο ίσες: ΒÂΔ = {} : 2 = <b>{}</b>.", deg(a), deg(a / 2)))
                }
                _ => {
                    let b = r.r(25, 75);
                    (format!("Στο τρίγωνο ΑΒΓ το ΑΗ είναι <b>ύψος</b> και B̂ = {}. Πόσες μοίρες είναι η γωνία ΒÂΗ;", deg(b)), 90 - b, format!("Το ύψος σχηματίζει ορθή γωνία: ΑĤΒ = 90°. Στο τρίγωνο ΑΒΗ: ΒÂΗ = 180° − 90° − {b}° = <b>{}</b>.", deg(90 - b)))
                }
            };
            Question {
                cat: "geo-tri2",
                prompt: q,
                groups: one(vec![num("Απάντηση:", ans)]),
                hints: hints("Διάμεσος: πάει στο μέσο της απέναντι πλευράς. Διχοτόμος: χωρίζει τη γωνία στα δύο. Ύψος: σχηματίζει ορθή γωνία.", "Κάνε ένα πρόχειρο σχήμα.", "Χρησιμοποίησε την ιδιότητα του τμήματος που σου δίνεται.", sol.clone()),
                solution: sol,
                diagnose: no_diag(),
            }
        }
    }
}
