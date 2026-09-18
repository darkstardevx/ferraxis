#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
MSRV="1.85.0"

command -v rustup >/dev/null 2>&1 || {
  echo "error: rustup is required for MSRV validation" >&2
  exit 1
}

if ! rustup run "$MSRV" rustc --version >/dev/null 2>&1; then
  echo "error: Rust $MSRV is not installed" >&2
  echo "install it with: rustup toolchain install $MSRV --profile minimal" >&2
  exit 1
fi

rustup run "$MSRV" rustc --version
cargo +"$MSRV" check --workspace --all-targets --all-features --locked
cargo +"$MSRV" test --workspace --all-features --locked
cargo +"$MSRV" check --workspace --all-targets --no-default-features --locked
