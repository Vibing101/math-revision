'use strict';
/* Thin client: questions, hints and checking come from the Rust API (/api/*).
   Points, streaks and progress stay in this browser (localStorage). */

const $ = (s) => document.querySelector(s);
const app = $('#app');
const esc = (s) => String(s).replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]));
const rand = (a, b) => a + Math.floor(Math.random() * (b - a + 1));
const pick = (a) => a[Math.floor(Math.random() * a.length)];
/* Line icons (lucide shapes), one size and stroke everywhere, colour from the text. */
const ICONS = {
  star: '<path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01z"/>',
  flame: '<path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.07-2.14-.22-4.05 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.15.43-2.29 1-3a2.5 2.5 0 0 0 2.5 2.5z"/>',
  bulb: '<path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"/><path d="M9 18h6"/><path d="M10 22h4"/>',
  award: '<circle cx="12" cy="8" r="6"/><path d="M15.48 12.89 17 22l-5-3-5 3 1.52-9.11"/>',
  play: '<path d="M6 3l14 9-14 9z"/>',
  back: '<path d="m12 19-7-7 7-7"/><path d="M19 12H5"/>',
  next: '<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>',
  cal: '<rect x="3" y="4" width="18" height="18" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/>',
  lock: '<rect x="3" y="11" width="18" height="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>',
};
const I = (n) => `<svg class="ic" viewBox="0 0 24 24" aria-hidden="true">${ICONS[n]}</svg>`;
const newSeed = () => { try { return crypto.getRandomValues(new Uint32Array(1))[0]; } catch (e) { return Math.floor(Math.random() * 4294967295); } };

async function api(path, opts = {}) {
  const res = await fetch(path, { ...opts, headers: { 'content-type': 'application/json', ...(opts.headers || {}) } });
  let body = null;
  try { body = await res.json(); } catch (e) { /* empty */ }
  if (!res.ok) { const err = new Error((body && body.error) || `Σφάλμα ${res.status}`); err.status = res.status; throw err; }
  return body;
}
const netError = (e) => `<div class="fb bad"><span class="h">Κάτι δεν πήγε καλά.</span><span>${esc(e.message || 'Ελέγξτε τη σύνδεση στο διαδίκτυο και δοκιμάστε ξανά.')}</span></div>`;

/* ---------- state ---------- */
const KEY = 'diairetotita-v1';
const fresh = () => ({ points: 0, streak: 0, best: 0, noHint: 0, cats: {}, tags: {}, badges: [], rounds: [] });
let S = fresh();
try { const s = JSON.parse(localStorage.getItem(KEY)); if (s && typeof s.points === 'number') S = Object.assign(fresh(), s); } catch (e) { /* private mode */ }
const save = () => { try { localStorage.setItem(KEY, JSON.stringify(S)); } catch (e) { /* ignore */ } };
const cs = (id) => S.cats[id] || (S.cats[id] = { att: 0, ok: 0, first: 0 });
let CATS = [], TAGS = {}, CHAPTERS = [], METHOD_CATS = [], NATURAL_TTS = false, NATURAL = [];
const chCats = (ch) => CATS.filter((c) => c.chapter === ch);
const GRADES = [{ id: 'a', name: 'Α΄ Γυμνασίου' }, { id: 'g', name: 'Γ΄ Γυμνασίου' }];
const gradeName = () => (GRADES.find((g) => g.id === S.grade) || GRADES[0]).name;
/** Chapters of the selected school year. */
const GCH = () => CHAPTERS.filter((c) => c.grade === (S.grade || 'a'));
const gradeCats = () => CATS.filter((c) => GCH().some((ch) => ch.id === c.chapter));
const chStats = (ch) => chCats(ch).reduce((a, c) => { const s = cs(c.id); return { att: a.att + s.att, ok: a.ok + s.ok }; }, { att: 0, ok: 0 });

const LEVELS = [[0, 'Ξεκίνημα'], [100, 'Σε ρυθμό'], [250, 'Σταθερά βήματα'], [450, 'Δυνατός λύτης'], [700, 'Έτοιμος για διαγώνισμα'], [1000, 'Μάστορας']];
function level() {
  let i = 0; while (i + 1 < LEVELS.length && S.points >= LEVELS[i + 1][0]) i++;
  const nx = LEVELS[i + 1];
  return { i, name: LEVELS[i][1], pct: nx ? (S.points - LEVELS[i][0]) / (nx[0] - LEVELS[i][0]) * 100 : 100, next: nx ? nx[0] - S.points : 0 };
}
const BADGES = [
  { id: 'first', t: 'Πρώτη σωστή', ok: () => Object.values(S.cats).some((c) => c.ok > 0) },
  { id: 's5', t: 'Σερί 5', ok: () => S.best >= 5 }, { id: 's10', t: 'Σερί 10', ok: () => S.best >= 10 },
  { id: 't25', t: '25 σωστές απαντήσεις', ok: () => totalOk() >= 25 },
  { id: 't100', t: '100 σωστές απαντήσεις', ok: () => totalOk() >= 100 },
  { id: 'round', t: 'Πρώτο επαναληπτικό', ok: () => S.rounds.length > 0 },
  { id: 'nh', t: '10 σωστά χωρίς βοήθεια', ok: () => S.noHint >= 10 },
  { id: 'all', t: 'Σωστό σε όλες τις κατηγορίες της τάξης', ok: () => gradeCats().length > 0 && gradeCats().every((c) => cs(c.id).ok > 0) },
  { id: 'r9', t: '90% σε επαναληπτικό', ok: () => S.rounds.some((r) => r >= 9) }];
function totalOk() { return Object.values(S.cats).reduce((a, c) => a + (c.ok || 0), 0); }
function newBadges() { const got = []; for (const b of BADGES) if (!S.badges.includes(b.id) && b.ok()) { S.badges.push(b.id); got.push(b.t); } return got; }

/* ---------- navigation: #  (chapters) · #ch-<id> · #year · #teacher ---------- */
let Q = null, round = null, qn = 0, hintsShown = 0, tries = 0, done = false, busy = false;
function go(hash) { round = null; if (location.hash === hash || (!location.hash && hash === '#')) route(); else location.hash = hash; }
function route() {
  round = null; stopSpeak();
  const h = location.hash;
  if (!S.grade || h === '#grade') return renderGrade();
  if (h === '#year') return renderYear();
  if (h === '#mistakes') return renderMistakes();
  if (h === '#teacher') return renderTeacher();
  if (h.startsWith('#th-') && CHAPTERS.some((c) => '#th-' + c.id === h)) return renderTheory(h.slice(4));
  if (h.startsWith('#ch-') && CHAPTERS.some((c) => '#ch-' + c.id === h)) return renderChapter(h.slice(4));
  renderHome();
}
window.addEventListener('hashchange', route);
const backLabel = () => (location.hash.startsWith('#ch-') || location.hash.startsWith('#th-') ? 'Κεφάλαιο' : location.hash === '#year' ? 'Επανάληψη χρονιάς' : location.hash === '#mistakes' ? 'Τα λάθη μου' : 'Αρχική');
const backBtn = (label, id = 'home') => `<button class="link" id="${id}">${I('back')}${label}</button>`;

function statsChips() { return `<div class="stats"><span class="stat">${I('star')}<b>${S.points}</b> πόντοι</span><span class="stat">${I('flame')}σερί <b>${S.streak}</b></span></div>`; }
/* progress only once the pupil has started; an empty bar on every card is noise */
const pctBar = (st) => (st.att ? `<span class="bar"><i style="width:${Math.round(st.ok / st.att * 100)}%"></i></span><span class="d">${st.ok} από ${st.att} σωστά</span>` : '');

function renderGrade() {
  app.innerHTML = `<header class="hero"><h1>Μαθηματικά Γυμνασίου</h1><p class="muted" style="margin:0">Σε ποια τάξη είσαι; Μπορείς να την αλλάξεις όποτε θέλεις από την αρχική οθόνη.</p></header>
  <div class="grid">${GRADES.map((g) => { const n = CHAPTERS.filter((c) => c.grade === g.id).length; return `<button class="cat chap grade" data-grade="${g.id}"><span class="t">${g.name}</span><span class="d">${n} κεφάλαια από το βιβλίο του Υπουργείου</span></button>`; }).join('')}</div>`;
  app.querySelectorAll('[data-grade]').forEach((b) => (b.onclick = () => { S.grade = b.dataset.grade; save(); go('#'); }));
}

/** The one thing to do next: the last practised section, or the first chapter. */
function nextUp() {
  const r = S.last && S.last.round;
  if (r && r.cats.length && r.cats.every((id) => gradeCats().some((c) => c.id === id))) {
    return { kicker: 'Συνέχισε από εκεί που σταμάτησες', title: r.title, sub: `Νέο επαναληπτικό με ${r.n} ερωτήσεις`, btn: 'Συνέχισε', go: () => startRound(r.cats, r.n, r.title) };
  }
  const last = typeof S.last === 'string' && gradeCats().find((c) => c.id === S.last);
  if (last) {
    const ch = CHAPTERS.find((c) => c.id === last.chapter), st = cs(last.id);
    return { kicker: 'Συνέχισε από εκεί που σταμάτησες', title: last.title, sub: `Κεφάλαιο ${ch.number}: ${ch.title}${st.att ? ` · ${st.ok} από ${st.att} σωστά` : ''}`, btn: 'Συνέχισε', go: () => { round = null; start(last.id); } };
  }
  const ch = GCH()[0];
  return { kicker: 'Ξεκίνα από εδώ', title: ch.title, sub: `Κεφάλαιο ${ch.number} · 10 ερωτήσεις από όλες τις ενότητες`, btn: 'Ξεκίνα', go: () => go('#ch-' + ch.id) };
}

