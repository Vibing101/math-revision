//! Small constructors that keep the generators readable.

use crate::model::*;
use crate::util::{PowMap, DAYS};

pub fn num(label: impl Into<String>, a: u64) -> Part {
    Part {
        label: label.into(),
        kind: Kind::Num,
        answer: Answer::Num(a as i64),
    }
}
pub fn yesno(a: bool) -> Part {
    Part {
        label: String::new(),
        kind: Kind::YesNo {
            yes: "Ναι",
            no: "Όχι",
        },
        answer: Answer::Bool(a),
    }
}
pub fn truefalse(a: bool) -> Part {
    Part {
        label: String::new(),
        kind: Kind::YesNo {
            yes: "Σωστό",
            no: "Λάθος",
        },
        answer: Answer::Bool(a),
    }
}
pub fn set(options: Vec<u64>, ans: Vec<u64>) -> Part {
    Part {
        label: String::new(),
        kind: Kind::Set {
            options: options.iter().map(|x| *x as i64).collect(),
            labels: vec![],
        },
        answer: Answer::Set(ans.iter().map(|x| *x as i64).collect()),
    }
}
pub fn select(label: impl Into<String>, options: Vec<(i64, String)>, ans: i64) -> Part {
    Part {
        label: label.into(),
        kind: Kind::Select { options },
        answer: Answer::Choice(ans),
    }
}
pub fn day_select(ans: usize) -> Part {
    select(
        "Μέρα:",
        DAYS.iter()
            .enumerate()
            .map(|(i, d)| (i as i64, d.to_string()))
            .collect(),
        ans as i64,
    )
}
pub fn exp(prefix: impl Into<String>, primes: Vec<u64>, ans: PowMap) -> Part {
    Part {
        label: String::new(),
        kind: Kind::Exp {
            prefix: prefix.into(),
            primes,
        },
        answer: Answer::Pow(ans),
    }
}
pub fn group(label: impl Into<String>, parts: Vec<Part>) -> Group {
    Group {
        label: label.into(),
        parts,
    }
}
pub fn one(parts: Vec<Part>) -> Vec<Group> {
    vec![group("", parts)]
}
pub fn tag(t: &'static str) -> Diag {
    Diag {
        tag: Some(t),
        msg: None,
    }
}
pub fn dm(t: &'static str, m: impl Into<String>) -> Diag {
    Diag {
        tag: Some(t),
        msg: Some(m.into()),
    }
}
pub fn msg(m: impl Into<String>) -> Diag {
    Diag {
        tag: None,
        msg: Some(m.into()),
    }
}
pub fn no_diag() -> DiagFn {
    Box::new(|_| vec![])
}
pub fn hints(
    a: impl Into<String>,
    b: impl Into<String>,
    c: impl Into<String>,
    d: impl Into<String>,
) -> [String; 4] {
    [a.into(), b.into(), c.into(), d.into()]
}
pub fn yn(b: bool) -> &'static str {
    if b {
        ""
    } else {
        "δεν "
    }
}

/// Multi-select over text options (e.g. letters); the answer is option indices.
pub fn set_labeled(labels: Vec<String>, ans: Vec<usize>) -> Part {
    Part {
        label: String::new(),
        kind: Kind::Set {
            options: (0..labels.len() as i64).collect(),
            labels,
        },
        answer: Answer::Set(ans.iter().map(|x| *x as i64).collect()),
    }
}
/// Signed integer answer.
pub fn int(label: impl Into<String>, a: i64) -> Part {
    Part {
        label: label.into(),
        kind: Kind::Int,
        answer: Answer::Num(a),
    }
}
/// Fraction answer, stored in lowest terms.
pub fn frac(label: impl Into<String>, n: i64, d: i64) -> Part {
    let (n, d) = crate::util::reduce(n, d);
    Part {
        label: label.into(),
        kind: Kind::Frac,
        answer: Answer::Frac(n, d),
    }
}
pub fn sign_select(label: impl Into<String>, ans: std::cmp::Ordering) -> Part {
    use std::cmp::Ordering::*;
    let v = match ans {
        Less => 0,
        Equal => 1,
        Greater => 2,
    };
    select(
        label,
        vec![(0, "<".into()), (1, "=".into()), (2, ">".into())],
        v,
    )
}
