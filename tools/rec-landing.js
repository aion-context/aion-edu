// Record the Act I landing film (silent video) timed to the narration clip
// lengths. Usage: node tools/rec-landing.js <durations.json> <out.webm> [url]
// Audio is muxed on afterward with ffmpeg.
const PW = process.env.PW_CORE || '/home/ops/Project/completeness-engine-pr12/demo/node_modules/playwright-core';
const CHROME = process.env.CHROME_BIN || '/home/ops/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome';
const { chromium } = require(PW);
const fs = require('fs');

const durFile = process.argv[2] || '/tmp/act1-durations.json';
const outDir = process.argv[3] || '/home/ops/Project/aion-edu/recordings';
const url = process.argv[4] || 'http://127.0.0.1:8080/';
const D = JSON.parse(fs.readFileSync(durFile, 'utf8')); // [hero, s1, s2, s3, s4, cta]
const W = 1280, H = 720;

const sleep = ms => new Promise(r => setTimeout(r, ms));

(async () => {
  const b = await chromium.launch({ executablePath: CHROME, args: ['--no-sandbox', '--disable-dev-shm-usage', '--force-prefers-reduced-motion=false'] });
  const ctx = await b.newContext({ viewport: { width: W, height: H }, recordVideo: { dir: outDir, size: { width: W, height: H } }, reducedMotion: 'no-preference' });
  const page = await ctx.newPage();
  await page.goto(url, { waitUntil: 'load' });

  async function focus(sel, addReveal) {
    await page.evaluate(({ sel, addReveal }) => {
      const el = document.querySelector(sel);
      if (!el) return;
      document.querySelectorAll('.speaking').forEach(e => e.classList.remove('speaking'));
      el.classList.add('speaking');
      if (addReveal) el.querySelectorAll('.reveal').forEach(r => r.classList.add('in'));
      el.scrollIntoView({ behavior: 'smooth', block: el.classList.contains('hero') ? 'start' : 'center' });
    }, { sel, addReveal });
  }

  // BEAT 0 — hero (the seal strikes in during the first ~2.4s of this hold)
  await focus('.hero', false);
  await sleep(D[0] * 1000);

  // BEATS 1..N — every scene, in order (count is dynamic). Tall scenes are panned
  // top→bottom over the hold so every card is seen; short scenes hold centered.
  const sceneCount = (await page.$$('.scene')).length;
  for (let i = 0; i < sceneCount; i++) {
    await page.evaluate((idx) => {
      const el = document.querySelectorAll('.scene')[idx];
      document.querySelectorAll('.speaking').forEach(e => e.classList.remove('speaking'));
      el.classList.add('speaking');
      el.querySelectorAll('.reveal').forEach(r => r.classList.add('in'));
    }, i);
    const plan = await page.evaluate((idx) => {
      const el = document.querySelectorAll('.scene')[idx];
      const r = el.getBoundingClientRect();
      return { top: window.scrollY + r.top - 70, overflow: Math.max(0, el.offsetHeight - (window.innerHeight - 110)) };
    }, i);
    const holdMs = D[i + 1] * 1000;
    if (plan.overflow > 40) {
      const steps = 28;
      for (let s = 0; s <= steps; s++) {
        await page.evaluate((y) => window.scrollTo({ top: y, behavior: 'auto' }), plan.top + plan.overflow * (s / steps));
        await sleep(holdMs / (steps + 1));
      }
    } else {
      await page.evaluate((idx) => document.querySelectorAll('.scene')[idx].scrollIntoView({ behavior: 'smooth', block: 'center' }), i);
      await sleep(holdMs);
    }
  }

  // FINAL BEAT — CTA
  await focus('.cta', false);
  await sleep(D[sceneCount + 1] * 1000);

  await ctx.close();
  await b.close();
  const f = fs.readdirSync(outDir).filter(x => x.endsWith('.webm')).sort();
  console.log('recorded:', f[f.length - 1]);
})().catch(e => { console.error('ERR', e.message); process.exit(1); });