function renderHome() {
  const L = level(), up = nextUp();
  const earned = BADGES.filter((b) => S.badges.includes(b.id));
  app.innerHTML = `<header class="hero"><h1>Μαθηματικά Γυμνασίου</h1><div class="seg" role="group" aria-label="Τάξη">${GRADES.map((g) => `<button class="tog gradechip" data-g="${g.id}" aria-pressed="${g.id === S.grade}">${g.name}</button>`).join('')}</div></header>
  <section class="card cont"><span class="muted">${up.kicker}</span><h2 class="cont-t"><span class="mark">${esc(up.title)}</span></h2><span class="muted">${esc(up.sub)}</span>
   <div class="row cont-row"><button class="btn big-btn" id="go-next">${up.btn}${I('next')}</button>${statsChips()}</div>
   <div class="lvl"><span class="muted">Επίπεδο ${L.i + 1}: <b class="fg">${L.name}</b>${L.next ? ` · ${L.next} πόντοι για το επόμενο` : ''}</span><div class="bar" aria-hidden="true"><i style="width:${L.pct}%"></i></div></div></section>
  ${goalCardHTML()}
  <h2>Κεφάλαια</h2>
  <div class="grid">${GCH().map((ch) => `<button class="cat chap" data-ch="${ch.id}"><span class="sec">Κεφάλαιο ${ch.number}</span><span class="t">${esc(ch.title)}</span><span class="d">${esc(ch.desc)}</span>${pctBar(chStats(ch.id))}</button>`).join('')}
   <button class="cat chap" id="go-year"><span class="sec">${I('cal')}Τέλος χρονιάς</span><span class="t">Επανάληψη χρονιάς</span><span class="d">Διάλεξε ποια κεφάλαια εξετάζονται και κάνε ένα μικτό επαναληπτικό.</span></button></div>
  <details class="card bdg"><summary>${I('award')}<span>Παράσημα: <b>${earned.length} από ${BADGES.length}</b></span></summary><ul class="blist">${BADGES.map((b) => `<li class="${S.badges.includes(b.id) ? 'got' : ''}">${S.badges.includes(b.id) ? I('award') : '<span class="ic"></span>'}${b.t}</li>`).join('')}</ul></details>
  <div class="row"><button class="link" id="go-teacher">${I('lock')}Για τον εκπαιδευτικό</button></div>`;
  $('#go-next').onclick = up.go;
  app.querySelectorAll('[data-ch]').forEach((b) => (b.onclick = () => go('#ch-' + b.dataset.ch)));
  app.querySelectorAll('[data-g]').forEach((b) => (b.onclick = () => { S.grade = b.dataset.g; save(); renderHome(); }));
  $('#go-year').onclick = () => go('#year');
  $('#goal-set').onclick = openGoal; $('#go-mk').onclick = () => go('#mistakes');
  $('#go-teacher').onclick = () => go('#teacher');
}

function renderChapter(id) {
  const ch = CHAPTERS.find((c) => c.id === id), cats = chCats(id);
  app.innerHTML = `<div class="top">${backBtn('Κεφάλαια')}${statsChips()}</div>
  <header class="hero"><span class="muted">Κεφάλαιο ${ch.number}</span><h1>${esc(ch.title)}</h1><ul class="secs">${ch.sections.map((x) => `<li>${esc(x)}</li>`).join('')}</ul></header>
  <section class="card cont"><h2>Θεωρία και παραδείγματα</h2><p style="margin:0">Οι βασικοί ορισμοί και κανόνες του κεφαλαίου, σε μικρές κάρτες με λυμένα παραδείγματα.</p>
   <div class="row"><button class="btn ghost" id="go-theory">${I('play')}Διάβασε τη θεωρία</button></div></section>
  <section class="card cont"><h2>Επαναληπτικό κεφαλαίου</h2><p style="margin:0">10 ερωτήσεις από όλες τις ενότητες, με περισσότερες από εκεί που κάνεις λάθη.</p>
   <div class="row"><button class="btn big-btn" id="go-round">Ξεκίνα το επαναληπτικό${I('next')}</button></div></section>
  <h2>Ή εξάσκηση ανά ενότητα</h2>
  <div class="grid">${cats.map((c) => `<button class="cat${c.focus ? ' focus' : ''}" data-cat="${c.id}">${c.focus ? '<span class="tag">Θέλει προσοχή</span>' : ''}<span class="sec">${esc(c.section)}</span><span class="t">${esc(c.title)}</span><span class="d">${esc(c.desc)}</span>${pctBar(cs(c.id))}</button>`).join('')}</div>`;
  $('#home').onclick = () => go('#');
  app.querySelectorAll('[data-cat]').forEach((b) => (b.onclick = () => { round = null; start(b.dataset.cat); }));
  $('#go-round').onclick = () => startRound(cats.map((c) => c.id), 10, ch.title);
  $('#go-theory').onclick = () => go('#th-' + id);
}

/* ---------- theory: public/theory/<chapter>.json, short cards with worked examples ---------- */
const THEORY = {};
const CARD_LABEL = { goal: 'Τι θα μάθουμε', def: 'Ορισμός', rule: 'Κανόνας', note: 'Να θυμάσαι', warn: 'Πρόσεξε' };
const TPKEY = 'theory-prog-v1';
let TP = {};
try { TP = JSON.parse(localStorage.getItem(TPKEY)) || {}; } catch (e) { /* ignore */ }
const saveTP = () => { try { localStorage.setItem(TPKEY, JSON.stringify(TP)); } catch (e) { /* ignore */ } };
const tprog = (id) => TP[id] || (TP[id] = { pos: 0, sec: {} });
const RATE = [
  ['2', 'Το κατάλαβα', 'Μπράβο! Δοκίμασε ασκήσεις για να το εδραιώσεις.'],
  ['1', 'Θέλω να το ξαναδώ', 'Εντάξει. Ξαναδιάβασε τα παραδείγματα, βήμα βήμα, και μετά δοκίμασε ασκήσεις. Μπορείς να ζητήσεις υπόδειξη όποτε χρειάζεται.'],
  ['0', 'Δεν το κατάλαβα ακόμα', 'Δεν πειράζει. Έτσι ξέρουμε τι να επαναλάβουμε. Ξεκίνα από τον ορισμό και το πρώτο παράδειγμα και πάρε ένα βήμα τη φορά. Αν χρειάζεται, ρώτησε τον/την καθηγητή/τρια.'],
];
/* Backward fading (Barton 6.6): after the full example come similar problems where the pupil does the last
   step(s) and then checks. Data: example.fade = [{ html, steps }, ...] with new numbers; the first rung hides the
   last step, the next ones hide all steps but the first. Progress is kept in tprog(id).fade[key]. */
const fadeHide = (n, r) => (r === 0 ? 1 : Math.max(1, n - 1));
function fadeHTML(c, key, done) {
  const rungs = c.fade || [];
  if (!rungs.length) return '';
  const body = rungs.map((u, r) => {
    const n = u.steps.length, hide = Math.min(fadeHide(n, r), n - 1), solved = r < done;
    const lis = u.steps.map((x, i) => (i >= n - hide && !solved ? `<li class="hid" hidden>${x}</li>` : `<li>${x}</li>`)).join('');
    return `<article class="tcard fx${solved ? ' solved' : ''}" data-rung="${r}"${r > done ? ' hidden' : ''}>
      <span class="tlabel">Δοκίμασε εσύ ${r + 1} από ${rungs.length}${r === 0 ? ': το τελευταίο βήμα είναι δικό σου' : ': κάνε μόνος/η τα βήματα μετά το πρώτο'}</span>
      <div class="tbody">${u.html}</div><ol class="tsteps">${lis}</ol>
      ${canListen() ? `<button class="btn ghost sm listen" type="button" aria-pressed="false" data-ak="${key}:r${r}">${I('play')}<span>Άκουσε</span></button>` : ''}
      ${solved ? '<p class="fmsg">✓ Το έκανες σωστά.</p>' : `<p class="fslot">Η σειρά σου: κάνε ${hide > 1 ? `τα ${hide} τελευταία βήματα` : 'το τελευταίο βήμα'} στο τετράδιό σου. Μετά πάτα «Δες τη λύση».</p>
      <div class="row"><button class="btn ghost sm" type="button" data-f="show">Δες τη λύση</button></div>
      <div class="row" hidden data-f-check><button class="btn sm" type="button" data-f="ok">Το έκανα σωστά</button><button class="btn ghost sm" type="button" data-f="no">Δεν μου βγήκε</button></div>
      <p class="fmsg" aria-live="polite"></p>`}</article>`;
  }).join('');
  return `<div class="fade" data-fk="${key}" data-total="${rungs.length}"${done === 0 && (c.steps || []).length > 1 ? ' hidden' : ''}><p class="flabel">Τώρα δοκίμασε εσύ${done >= rungs.length ? ' · ✓ ολοκληρώθηκε' : ''}</p>${body}</div>`;
}
function theoryCard(c, key, fdone) {
  const listen = canListen() ? `<button class="btn ghost sm listen" type="button" aria-pressed="false" data-ak="${key}">${I('play')}<span>Άκουσε</span></button>` : '';
  if (c.type === 'example') {
    const steps = (c.steps || []).map((x, i) => `<li${i ? ' hidden' : ''}>${x}</li>`).join('');
    return `<article class="tcard ex"><span class="tlabel">Παράδειγμα</span><div class="tbody">${c.html}</div>
      ${steps ? `<ol class="tsteps">${steps}</ol><button class="btn ghost sm tnext" type="button">Δείξε το επόμενο βήμα</button>` : ''}${listen}</article>${fadeHTML(c, key, fdone)}`;
  }
  return `<article class="tcard ${c.type}"><span class="tlabel">${CARD_LABEL[c.type] || ''}${c.title ? ': ' + esc(c.title) : ''}</span><div class="tbody">${c.html}</div>${listen}</article>`;
}
/* «Σκέψου πρώτα» (the Ministry model's investigation phase): a situation and a few questions to think about;
   each answer is shown on request, and the link to the mathematics only after the last one. Data: section.explore. */
