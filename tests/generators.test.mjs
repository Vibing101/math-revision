// Runs every question generator many times and checks that each question
// has valid answers, 4 hints, and that the feedback code never throws.
import { readFileSync } from 'node:fs';
const html = readFileSync(new URL('../public/index.html', import.meta.url), 'utf8');
const code = html.slice(html.indexOf('/* ---------- maths helpers'), html.indexOf('/* ---------- state'));
const GEN = new Function(code + '; return GEN;')();
let n = 0;
for (const [cat, gen] of Object.entries(GEN)) {
  for (let i = 0; i < 2000; i++) {
    const q = gen();
    const parts = q.groups.flatMap(g => g.parts);
    for (const p of parts) {
      if (p.answer === undefined) throw new Error(`${cat}: missing answer`);
      if (typeof p.answer === 'number' && (!Number.isInteger(p.answer) || p.answer < 0)) throw new Error(`${cat}: bad answer ${p.answer}`);
    }
    if (q.hints.length !== 4) throw new Error(`${cat}: expected 4 hints`);
    q.diagnose(parts.map(p => p.type === 'select' ? String(p.answer) : p.answer), parts);
    q.diagnose(parts.map(p => ({ num: 7, select: '3', digits: [1], exp: { 2: 1 } })[p.type] ?? true), parts);
    n++;
  }
}
console.log(`OK: ${n} generated questions checked`);
