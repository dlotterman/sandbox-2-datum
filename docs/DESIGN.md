# Design

Agents on platforms that restrict HTTP exchange A2A messages with an agent on Datum Compute, and nobody opens an HTTP port to
the internet. They dial each other with Iroh.

## Transport

[Iroh](https://www.iroh.computer/) gives an authenticated, encrypted QUIC connection to a peer identified by a public key (its
*endpoint id*).

- **Relays.** Datum runs relays at `iroh-relay.{us-west-1,us-central-1,us-east-1}.datumconnect.net`. Each endpoint keeps one
  outbound HTTPS connection (443) to a home relay. Early packets go through the callee's home relay while both sides try UDP
  hole-punching; a direct path takes over if it works, and the relay stays as fallback. Relays see only ciphertext and need no
  credentials.
- **No discovery.** Endpoints use a custom relay map and no address lookup, so nothing contacts n0's DNS or pkarr services. A
  dial target is an endpoint id plus every relay as a hint. A ticket is reduced to its relay hints, because the direct
  addresses inside it go stale.
- **Streams.** ALPN `a2a-iroh/http1/0`. Each request is one HTTP/1.1 exchange on its own bi-directional stream; one Iroh
  connection serves many.
- **A2A.** The Agent Card is at `/.well-known/agent-card.json`. A skill call is JSON-RPC `message/send` with a data part
  `{skill, args}`; the reply is a Task in state `completed`, `rejected` (policy denial), `failed` or `input-required`.

## Identity

The inside agent's key comes from a secret, so its endpoint id is stable across restarts and redeploys. Each outside agent has
its own keypair; the inside agent keeps an allowlist of their ids (`ALLOWED_PEERS`) and drops anyone else right after the
handshake. A caller's identity is always the authenticated key, never anything in the request. Enrolling an agent means
editing the allowlist and restarting the inside agent; there is no enrollment API yet.

## Inside agent

### Policy gate

Every skill call passes, in order:

1. **Allowlist** on the connection's key.
2. **Rules:** per-peer hourly quota; request shape and size. For the LLM: a fixed model, a token cap, a stated purpose. For
   data: allowlisted columns and parameterized filters only. For files: flat names, a size cap, UTF-8 text.
3. **LLM review** (`llm.complete`, `data.query`, `files.get`): the model is shown the request and its purpose and answers allow
   or deny. It can only add restrictions, never lift a rules denial, and anything but an explicit `true` is a denial. Replies
   are screened so the gateway key cannot be echoed back.
4. **Audit:** every decision goes to stdout and Postgres (allowed registry syncs are skipped).

Model calls are capped at two at a time and retry 429/5xx with backoff. Without that, several agents cycling together hit rate
limits, and the fail-closed review turned them into wrongful denials.

### Skills

| Skill | Purpose |
|---|---|
| `llm.complete` | Chat completion on a fixed model; the key never leaves this agent |
| `ip.lookup` | Geolocation, ASN and reverse DNS for one public IP |
| `data.catalog` / `data.query` | What datasets and files exist; read rows from an allowlisted table |
| `files.get` | One small text file from the rclone store |
| `agents.sync` | Report the caller's metadata; list every agent seen in the last 6 hours |

### Data and registry

Postgres holds `platforms` (reference data), `agent_audit` and `agents_seen`. rclone serves `demo/rclone-data` as read-only
WebDAV. Neither is reachable outside the private Datum network, and on Compute their addresses arrive through a ConfigMap
(no private DNS).

Each authenticated request updates the caller's `agents_seen` row. A `agents.sync` report is rebuilt from allowlisted, typed,
length-capped fields (public IPs only) before it is stored. Agents are listed for six hours, pruned after 24, and called live
if seen within 90 seconds. Reports are self-declared and unverified.

## Outside agent

One process, no LLM credentials, no network peer but the inside agent.

- **Facts** come from local files and environment only; variables that look secret are never sent.
- **Public IP** comes from Iroh's own address discovery, so no "what is my IP" service is needed. Some platforms learn it
  late, so the agent waits a few seconds first.
- **Cycle** (`REFRESH_SECS`, default 600): look up the IP, ask what data exists, one LLM call that states a hunch and picks
  data, fetch it through the gate, one short final answer. Two model calls keep it quick and cheap. Reasoning models spend
  tokens thinking, so give completions generous `max_tokens`.
- **Registry loop** (`SYNC_SECS`, default 30): report itself, receive the recent agents.
- **HTTP** (`PORT` or `HTTP_BIND`; `off` disables): `/`, `/about`, `/healthz`, and a JSON API under `/api/`. With
  `DASHBOARD_TOKEN` set, everything but `/healthz` needs `?t=<token>` (which sets a cookie), the cookie, or a bearer header.

Per-platform quirks are in [deploy/](deploy/README.md).

## Limitations

- The allowlist is static, and enrolling an agent restarts the inside agent.
- Registry reports are self-declared, not attested.
- Postgres on Compute is ephemeral: the registry and audit log survive inside-agent restarts, not a database redeploy.
- No streaming (`message/stream`) or push notifications.
- The dashboard shows hostnames, addresses and environment variable names; set `DASHBOARD_TOKEN` on public deployments.
- The LLM review is probabilistic, layered on the rules and never a substitute for them.
