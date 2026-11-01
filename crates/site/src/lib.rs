//! Cloudflare Worker for maths.savvy-des.com.
//!
//! Static files (the page itself) are served by Workers Assets; this Worker only
//! answers `/api/*`:
//!
//! | Route                          | What it does                                      |
//! |--------------------------------|---------------------------------------------------|
//! | GET  /api/categories           | categories and mistake labels                     |
//! | GET  /api/question?cat&seed    | a question without its answers                    |
//! | GET  /api/hint?cat&seed&n      | hint n (0–3)                                      |
//! | POST /api/check                | checks answers, returns feedback / solution       |
//! | POST /api/teacher/login        | PIN → signed session token                        |
//! | GET  /api/teacher/resources    | printable test and its solutions (token required) |
//! | GET  /api/teacher/worksheet    | worksheet with answer key (token required)        |
//!
//! The teacher PIN lives in the `TEACHER_PIN` secret, never in the page.

use hmac::{Hmac, Mac};
use quiz_core::{answer_text, check, generate, public, CATEGORIES, TAGS};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::Sha256;
use worker::*;

const TOKEN_HOURS: u64 = 12;
const EXAM: &str = include_str!("../teacher/exam.html");
const SOLUTIONS: &str = include_str!("../teacher/solutions.html");

fn json_res(v: &Value, status: u16) -> Result<Response> {
    let mut r = Response::from_json(v)?.with_status(status);
    r.headers_mut().set("cache-control", "no-store")?;
    r.headers_mut().set("x-content-type-options", "nosniff")?;
    Ok(r)
}
fn bad(msg: &str, status: u16) -> Result<Response> {
    json_res(&json!({ "error": msg }), status)
}

fn query(req: &Request) -> Result<std::collections::HashMap<String, String>> {
    Ok(req.url()?.query_pairs().into_owned().collect())
}

fn seed_of(q: &std::collections::HashMap<String, String>) -> Option<u32> {
    q.get("seed")?.parse().ok()
}

fn now_secs() -> u64 {
    Date::now().as_millis() / 1000
}

fn mac(pin: &str) -> Hmac<Sha256> {
    Hmac::<Sha256>::new_from_slice(format!("math-revision/teacher/{pin}").as_bytes())
        .expect("any key length works")
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
        .collect()
}

fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn make_token(pin: &str) -> (String, u64) {
    let exp = now_secs() + TOKEN_HOURS * 3600;
    let mut m = mac(pin);
    m.update(exp.to_string().as_bytes());
    (format!("{exp}.{}", hex(&m.finalize().into_bytes())), exp)
}

fn token_ok(pin: &str, token: &str) -> bool {
    let Some((exp, sig)) = token.split_once('.') else {
        return false;
    };
    let (Ok(exp_n), Some(sig)) = (exp.parse::<u64>(), unhex(sig)) else {
        return false;
    };
    if exp_n < now_secs() {
        return false;
    }
    let mut m = mac(pin);
    m.update(exp.as_bytes());
    m.verify_slice(&sig).is_ok()
}

fn teacher_pin(env: &Env) -> Option<String> {
    env.secret("TEACHER_PIN")
        .ok()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
}

fn authorised(req: &Request, env: &Env) -> bool {
    let Some(pin) = teacher_pin(env) else {
        return false;
    };
    let header = req
        .headers()
        .get("authorization")
        .ok()
        .flatten()
        .unwrap_or_default();
    header
        .strip_prefix("Bearer ")
        .is_some_and(|t| token_ok(&pin, t))
}

