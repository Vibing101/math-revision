//! Κεφάλαιο 4: Ακέραιοι – Ρητοί αριθμοί.

use crate::mk::*;
use crate::model::*;
use crate::util::*;
use std::cmp::Ordering;

type R = (i64, i64);

fn radd(a: R, b: R) -> R {
    reduce(a.0 * b.1 + b.0 * a.1, a.1 * b.1)
}
fn rsub(a: R, b: R) -> R {
    radd(a, (-b.0, b.1))
}
fn rmul(a: R, b: R) -> R {
    reduce(a.0 * b.0, a.1 * b.1)
}
fn rdiv(a: R, b: R) -> R {
    reduce(a.0 * b.1, a.1 * b.0)
}
fn rcmp(a: R, b: R) -> Ordering {
    (a.0 * b.1).cmp(&(b.0 * a.1))
}
/// A rational as shown inside an expression: integers in parentheses when negative.
fn rp(a: R) -> String {
    if a.1 == 1 {
        par(a.0)
    } else if a.0 < 0 {
        format!("({})", fr(a.0, a.1))
    } else {
        fr(a.0, a.1)
    }
}
fn nz(r: &mut Rng, lo: i64, hi: i64) -> i64 {
    loop {
        let x = lo + r.r(0, (hi - lo) as u64) as i64;
        if x != 0 {
            return x;
        }
    }
}
fn small_frac(r: &mut Rng) -> R {
    let d = r.pick(&[2i64, 3, 4, 5, 6, 8]);
    let n = nz(r, -(d * 2), d * 2);
    reduce(n, d)
}
fn sign_diag(answer: R) -> DiagFn {
    Box::new(move |v| {
        let flipped = match v[0] {
            Val::Num(n) => answer.1 == 1 && n == -answer.0 && n != 0,
            Val::Frac(n, d) => reduce(n, d) == (-answer.0, answer.1) && answer.0 != 0,
            _ => false,
        };
        if flipped {
            vec![dm("sign", "Ο αριθμός είναι σωστός, αλλά το <b>πρόσημο</b> όχι. Έλεγξε τον κανόνα των προσήμων.")]
        } else {
            vec![]
        }
    })
}
fn answer_part(label: &str, a: R) -> Part {
    if a.1 == 1 {
        int(label, a.0)
    } else {
        frac(label, a.0, a.1)
    }
}

