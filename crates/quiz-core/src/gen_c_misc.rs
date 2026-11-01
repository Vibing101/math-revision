//! Γ΄ Γυμνασίου: Κεφάλαιο 5 (Ευθεία – Γραμμικά Συστήματα), Κεφάλαιο 6 (Στατιστική),
//! Κεφάλαιο 7 (Στερεομετρία), Κεφάλαιο 8 (Παραβολή).

use crate::gen_c_alg::nz;
use crate::gen_c_geo::TRIPLES;
use crate::mk::*;
use crate::model::*;
use crate::util::*;

fn pt(name: &str, x: i64, y: i64) -> String {
    format!("{name}({}, {})", signed(x), signed(y))
}
/// "y = 2x − 3", "y = −x", "y = 4"
fn line(l: (i64, i64), b: (i64, i64)) -> String {
    let lam = match reduce(l.0, l.1) {
        (0, _) => String::new(),
        (1, 1) => "<i>x</i>".into(),
        (-1, 1) => "−<i>x</i>".into(),
        (n, d) => format!("{}<i>x</i>", fr(n, d)),
    };
    let (bn, bd) = reduce(b.0, b.1);
    let bs = if bn == 0 {
        String::new()
    } else if lam.is_empty() {
        fr(bn, bd)
    } else if bn < 0 {
        format!(" − {}", fr(-bn, bd))
    } else {
        format!(" + {}", fr(bn, bd))
    };
    let body = format!("{lam}{bs}");
    format!(
        "<i>y</i> = {}",
        if body.is_empty() { "0".into() } else { body }
    )
}

// ---------------- Κεφάλαιο 5 ----------------

pub fn c_dist(r: &mut Rng) -> Question {
    let (x1, y1) = (r.r(0, 16) as i64 - 8, r.r(0, 16) as i64 - 8);
    if r.chance(0.6) {
        let (dx, dy) = if r.chance(0.4) {
            let (a, b, _) = r.pick(&TRIPLES[..3]);
            (
                a * if r.chance(0.5) { 1 } else { -1 },
                b * if r.chance(0.5) { 1 } else { -1 },
            )
        } else {
            (nz(r, -7, 7), nz(r, -7, 7))
        };
        let (x2, y2) = (x1 + dx, y1 + dy);
        let d2 = dx * dx + dy * dy;
        let (sa, sb) = simplify_surd(1, d2);
        Question {
            cat: "c-dist",
            prompt: format!("Να βρεις την απόσταση των σημείων <b>{}</b> και <b>{}</b>.<br><span class=\"muted\">Αν δεν είναι ακέραιος, γράψε την ως απλοποιημένη ρίζα α√β.</span>", pt("Α", x1, y1), pt("Β", x2, y2)),
            groups: one(vec![surd("ΑΒ =", 1, d2)]),
            hints: hints(
                "ΑΒ = √((x₂ − x₁)² + (y₂ − y₁)²)",
                format!("x₂ − x₁ = {}, y₂ − y₁ = {}", signed(dx), signed(dy)),
                format!("ΑΒ = √({} + {}) = √{d2}", dx * dx, dy * dy),
                format!("ΑΒ = {}", surd_text(sa, sb)),
            ),
            solution: format!("ΑΒ = √({}² + {}²) = √{d2} = <b>{}</b>", par(dx), par(dy), surd_text(sa, sb)),
            diagnose: Box::new(move |v| match v[0] {
                Val::Surd(a, b) if a * a * b == (dx.abs() + dy.abs()).pow(2) => vec![dm("dist-sum", "Δεν αθροίζουμε τις διαφορές και μετά υψώνουμε. Κάθε διαφορά υψώνεται στο τετράγωνο <b>χωριστά</b>.")],
                _ => vec![],
            }),
        }
    } else {
        let (x2, y2) = (x1 + 2 * nz(r, -5, 5), y1 + 2 * nz(r, -5, 5));
        let (mx, my) = ((x1 + x2) / 2, (y1 + y2) / 2);
        Question {
            cat: "c-dist",
            prompt: format!("Να βρεις τις συντεταγμένες του μέσου Μ του ευθύγραμμου τμήματος με άκρα <b>{}</b> και <b>{}</b>.", pt("Α", x1, y1), pt("Β", x2, y2)),
            groups: one(vec![int("x<sub>Μ</sub> =", mx), int("y<sub>Μ</sub> =", my)]),
            hints: hints("Μ = ((x₁ + x₂)/2, (y₁ + y₂)/2)", format!("x<sub>Μ</sub> = ({} + {})/2", signed(x1), par(x2)), format!("y<sub>Μ</sub> = ({} + {})/2", signed(y1), par(y2)), pt("Μ", mx, my)),
            solution: format!("x<sub>Μ</sub> = ({} + {})/2 = {}, y<sub>Μ</sub> = ({} + {})/2 = {}. <b>{}</b>", signed(x1), par(x2), signed(mx), signed(y1), par(y2), signed(my), pt("Μ", mx, my)),
            diagnose: Box::new(move |v| if v[0].num() == Some((x2 - x1) / 2) && v[1].num() == Some((y2 - y1) / 2) && (x1 != 0 || y1 != 0) { vec![dm("midpoint", "Για το μέσο <b>προσθέτουμε</b> τις συντεταγμένες και διαιρούμε με το 2.")] } else { vec![] }),
        }
    }
}

