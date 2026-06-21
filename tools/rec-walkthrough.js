// Record the Act II walkthrough (silent video) by driving the real UI, each
// segment held to its narration clip length. Audio muxed afterward with ffmpeg.
// Pre-reqs: server on :8080, federation set, alice's credential minted.
const PW = process.env.PW_CORE || '/home/ops/Project/completeness-engine-pr12/demo/node_modules/playwright-core';
const CHROME = process.env.CHROME_BIN || '/home/ops/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome';
const { chromium } = require(PW);
const fs = require('fs');

const outDir = process.argv[2] || '/home/ops/Project/aion-edu/recordings';
const base = process.argv[3] || 'http://127.0.0.1:8080';
const W = 1280, H = 720;
const D = { enroll: 17.40, lesson: 16.85, credential: 14.03, wallet: 7.97, present: 18.83, verify: 15.57, federation: 21.08, close: 9.87 };
const sleep = ms => new Promise(r => setTimeout(r, ms));

(async () => {
  const b = await chromium.launch({ executablePath: CHROME, args: ['--no-sandbox', '--disable-dev-shm-usage'] });
  const ctx = await b.newContext({ viewport: { width: W, height: H }, recordVideo: { dir: outDir, size: { width: W, height: H } }, reducedMotion: 'no-preference' });
  const page = await ctx.newPage();

  async function seg(name, fn) {
    const t0 = Date.now();
    try { await fn(); } catch (e) { console.log('  (action note ' + name + ': ' + e.message + ')'); }
    const remain = D[name] * 1000 - (Date.now() - t0);
    if (remain > 0) await sleep(remain); else console.log('  over ' + name + ' by ' + (-remain) + 'ms');
    console.log('✓ ' + name);
  }
  const scrollTo = sel => page.evaluate(s => { const e = document.querySelector(s); if (e) e.scrollIntoView({ behavior: 'smooth', block: 'center' }); }, sel);

  // 01 — enroll
  await seg('enroll', async () => {
    await page.goto(base + '/learn', { waitUntil: 'load' });
    await page.fill('#learner', 'alice');
    await page.fill('#target', 'cs440-u1-l1');
    await sleep(700);
    await page.click('#enroll');
    await page.waitForSelector('#student:not(.hidden)', { timeout: 6000 });
    await scrollTo('#student');
  });

  // 02 — lesson (classroom + path; real lesson B-roll inserted in edit)
  await seg('lesson', async () => { await scrollTo('#pathPanel'); });

  // 03 — credential (the sealed credential in the wallet)
  await seg('credential', async () => { await scrollTo('#transcript'); });

  // 04 — wallet
  await seg('wallet', async () => { await scrollTo('#transcript'); });

  // 05 — present: partner-u ACCEPTED, then stranger-u REJECTED
  await seg('present', async () => {
    await page.fill('.pinp', 'partner-u');
    await page.click('.pbtn');
    await sleep(2200);
    await page.fill('.pinp', 'stranger-u');
    await page.click('.pbtn');
  });

  // 06 — verify offline (wallet button → AUTHENTIC)
  await seg('verify', async () => {
    await page.click('.vbtn');
  });

  // 07 — federation: declare + endorse x2 + snapshot
  await seg('federation', async () => {
    await page.goto(base + '/federate', { waitUntil: 'load' });
    await sleep(900);
    await page.click('button:has-text("Declare")'); await sleep(1300);
    await page.click('button:has-text("Endorse")'); await sleep(1200);
    await page.fill('#e_by', 'partner-u');
    await page.click('button:has-text("Endorse")'); await sleep(1400);
    await page.click('button:has-text("Snapshot")');
  });

  // 08 — close: the seal
  await seg('close', async () => {
    await page.goto(base + '/', { waitUntil: 'load' });
  });

  await ctx.close();
  await b.close();
  const f = fs.readdirSync(outDir).filter(x => x.endsWith('.webm')).sort();
  console.log('recorded:', f[f.length - 1]);
})().catch(e => { console.error('ERR', e.message); process.exit(1); });
