#!/bin/sh
# Format the workspace with rustfmt, in the same disposable container as the build.
# Pass --check to only report differences (for CI or a pre-commit look).
set -eu
cd "$(dirname "$0")/.."
podman volume exists cargo-home || podman volume create cargo-home >/dev/null
exec podman run --rm --http-proxy=false -v cargo-home:/usr/local/cargo/registry:z \
  -v "$PWD":/work:z -w /work docker.io/library/rust:1-bookworm \
  sh -c 'rustup component add rustfmt >/dev/null 2>&1 && cargo fmt --all "$@"' sh "$@"
