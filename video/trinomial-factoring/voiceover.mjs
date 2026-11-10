// Adds the Greek voice-over to the silent video with an Azure neural voice.
//   AZURE_SPEECH_KEY=… AZURE_SPEECH_REGION=westeurope node voiceover.mjs
// Runs in GitHub Actions (.github/workflows/voiceover.yml), where the key is a repository secret.
// Each line starts 0.3 s into its subtitle window; a line that would overrun is re-spoken a bit faster.
// Needs ffmpeg/ffprobe on PATH. VOICE=f picks Αθηνά instead of Νέστορας.
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const key = (process.env.AZURE_SPEECH_KEY || '').trim(), region = (process.env.AZURE_SPEECH_REGION || 'westeurope').trim();
if (!key) { console.error('AZURE_SPEECH_KEY is not set'); process.exit(1); }
const voice = process.env.VOICE === 'f' ? 'el-GR-AthinaNeural' : 'el-GR-NestorasNeural';
const BASE_RATE = -6, LEAD = 0.3, TAIL = 0.4;          // % speaking rate (as in the lesson player), seconds

// [start, end, what is spoken] — same windows as subtitles.el.srt; maths written the way it is read aloud
const LINES = [
  [0, 8, 'Πώς γράφουμε το χι στο τετράγωνο, συν επτά χι, συν δέκα, σαν γινόμενο; Πάμε βήμα βήμα.'],
  [8, 17, 'Ψάχνουμε δύο αριθμούς: γινόμενο δέκα, και άθροισμα επτά.'],
  [17, 22, 'Ένα και δέκα: άθροισμα έντεκα. Όχι.'],
  [22, 27, 'Δύο και πέντε: άθροισμα επτά. Αυτοί είναι!'],
  [27, 38, 'Όλα τα κομμάτια φτιάχνουν ένα ορθογώνιο. Πλευρές: χι συν δύο, και χι συν πέντε.'],
  [38, 46, 'Άρα: χι στο τετράγωνο, συν επτά χι, συν δέκα, ισούται με: χι συν δύο, επί χι συν πέντε.'],
  [46, 56, 'Έλεγχος: πολλαπλασιάζουμε και ξαναβρίσκουμε την αρχική παράσταση. Σωστό!'],
  [56, 65, 'Προσοχή! Αν ο τελευταίος αριθμός είναι αρνητικός, ο ένας είναι θετικός και ο άλλος αρνητικός.'],
  [65, 72, 'Θυμήσου: γινόμενο, άθροισμα, ζευγάρι, παρενθέσεις. Τώρα δοκίμασε εσύ!'],
];

const esc = s => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
async function speak(text, rate, file) {
  const ssml = `<speak version='1.0' xml:lang='el-GR'><voice name='${voice}'><prosody rate='${rate >= 0 ? '+' : ''}${rate}%'>${esc(text)}</prosody></voice></speak>`;
  const res = await fetch(`https://${region}.tts.speech.microsoft.com/cognitiveservices/v1`, {
    method: 'POST', body: ssml,
    headers: { 'Ocp-Apim-Subscription-Key': key, 'Content-Type': 'application/ssml+xml',
               'X-Microsoft-OutputFormat': 'riff-48khz-16bit-mono-pcm', 'User-Agent': 'voiceover-script' },
  });
  if (!res.ok) throw new Error(`Azure TTS answered ${res.status}`);
  writeFileSync(file, Buffer.from(await res.arrayBuffer()));
  const p = spawnSync('ffprobe', ['-v', 'error', '-show_entries', 'format=duration', '-of', 'csv=p=0', file], { encoding: 'utf8' });
  return parseFloat(p.stdout);
}

const tmp = mkdtempSync(join(tmpdir(), 'voice-'));
const clips = [];
for (const [i, [a, b, text]] of LINES.entries()) {
  const file = join(tmp, `l${i}.wav`), room = b - a - LEAD - TAIL;
  let rate = BASE_RATE, dur = await speak(text, rate, file);
  if (dur > room) {                                     // one retry, just fast enough to fit
    rate = Math.ceil((100 + BASE_RATE) * dur / room * 1.03 - 100);
    if (rate > 25) throw new Error(`line ${i + 1} needs +${rate}% to fit ${room.toFixed(1)} s — shorten the text`);
    dur = await speak(text, rate, file);
  }
  if (dur > b - a - LEAD) throw new Error(`line ${i + 1} is ${dur.toFixed(2)} s, window is ${b - a} s`);
  console.log(`line ${i + 1}: ${(a + LEAD).toFixed(1)} s → ${(a + LEAD + dur).toFixed(2)} s (window ends ${b} s, rate ${rate}%)`);
  clips.push({ file, at: a + LEAD });
}

const silent = join(here, 'paragontopoiisi-trionymou.mp4'), out = join(here, 'paragontopoiisi-trionymou-voice.mp4');
const filter = clips.map((c, i) => `[${i + 1}:a]adelay=${Math.round(c.at * 1000)}:all=1[a${i}]`).join(';') +
  `;${clips.map((_, i) => `[a${i}]`).join('')}amix=inputs=${clips.length}:normalize=0,apad,atrim=0:72[mix]`;
const r = spawnSync('ffmpeg', ['-y', '-loglevel', 'error', '-i', silent, ...clips.flatMap(c => ['-i', c.file]),
  '-filter_complex', filter, '-map', '0:v', '-map', '[mix]', '-c:v', 'copy', '-c:a', 'aac', '-b:a', '160k', '-ac', '2',
  '-t', '72', '-movflags', '+faststart', out], { stdio: 'inherit' });
rmSync(tmp, { recursive: true, force: true });
if (r.status !== 0) process.exit(r.status ?? 1);
console.log('wrote', out);
