//! Κεφάλαιο 10 (Λόγοι – Αναλογίες, Ποσοστά) και Κεφάλαιο 11 (Στατιστική – Πιθανότητες).

use crate::mk::*;
use crate::model::*;
use crate::util::*;

// ---------------- Κεφάλαιο 10 ----------------

pub fn ratio(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let (p, q) = loop {
                let (p, q) = (r.r(1, 9), r.r(1, 9));
                if p != q && gcd(p, q) == 1 {
                    break (p, q);
                }
            };
            let k = r.r(2, 12);
            let (a, b) = (p * k, q * k);
            let ctx = r.pick(&[
                (
                    "Σε μια τάξη υπάρχουν {a} κορίτσια και {b} αγόρια.",
                    "κοριτσιών προς αγόρια",
                ),
                (
                    "Σε μια συνταγή χρειάζονται {a} g ζάχαρη και {b} g βούτυρο.",
                    "ζάχαρης προς βούτυρο",
                ),
                (
                    "Ένα ορθογώνιο έχει μήκος {a} cm και πλάτος {b} cm.",
                    "μήκους προς πλάτος",
                ),
            ]);
            Question {
                cat: "ratio",
                prompt: format!(
                    "{} Γράψε τον λόγο {} ως λόγο <b>ακεραίων πρώτων μεταξύ τους</b>.",
                    ctx.0
                        .replace("{a}", &a.to_string())
                        .replace("{b}", &b.to_string()),
                    ctx.1
                ),
                groups: one(vec![num("Λόγος:", p), num(":", q)]),
                hints: hints(
                    format!("Ο λόγος είναι {a} : {b}."),
                    "Απλοποίησε διαιρώντας και τους δύο όρους με τον ΜΚΔ τους.",
                    format!("ΜΚΔ({a}, {b}) = {k}"),
                    format!("{p} : {q}"),
                ),
                solution: format!("{a} : {b} = ({a} : {k}) : ({b} : {k}) = <b>{p} : {q}</b>"),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some(q as i64) && v[1].num() == Some(p as i64) {
                        vec![dm(
                            "ratio-order",
                            "Η σειρά στον λόγο μετράει: γράψε πρώτα το πρώτο μέγεθος.",
                        )]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        1 => {
            // divide an amount in a given ratio
            let (p, q) = (r.r(1, 7), r.r(1, 7));
            let k = r.r(3, 30);
            let total = (p + q) * k;
            let who = r.pick(&[
                (
                    "Δύο αδέλφια μοιράζονται {t} € με λόγο {p} : {q}.",
                    "ο πρώτος",
                    "ο δεύτερος",
                    "€",
                ),
                (
                    "Ένα σχολείο μοιράζει {t} βιβλία σε δύο τάξεις με λόγο {p} : {q}.",
                    "η πρώτη τάξη",
                    "η δεύτερη τάξη",
                    "βιβλία",
                ),
            ]);
            Question {
                cat: "ratio",
                prompt: who
                    .0
                    .replace("{t}", &total.to_string())
                    .replace("{p}", &p.to_string())
                    .replace("{q}", &q.to_string())
                    + " Πόσα παίρνει ο καθένας;",
                groups: vec![
                    group(format!("Πόσα παίρνει {};", who.1), vec![num(who.3, p * k)]),
                    group(format!("Πόσα παίρνει {};", who.2), vec![num(who.3, q * k)]),
                ],
                hints: hints(
                    format!("Χωρίζουμε το σύνολο σε {p} + {q} = {} ίσα μέρη.", p + q),
                    format!("Κάθε μέρος είναι {total} : {} = {k}.", p + q),
                    format!("Ο πρώτος παίρνει {p} μέρη, ο δεύτερος {q}."),
                    format!("{p}·{k} = {} και {q}·{k} = {}", p * k, q * k),
                ),
                solution: format!(
                    "{total} : ({p} + {q}) = {k}. Άρα <b>{}</b> και <b>{}</b> {}.",
                    p * k,
                    q * k,
                    who.3
                ),
                diagnose: no_diag(),
            }
        }
        _ => {
            // scale
            let scale = r.pick(&[100u64, 200, 500, 1000, 10_000, 50_000, 100_000]);
            let cm = r.r(2, 15);
            let real_cm = cm * scale;
            let (val, unit) = if real_cm.is_multiple_of(100_000) {
                (real_cm / 100_000, "km")
            } else if real_cm.is_multiple_of(100) {
                (real_cm / 100, "m")
            } else {
                (real_cm, "cm")
            };
            Question {
                cat: "ratio",
                prompt: format!("Ένας χάρτης έχει κλίμακα <b>1 : {}</b>. Δύο σημεία απέχουν πάνω στον χάρτη {cm} cm. Πόση είναι η πραγματική τους απόσταση σε <b>{unit}</b>;", thousands(scale)),
                groups: one(vec![num(format!("{unit}:"), val)]),
                hints: hints(format!("Κλίμακα 1 : {} σημαίνει ότι 1 cm στον χάρτη είναι {} cm στην πραγματικότητα.", thousands(scale), thousands(scale)), format!("{cm}·{} = {} cm", thousands(scale), thousands(real_cm)), "1 m = 100 cm, 1 km = 100.000 cm.", format!("{val} {unit}")),
                solution: format!("{cm} cm · {} = {} cm = <b>{val} {unit}</b>.", thousands(scale), thousands(real_cm)),
                diagnose: Box::new(move |v| if v[0].num() == Some(real_cm as i64) && unit != "cm" { vec![dm("units", format!("Αυτό είναι σε cm. Μετάτρεψέ το σε {unit}."))] } else { vec![] }),
            }
        }
    }
}

pub fn analog(r: &mut Rng) -> Question {
    if r.chance(0.55) {
        let (a, b) = loop {
            let (a, b) = (r.r(2, 12), r.r(2, 12));
            if a != b {
                break (a, b);
            }
        };
        let k = r.r(2, 9);
        let pos = r.r(0, 3);
        let vals = [a, b, a * k, b * k];
        let ans = vals[pos as usize];
        let show: Vec<String> = vals
            .iter()
            .enumerate()
            .map(|(i, x)| {
                if i as u64 == pos {
                    "<i>x</i>".into()
                } else {
                    x.to_string()
                }
            })
            .collect();
        return Question {
            cat: "analog",
            prompt: format!(
                "Λύσε την αναλογία <span class=\"big\">{} = {}</span>",
                fr_html(&show[0], &show[1]),
                fr_html(&show[2], &show[3])
            ),
            groups: one(vec![num("x =", ans)]),
            hints: hints(
                "Σε μια αναλογία τα «χιαστί» γινόμενα είναι ίσα: α/β = γ/δ ⇔ α·δ = β·γ.",
                "Γράψε την ισότητα των χιαστί γινομένων.",
                format!("Το γνωστό χιαστί γινόμενο είναι {}.", a * b * k),
                format!("x = {ans}"),
            ),
            solution: format!(
                "Χιαστί γινόμενα: {}·{} = {}·{}. Άρα <b>x = {ans}</b>.",
                show[0], show[3], show[1], show[2]
            ),
            diagnose: no_diag(),
        };
    }
    // direct proportion word problem
    let (unit_a, unit_b, ctx) = r.pick(&[
        (
            "κιλά",
            "€",
            "Τα {a} κιλά πορτοκάλια κοστίζουν {pa} €. Πόσο κοστίζουν τα {b} κιλά;",
        ),
        (
            "λίτρα",
            "km",
            "Ένα αυτοκίνητο με {a} λίτρα βενζίνη διανύει {pa} km. Πόσα km διανύει με {b} λίτρα;",
        ),
        (
            "τετράδια",
            "€",
            "Τα {a} τετράδια κοστίζουν {pa} €. Πόσο κοστίζουν τα {b} τετράδια;",
        ),
    ]);
    let unit_price = r.r(2, 12);
    let (a, b) = loop {
        let (a, b) = (r.r(2, 10), r.r(2, 20));
        if a != b {
            break (a, b);
        }
    };
    let pa = a * unit_price;
    let ans = b * unit_price;
    let _ = unit_a;
    Question {
        cat: "analog",
        prompt: ctx
            .replace("{a}", &a.to_string())
            .replace("{pa}", &pa.to_string())
            .replace("{b}", &b.to_string()),
        groups: one(vec![num(format!("{unit_b}:"), ans)]),
        hints: hints(
            "Τα ποσά είναι ανάλογα: όσο μεγαλώνει το ένα, μεγαλώνει και το άλλο με τον ίδιο λόγο.",
            format!("Βρες πρώτα πόσο αντιστοιχεί στη μονάδα: {pa} : {a} = {unit_price}."),
            format!("Μετά πολλαπλασίασε με {b}."),
            format!("{b}·{unit_price} = {ans}"),
        ),
        solution: format!(
            "Αναλογία: {} = {} ⇔ x = {}·{pa} : {a} = <b>{ans} {unit_b}</b>.",
            fr_html(&a.to_string(), &pa.to_string()),
            fr_html(&b.to_string(), "x"),
            b
        ),
        diagnose: no_diag(),
    }
}

fn fr_html(n: &str, d: &str) -> String {
    format!("<span class=\"fr\"><span>{n}</span><span>{d}</span></span>")
}

pub fn percent(r: &mut Rng) -> Question {
    match r.r(0, 3) {
        0 => {
            let p = r.pick(&[5u64, 10, 15, 20, 25, 30, 40, 50, 60, 75, 80]);
            let n = r.r(2, 40) * 20;
            let ans = n * p / 100;
            Question {
                cat: "percent",
                prompt: format!("Πόσο είναι το <b>{p}%</b> του {n};"),
                groups: one(vec![num("Απάντηση:", ans)]),
                hints: hints(
                    "Το p% ενός αριθμού είναι p/100 του αριθμού.",
                    format!("{} · {n}", fr_html(&p.to_string(), "100")),
                    format!("{n} : 100 = {}", n as f64 / 100.0),
                    ans.to_string(),
                ),
                solution: format!(
                    "{p}% του {n} = {}·{n} = <b>{ans}</b>",
                    fr_html(&p.to_string(), "100")
                ),
                diagnose: no_diag(),
            }
        }
        1 => {
            let total = r.pick(&[20u64, 25, 40, 50, 80, 200, 400]);
            let part = loop {
                let x = r.r(1, total - 1);
                if (x * 100).is_multiple_of(total) {
                    break x;
                }
            };
            let p = part * 100 / total;
            Question {
                cat: "percent",
                prompt: format!("Σε ένα διαγώνισμα με {total} ερωτήσεις ο Νίκος απάντησε σωστά τις {part}. Τι ποσοστό των ερωτήσεων απάντησε σωστά;"),
                groups: one(vec![num("%:", p)]),
                hints: hints("Ποσοστό = μέρος : σύνολο · 100%.", format!("{} · 100%", fr_html(&part.to_string(), &total.to_string())), "Απλοποίησε το κλάσμα ή κάνε τον παρονομαστή 100.".to_string(), format!("{p}%")),
                solution: format!("{} = {} = <b>{p}%</b>", fr_html(&part.to_string(), &total.to_string()), fr_html(&p.to_string(), "100")),
                diagnose: no_diag(),
            }
        }
        2 => {
            let p = r.pick(&[10u64, 20, 25, 30, 40, 50]);
            let price = r.r(2, 30) * 20;
            let up = r.chance(0.5);
            let change = price * p / 100;
            let ans = if up { price + change } else { price - change };
            Question {
                cat: "percent",
                prompt: if up {
                    format!("Ένα ποδήλατο κόστιζε {price} €. Η τιμή του αυξήθηκε κατά <b>{p}%</b>. Πόσο κοστίζει τώρα;")
                } else {
                    format!("Ένα παιχνίδι κόστιζε {price} € και έχει έκπτωση <b>{p}%</b>. Πόσο θα πληρώσουμε;")
                },
                groups: one(vec![num("€:", ans)]),
                hints: hints(
                    format!("Βρες πρώτα το {p}% του {price}."),
                    format!("{p}% του {price} = {change}"),
                    if up {
                        "Πρόσθεσέ το στην αρχική τιμή."
                    } else {
                        "Αφαίρεσέ το από την αρχική τιμή."
                    },
                    format!("{ans} €"),
                ),
                solution: format!(
                    "{p}% του {price} = {change} €. {} = <b>{ans} €</b>.",
                    if up {
                        format!("{price} + {change}")
                    } else {
                        format!("{price} − {change}")
                    }
                ),
                diagnose: Box::new(move |v| {
                    if v[0].num() == Some(change as i64) {
                        vec![dm(
                            "pct-step",
                            format!(
                                "Βρήκες {} {change} €. Τώρα {} την αρχική τιμή.",
                                if up {
                                    "την αύξηση,"
                                } else {
                                    "την έκπτωση,"
                                },
                                if up {
                                    "πρόσθεσέ την στην"
                                } else {
                                    "αφαίρεσέ την από"
                                }
                            ),
                        )]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let p = r.pick(&[10u64, 20, 25, 50]);
            let orig = r.r(2, 25) * 20;
            let after = orig * (100 - p) / 100;
            Question {
                cat: "percent",
                prompt: format!("Μετά από έκπτωση <b>{p}%</b> ένα μπουφάν κοστίζει {after} €. Ποια ήταν η αρχική του τιμή;"),
                groups: one(vec![num("€:", orig)]),
                hints: hints(format!("Μετά την έκπτωση πληρώνουμε το {}% της αρχικής τιμής.", 100 - p), format!("{}% της αρχικής = {after} €", 100 - p), format!("1% = {after} : {} = {}", 100 - p, after / (100 - p)), format!("{orig} €")),
                solution: format!("{}% της τιμής = {after} €, άρα 1% = {} € και 100% = <b>{orig} €</b>.", 100 - p, after / (100 - p)),
                diagnose: Box::new(move |v| if v[0].num() == Some((after + after * p / 100) as i64) && after + after * p / 100 != orig { vec![dm("pct-base", "Η έκπτωση υπολογίστηκε στην <b>αρχική</b> τιμή, όχι στην τελική. Βρες πρώτα σε τι ποσοστό αντιστοιχεί η τελική τιμή.")] } else { vec![] }),
            }
        }
    }
}

// ---------------- Κεφάλαιο 11 ----------------

pub fn stat_var(r: &mut Rng) -> Question {
    let items = [
        ("το αγαπημένο άθλημα των μαθητών μιας τάξης", false),
        ("ο αριθμός των αδελφών κάθε μαθητή", true),
        ("το χρώμα των ματιών", false),
        ("το ύψος των μαθητών σε cm", true),
        ("η επαρχία στην οποία μένει κάθε μαθητής", false),
        ("ο βαθμός στο διαγώνισμα των Μαθηματικών", true),
        ("το είδος του κατοικίδιου που έχει κάθε οικογένεια", false),
        (
            "ο χρόνος που χρειάζεται κάθε μαθητής για να έρθει στο σχολείο",
            true,
        ),
        ("η αγαπημένη τηλεοπτική εκπομπή", false),
        ("η μάζα των νεογέννητων σε ένα μαιευτήριο", true),
    ];
    let (s, quant) = r.pick(&items);
    Question {
        cat: "stat-var",
        prompt: format!("Σε μια έρευνα καταγράφεται <b>{s}</b>. Τι είδους μεταβλητή είναι;"),
        groups: one(vec![select(
            "Μεταβλητή:",
            vec![(0, "ποιοτική".into()), (1, "ποσοτική".into())],
            quant as i64,
        )]),
        hints: hints(
            "Ποσοτική: οι τιμές της είναι αριθμοί (μετρήσεις ή μετρήματα).",
            "Ποιοτική: οι τιμές της είναι κατηγορίες, όχι αριθμοί.",
            "Μπορείς να κάνεις πράξεις με τις τιμές της (π.χ. μέσο όρο);",
            if quant {
                "ποσοτική"
            } else {
                "ποιοτική"
            },
        ),
        solution: format!(
            "Οι τιμές {}, άρα η μεταβλητή είναι <b>{}</b>.",
            if quant {
                "είναι αριθμοί"
            } else {
                "είναι κατηγορίες"
            },
            if quant {
                "ποσοτική"
            } else {
                "ποιοτική"
            }
        ),
        diagnose: Box::new(|_| vec![tag("stat-var")]),
    }
}

pub fn stat_chart(r: &mut Rng) -> Question {
    let cats = r.pick(&[
        ["Ποδόσφαιρο", "Μπάσκετ", "Βόλεϊ", "Τένις"],
        ["Κόκκινο", "Μπλε", "Πράσινο", "Κίτρινο"],
        ["Σκύλος", "Γάτα", "Ψάρι", "Πουλί"],
    ]);
    let total = r.pick(&[20u64, 24, 30, 36, 40, 60]);
    // frequencies that sum to total
    let f: Vec<u64> = loop {
        let a = r.r(1, total / 2);
        let b = r.r(1, total / 2);
        let c = r.r(1, total / 3);
        if a + b + c < total {
            break vec![a, b, c, total - a - b - c];
        }
    };
    let table = format!(
        "<div class=\"tbl\"><table class=\"st\"><tr><th>Τιμή</th>{}</tr><tr><th>Συχνότητα</th>{}</tr></table></div>",
        cats.iter().map(|c| format!("<td>{c}</td>")).collect::<String>(),
        f.iter().map(|x| format!("<td>{x}</td>")).collect::<String>()
    );
    let i = r.r(0, 3) as usize;
    match r.r(0, 2) {
        0 => Question {
            cat: "stat-chart",
            prompt: format!("Ο πίνακας δείχνει τις απαντήσεις μαθητών σε μια έρευνα.{table}Πόσοι μαθητές πήραν μέρος συνολικά;"),
            groups: one(vec![num("Μαθητές:", total)]),
            hints: hints("Το σύνολο είναι το άθροισμα όλων των συχνοτήτων.", f.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" + "), "Πρόσθεσε.", total.to_string()),
            solution: format!("{} = <b>{total}</b>", f.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" + ")),
            diagnose: no_diag(),
        },
        1 => {
            // relative frequency as a fraction in lowest terms
            let (n, d) = reduce(f[i] as i64, total as i64);
            Question {
                cat: "stat-chart",
                prompt: format!("Ο πίνακας δείχνει τις απαντήσεις μαθητών σε μια έρευνα.{table}Ποια είναι η <b>σχετική συχνότητα</b> της τιμής «{}» (ως ανάγωγο κλάσμα);", cats[i]),
                groups: one(vec![frac("Σχετική συχνότητα:", n, d)]),
                hints: hints("Σχετική συχνότητα = συχνότητα : σύνολο.", format!("Σύνολο: {total}"), format!("{} : {total}", f[i]), fr(n, d)),
                solution: format!("{} = <b>{}</b>", fr_raw(f[i] as i64, total as i64), fr(n, d)),
                diagnose: no_diag(),
            }
        }
        _ => {
            // pie chart angle; pick a category with an integer angle
            let i = (0..4).find(|k| (f[*k] * 360).is_multiple_of(total)).unwrap_or(i);
            if !(f[i] * 360).is_multiple_of(total) {
                return stat_chart(r);
            }
            let angle = f[i] * 360 / total;
            Question {
                cat: "stat-chart",
                prompt: format!("Ο πίνακας δείχνει τις απαντήσεις μαθητών σε μια έρευνα.{table}Θέλουμε να φτιάξουμε <b>κυκλικό διάγραμμα</b>. Πόσες μοίρες θα είναι ο κυκλικός τομέας της τιμής «{}»;", cats[i]),
                groups: one(vec![num("Μοίρες:", angle)]),
                hints: hints("Ολόκληρος ο κύκλος (360°) αντιστοιχεί σε όλους τους μαθητές.", format!("Σύνολο: {total} μαθητές"), format!("{} : {total} · 360°", f[i]), format!("{angle}°")),
                solution: format!("{} · 360° = <b>{angle}°</b>", fr_raw(f[i] as i64, total as i64)),
                diagnose: Box::new(move |v| if v[0].num() == Some((f[i] * 100 / total) as i64) { vec![dm("pie-360", "Αυτό είναι ποσοστό. Για τις μοίρες πολλαπλασιάζουμε με 360°, όχι με 100.")] } else { vec![] }),
            }
        }
    }
}

pub fn prob(r: &mut Rng) -> Question {
    match r.r(0, 3) {
        0 => {
            let events: [(&str, Vec<u64>); 7] = [
                ("άρτιος αριθμός", vec![2, 4, 6]),
                ("αριθμός μεγαλύτερος του 4", vec![5, 6]),
                ("πρώτος αριθμός", vec![2, 3, 5]),
                ("ο αριθμός 6", vec![6]),
                ("αριθμός μικρότερος του 7", vec![1, 2, 3, 4, 5, 6]),
                ("ο αριθμός 7", vec![]),
                ("πολλαπλάσιο του 3", vec![3, 6]),
            ];
            let (e, fav) = r.pick(&events);
            let (n, d) = reduce(fav.len() as i64, 6);
            Question {
                cat: "prob",
                prompt: format!("Ρίχνουμε ένα ζάρι μία φορά. Ποια είναι η πιθανότητα να φέρουμε <b>{e}</b>;<br><span class=\"muted\">Γράψε την ως ανάγωγο κλάσμα (0 ή 1 αν είναι αδύνατο ή βέβαιο).</span>"),
                groups: one(vec![frac("Ρ(Α) =", n, d)]),
                hints: hints("Δειγματικός χώρος: Ω = {1, 2, 3, 4, 5, 6}, άρα 6 δυνατά αποτελέσματα.", format!("Ευνοϊκά αποτελέσματα: {}", if fav.is_empty() { "κανένα".to_string() } else { list(&fav) }), "Ρ(Α) = πλήθος ευνοϊκών : πλήθος δυνατών.", format!("Ρ(Α) = {}", fr(n, d))),
                solution: format!("Ευνοϊκά: {} ({} αποτελέσματα) από 6. Ρ(Α) = {} = <b>{}</b>{}", if fav.is_empty() { "κανένα".into() } else { format!("{{{}}}", list(&fav)) }, fav.len(), fr_raw(fav.len() as i64, 6), fr(n, d), match fav.len() { 0 => " (αδύνατο ενδεχόμενο)", 6 => " (βέβαιο ενδεχόμενο)", _ => "" }),
                diagnose: no_diag(),
            }
        }
        1 => {
            let (red, blue, green) = (r.r(1, 9), r.r(1, 9), r.r(1, 9));
            let total = red + blue + green;
            let (col, k) = r.pick(&[("κόκκινη", red), ("μπλε", blue), ("πράσινη", green)]);
            let not = r.chance(0.35);
            let fav = if not { total - k } else { k };
            let (n, d) = reduce(fav as i64, total as i64);
            Question {
                cat: "prob",
                prompt: format!("Ένα κουτί έχει {red} κόκκινες, {blue} μπλε και {green} πράσινες μπάλες. Παίρνουμε τυχαία μία μπάλα. Ποια είναι η πιθανότητα η μπάλα {} {col};", if not { "να <b>μην</b> είναι" } else { "να είναι" }),
                groups: one(vec![frac("Ρ =", n, d)]),
                hints: hints(format!("Συνολικά μπάλες: {red} + {blue} + {green} = {total}."), if not { format!("Οι μπάλες που δεν είναι {col}: {total} − {k} = {fav}.") } else { format!("{col}: {k}.") }, "Ρ = ευνοϊκά : δυνατά. Απλοποίησε το κλάσμα.", fr(n, d)),
                solution: format!("Ρ = {} = <b>{}</b>", fr_raw(fav as i64, total as i64), fr(n, d)),
                diagnose: Box::new(move |v| {
                    let u = match v[0] { Val::Frac(a, b) => Some(reduce(a, b)), _ => None };
                    if not && u == Some(reduce(k as i64, total as i64)) { vec![dm("prob-not", "Αυτή είναι η πιθανότητα να <b>είναι</b> το χρώμα. Ζητείται να μην είναι.")] } else { vec![] }
                }),
            }
        }
        2 => {
            let n = r.pick(&[10u64, 12, 15, 20, 25, 30]);
            let k = r.r(2, 5);
            let fav = n / k;
            let (a, b) = reduce(fav as i64, n as i64);
            Question {
                cat: "prob",
                prompt: format!("Σε ένα κουτί υπάρχουν κάρτες με τους αριθμούς 1 έως {n}. Παίρνουμε μία στην τύχη. Ποια είναι η πιθανότητα ο αριθμός να είναι <b>πολλαπλάσιο του {k}</b>;"),
                groups: one(vec![frac("Ρ =", a, b)]),
                hints: hints(format!("Δυνατά αποτελέσματα: {n}."), format!("Πολλαπλάσια του {k} από 1 έως {n}: {}", (1..=n).filter(|x| x % k == 0).map(|x| x.to_string()).collect::<Vec<_>>().join(", ")), format!("Είναι {fav}."), fr(a, b)),
                solution: format!("Ευνοϊκά: {fav}, δυνατά: {n}. Ρ = {} = <b>{}</b>", fr_raw(fav as i64, n as i64), fr(a, b)),
                diagnose: no_diag(),
            }
        }
        _ => {
            let items = [
                (
                    "Η πιθανότητα ενός ενδεχομένου μπορεί να είναι μεγαλύτερη από 1.",
                    false,
                ),
                ("Η πιθανότητα ενός αδύνατου ενδεχομένου είναι 0.", true),
                ("Η πιθανότητα ενός βέβαιου ενδεχομένου είναι 1.", true),
                (
                    "Όταν ρίχνουμε ένα νόμισμα, ο δειγματικός χώρος έχει 2 στοιχεία.",
                    true,
                ),
                (
                    "Όταν ρίχνουμε ένα ζάρι, το «να φέρουμε 7» είναι βέβαιο ενδεχόμενο.",
                    false,
                ),
                (
                    "Η πιθανότητα να φέρουμε «κορόνα» σε ένα νόμισμα είναι 1/2.",
                    true,
                ),
                ("Η πιθανότητα μπορεί να είναι αρνητικός αριθμός.", false),
            ];
            let (s, t) = r.pick(&items);
            Question {
                cat: "prob",
                prompt: format!("Σωστό ή λάθος;<br><b>{s}</b>"),
                groups: one(vec![truefalse(t)]),
                hints: hints(
                    "Κάθε πιθανότητα είναι αριθμός από 0 έως 1.",
                    "Αδύνατο ενδεχόμενο: πιθανότητα 0. Βέβαιο: πιθανότητα 1.",
                    "Δειγματικός χώρος: όλα τα δυνατά αποτελέσματα.",
                    if t { "Σωστό." } else { "Λάθος." },
                ),
                solution: format!(
                    "Η πρόταση είναι <b>{}</b>. Κάθε πιθανότητα Ρ(Α) ικανοποιεί 0 ≤ Ρ(Α) ≤ 1.",
                    if t { "σωστή" } else { "λάθος" }
                ),
                diagnose: Box::new(|_| vec![tag("prob-def")]),
            }
        }
    }
}