pub fn c_line(r: &mut Rng) -> Question {
    match r.r(0, 3) {
        0 => {
            let (x1, y1) = (r.r(0, 12) as i64 - 6, r.r(0, 12) as i64 - 6);
            let (dx, dy) = (nz(r, -6, 6), r.r(0, 12) as i64 - 6);
            let (x2, y2) = (x1 + dx, y1 + dy);
            let (n, d) = reduce(dy, dx);
            Question {
                cat: "c-line",
                prompt: format!("Να βρεις την κλίση λ της ευθείας που περνά από τα σημεία <b>{}</b> και <b>{}</b>.", pt("Α", x1, y1), pt("Β", x2, y2)),
                groups: one(vec![frac("λ =", n, d)]),
                hints: hints("λ = (y₂ − y₁)/(x₂ − x₁)", format!("y₂ − y₁ = {}", signed(dy)), format!("x₂ − x₁ = {}", signed(dx)), format!("λ = {}", fr(n, d))),
                solution: format!("λ = ({} − {})/({} − {}) = {} = <b>{}</b>", signed(y2), par(y1), signed(x2), par(x1), fr_raw(dy, dx), fr(n, d)),
                diagnose: Box::new(move |v| match v[0] { Val::Frac(a, b) if reduce(a, b) == reduce(dx, dy) && dx != dy => vec![dm("slope-swap", "Αντέστρεψες τον λόγο: η κλίση είναι (διαφορά των y) / (διαφορά των x).")], _ => vec![] }),
            }
        }
        1 => {
            let (lam, b) = (nz(r, -5, 5), r.r(0, 16) as i64 - 8);
            let (x1, x2) = (r.r(0, 4) as i64 - 3, r.r(0, 4) as i64 + 2);
            let (y1, y2) = (lam * x1 + b, lam * x2 + b);
            Question {
                cat: "c-line",
                prompt: format!("Να βρεις την εξίσωση της ευθείας που περνά από τα σημεία <b>{}</b> και <b>{}</b>.", pt("Α", x1, y1), pt("Β", x2, y2)),
                groups: one(vec![slots(&["<i>y</i> = ", "<i>x</i> + ", ""], vec![lam, b], false)]),
                hints: hints("Η εξίσωση είναι y = λx + β. Βρες πρώτα την κλίση λ.", format!("λ = ({} − {})/({} − {}) = {}", signed(y2), par(y1), signed(x2), par(x1), signed(lam)), format!("Αντικατέστησε το Α: {} = {}·{} + β", signed(y1), signed(lam), par(x1)), line((lam, 1), (b, 1))),
                solution: format!("λ = {}. Από το Α: {} = {}·{} + β ⇒ β = {}. <b>{}</b>", signed(lam), signed(y1), signed(lam), par(x1), signed(b), line((lam, 1), (b, 1))),
                diagnose: no_diag(),
            }
        }
        2 => {
            if r.chance(0.5) {
                let (a, c) = (r.r(0, 8) as i64 - 4, nz(r, -6, 6));
                let k = c - a;
                let (b, d) = (nz(r, -9, 9), nz(r, -9, 9));
                Question {
                    cat: "c-line",
                    prompt: format!("Οι ευθείες <b><i>y</i> = (κ {} {})<i>x</i> {} {}</b> και <b>{}</b> είναι <b>παράλληλες</b>. Να βρεις το κ.", if a < 0 { "−" } else { "+" }, a.abs(), if b < 0 { "−" } else { "+" }, b.abs(), line((c, 1), (d, 1))),
                    groups: one(vec![int("κ =", k)]),
                    hints: hints("Δύο ευθείες είναι παράλληλες όταν έχουν την ίδια κλίση.", format!("κ {} {} = {}", if a < 0 { "−" } else { "+" }, a.abs(), signed(c)), "Λύσε ως προς κ.", format!("κ = {}", signed(k))),
                    solution: format!("Ίσες κλίσεις: κ {} {} = {} ⇒ <b>κ = {}</b>.", if a < 0 { "−" } else { "+" }, a.abs(), signed(c), signed(k)),
                    diagnose: no_diag(),
                }
            } else {
                let (p, q) = (nz(r, -6, 6), r.r(1, 5) as i64);
                let (p, q) = reduce(p, q);
                let (pn, pd) = reduce(-q, p);
                Question {
                    cat: "c-line",
                    prompt: format!("Μια ευθεία έχει κλίση λ = {}. Ποια είναι η κλίση μιας ευθείας <b>κάθετης</b> σε αυτήν;", fr(p, q)),
                    groups: one(vec![frac("λ′ =", pn, pd)]),
                    hints: hints("Δύο ευθείες είναι κάθετες όταν λ₁·λ₂ = −1.", format!("{}·λ′ = −1", fr(p, q)), "Η κάθετη κλίση είναι η αντίθετη αντίστροφη.", format!("λ′ = {}", fr(pn, pd))),
                    solution: format!("λ·λ′ = −1 ⇒ λ′ = −1 : {} = <b>{}</b>", fr(p, q), fr(pn, pd)),
                    diagnose: Box::new(move |v| match v[0] { Val::Frac(a, b) if reduce(a, b) == reduce(q, p) => vec![dm("perp", "Σωστό το αντίστροφο, αλλά η κάθετη κλίση έχει και <b>αντίθετο πρόσημο</b>.")], _ => vec![] }),
                }
            }
        }
        _ => {
            let l1 = nz(r, -4, 4);
            let b1 = r.r(0, 10) as i64 - 5;
            let kind = r.r(0, 3);
            // second line: slope as a fraction, intercept
            let (s2, b2): ((i64, i64), i64) = match kind {
                0 => ((l1, 1), b1 + nz(r, -4, 4)),
                1 => ((l1, 1), b1),
                2 => {
                    let mut l2 = nz(r, -4, 4);
                    while l2 == l1 || l1 * l2 == -1 {
                        l2 += 1;
                    }
                    ((l2, 1), r.r(0, 10) as i64 - 5)
                }
                _ => (reduce(-1, l1), r.r(0, 10) as i64 - 5),
            };
            let names = [
                "Παράλληλες",
                "Ταυτίζονται",
                "Τέμνονται (όχι κάθετα)",
                "Τέμνονται κάθετα",
            ];
            Question {
                cat: "c-line",
                prompt: format!(
                    "Ποια είναι η σχετική θέση των ευθειών <b>ε₁: {}</b> και <b>ε₂: {}</b>;",
                    line((l1, 1), (b1, 1)),
                    line(s2, (b2, 1))
                ),
                groups: one(vec![select(
                    "Θέση:",
                    names
                        .iter()
                        .enumerate()
                        .map(|(i, n)| (i as i64, n.to_string()))
                        .collect(),
                    kind as i64,
                )]),
                hints: hints(
                    "Σύγκρινε τις κλίσεις λ₁ και λ₂.",
                    "Ίδια κλίση και ίδιο β: ταυτίζονται. Ίδια κλίση, διαφορετικό β: παράλληλες.",
                    "Διαφορετικές κλίσεις: τέμνονται. Αν λ₁·λ₂ = −1, τέμνονται κάθετα.",
                    names[kind as usize],
                ),
                solution: format!(
                    "λ₁ = {}, λ₂ = {}. <b>{}</b>.",
                    signed(l1),
                    fr(s2.0, s2.1),
                    names[kind as usize]
                ),
                diagnose: Box::new(|_| vec![tag("line-pos")]),
            }
        }
    }
}

