'use strict';
/* Learning support (ADHD / Universal Design for Learning): display settings, calm mode,
   break reminder and read-aloud. Everything stays in this browser (localStorage); the
   site never asks for or stores a diagnosis. See docs/learning-design.md. */

const UIKEY = 'ui-prefs-v1';
const UI_DEFAULT = { size: 0, space: 0, calm: false, one: false, breakMin: 0 };
const UIP = Object.assign({}, UI_DEFAULT);
try { Object.assign(UIP, JSON.parse(localStorage.getItem(UIKEY)) || {}); } catch (e) { /* private mode */ }
const saveUI = () => { try { localStorage.setItem(UIKEY, JSON.stringify(UIP)); } catch (e) { /* ignore */ } };

function applyUI() {
  const r = document.documentElement;
  r.style.fontSize = ['100%', '112.5%', '125%'][UIP.size] || '100%';
  r.style.setProperty('--lh', ['1.55', '1.85'][UIP.space] || '1.55');
  r.classList.toggle('calm', !!UIP.calm);
}
applyUI();

/* ---------- settings dialog ---------- */
const seg = (name, opts, cur, label) => `<div class="pseg" role="radiogroup" aria-label="${label}">${opts.map((o, i) => `<label class="segopt"><input type="radio" name="${name}" value="${i}"${i === cur ? ' checked' : ''}><span>${o}</span></label>`).join('')}</div>`;

/** Reading voice and speed: the server's natural voices first, then the device's Greek voices. */
function voiceFieldset() {
  const nat = typeof NATURAL_TTS !== 'undefined' && NATURAL_TTS ? NATURAL.map((v) => [v.id, v.name]) : [];
  const dev = typeof greekVoices === 'function' ? greekVoices().map((v) => [v.voiceURI, `${v.name} (συσκευής)`]) : [];
  const opts = nat.concat(dev);
  if (!opts.length) return '';
  const cur = (typeof naturalVoice === 'function' && naturalVoice()) || (LV.voice && LV.voice.voiceURI) || opts[0][0];
  return `<fieldset><legend>Φωνή ανάγνωσης</legend><select id="p-voice" aria-label="Φωνή ανάγνωσης">${opts.map(([id, name]) => `<option value="${esc(id)}"${id === cur ? ' selected' : ''}>${esc(name)}</option>`).join('')}</select></fieldset>
  <fieldset><legend>Ταχύτητα ανάγνωσης</legend>${seg('rate', ['Αργά', 'Κανονικά', 'Γρήγορα'], speedIndex(), 'Ταχύτητα ανάγνωσης')}</fieldset>`;
}

function openPrefs() {
  let d = document.getElementById('prefs');
  if (!d) { d = document.createElement('dialog'); d.id = 'prefs'; d.className = 'prefs'; document.body.appendChild(d); }
  d.setAttribute('aria-labelledby', 'prefs-t');
  d.innerHTML = `<h2 id="prefs-t">Προσαρμογή</h2>
  <p class="muted">Διάλεξε ό,τι σε βοηθά. Οι επιλογές αποθηκεύονται μόνο σε αυτή τη συσκευή.</p>
  <fieldset><legend>Μέγεθος γραμμάτων</legend>${seg('size', ['Κανονικό', 'Μεγάλο', 'Πολύ μεγάλο'], UIP.size, 'Μέγεθος γραμμάτων')}</fieldset>
  <fieldset><legend>Απόσταση γραμμών</legend>${seg('space', ['Κανονική', 'Μεγαλύτερη'], UIP.space, 'Απόσταση γραμμών')}</fieldset>
  ${voiceFieldset()}
  <label class="chk"><input type="checkbox" id="p-calm"${UIP.calm ? ' checked' : ''}><span><b>Ήρεμη λειτουργία</b><br><span class="muted">Κρύβει πόντους, σερί και σήματα και σταματά τα εφέ. Μένει μόνο η εργασία.</span></span></label>
  <label class="chk"><input type="checkbox" id="p-one"${UIP.one ? ' checked' : ''}><span><b>Θεωρία: μία κάρτα τη φορά</b><br><span class="muted">Η επόμενη κάρτα εμφανίζεται όταν πατήσεις «Επόμενο».</span></span></label>
  <fieldset><legend>Υπενθύμιση για διάλειμμα</legend>${seg('brk', ['Όχι', 'Κάθε 10΄', 'Κάθε 15΄', 'Κάθε 20΄'], [0, 10, 15, 20].indexOf(UIP.breakMin), 'Υπενθύμιση για διάλειμμα')}</fieldset>
  <div class="row"><button class="btn" id="p-close">Έτοιμο</button></div>`;
  const get = (n) => +d.querySelector(`input[name="${n}"]:checked`).value;
  d.onchange = () => {
    UIP.size = get('size'); UIP.space = get('space'); UIP.breakMin = [0, 10, 15, 20][get('brk')];
    UIP.calm = d.querySelector('#p-calm').checked; UIP.one = d.querySelector('#p-one').checked;
    const vs = d.querySelector('#p-voice');
    if (vs && typeof LV !== 'undefined') { LV.prefs.voice = vs.value; if (typeof pickVoice === 'function') pickVoice(); }
    if (typeof LV !== 'undefined' && d.querySelector('input[name="rate"]:checked')) LV.prefs.rate = SPEEDS[get('rate')];
    if (typeof saveLV === 'function') saveLV();
    saveUI(); applyUI(); brkReset(); stopSpeak();
    if (location.hash.startsWith('#th-') && typeof route === 'function') route();
  };
  d.querySelector('#p-close').onclick = () => d.close();
  d.showModal();
}