/// What a printed worksheet must show for parts that are answered by choosing.
fn choices(p: &quiz_core::model::Part) -> String {
    use quiz_core::model::Kind;
    match &p.kind {
        Kind::YesNo { yes, no } => format!("{yes} / {no}"),
        // Ranges (digits 0–9, numbers 1..n) are obvious on paper; lists are not.
        Kind::Set { options } if !options.windows(2).all(|w| w[1] == w[0] + 1) => {
            format!(
                "Αριθμοί: {}",
                options
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        _ => String::new(),
    }
}

#[derive(Deserialize)]
struct CheckReq {
    cat: String,
    seed: u32,
    values: Vec<Value>,
    #[serde(default)]
    reveal: bool,
}

#[derive(Deserialize)]
struct LoginReq {
    pin: String,
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get("/api/categories", |_, _| {
            let tags: serde_json::Map<String, Value> = TAGS.iter().map(|(k, v)| (k.to_string(), Value::from(*v))).collect();
            json_res(&json!({ "categories": CATEGORIES, "tags": tags }), 200)
        })
        .get("/api/question", |req, _| {
            let q = query(&req)?;
            let (Some(cat), Some(seed)) = (q.get("cat"), seed_of(&q)) else { return bad("cat και seed απαιτούνται", 400) };
            match generate(cat, seed) {
                Some(question) => json_res(&serde_json::to_value(public(&question, seed))?, 200),
                None => bad("άγνωστη κατηγορία", 404),
            }
        })
        .get("/api/hint", |req, _| {
            let q = query(&req)?;
            let (Some(cat), Some(seed)) = (q.get("cat"), seed_of(&q)) else { return bad("cat και seed απαιτούνται", 400) };
            let n: usize = q.get("n").and_then(|x| x.parse().ok()).unwrap_or(0).min(3);
            match generate(cat, seed) {
                Some(question) => json_res(&json!({ "n": n, "hint": question.hints[n] }), 200),
                None => bad("άγνωστη κατηγορία", 404),
            }
        })
        .post_async("/api/check", |mut req, _| async move {
            let Ok(body) = req.json::<CheckReq>().await else { return bad("μη έγκυρο αίτημα", 400) };
            match generate(&body.cat, body.seed) {
                Some(question) => json_res(&serde_json::to_value(check(&question, &body.values, body.reveal))?, 200),
                None => bad("άγνωστη κατηγορία", 404),
            }
        })
        .post_async("/api/teacher/login", |mut req, ctx| async move {
            let Some(pin) = teacher_pin(&ctx.env) else { return bad("Δεν έχει οριστεί PIN εκπαιδευτικού.", 503) };
            // At most a few attempts per minute per visitor (binding in wrangler.jsonc).
            if let Ok(limiter) = ctx.env.rate_limiter("LOGIN_LIMITER") {
                let ip = req.headers().get("cf-connecting-ip").ok().flatten().unwrap_or_else(|| "unknown".into());
                if let Ok(outcome) = limiter.limit(ip).await {
                    if !outcome.success {
                        return bad("Πολλές προσπάθειες. Δοκιμάστε ξανά σε ένα λεπτό.", 429);
                    }
                }
            }
            let Ok(body) = req.json::<LoginReq>().await else { return bad("μη έγκυρο αίτημα", 400) };
            // Slow down guessing a little on every attempt.
            Delay::from(std::time::Duration::from_millis(400)).await;
            if ct_eq(body.pin.trim().as_bytes(), pin.as_bytes()) {
                let (token, exp) = make_token(&pin);
                json_res(&json!({ "token": token, "expires": exp }), 200)
            } else {
                bad("Λάθος PIN.", 401)
            }
        })
        .get("/api/teacher/resources", |req, ctx| {
            if !authorised(&req, &ctx.env) {
                return bad("Χρειάζεται PIN.", 401);
            }
            json_res(&json!({ "exam": EXAM, "solutions": SOLUTIONS }), 200)
        })
        .get("/api/teacher/worksheet", |req, ctx| {
            if !authorised(&req, &ctx.env) {
                return bad("Χρειάζεται PIN.", 401);
            }
            let q = query(&req)?;
            let n: u32 = q.get("n").and_then(|x| x.parse().ok()).unwrap_or(8).clamp(1, 30);
            let base: u32 = seed_of(&q).unwrap_or(1);
            let cats: Vec<&str> = match q.get("cat").map(String::as_str) {
                Some("all") | None => CATEGORIES.iter().map(|c| c.id).collect(),
                Some(c) => vec![CATEGORIES.iter().find(|x| x.id == c).map(|x| x.id).unwrap_or("num")],
            };
            let items: Vec<Value> = (0..n)
                .map(|i| {
                    let cat = cats[i as usize % cats.len()];
                    let seed = base.wrapping_mul(2_654_435_761).wrapping_add(i * 7919);
                    let question = generate(cat, seed).expect("known category");
                    json!({
                        "cat": cat,
                        "prompt": question.prompt,
                        "groups": question.groups.iter().map(|g| json!({
                            "label": g.label,
                            "parts": g.parts.iter().map(|p| json!({ "label": p.label, "answer": answer_text(p), "choices": choices(p) })).collect::<Vec<_>>()
                        })).collect::<Vec<_>>(),
                        "solution": question.solution,
                    })
                })
                .collect();
            json_res(&json!({ "items": items }), 200)
        })
        .or_else_any_method("/api/*path", |_, _| bad("δεν βρέθηκε", 404))
        .run(req, env)
        .await
}
