#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo '==> rustdoc'
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --all-features --locked

echo '==> project documentation structure'
cargo run -p xtask --locked -- validate-docs

if [[ "${FERRAXIS_SKIP_EXTERNAL_DOC_TOOLS:-0}" == "1" ]]; then
    echo '==> external documentation tools skipped by FERRAXIS_SKIP_EXTERNAL_DOC_TOOLS=1'
    exit 0
fi

command -v npx >/dev/null 2>&1 || {
  echo 'error: npx is required for markdownlint-cli2' >&2
  exit 1
}

echo '==> markdownlint'
npx --yes markdownlint-cli2@0.23.2 "**/*.md"

command -v lychee >/dev/null 2>&1 || {
  echo 'error: lychee is required for local link checking' >&2
  exit 1
}

echo '==> link check'
lychee --config lychee.toml "**/*.md"

echo '==> documentation gate passed'
