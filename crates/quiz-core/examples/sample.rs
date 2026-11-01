//! Print sample questions: cargo run -p quiz-core --example sample -- <cat> [count]
fn strip(s: &str) -> String {
    let mut out = String::new();
    let mut tag = false;
    for c in s.chars() {
        match c {
            '<' => tag = true,
            '>' => tag = false,
            _ if !tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(3);
    for seed in 0..n {
        let q = quiz_core::generate(&args[1], seed * 7 + 1).expect("unknown category");
        println!("--- {} #{seed}\nQ: {}", q.cat, strip(&q.prompt));
        for g in &q.groups {
            let ans: Vec<String> = g
                .parts
                .iter()
                .map(|p| format!("[{}] {}", strip(&p.label), quiz_core::answer_text(p)))
                .collect();
            println!("   {} => {}", strip(&g.label), ans.join(" | "));
        }
        println!("S: {}", strip(&q.solution.replace("<br>", " / ")));
    }
}