pub fn c_system(r: &mut Rng) -> Question {
    let (x, y) = (r.r(0, 16) as i64 - 8, r.r(0, 16) as i64 - 8);
    let (a1, b1, a2, b2) = loop {
        let (a1, b1, a2, b2) = (nz(r, -5, 5), nz(r, -5, 5), nz(r, -5, 5), nz(r, -5, 5));
        if a1 * b2 - a2 * b1 != 0 {
            break (a1, b1, a2, b2);
        }
    };
    let (c1, c2) = (a1 * x + b1 * y, a2 * x + b2 * y);
    let e = |a: i64, b: i64, c: i64| {
        format!(
            "{} = {}",
            crate::gen_c_alg::poly(&[(a, 1)], "x")
                + &if b < 0 {
                    format!(" − {}", crate::gen_c_alg::poly(&[(-b, 1)], "y"))
                } else {
                    format!(" + {}", crate::gen_c_alg::poly(&[(b, 1)], "y"))
                },
            signed(c)
        )
    };
    Question {
        cat: "c-system",
        prompt: format!("Λύσε το σύστημα:<br><span class=\"big\" style=\"font-size:1.2rem\">{}<br>{}</span>", e(a1, b1, c1), e(a2, b2, c2)),
        groups: one(vec![int("x =", x), int("y =", y)]),
        hints: hints(
            "Μέθοδος αντικατάστασης: λύσε μια εξίσωση ως προς έναν άγνωστο και αντικατέστησε στην άλλη. Ή μέθοδος αντίθετων συντελεστών.",
            format!("Αντίθετοι συντελεστές: πολλαπλασίασε την 1η με {} και τη 2η με {} ώστε να φύγει το y.", b2, -b1),
            format!("Βρίσκεις {}x = {}", a1 * b2 - a2 * b1, c1 * b2 - c2 * b1),
            format!("x = {}, y = {}", signed(x), signed(y)),
        ),
        solution: format!("Με αντίθετους συντελεστές: ({})x = {} ⇒ x = {}. Αντικαθιστούμε και βρίσκουμε y = {}. Επαλήθευση: {}·{} + {}·{} = {} ✓<br><b>(x, y) = ({}, {})</b>", a1 * b2 - a2 * b1, c1 * b2 - c2 * b1, signed(x), signed(y), signed(a1), par(x), signed(b1), par(y), signed(c1), signed(x), signed(y)),
        diagnose: Box::new(move |v| if v[0].num() == Some(y) && v[1].num() == Some(x) && x != y { vec![dm("sys-swap", "Αντέστρεψες τα x και y. Κάνε επαλήθευση στην πρώτη εξίσωση.")] } else { vec![] }),
    }
}

