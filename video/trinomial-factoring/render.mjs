// Renders index.html frame by frame (1920×1080, 30 fps, 72 s) and encodes an MP4 with ffmpeg.
//   node render.mjs                 → public/video/trinomial/paragontopoiisi-trionymou.mp4
//   node render.mjs --stills 4,12.5 → PNG stills at those seconds (for checking), into ./stills
// Needs: playwright (with Chromium) and ffmpeg on PATH. FRAMES_DIR overrides the temp frame folder.
import { createRequire } from 'node:module';
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const { chromium } = createRequire(import.meta.url)('playwright');   // require() also finds a global install (NODE_PATH)
const here = dirname(fileURLToPath(import.meta.url));
const FPS = 30, DURATION = 72, TOTAL = FPS * DURATION, WORKERS = 4;
const url = pathToFileURL(join(here, 'index.html')).href + '?render';
const out = join(here, '../../public/video/trinomial/paragontopoiisi-trionymou.mp4');   // served by the site

const browser = await chromium.launch();
const open = async () => {
  const page = await browser.newPage({ viewport: { width: 1920, height: 1080 }, deviceScaleFactor: 1 });
  await page.goto(url);
  await page.evaluate(() => window.ready);
  return page;
};

const stillsAt = process.argv.indexOf('--stills');
if (stillsAt > 0) {
  const dir = process.env.STILLS_DIR || join(here, 'stills');
  mkdirSync(dir, { recursive: true });
  const page = await open();
  for (const s of process.argv[stillsAt + 1].split(',')) {
    await page.evaluate(t => window.renderAt(t), Number(s));
    await page.screenshot({ path: join(dir, `t${Number(s).toFixed(2).padStart(5, '0')}.png`) });
  }
  await browser.close();
  process.exit(0);
}

const frames = process.env.FRAMES_DIR || join(tmpdir(), 'trinomial-frames');
rmSync(frames, { recursive: true, force: true });
mkdirSync(frames, { recursive: true });
let next = 0, done = 0;
await Promise.all(Array.from({ length: WORKERS }, async () => {
  const page = await open();
  for (let f = next++; f < TOTAL; f = next++) {
    await page.evaluate(t => window.renderAt(t), f / FPS);
    await page.screenshot({ path: join(frames, `f${String(f).padStart(5, '0')}.png`) });
    if (++done % 300 === 0) console.log(`${done} / ${TOTAL} frames`);
  }
}));
await browser.close();

const r = spawnSync('ffmpeg', ['-y', '-loglevel', 'error', '-framerate', String(FPS), '-i', join(frames, 'f%05d.png'),
  '-c:v', 'libx264', '-preset', 'slow', '-crf', '16', '-pix_fmt', 'yuv420p', '-r', String(FPS),
  '-movflags', '+faststart', out], { stdio: 'inherit' });
if (r.status !== 0) process.exit(r.status ?? 1);
rmSync(frames, { recursive: true, force: true });
console.log('wrote', out);
