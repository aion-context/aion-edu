// Capture a REAL interactive lesson (seg 02 B-roll). Resilient: always finalizes
// the video; logs progress + t_ask/t_send (ms into recording) for trimming.
const PW = process.env.PW_CORE || '/home/ops/Project/completeness-engine-pr12/demo/node_modules/playwright-core';
const CHROME = process.env.CHROME_BIN || '/home/ops/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome';
const { chromium } = require(PW);
const fs = require('fs');

const outDir = process.argv[2] || '/home/ops/Project/aion-edu/recordings/lesson';
const base = process.argv[3] || 'http://127.0.0.1:8080';
const WAIT_MS = parseInt(process.argv[4] || '300000', 10);
const W = 1280, H = 720;
const ANSWER = "With up to f Byzantine faults you need n ≥ 3f+1, so three generals and one traitor is impossible — but four can agree.";
const sleep = ms => new Promise(r => setTimeout(r, ms));

(async () => {
  fs.mkdirSync(outDir, { recursive: true });
  const b = await chromium.launch({ executablePath: CHROME, args: ['--no-sandbox', '--disable-dev-shm-usage'] });
  const ctx = await b.newContext({ viewport: { width: W, height: H }, recordVideo: { dir: outDir, size: { width: W, height: H } }, reducedMotion: 'no-preference' });
  const t0 = Date.now();
  const page = await ctx.newPage();
  page.on('pageerror', e => console.log('PAGEERR:', e.message));
  page.on('console', m => { if (m.type() === 'error') console.log('CONSOLE-ERR:', m.text()); });
  page.on('response', r => { if (r.url().includes('/api/session')) console.log('  RESP', r.status(), r.url().split(base)[1]); });
  page.on('requestfailed', r => { if (r.url().includes('/api/')) console.log('  REQFAIL', r.failure() && r.failure().errorText, r.url().split(base)[1]); });
  let tAsk = -1, tSend = -1;

  try {
    await page.goto(base + '/learn', { waitUntil: 'load' });
    await page.fill('#learner', 'alice');
    await page.fill('#target', 'cs440-u1-l1');
    await page.click('#enroll');
    await page.waitForSelector('#student:not(.hidden)', { timeout: 8000 });
    await page.click('#go');
    console.log('lesson started; waiting up to', WAIT_MS / 1000, 's for the professor to ask…');

    // progress watcher
    const watch = setInterval(async () => {
      try {
        const st = await page.evaluate(() => ({
          n: document.querySelectorAll('#feed .ev').length,
          last: (document.querySelector('#feed .ev:last-child') || {}).className || '',
          asking: document.querySelector('#reply') ? document.querySelector('#reply').classList.contains('on') : false,
        }));
        console.log(`  ${Math.round((Date.now() - t0) / 1000)}s · feed=${st.n} last=${st.last.replace('ev ', '')} asking=${st.asking}`);
      } catch (_) {}
    }, 6000);

    try {
      await page.waitForSelector('#reply.on', { timeout: WAIT_MS });
      tAsk = Date.now() - t0;
      console.log('t_ask_ms', tAsk);
      await sleep(1200);
      await page.click('#replyText');
      await page.type('#replyText', ANSWER, { delay: 38 });
      await sleep(500);
      await page.click('#send');
      tSend = Date.now() - t0;
      console.log('t_send_ms', tSend);
      await sleep(6000);
    } catch (e) {
      console.log('NOTE: no question within window (', e.message, ') — saving what streamed.');
    }
    clearInterval(watch);
  } finally {
    await ctx.close();   // finalizes the webm
    await b.close();
  }
  const f = fs.readdirSync(outDir).filter(x => x.endsWith('.webm')).sort();
  console.log('recorded', f[f.length - 1], '| t_ask_ms', tAsk, '| t_send_ms', tSend);
})().catch(e => { console.error('ERR', e.message); process.exit(1); });