// ---------------- Κεφάλαιο 6 ----------------

pub fn c_sample(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let n = r.pick(&[50u64, 80, 100, 120, 200, 250, 400]);
            let big = n * r.r(10, 60);
            let k = loop {
                let k = r.r(5, n - 5);
                if (big * k).is_multiple_of(n) {
                    break k;
                }
            };
            let ctx = r.pick(&[
                ("Σε έρευνα σε τυχαίο δείγμα {n} μαθητών μιας επαρχίας, οι {k} δήλωσαν ότι έρχονται στο σχολείο με τα πόδια. Στην επαρχία υπάρχουν {N} μαθητές.", "μαθητές της επαρχίας εκτιμάται ότι έρχονται με τα πόδια"),
                ("Ένα εργοστάσιο ελέγχει τυχαίο δείγμα {n} λαμπτήρων και βρίσκει {k} ελαττωματικούς. Την εβδομάδα παράγει {N} λαμπτήρες.", "ελαττωματικοί λαμπτήρες εκτιμάται ότι παράγονται την εβδομάδα"),
                ("Σε δημοσκόπηση σε τυχαίο δείγμα {n} κατοίκων μιας πόλης, οι {k} δήλωσαν ότι ανακυκλώνουν. Η πόλη έχει {N} κατοίκους.", "κάτοικοι εκτιμάται ότι ανακυκλώνουν"),
            ]);
            let ans = big * k / n;
            Question {
                cat: "c-sample",
                prompt: format!("{} Πόσοι {};", ctx.0.replace("{n}", &n.to_string()).replace("{k}", &k.to_string()).replace("{N}", &thousands(big)), ctx.1),
                groups: one(vec![num("Εκτίμηση:", ans)]),
                hints: hints("Υποθέτουμε ότι το δείγμα είναι αντιπροσωπευτικό: το ίδιο ποσοστό ισχύει για όλο τον πληθυσμό.", format!("Ποσοστό στο δείγμα: {k}/{n}"), format!("{}·{k}/{n}", thousands(big)), thousands(ans)),
                solution: format!("Στο δείγμα: {}. Εκτίμηση για τον πληθυσμό: {}·{} = <b>{}</b>.", fr_raw(k as i64, n as i64), thousands(big), fr_raw(k as i64, n as i64), thousands(ans)),
                diagnose: no_diag(),
            }
        }
        1 => {
            let opts = vec![
                "Επιλέγουμε τυχαία μαθητές από όλες τις τάξεις και όλα τα τμήματα".to_string(),
                "Ρωτάμε μόνο τους μαθητές ενός τμήματος".to_string(),
                "Ρωτάμε μόνο τους φίλους μας".to_string(),
                "Ρωτάμε μόνο τα μέλη του ομίλου πληροφορικής".to_string(),
            ];
            Question {
                cat: "c-sample",
                prompt: "Θέλουμε να εκτιμήσουμε πόσες ώρες την εβδομάδα περνούν στο διαδίκτυο οι μαθητές ενός σχολείου. Ποιος τρόπος δειγματοληψίας δίνει <b>αντιπροσωπευτικό</b> δείγμα;".into(),
                groups: one(vec![choice(r, "Δείγμα:", &opts)]),
                hints: hints("Αντιπροσωπευτικό δείγμα: μοιάζει με όλο τον πληθυσμό.", "Κάθε μέλος του πληθυσμού πρέπει να έχει την ίδια πιθανότητα να επιλεγεί.", "Ποια επιλογή ευνοεί μια συγκεκριμένη ομάδα;", opts[0].clone()),
                solution: format!("<b>{}</b>. Οι άλλοι τρόποι ευνοούν μια ομάδα (ένα τμήμα, τους φίλους, όσους ασχολούνται με υπολογιστές) και δίνουν μεροληπτικό δείγμα.", opts[0]),
                diagnose: Box::new(|_| vec![tag("sample-bias")]),
            }
        }
        _ => {
            let n = r.pick(&[5usize, 6, 7, 8]);
            let data: Vec<u64> = (0..n).map(|_| r.r(1, 20)).collect();
            let sum: u64 = data.iter().sum();
            let mean = sum as f64 / n as f64;
            let mut sorted = data.clone();
            sorted.sort_unstable();
            let med = if n % 2 == 1 {
                sorted[n / 2] as f64
            } else {
                (sorted[n / 2 - 1] + sorted[n / 2]) as f64 / 2.0
            };
            let ask_mean = r.chance(0.5);
            Question {
                cat: "c-sample",
                prompt: format!("Οι βαθμοί ενός δείγματος μαθητών σε ένα τεστ είναι: <b>{}</b>. Να βρεις τη {} (με δύο το πολύ δεκαδικά).", list(&data), if ask_mean { "μέση τιμή" } else { "διάμεσο" }),
                groups: one(vec![dec(if ask_mean { "Μέση τιμή ≈" } else { "Διάμεσος =" }, if ask_mean { (mean * 100.0).round() / 100.0 } else { med }, 0.011)]),
                hints: hints(
                    if ask_mean { "Μέση τιμή = άθροισμα τιμών : πλήθος τιμών." } else { "Διάμεσος: βάλε τις τιμές σε σειρά. Είναι η μεσαία τιμή (ή ο μέσος όρος των δύο μεσαίων)." },
                    if ask_mean { format!("Άθροισμα: {sum}, πλήθος: {n}") } else { format!("Σε σειρά: {}", list(&sorted)) },
                    if ask_mean { format!("{sum} : {n}") } else if n % 2 == 1 { format!("Η {}η τιμή", n / 2 + 1) } else { format!("({} + {})/2", sorted[n / 2 - 1], sorted[n / 2]) },
                    dec_text(if ask_mean { (mean * 100.0).round() / 100.0 } else { med }),
                ),
                solution: if ask_mean { format!("x̄ = {sum} : {n} ≈ <b>{}</b>", dec_text((mean * 100.0).round() / 100.0)) } else { format!("Σε σειρά: {}. Διάμεσος = <b>{}</b>", list(&sorted), dec_text(med)) },
                diagnose: Box::new(move |v| match v[0] { Val::Dec(u) if !ask_mean && (u - data[n / 2] as f64).abs() < 1e-9 && (u - med).abs() > 1e-9 => vec![dm("median-sort", "Για τη διάμεσο βάζουμε <b>πρώτα</b> τις τιμές σε σειρά από τη μικρότερη στη μεγαλύτερη.")], _ => vec![] }),
            }
        }
    }
}

