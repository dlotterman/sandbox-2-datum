#!/bin/sh
# Build the workspace in a disposable rootless container; target dir lives in a podman volume.
set -eu
cd "$(dirname "$0")/.."
podman volume exists cargo-home || podman volume create cargo-home >/dev/null
podman volume exists a2a-target || podman volume create a2a-target >/dev/null
exec podman run --rm --http-proxy=false -v cargo-home:/usr/local/cargo/registry:z \
  -v "$PWD":/work:z -v a2a-target:/work/target -w /work docker.io/library/rust:1-bookworm \
  cargo build --release "$@"
