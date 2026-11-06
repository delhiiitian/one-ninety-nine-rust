#!/usr/bin/env bash
# Runs every lesson example and every project's tests.
# Usage: scripts/run-all.sh
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build -q --examples
failed=0
count=0
for name in $(grep -E '^name = "ch[0-9]+-[0-9]+"' Cargo.toml | sed -E 's/name = "(.*)"/\1/'); do
  count=$((count + 1))
  # Lessons that read the keyboard get sample input piped in.
  case "$name" in
    ch06-01) input=$'Asha\n120\n45.5\nchai\n\n' ;;
    *) input="" ;;
  esac
  if printf '%s' "$input" | timeout 60 cargo run -q --example "$name" > /dev/null 2>"/tmp/rust199-$name.err"; then
    echo "ok   $name"
  else
    echo "FAIL $name"; sed 's/^/     /' "/tmp/rust199-$name.err"
    failed=$((failed + 1))
  fi
done
echo "$count lessons run, $failed failed"

cargo test -q --workspace
[ "$failed" -eq 0 ]