// ---------------- Κεφάλαιο 7 ----------------

pub fn c_solid(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let a = r.r(2, 12);
            Question {
                cat: "c-solid",
                prompt: format!("Ένας κύβος έχει ακμή α = {a} cm. Να βρεις το εμβαδόν ολικής επιφάνειας και τον όγκο του."),
                groups: vec![group("", vec![num("Ε<sub>ολ</sub> (cm²) =", 6 * a * a), num("V (cm³) =", a * a * a)])],
                hints: hints("Ο κύβος έχει 6 ίσες τετράγωνες έδρες.", "Ε<sub>ολ</sub> = 6α², V = α³", format!("6·{a}² και {a}³"), format!("Ε = {}, V = {}", 6 * a * a, a * a * a)),
                solution: format!("Ε<sub>ολ</sub> = 6·{a}² = <b>{} cm²</b>, V = {a}³ = <b>{} cm³</b>", 6 * a * a, a * a * a),
                diagnose: no_diag(),
            }
        }
        1 => {
            let (a, b, c) = (r.r(2, 12), r.r(2, 12), r.r(2, 12));
            Question {
                cat: "c-solid",
                prompt: format!("Ένα ορθογώνιο παραλληλεπίπεδο έχει διαστάσεις {a} cm, {b} cm και {c} cm. Να βρεις το εμβαδόν ολικής επιφάνειας και τον όγκο του."),
                groups: vec![group("", vec![num("Ε<sub>ολ</sub> (cm²) =", 2 * (a * b + b * c + a * c)), num("V (cm³) =", a * b * c)])],
                hints: hints("Έχει 6 έδρες, ανά δύο ίσες.", "Ε<sub>ολ</sub> = 2(αβ + βγ + αγ), V = αβγ", format!("2({} + {} + {})", a * b, b * c, a * c), format!("Ε = {}, V = {}", 2 * (a * b + b * c + a * c), a * b * c)),
                solution: format!("Ε<sub>ολ</sub> = 2({}·{} + {}·{} + {}·{}) = <b>{} cm²</b>, V = {a}·{b}·{c} = <b>{} cm³</b>", a, b, b, c, a, c, 2 * (a * b + b * c + a * c), a * b * c),
                diagnose: no_diag(),
            }
        }
        _ => {
            // square pyramid: half side, height, apothem form a right triangle
            let (h0, u0, ap0) = r.pick(&[
                (3i64, 4i64, 5i64),
                (4, 3, 5),
                (5, 12, 13),
                (6, 8, 10),
                (8, 6, 10),
                (12, 5, 13),
                (8, 15, 17),
            ]);
            let alpha = 2 * h0;
            let (v_num, v_den) = reduce(alpha * alpha * u0, 3);
            let e = alpha * alpha + 2 * alpha * ap0;
            Question {
                cat: "c-solid",
                prompt: format!("Μια κανονική τετραγωνική πυραμίδα έχει πλευρά βάσης α = {alpha} cm και ύψος υ = {u0} cm. Να βρεις το απόστημα h, το εμβαδόν ολικής επιφάνειας και τον όγκο της."),
                groups: vec![
                    group("α) Απόστημα", vec![num("h (cm) =", ap0 as u64)]),
                    group("β) Εμβαδόν ολικής επιφάνειας", vec![num("Ε<sub>ολ</sub> (cm²) =", e as u64)]),
                    group("γ) Όγκος", vec![frac("V (cm³) =", v_num, v_den)]),
                ],
                hints: hints(
                    format!("Το ύψος, το μισό της πλευράς ({h0}) και το απόστημα σχηματίζουν ορθογώνιο τρίγωνο."),
                    format!("h² = υ² + (α/2)² = {} + {} = {}", u0 * u0, h0 * h0, ap0 * ap0),
                    "Ε<sub>ολ</sub> = α² + 4·(α·h/2) = α² + 2αh, V = (1/3)·α²·υ",
                    format!("h = {ap0}, Ε = {e}, V = {}", fr(v_num, v_den)),
                ),
                solution: format!("h = √({u0}² + {h0}²) = <b>{ap0} cm</b>. Ε<sub>ολ</sub> = {alpha}² + 2·{alpha}·{ap0} = <b>{e} cm²</b>. V = (1/3)·{alpha}²·{u0} = <b>{} cm³</b>.", fr(v_num, v_den)),
                diagnose: Box::new(move |v| match v[2] { Val::Frac(n, d) if reduce(n, d) == (alpha * alpha * u0, 1) => vec![dm("cone-third", "Ο όγκος πυραμίδας (και κώνου) είναι το <b>1/3</b> του εμβαδού βάσης επί το ύψος.")], _ => vec![] }),
            }
        }
    }
}

