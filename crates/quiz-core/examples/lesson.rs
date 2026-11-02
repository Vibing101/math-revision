//! Prints the lessons for a category: `cargo run -p quiz-core --example lesson -- c-factor 3`
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let cat = a.get(1).map(String::as_str).unwrap_or("num");
    let n: u32 = a.get(2).and_then(|x| x.parse().ok()).unwrap_or(2);
    for seed in 1..=n {
        let q = quiz_core::generate(cat, seed).expect("category");
        println!("=== {cat} seed {seed}: {}", q.prompt);
        let mut ls = vec![quiz_core::lesson::instructions(&q)];
        ls.extend(quiz_core::lesson::method(&q));
        for l in ls {
            println!("--- {}", l.title);
            for s in l.steps {
                println!("[{}]\n  show: {}\n  say:  {}", s.title, s.show, s.say);
            }
        }
    }
}
