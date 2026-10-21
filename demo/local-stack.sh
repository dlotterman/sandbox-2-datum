#!/usr/bin/env bash
# Run the whole demo on one machine with rootless Podman: Postgres, rclone, the inside agent and an outside agent,
# on separate networks. The agents still meet through Datum's public Iroh relays (override with RELAYS=...).
#
#   demo/local-stack.sh up      needs: scripts/build.sh already run, BASETEN_API_KEY exported
#   demo/local-stack.sh down
#
# Env: BASETEN_API_KEY (required), BASETEN_MODEL, BASETEN_BASE_URL, PORT (dashboard port, default 8088).
set -euo pipefail
cd "$(dirname "$0")/.."
STATE=.local-stack            # keys live here (git-ignored) so the agents keep their identities between runs
PORT=${PORT:-8088}
C=(a2a-local-pg a2a-local-rclone a2a-local-inside a2a-local-outside)

if [ "${1:-up}" = down ]; then
  podman rm -f "${C[@]}" >/dev/null 2>&1 || true
  podman network rm a2a-local-in a2a-local-out >/dev/null 2>&1 || true
  echo "stopped"; exit 0
fi

: "${BASETEN_API_KEY:?export BASETEN_API_KEY first}"
mkdir -p "$STATE/bin"; umask 077

# Binaries come out of the build volume created by scripts/build.sh.
podman run --rm --http-proxy=false -v a2a-target:/t:ro,z -v "$PWD/$STATE/bin":/out:z docker.io/library/debian:bookworm-slim \
  sh -c 'cp /t/release/inside-agent /t/release/outside-agent /out/'
for who in inside outside; do
  podman build -q --http-proxy=false -t "localhost/a2a-$who:local" -f "deploy/Containerfile.$who" "$STATE/bin" >/dev/null
  if [ ! -f "$STATE/$who.key" ]; then
    podman run --rm --http-proxy=false -v "$PWD/$STATE/bin":/i:ro,z docker.io/library/debian:bookworm-slim "/i/outside-agent" keygen > "$STATE/$who.keygen"
    sed -n 's/^SECRET=//p' "$STATE/$who.keygen" > "$STATE/$who.key"; sed -n 's/^ENDPOINT_ID=//p' "$STATE/$who.keygen" > "$STATE/$who.id"
    rm -f "$STATE/$who.keygen"
  fi
done
podman build -q --http-proxy=false -t localhost/a2a-rclone:local -f deploy/Containerfile.rclone . >/dev/null

"$0" down
podman network create a2a-local-in >/dev/null; podman network create a2a-local-out >/dev/null
PW=$(head -c 18 /dev/urandom | base64 | tr -d '/+=')
podman run -d --name a2a-local-pg --network a2a-local-in --http-proxy=false \
  -e POSTGRES_USER=a2a -e POSTGRES_PASSWORD="$PW" -e POSTGRES_DB=a2a docker.io/library/postgres:17 >/dev/null
podman run -d --name a2a-local-rclone --network a2a-local-in --http-proxy=false -e RC_USER=a2a -e RC_PASS="$PW" localhost/a2a-rclone:local >/dev/null
{ echo "BASETEN_API_KEY=$BASETEN_API_KEY"; echo "BASETEN_MODEL=${BASETEN_MODEL:-zai-org/GLM-5.3-Flash}"
  [ -n "${BASETEN_BASE_URL:-}" ] && echo "BASETEN_BASE_URL=$BASETEN_BASE_URL"
  echo "INSIDE_SECRET_KEY_HEX=$(cat "$STATE/inside.key")"; echo "ALLOWED_PEERS=$(cat "$STATE/outside.id")"
  echo "POSTGRES_HOST=a2a-local-pg"; echo "POSTGRES_USER=a2a"; echo "POSTGRES_PASSWORD=$PW"; echo "POSTGRES_DB=a2a"
  echo "RCLONE_URL=http://a2a-local-rclone:8080"; echo "RC_USER=a2a"; echo "RC_PASS=$PW"
  [ -n "${RELAYS:-}" ] && echo "RELAYS=$RELAYS"; true; } > "$STATE/inside.env"
podman run -d --name a2a-local-inside --network a2a-local-in --http-proxy=false --env-file "$STATE/inside.env" localhost/a2a-inside:local >/dev/null
rm -f "$STATE/inside.env"
podman run -d --name a2a-local-outside --network a2a-local-out --http-proxy=false -p "$PORT:8080" \
  -v "$PWD/$STATE/outside.key:/keys/outside.key:ro,z" -e OUTSIDE_KEY_FILE=/keys/outside.key \
  -e INSIDE_ENDPOINT_ID="$(cat "$STATE/inside.id")" ${RELAYS:+-e RELAYS="$RELAYS"} localhost/a2a-outside:local >/dev/null
echo "dashboard: http://127.0.0.1:$PORT/   (first cycle takes a minute)"
