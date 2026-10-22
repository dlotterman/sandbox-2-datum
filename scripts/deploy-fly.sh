#!/usr/bin/env bash
# Deploy the outside agent to Fly.io (Fly Apps) with a remote build: a bundle dir holding the Dockerfile and the
# prebuilt binary, so no registry is involved. flyctl runs in a container.
#
#   APP=a2a-outside-b BIN=path/to/outside-agent INSIDE_ID_FILE=keys/inside-b.id \
#     ENROLLMENT_TOKEN_FILE=keys/enrollment-b.token DASHBOARD_TOKEN_FILE=keys/fly-b.token  scripts/deploy-fly.sh
#
#   APP                    globally unique app name (becomes APP.fly.dev)                       (required)
#   BIN                    the built outside-agent binary                                       (required)
#   INSIDE_ID_FILE         file with the inside agent's endpoint id                             (required)
#   ENROLLMENT_TOKEN_FILE  the inside agent's enrollment token: no key needed (preferred)       (one of these two)
#   KEY_FILE               a pre-registered secret key (hex) instead                            (one of these two)
#   DASHBOARD_TOKEN_FILE   token protecting the public dashboard                                (required)
#   FLY_ORG                organization slug (default personal)
#
# Needs FLYIO_TOKEN in .env (parsed, never sourced: the token contains a space).
set -euo pipefail
cd "$(dirname "$0")/.."
: "${APP:?set APP to a globally unique app name}"
: "${BIN:?set BIN to the path of the built outside-agent binary}"
: "${INSIDE_ID_FILE:?set INSIDE_ID_FILE}"
: "${DASHBOARD_TOKEN_FILE:?set DASHBOARD_TOKEN_FILE}"
[ -n "${ENROLLMENT_TOKEN_FILE:-}${KEY_FILE:-}" ] || { echo "set ENROLLMENT_TOKEN_FILE (or KEY_FILE)" >&2; exit 1; }
ORG=${FLY_ORG:-personal}

WORK=$(mktemp -d); trap 'rm -rf "$WORK"' EXIT
mkdir "$WORK/bundle"
cp deploy/Containerfile.outside "$WORK/bundle/Dockerfile"
cp "$BIN" "$WORK/bundle/outside-agent"
# The app name comes from -a below, so the checked-in fly.toml's own name is dropped from the bundle copy.
grep -v '^app = ' deploy/fly/fly.toml > "$WORK/bundle/fly.toml"

python3 - "$WORK" <<'PY'
import os, sys
env = {}
for line in open(".env"):
    line = line.strip()
    if line and not line.startswith("#") and "=" in line:
        name, value = line.split("=", 1)
        env[name.strip()] = value.strip().strip("\"'")
work = sys.argv[1]
read = lambda path: open(path).read().strip()
open(f"{work}/fly.env", "w").write("FLY_API_TOKEN=" + env["FLYIO_TOKEN"] + "\n")
secrets = {"INSIDE_ENDPOINT_ID": read(os.environ["INSIDE_ID_FILE"]), "DASHBOARD_TOKEN": read(os.environ["DASHBOARD_TOKEN_FILE"])}
if os.environ.get("ENROLLMENT_TOKEN_FILE"):
    secrets["ENROLLMENT_TOKEN"] = read(os.environ["ENROLLMENT_TOKEN_FILE"])
else:
    secrets["OUTSIDE_SECRET_KEY_HEX"] = read(os.environ["KEY_FILE"])
open(f"{work}/secrets", "w").write("".join(f"{k}={v}\n" for k, v in secrets.items()))
PY

FLYCTL="docker.io/flyio/flyctl:latest"
fly() { podman run --rm --http-proxy=false -e HOME=/tmp --env-file "$WORK/fly.env" "$@"; }
fly "$FLYCTL" apps create "$APP" --org "$ORG" 2>&1 | tail -1 || true
sleep 3
# Secrets are staged from a file so none appears on a command line.
podman run --rm -i --http-proxy=false -e HOME=/tmp --env-file "$WORK/fly.env" "$FLYCTL" secrets import -a "$APP" --stage < "$WORK/secrets"
sleep 3
fly -v "$WORK/bundle:/b:z" -w /b "$FLYCTL" deploy --remote-only --ha=false -a "$APP"
echo "dashboard: https://$APP.fly.dev/?t=<contents of $DASHBOARD_TOKEN_FILE>"
