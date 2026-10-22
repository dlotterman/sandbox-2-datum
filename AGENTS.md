# Working on this repo

Two Rust agents exchange A2A over Iroh, brokered by Datum's relays. The **inside agent** (on Datum Compute) alone holds the LLM
key and is the only door to Postgres and rclone. **Outside agents** run anywhere with outbound HTTPS, introspect themselves,
ask the inside agent for help, and serve a dashboard. Overview: [README.md](README.md). How and why: [docs/DESIGN.md](docs/DESIGN.md).

## Where things are

Each crate's `main.rs` or `lib.rs` opens with a module map; start there.

- `crates/a2a-iroh`: the shared tunnel (endpoints, server, client, A2A messages). `tests/tunnel.rs` is a real Iroh round trip.
- `crates/inside-agent`: `rpc.rs` (request handling), `enrollment.rs` (allowlist and token enrollment), `policy.rs` (the
  gate), `skills.rs`, `llm.rs`, `data/`. All its
  environment variables are documented at the top of `config.rs`.
- `crates/outside-agent`: `cycle.rs` and `investigation.rs` (the hosting-guess story and its prompts), `registry.rs`, `web.rs`.
  Environment variables are at the top of `config.rs`. The UI is plain HTML, CSS and JavaScript in `assets/`, compiled into
  the binary, so edits need a rebuild.
- `deploy/`, `scripts/`, `demo/`, `sbom.md`. `deploy/vm/` unpacks the image onto VM-like targets (Vultr, Fly Sprites). **Read `docs/deploy/` for your platform before deploying.**

## Code style

Written to be read by people first; pick the clearer form over the shorter one.

- Name things for what they are, give magic numbers a named constant, prefer small named functions to long blocks.
- Comments say *why* (a constraint, a platform quirk), not what the next line does.
- Prompts are named constants at the top of their module.
- JavaScript follows the same rules, and sets text only through `textContent` or text nodes, never `innerHTML`.
- Before committing: `scripts/fmt.sh` and `scripts/lint.sh` (rustfmt at 120 columns; clippy with warnings as errors).

## Tooling

- No Rust toolchain is assumed on the host. `scripts/build.sh` builds in a disposable container; binaries land in the
  `a2a-target` volume under `release/` (see `demo/local-stack.sh` for copying them out).
- Commands use `podman`; Docker or any compatible runtime works too (rootless is just how we run it). Podman-specific:
  `--http-proxy=false` (stops hosts with proxy variables leaking them into containers); `:z` on bind mounts is for SELinux hosts.
- Never `source` a `.env`: values can contain spaces. Parse it (`scripts/deploy-exe.py` shows how).
- `gcloud` and `datumctl` may need to be on `PATH` or run from a container.

## Invariants (do not weaken)

- **Check order:** allowlisted or enrolled, then rules, then LLM review. A stranger reaches nothing but the Agent Card and
  `agents.enroll`, and the enrolled check comes before any skill runs or the registry is touched. The review only ever adds restrictions, and any error or
  unparsable verdict is a denial. Do not loosen it to "fix" a denial caused by an upstream error.
- The outside agent has **no LLM credentials** and **no peer but the inside agent**: no discovery service, no external IP
  lookup (its public IP comes from Iroh).
- The inside agent has **no HTTP listener**. Never add a proxy, load balancer or NetworkService that exposes it.
- Data skills take only allowlisted columns and parameterized filters (never free-form SQL), flat file names and size caps.
- Registry reports are self-declared; sanitize them and never treat them as attested.
- Secrets stay out of git, logs, images and chat: keys in `keys/` and `.env` are git-ignored, deployments use the platform's
  secret store, and you print only ids and prefixes. Public dashboards must set `DASHBOARD_TOKEN`.

## Validating changes

`python3 -m unittest discover -s deploy/vm` tests the image unpacker. `scripts/test.sh` runs the Rust tests in a container (`-p <crate>` for one; the first run compiles everything). The suite is small
and aimed at intent, not coverage, and has been mutation-checked: breaking the strict review verdict, file-name validation,
key-echo screening, the allowlist, the enrollment gate, token check, expiry and cap, the dashboard gate, the public-IP check, SQL
parameter binding or the catalog filter each fails a test. A mock LLM server (`testutil::mock_llm`) scripts model replies.

Not covered by tests: Postgres and rclone paths, relays and hole-punching. Check those and everything else by hand:

1. `scripts/build.sh` finishes with no warnings, and `scripts/test.sh` passes.
2. `demo/local-stack.sh up` (needs `BASETEN_API_KEY`; set `PORT` if 8088 is taken), then check `/api/state`: connected, a
   completed cycle, no `last_error`. `down` when done.
3. UI changes: `scripts/ui-check.sh` (after `build.sh`) runs 30 checks in a headless browser against a fake state, in light and
   dark. `SCREENSHOT_DIR=…` saves screenshots; pages fill in after a fetch, so a screenshot at load shows empty sections.
4. New security behavior gets a test that fails without it. Break the behavior once to see it fail.

## Working agreements

- Deploys, deletions, force-pushes and anything touching someone else's cloud account are outward-facing: confirm scope
  first, look at the target before deleting, and say plainly what you did and did not verify.
- When deploying, go one resource at a time, let endpoints settle, and check each result before the next step.
- Learned something new about a platform? Update its page in `docs/deploy/`.

## Licensing

The Unlicense covers this project's code; dependencies keep their own licenses. When you add or upgrade a dependency or change
a base image: check its license (flag copyleft crates in the executables to the owner), run `scripts/build.sh` then
`scripts/generate-sbom.py`, commit `sbom.md`, and read its "Worth calling out" section. Don't copy code from other repositories
without checking its license.
