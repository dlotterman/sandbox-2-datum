// Browser checks for the dashboard and about page. Run by scripts/ui-check.sh, inside the Playwright container.
//
// The real outside-agent binary serves the pages; this script replaces /api/state with a fixed, realistic
// fake so rendering is deterministic and needs no LLM and no inside agent.
const { chromium } = require("playwright");

const PORT = process.env.PORT || "8099";
const BASE = `http://127.0.0.1:${PORT}`;
const SCREENSHOT_DIR = process.env.SCREENSHOT_DIR; // optional: where to save full-page screenshots

const now = Math.floor(Date.now() / 1000);
const THIS_AGENT = "c55c8dccc362f85a16ef6e2679138d415a564868c032b200acef24b1dbbca2cf";

/** A state with one live agent (this one), one stale agent, and one brand-new agent that has not reported yet. */
const fakeState = {
  started: now - 3600, now,
  endpoint_id: THIS_AGENT,
  inside_id: "aa2488b2b8ba3d9d45b9aadb44ddae2c63ea0ee06c909c5aa11f0b14e5cafb59",
  connected: true, connect_ms: 412,
  inside_card: { skills: [{ id: "llm.complete" }, { id: "ip.lookup" }, { id: "agents.sync" }] },
  facts: { hostname: "h1", os: "Debian GNU/Linux 12" },
  public_ips: ["203.0.113.7"],
  ip_info: [{ org: "AS64500 Example ISP", city: "Springfield", region: "Oregon", country: "US", timezone: "America/Los_Angeles" }],
  cycles: 4, last_cycle: now - 120, last_error: null,
  findings: [
    { id: 1, at: now - 300, title: "Where does 203.0.113.7 sit on the internet?", skill: "ip.lookup", state: "completed", answer: "lookup for 203.0.113.7", detail: { city: "Springfield" }, ms: 120 },
    { id: 2, at: now - 290, title: "What do I suspect, and what data do I need?", skill: "llm.complete", state: "completed", answer: '{"hunch":"home machine"}', detail: null, ms: 4100 },
    { id: 3, at: now - 280, title: "Asked inside for file: regions.csv", skill: "files.get", state: "rejected", answer: "denied by review: not relevant", detail: null, ms: 900 },
    {
      id: 4, at: now - 270, title: "Hosting guess, refined with inside data", skill: "llm.complete", state: "completed", detail: null, ms: 3000,
      // Includes markup on purpose: it must be shown as text, never executed.
      answer: "**Provider:** Local machine\n**Region:** Springfield\n- `/run/.containerenv` present, matches Podman\n- Note <script>alert(1)</script> stays text",
    },
  ],
  agents: [
    { endpoint_id: THIS_AGENT, first_seen: now - 3600, last_seen: now - 5, requests: 120, active: true,
      report: { platform_hint: "container", provider_guess: "Local machine", ip_org: "AS64500 Example ISP", ip_city: "Springfield", ip_region: "Oregon", public_ip: "203.0.113.7", hostname: "h1", os: "Debian 12", cycles: 4, hosting_guess: "**Provider:** Local machine" } },
    { endpoint_id: "ce431bae891b289357a11879ab17ec393f73fb93dfa9817e1ab29af1b01f98f0", first_seen: now - 7200, last_seen: now - 900, requests: 55, active: false,
      report: { platform_hint: "Fly.io lax", ip_org: "AS60068 Datacamp", ip_city: "Los Angeles", public_ip: "198.51.100.4", hostname: "83e646c71d91d8" } },
    { endpoint_id: "583c0d1bb054fa15405355ceb62f2cf2a41727fd6f5987abf91c8e26282ad9b3", first_seen: now - 60, last_seen: now - 40, requests: 3, active: true, report: null },
  ],
  agents_synced: now - 8, agents_error: null, agents_window_secs: 21600,
};

let failures = 0;
function check(name, passed) {
  console.log(`${passed ? "PASS" : "FAIL"} ${name}`);
  if (!passed) failures++;
}

/** Open a page with /api/state faked, and collect anything it logs as an error. */
async function openPage(browser, scheme, path) {
  const page = await browser.newPage({ viewport: { width: 1100, height: 900 }, colorScheme: scheme });
  page.errors = [];
  page.on("pageerror", (error) => page.errors.push(String(error)));
  page.on("console", (message) => message.type() === "error" && page.errors.push(message.text()));
  await page.route("**/api/state", (route) => route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify(fakeState) }));
  await page.goto(BASE + path);
  return page;
}

async function checkDashboard(browser, scheme) {
  const label = `[${scheme}] dashboard`;
  const page = await openPage(browser, scheme, "/");
  await page.waitForSelector("#agents table", { timeout: 15000 });
  const count = (selector) => page.locator(selector).count();
  const text = (selector) => page.locator(selector).innerText();

  check(`${label}: no console or page errors ${JSON.stringify(page.errors)}`, page.errors.length === 0);
  check(`${label}: four stat cards`, (await count("#stats .stat")) === 4);
  check(`${label}: the final answer is featured`, (await text("#guess .finding.feature h3")).startsWith("Hosting guess"));
  check(`${label}: **bold** becomes <strong>`, (await count("#guess strong")) >= 2);
  check(`${label}: markup from the model stays text`, (await count("#guess script")) === 0 && (await text("#guess")).includes("<script>alert(1)</script>"));
  check(`${label}: three findings listed, the featured one excluded`, (await count("#findings .finding")) === 3);
  check(`${label}: a rejected finding has a rejected chip`, (await count("#findings .chip.rejected")) === 1);
  check(`${label}: three agent rows`, (await count("#agents tbody tr")) === 3);
  check(`${label}: this agent is highlighted`, (await count("#agents tr.me")) === 1);
  check(`${label}: active agents show a live chip`, (await count("#agents .chip.live")) === 2);
  check(`${label}: a stale agent shows how long ago`, (await text("#agents tbody tr:nth-child(2)")).includes("15m ago"));
  check(`${label}: an agent with no report still renders`, (await text("#agents tbody tr:nth-child(3)")).includes("no public IP seen"));
  check(`${label}: the registry note names the window`, (await text("#agents-note")).includes("last 6 hours"));
  check(`${label}: the footer shows this agent's id`, (await page.locator("#foot dd").first().innerText()).startsWith("c55c8dcc"));
  if (SCREENSHOT_DIR) await page.screenshot({ path: `${SCREENSHOT_DIR}/dashboard-${scheme}.png`, fullPage: true });
  await page.close();
}

async function checkAbout(browser, scheme) {
  const label = `[${scheme}] about`;
  const page = await openPage(browser, scheme, "/about");
  await page.waitForSelector("#ids dd", { timeout: 15000 });
  const ids = await page.locator("#ids").innerText();

  check(`${label}: no console or page errors ${JSON.stringify(page.errors)}`, page.errors.length === 0);
  check(`${label}: both ids and the offered skills are shown`, ids.includes(THIS_AGENT) && ids.includes("llm.complete"));
  if (SCREENSHOT_DIR) await page.screenshot({ path: `${SCREENSHOT_DIR}/about-${scheme}.png`, fullPage: true });
  await page.close();
}

(async () => {
  const browser = await chromium.launch();
  for (const scheme of ["light", "dark"]) {
    await checkDashboard(browser, scheme);
    await checkAbout(browser, scheme);
  }
  await browser.close();
  console.log(failures === 0 ? "ALL PASSED" : `${failures} FAILED`);
  process.exit(failures === 0 ? 0 : 1);
})();
