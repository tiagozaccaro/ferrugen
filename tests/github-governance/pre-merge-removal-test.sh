#!/usr/bin/env bash
# Acceptance test: the automatic pre-merge backup tag mechanism is fully removed
# while the remaining recovery guidance stays intact.
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

fail=0

if [ -e .github/workflows/backup-tag.yml ]; then
  echo "FAIL: .github/workflows/backup-tag.yml still exists"
  fail=1
fi

if git grep -n --no-color -e 'pre-merge' -- . ':!tests' >/dev/null 2>&1; then
  echo "FAIL: 'pre-merge' is still referenced:"
  git grep -n --no-color -e 'pre-merge' -- . ':!tests' || true
  fail=1
fi

# Removing the tag mechanism must not remove rollback guidance.
for needle in 'git revert' 'revert.yml' 'git bisect'; do
  if ! grep -qF "$needle" docs/recovery.md; then
    echo "FAIL: docs/recovery.md lost recovery guidance: $needle"
    fail=1
  fi
done

if [ "$fail" -ne 0 ]; then
  exit 1
fi

echo "PASS: pre-merge backup tag mechanism removed"
