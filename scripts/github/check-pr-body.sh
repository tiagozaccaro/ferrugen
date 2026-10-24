#!/usr/bin/env bash
# Validates the mandatory pull-request template sections.
#
# The body is read from $PR_BODY as data. It must never be interpolated into a
# `run:` script, because PR descriptions are attacker-controlled.
set -uo pipefail

body=${PR_BODY:-}
fail=0

require() { # pattern, message  (case-insensitive)
  if printf '%s\n' "$body" | grep -qiE "$1"; then
    return 0
  fi
  printf 'FAIL: %s\n' "$2" >&2
  fail=1
}

require_exact() { # pattern, message  (case-sensitive section headings)
  if printf '%s\n' "$body" | grep -qE "$1"; then
    return 0
  fi
  printf 'FAIL: %s\n' "$2" >&2
  fail=1
}

require 'Agent.*:|Agent / harness' 'Agent/harness not specified'
require 'Scope.*:|Scope' 'Change scope not specified'
require 'Closes #|Issue.*:|specs/' 'Issue/spec link missing'
require 'Independent reviews|review' 'Independent review mention missing'
require_exact '## Summary' "PR template section '## Summary' missing"
require_exact '## Issue' "PR template section '## Issue' missing"
require_exact '## Acceptance criteria' "PR template section '## Acceptance criteria' missing"

if [ "$fail" -eq 1 ]; then
  exit 1
fi

printf 'PASS: Agent tracking, scope, issue link, reviews present\n'
