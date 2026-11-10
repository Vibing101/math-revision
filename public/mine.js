'use strict';
/* «Τα λάθη μου» and the weekly personal target. Both live only in this browser.
   - A question the pupil could not solve is kept as {cat, seed}: the server rebuilds the same
     question from those two, so it can be tried again. Solving it removes it from the list.
   - The weekly target counts effort (questions finished, theory sections reviewed), never
     marks, and a missed week is never punished. See docs/learning-design.md. */

const MKKEY = 'mistakes-v1', ACTKEY = 'activity-v1', GOALKEY = 'goals-v1';
const readJSON = (k, d) => { try { const v = JSON.parse(localStorage.getItem(k)); return v == null ? d : v; } catch (e) { return d; } };
const writeJSON = (k, v) => { try { localStorage.setItem(k, JSON.stringify(v)); } catch (e) { /* private mode */ } };

/* ---------- mistakes ---------- */
let MK = readJSON(MKKEY, []);
if (!Array.isArray(MK)) MK = [];
const plain = (h) => String(h || '').replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim().slice(0, 240);
function mkAdd(cat, seed, msg) {
  MK = MK.filter((m) => !(m.cat === cat && m.seed === seed));
  MK.unshift({ cat, seed, msg: plain(msg), ts: Date.now() });
  MK = MK.slice(0, 40); writeJSON(MKKEY, MK);
}
function mkRemove(cat, seed) {
  const n = MK.length;
  MK = MK.filter((m) => !(m.cat === cat && m.seed === seed));
  if (MK.length !== n) writeJSON(MKKEY, MK);
  return MK.length !== n;
}