function exploreHTML(ex, s) {
  const listen = canListen() ? `<button class="btn ghost sm listen" type="button" aria-pressed="false" data-ak="${s}:x">${I('play')}<span>Άκουσε</span></button>` : '';
  const qs = (ex.qs || []).map((u, i) => `<li><div class="eq" data-k="${s}:x:q${i}">${u.q}</div><button class="btn ghost sm" type="button" data-ea="${i}">Δες μια απάντηση</button><div class="ea" hidden data-k="${s}:x:a${i}">${u.a}</div></li>`).join('');
  return `<article class="tcard explore"><span class="tlabel">Σκέψου πρώτα · Διερεύνηση</span><div class="tbody" data-k="${s}:x">${ex.html}</div>
    <ol class="eqs">${qs}</ol>${ex.bridge ? `<div class="ebridge" hidden data-k="${s}:x:b">${ex.bridge}</div>` : ''}${listen}</article>`;
}
function theoryDone(id) { const th = THEORY[id], p = tprog(id); return th ? th.sections.filter((_, i) => p.sec[i] === '2').length : 0; }
async function renderTheory(id) {
  const ch = CHAPTERS.find((c) => c.id === id);
  stopSpeak();
  app.innerHTML = `<div class="top">${backBtn('Κεφάλαιο')}${statsChips()}</div><header class="hero"><span class="muted">Κεφάλαιο ${ch.number} · Θεωρία</span><h1>${esc(ch.title)}</h1></header><p class="loading">Φόρτωση…</p>`;
  $('#home').onclick = () => go('#ch-' + id);
  let th = THEORY[id];
  if (!th) {
    try { const r = await fetch(`/theory/${id}.json`); if (!r.ok) throw new Error(); th = THEORY[id] = await r.json(); }
    catch (e) { const ph = app.querySelector('.loading'); if (ph) ph.outerHTML = '<div class="card"><p style="margin:0">Η θεωρία αυτού του κεφαλαίου θα προστεθεί σύντομα.</p></div>'; return; }
  }
  const slot = app.querySelector('.loading');
  if (!slot || location.hash !== '#th-' + id) return; // another navigation got here first
  const P = tprog(id), one = !!UIP.one;
  /* units: a card (with its section heading when it opens a section), and one self-check per section */
  let n = 0;
  const secHTML = th.sections.map((s, i) => {
    const cards = s.cards.map((c, k) => { const fk = `${i}:${k}`; return `<div class="tu" data-u="${n++}">${k === 0 ? `<h2>${esc(s.title)}</h2>` : ''}${theoryCard(c, fk, (P.fade || {})[fk] || 0)}</div>${k === 0 && s.explore ? `<div class="tu" data-u="${n++}">${exploreHTML(s.explore, i)}</div>` : ''}`; }).join('');
    const rated = P.sec[i];
    const check = `<div class="tu" data-u="${n++}"><div class="tcard check" data-sec="${i}"><span class="tlabel">Έλεγχος</span>
      <p><b>Πώς σου φάνηκε η ενότητα «${esc(s.title)}»;</b></p>
      <div class="rate" role="group" aria-label="Πώς σου φάνηκε η ενότητα">${RATE.map(([v, t]) => `<button class="btn ghost sm" type="button" data-r="${v}" aria-pressed="${rated === v}">${t}</button>`).join('')}</div>
      <p class="rmsg muted" aria-live="polite">${rated ? RATE.find((r) => r[0] === rated)[2] : ''}</p>
      ${s.practice ? `<div class="row"><button class="btn" type="button" data-cat="${s.practice}">${I('next')}Δοκίμασε ασκήσεις</button></div>` : ''}</div></div>`;
    return `<section class="tsec" id="ts-${i}">${cards}${check}</section>`;
  }).join('');
  const total = n;
  slot.outerHTML = `<div class="tbar"><div class="row"><span class="cnt" id="t-cnt" aria-live="polite"></span>${one ? '' : `<span class="cnt">Ενότητες που κατάλαβες: ${theoryDone(id)} από ${th.sections.length}</span>`}</div><div class="bar"><i id="t-bar"></i></div></div>
    <nav class="toc card" aria-label="Ενότητες">${th.sections.map((s, i) => `<a href="#th-${id}" data-s="${i}"${P.sec[i] === '2' ? ' class="done"' : ''}>${esc(s.title)}</a>`).join('')}</nav>
    <div id="t-body" data-ch="${id}" class="${one ? 'one' : ''}">${secHTML}</div>
    ${one ? `<div class="row tnav"><button class="btn ghost" id="t-prev" type="button">${I('back')}Πίσω</button><button class="btn big-btn" id="t-next" type="button">Επόμενο${I('next')}</button></div>` : ''}`;
  const units = [...app.querySelectorAll('.tu')];
  const show = (pos, scroll) => {
    pos = Math.max(0, Math.min(total - 1, pos)); P.pos = pos; saveTP();
    if (one) units.forEach((u, i) => { u.hidden = i > pos; u.classList.toggle('cur', i === pos); });
    $('#t-cnt').textContent = `Βήμα ${pos + 1} από ${total}`;
    $('#t-bar').style.width = `${Math.round((pos + 1) / total * 100)}%`;
    if (one) { const nx = $('#t-next'), pv = $('#t-prev'); nx.disabled = pos >= total - 1; pv.disabled = pos <= 0; if (scroll) setTimeout(() => units[pos].scrollIntoView({ behavior: UIP.calm ? 'auto' : 'smooth', block: 'center' }), 60); }
  };
  if (one) { $('#t-next').onclick = () => { stopSpeak(); show(P.pos + 1, true); }; $('#t-prev').onclick = () => { stopSpeak(); show(P.pos - 1, true); }; show(Math.min(P.pos, total - 1), P.pos > 0); }
  else { show(0, false); $('#t-bar').style.width = `${Math.round(theoryDone(id) / th.sections.length * 100)}%`; $('#t-cnt').textContent = 'Διάβασε με τη σειρά, όσο γρήγορα θέλεις.'; }
  app.querySelectorAll('.toc a').forEach((a) => (a.onclick = (e) => { e.preventDefault(); const i = +a.dataset.s; if (one) { const u = units.find((x) => x.querySelector(`#ts-${i}`) || x.closest(`#ts-${i}`)); show(units.indexOf(u), true); } else $('#ts-' + i).scrollIntoView({ behavior: UIP.calm ? 'auto' : 'smooth' }); }));
  app.querySelectorAll('.tnext').forEach((b) => (b.onclick = () => {
    const li = b.previousElementSibling.querySelector('li[hidden]');
    if (li) li.hidden = false;
    if (!b.previousElementSibling.querySelector('li[hidden]')) { const fd = b.closest('.tu').querySelector('.fade'); b.remove(); if (fd) fd.hidden = false; }
  }));
  app.querySelectorAll('.fade').forEach((box) => box.addEventListener('click', (e) => {
    const b = e.target.closest('[data-f]'); if (!b) return;
    const rung = b.closest('.fx'), r = +rung.dataset.rung, total = +box.dataset.total, msg = rung.querySelector('.fmsg');
    const next = box.querySelector(`.fx[data-rung="${r + 1}"]`);
    const save = (n) => { P.fade = P.fade || {}; P.fade[box.dataset.fk] = Math.max(P.fade[box.dataset.fk] || 0, n); saveTP(); };
    if (b.dataset.f === 'show') {
      rung.querySelectorAll('li.hid').forEach((li) => (li.hidden = false)); rung.querySelector('.fslot').hidden = true; b.parentElement.hidden = true;
      rung.querySelector('[data-f-check]').hidden = false; msg.textContent = 'Σύγκρινε με ό,τι έκανες. Βγήκε το ίδιο;';
    } else if (b.dataset.f === 'ok') {
      rung.querySelector('[data-f-check]').hidden = true; rung.classList.add('solved'); save(r + 1);
      msg.textContent = next ? '✓ Μπράβο! Πάμε σε ένα παρόμοιο, με λιγότερη βοήθεια.' : '✓ Μπράβο! Το έκανες μόνος/η, από την αρχή μέχρι το τέλος.';
      if (next) { next.hidden = false; setTimeout(() => next.scrollIntoView({ behavior: UIP.calm ? 'auto' : 'smooth', block: 'nearest' }), 60); }
      else box.querySelector('.flabel').textContent = 'Τώρα δοκίμασε εσύ · ✓ ολοκληρώθηκε';
    } else if (b.dataset.f === 'no') {
      rung.querySelector('[data-f-check]').hidden = true;
      msg.textContent = 'Δεν πειράζει, έτσι μαθαίνουμε. Ξαναδιάβασε το παράδειγμα και τη λύση, βήμα βήμα.' + (next ? ' Μετά δοκίμασε το επόμενο.' : ' Μετά δοκίμασε ασκήσεις.');
      if (next) next.hidden = false;
    }
  }));
  app.querySelectorAll('.explore').forEach((box) => box.addEventListener('click', (e) => {
    const b = e.target.closest('[data-ea]'); if (!b) return;
    b.parentElement.querySelector('.ea').hidden = false; b.hidden = true;
    if (![...box.querySelectorAll('.ea')].some((a) => a.hidden)) { const br = box.querySelector('.ebridge'); if (br) br.hidden = false; }
  }));
  app.querySelectorAll('.listen').forEach((b) => (b.onclick = () => speakCard(b, b.closest('.tcard'))));
  app.querySelectorAll('.tcard.check').forEach((box) => box.querySelectorAll('[data-r]').forEach((b) => (b.onclick = () => {
    const i = +box.dataset.sec; if (P.sec[i] === undefined) actLog('t'); P.sec[i] = b.dataset.r; saveTP();
    box.querySelectorAll('[data-r]').forEach((x) => x.setAttribute('aria-pressed', String(x === b)));
    box.querySelector('.rmsg').textContent = RATE.find((r) => r[0] === b.dataset.r)[2];
    app.querySelector(`.toc a[data-s="${i}"]`).classList.toggle('done', b.dataset.r === '2');
    if (!one) { const c = app.querySelector('.tbar .cnt:last-child'); if (c) c.textContent = `Ενότητες που κατάλαβες: ${theoryDone(id)} από ${th.sections.length}`; $('#t-bar').style.width = `${Math.round(theoryDone(id) / th.sections.length * 100)}%`; }
  })));
  app.querySelectorAll('[data-cat]').forEach((b) => (b.onclick = () => { round = null; start(b.dataset.cat); }));
}
function renderYear(msg) {
  const saved = (S.years || {})[S.grade] || (S.grade === 'a' && S.year) || null;
  const sel = (saved && saved.chapters) || GCH().map((c) => c.id);
  const n = (saved && saved.n) || 20;
  app.innerHTML = `<div class="top">${backBtn('Κεφάλαια')}${statsChips()}</div>
  <header class="hero"><h1>Επανάληψη χρονιάς</h1><span class="muted">${gradeName()}</span><p class="muted" style="margin:0">Διάλεξε τα κεφάλαια που εξετάζονται στις εξετάσεις τέλους χρονιάς. Η επιλογή σου αποθηκεύεται.</p></header>
  <form class="card" id="yf" style="display:flex;flex-direction:column;gap:12px">
   <fieldset style="border:0;padding:0;margin:0;display:flex;flex-direction:column;gap:8px"><legend style="font-weight:700;margin-bottom:8px">Κεφάλαια</legend>
   ${GCH().map((ch) => { const st = chStats(ch.id); return `<label class="chk"><input type="checkbox" name="ch" value="${ch.id}" id="y-${ch.id}"${sel.includes(ch.id) ? ' checked' : ''}><span><b>Κεφάλαιο ${ch.number}: ${esc(ch.title)}</b><br><span class="muted" style="font-size:.88rem">${esc(ch.desc)}${st.att ? ` · ${Math.round(st.ok / st.att * 100)}% σωστά μέχρι τώρα` : ''}</span></span></label>`; }).join('')}
   </fieldset>
   <div class="row"><label for="y-n" style="font-weight:700">Ερωτήσεις</label><select id="y-n">${[10, 20, 30, 40].map((x) => `<option value="${x}"${x === n ? ' selected' : ''}>${x}</option>`).join('')}</select></div>
   <span class="err" role="alert">${msg ? esc(msg) : ''}</span>
   <div class="row"><button class="btn big-btn" type="submit">Ξεκίνα την επανάληψη${I('next')}</button></div>
   <span class="muted" style="font-size:.85rem">Οι ερωτήσεις μοιράζονται σε όλα τα κεφάλαια που διάλεξες, με περισσότερες από τις ενότητες όπου κάνεις λάθη. Στο τέλος βλέπεις αποτέλεσμα ανά κεφάλαιο.</span>
  </form>`;
  $('#home').onclick = () => go('#');
  app.querySelectorAll('input[name=ch]').forEach((i) => i.addEventListener('change', () => { app.querySelector('#yf .err').textContent = ''; }));
  $('#yf').addEventListener('submit', (e) => {
    e.preventDefault();
    const chapters = [...app.querySelectorAll('input[name=ch]:checked')].map((i) => i.value);
    const count = +$('#y-n').value;
    if (!chapters.length) return renderYear('Διάλεξε τουλάχιστον ένα κεφάλαιο.');
    S.years = Object.assign(S.years || {}, { [S.grade]: { chapters, n: count } }); save();
    startRound(CATS.filter((c) => chapters.includes(c.chapter)).map((c) => c.id), count, 'Επανάληψη χρονιάς');
  });
}

