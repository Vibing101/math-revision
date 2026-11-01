//! Prime factorisation, GCD/LCM and the word problems built on them.

use crate::mk::*;
use crate::model::*;
use crate::util::*;

pub fn fact(r: &mut Rng) -> Question {
    let n = loop {
        let (mut n, mut c) = (1u64, 0);
        let stop = r.r(3, 6);
        loop {
            let p = r.pick(&[2u64, 2, 2, 3, 3, 3, 5, 5, 7, 11, 13]);
            if n * p > 999 {
                break;
            }
            n *= p;
            c += 1;
            if c >= stop {
                break;
            }
        }
        if n >= 40 && factor(n).len() >= 2 {
            break n;
        }
    };
    let m = factor(n);
    let first = *m.keys().next().unwrap();
    let fp = fmt_pow(&m);
    Question {
        cat: "fact",
        prompt: format!("Ανάλυσε τον αριθμό <span class=\"big\">{n}</span> σε γινόμενο πρώτων παραγόντων.<br><span class=\"muted\">Γράψε τον εκθέτη πάνω από κάθε πρώτο. Άφησε κενό ή 0 για όσους δεν χρειάζονται.</span>"),
        groups: one(vec![exp(format!("{n} ="), vec![2, 3, 5, 7, 11, 13], m.clone())]),
        hints: hints(
            "Διαίρεσε με τον μικρότερο πρώτο που χωράει (2, 3, 5, 7, …) και συνέχισε μέχρι να φτάσεις στο 1.",
            format!("Το {n} διαιρείται με το {first}: {n} : {first} = {}.", n / first),
            format!("Τα πρώτα βήματα της σκάλας:{}", ladder(n, Some(2))),
            format!("{n} = {fp}"),
        ),
        solution: format!("{}{n} = <b>{fp}</b>", ladder(n, None)),
        diagnose: Box::new(move |v| match v[0].pow() {
            Some(u) if map_val(u) != n => vec![dm("fact-product", format!("Έλεγξε το αποτέλεσμά σου: το γινόμενο που έγραψες κάνει <b>{}</b>, όχι {n}.", map_val(u)))],
            _ => vec![],
        }),
    }
}

