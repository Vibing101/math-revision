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
//! | GET  /api/lesson?cat&seed&kind | narrated step lesson: kind=instr or kind=method   |
//! | GET  /api/tts?…&step&k&v       | one lesson sentence as speech (Azure, cached)     |
//! | POST /api/teacher/login        | PIN → signed session token                        |
//! | GET  /api/teacher/resources    | printable test and its solutions (token required) |
//! | GET  /api/teacher/worksheet    | worksheet with answer key (token required)        |
//!
//! The teacher PIN lives in the `TEACHER_PIN` secret, never in the page.
//! Natural voices need the `AZURE_SPEECH_KEY` and `AZURE_SPEECH_REGION` secrets;
//! without them the page uses the device's own voice.

use hmac::{Hmac, Mac};
use quiz_core::{
    answer_text, chapter_categories, check, generate, public, CATEGORIES, CHAPTERS, TAGS,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::Sha256;
use worker::*;

const TOKEN_HOURS: u64 = 12;
/// Printable tests for the teacher area: (id, grade, title, test, solutions).
const PAPERS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "a-diairetotita",
        "a",
        "Επαναληπτικό: Διαιρετότητα",
        include_str!("../teacher/exam.html"),
        include_str!("../teacher/solutions.html"),
    ),
    (
        "g-parag-1",
        "g",
        "Παραγοντοποίηση – Εξισώσεις: Διαγώνισμα 1",
        include_str!("../teacher/g_test1.html"),
        include_str!("../teacher/g_test1_sol.html"),
    ),
    (
        "g-parag-2",
        "g",
        "Παραγοντοποίηση – Εξισώσεις: Διαγώνισμα 2",
        include_str!("../teacher/g_test2.html"),
        include_str!("../teacher/g_test2_sol.html"),
    ),
    (
        "g-parag-3",
        "g",
        "Παραγοντοποίηση – Εξισώσεις: Διαγώνισμα 3",
        include_str!("../teacher/g_test3.html"),
        include_str!("../teacher/g_test3_sol.html"),
    ),
];

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
        Kind::Set { options, labels }
            if labels.is_empty() && !options.windows(2).all(|w| w[1] == w[0] + 1) =>
        {
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

/// Azure neural voices: `m` Νέστορας, `f` Αθηνά.
fn azure_voice(v: &str) -> &'static str {
    if v == "f" {
        "el-GR-AthinaNeural"
    } else {
        "el-GR-NestorasNeural"
    }
}

