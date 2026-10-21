#!/usr/bin/env bash
# Deploy the outside agent to Fly.io with a remote build (no registry needed): a bundle dir holding the
# Dockerfile and the prebuilt binary. flyctl runs in a container.
# Needs: .env with FLYIO_TOKEN, keys/fly.key, keys/fly.token, keys/inside.id, and a built outside-agent
# binary (scripts/build.sh; copy it out of the a2a-target volume to $BIN).
set -euo pipefail
cd "$(dirname "$0")/.."
APP=a2a-outside-dlott
WORK=$(mktemp -d); trap 'rm -rf "$WORK"' EXIT
BIN=${BIN:?set BIN to the path of the built outside-agent binary}
mkdir "$WORK/bundle"; cp deploy/Containerfile.outside "$WORK/bundle/Dockerfile"; cp deploy/fly/fly.toml "$BIN" "$WORK/bundle/"
python3 - "$WORK" <<'PY'
import sys
env = {}
for l in open(".env"):
    l = l.strip()
    if l and not l.startswith("#") and "=" in l:
        k, v = l.split("=", 1); env[k.strip()] = v.strip().strip("\"'")
w = sys.argv[1]
open(f"{w}/fly.env", "w").write("FLY_API_TOKEN=" + env["FLYIO_TOKEN"] + "\n")
open(f"{w}/secrets", "w").write("OUTSIDE_SECRET_KEY_HEX=%s\nINSIDE_ENDPOINT_ID=%s\nDASHBOARD_TOKEN=%s\n" % tuple(open(f).read().strip() for f in ("keys/fly.key", "keys/inside.id", "keys/fly.token")))
PY
FLY="podman run --rm --http-proxy=false -e HOME=/tmp --env-file $WORK/fly.env"
$FLY docker.io/flyio/flyctl:latest apps create "$APP" --org personal 2>&1 | tail -1 || true
podman run --rm -i --http-proxy=false -e HOME=/tmp --env-file "$WORK/fly.env" docker.io/flyio/flyctl:latest secrets import -a "$APP" --stage < "$WORK/secrets"
$FLY -v "$WORK/bundle:/b:z" -w /b docker.io/flyio/flyctl:latest deploy --remote-only --ha=false -a "$APP"
echo "dashboard: https://$APP.fly.dev/?t=\$(cat keys/fly.token)"