pub fn c_round(r: &mut Rng) -> Question {
    let note = "<br><span class=\"muted\">Οι απαντήσεις συναρτήσει του π: γράψε τον αριθμό που πολλαπλασιάζει το π (μπορεί να είναι κλάσμα).</span>";
    match r.r(0, 2) {
        0 => {
            let (rr, h) = (r.r(1, 10) as i64, r.r(2, 15) as i64);
            Question {
                cat: "c-round",
                prompt: format!("Ένας κύλινδρος έχει ακτίνα βάσης ρ = {rr} cm και ύψος υ = {h} cm. Να βρεις το εμβαδόν ολικής επιφάνειας και τον όγκο του.{note}"),
                groups: vec![group("Ε<sub>ολ</sub> = □·π cm²", vec![frac("Συντελεστής του π:", 2 * rr * h + 2 * rr * rr, 1)]), group("V = □·π cm³", vec![frac("Συντελεστής του π:", rr * rr * h, 1)])],
                hints: hints("Ε<sub>ολ</sub> = 2πρυ + 2πρ² (παράπλευρη + δύο βάσεις)", "V = πρ²υ", format!("2π·{rr}·{h} + 2π·{rr}² = {}π + {}π", 2 * rr * h, 2 * rr * rr), format!("Ε = {}π, V = {}π", 2 * rr * h + 2 * rr * rr, rr * rr * h)),
                solution: format!("Ε<sub>ολ</sub> = 2π·{rr}·{h} + 2π·{rr}² = <b>{}π cm²</b>. V = π·{rr}²·{h} = <b>{}π cm³</b>.", 2 * rr * h + 2 * rr * rr, rr * rr * h),
                diagnose: Box::new(move |v| match v[0] { Val::Frac(n, 1) if n == 2 * rr * h + rr * rr || n == 2 * rr * h => vec![dm("solid-bases", "Στην ολική επιφάνεια του κυλίνδρου μετρούν <b>δύο</b> βάσεις, 2πρ².")], _ => vec![] }),
            }
        }
        1 => {
            let (a, b, c) = r.pick(&TRIPLES[..6]);
            let (rr, h, l) = if r.chance(0.5) { (a, b, c) } else { (b, a, c) };
            let (vn, vd) = reduce(rr * rr * h, 3);
            Question {
                cat: "c-round",
                prompt: format!("Ένας κώνος έχει ακτίνα βάσης ρ = {rr} cm και ύψος υ = {h} cm. Να βρεις τη γενέτειρα λ, το εμβαδόν ολικής επιφάνειας και τον όγκο του.{note}"),
                groups: vec![
                    group("α) Γενέτειρα", vec![num("λ (cm) =", l as u64)]),
                    group("β) Ε<sub>ολ</sub> = □·π cm²", vec![frac("Συντελεστής του π:", rr * l + rr * rr, 1)]),
                    group("γ) V = □·π cm³", vec![frac("Συντελεστής του π:", vn, vd)]),
                ],
                hints: hints("Το ύψος, η ακτίνα και η γενέτειρα σχηματίζουν ορθογώνιο τρίγωνο: λ² = ρ² + υ².", format!("λ² = {} + {} = {}", rr * rr, h * h, l * l), "Ε<sub>ολ</sub> = πρλ + πρ², V = (1/3)πρ²υ", format!("λ = {l}, Ε = {}π, V = {}π", rr * l + rr * rr, fr(vn, vd))),
                solution: format!("λ = √({rr}² + {h}²) = <b>{l} cm</b>. Ε<sub>ολ</sub> = π·{rr}·{l} + π·{rr}² = <b>{}π cm²</b>. V = (1/3)π·{rr}²·{h} = <b>{}π cm³</b>.", rr * l + rr * rr, fr(vn, vd)),
                diagnose: Box::new(move |v| match v[2] { Val::Frac(n, d) if reduce(n, d) == (rr * rr * h, 1) => vec![dm("cone-third", "Ο όγκος του κώνου είναι το <b>1/3</b> του όγκου κυλίνδρου με ίδια βάση και ύψος.")], _ => vec![] }),
            }
        }
        _ => {
            let rr = r.r(1, 9) as i64;
            let (vn, vd) = reduce(4 * rr.pow(3), 3);
            Question {
                cat: "c-round",
                prompt: format!("Μια σφαίρα έχει ακτίνα ρ = {rr} cm. Να βρεις το εμβαδόν της επιφάνειας και τον όγκο της.{note}"),
                groups: vec![group("Ε = □·π cm²", vec![frac("Συντελεστής του π:", 4 * rr * rr, 1)]), group("V = □·π cm³", vec![frac("Συντελεστής του π:", vn, vd)])],
                hints: hints("Ε = 4πρ²", "V = (4/3)πρ³", format!("4·{rr}² = {} και (4/3)·{rr}³ = {}", 4 * rr * rr, fr(vn, vd)), format!("Ε = {}π, V = {}π", 4 * rr * rr, fr(vn, vd))),
                solution: format!("Ε = 4π·{rr}² = <b>{}π cm²</b>. V = (4/3)π·{rr}³ = <b>{}π cm³</b>.", 4 * rr * rr, fr(vn, vd)),
                diagnose: no_diag(),
            }
        }
    }
}

