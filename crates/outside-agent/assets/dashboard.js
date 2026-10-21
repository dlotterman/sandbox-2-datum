// The dashboard. Every few seconds it fetches /api/state and redraws each section of the page.
//
// Fields of /api/state used here (see `api_state` in web.rs):
//   endpoint_id, inside_id                 this agent's and the inside agent's ids
//   connected, connect_ms, last_error      the link to the inside agent
//   public_ips, ip_info                    what Iroh learned of our public address, and its lookup
//   cycles, last_cycle, started            how long we have been running
//   findings                               every exchange with the inside agent (see Finding in state.rs)
//   agents, agents_synced, agents_error,   the registry of agents the inside agent has seen
//     agents_window_secs
//   facts                                  everything this process could see locally

const REFRESH_EVERY_MS = 10_000;
const REFRESH_BUTTON_LABEL_MS = 3_000;
const DEFAULT_REGISTRY_WINDOW_SECS = 6 * 3600;

// The outside agent titles its final answer "Hosting guess, ..." (ANSWER_TITLE in investigation.rs).
const FINAL_ANSWER_TITLE_PREFIX = "Hosting guess";

/** Fetch the current state, or return null (and mark the link down) if the agent itself is unreachable. */
async function fetchState() {
  try {
    const response = await fetch("/api/state");
    return await response.json();
  } catch (error) {
    byId("dot").className = "dot down";
    return null;
  }
}

async function refresh() {
  const state = await fetchState();
  if (!state) return;

  // Newest first reads best on the page; the final answer is featured on its own, so it is left out of the list.
  const findingsNewestFirst = [...state.findings].reverse();
  const finalAnswer = findingsNewestFirst.find((finding) => finding.title.startsWith(FINAL_ANSWER_TITLE_PREFIX));

  renderLinkIndicator(state);
  renderStatCards(state);
  renderHostingGuess(finalAnswer);
  renderAgentsTable(state);
  renderFindings(findingsNewestFirst.filter((finding) => finding !== finalAnswer));
  renderFacts(state.facts);
  renderFooter(state);
}

// ---- the sections, top to bottom

function renderLinkIndicator(state) {
  byId("dot").className = state.connected ? "dot" : "dot down";
}

function renderStatCards(state) {
  const ipDetails = (state.ip_info || [])[0] || {};
  const linkDetail = state.connected ? `agent card in ${state.connect_ms} ms` : state.last_error || "";
  const place = ipDetails.city ? `${ipDetails.city}, ${ipDetails.region || ipDetails.country}` : "—";

  const runtime = `last ${timeAgo(state.last_cycle)} · up since ${formatTime(state.started)}`;

  byId("stats").replaceChildren(
    statCard("Link to inside agent", state.connected ? "Up" : "Down", linkDetail),
    statCard("Public address", state.public_ips[0] || "unknown", ipDetails.org || "seen by Iroh via the relay"),
    statCard("Looks like", place, ipDetails.timezone || ""),
    statCard("Cycles", String(state.cycles), runtime),
  );
}

function statCard(label, value, detail) {
  return element(
    "div",
    { className: "card stat" },
    textElement("div", "k", label),
    textElement("div", "v", value),
    detail ? textElement("div", "s", detail) : "",
  );
}

function renderHostingGuess(finalAnswer) {
  const content = finalAnswer
    ? findingCard(finalAnswer, { featured: true })
    : textElement("p", "meta", "Not learned yet. The first cycle takes a minute or so.");
  byId("guess").replaceChildren(content);
}

function renderAgentsTable(state) {
  const hours = Math.round((state.agents_window_secs || DEFAULT_REGISTRY_WINDOW_SECS) / 3600);
  const synced = state.agents_synced ? timeAgo(state.agents_synced) : "…";
  byId("agents-note").textContent =
    `Everything the inside agent saw in the last ${hours} hours, refreshed over Iroh ${synced}. ` +
    "Agents on platforms that block inbound HTTP still show up here.";

  byId("agents").replaceChildren(agentsTable(state));
}

function renderFindings(findings) {
  byId("findings").replaceChildren(...findings.map((finding) => findingCard(finding, { featured: false })));
}

function renderFacts(facts) {
  const everything = facts ? JSON.stringify(facts, null, 2) : "none yet";
  byId("facts").replaceChildren(
    element(
      "div",
      { className: "card" },
      expandable("show everything this process could see locally", element("pre", { textContent: everything })),
    ),
  );
}

function renderFooter(state) {
  byId("foot").replaceChildren(
    definitionList({
      "this agent": state.endpoint_id,
      "inside agent": state.inside_id,
      "last error": state.last_error || "none",
    }),
  );
}

// ---- one exchange with the inside agent

/**
 * A card for one finding. LLM answers get light markdown; other skills show their raw text plus the
 * data they returned.
 */
