//! Number theory helpers, a small deterministic RNG and Greek text formatting.

use std::collections::BTreeMap;

/// Prime -> exponent.
pub type PowMap = BTreeMap<u64, u32>;

/// SplitMix64: tiny, fast and deterministic, so a question can be rebuilt from its seed.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xD1B5_4A32_D192_ED03)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform integer in `a..=b`.
    pub fn r(&mut self, a: u64, b: u64) -> u64 {
        a + self.next_u64() % (b - a + 1)
    }
    pub fn chance(&mut self, p: f64) -> bool {
        ((self.next_u64() >> 11) as f64 / (1u64 << 53) as f64) < p
    }
    pub fn pick<T: Clone>(&mut self, v: &[T]) -> T {
        v[(self.next_u64() % v.len() as u64) as usize].clone()
    }
    pub fn shuffle<T: Clone>(&mut self, v: &[T]) -> Vec<T> {
        let mut out = v.to_vec();
        for i in (1..out.len()).rev() {
            let j = (self.next_u64() % (i as u64 + 1)) as usize;
            out.swap(i, j);
        }
        out
    }
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
pub fn gcd_all(v: &[u64]) -> u64 {
    v.iter().copied().reduce(gcd).unwrap_or(0)
}
pub fn lcm_all(v: &[u64]) -> u64 {
    v.iter().copied().reduce(lcm).unwrap_or(1)
}

pub fn factor(mut n: u64) -> PowMap {
    let mut m = PowMap::new();
    let mut p = 2;
    while n > 1 {
        if n.is_multiple_of(p) {
            *m.entry(p).or_insert(0) += 1;
            n /= p;
        } else {
            p += 1;
        }
    }
    m
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

pub fn divisors(n: u64) -> Vec<u64> {
    (1..=n).filter(|d| n.is_multiple_of(*d)).collect()
}

pub fn sup(n: u32) -> String {
    const S: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    n.to_string()
        .chars()
        .map(|c| S[c.to_digit(10).unwrap() as usize])
        .collect()
}

/// `2³·3·5²` (exponent 1 omitted, `1` for the empty product).
pub fn fmt_pow(m: &PowMap) -> String {
    let parts: Vec<String> = m
        .iter()
        .filter(|(_, e)| **e > 0)
        .map(|(p, e)| {
            if *e > 1 {
                format!("{p}{}", sup(*e))
            } else {
                p.to_string()
            }
        })
        .collect();
    if parts.is_empty() {
        "1".into()
    } else {
        parts.join("·")
    }
}

/// `2³·3¹·5²` (every exponent shown), used when the exponents are the point.
pub fn fmt_pow_full(m: &PowMap) -> String {
    m.iter()
        .filter(|(_, e)| **e > 0)
        .map(|(p, e)| format!("{p}{}", sup(*e)))
        .collect::<Vec<_>>()
        .join("·")
}

/// `8·3·25`: each prime power written out as its value.
pub fn expand(m: &PowMap) -> String {
    m.iter()
        .filter(|(_, e)| **e > 0)
        .map(|(p, e)| p.pow(*e).to_string())
        .collect::<Vec<_>>()
        .join("·")
}

pub fn map_val(m: &PowMap) -> u64 {
    m.iter().map(|(p, e)| p.pow(*e)).product()
}

pub fn min_map(ms: &[PowMap]) -> PowMap {
    let mut r = PowMap::new();
    let primes: std::collections::BTreeSet<u64> =
        ms.iter().flat_map(|m| m.keys().copied()).collect();
    for p in primes {
        let e = ms
            .iter()
            .map(|m| *m.get(&p).unwrap_or(&0))
            .min()
            .unwrap_or(0);
        if e > 0 {
            r.insert(p, e);
        }
    }
    r
}

pub fn max_map(ms: &[PowMap]) -> PowMap {
    let mut r = PowMap::new();
    for m in ms {
        for (p, e) in m {
            let x = r.entry(*p).or_insert(0);
            *x = (*x).max(*e);
        }
    }
    r
}

/// The vertical "ladder" layout used in Cypriot classrooms for prime factorisation.
pub fn ladder(n: u64, limit: Option<usize>) -> String {
    let (mut rows, mut x, mut p, mut c) = (String::new(), n, 2u64, 0usize);
    while x > 1 && limit.is_none_or(|l| c < l) {
        if x % p == 0 {
            rows += &format!("<tr><td>{x}</td><td>{p}</td></tr>");
            x /= p;
            c += 1;
        } else {
            p += 1;
        }
    }
    if x == 1 {
        rows += "<tr><td>1</td><td></td></tr>";
    } else {
        rows += &format!("<tr><td>{x}</td><td>…</td></tr>");
    }
    format!("<table class=\"ladder\">{rows}</table>")
}

pub fn ladders(ns: &[u64]) -> String {
    format!(
        "<div class=\"ladders\">{}</div>",
        ns.iter().map(|n| ladder(*n, None)).collect::<String>()
    )
}

pub fn fac_lines(ns: &[u64]) -> String {
    ns.iter()
        .map(|n| format!("{n} = {}", fmt_pow(&factor(*n))))
        .collect::<Vec<_>>()
        .join("<br>")
}

/// LCM values a pupil gets by taking one prime with a smaller exponent (or leaving it out).
pub fn lcm_mistakes(ns: &[u64]) -> Vec<(u64, &'static str)> {
    let ms: Vec<PowMap> = ns.iter().map(|n| factor(*n)).collect();
    let mx = max_map(&ms);
    let l = map_val(&mx);
    let mut out = Vec::new();
    for (p, emax) in &mx {
        for m in &ms {
            let e = *m.get(p).unwrap_or(&0);
            if e < *emax {
                out.push((
                    l / p.pow(emax - e),
                    if e == 0 { "lcm-missing" } else { "lcm-min" },
                ));
            }
        }
    }
    out
}

pub fn list(v: &[u64]) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// "α, β και γ"
pub fn join_and(items: &[String]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        n => format!("{} και {}", items[..n - 1].join(", "), items[n - 1]),
    }
}

/// Greek thousands separator: 12.345
pub fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push('.');
        }
        out.push(c);
    }
    out
}

pub const DAYS: [&str; 7] = [
    "Δευτέρα",
    "Τρίτη",
    "Τετάρτη",
    "Πέμπτη",
    "Παρασκευή",
    "Σάββατο",
    "Κυριακή",
];

pub fn day_art(d: usize) -> String {
    format!("{}{}", if d == 5 { "το " } else { "την " }, DAYS[d])
}

/// Hour of day as Greek clock text.
pub fn tl(h: u64) -> String {
    match h {
        0 => "12:00 τα μεσάνυχτα (00:00)".into(),
        12 => "12:00 το μεσημέρι".into(),
        h if h < 12 => format!("{h}:00 π.μ."),
        h => format!("{}:00 μ.μ. ({h}:00)", h - 12),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn helpers() {
        assert_eq!(fmt_pow(&factor(360)), "2³·3²·5");
        assert_eq!(lcm_all(&[12, 15, 20]), 60);
        assert_eq!(gcd_all(&[144, 90, 54]), 18);
        assert_eq!(thousands(1234567), "1.234.567");
        assert!(lcm_mistakes(&[40, 50, 144])
            .iter()
            .any(|(v, t)| *v == 720 && *t == "lcm-min"));
        assert!(is_prime(97) && !is_prime(91) && !is_prime(1));
        assert_eq!(tl(22), "10:00 μ.μ. (22:00)");
    }
}