pub fn pow(r: &mut Rng) -> Question {
    const P: [u64; 4] = [2, 3, 5, 7];
    let ms: Vec<PowMap> = loop {
        let k = if r.chance(0.35) { 3 } else { 2 };
        let ms: Vec<PowMap> = (0..k)
            .map(|_| {
                let mut m = PowMap::new();
                for p in P {
                    if r.chance(0.6) {
                        m.insert(p, r.r(1, 3) as u32);
                    }
                }
                m
            })
            .collect();
        if ms.iter().any(|m| m.len() < 2) {
            continue;
        }
        let (g, mx) = (min_map(&ms), max_map(&ms));
        let diff = P.iter().any(|p| {
            ms.iter().all(|m| m.contains_key(p))
                && ms.iter().map(|m| m[p]).min() != mx.get(p).copied()
        });
        let only = P.iter().any(|p| mx.contains_key(p) && !g.contains_key(p));
        if diff && only && !g.is_empty() && map_val(&mx) <= 60_000 {
            break ms;
        }
    };
    let (g, mx) = (min_map(&ms), max_map(&ms));
    let (gv, lv) = (map_val(&g), map_val(&mx));
    let names: Vec<&str> = ["Α", "Β", "Γ"][..ms.len()].to_vec();
    let used: Vec<u64> = P.iter().copied().filter(|p| mx.contains_key(p)).collect();
    let listing = names
        .iter()
        .zip(&ms)
        .map(|(n, m)| format!("<b>{n} = {}</b>", fmt_pow_full(m)))
        .collect::<Vec<_>>()
        .join(", &nbsp;");
    let nl = names.join(", ");
    let common: Vec<u64> = g.keys().copied().collect();
    let (ms2, g2, mx2, used2) = (ms.clone(), g.clone(), mx.clone(), used.clone());
    Question {
        cat: "pow",
        prompt: format!("Δίνονται οι αριθμοί {listing}.<br>Γράψε τον ΜΚΔ και το ΕΚΠ ως γινόμενο δυνάμεων και μετά υπολόγισε την τιμή τους."),
        groups: vec![
            group(format!("α) ΜΚΔ({nl})"), vec![exp("ΜΚΔ =", used.clone(), g.clone()), num("Τιμή ΜΚΔ =", gv)]),
            group(format!("β) ΕΚΠ({nl})"), vec![exp("ΕΚΠ =", used.clone(), mx.clone()), num("Τιμή ΕΚΠ =", lv)]),
        ],
        hints: hints(
            "ΜΚΔ: μόνο οι <b>κοινοί</b> πρώτοι, με τον <b>μικρότερο</b> εκθέτη. ΕΚΠ: <b>όλοι</b> οι πρώτοι, με τον <b>μεγαλύτερο</b> εκθέτη.",
            format!("Ποιοι πρώτοι υπάρχουν σε <b>όλους</b> τους αριθμούς; {}. Αυτοί μπαίνουν στον ΜΚΔ.", list(&common)),
            format!("ΜΚΔ = {}. Για το ΕΚΠ πάρε κάθε πρώτο ({}) με τον μεγαλύτερο εκθέτη που βλέπεις.", fmt_pow_full(&g), list(&used)),
            format!("ΜΚΔ = {} = {gv}, &nbsp; ΕΚΠ = {} = {lv}", fmt_pow(&g), fmt_pow(&mx)),
        ),
        solution: format!(
            "ΜΚΔ = <b>{}</b> = {} = <b>{gv}</b><br>ΕΚΠ = <b>{}</b> = {} = <b>{lv}</b>",
            fmt_pow(&g),
            expand(&g),
            fmt_pow(&mx),
            expand(&mx)
        ),
        diagnose: Box::new(move |v| {
            let mut out = vec![];
            if let Some(um) = v[2].pow() {
                for p in &used2 {
                    let ue = *um.get(p).unwrap_or(&0);
                    let present: Vec<u32> = ms2.iter().filter_map(|m| m.get(p).copied()).collect();
                    let in_all = present.len() == ms2.len();
                    if ue > 0 && ue < mx2[p] && Some(ue) == present.iter().min().copied() {
                        out.push(dm("lcm-min", format!("Στο ΕΚΠ, για τον πρώτο {p} πήρες τον μικρότερο εκθέτη. Στο ΕΚΠ κάθε πρώτος μπαίνει με τον <b>μεγαλύτερο</b> εκθέτη: {p}{}.", sup(mx2[p]))));
                        break;
                    }
                    if ue == 0 && !in_all {
                        out.push(dm("lcm-missing", format!("Στο ΕΚΠ λείπει ο πρώτος {p}. Στο ΕΚΠ μπαίνουν <b>όλοι</b> οι πρώτοι, ακόμα κι αν υπάρχουν μόνο σε έναν αριθμό.")));
                        break;
                    }
                }
            }
            if let Some(ug) = v[0].pow() {
                for p in &used2 {
                    let ue = *ug.get(p).unwrap_or(&0);
                    if ue > 0 && !g2.contains_key(p) {
                        out.push(dm("gcd-noncommon", format!("Στον ΜΚΔ έβαλες το {p}, αλλά δεν υπάρχει σε όλους τους αριθμούς. Στον ΜΚΔ μπαίνουν μόνο οι <b>κοινοί</b> πρώτοι.")));
                        break;
                    }
                    if let Some(ge) = g2.get(p) {
                        if ue > *ge {
                            out.push(dm("gcd-max", format!("Στον ΜΚΔ, για το {p} πήρες μεγαλύτερο εκθέτη. Στον ΜΚΔ παίρνουμε τον <b>μικρότερο</b>: {p}{}.", sup(*ge))));
                            break;
                        }
                    }
                }
            }
            let mult = |m: &PowMap| m.iter().map(|(p, e)| if *e > 1 { p * *e as u64 } else { *p }).product::<u64>() as i64;
            let ulv = v[3].num();
            let ugv = v[1].num();
            if (ulv.is_some() && ulv != Some(lv as i64) && ulv == Some(mult(&mx2))) || (ugv.is_some() && ugv != Some(gv as i64) && ugv == Some(mult(&g2))) {
                out.push(dm("pow-mult", "Πρόσεξε τις δυνάμεις: π.χ. 5² = 5·5 = 25 (όχι 5·2 = 10) και 2³ = 2·2·2 = 8."));
            }
            out
        }),
    }
}

fn nice_nums(r: &mut Rng, k: usize) -> Vec<u64> {
    let pool: Vec<u64> = (12..=180)
        .filter(|n| {
            let f = factor(*n);
            f.len() >= 2 && f.keys().all(|p| *p <= 7)
        })
        .collect();
    loop {
        let mut ns: Vec<u64> = r.shuffle(&pool)[..k].to_vec();
        ns.sort_unstable();
        let (g, l) = (gcd_all(&ns), lcm_all(&ns));
        if g >= 2 && l <= 5000 && !ns.iter().all(|n| n % ns[0] == 0) && l != ns[k - 1] {
            return ns;
        }
    }
}