// ---------------- Κεφάλαιο 8 ----------------

pub fn c_parab(r: &mut Rng) -> Question {
    let a = {
        let n = nz(r, -4, 4);
        let d = r.pick(&[1i64, 1, 2, 3, 4]);
        reduce(n, d)
    };
    let ay = |x: i64| reduce(a.0 * x * x, a.1);
    let ytext = format!(
        "<i>y</i> = {}<i>x</i>²",
        if a == (1, 1) {
            String::new()
        } else if a == (-1, 1) {
            "−".into()
        } else {
            fr(a.0, a.1)
        }
    );
    match r.r(0, 3) {
        0 => {
            let x0 = nz(r, -4, 4);
            let y0 = ay(x0);
            Question {
                cat: "c-parab",
                prompt: format!("Η παραβολή <i>y</i> = α<i>x</i>² περνά από το σημείο <b>Α({}, {})</b>. Να βρεις το α.", signed(x0), fr(y0.0, y0.1)),
                groups: one(vec![frac("α =", a.0, a.1)]),
                hints: hints("Αντικατέστησε τις συντεταγμένες του σημείου στον τύπο.", format!("{} = α·{}²", fr(y0.0, y0.1), par(x0)), format!("{} = {}α", fr(y0.0, y0.1), x0 * x0), format!("α = {}", fr(a.0, a.1))),
                solution: format!("{} = α·{}² ⇒ α = {} : {} = <b>{}</b>", fr(y0.0, y0.1), par(x0), fr(y0.0, y0.1), x0 * x0, fr(a.0, a.1)),
                diagnose: Box::new(move |v| match v[0] { Val::Frac(n, d) if reduce(n, d) == reduce(-a.0, a.1) => vec![dm("neg-pow", "Πρόσεξε: (−x)² είναι θετικό.")], _ => vec![] }),
            }
        }
        1 => {
            let x0 = nz(r, -4, 4);
            let on = r.chance(0.5);
            let y = if on {
                ay(x0)
            } else {
                reduce(ay(x0).0 + ay(x0).1 * nz(r, -2, 2), ay(x0).1)
            };
            Question {
                cat: "c-parab",
                prompt: format!(
                    "Ανήκει το σημείο <b>Β({}, {})</b> στην παραβολή <b>{ytext}</b>;",
                    signed(x0),
                    fr(y.0, y.1)
                ),
                groups: one(vec![yesno(on)]),
                hints: hints(
                    "Βάλε τη x-συντεταγμένη στον τύπο και σύγκρινε με την y.",
                    format!("x = {}", signed(x0)),
                    format!("y = {}", fr(ay(x0).0, ay(x0).1)),
                    if on {
                        "Ίσο: ανήκει."
                    } else {
                        "Διαφορετικό: δεν ανήκει."
                    },
                ),
                solution: format!(
                    "Για x = {}: y = {}. {}",
                    signed(x0),
                    fr(ay(x0).0, ay(x0).1),
                    if on {
                        "Είναι ίσο με την τεταγμένη του Β, άρα <b>ανήκει</b>."
                    } else {
                        "Δεν είναι ίσο με την τεταγμένη του Β, άρα <b>δεν ανήκει</b>."
                    }
                ),
                diagnose: no_diag(),
            }
        }
        2 => {
            let opts = [
                "Ανοίγει προς τα πάνω και έχει ελάχιστο στο (0, 0)",
                "Ανοίγει προς τα κάτω και έχει μέγιστο στο (0, 0)",
                "Ανοίγει προς τα πάνω και έχει μέγιστο στο (0, 0)",
                "Ανοίγει προς τα κάτω και έχει ελάχιστο στο (0, 0)",
            ];
            let k = if a.0 > 0 { 0 } else { 1 };
            Question {
                cat: "c-parab",
                prompt: format!("Ποια πρόταση ισχύει για την παραβολή <b>{ytext}</b>;"),
                groups: one(vec![select(
                    "Πρόταση:",
                    opts.iter()
                        .enumerate()
                        .map(|(i, o)| (i as i64, o.to_string()))
                        .collect(),
                    k,
                )]),
                hints: hints(
                    "Η κορυφή της y = αx² είναι πάντα το (0, 0).",
                    "Αν α &gt; 0 ανοίγει προς τα πάνω, αν α &lt; 0 προς τα κάτω.",
                    format!("Εδώ α = {}.", fr(a.0, a.1)),
                    opts[k as usize],
                ),
                solution: format!(
                    "α = {} {} 0, άρα <b>{}</b>.",
                    fr(a.0, a.1),
                    if a.0 > 0 { "&gt;" } else { "&lt;" },
                    opts[k as usize].to_lowercase()
                ),
                diagnose: Box::new(|_| vec![tag("parab-shape")]),
            }
        }
        _ => {
            let x0 = nz(r, -5, 5);
            let y = ay(x0);
            Question {
                cat: "c-parab",
                prompt: format!(
                    "Για την παραβολή <b>{ytext}</b>, να βρεις την τιμή του y για x = {}.",
                    signed(x0)
                ),
                groups: one(vec![frac("y =", y.0, y.1)]),
                hints: hints(
                    "Αντικατέστησε το x.",
                    format!("y = {}·{}²", fr(a.0, a.1), par(x0)),
                    format!("{}² = {}", par(x0), x0 * x0),
                    format!("y = {}", fr(y.0, y.1)),
                ),
                solution: format!(
                    "y = {}·{}² = {}·{} = <b>{}</b>",
                    fr(a.0, a.1),
                    par(x0),
                    fr(a.0, a.1),
                    x0 * x0,
                    fr(y.0, y.1)
                ),
                diagnose: Box::new(move |v| match v[0] {
                    Val::Frac(n, d) if reduce(n, d) == reduce(-y.0, y.1) && y.0 != 0 => vec![dm(
                        "neg-pow",
                        "Το x² είναι πάντα θετικό, άρα το πρόσημο του y εξαρτάται μόνο από το α.",
                    )],
                    _ => vec![],
                }),
            }
        }
    }
}