function mountPrefsButton() {
  const b = document.createElement('button');
  b.className = 'prefs-btn'; b.type = 'button'; b.setAttribute('aria-label', 'Προσαρμογή εμφάνισης και βοήθειας');
  b.innerHTML = '<svg class="ic" viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg><span>Προσαρμογή</span>';
  b.onclick = openPrefs;
  document.body.appendChild(b);
}

/* ---------- break reminder: counts minutes the page is visible; only a banner, never a pop-up over a question ---------- */
let activeMin = 0, brkTimer = 0;
function brkReset() { activeMin = 0; const b = document.getElementById('brk'); if (b) b.remove(); }
setInterval(() => {
  if (!UIP.breakMin || document.visibilityState !== 'visible') return;
  if (++activeMin >= UIP.breakMin && !document.getElementById('brk')) showBreakBanner();
}, 60000);

function showBreakBanner() {
  const b = document.createElement('div');
  b.id = 'brk'; b.className = 'brk card'; b.setAttribute('role', 'status');
  b.innerHTML = `<span>Δουλεύεις καλά εδώ και ${UIP.breakMin} λεπτά. Ένα μικρό διάλειμμα βοηθά τη συγκέντρωση.</span>
    <span class="row"><button class="btn sm" id="brk-go">Διάλειμμα 2 λεπτών</button><button class="btn ghost sm" id="brk-later">Αργότερα</button></span>`;
  document.body.appendChild(b);
  b.querySelector('#brk-go').onclick = () => { b.remove(); startBreak(); };
  b.querySelector('#brk-later').onclick = () => { b.remove(); activeMin = Math.max(0, UIP.breakMin - 5); };
}

function startBreak() {
  let d = document.getElementById('break');
  if (!d) { d = document.createElement('dialog'); d.id = 'break'; d.className = 'prefs'; document.body.appendChild(d); }
  let left = 120;
  d.setAttribute('aria-labelledby', 'break-t');
  d.innerHTML = `<h2 id="break-t">Διάλειμμα</h2>
    <p class="breaktime" id="break-time" aria-hidden="true">2:00</p>
    <ul><li>Σηκώσου και τέντωσε τα χέρια.</li><li>Κοίταξε κάτι μακριά για λίγο.</li><li>Πάρε μερικές αργές ανάσες και πιες νερό.</li></ul>
    <p id="break-msg" aria-live="polite" class="muted"></p>
    <div class="row"><button class="btn" id="break-end">Συνέχισε</button></div>`;
  const t = d.querySelector('#break-time'), m = d.querySelector('#break-msg');
  clearInterval(brkTimer);
  brkTimer = setInterval(() => {
    left--; t.textContent = `${Math.floor(left / 60)}:${String(left % 60).padStart(2, '0')}`;
    if (left <= 0) { clearInterval(brkTimer); m.textContent = 'Το διάλειμμα τελείωσε. Όταν είσαι έτοιμος/η, συνέχισε.'; }
  }, 1000);
  d.querySelector('#break-end').onclick = () => d.close();
  d.onclose = () => { clearInterval(brkTimer); activeMin = 0; };
  d.showModal();
}

/* ---------- read-aloud ----------
   The server says what is read (/api/theory-speech) and speaks it with the natural voices
   (/api/theory-tts: every sentence is made once and kept). Without a natural voice, or when the
   server cannot answer, the device's own Greek voice reads the same sentences. The voice and the
   speed are the ones the pupil chose in «Προσαρμογή» (shared with the narrated lessons). */