fn azure(env: &Env) -> Option<(String, String)> {
    let get = |k: &str| {
        env.secret(k)
            .ok()
            .map(|s| s.to_string())
            .or_else(|| env.var(k).ok().map(|s| s.to_string()))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };
    Some((get("AZURE_SPEECH_KEY")?, get("AZURE_SPEECH_REGION")?))
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// The sentence the player asks for, taken from the lesson itself: the endpoint never
/// speaks arbitrary text, so nobody can use the speech quota for anything else.
fn lesson_sentence(q: &std::collections::HashMap<String, String>) -> Option<String> {
    let question = generate(q.get("cat")?, seed_of(q)?)?;
    let lesson = match q.get("kind").map(String::as_str) {
        Some("method") => quiz_core::lesson::method(&question)?,
        _ => quiz_core::lesson::instructions(&question),
    };
    let step: usize = q.get("step")?.parse().ok()?;
    let k: usize = q.get("k")?.parse().ok()?;
    lesson.steps.get(step)?.parts.get(k).cloned()
}

async fn tts(req: Request, env: &Env) -> Result<Response> {
    let Some((key, region)) = azure(env) else {
        return bad("δεν υπάρχει φυσική φωνή", 503);
    };
    let q = query(&req)?;
    let Some(text) = lesson_sentence(&q) else {
        return bad("δεν βρέθηκε", 404);
    };
    let voice = azure_voice(q.get("v").map(String::as_str).unwrap_or("m"));
    // Same sentence + voice → same audio, whatever question it came from.
    let digest = hex(&<Sha256 as sha2::Digest>::digest(
        format!("{voice}|{text}").as_bytes(),
    ));
    let cache_key = format!("https://maths.savvy-des.com/tts-cache/{digest}.mp3");
    let cache = Cache::default();
    if let Ok(Some(hit)) = cache.get(&cache_key, false).await {
        return Ok(hit);
    }
    if let Ok(limiter) = env.rate_limiter("TTS_LIMITER") {
        let ip = req
            .headers()
            .get("cf-connecting-ip")
            .ok()
            .flatten()
            .unwrap_or_else(|| "unknown".into());
        if let Ok(outcome) = limiter.limit(ip).await {
            if !outcome.success {
                return bad("Πολλά αιτήματα φωνής. Δοκίμασε ξανά σε λίγο.", 429);
            }
        }
    }
    let ssml = format!(
        "<speak version='1.0' xml:lang='el-GR'><voice name='{voice}'><prosody rate='-6%'>{}</prosody></voice></speak>",
        xml_escape(&text)
    );
    let headers = Headers::new();
    headers.set("Ocp-Apim-Subscription-Key", &key)?;
    headers.set("Content-Type", "application/ssml+xml")?;
    headers.set(
        "X-Microsoft-OutputFormat",
        "audio-24khz-48kbitrate-mono-mp3",
    )?;
    headers.set("User-Agent", "maths-savvy-des")?;
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(ssml.into()));
    let url = format!("https://{region}.tts.speech.microsoft.com/cognitiveservices/v1");
    let mut res = match Fetch::Request(Request::new_with_init(&url, &init)?)
        .send()
        .await
    {
        Ok(r) if r.status_code() == 200 => r,
        Ok(r) => {
            console_error!("azure tts: status {}", r.status_code());
            return bad("η φυσική φωνή δεν είναι διαθέσιμη", 502);
        }
        Err(e) => {
            console_error!("azure tts: {e}");
            return bad("η φυσική φωνή δεν είναι διαθέσιμη", 502);
        }
    };
    let audio = res.bytes().await?;
    let mut out = Response::from_bytes(audio)?;
    out.headers_mut().set("content-type", "audio/mpeg")?;
    out.headers_mut()
        .set("cache-control", "public, max-age=31536000, immutable")?;
    let _ = cache.put(&cache_key, out.cloned()?).await;
    Ok(out)
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
        .get("/api/categories", |_, ctx| {
            let tags: serde_json::Map<String, Value> = TAGS.iter().map(|(k, v)| (k.to_string(), Value::from(*v))).collect();
            json_res(&json!({ "chapters": CHAPTERS, "categories": CATEGORIES, "tags": tags, "method_cats": quiz_core::lesson::METHOD_CATS, "tts": azure(&ctx.env).is_some() }), 200)
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
        .get("/api/lesson", |req, _| {
            let q = query(&req)?;
            let (Some(cat), Some(seed)) = (q.get("cat"), seed_of(&q)) else { return bad("cat και seed απαιτούνται", 400) };
            let Some(question) = generate(cat, seed) else { return bad("άγνωστη κατηγορία", 404) };
            let lesson = match q.get("kind").map(String::as_str) {
                Some("method") => quiz_core::lesson::method(&question),
                _ => Some(quiz_core::lesson::instructions(&question)),
            };
            match lesson {
                Some(l) => json_res(&serde_json::to_value(l)?, 200),
                None => bad("δεν υπάρχει βίντεο μεθόδου για αυτή την άσκηση", 404),
            }
        })
        .get_async("/api/tts", |req, ctx| async move { tts(req, &ctx.env).await })
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
            let papers: Vec<Value> = PAPERS
                .iter()
                .map(|(id, grade, title, exam, sol)| json!({ "id": id, "grade": grade, "title": title, "exam": exam, "solutions": sol }))
                .collect();
            json_res(&json!({ "papers": papers }), 200)
        })
        .get("/api/teacher/worksheet", |req, ctx| {
            if !authorised(&req, &ctx.env) {
                return bad("Χρειάζεται PIN.", 401);
            }
            let q = query(&req)?;
            let n: u32 = q.get("n").and_then(|x| x.parse().ok()).unwrap_or(8).clamp(1, 30);
            let base: u32 = seed_of(&q).unwrap_or(1);
            // `cat=<id>` for one category, `chapters=a,b` for whole chapters (year review).
            let cats: Vec<&str> = match (q.get("cat"), q.get("chapters")) {
                (Some(c), _) if c != "all" => match CATEGORIES.iter().find(|x| x.id == c) {
                    Some(x) => vec![x.id],
                    None => return bad("άγνωστη κατηγορία", 404),
                },
                (_, Some(list)) => list.split(',').flat_map(chapter_categories).collect(),
                _ => CATEGORIES.iter().map(|c| c.id).collect(),
            };
            if cats.is_empty() {
                return bad("δεν επιλέχθηκε κεφάλαιο", 400);
            }
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