function startRound(cats, n, title) { round = { n, i: 0, ok: 0, cats, title, by: {} }; S.last = { round: { cats, n, title } }; save(); start(null); }
function weightedCat(ids) {
  const prior = { pow: 0.3, time: 0.3, num: 0.4 };
  const list = CATS.filter((c) => ids.includes(c.id));
  // Spread a multi-chapter review evenly across chapters first, then weight by weak spots.
  const chs = [...new Set(list.map((c) => c.chapter))];
  const ch = chs[Math.floor(Math.random() * chs.length)];
  const pool = list.filter((c) => c.chapter === ch);
  const w = pool.map((c) => { const s = cs(c.id); const acc = s.att ? s.ok / s.att : (prior[c.id] ?? 0.5); return 1 + 2 * (1 - acc); });
  let x = Math.random() * w.reduce((a, b) => a + b, 0);
  for (let i = 0; i < w.length; i++) { x -= w[i]; if (x <= 0) return pool[i].id; }
  return pool[pool.length - 1].id;
}

/* ---------- question ---------- */
async function start(cat, seed) {
  const mk = round && round.mk ? round.mk[round.i] : null;
  const c = mk ? mk.cat : cat || weightedCat(round.cats);
  const useSeed = mk ? mk.seed : seed != null ? seed : newSeed();
  app.innerHTML = `<div class="top">${backBtn(backLabel())}${statsChips()}</div><section class="card"><p class="loading">Φόρτωση ερώτησης…</p></section>`;
  if (!round) { S.last = c; save(); }
  $('#home').onclick = goHome;
  try {
    Q = await api(`/api/question?cat=${encodeURIComponent(c)}&seed=${useSeed}`);
  } catch (e) {
    app.querySelector('.card').innerHTML = netError(e) + '<div class="row" style="margin-top:12px"><button class="btn" id="retry">Δοκίμασε ξανά</button></div>';
    $('#retry').onclick = () => start(cat, seed);
    return;
  }
  Q.fixed = cat || c; qn++; hintsShown = 0; tries = 0; done = false; busy = false;
  renderQ();
}
function goHome() { round = null; route(); }
function partHTML(p, id) {
  // labels come from our own server and may contain <sub>/<sup>
  const lab = p.label ? `<label for="${id}">${p.label}</label>` : '';
  switch (p.type) {
    case 'num': return `${lab}<input class="num" id="${id}" inputmode="numeric" autocomplete="off">`;
    case 'int': return `${lab}<span class="pmrow"><button class="tog pm" type="button" data-pm="${id}" aria-label="Αλλαγή προσήμου">±</button><input class="num" id="${id}" inputmode="numeric" autocomplete="off"></span>`;
    case 'frac': return `${lab}<span class="pmrow"><button class="tog pm" type="button" data-pm="${id}-n" aria-label="Αλλαγή προσήμου">±</button><span class="fracin"><input class="num" id="${id}-n" inputmode="numeric" autocomplete="off" aria-label="αριθμητής"><input class="num" id="${id}-d" inputmode="numeric" autocomplete="off" aria-label="παρονομαστής"></span></span><span class="muted" style="font-size:.82rem">Για ακέραιο, άφησε κενό τον παρονομαστή.</span>`;
    case 'yesno': return `${lab}<button class="tog" data-yn="1" id="${id}-y" aria-pressed="false">${esc(p.yes)}</button><button class="tog" data-yn="0" id="${id}-n" aria-pressed="false">${esc(p.no)}</button>`;
    case 'slots': return `${lab}<span class="slots">${p.pieces.map((pc, i) => `${pc}${i < p.pieces.length - 1 ? `<span class="pmrow"><button class="tog pm sm" type="button" data-pm="${id}-${i}" aria-label="Αλλαγή προσήμου">±</button><input class="num slot" id="${id}-${i}" inputmode="numeric" autocomplete="off" aria-label="κενό ${i + 1}"></span>` : ''}`).join('')}</span>`;
    case 'surd': return `${lab}<span class="pmrow"><button class="tog pm" type="button" data-pm="${id}-a" aria-label="Αλλαγή προσήμου">±</button><input class="num slot" id="${id}-a" inputmode="numeric" autocomplete="off" aria-label="αριθμός μπροστά από τη ρίζα"><span class="root">√<input class="num slot" id="${id}-b" inputmode="numeric" autocomplete="off" aria-label="αριθμός μέσα στη ρίζα"></span></span><span class="muted" style="font-size:.82rem">Για ακέραιο, άφησε κενή τη ρίζα.</span>`;
    case 'dec': return `${lab}<span class="pmrow"><button class="tog pm" type="button" data-pm="${id}" aria-label="Αλλαγή προσήμου">±</button><input class="num" id="${id}" inputmode="decimal" autocomplete="off"></span>`;
    case 'set': return `${lab}<div class="setgrid">${p.options.map((d, i) => `<button class="tog" data-st="${d}" id="${id}-${d}" aria-pressed="false">${esc(p.labels && p.labels.length ? p.labels[i] : d)}</button>`).join('')}</div>`;
    case 'select': return `${lab}<select id="${id}"><option value="">— διάλεξε —</option>${p.options.map((o) => `<option value="${o.v}">${esc(o.l)}</option>`).join('')}</select>`;
    case 'exp': return `<div class="exprow"><span>${esc(p.prefix)}&nbsp;</span>${p.primes.map((q, i) => `${i ? '<span class="dot">·</span>' : ''}<span>${q}<input class="ex" id="${id}-${q}" inputmode="numeric" aria-label="εκθέτης του ${q}" autocomplete="off"></span>`).join('')}</div>`;
  }
  return '';
}
const parts = () => Q.groups.flatMap((g) => g.parts);
function renderQ() {
  const cat = CATS.find((c) => c.id === Q.cat);
  let pi = 0;
  const chap = cat && CHAPTERS.find((c) => c.id === cat.chapter);
  app.innerHTML = `<div class="top">${backBtn(backLabel())}${statsChips()}</div>
  <section class="card q">${round ? `<div class="rprog"><span class="muted">${esc(round.title)}</span><span class="muted">Ερώτηση <b class="fg">${round.i + 1}</b> από ${round.n}</span><div class="bar" aria-hidden="true"><i style="width:${Math.round((round.i / round.n) * 100)}%"></i></div></div>` : ''}
  <span class="sec">${round && chap ? esc(chap.title) + ' · ' : ''}${esc(cat ? cat.title : '')}</span>
  <div class="qtext">${Q.prompt}</div>
  <div class="row lvbtns"><span class="muted">Βίντεο</span><button class="btn ghost sm" data-lesson="instr">${I('play')}Οδηγίες</button>${METHOD_CATS.includes(Q.cat) ? `<button class="btn ghost sm" data-lesson="method">${I('play')}Μέθοδος</button>` : ''}</div>
  ${Q.groups.map((g) => `<div class="group">${g.label ? `<div class="gl">${g.label}</div>` : ''}${g.parts.map((p) => { const id = `p${qn}_${pi++}`; p.id = id; return `<div class="part" id="${id}-w">${partHTML(p, id)}</div>`; }).join('')}</div>`).join('')}
  <div class="row actions"><button class="btn big-btn" id="check">Έλεγχος</button><button class="btn ghost" id="hint">${I('bulb')}Βοήθεια <span class="cnt">0/4</span></button><button class="btn big-btn" id="next" hidden>${round && round.i + 1 >= round.n ? 'Τέλος γύρου' : 'Επόμενη'}${I('next')}</button></div>
  <div class="hints" id="hints"></div><div id="fb"></div></section>`;
  colourMath();
  $('#home').onclick = goHome;
  app.querySelectorAll('[data-yn]').forEach((b) => (b.onclick = () => b.parentElement.querySelectorAll('[data-yn]').forEach((x) => x.setAttribute('aria-pressed', x === b))));
  app.querySelectorAll('[data-st]').forEach((b) => (b.onclick = () => b.setAttribute('aria-pressed', b.getAttribute('aria-pressed') !== 'true')));
  app.querySelectorAll('[data-pm]').forEach((b) => (b.onclick = () => { const i = $('#' + b.dataset.pm); const v = i.value.trim(); i.value = /^[-−]/.test(v) ? v.replace(/^[-−]\s*/, '') : '−' + v; i.focus(); }));
  $('#hint').onclick = showHint; $('#check').onclick = check;
  app.querySelectorAll('[data-lesson]').forEach((b) => (b.onclick = () => openLesson(b.dataset.lesson, b)));
  $('#next').onclick = () => { if (round) { round.i++; if (round.i >= round.n) return endRound(); } start(round ? null : Q.fixed); };
  app.querySelectorAll('input').forEach((i) => i.addEventListener('keydown', (e) => { if (e.key === 'Enter') { e.preventDefault(); done ? $('#next').click() : check(); } }));
  const f = app.querySelector('input,select'); if (f && matchMedia('(pointer:fine)').matches) f.focus();
}
/* Colour the main expression like the method videos: x² blue, x terms green, numbers orange. */
function colourMath() {
  // A single letter (or <i>letter</i>) is a variable; longer words are left alone.
  const re = /(\d*)(<i>[^<]{1,3}<\/i>|[A-Za-zΑ-Ωα-ωά-ώ]+)([²³⁴⁶]?)|(<[^>]+>)|(\d+(?:,\d+)?)/g;
  app.querySelectorAll('.qtext .big').forEach((el) => {
    if (el.querySelector('svg, table, input')) return;
    el.innerHTML = el.innerHTML.replace(re, (m, k, v, e, tag, n) => {
      if (tag) return m;
      if (n) return `<span class="m0">${n}</span>`;
      if (!v.startsWith('<i>') && (v.length > 1 || v !== v.toLowerCase())) return m;
      return `<span class="${e ? 'm2' : 'm1'}">${k}${v}${e}</span>`;
    });
  });
}
async function showHint() {
  if (hintsShown >= 4 || done || busy) return;
  const b = $('#hint'); b.disabled = true;
  try {
    const r = await api(`/api/hint?cat=${Q.cat}&seed=${Q.seed}&n=${hintsShown}`);
    $('#hints').insertAdjacentHTML('beforeend', `<div class="hint"><b>Βοήθεια ${hintsShown + 1}:</b> ${r.hint}</div>`);
    hintsShown++;
  } catch (e) { $('#fb').innerHTML = netError(e); }
  b.querySelector('.cnt').textContent = `${hintsShown}/4`; b.disabled = hintsShown >= 4;
}
function readVal(p) {
  const id = p.id;
  switch (p.type) {
    case 'num': case 'int': case 'dec': { const s = $('#' + id).value.trim(); return s === '' ? null : s; }
    case 'slots': return p.pieces.slice(0, -1).map((_, i) => { const s = $(`#${id}-${i}`).value.trim(); return s === '' ? null : s; });
    case 'surd': { const a = $(`#${id}-a`).value.trim(), b = $(`#${id}-b`).value.trim(); return a === '' && b === '' ? null : { a: a === '' ? null : a, b: b === '' ? null : b }; }
    case 'frac': { const n = $(`#${id}-n`).value.trim(), d = $(`#${id}-d`).value.trim(); return n === '' && d === '' ? null : { n: n === '' ? null : n, d: d === '' ? null : d }; }
    case 'yesno': { const y = $(`#${id}-y`).getAttribute('aria-pressed') === 'true', n = $(`#${id}-n`).getAttribute('aria-pressed') === 'true'; return y ? true : n ? false : null; }
    case 'set': return p.options.filter((d) => $(`#${id}-${d}`).getAttribute('aria-pressed') === 'true');
    case 'select': { const v = $('#' + id).value; return v === '' ? null : v; }
    case 'exp': { const m = {}; for (const q of p.primes) { const s = $(`#${id}-${q}`).value.trim(); if (s !== '') m[q] = s; } return m; }
  }
  return null;
}
async function check() {
  if (done || busy) return;
  busy = true; $('#check').disabled = true;
  const ps = parts(), fb = $('#fb');
  let r;
  try {
    r = await api('/api/check', { method: 'POST', body: JSON.stringify({ cat: Q.cat, seed: Q.seed, values: ps.map(readVal), reveal: tries >= 1 }) });
  } catch (e) { fb.innerHTML = netError(e); busy = false; $('#check').disabled = false; return; }
  busy = false; $('#check').disabled = false;
  if (r.blank.some(Boolean)) {
    S.tags.blank = (S.tags.blank || 0) + 1; save();
    ps.forEach((p, i) => { const w = $('#' + p.id + '-w'); w.classList.remove('ok', 'bad'); w.classList.toggle('miss', r.blank[i]); });
    fb.innerHTML = '<div class="fb info"><span>Συμπλήρωσε πρώτα τα κουτάκια με το κίτρινο περίγραμμα. Στο διαγώνισμα, ένα κενό χάνει μονάδες.</span></div>';
    return;
  }
  ps.forEach((p, i) => { const w = $('#' + p.id + '-w'); w.classList.remove('miss'); w.classList.toggle('ok', r.ok[i]); w.classList.toggle('bad', !r.ok[i]); });
  tries++;
  const c = cs(Q.cat);
  if (r.correct) {
    done = true; c.att++; c.ok++; if (tries === 1) c.first++;
    S.streak++; S.best = Math.max(S.best, S.streak); if (!hintsShown && tries === 1) S.noHint++;
    const pts = tries === 1 ? Math.max(4, 10 - 2 * hintsShown) : Math.max(2, 5 - hintsShown);
    const bonus = S.streak % 3 === 0 ? 5 : 0;
    S.points += pts + bonus; if (round) round.ok++; roundRecord(true);
    const fromList = mkRemove(Q.cat, Q.seed); actLog('q');
    const nb = newBadges(); save();
    fb.innerHTML = `<div class="fb ok"><span class="h">${pick(['Σωστό!', 'Μπράβο!', 'Τέλεια!', 'Εξαιρετικά!'])}${UIP.calm ? '' : ` +${pts} πόντοι${bonus ? ` · μπόνους σερί +${bonus}` : ''}`}</span>${nb.map((b) => `<span class="nb">${I('award')}Νέο παράσημο: <b>${b}</b></span>`).join('')}${fromList ? '<span>Η ερώτηση βγήκε από τη λίστα «Τα λάθη μου».</span>' : ''}<div class="sol">${r.solution}</div></div>`;
    celebrate(pts + bonus); finishUI();
  } else {
    for (const d of r.diag) if (d.tag) S.tags[d.tag] = (S.tags[d.tag] || 0) + 1;
    const msgs = r.diag.filter((d) => d.msg).map((d) => `<span>${d.msg}</span>`).join('');
    const nOk = r.ok.filter(Boolean).length;
    if (!r.solution) {
      save();
      fb.innerHTML = `<div class="fb bad"><span class="h">Όχι ακόμα${ps.length > 1 ? ` · ${nOk} από ${ps.length} σωστά` : ''}.</span>${msgs || '<span>Έλεγξε ξανά τις πράξεις σου. Μπορείς να πατήσεις «Βοήθεια».</span>'}<span>Διόρθωσε τα κόκκινα και πάτα ξανά «Έλεγχος».</span></div>`;
    } else {
      done = true; c.att++; S.streak = 0; roundRecord(false); mkAdd(Q.cat, Q.seed, msgs); actLog('q'); save();
      fb.innerHTML = `<div class="fb bad"><span class="h">Δες τη λύση και συνέχισε. Την επόμενη φορά θα το βρεις!</span>${msgs}<div class="sol">${r.solution}</div></div>`;
      finishUI();
    }
  }
  const s = app.querySelector('.top .stats'); if (s) s.outerHTML = statsChips();
}
function finishUI() { $('#check').hidden = true; $('#hint').hidden = true; const n = $('#next'); n.hidden = false; n.focus(); }
function celebrate(p) {
  if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  const b = $('#fb').getBoundingClientRect(), x = b.left + 40, y = b.top + 10;
  const pop = document.createElement('div'); pop.className = 'pop'; pop.textContent = `+${p}`; pop.style.left = x + 'px'; pop.style.top = y + 'px'; document.body.appendChild(pop); setTimeout(() => pop.remove(), 1000);
  const cols = ['var(--sun)', 'var(--accent)', 'var(--ok)', 'var(--bad)'];
  for (let i = 0; i < 18; i++) { const s = document.createElement('div'); s.className = 'spark'; s.style.left = x + 'px'; s.style.top = y + 'px'; s.style.background = cols[i % 4]; s.style.setProperty('--dx', rand(-120, 120) + 'px'); s.style.setProperty('--dy', rand(-110, 40) + 'px'); document.body.appendChild(s); setTimeout(() => s.remove(), 900); }
}
function roundRecord(ok) {
  if (!round) return;
  const ch = (CATS.find((c) => c.id === Q.cat) || {}).chapter || '';
  const b = round.by[ch] || (round.by[ch] = { ok: 0, n: 0 });
  b.n++; if (ok) b.ok++;
}
function endRound() {
  S.rounds.push(Math.round(round.ok / round.n * 10)); const nb = newBadges(); save(); const r = round; round = null;
  const pct = r.ok / r.n;
  const msg = pct >= 0.9 ? 'Είσαι έτοιμος για το διαγώνισμα!' : pct >= 0.7 ? 'Πολύ καλά! Λίγη εξάσκηση ακόμα στα δύσκολα.' : pct >= 0.5 ? 'Καλή προσπάθεια. Ξαναδοκίμασε έναν γύρο.' : 'Ας κάνουμε εξάσκηση ενότητα-ενότητα, με τη βοήθεια.';
  const rows = Object.entries(r.by).map(([id, b]) => { const ch = CHAPTERS.find((c) => c.id === id); return `<tr><td>${esc(ch ? ch.title : id)}</td><td>${b.ok} / ${b.n}</td></tr>`; }).join('');
  app.innerHTML = `<section class="card" style="text-align:center;display:flex;flex-direction:column;gap:10px;align-items:center"><span class="muted">${esc(r.title)}</span><h1>Τέλος επαναληπτικού</h1><div class="score"><span class="mark">${r.ok} / ${r.n}</span></div><p style="margin:0">${msg}</p>${nb.map((b) => `<span class="nb">${I('award')}Νέο παράσημο: <b>${b}</b></span>`).join('')}
  ${Object.keys(r.by).length > 1 ? `<div class="tbl" style="width:100%;max-width:420px"><table class="st"><tr><th>Κεφάλαιο</th><th>Σωστά</th></tr>${rows}</table></div>` : ''}${statsChips()}
  <div class="row" style="justify-content:center"><button class="btn big-btn" id="again">Νέο επαναληπτικό</button><button class="btn ghost" id="home">Πίσω</button></div></section>`;
  $('#again').onclick = () => startRound(r.cats, r.n, r.title); $('#home').onclick = route;
}