function findingCard(finding, { featured }) {
  const isLlmAnswer = finding.skill === "llm.complete";

  const header = element(
    "div",
    { className: "row" },
    element("h3", { textContent: finding.title }),
    element(
      "span",
      {},
      textElement("span", `chip ${finding.state}`, finding.state),
      " ",
      textElement("span", "chip skill", finding.skill),
    ),
  );
  const timing = textElement("div", "meta", `${finding.ms} ms · ${formatTime(finding.at)}`);
  const answer = isLlmAnswer
    ? element("div", { className: "answer" }, richText(finding.answer))
    : element("pre", { textContent: finding.answer });

  const cardClass = featured ? "card finding feature" : "card finding";
  const card = element("div", { className: cardClass }, header, timing, answer);
  if (finding.detail && !isLlmAnswer) {
    card.append(expandable("returned data", element("pre", { textContent: JSON.stringify(finding.detail, null, 2) })));
  }
  return card;
}

/** A collapsed `<details>` with a one-line summary. */
function expandable(summary, content) {
  return element("details", {}, element("summary", { textContent: summary }), content);
}

// ---- the registry table

const AGENT_TABLE_COLUMNS = ["Agent", "Where it runs", "Network", "Last seen", "Activity"];

function agentsTable(state) {
  const card = element("div", { className: "card tablecard" });
  const agents = state.agents || [];

  if (agents.length === 0) {
    const message = state.agents_error
      ? `Registry unavailable: ${state.agents_error}`
      : "Waiting for the first registry sync…";
    card.append(element("p", { className: "meta", style: "padding:16px;margin:0", textContent: message }));
    return card;
  }

  const headerRow = element("tr", {}, AGENT_TABLE_COLUMNS.map((name) => textElement("th", "", name)));
  const rows = agents.map((agent) => agentRow(agent, state.endpoint_id));
  card.append(element("table", {}, element("thead", {}, headerRow), element("tbody", {}, rows)));
  return card;
}

/** One agent's row. `agent.report` is what the agent said about itself; it may be missing for a brand-new agent. */
function agentRow(agent, thisAgentId) {
  const report = agent.report || {};
  const isThisAgent = agent.endpoint_id === thisAgentId;

  return element(
    "tr",
    { className: isThisAgent ? "me" : "" },
    agentCell(agent, report, isThisAgent),
    whereItRunsCell(report),
    networkCell(report),
    lastSeenCell(agent),
    activityCell(agent, report),
  );
}

function agentCell(agent, report, isThisAgent) {
  const name = element(
    "div",
    {},
    textElement("span", "name", report.platform_hint || "unknown"),
    " ",
    isThisAgent ? textElement("span", "chip", "this agent") : "",
  );
  const cell = element("td", {}, name, textElement("div", "sub", shortId(agent.endpoint_id)));
  if (report.hosting_guess) {
    const guess = element("div", { className: "answer" }, richText(report.hosting_guess));
    cell.append(expandable("its hosting guess", guess));
  }
  return cell;
}

function whereItRunsCell(report) {
  // The agent's own guess is the headline; the network operator is the headline only when there is no guess.
  const headline = report.provider_guess || report.ip_org || "—";
  const location = joinNonEmpty([report.ip_city, report.ip_region], ", ");
  const operator = report.provider_guess ? report.ip_org : "";
  const details = joinNonEmpty([location, operator], " · ");
  return element("td", {}, textElement("div", "name", headline), textElement("div", "sub", details));
}

function networkCell(report) {
  return element(
    "td",
    {},
    textElement("div", "sub", report.public_ip || "no public IP seen"),
    textElement("div", "sub", joinNonEmpty([report.hostname, report.os], " · ")),
  );
}

function lastSeenCell(agent) {
  const status = agent.active
    ? textElement("span", "chip live", "live")
    : textElement("span", "chip", timeAgo(agent.last_seen));
  return element("td", {}, status, textElement("div", "sub", `first seen ${timeAgo(agent.first_seen)}`));
}

function activityCell(agent, report) {
  const cycles = report.cycles != null ? `${report.cycles} cycles` : "";
  return element("td", {}, textElement("div", "sub", `${agent.requests} requests`), textElement("div", "sub", cycles));
}

/** The start and end of an endpoint id, enough to tell agents apart. */
function shortId(endpointId) {
  return `${endpointId.slice(0, 8)}…${endpointId.slice(-4)}`;
}

// ---- wiring

byId("refresh").addEventListener("click", async (event) => {
  event.preventDefault();
  const button = event.target;
  await fetch("/api/refresh");
  button.textContent = "Queued…";
  setTimeout(() => (button.textContent = "Refresh now"), REFRESH_BUTTON_LABEL_MS);
});

refresh();
setInterval(refresh, REFRESH_EVERY_MS);
