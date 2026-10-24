#!/usr/bin/env bash
# Regression tests for workflow safety around pull-request bodies.
#
# `github.event.pull_request.body` is attacker-controlled. It must never be
# interpolated into a `run:` script, and the scripts that consume it must treat
# it as data. Otherwise quotes, backticks or `$()` in a PR description break CI
# (or execute arbitrary commands on the runner).
set -uo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT" || exit 1

FAIL=0
pass() { printf 'PASS %s\n' "$1"; }
fail() { printf 'FAIL %s\n' "$1" >&2; FAIL=1; }

# --- 1. Static guard: no event data interpolated inside `run:` blocks --------
lint_workflows() {
  awk '
    /^[[:space:]]*run:[[:space:]]*\|[[:space:]]*$/ {
      match($0, /[^ ]/); inrun = 1; runindent = RSTART; next
    }
    /^[[:space:]]*run:[[:space:]]*[^|]/ {
      if (index($0, "${{") > 0 && index($0, "github.event") > 0) {
        printf "%s:%d: %s\n", FILENAME, FNR, $0
      }
      next
    }
    inrun {
      if ($0 ~ /^[[:space:]]*$/) next
      match($0, /[^ ]/)
      if (RSTART <= runindent) { inrun = 0; next }
      if (index($0, "${{") > 0 && index($0, "github.event") > 0) {
        printf "%s:%d: %s\n", FILENAME, FNR, $0
      }
    }
  ' .github/workflows/*.yml
}

hits=$(lint_workflows)
if [ -z "$hits" ]; then
  pass 'no github.event interpolation inside run: blocks'
else
  printf '%s\n' "$hits" >&2
  fail 'github.event interpolation inside run: blocks'
fi

# --- 2. Scripts treat the body as data --------------------------------------
WORK=$(mktemp -d "${TMPDIR:-/tmp}/audit-log-test.XXXXXX") || exit 1
trap 'rm -rf "$WORK"' EXIT

HOSTILE_BODY=$(cat "$ROOT/tests/github-governance/fixtures/hostile-pr-body.md")

audit_out=$(
  cd "$WORK" || exit 1
  PR_NUMBER=7 \
  PR_AUTHOR=evil \
  PR_BODY="$HOSTILE_BODY" \
  BASE_SHA=0000000 \
  HEAD_SHA=1111111 \
  bash "$ROOT/scripts/github/audit-agent-log.sh" 2>&1
)
audit_rc=$?

if [ "$audit_rc" -eq 0 ]; then
  pass 'audit script exits 0 for a hostile PR body'
else
  fail "audit script exited $audit_rc"
fi

for expected in '## Agent Audit Log' 'PR: 7' 'Author: evil' 'Agent/Harness: Agent / harness used: opencode'; do
  if printf '%s\n' "$audit_out" | grep -qF "$expected"; then
    pass "audit line: $expected"
  else
    fail "audit output missing: $expected"
  fi
done

TEMPLATE_BODY=$(cat <<'EOF'
## Summary
- test

## Agent / Change Tracking
- Agent / harness used: opencode
EOF
)
audit_out=$(
  cd "$WORK" || exit 1
  PR_NUMBER=8 \
  PR_AUTHOR=test \
  PR_BODY="$TEMPLATE_BODY" \
  BASE_SHA=0000000 \
  HEAD_SHA=1111111 \
  bash "$ROOT/scripts/github/audit-agent-log.sh" 2>&1
)
if printf '%s\n' "$audit_out" | grep -qF 'Agent/Harness: Agent / harness used: opencode'; then
  pass 'audit script uses the agent field, not the section heading'
else
  fail 'audit script matched the Agent / Change Tracking heading'
fi

if [ ! -e "$WORK/pwned" ]; then
  pass 'audit script did not execute body contents'
else
  fail 'audit script executed body contents'
fi

if (
  cd "$WORK" || exit 1
  PR_BODY="$HOSTILE_BODY" bash "$ROOT/scripts/github/check-pr-body.sh" >/dev/null 2>&1
); then
  pass 'PR body validator accepts a valid hostile body'
else
  fail 'PR body validator rejected a valid hostile body'
fi

if [ ! -e "$WORK/pwned" ]; then
  pass 'PR body validator did not execute body contents'
else
  fail 'PR body validator executed body contents'
fi

if (
  cd "$WORK" || exit 1
  PR_BODY=$(cat "$ROOT/tests/github-governance/fixtures/incomplete-pr-body.md")
  PR_BODY="$PR_BODY" bash "$ROOT/scripts/github/check-pr-body.sh" >/dev/null 2>&1
); then
  fail 'PR body validator accepted an incomplete body'
else
  pass 'PR body validator rejects an incomplete body'
fi

# --- Result -----------------------------------------------------------------
if [ "$FAIL" -eq 1 ]; then
  printf 'Workflow injection tests FAILED\n' >&2
  exit 1
fi

printf 'Workflow injection tests passed\n'
