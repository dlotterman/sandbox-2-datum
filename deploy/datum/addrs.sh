#!/usr/bin/env bash
# Print a ConfigMap with the private IPv6 addresses of the Postgres and rclone instances, for
# `datumctl apply -f -`. Datum has no private DNS yet (same approach as datum-loreate).
set -euo pipefail
DATUMCTL="${DATUMCTL:-podman run -i --rm --http-proxy=false --network host -v datumctl-config:/root/.datumctl:z docker.io/dlotterman/datumctl-container:latest}"
addr() {
  # INTERNAL IP is the instance's /96 block; the connectable address is its first address.
  local a
  a=$($DATUMCTL compute instances --workload="$1" --no-headers | awk 'NR==1 {print $4}' | cut -d/ -f1)
  [ -n "$a" ] || { echo "no instance for workload $1" >&2; exit 1; }
  echo "$a"
}
cat <<JSON
{"apiVersion":"v1","kind":"ConfigMap","metadata":{"name":"a2a-addrs"},
 "data":{"POSTGRES_HOST":"$(addr a2a-postgres)","RCLONE_HOST":"$(addr a2a-rclone)"}}
JSON
