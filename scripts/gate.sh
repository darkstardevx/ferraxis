#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
mode="${1:-fast}"

is_implementation_path() {
  case "$1" in
    *.rs|Cargo.toml|Cargo.lock|*/Cargo.toml|rust-toolchain.toml|rustfmt.toml|clippy.toml|.clippy.toml)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

approved_plan_from_head() {
  git cat-file -e HEAD:.plans/ACTIVE 2>/dev/null || {
    echo "ERROR: implementation changes require an Approved plan already committed in HEAD." >&2
    return 1
  }

  local active status
  active="$(git show HEAD:.plans/ACTIVE | tr -d '\r\n')"
  [[ "$active" == .plans/*.plan.md ]] || {
    echo "ERROR: invalid active plan path in HEAD: $active" >&2
    return 1
  }

  git cat-file -e "HEAD:$active" 2>/dev/null || {
    echo "ERROR: active plan is missing from HEAD: $active" >&2
    return 1
  }

  status="$(git show "HEAD:$active" | grep '^Status:' | head -n1 || true)"
  [[ "$status" == "Status: Approved" ]] || {
    echo "ERROR: active plan is not Approved in HEAD: ${status:-<missing>}" >&2
    return 1
  }

  if git diff --cached --name-only -- .plans/ACTIVE "$active" | grep -q .; then
    echo "ERROR: approved plan and implementation cannot change in the same commit." >&2
    return 1
  fi

  printf '%s\n' "$active"
}

precommit() {
  git diff --cached --check
  staged="$(git diff --cached --name-only --diff-filter=ACMR)"
  [[ -n "$staged" ]] || exit 0

  has_impl=0
  while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    if is_implementation_path "$path"; then
      has_impl=1
      break
    fi
  done <<< "$staged"

  if [[ "$has_impl" -eq 1 ]]; then
    echo "Approved committed plan: $(approved_plan_from_head)"
    cargo fmt --all -- --check
    cargo check --workspace --all-targets --all-features --locked
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
  fi

  ./scripts/check-text-files
}

fast() {
  git diff --check
  git diff --cached --check
  ./scripts/check-text-files
  cargo fmt --all -- --check
  cargo metadata --locked --format-version 1 >/dev/null
  cargo check --workspace --all-targets --all-features --locked
  cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
  cargo test --workspace --all-features --locked
  cargo run -p xtask --locked -- validate
}

full() {
  fast
  ./scripts/check-features.sh
  ./scripts/check-msrv.sh
  ./scripts/check-shell.sh
  ./scripts/check-docs.sh
}

case "$mode" in
  precommit) precommit ;;
  fast) fast ;;
  docs) exec ./scripts/check-docs.sh ;;
  full) full ;;
  release)
    full
    ./scripts/package-check.sh
    ;;
  *)
    echo "usage: ./scripts/gate.sh {precommit|fast|docs|full|release}" >&2
    exit 2
    ;;
esac
