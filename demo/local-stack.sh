#!/usr/bin/env bash
# Run the whole demo on one machine in containers: Postgres, rclone, the inside agent and an outside agent, on
# separate networks. Written for Podman; Docker or another runtime should work with minor changes. The agents still
# meet through Datum's public Iroh relays (override with RELAYS=...).
#
# The outside agents have no keys of their own. They hold an enrollment token, enroll themselves with the inside agent
# on first contact, and so can be added at any time without restarting it.
#
#   demo/local-stack.sh up      needs: scripts/build.sh already run, BASETEN_API_KEY exported
#   demo/local-stack.sh add     start another outside agent against the running stack (no restart of anything else)
#   demo/local-stack.sh down
#
# Env: BASETEN_API_KEY (required), BASETEN_MODEL, BASETEN_BASE_URL, PORT (first dashboard port, default 8088).
set -euo pipefail
cd "$(dirname "$0")/.."
STATE=.local-stack            # git-ignored: the inside agent's key (stable identity) and the enrollment token
PORT=${PORT:-8088}
INSIDE=a2a-local-inside
OUTSIDE_IMAGE=localhost/a2a-outside:local

# Start an outside agent named $1, publishing its dashboard on host port $2 (empty: any free port).
start_outside() {
  local name=$1 host_port=$2
  podman run -d --name "$name" --network a2a-local-out --http-proxy=false -p "${host_port}:8080" \
    -e INSIDE_ENDPOINT_ID="$(cat "$STATE/inside.id")" -e ENROLLMENT_TOKEN="$(cat "$STATE/enrollment.token")" \
    ${RELAYS:+-e RELAYS="$RELAYS"} "$OUTSIDE_IMAGE" >/dev/null
}

stop_everything() {
  podman ps -a --format '{{.Names}}' | grep '^a2a-local-' | xargs -r podman rm -f >/dev/null 2>&1 || true
  podman network rm a2a-local-in a2a-local-out >/dev/null 2>&1 || true
}

case "${1:-up}" in
down)
  stop_everything
  echo "stopped"
  exit 0
  ;;
add)
  podman container exists "$INSIDE" || { echo "the stack is not running: run '$0 up' first" >&2; exit 1; }
  number=$(($(podman ps -a --format '{{.Names}}' | grep -c '^a2a-local-outside') + 1))
  name="a2a-local-outside-$number"
  start_outside "$name" ""
  echo "$name: dashboard at http://127.0.0.1:$(podman port "$name" 8080/tcp | head -1 | sed 's/.*://')/"
  exit 0
  ;;
esac

: "${BASETEN_API_KEY:?export BASETEN_API_KEY first}"
mkdir -p "$STATE/bin"
umask 077

# Binaries come out of the build volume created by scripts/build.sh.
podman run --rm --http-proxy=false -v a2a-target:/t:ro,z -v "$PWD/$STATE/bin":/out:z docker.io/library/debian:bookworm-slim \
  sh -c 'cp /t/release/inside-agent /t/release/outside-agent /out/'
podman build -q --http-proxy=false -t localhost/a2a-inside:local -f deploy/Containerfile.inside "$STATE/bin" >/dev/null
podman build -q --http-proxy=false -t "$OUTSIDE_IMAGE" -f deploy/Containerfile.outside "$STATE/bin" >/dev/null
podman build -q --http-proxy=false -t localhost/a2a-rclone:local -f deploy/Containerfile.rclone . >/dev/null

# Identities and secrets are made once and kept, so the inside agent's endpoint id is stable between runs.
if [ ! -f "$STATE/inside.key" ]; then
  podman run --rm --http-proxy=false -v "$PWD/$STATE/bin":/i:ro,z docker.io/library/debian:bookworm-slim \
    /i/inside-agent keygen > "$STATE/inside.keygen"
  sed -n 's/^SECRET=//p' "$STATE/inside.keygen" > "$STATE/inside.key"
  sed -n 's/^ENDPOINT_ID=//p' "$STATE/inside.keygen" > "$STATE/inside.id"
  rm -f "$STATE/inside.keygen"
fi
[ -f "$STATE/enrollment.token" ] || head -c 18 /dev/urandom | base64 | tr -d '/+=' > "$STATE/enrollment.token"

stop_everything
podman network create a2a-local-in >/dev/null
podman network create a2a-local-out >/dev/null

PASSWORD=$(head -c 18 /dev/urandom | base64 | tr -d '/+=')
podman run -d --name a2a-local-pg --network a2a-local-in --http-proxy=false \
  -e POSTGRES_USER=a2a -e POSTGRES_PASSWORD="$PASSWORD" -e POSTGRES_DB=a2a docker.io/library/postgres:17 >/dev/null
podman run -d --name a2a-local-rclone --network a2a-local-in --http-proxy=false \
  -e RC_USER=a2a -e RC_PASS="$PASSWORD" localhost/a2a-rclone:local >/dev/null

# The inside agent's settings go through a temporary file so the LLM key never appears on a command line.
{
  echo "BASETEN_API_KEY=$BASETEN_API_KEY"
  echo "BASETEN_MODEL=${BASETEN_MODEL:-zai-org/GLM-5.3-Flash}"
  [ -n "${BASETEN_BASE_URL:-}" ] && echo "BASETEN_BASE_URL=$BASETEN_BASE_URL"
  echo "INSIDE_SECRET_KEY_HEX=$(cat "$STATE/inside.key")"
  echo "ENROLLMENT_TOKEN=$(cat "$STATE/enrollment.token")"
  echo "POSTGRES_HOST=a2a-local-pg"; echo "POSTGRES_USER=a2a"; echo "POSTGRES_PASSWORD=$PASSWORD"; echo "POSTGRES_DB=a2a"
  echo "RCLONE_URL=http://a2a-local-rclone:8080"; echo "RC_USER=a2a"; echo "RC_PASS=$PASSWORD"
  [ -n "${RELAYS:-}" ] && echo "RELAYS=$RELAYS"
  true
} > "$STATE/inside.env"
podman run -d --name "$INSIDE" --network a2a-local-in --http-proxy=false --env-file "$STATE/inside.env" \
  localhost/a2a-inside:local >/dev/null
rm -f "$STATE/inside.env"

start_outside a2a-local-outside "$PORT"
echo "dashboard: http://127.0.0.1:$PORT/   (first cycle takes a minute; '$0 add' starts more outside agents)"
