#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo '==> git diff --check'
git diff --check
git diff --cached --check

echo '==> cargo fmt'
cargo fmt --all -- --check

echo '==> cargo check'
cargo check --workspace --all-targets --all-features

echo '==> cargo clippy'
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo '==> cargo test'
cargo test --workspace --all-features

echo '==> repository validation'
cargo run -p xtask -- validate

echo '==> rustdoc'
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --all-features

echo '==> complete gate passed'
