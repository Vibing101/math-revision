// Makes a copy of the video with the Greek voice-over mixed in: paragontopoiisi-trionymou-voice.mp4.
//   node voiceover.mjs            (VOICE=e Ελένη (default), m Νέστορας, f Αθηνά; SITE=… for another host)
// The narration clips come from the site's own /api/video-tts (the speech keys live only in the Worker),
// so this works once that endpoint is deployed. Each line starts 0.3 s into its subtitle window,
// exactly as on the page /video/trinomial/. Needs ffmpeg/ffprobe on PATH.
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const site = (process.env.SITE || 'https://maths.savvy-des.com').replace(/\/$/, ''), v = ['e', 'm', 'f'].includes(process.env.VOICE) ? process.env.VOICE : 'e';
const LEAD = 0.3, WINDOWS = [[0, 8], [8, 17], [17, 22], [22, 27], [27, 38], [38, 46], [46, 56], [56, 65], [65, 72]];

const tmp = mkdtempSync(join(tmpdir(), 'voice-'));
const clips = [];
for (const [i, [a, b]] of WINDOWS.entries()) {
  const res = await fetch(`${site}/api/video-tts?id=trinomial&line=${i}&v=${v}`);
  if (!res.ok) { console.error(`line ${i + 1}: the site answered ${res.status}`); process.exit(1); }
  const file = join(tmp, `l${i}.mp3`);
  writeFileSync(file, Buffer.from(await res.arrayBuffer()));
  const dur = parseFloat(spawnSync('ffprobe', ['-v', 'error', '-show_entries', 'format=duration', '-of', 'csv=p=0', file], { encoding: 'utf8' }).stdout);
  const end = a + LEAD + dur;
  console.log(`line ${i + 1}: ${(a + LEAD).toFixed(1)} s → ${end.toFixed(2)} s (window ends ${b} s)${end > b ? '  ← OVERRUNS' : ''}`);
  if (end > b) { console.error('a line is longer than its subtitle window; shorten it in VIDEO_LINES (crates/site/src/lib.rs)'); process.exit(1); }
  clips.push({ file, at: a + LEAD });
}

const silent = join(here, '../../public/video/trinomial/paragontopoiisi-trionymou.mp4'), out = join(here, 'paragontopoiisi-trionymou-voice.mp4');
const filter = clips.map((c, i) => `[${i + 1}:a]adelay=${Math.round(c.at * 1000)}:all=1[a${i}]`).join(';') +
  `;${clips.map((_, i) => `[a${i}]`).join('')}amix=inputs=${clips.length}:normalize=0,apad,atrim=0:72[mix]`;
const r = spawnSync('ffmpeg', ['-y', '-loglevel', 'error', '-i', silent, ...clips.flatMap(c => ['-i', c.file]),
  '-filter_complex', filter, '-map', '0:v', '-map', '[mix]', '-c:v', 'copy', '-c:a', 'aac', '-b:a', '160k', '-ac', '2',
  '-t', '72', '-movflags', '+faststart', out], { stdio: 'inherit' });
rmSync(tmp, { recursive: true, force: true });
if (r.status !== 0) process.exit(r.status ?? 1);
console.log('wrote', out);