pub fn num_q(r: &mut Rng) -> Question {
    let k = if r.chance(0.4) { 3 } else { 2 };
    let ns = nice_nums(r, k);
    let (g, l) = (gcd_all(&ns), lcm_all(&ns));
    let ms: Vec<PowMap> = ns.iter().map(|n| factor(*n)).collect();
    let mist = lcm_mistakes(&ns);
    let nl = list(&ns);
    let ns2 = ns.clone();
    Question {
        cat: "num",
        prompt: format!("Βρες τον <b>ΜΚΔ</b> και το <b>ΕΚΠ</b> των αριθμών <span class=\"big\">{nl}</span>."),
        groups: one(vec![num(format!("ΜΚΔ({nl}) ="), g), num(format!("ΕΚΠ({nl}) ="), l)]),
        hints: hints(
            "Ανάλυσε πρώτα κάθε αριθμό σε γινόμενο πρώτων παραγόντων.",
            fac_lines(&ns),
            format!(
                "ΜΚΔ: κοινοί πρώτοι με τον μικρότερο εκθέτη → {}.<br>ΕΚΠ: όλοι οι πρώτοι με τον μεγαλύτερο εκθέτη → {}.",
                fmt_pow(&min_map(&ms)),
                fmt_pow(&max_map(&ms))
            ),
            format!("ΜΚΔ = {g}, ΕΚΠ = {l}"),
        ),
        solution: format!(
            "{}{}<br>ΜΚΔ = {} = <b>{g}</b><br>ΕΚΠ = {} = <b>{l}</b>",
            ladders(&ns),
            fac_lines(&ns),
            fmt_pow(&min_map(&ms)),
            fmt_pow(&max_map(&ms))
        ),
        diagnose: Box::new(move |v| {
            let mut out = vec![];
            if let Some(ul) = v[1].num() {
                let ul = ul as u64;
                if ul != l {
                    if let Some((_, t)) = mist.iter().find(|(x, _)| *x == ul) {
                        out.push(dm(
                            t,
                            if *t == "lcm-min" {
                                "Στο ΕΚΠ κάποιος πρώτος μπήκε με μικρότερο εκθέτη. Πάρε κάθε πρώτο με τον <b>μεγαλύτερο</b> εκθέτη."
                            } else {
                                "Στο ΕΚΠ ξέχασες έναν πρώτο που υπάρχει μόνο σε έναν από τους αριθμούς. Στο ΕΚΠ μπαίνουν <b>όλοι</b>."
                            },
                        ));
                    } else if ul == g {
                        out.push(dm("gcd-vs-lcm", "Στο ΕΚΠ έγραψες τον ΜΚΔ. Το ΕΚΠ είναι πολλαπλάσιο όλων των αριθμών, άρα δεν μπορεί να είναι μικρότερο από αυτούς."));
                    } else if ul > 0 && ul.is_multiple_of(l) {
                        out.push(dm("not-least", format!("Το {ul} είναι κοινό πολλαπλάσιο, αλλά όχι το <b>ελάχιστο</b>.")));
                    }
                }
            }
            if let Some(ug) = v[0].num() {
                let ug = ug as u64;
                if ug != g && ug > 0 && ns2.iter().all(|n| n % ug == 0) {
                    out.push(dm("not-greatest", format!("Το {ug} διαιρεί όλους τους αριθμούς, αλλά υπάρχει <b>μεγαλύτερος</b> κοινός διαιρέτης.")));
                }
            }
            out
        }),
    }
}

struct GcdT {
    ctx: &'static str,
    q: &'static str,
    per: &'static str,
    items: &'static [&'static str],
    unit: &'static str,
}

