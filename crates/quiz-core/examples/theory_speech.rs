//! Lists everything the theory pages read aloud, one sentence per line: `chapter key k characters`.
//! A summary goes to stderr. Used to size and to warm the audio cache:
//!
//!     cargo run -q -p quiz-core --release --example theory_speech -- public/theory
use quiz_core::theory::{keys, parts};

fn main() {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "public/theory".into());
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("theory folder")
        .filter_map(|f| f.ok().map(|f| f.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    let (mut sentences, mut chars) = (0usize, 0usize);
    for f in files {
        let ch = f.file_stem().unwrap().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&f).expect("read");
        for key in keys(&text) {
            for (k, s) in parts(&text, &key).unwrap_or_default().iter().enumerate() {
                println!("{ch} {key} {k} {}", s.chars().count());
                sentences += 1;
                chars += s.chars().count();
            }
        }
    }
    eprintln!("{sentences} sentences, {chars} characters");
}