pub fn rat_abs(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let a = nz(r, -40, 40);
            let opp = r.chance(0.5);
            let (q, ans, why) = if opp {
                (format!("Ποιος είναι ο <b>αντίθετος</b> του {}", signed(a)), -a, format!("Αντίθετοι είναι δύο αριθμοί με ίδια απόλυτη τιμή και αντίθετο πρόσημο. Ο αντίθετος του {} είναι ο {}.", signed(a), signed(-a)))
            } else {
                (format!("Βρες την απόλυτη τιμή <span class=\"big\">|{}|</span>", signed(a)), a.abs(), format!("Η απόλυτη τιμή είναι η απόσταση από το 0, άρα δεν είναι ποτέ αρνητική: |{}| = {}.", signed(a), a.abs()))
            };
            Question {
                cat: "rat-abs",
                prompt: format!("{q};"),
                groups: one(vec![int("Απάντηση:", ans)]),
                hints: hints(
                    if opp {
                        "Ο αντίθετος έχει την ίδια απόλυτη τιμή και αντίθετο πρόσημο."
                    } else {
                        "Η απόλυτη τιμή ενός αριθμού είναι η απόστασή του από το 0 στον άξονα."
                    },
                    "Σκέψου τον αριθμό πάνω στον άξονα των ρητών.",
                    if opp {
                        "Αλλάζει μόνο το πρόσημο.".to_string()
                    } else {
                        "Μια απόσταση δεν είναι ποτέ αρνητική.".to_string()
                    },
                    why.clone(),
                ),
                solution: why,
                diagnose: Box::new(move |v| {
                    if !opp && v[0].num() == Some(a) && a < 0 {
                        vec![dm("abs", "Η απόλυτη τιμή δεν είναι ποτέ αρνητική.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
        _ => {
            let (a, b): (R, R) = match r.r(0, 3) {
                0 => {
                    let a = nz(r, -20, -1);
                    (reduce(a, 1), reduce(nz(r, -20, -1), 1))
                }
                1 => (reduce(nz(r, -20, 20), 1), reduce(nz(r, -20, 20), 1)),
                2 => {
                    let a = small_frac(r);
                    let b = small_frac(r);
                    (
                        if a.0 > 0 { (-a.0, a.1) } else { a },
                        if b.0 > 0 { (-b.0, b.1) } else { b },
                    )
                }
                _ => (small_frac(r), small_frac(r)),
            };
            let ord = rcmp(a, b);
            let both_neg = a.0 < 0 && b.0 < 0;
            let abs_ord = rcmp((a.0.abs(), a.1), (b.0.abs(), b.1));
            let txt = |x: R| fr(x.0, x.1);
            let why = if both_neg && ord != Ordering::Equal {
                "Και οι δύο είναι αρνητικοί: μεγαλύτερος είναι αυτός με τη <b>μικρότερη</b> απόλυτη τιμή, δηλαδή ο πιο κοντά στο 0.".to_string()
            } else if a.0.signum() != b.0.signum() {
                "Κάθε θετικός είναι μεγαλύτερος από κάθε αρνητικό.".to_string()
            } else {
                "Σύγκρινε τους αριθμούς (για κλάσματα, κάν' τα ομώνυμα).".to_string()
            };
            let sym = match ord {
                Ordering::Less => "&lt;",
                Ordering::Equal => "=",
                Ordering::Greater => "&gt;",
            };
            Question {
                cat: "rat-abs",
                prompt: format!("Βάλε το σωστό σύμβολο (&lt;, = ή &gt;):<br><span class=\"big\">{} &nbsp;□&nbsp; {}</span>", txt(a), txt(b)),
                groups: one(vec![sign_select("Σύμβολο:", ord)]),
                hints: hints(
                    "Στον άξονα, μεγαλύτερος είναι ο αριθμός που βρίσκεται πιο δεξιά.",
                    "Κάθε θετικός είναι μεγαλύτερος από το 0 και από κάθε αρνητικό.",
                    "Από δύο αρνητικούς μεγαλύτερος είναι αυτός που είναι πιο κοντά στο 0.",
                    format!("{} {sym} {}", txt(a), txt(b)),
                ),
                solution: format!("{why}<br><b>{} {sym} {}</b>", txt(a), txt(b)),
                diagnose: Box::new(move |v| {
                    let chosen = v[0].num().map(|c| match c {
                        0 => Ordering::Less,
                        1 => Ordering::Equal,
                        _ => Ordering::Greater,
                    });
                    if both_neg && chosen == Some(abs_ord) && abs_ord != ord {
                        vec![dm("neg-compare", "Σύγκρινες τις απόλυτες τιμές. Στους αρνητικούς ισχύει το αντίθετο: −3 &gt; −7, γιατί το −3 είναι πιο κοντά στο 0.")]
                    } else {
                        vec![]
                    }
                }),
            }
        }
    }
}

pub fn rat_add(r: &mut Rng) -> Question {
    let fractions = r.chance(0.3);
    if fractions {
        let a = small_frac(r);
        let b = small_frac(r);
        let sub = r.chance(0.5);
        let res = if sub { rsub(a, b) } else { radd(a, b) };
        let l = lcm(a.1 as u64, b.1 as u64) as i64;
        let expr = format!("{} {} {}", rp(a), if sub { "−" } else { "+" }, rp(b));
        return Question {
            cat: "rat-add",
            prompt: format!("Υπολόγισε: <span class=\"big\">{expr}</span><br><span class=\"muted\">Γράψε το αποτέλεσμα ως ανάγωγο κλάσμα (ή ακέραιο).</span>"),
            groups: one(vec![frac("Αποτέλεσμα:", res.0, res.1)]),
            hints: hints(
                if sub { "Η αφαίρεση γίνεται πρόσθεση του αντιθέτου: α − β = α + (−β)." } else { "Για να προσθέσεις κλάσματα, κάν' τα πρώτα ομώνυμα." },
                format!("Κοινός παρονομαστής: ΕΚΠ({}, {}) = {l}.", a.1, b.1),
                format!("{} = {} και {} = {}", fr(a.0, a.1), fr_raw(a.0 * l / a.1, l), fr(b.0, b.1), fr_raw(b.0 * l / b.1, l)),
                format!("{expr} = {}", fr(res.0, res.1)),
            ),
            solution: format!(
                "{expr} = {} {} {} = {} = <b>{}</b>",
                fr_raw(a.0 * l / a.1, l),
                if sub { "−" } else { "+" },
                rp((b.0 * l / b.1, l)),
                fr_raw(if sub { a.0 * l / a.1 - b.0 * l / b.1 } else { a.0 * l / a.1 + b.0 * l / b.1 }, l),
                fr(res.0, res.1)
            ),
            diagnose: sign_diag(res),
        };
    }
    let n = r.r(2, 4) as usize;
    let nums: Vec<i64> = (0..n).map(|_| nz(r, -25, 25)).collect();
    let ops: Vec<bool> = (1..n).map(|_| r.chance(0.5)).collect(); // true = subtraction
    let mut expr = par(nums[0]);
    let mut total = nums[0];
    let mut as_add = vec![signed(nums[0])];
    let mut naive = nums[0];
    for i in 1..n {
        let x = nums[i];
        if ops[i - 1] {
            expr += &format!(" − {}", par(x));
            total -= x;
            naive -= x.abs();
            as_add.push(par(-x));
        } else {
            expr += &format!(" + {}", par(x));
            total += x;
            naive += x;
            as_add.push(par(x));
        }
    }
    let has_minus_neg = nums.iter().skip(1).zip(&ops).any(|(x, s)| *s && *x < 0);
    Question {
        cat: "rat-add",
        prompt: format!("Υπολόγισε: <span class=\"big\">{expr}</span>"),
        groups: one(vec![int("Αποτέλεσμα:", total)]),
        hints: hints(
            "Η αφαίρεση γίνεται πρόσθεση του αντιθέτου: α − β = α + (−β). Άρα − (−5) = + 5.",
            format!("Γράψε τα πάντα ως πρόσθεση: {}", as_add.join(" + ")),
            "Ομόσημοι: προσθέτεις τις απόλυτες τιμές και κρατάς το πρόσημο. Ετερόσημοι: αφαιρείς τις απόλυτες τιμές και κρατάς το πρόσημο του μεγαλύτερου.",
            format!("{expr} = {}", signed(total)),
        ),
        solution: format!("{expr} = {} = <b>{}</b>", as_add.join(" + "), signed(total)),
        diagnose: Box::new(move |v| {
            let u = v[0].num();
            if has_minus_neg && u == Some(naive) && naive != total {
                vec![dm("minus-minus", "Πρόσεξε το «− (−…)»: αφαιρώ αρνητικό σημαίνει προσθέτω τον αντίθετο, δηλαδή θετικό.")]
            } else if u == Some(-total) && total != 0 {
                vec![dm("sign", "Ο αριθμός είναι σωστός, αλλά το <b>πρόσημο</b> όχι.")]
            } else {
                vec![]
            }
        }),
    }
}

pub fn rat_mul(r: &mut Rng) -> Question {
    let (expr, res, steps): (String, R, String) = match r.r(0, 3) {
        0 => {
            let (a, b) = (nz(r, -12, 12), nz(r, -12, 12));
            (
                format!("{}·{}", par(a), par(b)),
                (a * b, 1),
                format!(
                    "{} επί {}: {}. Μέτρο: {}·{} = {}",
                    if a < 0 {
                        "Αρνητικός"
                    } else {
                        "Θετικός"
                    },
                    if b < 0 {
                        "αρνητικό"
                    } else {
                        "θετικό"
                    },
                    if (a < 0) == (b < 0) {
                        "ομόσημοι → θετικό"
                    } else {
                        "ετερόσημοι → αρνητικό"
                    },
                    a.abs(),
                    b.abs(),
                    (a * b).abs()
                ),
            )
        }
        1 => {
            let (a, b, c) = (nz(r, -6, 6), nz(r, -6, 6), nz(r, -5, 5));
            // at least one negative factor: this section is about signs
            let c = if a > 0 && b > 0 && c > 0 { -c } else { c };
            let negs = [a, b, c].iter().filter(|x| **x < 0).count();
            (
                format!("{}·{}·{}", par(a), par(b), par(c)),
                (a * b * c, 1),
                format!(
                    "Πλήθος αρνητικών παραγόντων: {negs} ({}) → πρόσημο {}. Μέτρο: {}·{}·{} = {}",
                    if negs % 2 == 0 {
                        "άρτιο"
                    } else {
                        "περιττό"
                    },
                    if negs % 2 == 0 { "+" } else { "−" },
                    a.abs(),
                    b.abs(),
                    c.abs(),
                    (a * b * c).abs()
                ),
            )
        }
        2 => {
            let (q, b) = (nz(r, -12, 12), nz(r, -9, 9));
            let a = q * b;
            (
                format!("{} : {}", par(a), par(b)),
                (q, 1),
                format!(
                    "Ο κανόνας προσήμων ισχύει και στη διαίρεση: {}. {} : {} = {}",
                    if (a < 0) == (b < 0) {
                        "ομόσημοι → θετικό"
                    } else {
                        "ετερόσημοι → αρνητικό"
                    },
                    a.abs(),
                    b.abs(),
                    q.abs()
                ),
            )
        }
        _ => {
            let a = small_frac(r);
            let b = small_frac(r);
            if r.chance(0.5) {
                let res = rmul(a, b);
                (
                    format!("{}·{}", rp(a), rp(b)),
                    res,
                    format!(
                        "Αριθμητής επί αριθμητή, παρονομαστής επί παρονομαστή: {} = {}",
                        fr_raw(a.0 * b.0, a.1 * b.1),
                        fr(res.0, res.1)
                    ),
                )
            } else {
                let res = rdiv(a, b);
                (
                    format!("{} : {}", rp(a), rp(b)),
                    res,
                    format!(
                        "Διαιρώ = πολλαπλασιάζω με τον αντίστροφο: {}·{} = {}",
                        rp(a),
                        rp(reduce(b.1, b.0)),
                        fr(res.0, res.1)
                    ),
                )
            }
        }
    };
    Question {
        cat: "rat-mul",
        prompt: format!("Υπολόγισε: <span class=\"big\">{expr}</span>{}", if res.1 != 1 || expr.contains("fr") { "<br><span class=\"muted\">Γράψε το αποτέλεσμα ως ανάγωγο κλάσμα (ή ακέραιο).</span>" } else { "" }),
        groups: one(vec![if expr.contains("fr") { frac("Αποτέλεσμα:", res.0, res.1) } else { answer_part("Αποτέλεσμα:", res) }]),
        hints: hints(
            "Κανόνας προσήμων: ομόσημοι δίνουν θετικό, ετερόσημοι αρνητικό. Το ίδιο και στη διαίρεση.",
            "Με πολλούς παράγοντες: άρτιο πλήθος αρνητικών → θετικό, περιττό → αρνητικό.",
            "Βρες πρώτα το πρόσημο και μετά κάνε την πράξη με τις απόλυτες τιμές.",
            steps.clone(),
        ),
        solution: format!("{steps}<br><b>{expr} = {}</b>", fr(res.0, res.1)),
        diagnose: sign_diag(res),
    }
}

pub fn rat_pow(r: &mut Rng) -> Question {
    let (expr, res, why, trap): (String, R, String, Option<i64>) = match r.r(0, 3) {
        0 => {
            let a = nz(r, -6, -2);
            let n = r.r(2, 4) as u32;
            let v = a.pow(n);
            (
                format!("({})<sup>{n}</sup>", signed(a)),
                (v, 1),
                format!(
                    "({}){} = {} = {}. {} εκθέτης → {} αποτέλεσμα.",
                    signed(a),
                    sup(n),
                    vec![par(a); n as usize].join("·"),
                    signed(v),
                    if n.is_multiple_of(2) {
                        "Άρτιος"
                    } else {
                        "Περιττός"
                    },
                    if n.is_multiple_of(2) {
                        "θετικό"
                    } else {
                        "αρνητικό"
                    }
                ),
                None,
            )
        }
        1 => {
            let a = r.r(2, 5) as i64;
            let n = r.pick(&[2u32, 4]);
            let v = -a.pow(n);
            (
                format!("−{a}<sup>{n}</sup>"),
                (v, 1),
                format!(
                    "Η δύναμη αφορά μόνο το {a} (δεν υπάρχει παρένθεση): −{a}{} = −({}) = {}.",
                    sup(n),
                    a.pow(n),
                    signed(v)
                ),
                Some(a.pow(n)),
            )
        }
        2 => {
            let n = r.r(5, 40) as u32;
            let v = if n.is_multiple_of(2) { 1 } else { -1 };
            (
                format!("(−1)<sup>{n}</sup>"),
                (v, 1),
                format!(
                    "Ο εκθέτης {n} είναι {}, άρα (−1){} = {}.",
                    if n.is_multiple_of(2) {
                        "άρτιος"
                    } else {
                        "περιττός"
                    },
                    sup(n),
                    signed(v)
                ),
                None,
            )
        }
        _ => {
            let (num_, den) = (r.r(1, 3) as i64, r.r(2, 5) as i64);
            let (num_, den) = reduce(num_, den);
            let n = r.r(2, 3) as u32;
            let v = reduce((-num_).pow(n), den.pow(n));
            (
                format!("({})<sup>{n}</sup>", fr(-num_, den)),
                v,
                format!(
                    "Υψώνουμε αριθμητή και παρονομαστή: {} = {}.",
                    fr_raw((-num_).pow(n), den.pow(n)),
                    fr(v.0, v.1)
                ),
                None,
            )
        }
    };
    Question {
        cat: "rat-pow",
        prompt: format!("Υπολόγισε: <span class=\"big\">{expr}</span>"),
        groups: one(vec![answer_part("Αποτέλεσμα:", res)]),
        hints: hints(
            "Αρνητικός αριθμός σε άρτιο εκθέτη δίνει θετικό, σε περιττό εκθέτη αρνητικό.",
            "Πρόσεξε την παρένθεση: (−2)⁴ = 16, αλλά −2⁴ = −16.",
            "Γράψε τη δύναμη ως γινόμενο ίσων παραγόντων.",
            why.clone(),
        ),
        solution: why.to_string(),
        diagnose: Box::new(move |v| {
            let u = v[0].num();
            if trap.is_some() && u == trap {
                vec![dm("neg-pow", "Χωρίς παρένθεση η δύναμη αφορά μόνο τον αριθμό, όχι το πρόσημο: −2⁴ = −(2·2·2·2).")]
            } else if res.1 == 1 && u == Some(-res.0) {
                vec![dm(
                    "sign",
                    "Πρόσεξε το πρόσημο: άρτιος εκθέτης → θετικό, περιττός εκθέτης → αρνητικό.",
                )]
            } else {
                vec![]
            }
        }),
    }
}

pub fn rat_ops(r: &mut Rng) -> Question {
    let (a, b, c) = (nz(r, -9, 9), nz(r, -6, 6), nz(r, -9, 9));
    let (expr, val, wrong, steps): (String, i64, i64, String) = match r.r(0, 3) {
        0 => (
            format!("{} + {}·{}", par(a), par(b), par(c)),
            a + b * c,
            (a + b) * c,
            format!(
                "Πρώτα ο πολλαπλασιασμός: {}·{} = {}. Μετά {} + {} = {}",
                par(b),
                par(c),
                signed(b * c),
                par(a),
                par(b * c),
                signed(a + b * c)
            ),
        ),
        1 => {
            let q = b * c;
            (
                format!("{} : {} − {}²", par(q), par(b), par(a.abs().min(5))),
                q / b - a.abs().min(5).pow(2),
                q / b + a.abs().min(5).pow(2),
                format!(
                    "{} : {} = {}, {}² = {}, {} − {} = {}",
                    par(q),
                    par(b),
                    signed(c),
                    a.abs().min(5),
                    a.abs().min(5).pow(2),
                    signed(c),
                    a.abs().min(5).pow(2),
                    signed(c - a.abs().min(5).pow(2))
                ),
            )
        }
        2 => (
            format!("{} − {}·({} − {})", par(a), par(b), par(c), par(a)),
            a - b * (c - a),
            (a - b) * (c - a),
            format!(
                "Παρένθεση: {} − {} = {}. Μετά {}·{} = {}. Τέλος {} − {} = {}",
                par(c),
                par(a),
                signed(c - a),
                par(b),
                par(c - a),
                signed(b * (c - a)),
                par(a),
                par(b * (c - a)),
                signed(a - b * (c - a))
            ),
        ),
        _ => (
            format!("({} + {})·{} − {}", par(a), par(b), par(c), par(b)),
            (a + b) * c - b,
            a + b * c - b,
            format!(
                "Παρένθεση: {} + {} = {}. {}·{} = {}. {} − {} = {}",
                par(a),
                par(b),
                signed(a + b),
                par(a + b),
                par(c),
                signed((a + b) * c),
                signed((a + b) * c),
                par(b),
                signed((a + b) * c - b)
            ),
        ),
    };
    Question {
        cat: "rat-ops",
        prompt: format!("Υπολόγισε την τιμή της παράστασης: <span class=\"big\">{expr}</span>"),
        groups: one(vec![int("Τιμή:", val)]),
        hints: hints(
            "Σειρά πράξεων: παρενθέσεις, δυνάμεις, πολλαπλασιασμοί/διαιρέσεις, προσθέσεις/αφαιρέσεις.",
            "Κάθε πράξη με αρνητικούς ακολουθεί τον κανόνα των προσήμων.",
            steps.split(". ").next().unwrap_or("").to_string(),
            format!("{expr} = {}", signed(val)),
        ),
        solution: format!("{steps}<br><b>{expr} = {}</b>", signed(val)),
        diagnose: Box::new(move |v| {
            let u = v[0].num();
            if u == Some(wrong) && wrong != val {
                vec![dm("order", "Πρόσεξε τη σειρά των πράξεων: ο πολλαπλασιασμός και η διαίρεση γίνονται πριν από την πρόσθεση και την αφαίρεση.")]
            } else if u == Some(-val) && val != 0 {
                vec![dm("sign", "Ο αριθμός είναι σωστός, αλλά το πρόσημο όχι.")]
            } else {
                vec![]
            }
        }),
    }
}

fn only_2_5(mut d: i64) -> bool {
    while d % 2 == 0 {
        d /= 2;
    }
    while d % 5 == 0 {
        d /= 5;
    }
    d == 1
}

pub fn rat_per(r: &mut Rng) -> Question {
    match r.r(0, 2) {
        0 => {
            let (n, d) = loop {
                let d = r.pick(&[3i64, 4, 6, 7, 8, 9, 11, 12, 15, 16, 20, 25, 30, 40]);
                let n = r.r(1, (d - 1) as u64) as i64;
                let (n2, d2) = reduce(n, d);
                if d2 > 1 {
                    break (n2, d2);
                }
            };
            let fin = only_2_5(d);
            let f = factor(d as u64);
            Question {
                cat: "rat-per",
                prompt: format!("Ο ανάγωγος ρητός <span class=\"big\">{}</span> γράφεται ως δεκαδικός με <b>πεπερασμένο</b> πλήθος δεκαδικών ψηφίων;", fr(n, d)),
                groups: one(vec![yesno(fin)]),
                hints: hints(
                    "Ένας ανάγωγος ρητός γίνεται δεκαδικός με πεπερασμένα ψηφία αν και μόνο αν ο παρονομαστής έχει πρώτους παράγοντες μόνο 2 και 5.",
                    "Ανάλυσε τον παρονομαστή σε πρώτους παράγοντες.",
                    format!("{d} = {}", fmt_pow(&f)),
                    if fin { "Μόνο 2 και 5 → πεπερασμένος.".to_string() } else { "Υπάρχει άλλος πρώτος παράγοντας → περιοδικός.".to_string() },
                ),
                solution: format!("{d} = {}. {}", fmt_pow(&f), if fin { "Οι παράγοντες είναι μόνο 2 και 5, άρα ο αριθμός είναι δεκαδικός με <b>πεπερασμένα</b> ψηφία." } else { "Υπάρχει πρώτος παράγοντας εκτός από 2 και 5, άρα ο αριθμός είναι <b>περιοδικός</b> δεκαδικός." }),
                diagnose: Box::new(|_| vec![tag("periodic")]),
            }
        }
        1 => {
            // periodic -> fraction
            let (shown, res, how) = match r.r(0, 2) {
                0 => {
                    let a = r.r(1, 8) as i64;
                    (
                        format!("0,<span class=\"ov\">{a}</span>"),
                        reduce(a, 9),
                        format!(
                            "x = 0,{a}{a}{a}…, 10x = {a},{a}{a}…, 10x − x = {a}, 9x = {a}, x = {}",
                            fr_raw(a, 9)
                        ),
                    )
                }
                1 => {
                    let ab = loop {
                        let x = r.r(10, 98) as i64;
                        if x % 11 != 0 {
                            break x;
                        }
                    };
                    (format!("0,<span class=\"ov\">{ab}</span>"), reduce(ab, 99), format!("x = 0,{ab}{ab}…, 100x = {ab},{ab}…, 100x − x = {ab}, 99x = {ab}, x = {}", fr_raw(ab, 99)))
                }
                _ => {
                    let a = r.r(1, 9) as i64;
                    let b = loop {
                        let b = r.r(1, 9) as i64;
                        if b != a {
                            break b;
                        }
                    };
                    let ab = a * 10 + b;
                    (format!("0,{a}<span class=\"ov\">{b}</span>"), reduce(ab - a, 90), format!("x = 0,{a}{b}{b}…, 100x = {ab},{b}{b}…, 10x = {a},{b}{b}…, 100x − 10x = {ab} − {a}, 90x = {}, x = {}", ab - a, fr_raw(ab - a, 90)))
                }
            };
            Question {
                cat: "rat-per",
                prompt: format!("Μετάτρεψε τον περιοδικό δεκαδικό <span class=\"big\">{shown}</span> σε ανάγωγο κλάσμα."),
                groups: one(vec![frac("Κλάσμα:", res.0, res.1)]),
                hints: hints(
                    "Θέσε τον αριθμό ίσο με x.",
                    "Πολλαπλασίασε με 10, 100, … ώστε το περιοδικό τμήμα να «ευθυγραμμιστεί», και αφαίρεσε κατά μέλη.",
                    how.clone(),
                    format!("Απλοποίησε: {}", fr(res.0, res.1)),
                ),
                solution: format!("{how} = <b>{}</b>", fr(res.0, res.1)),
                diagnose: Box::new(|_| vec![tag("periodic")]),
            }
        }
        _ => {
            // fraction -> periodic part and period
            let (n, d, rep, pre) = loop {
                let d = r.pick(&[3i64, 6, 7, 9, 11, 12, 15, 27, 33, 37]);
                let n = r.r(1, (d - 1) as u64) as i64;
                let (n, d) = reduce(n, d);
                if d == 1 || only_2_5(d) {
                    continue;
                }
                // long division to find the repetend
                let mut seen = std::collections::HashMap::new();
                let mut digits = String::new();
                let mut rem = n % d;
                while !seen.contains_key(&rem) {
                    seen.insert(rem, digits.len());
                    rem *= 10;
                    digits.push(char::from(b'0' + (rem / d) as u8));
                    rem %= d;
                }
                let start = seen[&rem];
                let rep = digits[start..].to_string();
                if rep.starts_with('0') || rep.len() > 3 {
                    continue;
                }
                break (n, d, rep, digits[..start].to_string());
            };
            let simple = pre.is_empty();
            let shown = format!("0,{pre}<span class=\"ov\">{rep}</span>");
            Question {
                cat: "rat-per",
                prompt: format!("Γράψε τον ρητό <span class=\"big\">{}</span> ως δεκαδικό (κάνε τη διαίρεση {n} : {d}).", fr(n, d)),
                groups: vec![
                    group("α) Ποιο είναι το περιοδικό τμήμα;", vec![num("Περιοδικό τμήμα:", rep.parse().unwrap())]),
                    group("β) Ποια είναι η περίοδος (πόσα ψηφία έχει το περιοδικό τμήμα);", vec![num("Περίοδος:", rep.len() as u64)]),
                    group("γ) Ο αριθμός είναι απλός ή μικτός περιοδικός;", vec![select("Είδος:", vec![(0, "απλός περιοδικός".into()), (1, "μικτός περιοδικός".into())], if simple { 0 } else { 1 })]),
                ],
                hints: hints(
                    format!("Κάνε τη διαίρεση {n} : {d} με δεκαδικά, μέχρι να αρχίσουν να επαναλαμβάνονται τα υπόλοιπα."),
                    "Περιοδικό τμήμα είναι τα ψηφία που επαναλαμβάνονται. Περίοδος είναι το πλήθος τους.",
                    "Απλός: η επανάληψη αρχίζει αμέσως μετά την υποδιαστολή. Μικτός: υπάρχουν ψηφία πριν αρχίσει.",
                    format!("{} = {shown}", fr(n, d)),
                ),
                solution: format!("{} = 0,{pre}{rep}{rep}… = <b>{shown}</b>. Περιοδικό τμήμα {rep}, περίοδος {}, {} περιοδικός.", fr(n, d), rep.len(), if simple { "απλός" } else { "μικτός" }),
                diagnose: Box::new(|_| vec![tag("periodic")]),
            }
        }
    }
}

pub fn rat_eq(r: &mut Rng) -> Question {
    let x = "<i>x</i>";
    let int_sol = r.chance(0.7);
    let sol: R = if int_sol {
        (nz(r, -12, 12), 1)
    } else {
        small_frac(r)
    };
    let a = nz(r, -9, 9);
    let b = nz(r, -15, 15);
    let rhs = radd(rmul((a, 1), sol), (b, 1));
    let bstr = if b < 0 {
        format!("− {}", -b)
    } else {
        format!("+ {b}")
    };
    let ax = if a == 1 {
        x.to_string()
    } else if a == -1 {
        format!("−{x}")
    } else {
        format!("{}{x}", signed(a))
    };
    let equation = format!("{ax} {bstr} = {}", fr(rhs.0, rhs.1));
    let moved = rsub(rhs, (b, 1));
    let wrong = rdiv(radd(rhs, (b, 1)), (a, 1));
    Question {
        cat: "rat-eq",
        prompt: format!("Λύσε την εξίσωση <span class=\"big\">{equation}</span>.<br><span class=\"muted\">Αν η λύση δεν είναι ακέραιος, γράψ' την ως ανάγωγο κλάσμα.</span>"),
        groups: one(vec![frac("x =", sol.0, sol.1)]),
        hints: hints(
            "Μεταφέρουμε τους γνωστούς όρους στο δεύτερο μέλος: όταν ένας όρος αλλάζει μέλος, αλλάζει και πρόσημο.",
            format!("{ax} = {} {} {} = {}", fr(rhs.0, rhs.1), if b < 0 { "+" } else { "−" }, b.abs(), fr(moved.0, moved.1)),
            format!("Διαίρεσε και τα δύο μέλη με τον συντελεστή του x, δηλαδή με {}.", par(a)),
            format!("x = {}", fr(sol.0, sol.1)),
        ),
        solution: format!(
            "{equation}<br>⇔ {ax} = {} {} {}<br>⇔ {ax} = {}<br>⇔ x = {} : {} = <b>{}</b>",
            fr(rhs.0, rhs.1),
            if b < 0 { "+" } else { "−" },
            b.abs(),
            fr(moved.0, moved.1),
            rp(moved),
            par(a),
            fr(sol.0, sol.1)
        ),
        diagnose: Box::new(move |v| {
            let u = match v[0] {
                Val::Frac(n, d) => Some(reduce(n, d)),
                Val::Num(n) => Some((n, 1)),
                _ => None,
            };
            if u == Some(wrong) && wrong != sol {
                vec![dm("eq-move", "Όταν ένας όρος αλλάζει μέλος, <b>αλλάζει πρόσημο</b>.")]
            } else if u == Some((-sol.0, sol.1)) && sol.0 != 0 {
                vec![dm("sign", "Ο αριθμός είναι σωστός, αλλά το πρόσημο όχι. Πρόσεξε τη διαίρεση με αρνητικό συντελεστή.")]
            } else {
                vec![]
            }
        }),
    }
}