/* ---------- teacher area (PIN checked by the server) ---------- */
const TK = 'diairetotita-teacher';
const getToken = () => { try { const t = JSON.parse(sessionStorage.getItem(TK)); return t && t.expires * 1000 > Date.now() ? t.token : null; } catch (e) { return null; } };
const setToken = (t) => { try { t ? sessionStorage.setItem(TK, JSON.stringify(t)) : sessionStorage.removeItem(TK); } catch (e) { /* ignore */ } };
let memToken = null;
const token = () => getToken() || memToken;
const authed = (path) => api(path, { headers: { authorization: `Bearer ${token()}` } });

function renderTeacher(err) {
  if (!token()) return renderPin(err);
  renderTeacherPanel();
}
function renderPin(err) {
  app.innerHTML = `<div class="top">${backBtn('Αρχική')}</div><h1>Για τον εκπαιδευτικό</h1>
  <section class="card"><form class="pin" id="pinf" autocomplete="off"><label for="pin">Γράψτε το PIN για να δείτε την πρόοδο, τις λύσεις και τα φύλλα εργασίας.</label>
  <input id="pin" type="password" inputmode="numeric" maxlength="12" autocomplete="one-time-code" aria-describedby="pinerr">
  <span class="err" id="pinerr" role="alert">${err ? esc(err) : ''}</span>
  <button class="btn" id="pinbtn" type="submit">Είσοδος</button></form></section>`;
  $('#home').onclick = () => go('#');
  const inp = $('#pin'); inp.focus();
  $('#pinf').addEventListener('submit', async (e) => {
    e.preventDefault();
    const pin = inp.value.trim(); if (!pin) return;
    $('#pinbtn').disabled = true; $('#pinerr').textContent = '';
    try {
      const t = await api('/api/teacher/login', { method: 'POST', body: JSON.stringify({ pin }) });
      memToken = t.token; setToken(t); renderTeacherPanel();
    } catch (ex) {
      $('#pinerr').textContent = ex.message; $('#pinbtn').disabled = false; inp.select();
    }
  });
}
let confirmReset = false;
function renderTeacherPanel() {
  const tags = Object.entries(S.tags).sort((a, b) => b[1] - a[1]);
  app.innerHTML = `<div class="top">${backBtn('Αρχική')}<button class="btn ghost" id="logout">Αποσύνδεση</button></div><h1>Για τον εκπαιδευτικό</h1><p class="muted" style="margin:0">Τάξη: <b>${gradeName()}</b>. Η τάξη αλλάζει από την αρχική οθόνη.</p>
  <div class="tools">
   <section class="card"><h2>Διαγωνίσματα για εκτύπωση</h2><p class="muted" style="margin:0">Έτοιμα διαγωνίσματα της ${gradeName()} με τις λύσεις τους.</p><div id="papers" class="papers"><span class="muted">Φόρτωση…</span></div></section>
   <section class="card"><h2>Φύλλο εργασίας</h2><p class="muted" style="margin:0">Καινούργιες ασκήσεις με απαντήσεις στο τέλος.</p>
    <label for="ws-cat">Ενότητα</label><select id="ws-cat"><optgroup label="Ολόκληρα κεφάλαια"><option value="chapters=${GCH().map((c) => c.id).join(',')}">Όλα τα κεφάλαια της ${gradeName()} (επανάληψη χρονιάς)</option>${GCH().map((ch) => `<option value="chapters=${ch.id}">Κεφάλαιο ${ch.number}: ${esc(ch.title)}</option>`).join('')}</optgroup>${GCH().map((ch) => `<optgroup label="${esc(ch.title)}">${chCats(ch.id).map((c) => `<option value="cat=${c.id}">${esc(c.title)}</option>`).join('')}</optgroup>`).join('')}</select>
    <label for="ws-n">Αριθμός ασκήσεων</label><select id="ws-n">${[6, 8, 10, 13, 15, 20].map((n) => `<option${n === 10 ? ' selected' : ''}>${n}</option>`).join('')}</select>
    <button class="btn" id="ws-go">Δημιουργία φύλλου</button></section>
  </div>
  <div id="tmsg"></div>
  <p class="muted" style="margin:0">Η πρόοδος του μαθητή αποθηκεύεται μόνο σε αυτή τη συσκευή και σε αυτόν τον browser.</p>
  <section class="card"><h2 style="margin-bottom:8px">Πρόοδος ανά κατηγορία</h2><div class="tbl"><table class="st"><tr><th>Κατηγορία</th><th>Ερωτήσεις</th><th>Σωστά</th><th>Με την 1η</th><th>Ποσοστό</th></tr>
  ${GCH().map((ch) => { const t = chStats(ch.id); return `<tr><th colspan="4">Κεφάλαιο ${ch.number}: ${esc(ch.title)}</th><th>${t.att ? Math.round(t.ok / t.att * 100) + '%' : '–'}</th></tr>` + chCats(ch.id).map((c) => { const s = cs(c.id); return `<tr><td>${esc(c.title)}</td><td>${s.att}</td><td>${s.ok}</td><td>${s.first}</td><td>${s.att ? Math.round(s.ok / s.att * 100) + '%' : '–'}</td></tr>`; }).join(''); }).join('')}</table></div></section>
  <section class="card"><h2 style="margin-bottom:8px">Συχνά λάθη</h2>${tags.length ? `<div class="tbl"><table class="st"><tr><th>Λάθος</th><th>Φορές</th></tr>${tags.map(([t, n]) => `<tr><td>${esc(TAGS[t] || t)}</td><td>${n}</td></tr>`).join('')}</table></div>` : '<p class="muted" style="margin:0">Δεν έχουν καταγραφεί λάθη ακόμα.</p>'}</section>
  <section class="card"><h2 style="margin-bottom:8px">Σύνολο</h2><p style="margin:0">Πόντοι: ${S.points} · Καλύτερο σερί: ${S.best} · Επαναληπτικά: ${S.rounds.length ? S.rounds.join(', ') + ' (βαθμός στα 10)' : 'κανένα'}</p>
  <div style="margin-top:12px">${confirmReset ? '<div class="confirm"><span>Να σβηστεί όλη η πρόοδος;</span><button class="btn" id="yes" style="background:var(--bad)">Ναι, σβήσ\' την</button><button class="btn ghost" id="no">Άκυρο</button></div>' : '<button class="btn ghost" id="reset">Μηδενισμός προόδου</button>'}</div></section>`;
  $('#home').onclick = () => { confirmReset = false; go('#'); };
  $('#logout').onclick = () => { memToken = null; setToken(null); go('#'); };
  const r = $('#reset'); if (r) r.onclick = () => { confirmReset = true; renderTeacherPanel(); };
  const y = $('#yes'); if (y) y.onclick = () => { S = fresh(); save(); confirmReset = false; renderTeacherPanel(); };
  const n = $('#no'); if (n) n.onclick = () => { confirmReset = false; renderTeacherPanel(); };
  loadPapers();
  $('#ws-go').onclick = () => openDoc(async () => worksheetHTML(await authed(`/api/teacher/worksheet?${$('#ws-cat').value}&n=${$('#ws-n').value}&seed=${newSeed()}`)));
}
let PAPERS = null;
async function loadPapers() {
  const box = $('#papers');
  try {
    if (!PAPERS) PAPERS = (await authed('/api/teacher/resources')).papers;
  } catch (e) {
    if (e.status === 401) { memToken = null; setToken(null); renderPin('Η σύνδεση έληξε. Γράψτε ξανά το PIN.'); return; }
    box.innerHTML = netError(e); return;
  }
  const mine = PAPERS.filter((p) => p.grade === S.grade);
  box.innerHTML = mine.length ? mine.map((p, i) => `<div class="paper"><b>${esc(p.title)}</b><div class="row"><button class="btn" data-paper="${i}" data-kind="exam">Διαγώνισμα</button><button class="btn ghost" data-paper="${i}" data-kind="solutions">Λύσεις</button></div></div>`).join('') : '<span class="muted">Δεν υπάρχουν ακόμα έτοιμα διαγωνίσματα για αυτή την τάξη. Φτιάξτε φύλλο εργασίας από τα δεξιά.</span>';
  box.querySelectorAll('[data-paper]').forEach((b) => (b.onclick = () => { const p = mine[+b.dataset.paper]; openDoc(async () => p[b.dataset.kind]); }));
}
async function openDoc(load) {
  const w = window.open('', '_blank');
  if (w) w.document.write('<p style="font-family:sans-serif">Φόρτωση…</p>');
  try {
    const html = await load();
    if (w) { w.document.open(); w.document.write(html); w.document.close(); }
    else $('#tmsg').innerHTML = '<div class="fb bad"><span>Ο browser μπλόκαρε το νέο παράθυρο. Επιτρέψτε τα αναδυόμενα παράθυρα για αυτή τη σελίδα.</span></div>';
  } catch (e) {
    if (w) w.close();
    if (e.status === 401) { memToken = null; setToken(null); renderPin('Η σύνδεση έληξε. Γράψτε ξανά το PIN.'); return; }
    $('#tmsg').innerHTML = netError(e);
  }
}
function worksheetHTML(ws) {
  const title = `Φύλλο εργασίας – Μαθηματικά ${gradeName()}`;
  const qs = ws.items.map((it, i) => `<div class="q"><b>${i + 1})</b> ${it.prompt}${it.groups.map((g) => `<div class="g">${g.label ? `<div>${g.label}</div>` : ''}${g.parts.map((p) => `${p.choices ? `<div>${p.choices}</div>` : ''}<div class="ln">${p.label || ''} …………………</div>`).join('')}</div>`).join('')}</div>`).join('');
  const key = ws.items.map((it, i) => `<div class="k"><b>${i + 1})</b> ${it.groups.map((g) => g.parts.map((p) => `${p.label ? p.label + ' ' : ''}<b>${p.answer}</b>`).join(' · ')).join(' · ')}<div class="s">${it.solution}</div></div>`).join('');
  return `<!doctype html><html lang="el"><head><meta charset="utf-8"><title>${title}</title><style>
  @page{size:A4;margin:15mm}body{font-family:"Noto Sans",Arial,sans-serif;font-size:11pt;line-height:1.45;color:#111;max-width:180mm;margin:0 auto;padding:8mm}
  h1{font-size:16pt;text-align:center}.q{margin:0 0 6mm;page-break-inside:avoid}.g{margin:2mm 0 0 6mm}.ln{margin:2mm 0;color:#444}.gap{display:inline-block;min-width:1.2em;border-bottom:2px solid #000;text-align:center}
  .big{font-weight:700}.muted{color:#555}.k{margin:0 0 4mm;page-break-inside:avoid}.s{font-size:9.5pt;color:#333;margin:1mm 0 0 6mm}
  table.ladder{border-collapse:collapse;margin:2px 0}table.ladder td{padding:0 8px;text-align:right}table.ladder td:first-child{border-right:2px solid #000}.ladders{display:flex;gap:14px;flex-wrap:wrap}
  .fr{display:inline-flex;flex-direction:column;align-items:center;vertical-align:middle;font-size:.85em;line-height:1.15}.fr>span:first-child{border-bottom:1px solid #000;padding:0 2px}.ov{text-decoration:overline}.fig{max-width:90mm;height:auto;display:block;margin:2mm 0}.vec{position:relative;display:inline-block;padding-top:.3em}.vec::before{content:"";position:absolute;left:0;right:.2em;top:.18em;border-top:1px solid #000}.vec::after{content:"";position:absolute;right:0;top:.04em;width:.28em;height:.28em;border:solid #000;border-width:1px 1px 0 0;transform:rotate(45deg)}table.st{border-collapse:collapse}table.st td,table.st th{border:1px solid #999;padding:2px 6px}.key{page-break-before:always}@media screen{.key{border-top:2px dashed #999;margin-top:10mm;padding-top:6mm}}</style></head><body>
  <h1>${title}</h1><p>Όνομα: ……………………………… &nbsp; Ημερομηνία: ……………</p>${qs}<div class="key"><h1>Απαντήσεις</h1>${key}</div></body></html>`;
}

