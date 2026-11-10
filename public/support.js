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

function openPrefs() {
  let d = document.getElementById('prefs');
  if (!d) { d = document.createElement('dialog'); d.id = 'prefs'; d.className = 'prefs'; document.body.appendChild(d); }
  d.setAttribute('aria-labelledby', 'prefs-t');
  d.innerHTML = `<h2 id="prefs-t">Προσαρμογή</h2>
  <p class="muted">Διάλεξε ό,τι σε βοηθά. Οι επιλογές αποθηκεύονται μόνο σε αυτή τη συσκευή.</p>
  <fieldset><legend>Μέγεθος γραμμάτων</legend>${seg('size', ['Κανονικό', 'Μεγάλο', 'Πολύ μεγάλο'], UIP.size, 'Μέγεθος γραμμάτων')}</fieldset>
  <fieldset><legend>Απόσταση γραμμών</legend>${seg('space', ['Κανονική', 'Μεγαλύτερη'], UIP.space, 'Απόσταση γραμμών')}</fieldset>
  <label class="chk"><input type="checkbox" id="p-calm"${UIP.calm ? ' checked' : ''}><span><b>Ήρεμη λειτουργία</b><br><span class="muted">Κρύβει πόντους, σερί και σήματα και σταματά τα εφέ. Μένει μόνο η εργασία.</span></span></label>
  <label class="chk"><input type="checkbox" id="p-one"${UIP.one ? ' checked' : ''}><span><b>Θεωρία: μία κάρτα τη φορά</b><br><span class="muted">Η επόμενη κάρτα εμφανίζεται όταν πατήσεις «Επόμενο».</span></span></label>
  <fieldset><legend>Υπενθύμιση για διάλειμμα</legend>${seg('brk', ['Όχι', 'Κάθε 10΄', 'Κάθε 15΄', 'Κάθε 20΄'], [0, 10, 15, 20].indexOf(UIP.breakMin), 'Υπενθύμιση για διάλειμμα')}</fieldset>
  <div class="row"><button class="btn" id="p-close">Έτοιμο</button></div>`;
  const get = (n) => +d.querySelector(`input[name="${n}"]:checked`).value;
  d.onchange = () => {
    UIP.size = get('size'); UIP.space = get('space'); UIP.breakMin = [0, 10, 15, 20][get('brk')];
    UIP.calm = d.querySelector('#p-calm').checked; UIP.one = d.querySelector('#p-one').checked;
    saveUI(); applyUI(); brkReset();
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

/* ---------- read-aloud (the device's own voice) ---------- */
const canSpeak = 'speechSynthesis' in window && 'SpeechSynthesisUtterance' in window;
const SPOKEN = [[/≤/g, ' μικρότερο ή ίσο του '], [/≥/g, ' μεγαλύτερο ή ίσο του '], [/≠/g, ' διάφορο του '], [/∉/g, ' δεν ανήκει στο '], [/∈/g, ' ανήκει στο '], [/⊆/g, ' υποσύνολο του '], [/∪/g, ' ένωση '], [/∩/g, ' τομή '], [/∅/g, ' κενό σύνολο '],
  [/²/g, ' στο τετράγωνο'], [/³/g, ' στον κύβο'], [/√/g, ' τετραγωνική ρίζα του '], [/π/g, ' πι '], [/[−–]/g, ' μείον '], [/\+/g, ' συν '], [/[·×]/g, ' επί '], [/÷/g, ' διά '], [/=/g, ' ίσον '], [/\s+/g, ' ']];
function spokenText(el) {
  let t = [...el.querySelectorAll('li:not([hidden]), p')].map((x) => x.innerText).join('. ') || el.innerText;
  for (const [re, to] of SPOKEN) t = t.replace(re, to);
  return t;
}
let speakingBtn = null;
function stopSpeak() {
  if (!canSpeak) return;
  speechSynthesis.cancel();
  if (speakingBtn) { speakingBtn.setAttribute('aria-pressed', 'false'); speakingBtn.lastChild.textContent = 'Άκουσε'; speakingBtn = null; }
}
function speakCard(btn, el) {
  const same = speakingBtn === btn;
  stopSpeak();
  if (same) return;
  const u = new SpeechSynthesisUtterance(spokenText(el));
  const v = speechSynthesis.getVoices().find((x) => x.lang.toLowerCase().startsWith('el'));
  if (v) u.voice = v;
  u.lang = 'el-GR'; u.rate = 0.9;
  u.onend = u.onerror = () => { if (speakingBtn === btn) stopSpeak(); };
  speakingBtn = btn; btn.setAttribute('aria-pressed', 'true'); btn.lastChild.textContent = 'Στοπ';
  speechSynthesis.speak(u);
}

document.addEventListener('DOMContentLoaded', mountPrefsButton);
