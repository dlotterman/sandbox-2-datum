# Deploy cheat sheets

Field notes from doing it: exact steps, what to check, what went wrong, how to tear down. Commands marked *not run here* are
standard tooling we did not exercise.

| Page | Runs | Status |
|---|---|---|
| [Datum Compute](datum-compute.md) | Inside agent, Postgres, rclone | Deployed, repeatedly |
| [Fly.io](fly-io.md) | Outside agent | Deployed, verified |
| [Cloud Run](cloud-run.md) | Outside agent | Deployed, verified |
| [exe.dev](exe-dev.md) | Outside agent | Deployed, verified |
| [Local Podman](local-podman.md) | Outside agent, or everything | Deployed, verified |
| [Railway](railway.md) | Outside agent | **Not deployed** (Railway outage) |

## The order of things

```mermaid
flowchart LR
  build["Build binaries<br/>scripts/build.sh"] --> images["Build and push images<br/>(public registry, pin by digest)"]
  images --> datum["Datum Compute<br/>network → secrets → Postgres + rclone<br/>→ addresses → inside agent"]
  datum --> id["Note the inside agent's<br/>endpoint id"]
  keys["One keypair + dashboard token<br/>per outside agent"] --> allow["Add each public id to<br/>ALLOWED_PEERS, restart inside"]
  datum --> allow
  id --> outside["Deploy outside agents<br/>(Fly · Cloud Run · exe.dev · Podman)"]
  allow --> outside
  outside --> verify["Check the inside agent's log<br/>and any dashboard's agents table"]
```

## Common steps

**1. Build** (no Rust toolchain needed on the host): `scripts/build.sh`. Binaries land in the `a2a-target` volume.

**2. Build and push images.** Every platform runs the same Containerfiles in `deploy/`.

```sh
mkdir -p .local-stack/bin
podman run --rm --http-proxy=false -v a2a-target:/t:ro,z -v "$PWD/.local-stack/bin":/out:z \
  docker.io/library/debian:bookworm-slim sh -c 'cp /t/release/inside-agent /t/release/outside-agent /out/'
podman build -t ghcr.io/OWNER/REPO:inside  -f deploy/Containerfile.inside  .local-stack/bin
podman build -t ghcr.io/OWNER/REPO:outside -f deploy/Containerfile.outside .local-stack/bin
podman build -t ghcr.io/OWNER/REPO:rclone  -f deploy/Containerfile.rclone  .
podman push ghcr.io/OWNER/REPO:inside --digestfile /tmp/inside.digest     # likewise outside and rclone
```

Platforms pull by digest (`@sha256:…`), which pins exactly what runs. exe.dev needs a **public** image, and GitHub has no API
for that: flip the package in its settings once.

**3. Make a keypair and a dashboard token per outside agent.**

```sh
podman run --rm --http-proxy=false -v "$PWD/.local-stack/bin":/i:ro,z docker.io/library/debian:bookworm-slim \
  /i/outside-agent keygen          # prints SECRET=<hex> and ENDPOINT_ID=<hex>; inside-agent keygen makes the inside key
python3 -c "import secrets; print(secrets.token_urlsafe(18))" > keys/fly.token
```

Keep them in git-ignored `keys/` as `NAME.key` (secret), `NAME.id` (public id) and `NAME.token`.

**4. Allowlist the outside agents:** put their ids in the inside agent's `ALLOWED_PEERS` and restart it
([how](datum-compute.md#allowlist-an-agent)).

**5. Verify.** Look for the agent in the inside log, and in the **Agents the inside agent has seen** table on any dashboard,
usually within a minute. That table is the best check for platforms that block inbound HTTP: read it from a different agent.

```sh
datumctl compute logs a2a-inside | grep -E 'peer (connected|disconnected)|denied peer'
```

## When an agent does not show up

```mermaid
flowchart TD
  start["Agent missing from the registry table"] --> dash{"Does its own dashboard load?"}
  dash -- "no" --> plat["Platform side: port, proxy, token.<br/>See that platform's page"]
  dash -- "yes" --> link{"Link says Up?"}
  link -- "no" --> denied{"Inside log shows<br/>'denied peer'?"}
  denied -- "yes" --> fix1["Add its id to ALLOWED_PEERS,<br/>restart the inside agent"]
  denied -- "no" --> fix2["Wrong INSIDE_ENDPOINT_ID, or the inside<br/>agent is down: datumctl compute instances"]
  link -- "yes" --> cycle{"Cycle completed?"}
  cycle -- "no" --> fix3["Read last_error on /api/state.<br/>LLM 429s or denials: inside audit log"]
  cycle -- "yes" --> fix4["Wait one sync interval (30 s).<br/>Still missing: is Postgres reachable?"]
  start -. "dashboard works but<br/>container never started" .-> fix5["Look at the platform's logs:<br/>setup scripts that never ran look like this"]
```

## Gotchas on every platform

- Podman copies the host's `http_proxy` into containers: pass `--http-proxy=false`.
- Never `source` the `.env`: values like Fly's `FlyV1 …` token contain spaces, and bash runs (and echoes) them. Parse the file.
- Give each step a few seconds before the next; Datum in particular pushes back on rapid applies.
