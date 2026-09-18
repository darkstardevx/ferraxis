#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "==> no-default-features check"
cargo check --workspace --all-targets --no-default-features --locked

echo "==> no-default-features tests"
cargo test --workspace --no-default-features --locked

echo "==> all-features check"
cargo check --workspace --all-targets --all-features --locked