/* ---------- narrated step "videos" (instructions / method) ----------
   Each lesson is a list of steps {title, show, say}. The step is shown on screen,
   `say` is read by the device's own voice (Web Speech API) and shown as subtitles. */
const LV = { lesson: null, kind: 'instr', i: 0, k: 0, playing: false, token: 0, timer: 0, opener: null, voice: null, audio: null, prefs: { rate: 1, auto: true } };
/** Natural voices, spoken by the server: Ελένη / Κώστας (Speechify), Νέστορας / Αθηνά (Azure).
    The server says which ones it has; the first of them is the default. */
const NATURAL_NAMES = { e: ['sp:e', 'Ελένη'], k: ['sp:k', 'Κώστας'], m: ['az:m', 'Νέστορας'], f: ['az:f', 'Αθηνά'] };
const setNatural = (codes) => { NATURAL = codes.filter((c) => NATURAL_NAMES[c]).map((c) => ({ id: NATURAL_NAMES[c][0], name: `${NATURAL_NAMES[c][1]} (φυσική φωνή)` })); };
const isNatural = (id) => /^(az|sp):/.test(id || '');
/** The natural voice in use, or null when the pupil chose a voice of the device. */
function naturalVoice() {
  if (!NATURAL_TTS || !NATURAL.length) return null;
  const saved = LV.prefs.voice;
  if (saved && !isNatural(saved)) return null;
  return NATURAL.some((v) => v.id === saved) ? saved : NATURAL[0].id;
}
const SILENT = 'data:audio/wav;base64,UklGRiQAAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQAAAAA=';
try { Object.assign(LV.prefs, JSON.parse(localStorage.getItem('lesson-prefs')) || {}); } catch (e) { /* ignore */ }
const saveLV = () => { try { localStorage.setItem('lesson-prefs', JSON.stringify(LV.prefs)); } catch (e) { /* ignore */ } };
const hasTTS = 'speechSynthesis' in window && 'SpeechSynthesisUtterance' in window;
const greekVoices = () => (hasTTS ? speechSynthesis.getVoices().filter((v) => /^el/i.test(v.lang)) : []);
/** Better-sounding voices first: the pupil's saved choice, then enhanced / premium ones. */
const voiceScore = (v) => (/premium|enhanced|ενισχυμ|βελτιωμ|neural|natural|online/i.test(v.name) ? 2 : 0) + (v.localService ? 0 : 1);
function pickVoice() {
  const vs = greekVoices();
  LV.voice = vs.find((v) => v.voiceURI === LV.prefs.voice) || vs.slice().sort((a, b) => voiceScore(b) - voiceScore(a))[0] || null;
  fillVoices();
  if (LV.lesson) noteVoice();
}
function fillVoices() {
  const sel = $('#lv-voice'); if (!sel) return;
  const vs = greekVoices();
  const opts = (NATURAL_TTS ? NATURAL : []).map((v) => `<option value="${v.id}">${esc(v.name)}</option>`)
    .concat(vs.map((v) => `<option value="${esc(v.voiceURI)}">${esc(v.name)} (συσκευής)</option>`));
  sel.innerHTML = opts.join('');
  sel.parentElement.hidden = opts.length < 2;
  const nat = naturalVoice();
  if (nat) sel.value = nat; else if (LV.voice) sel.value = LV.voice.voiceURI;
}
if (hasTTS) { pickVoice(); speechSynthesis.addEventListener && speechSynthesis.addEventListener('voiceschanged', pickVoice); }
const sentences = (t) => (t.match(/[^.!;?]+[.!;?]*/g) || [t]).map((x) => x.trim()).filter(Boolean);