/* ---------- activity log and weekly target ---------- */
let ACT = readJSON(ACTKEY, {});
if (typeof ACT !== 'object' || Array.isArray(ACT)) ACT = {};
const dayKey = (d = new Date()) => `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
const mondayOf = (d = new Date()) => { const x = new Date(d.getFullYear(), d.getMonth(), d.getDate()); x.setDate(x.getDate() - ((x.getDay() + 6) % 7)); return x; };
function actLog(kind) {
  const k = dayKey(), e = ACT[k] || (ACT[k] = { q: 0, t: 0 });
  e[kind] = (e[kind] || 0) + 1;
  const keep = Object.keys(ACT).sort().slice(-70); // about ten weeks
  ACT = Object.fromEntries(keep.map((x) => [x, ACT[x]]));
  writeJSON(ACTKEY, ACT);
}
/** Totals for the week that starts `back` weeks before the current one. */
function weekTotals(back = 0) {
  const m = mondayOf(); m.setDate(m.getDate() - 7 * back);
  const t = { q: 0, t: 0 };
  for (let i = 0; i < 7; i++) { const d = new Date(m); d.setDate(m.getDate() + i); const e = ACT[dayKey(d)]; if (e) { t.q += e.q || 0; t.t += e.t || 0; } }
  return t;
}
let GOAL = readJSON(GOALKEY, null);
const saveGoal = () => writeJSON(GOALKEY, GOAL);
const GOAL_Q = [0, 10, 20, 30, 50], GOAL_T = [0, 1, 3, 5];

function goalCardHTML() {
  if (!GOAL) {
    return `<section class="card goal"><h2>Ο στόχος μου αυτή την εβδομάδα</h2>
      <p style="margin:0">Βάλε έναν μικρό στόχο που μπορείς να πετύχεις. Μετράει η προσπάθεια, όχι ο βαθμός.</p>
      <div class="row"><button class="btn" id="goal-set" type="button">Βάλε στόχο</button>${mkLink()}</div></section>`;
  }
  const now = weekTotals(0), last = weekTotals(1);
  const rows = [['q', 'Ερωτήσεις που ολοκλήρωσα', GOAL.q], ['t', 'Ενότητες θεωρίας που είδα', GOAL.t]].filter((r) => r[2] > 0).map(([k, label, goal]) => {
    const v = now[k], pct = Math.min(100, Math.round(v / goal * 100));
    return `<div class="grow"><span>${label}: <b>${v}</b> από ${goal}</span><div class="bar" role="progressbar" aria-valuemin="0" aria-valuemax="${goal}" aria-valuenow="${Math.min(v, goal)}" aria-label="${label}"><i style="width:${pct}%"></i></div></div>`;
  }).join('');
  const reached = [['q', GOAL.q], ['t', GOAL.t]].filter((r) => r[1] > 0).every(([k, g]) => now[k] >= g) && (GOAL.q > 0 || GOAL.t > 0);
  const msg = !(GOAL.q > 0 || GOAL.t > 0) ? 'Δεν έχεις βάλει στόχο αυτή την εβδομάδα.'
    : reached ? 'Έφτασες τον στόχο σου! Ό,τι κάνεις πέρα από αυτόν είναι μπόνους.'
    : (now.q + now.t) === 0 ? 'Ένα μικρό πρώτο βήμα αρκεί για να ξεκινήσεις.' : 'Συνέχισε με τον δικό σου ρυθμό. Και τα μικρά βήματα μετράνε.';
  const prev = last.q + last.t ? `<p class="muted" style="margin:0">Την προηγούμενη εβδομάδα: ${last.q} ερωτήσεις, ${last.t} ενότητες θεωρίας.</p>` : '';
  return `<section class="card goal"><h2>Ο στόχος μου αυτή την εβδομάδα</h2>${rows}<p style="margin:0" aria-live="polite">${msg}</p>${prev}
    <div class="row"><button class="btn ghost sm" id="goal-set" type="button">Άλλαξε τον στόχο</button>${mkLink()}</div></section>`;
}
const mkLink = () => `<button class="btn ghost sm" id="go-mk" type="button">Τα λάθη μου${MK.length ? ` (${MK.length})` : ''}</button>`;

function openGoal() {
  let d = document.getElementById('goald');
  if (!d) { d = document.createElement('dialog'); d.id = 'goald'; d.className = 'prefs'; document.body.appendChild(d); }
  const cur = GOAL || { q: 20, t: 3 };
  d.setAttribute('aria-labelledby', 'goal-t');
  d.innerHTML = `<h2 id="goal-t">Ο στόχος της εβδομάδας</h2>
    <p class="muted">Διάλεξε κάτι που μπορείς να πετύχεις. Μπορείς να τον αλλάξεις όποτε θέλεις. Ο στόχος ξεκινά από την αρχή κάθε Δευτέρα.</p>
    <fieldset><legend>Ερωτήσεις που θα ολοκληρώσω</legend>${seg('gq', GOAL_Q.map((n) => (n ? String(n) : 'Κανένας στόχος')), Math.max(0, GOAL_Q.indexOf(cur.q)), 'Στόχος ερωτήσεων')}</fieldset>
    <fieldset><legend>Ενότητες θεωρίας που θα δω</legend>${seg('gt', GOAL_T.map((n) => (n ? String(n) : 'Κανένας στόχος')), Math.max(0, GOAL_T.indexOf(cur.t)), 'Στόχος ενοτήτων θεωρίας')}</fieldset>
    <div class="row"><button class="btn" id="goal-ok" type="button">Αποθήκευση</button><button class="btn ghost" id="goal-cancel" type="button">Άκυρο</button></div>`;
  d.querySelector('#goal-ok').onclick = () => {
    const v = (n) => +d.querySelector(`input[name="${n}"]:checked`).value;
    GOAL = { q: GOAL_Q[v('gq')], t: GOAL_T[v('gt')] }; saveGoal(); d.close();
    if (!location.hash || location.hash === '#') route();
  };
  d.querySelector('#goal-cancel').onclick = () => d.close();
  d.showModal();
}

/* ---------- the «Τα λάθη μου» page ---------- */
function renderMistakes() {
  const catOf = (id) => CATS.find((c) => c.id === id);
  const live = MK.filter((m) => catOf(m.cat));
  const items = live.map((m) => {
    const c = catOf(m.cat), ch = CHAPTERS.find((x) => x.id === c.chapter);
    return `<li class="card mk"><span class="muted">${esc(ch ? ch.title : '')} · ${esc(c.section || '')}</span><b>${esc(c.title)}</b>
      ${m.msg ? `<span class="muted">Σημείωση: ${esc(m.msg)}</span>` : ''}
      <span class="row"><button class="btn sm" type="button" data-retry="${m.cat}|${m.seed}">Δοκίμασε ξανά</button><button class="btn ghost sm" type="button" data-drop="${m.cat}|${m.seed}" aria-label="Αφαίρεση από τη λίστα: ${esc(c.title)}">Αφαίρεση</button></span></li>`;
  }).join('');
  const topTags = Object.entries(S.tags || {}).filter(([t]) => TAGS[t]).sort((a, b) => b[1] - a[1]).slice(0, 3);
  app.innerHTML = `<div class="top">${backBtn('Αρχική')}${statsChips()}</div>
    <header class="hero"><h1>Τα λάθη μου</h1><p class="muted" style="margin:0">Εδώ μαζεύονται οι ερωτήσεις που δεν βγήκαν. Δεν είναι αποτυχίες: είναι η λίστα με ό,τι αξίζει να ξαναδείς. Όταν λύσεις μια ερώτηση, φεύγει από τη λίστα.</p></header>
    ${live.length > 1 ? `<div class="row"><button class="btn big-btn" id="mk-all" type="button">Επανάληψη όλων (${Math.min(live.length, 10)})${I('next')}</button></div>` : ''}
    ${live.length ? `<ul class="mklist">${items}</ul>` : '<section class="card"><p style="margin:0">Δεν έχεις ερωτήσεις για επανάληψη αυτή τη στιγμή. Συνέχισε την εξάσκηση: αν κάτι δεν βγει, θα εμφανιστεί εδώ.</p></section>'}
    ${topTags.length ? `<section class="card"><h2 style="margin-bottom:8px">Τι να προσέχεις</h2><p class="muted" style="margin:0 0 8px">Τα λάθη που εμφανίζονται πιο συχνά στις απαντήσεις σου:</p><ul style="margin:0;padding-left:1.2em">${topTags.map(([t]) => `<li>${esc(TAGS[t])}</li>`).join('')}</ul></section>` : ''}`;
  $('#home').onclick = () => go('#');
  const parse = (v) => { const i = v.lastIndexOf('|'); return [v.slice(0, i), Number(v.slice(i + 1))]; };
  app.querySelectorAll('[data-retry]').forEach((b) => (b.onclick = () => { const [c, s] = parse(b.dataset.retry); round = null; start(c, s); }));
  app.querySelectorAll('[data-drop]').forEach((b) => (b.onclick = () => { const [c, s] = parse(b.dataset.drop); mkRemove(c, s); renderMistakes(); const f = app.querySelector('.mklist button, #home'); if (f) f.focus(); }));
  const all = $('#mk-all');
  if (all) all.onclick = () => startMistakes(live.slice(0, 10));
}
function startMistakes(list) {
  round = { n: list.length, i: 0, ok: 0, cats: list.map((m) => m.cat), title: 'Τα λάθη μου', by: {}, mk: list };
  start(null);
}