pub fn gcdp(r: &mut Rng) -> Question {
    let ts = [
        GcdT { ctx: "Οι μαθητές της Α΄ Γυμνασίου μάζεψαν {0} πακέτα μακαρόνια, {1} κουτιά γάλα και {2} πακέτα αλεύρι. Θέλουν να φτιάξουν <b>όσο το δυνατόν περισσότερα όμοια δέματα</b>, χρησιμοποιώντας όλα τα τρόφιμα.", q: "Πόσα δέματα θα φτιάξουν;", per: "Πόσα από κάθε είδος θα έχει κάθε δέμα;", items: &["μακαρόνια", "γάλα", "αλεύρι"], unit: "δέματα" },
        GcdT { ctx: "Ένας χορευτικός όμιλος έχει {0} κορίτσια και {1} αγόρια. Ο καθηγητής θέλει να φτιάξει <b>όσο το δυνατόν περισσότερες ομοιόμορφες ομάδες</b> με όλα τα παιδιά.", q: "Πόσες ομάδες θα φτιάξει;", per: "Πόσα κορίτσια και πόσα αγόρια θα έχει κάθε ομάδα;", items: &["κορίτσια", "αγόρια"], unit: "ομάδες" },
        GcdT { ctx: "Ένα ανθοπωλείο έχει {0} τριαντάφυλλα, {1} γαρύφαλλα και {2} μαργαρίτες. Θέλει να φτιάξει <b>όσο το δυνατόν περισσότερες όμοιες ανθοδέσμες</b>, χρησιμοποιώντας όλα τα λουλούδια.", q: "Πόσες ανθοδέσμες θα φτιάξει;", per: "Πόσα λουλούδια από κάθε είδος θα έχει κάθε ανθοδέσμη;", items: &["τριαντάφυλλα", "γαρύφαλλα", "μαργαρίτες"], unit: "ανθοδέσμες" },
        GcdT { ctx: "Σε μια εκδρομή πάνε {0} μαθητές της Α΄, {1} της Β΄ και {2} της Γ΄ Γυμνασίου. Οι καθηγητές θέλουν <b>όσο το δυνατόν περισσότερες ομάδες</b>, που να έχουν όλες τον ίδιο αριθμό μαθητών από κάθε τάξη.", q: "Πόσες ομάδες θα γίνουν;", per: "Πόσους μαθητές από κάθε τάξη θα έχει κάθε ομάδα;", items: &["Α΄ τάξη", "Β΄ τάξη", "Γ΄ τάξη"], unit: "ομάδες" },
        GcdT { ctx: "Ένας προπονητής έχει {0} μπάλες ποδοσφαίρου και {1} μπάλες μπάσκετ. Θέλει να τις μοιράσει σε <b>όσο το δυνατόν περισσότερα όμοια καλάθια</b>, χωρίς να περισσέψει καμία.", q: "Πόσα καλάθια θα γεμίσει;", per: "Πόσες μπάλες από κάθε είδος θα έχει κάθε καλάθι;", items: &["ποδοσφαίρου", "μπάσκετ"], unit: "καλάθια" },
        GcdT { ctx: "Ένα ξυλουργείο έχει τρεις σανίδες με μήκη {0} cm, {1} cm και {2} cm. Θέλει να τις κόψει σε κομμάτια ίδιου μήκους, <b>όσο το δυνατόν μεγαλύτερα</b>, χωρίς να περισσέψει ξύλο.", q: "Πόσο μήκος (cm) θα έχει κάθε κομμάτι;", per: "Σε πόσα κομμάτια θα κοπεί κάθε σανίδα;", items: &["1η σανίδα", "2η σανίδα", "3η σανίδα"], unit: "cm" },
    ];
    let t = &ts[r.r(0, ts.len() as u64 - 1) as usize];
    let n = t.items.len();
    let (g, mul, vals) = loop {
        let g = r.r(4, 24);
        let mul: Vec<u64> = r.shuffle(&[2, 3, 4, 5, 6, 7, 8, 9, 10, 11])[..n].to_vec();
        let vals: Vec<u64> = mul.iter().map(|m| m * g).collect();
        if gcd_all(&mul) == 1 && *vals.iter().max().unwrap() <= 320 {
            break (g, mul, vals);
        }
    };
    let mut ctx = t.ctx.to_string();
    for (i, v) in vals.iter().enumerate() {
        ctx = ctx.replace(&format!("{{{i}}}"), &v.to_string());
    }
    let mut per = vec![];
    for (i, it) in t.items.iter().enumerate() {
        per.push(num(format!("{it}:"), mul[i]));
    }
    let unit = t.unit;
    let lines = vals
        .iter()
        .zip(&mul)
        .zip(t.items)
        .map(|((v, m), it)| format!("{v} : {g} = <b>{m}</b> ({it})"))
        .collect::<Vec<_>>()
        .join("<br>");
    let vals2 = vals.clone();
    Question {
        cat: "gcdp",
        prompt: ctx,
        groups: vec![group(format!("α) {}", t.q), vec![num(format!("{unit}:"), g)]), group(format!("β) {}", t.per), per)],
        hints: hints(
            "«Όσο το δυνατόν περισσότερες όμοιες ομάδες» ή «όσο το δυνατόν μεγαλύτερα ίσα κομμάτια» → ψάχνουμε τον <b>Μέγιστο Κοινό Διαιρέτη</b>.",
            fac_lines(&vals),
            format!("ΜΚΔ({}) = {g}. Τώρα διαίρεσε κάθε ποσότητα με το {g}.", list(&vals)),
            format!("{g}. Και: {}.", vals.iter().zip(&mul).map(|(v, m)| format!("{v} : {g} = {m}")).collect::<Vec<_>>().join(", ")),
        ),
        solution: format!("{}{}<br>ΜΚΔ = {} = <b>{g}</b> {unit}.<br>{lines}", ladders(&vals), fac_lines(&vals), fmt_pow(&min_map(&vals.iter().map(|v| factor(*v)).collect::<Vec<_>>()))),
        diagnose: Box::new(move |v| {
            let mut out = vec![];
            if let Some(ug) = v[0].num() {
                let ug = ug as u64;
                if ug != g {
                    if ug == lcm_all(&vals2) {
                        out.push(dm("gcd-vs-lcm", "Εδώ <b>μοιράζουμε</b>, άρα χρειαζόμαστε τον ΜΚΔ, όχι το ΕΚΠ."));
                    } else if ug > 0 && vals2.iter().all(|x| x % ug == 0) {
                        out.push(dm("not-greatest", format!("Με {ug} μοιράζονται όλα, αλλά ζητάμε το <b>μεγαλύτερο</b> δυνατό. Υπάρχει μεγαλύτερος κοινός διαιρέτης.")));
                    }
                    if ug > 0 && vals2.iter().enumerate().all(|(i, x)| v[i + 1].num() == Some((x / ug) as i64) && x % ug == 0) {
                        out.push(msg("Οι διαιρέσεις στο (β) είναι σωστές για τον αριθμό που βρήκες στο (α). Διόρθωσε το (α) και θα διορθωθεί και το (β)."));
                    }
                }
            }
            out
        }),
    }
}

