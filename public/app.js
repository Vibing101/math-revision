'use strict';
/* Thin client: questions, hints and checking come from the Rust API (/api/*).
   Points, streaks and progress stay in this browser (localStorage). */

const $ = (s) => document.querySelector(s);
const app = $('#app');
const esc = (s) => String(s).replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]));
const rand = (a, b) => a + Math.floor(Math.random() * (b - a + 1));
const pick = (a) => a[Math.floor(Math.random() * a.length)];
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
let CATS = [], TAGS = {}, CHAPTERS = [];
const chCats = (ch) => CATS.filter((c) => c.chapter === ch);
const GRADES = [{ id: 'a', name: 'Α΄ Γυμνασίου' }, { id: 'g', name: 'Γ΄ Γυμνασίου' }];
const gradeName = () => (GRADES.find((g) => g.id === S.grade) || GRADES[0]).name;
/** Chapters of the selected school year. */
const GCH = () => CHAPTERS.filter((c) => c.grade === (S.grade || 'a'));
const gradeCats = () => CATS.filter((c) => GCH().some((ch) => ch.id === c.chapter));
const chStats = (ch) => chCats(ch).reduce((a, c) => { const s = cs(c.id); return { att: a.att + s.att, ok: a.ok + s.ok }; }, { att: 0, ok: 0 });

const LEVELS = [[0, 'Μαθητευόμενος'], [100, 'Εξερευνητής διαιρετών'], [250, 'Κυνηγός πρώτων'], [450, 'Μάστορας του ΜΚΔ'], [700, 'Μάγος του ΕΚΠ'], [1000, 'Θρύλος της Διαιρετότητας']];
function level() {
  let i = 0; while (i + 1 < LEVELS.length && S.points >= LEVELS[i + 1][0]) i++;
  const nx = LEVELS[i + 1];
  return { i, name: LEVELS[i][1], pct: nx ? (S.points - LEVELS[i][0]) / (nx[0] - LEVELS[i][0]) * 100 : 100, next: nx ? nx[0] - S.points : 0 };
}
const BADGES = [
  { id: 'first', t: 'Πρώτη σωστή', ok: () => Object.values(S.cats).some((c) => c.ok > 0) },
  { id: 's5', t: 'Σερί 5', ok: () => S.best >= 5 }, { id: 's10', t: 'Σερί 10', ok: () => S.best >= 10 },
  { id: 'pow', t: 'Κυνηγός εκθετών (5 σωστά)', ok: () => cs('pow').ok >= 5 },
  { id: 'time', t: 'Άρχοντας του χρόνου (3 σωστά)', ok: () => cs('time').ok >= 3 },
  { id: 'prime', t: 'Ανιχνευτής πρώτων (5 σωστά)', ok: () => cs('prime').ok >= 5 },
  { id: 'nh', t: '10 σωστά χωρίς βοήθεια', ok: () => S.noHint >= 10 },
  { id: 'all', t: 'Σωστό σε όλες τις κατηγορίες της τάξης', ok: () => gradeCats().length > 0 && gradeCats().every((c) => cs(c.id).ok > 0) },
  { id: 'r9', t: '90% σε επαναληπτικό', ok: () => S.rounds.some((r) => r >= 9) }];
function newBadges() { const got = []; for (const b of BADGES) if (!S.badges.includes(b.id) && b.ok()) { S.badges.push(b.id); got.push(b.t); } return got; }

/* ---------- navigation: #  (chapters) · #ch-<id> · #year · #teacher ---------- */
let Q = null, round = null, qn = 0, hintsShown = 0, tries = 0, done = false, busy = false;
function go(hash) { round = null; if (location.hash === hash || (!location.hash && hash === '#')) route(); else location.hash = hash; }
function route() {
  round = null;
  const h = location.hash;
  if (!S.grade || h === '#grade') return renderGrade();
  if (h === '#year') return renderYear();
  if (h === '#teacher') return renderTeacher();
  if (h.startsWith('#ch-') && CHAPTERS.some((c) => '#ch-' + c.id === h)) return renderChapter(h.slice(4));
  renderHome();
}
window.addEventListener('hashchange', route);
const backLabel = () => (location.hash.startsWith('#ch-') ? '← Κεφάλαιο' : location.hash === '#year' ? '← Επανάληψη χρονιάς' : '← Αρχική');

