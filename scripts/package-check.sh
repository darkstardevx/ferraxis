#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

if [[ ! -f LICENSE && ! -f LICENSE-APACHE && ! -f LICENSE-MIT ]]; then
  echo "release-readiness blocked: P0-M021 licensing decision is incomplete" >&2
  exit 1
fi

cargo metadata --locked --format-version 1 >/dev/null

mapfile -t packages < <(
  cargo metadata --locked --format-version 1 |
    python3 -c '
import json, sys
metadata = json.load(sys.stdin)
for package in metadata["packages"]:
    if package.get("publish") is not False and package["source"] is None:
        print(package["name"])
'
)

[[ "${#packages[@]}" -gt 0 ]] || {
  echo "error: no publishable workspace packages found" >&2
  exit 1
}

for package in "${packages[@]}"; do
  echo "==> cargo package -p $package"
  cargo package -p "$package" --locked --allow-dirty --no-verify
done