fn pick_periods(r: &mut Rng, pool: &[u64], lo: u64, hi: u64, not24: bool) -> Vec<u64> {
    loop {
        let k = if r.chance(0.6) { 3 } else { 2 };
        let mut ps: Vec<u64> = r.shuffle(pool)[..k].to_vec();
        ps.sort_unstable();
        if ps.iter().any(|p| ps.iter().any(|q| q != p && q % p == 0)) {
            continue;
        }
        let l = lcm_all(&ps);
        if l >= lo && l <= hi && (!not24 || !l.is_multiple_of(24)) {
            return ps;
        }
    }
}

struct TimeT {
    three: &'static str,
    intro: &'static str,
    meet: &'static str,
    names: [&'static str; 3],
    verb: &'static str,
    qc: &'static str,
    cnt: &'static str,
    short: [&'static str; 3],
    place: &'static str,
}

pub fn time(r: &mut Rng) -> Question {
    let ts = [
        TimeT {
            three: "Τρία",
            intro: "τρένα ξεκινούν ταυτόχρονα από τη Λευκωσία",
            meet: "ξαναβρεθούν και τα",
            names: ["Το α΄ τρένο", "το β΄", "το γ΄"],
            verb: "επιστρέφει κάθε {p} ώρες",
            qc: "Πόσες διαδρομές θα έχει κάνει κάθε τρένο μέχρι τότε;",
            cnt: "διαδρομές",
            short: ["α΄ τρένο", "β΄ τρένο", "γ΄ τρένο"],
            place: " στη Λευκωσία",
        },
        TimeT {
            three: "Τρία",
            intro: "πλοία φεύγουν μαζί από το λιμάνι της Λεμεσού",
            meet: "βρεθούν ξανά και τα",
            names: ["Το α΄ πλοίο", "το β΄", "το γ΄"],
            verb: "επιστρέφει κάθε {p} ώρες",
            qc: "Πόσα ταξίδια θα έχει κάνει κάθε πλοίο μέχρι τότε;",
            cnt: "ταξίδια",
            short: ["α΄ πλοίο", "β΄ πλοίο", "γ΄ πλοίο"],
            place: " στο λιμάνι",
        },
        TimeT {
            three: "Τρεις",
            intro: "δορυφόροι του πλανήτη Π βρίσκονται στην ίδια ευθεία με τον πλανήτη",
            meet: "ξαναβρεθούν στην ίδια ευθεία και οι",
            names: ["Ο Δ₁", "ο Δ₂", "ο Δ₃"],
            verb: "κάνει μια περιστροφή κάθε {p} ώρες",
            qc: "Πόσες περιστροφές θα έχει κάνει κάθε δορυφόρος μέχρι τότε;",
            cnt: "περιστροφές",
            short: ["Δ₁", "Δ₂", "Δ₃"],
            place: "",
        },
    ];
    let t = &ts[r.r(0, 2) as usize];
    let ps = pick_periods(
        r,
        &[4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 30, 36, 40, 45],
        30,
        260,
        true,
    );
    let k = ps.len();
    let l = lcm_all(&ps);
    let (sd, sh) = (r.r(0, 6) as usize, r.r(6, 11));
    let tot = sh + l;
    let (d_off, eh) = (tot / 24, tot % 24);
    let ed = (sd + d_off as usize) % 7;
    let (d24, r24) = (l / 24, l % 24);
    let count_word = if k == 2 { "Δύο" } else { t.three };
    let all = if k == 2 {
        "δύο".to_string()
    } else {
        t.three.to_lowercase()
    };
    let desc = join_and(
        &ps.iter()
            .enumerate()
            .map(|(i, p)| format!("{} {}", t.names[i], t.verb.replace("{p}", &p.to_string())))
            .collect::<Vec<_>>(),
    );
    let start = tl(sh);
    let dot = if start.ends_with('.') { "" } else { "." };
    let hours: Vec<(i64, String)> = (0..24).map(|h| (h as i64, tl(h))).collect();
    let carry = if sh + r24 >= 24 {
        format!(
            " Επειδή {sh} + {r24} = {} ≥ 24, περνάμε σε μία ακόμα μέρα.",
            sh + r24
        )
    } else {
        String::new()
    };
    let mut per = vec![];
    for (i, p) in ps.iter().enumerate() {
        per.push(num(format!("{}:", t.short[i]), l / p));
    }
    let lp = fmt_pow(&max_map(&ps.iter().map(|p| factor(*p)).collect::<Vec<_>>()));
    let cnt = t.cnt;
    let shorts = t.short;
    let ps2 = ps.clone();
    let n = 1 + k;
    Question {
        cat: "time",
        prompt: format!("{count_word} {} {} στις <b>{start}</b>{dot} {desc}.", t.intro, day_art(sd)),
        groups: vec![
            group(format!("α) Μετά από πόσες ώρες θα {} {all}{};", t.meet, t.place), vec![num("ώρες:", l)]),
            group(format!("β) {}", t.qc), per),
            group("γ) Ποια μέρα και ποια ώρα θα γίνει αυτό;", vec![day_select(ed), select("Ώρα:", hours, eh as i64)]),
        ],
        hints: hints(
            "Ψάχνουμε την πρώτη φορά που οι χρόνοι «συμπίπτουν» → <b>Ελάχιστο Κοινό Πολλαπλάσιο</b>.",
            format!("{}<br>ΕΚΠ({}) = {lp} = {l} ώρες.", fac_lines(&ps), list(&ps)),
            format!("Μετάτρεψε τις ώρες σε μέρες: {l} : 24 = {d24} μέρες και περισσεύουν <b>{r24} ώρες</b>. Μην ξεχάσεις αυτές τις {r24} ώρες!{carry}"),
            format!("{} {start} + {d24} μέρες και {r24} ώρες → <b>{}, {}</b>.", day_art(sd), DAYS[ed], tl(eh)),
        ),
        solution: format!(
            "ΕΚΠ({}) = {lp} = <b>{l} ώρες</b>.<br>{}<br>{l} ώρες = {d24}·24 + {r24} = <b>{d24} μέρες και {r24} ώρες</b>.{carry}<br>Από {} στις {start}, φτάνουμε {} στις <b>{}</b>.",
            list(&ps),
            ps.iter().enumerate().map(|(i, p)| format!("{}: {l} : {p} = <b>{}</b> {cnt}", shorts[i], l / p)).collect::<Vec<_>>().join("<br>"),
            day_art(sd),
            day_art(ed),
            tl(eh)
        ),
        diagnose: Box::new(move |v| {
            let mut out = vec![];
            if let Some(ul) = v[0].num() {
                let ul = ul as u64;
                if ul != l {
                    if let Some((_, t)) = lcm_mistakes(&ps2).iter().find(|(x, _)| *x == ul) {
                        out.push(dm(t, "Το ΕΚΠ δεν βγήκε σωστό: πάρε κάθε πρώτο παράγοντα με τον <b>μεγαλύτερο</b> εκθέτη."));
                    } else if ul > 0 && ul.is_multiple_of(l) {
                        out.push(dm("not-least", format!("Το {ul} είναι κοινό πολλαπλάσιο, αλλά ζητάμε την <b>πρώτη</b> φορά, δηλαδή το ελάχιστο.")));
                    }
                }
            }
            let no_rem_day = ((sd + d24 as usize) % 7) as i64;
            if let (Some(dd), Some(hh)) = (v[n].num(), v[n + 1].num()) {
                if r24 != 0 && hh == sh as i64 && dd == no_rem_day {
                    out.push(dm("time-remainder", format!("Μέτρησες σωστά τις {d24} ολόκληρες μέρες, αλλά ξέχασες τις <b>{r24} ώρες</b> που περισσεύουν ({l} = {d24}·24 + {r24}).")));
                } else if dd == ed as i64 && (hh - eh as i64).abs() == 12 {
                    out.push(dm("time-12", "Η μέρα είναι σωστή. Πρόσεξε το π.μ. και μ.μ.: μετά τις 12 το μεσημέρι είναι μ.μ."));
                } else if dd != ed as i64 && hh == eh as i64 {
                    out.push(dm("time-day", format!("Η ώρα είναι σωστή. Μέτρησε ξανά τις μέρες: {l} : 24 = {d24} και περισσεύουν {r24}.")));
                }
            }
            out
        }),
    }
}

pub fn years(r: &mut Rng) -> Question {
    let ps = pick_periods(
        r,
        &[3, 4, 5, 6, 8, 9, 10, 12, 15, 18, 20, 25],
        20,
        150,
        false,
    );
    let k = ps.len();
    let l = lcm_all(&ps);
    let y = r.r(1995, 2025);
    let ns: Vec<u64> = [100u64, 120, 150, 200, 250, 300, 350, 400, 500]
        .into_iter()
        .filter(|n| n % l != 0 && n / l >= 2 && n / l <= 9)
        .collect();
    let n = if ns.is_empty() {
        l * 3 + l / 2
    } else {
        r.pick(&ns)
    };
    let times = n / l;
    let pl: Vec<String> = ps.iter().map(|p| p.to_string()).collect();
    let (prompt, ev) = match r.r(0, 2) {
        0 => (
            format!(
                "{} κομήτες εμφανίζονται στη Γη: {}. Εμφανίστηκαν όλοι μαζί το <b>{y}</b>.",
                if k == 2 { "Δύο" } else { "Τρεις" },
                join_and(&ps.iter().enumerate().map(|(i, p)| format!("ο {} κάθε {p} χρόνια", ["α΄", "β΄", "γ΄"][i])).collect::<Vec<_>>())
            ),
            ("θα εμφανιστούν ξανά μαζί", "θα εμφανιστούν μαζί"),
        ),
        1 => (
            format!("{} εταιρείες αυτοκινήτων βγάζουν νέο μοντέλο κάθε {} χρόνια αντίστοιχα. Το <b>{y}</b> έβγαλαν όλες μαζί νέο μοντέλο.", if k == 2 { "Δύο" } else { "Τρεις" }, join_and(&pl)),
            ("θα βγάλουν ξανά όλες μαζί νέο μοντέλο", "θα βγάλουν όλες μαζί νέο μοντέλο"),
        ),
        _ => (
            format!("{} φεστιβάλ μιας πόλης γίνονται κάθε {} χρόνια αντίστοιχα. Το <b>{y}</b> έγιναν όλα την ίδια χρονιά.", if k == 2 { "Δύο" } else { "Τρία" }, join_and(&pl)),
            ("θα γίνουν ξανά όλα την ίδια χρονιά", "θα γίνουν όλα την ίδια χρονιά"),
        ),
    };
    let lp = fmt_pow(&max_map(&ps.iter().map(|p| factor(*p)).collect::<Vec<_>>()));
    Question {
        cat: "years",
        prompt,
        groups: vec![
            group(format!("α) Ποια χρονιά {} για πρώτη φορά;", ev.0), vec![num("Χρονιά:", y + l)]),
            group(format!("β) Πόσες φορές {} στα επόμενα {n} χρόνια (χωρίς να μετρήσεις το {y});", ev.1), vec![num("φορές:", times)]),
        ],
        hints: hints(
            "Το ίδιο γεγονός επαναλαμβάνεται κάθε <b>ΕΚΠ</b> χρόνια.",
            format!("ΕΚΠ({}) = {l} χρόνια.", list(&ps)),
            format!("Πρώτη φορά: {y} + {l}. Για το (β): {n} : {l} = ; Μετράμε μόνο τις <b>ολόκληρες</b> φορές."),
            format!("{} &nbsp;και&nbsp; {times} φορές", y + l),
        ),
        solution: format!(
            "{}<br>ΕΚΠ = {lp} = <b>{l}</b> χρόνια.<br>α) {y} + {l} = <b>{}</b><br>β) {n} = {times}·{l} + {}, άρα <b>{times}</b> φορές ({}).",
            fac_lines(&ps),
            y + l,
            n - times * l,
            (1..=times).map(|i| (y + l * i).to_string()).collect::<Vec<_>>().join(", ")
        ),
        diagnose: Box::new(move |v| {
            let mut out = vec![];
            if v[1].num() == Some(times as i64 + 1) {
                out.push(dm("years-count", format!("Το {n} : {l} δεν βγαίνει ακριβώς. Μετράμε μόνο τις <b>ολόκληρες</b> φορές, δηλαδή στρογγυλεύουμε προς τα κάτω.")));
            }
            if let Some(u) = v[0].num() {
                let u = u as u64;
                if u != y + l {
                    if u == l {
                        out.push(msg(format!("Βρήκες σωστά ότι ξανασυμβαίνει κάθε {l} χρόνια. Τώρα πρόσθεσέ τα στο {y} για να βρεις τη χρονιά.")));
                    } else if u > y && (u - y).is_multiple_of(l) {
                        out.push(dm("not-least", "Αυτή η χρονιά είναι κοινή, αλλά δεν είναι η <b>πρώτη</b>. Χρησιμοποίησε το ελάχιστο κοινό πολλαπλάσιο."));
                    }
                }
            }
            out
        }),
    }
}

pub fn chal(r: &mut Rng) -> Question {
    let with_r = r.chance(0.5);
    loop {
        let ps = pick_periods(r, &[6, 8, 9, 10, 12, 15, 18, 20, 24, 30], 36, 200, false);
        let l = lcm_all(&ps);
        let rem = if with_r {
            r.r(1, ps.iter().min().unwrap() - 1)
        } else {
            0
        };
        let ms: Vec<u64> = (1..)
            .map(|i| i * l + rem)
            .take_while(|x| *x <= 999)
            .filter(|x| *x >= 100)
            .collect();
        if ms.len() < 2 {
            continue;
        }
        let target = r.pick(&ms);
        let span = (l - 5).min(70);
        let lo = (target - r.r(5, span)) / 10 * 10;
        let hi = (target + r.r(5, span)).div_ceil(10) * 10;
        if ms.iter().filter(|x| **x > lo && **x < hi).count() != 1 || lo < 50 {
            continue;
        }
        let pl: Vec<String> = ps.iter().map(|p| p.to_string()).collect();
        let or_list = |sep: &str| {
            format!(
                "{} ή {sep}{}",
                pl[..pl.len() - 1].join(&format!(", {sep}")),
                pl[pl.len() - 1]
            )
        };
        let prompt = if with_r {
            if r.chance(0.5) {
                format!("Οι καρέκλες μιας αίθουσας είναι περισσότερες από {lo} και λιγότερες από {hi}. Όταν τις βάζουμε σε σειρές των {}, περισσεύουν πάντα <b>{rem}</b>. Πόσες είναι οι καρέκλες;", or_list("των "))
            } else {
                format!("Ένας συλλέκτης έχει περισσότερα από {lo} και λιγότερα από {hi} γραμματόσημα. Αν τα βάλει σε σελίδες των {}, του περισσεύουν πάντα <b>{rem}</b>. Πόσα γραμματόσημα έχει;", or_list("των "))
            }
        } else if r.chance(0.5) {
            format!("Σε μια αποθήκη υπάρχει τριψήφιος αριθμός κουτιών με χυμό. Όταν τα συσκευάζουμε ανά {}, δεν περισσεύει κανένα. Πόσα είναι τα κουτιά, αν είναι περισσότερα από {lo} και λιγότερα από {hi};", or_list(""))
        } else {
            format!("Οι μαθητές ενός σχολείου μπορούν να χωριστούν σε ομάδες των {} χωρίς να περισσέψει κανείς. Πόσοι είναι οι μαθητές, αν είναι περισσότεροι από {lo} και λιγότεροι από {hi};", or_list("των "))
        };
        let cands: Vec<u64> = ms.iter().copied().filter(|x| *x < hi + l).take(6).collect();
        let lp = fmt_pow(&max_map(&ps.iter().map(|p| factor(*p)).collect::<Vec<_>>()));
        let check = if with_r {
            format!(
                "<br>Έλεγχος: {}.",
                ps.iter()
                    .map(|p| format!("{target} = {p}·{} + {rem}", (target - rem) / p))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            String::new()
        };
        return Question {
            cat: "chal",
            prompt,
            groups: one(vec![num("Απάντηση:", target)]),
            hints: hints(
                if with_r {
                    format!("Αν αφαιρέσουμε τα {rem} που περισσεύουν, ο αριθμός που μένει διαιρείται ακριβώς με τα {}.", list(&ps))
                } else {
                    format!("«Δεν περισσεύει κανένα» σημαίνει ότι ο αριθμός είναι <b>κοινό πολλαπλάσιο</b> των {}.", list(&ps))
                },
                format!("ΕΚΠ({}) = {l}. Ο αριθμός είναι {}.", list(&ps), if with_r { format!("πολλαπλάσιο του {l} συν {rem}") } else { format!("πολλαπλάσιο του {l}") }),
                format!("Γράψε τους υποψήφιους: {}, … Ποιος είναι ανάμεσα στο {lo} και το {hi};", list(&cands)),
                format!("Ο αριθμός είναι {target}."),
            ),
            solution: format!("{}<br>ΕΚΠ = {lp} = {l}.<br>Υποψήφιοι: {}, …<br>Ανάμεσα στο {lo} και το {hi} είναι μόνο το <b>{target}</b>.{check}", fac_lines(&ps), list(&cands)),
            diagnose: Box::new(move |v| match v[0].num() {
                Some(u) if u as u64 == l => vec![dm("range-lcm", format!("Το {l} είναι το ΕΚΠ. Ο αριθμός όμως πρέπει να είναι ανάμεσα στο {lo} και το {hi}: ψάξε στα πολλαπλάσιά του."))],
                Some(u) if with_r && u as u64 == target - rem => vec![dm("range-rem", format!("Σχεδόν! Το {} διαιρείται ακριβώς. Μην ξεχάσεις να προσθέσεις τα <b>{rem}</b> που περισσεύουν.", target - rem))],
                _ => vec![],
            }),
        };
    }
}