function statsChips() { const L = level(); return `<div class="stats"><span class="chip">⭐ <b>${S.points}</b> πόντοι</span><span class="chip">🔥 σερί <b>${S.streak}</b></span><span class="chip">Επίπεδο ${L.i + 1}: <b>${L.name}</b></span></div>`; }
const pctBar = (st) => `<span class="bar"><i style="width:${st.att ? Math.round(st.ok / st.att * 100) : 0}%"></i></span><span class="d">${st.att ? `${st.ok}/${st.att} σωστά` : 'Δεν ξεκίνησε ακόμα'}</span>`;

function renderGrade() {
  app.innerHTML = `<header class="hero"><h1>Επανάληψη Μαθηματικών</h1><p class="muted" style="margin:0">Σε ποια τάξη είσαι; Μπορείς να την αλλάξεις όποτε θέλεις από την αρχική οθόνη.</p></header>
  <div class="grid">${GRADES.map((g) => { const n = CHAPTERS.filter((c) => c.grade === g.id).length; return `<button class="cat chap grade" data-grade="${g.id}"><span class="t">${g.name}</span><span class="d">${n} κεφάλαια από το βιβλίο του Υπουργείου</span></button>`; }).join('')}</div>`;
  app.querySelectorAll('[data-grade]').forEach((b) => (b.onclick = () => { S.grade = b.dataset.grade; save(); go('#'); }));
}

function renderHome() {
  const L = level();
  app.innerHTML = `<header class="hero"><h1>Επανάληψη Μαθηματικών</h1><div class="row" role="group" aria-label="Τάξη">${GRADES.map((g) => `<button class="tog gradechip" data-g="${g.id}" aria-pressed="${g.id === S.grade}">${g.name}</button>`).join('')}</div><p class="muted" style="margin:0">Διάλεξε κεφάλαιο για το επαναληπτικό του ή κάνε επανάληψη για το τέλος της χρονιάς.</p></header>
  <section class="card level">${statsChips()}<div class="bar" aria-hidden="true"><i style="width:${L.pct}%"></i></div><span class="muted" style="font-size:.88rem">${L.next ? `${L.next} πόντοι για το επόμενο επίπεδο` : 'Έφτασες στο ανώτερο επίπεδο!'}</span></section>
  <h2>Κεφάλαια</h2>
  <div class="grid">${GCH().map((ch) => `<button class="cat chap" data-ch="${ch.id}"><span class="sec">Κεφάλαιο ${ch.number}</span><span class="t">${esc(ch.title)}</span><span class="d">${esc(ch.desc)}</span>${pctBar(chStats(ch.id))}</button>`).join('')}
   <button class="cat year" id="go-year"><span class="tag">Τέλος χρονιάς</span><span class="t">Επανάληψη χρονιάς</span><span class="d">Διάλεξε ποια κεφάλαια εξετάζονται και κάνε ένα μικτό επαναληπτικό από όλα μαζί.</span></button></div>
  <section class="card"><h2 style="margin-bottom:8px">Παράσημα</h2><div class="badges">${BADGES.map((b) => `<span class="badge${S.badges.includes(b.id) ? '' : ' off'}">${S.badges.includes(b.id) ? '🏅 ' : ''}${b.t}</span>`).join('')}</div></section>
  <div class="row"><button class="btn ghost" id="go-teacher">Για τον εκπαιδευτικό</button></div>`;
  app.querySelectorAll('[data-ch]').forEach((b) => (b.onclick = () => go('#ch-' + b.dataset.ch)));
  app.querySelectorAll('[data-g]').forEach((b) => (b.onclick = () => { S.grade = b.dataset.g; save(); renderHome(); }));
  $('#go-year').onclick = () => go('#year');
  $('#go-teacher').onclick = () => go('#teacher');
}

