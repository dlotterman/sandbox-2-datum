// The about page: show who this agent is talking to, using the same state the dashboard uses.

async function showWhoIsTalking() {
  const response = await fetch("/api/state");
  const state = await response.json();
  const offeredSkills = ((state.inside_card || {}).skills || []).map((skill) => skill.id);

  byId("ids").replaceChildren(
    definitionList({
      "this agent": state.endpoint_id,
      "inside agent": state.inside_id,
      "link": state.connected ? `up, agent card fetched in ${state.connect_ms} ms` : "down",
      "skills offered": offeredSkills.join(", ") || "—",
    }),
  );
}

// If the state cannot be fetched the box simply stays empty; the rest of the page does not depend on it.
showWhoIsTalking().catch(() => {});