function lvDialog() {
  let d = $('#lv');
  if (d) return d;
  document.body.insertAdjacentHTML('beforeend', `<dialog id="lv" class="lv" aria-labelledby="lv-t">
  <div class="lv-head"><h2 id="lv-t"></h2><button class="lv-x" id="lv-x" aria-label="Κλείσιμο">✕</button></div>
  <div class="lv-prog"><div class="lv-dots" id="lv-dots"></div><span class="muted lv-count" id="lv-count"></span></div>
  <div class="bar lv-bar"><i id="lv-bar"></i></div>
  <h3 class="lv-st" id="lv-st"></h3>
  <div class="lv-stage" id="lv-stage"></div>
  <p class="lv-cap" id="lv-cap" aria-live="polite"></p>
  <p class="lv-note" id="lv-note" hidden></p>
  <div class="lv-ctrl">
    <button class="btn ghost" id="lv-prev" aria-label="Προηγούμενο βήμα">◀ Πίσω</button>
    <button class="btn" id="lv-play">▶ Παίξε</button>
    <button class="btn ghost" id="lv-next" aria-label="Επόμενο βήμα">Επόμενο ▶</button>
    <button class="btn ghost" id="lv-again" aria-label="Ξανά αυτό το βήμα">↺ Ξανά</button>
  </div>
  <div class="lv-opt">
    <label>Ταχύτητα <select id="lv-rate"><option value="0.75">αργά</option><option value="1">κανονικά</option><option value="1.2">γρήγορα</option></select></label>
    <label hidden>Φωνή <select id="lv-voice"></select></label>
    <label class="lv-auto"><input type="checkbox" id="lv-auto"> Συνέχεια αυτόματα</label>
  </div>
</dialog>`);
  d = $('#lv');
  $('#lv-x').onclick = () => d.close();
  d.addEventListener('close', lvStop);
  // a click on the dark backdrop (outside the box) closes the video
  d.addEventListener('click', (e) => { if (e.target !== d) return; const r = d.getBoundingClientRect(); if (e.clientX < r.left || e.clientX > r.right || e.clientY < r.top || e.clientY > r.bottom) d.close(); });
  $('#lv-prev').onclick = () => lvGo(LV.i - 1);
  $('#lv-next').onclick = () => lvGo(LV.i + 1);
  $('#lv-again').onclick = () => { LV.playing = true; lvGo(LV.i); };
  $('#lv-play').onclick = () => { LV.playing = !LV.playing; if (LV.playing) { if (LV.k >= LV.lesson.steps[LV.i].parts.length) LV.k = 0; lvSpeak(); } else lvHalt(); lvButtons(); };
  $('#lv-rate').onchange = (e) => { LV.prefs.rate = +e.target.value; saveLV(); if (LV.playing) lvSpeak(); };
  $('#lv-voice').onchange = (e) => { LV.prefs.voice = e.target.value; saveLV(); pickVoice(); if (LV.playing) lvSpeak(); };
  $('#lv-auto').onchange = (e) => { LV.prefs.auto = e.target.checked; saveLV(); };
  d.addEventListener('keydown', (e) => {
    if (e.target.tagName === 'SELECT') return;
    if (e.key === 'ArrowRight') { e.preventDefault(); lvGo(LV.i + 1); }
    else if (e.key === 'ArrowLeft') { e.preventDefault(); lvGo(LV.i - 1); }
  });
  return d;
}
function noteVoice() {
  const n = $('#lv-note'); if (!n) return;
  if (!naturalVoice() && (!hasTTS || !LV.voice)) {
    n.hidden = false;
    n.textContent = hasTTS
      ? 'Η συσκευή δεν έχει ελληνική φωνή, γι’ αυτό διάβασε τους υπότιτλους. Μπορείς να προσθέσεις ελληνική φωνή από τις ρυθμίσεις της συσκευής (Προσβασιμότητα / Μετατροπή κειμένου σε ομιλία).'
      : 'Αυτός ο browser δεν μπορεί να διαβάσει δυνατά. Διάβασε τους υπότιτλους.';
  } else n.hidden = true;
}
async function openLesson(kind, btn) {
  // iOS only lets a page speak after a tap: start (silently) inside the tap.
  if (hasTTS) { try { speechSynthesis.cancel(); const u = new SpeechSynthesisUtterance(' '); u.volume = 0; speechSynthesis.speak(u); } catch (e) { /* ignore */ } }
  if (NATURAL_TTS) { try { LV.audio = LV.audio || new Audio(); LV.audio.src = SILENT; LV.audio.play().catch(() => {}); } catch (e) { /* ignore */ } }
  if (LV.loading) return;
  LV.loading = true;
  try {
    const l = await api(`/api/lesson?cat=${Q.cat}&seed=${Q.seed}&kind=${kind}`);
    const d = lvDialog();
    LV.lesson = l; LV.kind = kind; LV.opener = btn; LV.playing = true;
    $('#lv-t').textContent = l.title;
    $('#lv-rate').value = String(LV.prefs.rate); if (!$('#lv-rate').value) $('#lv-rate').value = '1';
    $('#lv-auto').checked = LV.prefs.auto;
    $('#lv-dots').innerHTML = l.steps.map((s, i) => `<button class="lv-dot" data-i="${i}" title="${esc(s.title)}" aria-label="Βήμα ${i + 1}: ${esc(s.title)}">${i + 1}</button>`).join('');
    d.querySelectorAll('.lv-dot').forEach((b) => (b.onclick = () => { LV.playing = true; lvGo(+b.dataset.i); }));
    pickVoice();
    if (!d.open) d.showModal();
    lvGo(0);
    $('#lv-play').focus();
  } catch (e) {
    $('#fb').innerHTML = netError(e);
  } finally { LV.loading = false; }
}
function lvHalt() {
  LV.token++; clearTimeout(LV.timer);
  if (hasTTS) speechSynthesis.cancel();
  if (LV.audio) { LV.audio.onended = LV.audio.onerror = null; LV.audio.pause(); }
}
function lvStop() { lvHalt(); LV.playing = false; if (LV.opener && document.contains(LV.opener)) LV.opener.focus(); }
function lvButtons() {
  const n = LV.lesson.steps.length;
  $('#lv-play').textContent = LV.playing ? '⏸ Παύση' : '▶ Παίξε';
  $('#lv-prev').disabled = LV.i <= 0;
  $('#lv-next').disabled = LV.i >= n - 1;
}
function lvGo(i) {
  const steps = LV.lesson.steps;
  if (i < 0 || i >= steps.length) return;
  lvHalt();
  LV.i = i; LV.k = 0;
  const s = steps[i];
  $('#lv-st').textContent = s.title;
  $('#lv-count').textContent = `Βήμα ${i + 1} από ${steps.length}`;
  $('#lv-bar').style.width = `${Math.round(((i + 1) / steps.length) * 100)}%`;
  document.querySelectorAll('.lv-dot').forEach((b, j) => { b.classList.toggle('done', j < i); b.classList.toggle('cur', j === i); b.setAttribute('aria-current', j === i ? 'step' : 'false'); b.textContent = j < i ? '✓' : String(j + 1); });
  const st = $('#lv-stage'); st.classList.remove('in'); st.innerHTML = `<div>${s.show}</div>`; void st.offsetWidth; st.classList.add('in');
  $('#lv-cap').innerHTML = s.parts.map((t, k) => `<span data-k="${k}">${esc(t)}</span>`).join(' ');
  lvButtons();
  if (LV.playing) lvSpeak();
}
function lvMark() { document.querySelectorAll('#lv-cap span').forEach((x) => x.classList.toggle('cur', +x.dataset.k === LV.k)); }
function lvSpeak() {
  lvHalt();
  const my = LV.token;
  const parts = LV.lesson.steps[LV.i].parts;
  const ttsUrl = (step, k) => `/api/tts?cat=${encodeURIComponent(Q.cat)}&seed=${Q.seed}&kind=${LV.kind}&step=${step}&k=${k}&v=${naturalVoice().slice(3)}`;
  const rate = LV.prefs.rate || 1;
  const after = () => {
    if (my !== LV.token) return;
    LV.k++;
    if (LV.k < parts.length) return say();
    lvMark();
    // end of step
    if (LV.prefs.auto && LV.i < LV.lesson.steps.length - 1) LV.timer = setTimeout(() => { if (my === LV.token) lvGo(LV.i + 1); }, 1100 / rate);
    else { LV.playing = false; lvButtons(); }
  };
  const say = () => {
    lvMark();
    const text = parts[LV.k];
    if (naturalVoice()) {
      const a = LV.audio || (LV.audio = new Audio());
      let failed = false;
      const fail = () => { if (failed || my !== LV.token) return; failed = true; a.onended = a.onerror = null; device(text); };
      a.onended = () => { if (my === LV.token) after(); };
      a.onerror = fail;
      a.src = ttsUrl(LV.i, LV.k);
      a.playbackRate = rate;
      a.play().catch(fail);
      // fetch the next sentence while this one plays
      const steps = LV.lesson.steps;
      const nx = LV.k + 1 < parts.length ? [LV.i, LV.k + 1] : LV.i + 1 < steps.length ? [LV.i + 1, 0] : null;
      if (nx) fetch(ttsUrl(nx[0], nx[1])).catch(() => {});
    } else device(text);
  };
  const device = (text) => {
    if (hasTTS && LV.voice) {
      const u = new SpeechSynthesisUtterance(text);
      u.voice = LV.voice; u.lang = LV.voice.lang; u.rate = 0.95 * rate;
      u.onend = after; u.onerror = (e) => { if (e.error !== 'interrupted' && e.error !== 'canceled') after(); };
      speechSynthesis.speak(u);
    } else {
      // no voice: give time to read the subtitle
      LV.timer = setTimeout(after, Math.max(1800, text.length * 75) / rate);
    }
  };
  say();
}

/* ---------- boot ---------- */
(async function boot() {
  try {
    const r = await api('/api/categories');
    CATS = r.categories; TAGS = r.tags; CHAPTERS = r.chapters; METHOD_CATS = r.method_cats || []; NATURAL_TTS = !!r.tts; setNatural(r.tts_voices || (r.tts ? ['m', 'f'] : []));
    route();
  } catch (e) {
    app.innerHTML = netError(e) + '<div class="row" style="margin-top:12px"><button class="btn" onclick="location.reload()">Δοκίμασε ξανά</button></div>';
  }
})();