function renderChapter(id) {
  const ch = CHAPTERS.find((c) => c.id === id), cats = chCats(id);
  app.innerHTML = `<div class="top"><button class="btn ghost" id="home">← Κεφάλαια</button>${statsChips()}</div>
  <header class="hero"><span class="muted">Κεφάλαιο ${ch.number}</span><h1>${esc(ch.title)}</h1><p class="muted" style="margin:0">${ch.sections.map(esc).join(' · ')}</p></header>
  <section class="card level"><h2>Επαναληπτικό κεφαλαίου</h2><p style="margin:0">10 ερωτήσεις από όλες τις ενότητες του κεφαλαίου, με περισσότερες από εκεί που κάνεις λάθη.</p>
   <div class="row"><button class="btn sun" id="go-round">Ξεκίνα το επαναληπτικό</button></div></section>
  <h2>Ή εξάσκηση ανά ενότητα</h2>
  <div class="grid">${cats.map((c) => `<button class="cat${c.focus ? ' focus' : ''}" data-cat="${c.id}">${c.focus ? '<span class="tag">Να το προσέξεις</span>' : ''}<span class="sec">${esc(c.section)}</span><span class="t">${esc(c.title)}</span><span class="d">${esc(c.desc)}</span>${pctBar(cs(c.id))}</button>`).join('')}</div>`;
  $('#home').onclick = () => go('#');
  app.querySelectorAll('[data-cat]').forEach((b) => (b.onclick = () => { round = null; start(b.dataset.cat); }));
  $('#go-round').onclick = () => startRound(cats.map((c) => c.id), 10, ch.title);
}

