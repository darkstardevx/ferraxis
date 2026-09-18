#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

mapfile -t files < <(
  {
    find scripts -maxdepth 1 -type f -print
    find .githooks -maxdepth 1 -type f -print
  } | sort
)

for file in "${files[@]}"; do
  first="$(head -n1 "$file" 2>/dev/null || true)"
  if [[ "$first" == "#!/usr/bin/env bash" || "$first" == "#!/bin/bash" ]]; then
    bash -n "$file"
  fi
done

if command -v shellcheck >/dev/null 2>&1; then
  shellcheck "${files[@]}"
elif [[ "${FERRAXIS_REQUIRE_SHELLCHECK:-0}" == "1" ]]; then
  echo "error: shellcheck is required by this gate" >&2
  exit 1
else
  echo "warning: shellcheck not installed; CI will enforce it" >&2
fi

echo "shell-script policy passed"