const canSpeak = typeof speechSynthesis !== 'undefined' && typeof SpeechSynthesisUtterance !== 'undefined';
const SPOKEN = [[/≤/g, ' μικρότερο ή ίσο του '], [/≥/g, ' μεγαλύτερο ή ίσο του '], [/≠/g, ' διάφορο του '], [/∉/g, ' δεν ανήκει στο '], [/∈/g, ' ανήκει στο '], [/⊆/g, ' υποσύνολο του '], [/∪/g, ' ένωση '], [/∩/g, ' τομή '], [/∅/g, ' κενό σύνολο '],
  [/²/g, ' στο τετράγωνο'], [/³/g, ' στον κύβο'], [/√/g, ' ρίζα '], [/[−–]/g, ' μείον '], [/\+/g, ' συν '], [/[·×]/g, ' επί '], [/÷/g, ' διά '], [/=/g, ' ίσον '], [/\s+/g, ' ']];
/** Fallback text when the server cannot be reached: the card as it is on screen. */
function spokenText(el) {
  let t = [...el.querySelectorAll('.tbody p, .tbody li, .tsteps li:not([hidden])')].map((x) => x.innerText).join('. ') || el.innerText;
  for (const [re, to] of SPOKEN) t = t.replace(re, to);
  return t;
}
/** A listen button makes sense when the device can speak or the site has natural voices. */
const canListen = () => canSpeak || (typeof NATURAL_TTS !== 'undefined' && NATURAL_TTS);
const SPEEDS = [0.85, 1, 1.2];
const speedIndex = () => { const r = (LV.prefs && LV.prefs.rate) || 1; return r < 0.95 ? 0 : r > 1.1 ? 2 : 1; };
const spk = { token: 0, btn: null, audio: null };
function stopSpeak() {
  spk.token++;
  if (spk.audio) { spk.audio.onended = spk.audio.onerror = null; spk.audio.pause(); }
  if (canSpeak) speechSynthesis.cancel();
  if (spk.btn) { spk.btn.setAttribute('aria-pressed', 'false'); spk.btn.lastChild.textContent = 'Άκουσε'; spk.btn = null; }
}
/** Keys (see crates/quiz-core/src/theory.rs) for what is on screen in this card: its text and the steps shown so far. */
function listenKeys(btn, card) {
  const base = btn.dataset.ak, keys = [base];
  if (card.classList.contains('explore')) { // the situation, then whatever has been shown so far, in order
    card.querySelectorAll('[data-k]:not([hidden])').forEach((el) => { if (el.dataset.k !== base) keys.push(el.dataset.k); });
    return keys;
  }
  card.querySelectorAll('.tsteps li').forEach((li, n) => { if (!li.hidden) keys.push(`${base}:${n}`); });
  return keys;
}
async function speechParts(ch, key) {
  const r = await fetch(`/api/theory-speech?ch=${encodeURIComponent(ch)}&key=${encodeURIComponent(key)}`);
  if (!r.ok) throw new Error('speech');
  return (await r.json()).parts;
}
async function speakCard(btn, card) {
  const same = spk.btn === btn;
  stopSpeak();
  if (same) return;
  const my = ++spk.token, rate = SPEEDS[speedIndex()];
  spk.btn = btn; btn.setAttribute('aria-pressed', 'true'); btn.lastChild.textContent = 'Στοπ';
  const ch = btn.closest('[data-ch]').dataset.ch;
  const nat = typeof naturalVoice === 'function' ? naturalVoice() : null;
  let items = [];
  try {
    for (const key of listenKeys(btn, card)) (await speechParts(ch, key)).forEach((text, k) => items.push({ text, url: `/api/theory-tts?ch=${encodeURIComponent(ch)}&key=${encodeURIComponent(key)}&k=${k}&v=${nat ? nat.slice(3) : ''}` }));
  } catch (e) { items = [{ text: spokenText(card), url: null }]; }
  if (my !== spk.token) return;
  const device = (text, next) => {
    if (!canSpeak) return next();
    const u = new SpeechSynthesisUtterance(text), v = typeof LV !== 'undefined' && LV.voice;
    if (v) { u.voice = v; u.lang = v.lang; } else u.lang = 'el-GR';
    u.rate = 0.95 * rate; u.onend = next; u.onerror = (e) => { if (e.error !== 'interrupted' && e.error !== 'canceled') next(); };
    speechSynthesis.speak(u);
  };
  const play = (i) => {
    if (my !== spk.token) return;
    if (i >= items.length) return stopSpeak();
    const next = () => play(i + 1), it = items[i];
    if (nat && it.url) {
      const a = spk.audio || (spk.audio = new Audio());
      let failed = false;
      const fail = () => { if (failed || my !== spk.token) return; failed = true; a.onended = a.onerror = null; device(it.text, next); };
      a.onended = () => { if (my === spk.token) next(); };
      a.onerror = fail; a.src = it.url; a.playbackRate = rate;
      a.play().catch(fail);
      if (items[i + 1] && items[i + 1].url) fetch(items[i + 1].url).catch(() => {}); // have the next sentence ready
    } else device(it.text, next);
  };
  play(0);
}

if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', mountPrefsButton); else mountPrefsButton();
