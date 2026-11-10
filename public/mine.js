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

/* ---------- tips for pupils: one reminder per mistake type (the teacher panel keeps its own wording) ---------- */
const PUPIL_TIPS = {
  'ident-middle': 'Στο (α + β)² υπάρχει και ο μεσαίος όρος 2αβ.',
  'root-sign': 'Αν ο παράγοντας είναι (x − 3), η ρίζα είναι x = 3: το πρόσημο αλλάζει.',
  'lost-root': 'Αν διαιρέσεις με x, μπορεί να χάσεις τη λύση x = 0. Καλύτερα βγάλε κοινό παράγοντα.',
  'factor': 'Βρες πρώτα τον κοινό παράγοντα και μετά δες αν ταιριάζει κάποια ταυτότητα.',
  'factor-method': 'Πριν παραγοντοποιήσεις, διάλεξε μέθοδο: κοινός παράγοντας, ταυτότητα ή τριώνυμο.',
  'eq-product-const': 'Μηδενίζουμε τους παράγοντες μόνο όταν το γινόμενο είναι 0. Αν είναι άλλος αριθμός, πρώτα κάνε τις πράξεις.',
  'eq-squares': 'Από Α² = Β² προκύπτει Α = Β ή Α = −Β.',
  'eq-kind': 'Έλεγξε αν η εξίσωση έχει λύση: το τετράγωνο ενός αριθμού δεν είναι ποτέ αρνητικό.',
  'eq-split': 'Όταν η ρίζα είναι κλάσμα, σπάσε τον μεσαίο όρο και παραγοντοποίησε με ομαδοποίηση.',
  'rational-domain': 'Ο παρονομαστής δεν μπορεί να είναι 0. Βρες πρώτα ποιες τιμές του x απαγορεύονται.',
  'rational-simplify': 'Απλοποιούμε μόνο παράγοντες, όχι όρους που προστίθενται. Παραγοντοποίησε πρώτα.',
  'tri-ineq': 'Σε κάθε τρίγωνο, κάθε πλευρά είναι μικρότερη από το άθροισμα των άλλων δύο.',
  'congr': 'Για ίσα τρίγωνα ψάξε Π-Π-Π, Π-Γ-Π ή Γ-Π-Γ, με τα ίσα στοιχεία στις σωστές θέσεις.',
  'trig-sides': 'Η «απέναντι» και η «προσκείμενη» πλευρά εξαρτώνται από τη γωνία που κοιτάς. Σημείωσε πρώτα τη γωνία.',
  'trig-monotone': 'Από 0° έως 90°, όσο μεγαλώνει η γωνία, μεγαλώνουν το ημίτονο και η εφαπτομένη και μικραίνει το συνημίτονο.',
  'surd-simplify': 'Απλοποίησε τη ρίζα βγάζοντας τα τέλεια τετράγωνα, π.χ. √12 = 2√3.',
  'dist-sum': 'Απόσταση: αφαίρεσε τις συντεταγμένες, ύψωσε στο τετράγωνο, πρόσθεσε και πάρε τετραγωνική ρίζα.',
  'midpoint': 'Το μέσο βρίσκεται προσθέτοντας τις συντεταγμένες και διαιρώντας με το 2, όχι αφαιρώντας.',
  'slope-swap': 'Κλίση = μεταβολή του y διά μεταβολή του x. Το y είναι από πάνω.',
  'perp': 'Οι κάθετες ευθείες έχουν κλίσεις με γινόμενο −1: αντίστροφες και με αντίθετο πρόσημο.',
  'line-pos': 'Ίδια κλίση: παράλληλες (ή η ίδια ευθεία). Διαφορετική κλίση: τέμνονται.',
  'sys-swap': 'Γράψε τη λύση ως (x, y) με αυτή τη σειρά και έλεγξέ τη και στις δύο εξισώσεις.',
  'sample-bias': 'Ένα δείγμα είναι αντιπροσωπευτικό όταν κάθε μέλος του πληθυσμού έχει ίση πιθανότητα να επιλεγεί.',
  'median-sort': 'Για τη διάμεσο, γράψε πρώτα τις τιμές από τη μικρότερη στη μεγαλύτερη.',
  'cone-third': 'Ο όγκος κώνου και πυραμίδας έχει το 1/3: V = ⅓ · (εμβαδόν βάσης) · ύψος.',
  'solid-bases': 'Ο κύλινδρος έχει δύο βάσεις. Στο ολικό εμβαδόν πρόσθεσε και τις δύο.',
  'parab-shape': 'Στην y = αx²: αν α > 0 ανοίγει προς τα πάνω και έχει ελάχιστο, αν α < 0 ανοίγει προς τα κάτω και έχει μέγιστο.',
  'rhombus-area': 'Εμβαδόν ρόμβου = (γινόμενο των διαγωνίων) : 2. Μην ξεχνάς το μισό.',
  'quad-props': 'Μάθε ποιες ιδιότητες έχει το παραλληλόγραμμο, το ορθογώνιο, ο ρόμβος και το τετράγωνο.',
  'seq-off': 'Για να φτάσεις στον ν-οστό όρο κάνεις ν − 1 βήματα, όχι ν.',
  'seq-formula': 'Βρες πρώτα τον κανόνα της ακολουθίας και δοκίμασε τον γενικό τύπο στους πρώτους όρους.',
  'units': 'Πριν υπολογίσεις, κάνε όλες τις μονάδες ίδιες και έλεγξε τον συντελεστή μετατροπής.',
  'angle-kind': 'Οξεία: μικρότερη από 90°. Ορθή: 90°. Αμβλεία: μεταξύ 90° και 180°. Ευθεία: 180°.',
  'comp-supp': 'Συμπληρωματικές: άθροισμα 90°. Παραπληρωματικές: άθροισμα 180°.',
  'circle-360': 'Ολόκληρος ο κύκλος έχει 360°.',
  'circle-terms': 'Η διάμετρος είναι η μεγαλύτερη χορδή και περνά από το κέντρο. Η επίκεντρη γωνία έχει κορυφή το κέντρο.',
  'vec-kind': 'Ένα διάνυσμα έχει μέτρο, διεύθυνση και φορά. Ίσα διανύσματα έχουν και τα τρία ίδια.',
  'vec-dir': 'Αντίθετα διανύσματα έχουν ίδιο μέτρο και διεύθυνση, αλλά αντίθετη φορά.',
  'coord-swap': 'Στο σημείο (x, y) πρώτα γράφουμε το x (οριζόντια) και μετά το y (κάθετα).',
  'quadrant': 'Δες τα πρόσημα: Ι τεταρτημόριο (+, +), ΙΙ (−, +), ΙΙΙ (−, −), ΙV (+, −).',
  'fn-def': 'Σε μια συνάρτηση, κάθε τιμή του x αντιστοιχεί σε μία μόνο τιμή του y.',
  'par-angle': 'Οι εντός εναλλάξ και οι εντός εκτός και επί τα αυτά γωνίες είναι ίσες. Οι εντός και επί τα αυτά έχουν άθροισμα 180°.',
  'tri-180': 'Σε κάθε τρίγωνο οι τρεις γωνίες έχουν άθροισμα 180°.',
  'tri-ext': 'Η εξωτερική γωνία τριγώνου ισούται με το άθροισμα των δύο απέναντι εσωτερικών γωνιών.',
  'tri-kind': 'Δες τις πλευρές (σκαληνό, ισοσκελές, ισόπλευρο) και τις γωνίες (οξυγώνιο, ορθογώνιο, αμβλυγώνιο) χωριστά.',
  'tri-lines': 'Διάμεσος: από κορυφή στο μέσο της απέναντι πλευράς. Ύψος: κάθετο στην απέναντι πλευρά. Διχοτόμος: χωρίζει τη γωνία στη μέση.',
  'ratio-order': 'Στον λόγο α : β έχει σημασία η σειρά: πρώτος όρος είναι αυτός που αναφέρεται πρώτος.',
  'pct-step': 'Σε αύξηση ή έκπτωση κάνε και το τελευταίο βήμα: πρόσθεσε ή αφαίρεσε το ποσό από την αρχική τιμή.',
  'pct-base': 'Υπολόγισε το ποσοστό πάνω στη σωστή τιμή: αυτή που η εκφώνηση θεωρεί 100%.',
  'stat-var': 'Ποιοτική μεταβλητή: λέξεις ή κατηγορίες. Ποσοτική: αριθμοί.',
  'pie-360': 'Στο κυκλικό διάγραμμα ολόκληρος ο κύκλος, 360°, αντιστοιχεί στο 100%.',
  'prob-not': 'P(να μην συμβεί) = 1 − P(να συμβεί).',
  'prob-def': 'Η πιθανότητα είναι αριθμός από 0 μέχρι 1: το 0 σημαίνει αδύνατο και το 1 βέβαιο.',
  'set-elem': 'Έλεγξε κάθε στοιχείο: ανήκει (∈) στο σύνολο μόνο αν έχει την ιδιότητά του.',
  'set-dup': 'Σε ένα σύνολο γράφουμε και μετράμε κάθε στοιχείο μία μόνο φορά.',
  'set-sub': 'Το Β είναι υποσύνολο του Α όταν ΟΛΑ τα στοιχεία του Β ανήκουν στο Α. Ίσα σύνολα έχουν ακριβώς τα ίδια στοιχεία.',
  'set-double': 'ν(Α ∪ Β) = ν(Α) + ν(Β) − ν(Α ∩ Β). Τα κοινά στοιχεία μετρήθηκαν δύο φορές, γι’ αυτό αφαιρούνται μία.',
  'set-empty': 'Το ∅ σημαίνει «κανένα στοιχείο». Δεν το γράφουμε ως στοιχείο μέσα σε ένα σύνολο.',
  'set-union-inter': 'Ένωση (∪): στοιχεία που ανήκουν στο ένα Ή στο άλλο. Τομή (∩): μόνο όσα ανήκουν και στα δύο.',
  'pow-zero': 'Κάθε αριθμός εκτός από το 0, υψωμένος στο 0, κάνει 1: α⁰ = 1.',
  'order': 'Σειρά πράξεων: παρενθέσεις, δυνάμεις, πολλαπλασιασμοί και διαιρέσεις, και στο τέλος προσθέσεις και αφαιρέσεις.',
  'bin-digit': 'Στο δυαδικό σύστημα χρησιμοποιούμε μόνο τα ψηφία 0 και 1.',
  'bin-rev': 'Στο δυαδικό, το τελευταίο ψηφίο δεξιά αξίζει 1, το επόμενο 2, μετά 4, 8… Πρόσεξε τη σειρά των ψηφίων.',
  'distrib': 'α(β + γ) = αβ + αγ. Πολλαπλασίασε τον αριθμό με κάθε όρο της παρένθεσης.',
  'alg-words': 'Μετάφρασε τη φράση κομμάτι-κομμάτι: «διπλάσιο του x» είναι 2x και «κατά 3 περισσότερο» σημαίνει + 3.',
  'val-sq': 'Το x² σημαίνει x · x και όχι 2 · x.',
  'eq-inverse': 'Για να λύσεις, κάνε την αντίστροφη πράξη: το + γίνεται −, και ο πολλαπλασιασμός γίνεται διαίρεση.',
  'abs': 'Η απόλυτη τιμή δεν είναι ποτέ αρνητική: |−5| = 5.',
  'neg-compare': 'Στους αρνητικούς, μεγαλύτερος είναι αυτός με τη μικρότερη απόλυτη τιμή: −2 > −7.',
  'sign': 'Έλεγξε το πρόσημο σε κάθε βήμα και ξανά στο τέλος.',
  'minus-minus': 'Το μείον μπροστά από αρνητικό αριθμό γίνεται συν: − (−α) = + α.',
  'neg-pow': 'Το −2⁴ είναι −16, ενώ το (−2)⁴ είναι 16. Η παρένθεση αλλάζει το αποτέλεσμα.',
  'periodic': 'Στον περιοδικό αριθμό επαναλαμβάνεται η περίοδος. Δες ποια ψηφία έχουν την παύλα από πάνω.',
  'eq-move': 'Όταν μεταφέρεις όρο στο άλλο μέλος της εξίσωσης, αλλάζει πρόσημο.',
  'frac-reduce': 'Απλοποίησε το κλάσμα διαιρώντας αριθμητή και παρονομαστή με τον ΜΚΔ τους.',
  'lcm-min': 'ΕΚΠ: από κάθε πρώτο παράγοντα πάρε τον μεγαλύτερο εκθέτη.',
  'lcm-missing': 'ΕΚΠ: πάρε όλους τους πρώτους παράγοντες, ακόμα κι αυτούς που υπάρχουν μόνο στον έναν αριθμό.',
  'gcd-noncommon': 'ΜΚΔ: πάρε μόνο τους πρώτους παράγοντες που είναι κοινοί και στους δύο αριθμούς.',
  'gcd-max': 'ΜΚΔ: από κάθε κοινό παράγοντα πάρε τον μικρότερο εκθέτη.',
  'pow-mult': 'Η δύναμη 5² σημαίνει 5 · 5 = 25, και όχι 5 · 2.',
  'not-least': 'Το ΕΚΠ είναι το ΜΙΚΡΟΤΕΡΟ κοινό πολλαπλάσιο. Έλεγξε ότι δεν υπάρχει μικρότερο.',
  'not-greatest': 'Ο ΜΚΔ είναι ο ΜΕΓΑΛΥΤΕΡΟΣ κοινός διαιρέτης. Έλεγξε ότι δεν υπάρχει μεγαλύτερος.',
  'gcd-vs-lcm': 'ΜΚΔ: διαιρεί και τους δύο αριθμούς, άρα δεν είναι μεγαλύτερος από αυτούς. ΕΚΠ: διαιρείται και από τους δύο, άρα δεν είναι μικρότερος από αυτούς.',
  'time-remainder': 'Όταν μετατρέπεις ώρες σε μέρες, διαίρεσε με το 24 και κράτα και τις ώρες που περισσεύουν.',
  'time-12': 'Μέχρι τις 12:00 το μεσημέρι είναι π.μ. και μετά τις 12:00 είναι μ.μ.',
  'time-day': 'Μέτρα προσεκτικά τις ημέρες και πρόσεξε αν μετράς και την πρώτη και την τελευταία.',
  'blank': 'Συμπλήρωσε όλα τα κουτάκια πριν πατήσεις «Έλεγχος».',
  'eucl-rem': 'Στην Ευκλείδεια διαίρεση το υπόλοιπο είναι πάντα μικρότερο από τον διαιρέτη.',
  'eucl-week': 'Για τις μέρες της εβδομάδας, βρες το υπόλοιπο της διαίρεσης με το 7.',
  'ceil': 'Στο «πόσα χρειάζονται τουλάχιστον», αν περισσεύει υπόλοιπο χρειάζεσαι ένα ακόμα.',
  'div-trivial': 'Κάθε αριθμός έχει διαιρέτες το 1 και τον ίδιο τον εαυτό του.',
  'div-wrong': 'Έλεγξε ότι ο αριθμός διαιρεί ακριβώς, δηλαδή με υπόλοιπο 0.',
  'div-missing': 'Βρες τους διαιρέτες σε ζεύγη: 1 και ο αριθμός, 2 και ο αριθμός : 2, και συνέχισε.',
  'div-tf': 'Ο διαιρέτης διαιρεί τον αριθμό. Το πολλαπλάσιο προκύπτει όταν πολλαπλασιάσεις τον αριθμό.',
  'prop': 'Αν ένας αριθμός διαιρεί δύο αριθμούς, διαιρεί και το άθροισμα και τη διαφορά τους.',
  'fact-product': 'Μετά την ανάλυση, πολλαπλασίασε τους παράγοντες για να ελέγξεις ότι βγαίνει ο αρχικός αριθμός.',
  'digits-partial': 'Δοκίμασε όλα τα ψηφία από 0 έως 9: μπορεί να υπάρχουν περισσότερες λύσεις.',
  'crit': 'Ξαναδές το κριτήριο: ο 3 και ο 9 με το άθροισμα των ψηφίων, ο 4 με τα δύο τελευταία ψηφία, ο 5 με το τελευταίο ψηφίο.',
  'prime-one': 'Το 1 δεν είναι ούτε πρώτος ούτε σύνθετος αριθμός.',
  'prime-comp': 'Πρώτος έχει μόνο δύο διαιρέτες: το 1 και τον εαυτό του. Σύνθετος έχει περισσότερους.',
  'prime-missed': 'Δοκίμασε τα κριτήρια διαιρετότητας: αν δεν βρεις άλλον διαιρέτη, ο αριθμός είναι πρώτος.',
  'coprime': 'Δύο αριθμοί είναι πρώτοι μεταξύ τους όταν ο ΜΚΔ τους είναι 1.',
  'enrich': 'Μάθε τα κριτήρια για το 7, το 11 και το 19 και εφάρμοσέ τα βήμα βήμα.',
  'range-lcm': 'Στα προβλήματα με όρια, βρες πρώτα το ΕΚΠ και μετά το πολλαπλάσιό του που ταιριάζει στα όρια.',
  'range-rem': 'Όταν υπάρχει υπόλοιπο, πρόσθεσέ το στο πολλαπλάσιο που βρήκες.',
  'years-count': 'Όταν μετράς πόσες φορές χωράει κάτι ολόκληρο, κράτα το ακέραιο μέρος της διαίρεσης και μη στρογγυλεύεις προς τα πάνω.',
};

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
  const topTags = Object.entries(S.tags || {}).filter(([t]) => PUPIL_TIPS[t]).sort((a, b) => b[1] - a[1]).slice(0, 3);
  app.innerHTML = `<div class="top">${backBtn('Αρχική')}${statsChips()}</div>
    <header class="hero"><h1>Τα λάθη μου</h1><p class="muted" style="margin:0">Εδώ μαζεύονται οι ερωτήσεις που δεν βγήκαν. Δεν είναι αποτυχίες: είναι η λίστα με ό,τι αξίζει να ξαναδείς. Όταν λύσεις μια ερώτηση, φεύγει από τη λίστα.</p></header>
    ${live.length > 1 ? `<div class="row"><button class="btn big-btn" id="mk-all" type="button">Επανάληψη όλων (${Math.min(live.length, 10)})${I('next')}</button></div>` : ''}
    ${live.length ? `<ul class="mklist">${items}</ul>` : '<section class="card"><p style="margin:0">Δεν έχεις ερωτήσεις για επανάληψη αυτή τη στιγμή. Συνέχισε την εξάσκηση: αν κάτι δεν βγει, θα εμφανιστεί εδώ.</p></section>'}
    ${topTags.length ? `<section class="card"><h2 style="margin-bottom:8px">Τι να προσέχεις</h2><p class="muted" style="margin:0 0 8px">Αυτές οι υπενθυμίσεις σού χρειάστηκαν πιο συχνά:</p><ul style="margin:0;padding-left:1.2em">${topTags.map(([t]) => `<li>${esc(PUPIL_TIPS[t])}</li>`).join('')}</ul></section>` : ''}`;
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