function renderYear(msg) {
  const saved = (S.years || {})[S.grade] || (S.grade === 'a' && S.year) || null;
  const sel = (saved && saved.chapters) || GCH().map((c) => c.id);
  const n = (saved && saved.n) || 20;
  app.innerHTML = `<div class="top"><button class="btn ghost" id="home">← Κεφάλαια</button>${statsChips()}</div>
  <header class="hero"><h1>Επανάληψη χρονιάς</h1><span class="muted">${gradeName()}</span><p class="muted" style="margin:0">Διάλεξε τα κεφάλαια που εξετάζονται στις εξετάσεις τέλους χρονιάς. Η επιλογή σου αποθηκεύεται.</p></header>
  <form class="card" id="yf" style="display:flex;flex-direction:column;gap:12px">
   <fieldset style="border:0;padding:0;margin:0;display:flex;flex-direction:column;gap:8px"><legend style="font-weight:700;margin-bottom:6px">Κεφάλαια</legend>
   ${GCH().map((ch) => { const st = chStats(ch.id); return `<label class="chk"><input type="checkbox" name="ch" value="${ch.id}" id="y-${ch.id}"${sel.includes(ch.id) ? ' checked' : ''}><span><b>Κεφάλαιο ${ch.number}: ${esc(ch.title)}</b><br><span class="muted" style="font-size:.88rem">${esc(ch.desc)}${st.att ? ` · ${Math.round(st.ok / st.att * 100)}% σωστά μέχρι τώρα` : ''}</span></span></label>`; }).join('')}
   </fieldset>
   <div class="row"><label for="y-n" style="font-weight:700">Ερωτήσεις</label><select id="y-n">${[10, 20, 30, 40].map((x) => `<option value="${x}"${x === n ? ' selected' : ''}>${x}</option>`).join('')}</select></div>
   <span class="err" role="alert">${msg ? esc(msg) : ''}</span>
   <div class="row"><button class="btn sun" type="submit">Ξεκίνα την επανάληψη</button></div>
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

function startRound(cats, n, title) { round = { n, i: 0, ok: 0, cats, title, by: {} }; start(null); }
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
async function start(cat) {
  const c = cat || weightedCat(round.cats);
  app.innerHTML = `<div class="top"><button class="btn ghost" id="home">${backLabel()}</button>${statsChips()}</div><section class="card"><p class="loading">Φόρτωση ερώτησης…</p></section>`;
  $('#home').onclick = goHome;
  try {
    Q = await api(`/api/question?cat=${encodeURIComponent(c)}&seed=${newSeed()}`);
  } catch (e) {
    app.querySelector('.card').innerHTML = netError(e) + '<div class="row" style="margin-top:12px"><button class="btn" id="retry">Δοκίμασε ξανά</button></div>';
    $('#retry').onclick = () => start(cat);
    return;
  }
  Q.fixed = cat; qn++; hintsShown = 0; tries = 0; done = false; busy = false;
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
  app.innerHTML = `<div class="top"><button class="btn ghost" id="home">${backLabel()}</button>${statsChips()}</div>
  <section class="card"><div class="row" style="justify-content:space-between"><span class="tag">${round && chap && CHAPTERS.length > 1 ? esc(chap.title) + ' · ' : ''}${esc(cat ? cat.title : '')}</span><span class="muted" style="font-size:.88rem">${round ? `${esc(round.title)} · ερώτηση ${round.i + 1} από ${round.n}` : ''}</span></div>
  <div class="qtext" style="margin-top:10px">${Q.prompt}</div>
  ${Q.groups.map((g) => `<div class="group">${g.label ? `<div class="gl">${g.label}</div>` : ''}${g.parts.map((p) => { const id = `p${qn}_${pi++}`; p.id = id; return `<div class="part" id="${id}-w">${partHTML(p, id)}</div>`; }).join('')}</div>`).join('')}
  <div class="row" style="margin-top:16px"><button class="btn" id="check">Έλεγχος</button><button class="btn ghost" id="hint">💡 Βοήθεια (0/4)</button><button class="btn sun" id="next" hidden>${round && round.i + 1 >= round.n ? 'Τέλος γύρου' : 'Επόμενη ➜'}</button></div>
  <div class="hints" id="hints" style="margin-top:12px"></div><div id="fb" style="margin-top:12px"></div></section>`;
  $('#home').onclick = goHome;
  app.querySelectorAll('[data-yn]').forEach((b) => (b.onclick = () => b.parentElement.querySelectorAll('[data-yn]').forEach((x) => x.setAttribute('aria-pressed', x === b))));
  app.querySelectorAll('[data-st]').forEach((b) => (b.onclick = () => b.setAttribute('aria-pressed', b.getAttribute('aria-pressed') !== 'true')));
  app.querySelectorAll('[data-pm]').forEach((b) => (b.onclick = () => { const i = $('#' + b.dataset.pm); const v = i.value.trim(); i.value = /^[-−]/.test(v) ? v.replace(/^[-−]\s*/, '') : '−' + v; i.focus(); }));
  $('#hint').onclick = showHint; $('#check').onclick = check;
  $('#next').onclick = () => { if (round) { round.i++; if (round.i >= round.n) return endRound(); } start(round ? null : Q.fixed); };
  app.querySelectorAll('input').forEach((i) => i.addEventListener('keydown', (e) => { if (e.key === 'Enter') { e.preventDefault(); done ? $('#next').click() : check(); } }));
  const f = app.querySelector('input,select'); if (f && matchMedia('(pointer:fine)').matches) f.focus();
}
async function showHint() {
  if (hintsShown >= 4 || done || busy) return;
  const b = $('#hint'); b.disabled = true;
  try {
    const r = await api(`/api/hint?cat=${Q.cat}&seed=${Q.seed}&n=${hintsShown}`);
    $('#hints').insertAdjacentHTML('beforeend', `<div class="hint"><b>Βοήθεια ${hintsShown + 1}:</b> ${r.hint}</div>`);
    hintsShown++;
  } catch (e) { $('#fb').innerHTML = netError(e); }
  b.textContent = `💡 Βοήθεια (${hintsShown}/4)`; b.disabled = hintsShown >= 4;
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
    ps.forEach((p, i) => { const w = $('#' + p.id + '-w'); w.classList.remove('ok'); w.classList.toggle('bad', r.blank[i]); });
    fb.innerHTML = '<div class="fb bad"><span class="h">Απάντησε σε όλα τα ερωτήματα πριν τον έλεγχο.</span><span>Στο διαγώνισμα κάθε κενό ερώτημα χάνει μονάδες. Τα κουτάκια με κόκκινο περίγραμμα είναι κενά ή δεν είναι αριθμοί.</span></div>';
    return;
  }
  ps.forEach((p, i) => { const w = $('#' + p.id + '-w'); w.classList.toggle('ok', r.ok[i]); w.classList.toggle('bad', !r.ok[i]); });
  tries++;
  const c = cs(Q.cat);
  if (r.correct) {
    done = true; c.att++; c.ok++; if (tries === 1) c.first++;
    S.streak++; S.best = Math.max(S.best, S.streak); if (!hintsShown && tries === 1) S.noHint++;
    const pts = tries === 1 ? Math.max(4, 10 - 2 * hintsShown) : Math.max(2, 5 - hintsShown);
    const bonus = S.streak % 3 === 0 ? 5 : 0;
    S.points += pts + bonus; if (round) round.ok++; roundRecord(true);
    const nb = newBadges(); save();
    fb.innerHTML = `<div class="fb ok"><span class="h">${pick(['Σωστό!', 'Μπράβο!', 'Τέλεια!', 'Εξαιρετικά!'])} +${pts} πόντοι${bonus ? ` · μπόνους σερί +${bonus}` : ''}</span>${nb.map((b) => `<span>🏅 Νέο παράσημο: <b>${b}</b></span>`).join('')}<div class="sol">${r.solution}</div></div>`;
    celebrate(pts + bonus); finishUI();
  } else {
    for (const d of r.diag) if (d.tag) S.tags[d.tag] = (S.tags[d.tag] || 0) + 1;
    const msgs = r.diag.filter((d) => d.msg).map((d) => `<span>${d.msg}</span>`).join('');
    const nOk = r.ok.filter(Boolean).length;
    if (!r.solution) {
      save();
      fb.innerHTML = `<div class="fb bad"><span class="h">Όχι ακόμα${ps.length > 1 ? ` · ${nOk} από ${ps.length} σωστά` : ''}.</span>${msgs || '<span>Έλεγξε ξανά τις πράξεις σου. Μπορείς να πατήσεις «Βοήθεια».</span>'}<span>Διόρθωσε τα κόκκινα και πάτα ξανά «Έλεγχος».</span></div>`;
    } else {
      done = true; c.att++; S.streak = 0; roundRecord(false); save();
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
  app.innerHTML = `<section class="card" style="text-align:center;display:flex;flex-direction:column;gap:10px;align-items:center"><span class="muted">${esc(r.title)}</span><h1>Τέλος επαναληπτικού</h1><div class="big" style="font-size:2.6rem">${r.ok} / ${r.n}</div><p style="margin:0">${msg}</p>${nb.map((b) => `<span>🏅 Νέο παράσημο: <b>${b}</b></span>`).join('')}
  ${Object.keys(r.by).length > 1 ? `<div class="tbl" style="width:100%;max-width:420px"><table class="st"><tr><th>Κεφάλαιο</th><th>Σωστά</th></tr>${rows}</table></div>` : ''}${statsChips()}
  <div class="row" style="justify-content:center"><button class="btn sun" id="again">Νέο επαναληπτικό</button><button class="btn ghost" id="home">Πίσω</button></div></section>`;
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
  app.innerHTML = `<div class="top"><button class="btn ghost" id="home">← Αρχική</button></div><h1>Για τον εκπαιδευτικό</h1>
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
  app.innerHTML = `<div class="top"><button class="btn ghost" id="home">← Αρχική</button><button class="btn ghost" id="logout">Αποσύνδεση</button></div><h1>Για τον εκπαιδευτικό</h1><p class="muted" style="margin:0">Τάξη: <b>${gradeName()}</b>. Η τάξη αλλάζει από την αρχική οθόνη.</p>
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
  <div style="margin-top:10px">${confirmReset ? '<div class="confirm"><span>Να σβηστεί όλη η πρόοδος;</span><button class="btn" id="yes" style="background:var(--bad)">Ναι, σβήσ\' την</button><button class="btn ghost" id="no">Άκυρο</button></div>' : '<button class="btn ghost" id="reset">Μηδενισμός προόδου</button>'}</div></section>`;
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

/* ---------- boot ---------- */
(async function boot() {
  try {
    const r = await api('/api/categories');
    CATS = r.categories; TAGS = r.tags; CHAPTERS = r.chapters;
    route();
  } catch (e) {
    app.innerHTML = netError(e) + '<div class="row" style="margin-top:12px"><button class="btn" onclick="location.reload()">Δοκίμασε ξανά</button></div>';
  }
})();
