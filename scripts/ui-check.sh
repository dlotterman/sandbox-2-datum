#!/bin/sh
# Check the dashboard and about page in a real headless browser (Playwright, in a container).
#
# Serves the real outside-agent binary and feeds the pages a fixed fake state, so it needs no LLM, no inside
# agent and no network beyond installing Playwright. Run scripts/build.sh first.
#
#   scripts/ui-check.sh                          # run the checks
#   SCREENSHOT_DIR=/tmp/shots scripts/ui-check.sh   # also save full-page screenshots (light and dark)
set -eu
cd "$(dirname "$0")/.."
PORT=${PORT:-8099}
WORK=$(mktemp -d)
trap 'podman rm -f a2a-ui-check >/dev/null 2>&1 || true; rm -rf "$WORK"' EXIT

# 1. Put the built binary into a throwaway image.
podman run --rm --http-proxy=false -v a2a-target:/t:ro,z -v "$WORK":/out:z docker.io/library/debian:bookworm-slim \
  cp /t/release/outside-agent /out/
podman build -q --http-proxy=false -t localhost/a2a-outside:ui-check -f deploy/Containerfile.outside "$WORK" >/dev/null

# 2. Start it. It needs a syntactically valid inside endpoint id to start; it never gets an answer, which is fine
#    because the checks replace /api/state.
INSIDE_ID=$(podman run --rm --http-proxy=false localhost/a2a-outside:ui-check keygen | sed -n 's/^ENDPOINT_ID=//p')
podman rm -f a2a-ui-check >/dev/null 2>&1 || true
podman run -d --name a2a-ui-check --http-proxy=false -p "$PORT:8080" -e INSIDE_ENDPOINT_ID="$INSIDE_ID" \
  localhost/a2a-outside:ui-check >/dev/null
sleep 3

# 3. Run the browser checks.
SHOTS=""
if [ -n "${SCREENSHOT_DIR:-}" ]; then mkdir -p "$SCREENSHOT_DIR"; SHOTS="-v $SCREENSHOT_DIR:/shots:z -e SCREENSHOT_DIR=/shots"; fi
# shellcheck disable=SC2086
podman run --rm --network host --http-proxy=false -e PORT="$PORT" $SHOTS -v "$PWD/scripts/ui-check.js":/w/ui-check.js:ro,z \
  mcr.microsoft.com/playwright:v1.49.0-jammy \
  sh -c 'cd /tmp && npm i --silent playwright@1.49.0 >/dev/null 2>&1 && NODE_PATH=/tmp/node_modules node /w/ui-check.js'
