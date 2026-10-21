#!/bin/sh
# Run clippy over every crate and test target, in the same disposable container as the build.
# Warnings are errors, so a clean run means a clean tree.
set -eu
cd "$(dirname "$0")/.."
podman volume exists cargo-home || podman volume create cargo-home >/dev/null
podman volume exists a2a-target || podman volume create a2a-target >/dev/null
exec podman run --rm --http-proxy=false -v cargo-home:/usr/local/cargo/registry:z \
  -v "$PWD":/work:z -v a2a-target:/work/target -w /work docker.io/library/rust:1-bookworm \
  sh -c 'rustup component add clippy >/dev/null 2>&1 && cargo clippy --locked --workspace --all-targets "$@" -- -D warnings' sh "$@"
