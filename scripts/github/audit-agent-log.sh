#!/usr/bin/env bash
# Renders one audit-log entry for a pull request.
#
# Inputs are read from the environment and treated as data, never evaluated:
# PR_NUMBER, PR_AUTHOR, PR_BODY, BASE_SHA, HEAD_SHA.
set -uo pipefail

pr_number=${PR_NUMBER:-unknown}
author=${PR_AUTHOR:-unknown}
body=${PR_BODY:-}
base=${BASE_SHA:-}
head=${HEAD_SHA:-}

agent=$(printf '%s\n' "$body" | grep -i -m1 -o -E 'Agent.*|Harness.*' || true)
[ -n "$agent" ] || agent=unknown

scope=$(printf '%s\n' "$body" | grep -i -m1 -A2 'Change scope' || true)
scope=$(printf '%s\n' "$scope" | head -n 1)
[ -n "$scope" ] || scope=unspecified

commits=unknown
if [ -n "$base" ] && [ -n "$head" ]; then
  commits=$(git rev-list --count "${base}..${head}" 2>/dev/null || echo unknown)
fi

printf '## Agent Audit Log\n'
printf 'PR: %s\n' "$pr_number"
printf 'Author: %s\n' "$author"
printf 'Agent/Harness: %s\n' "$agent"
printf 'Scope (from PR body): %s\n' "$scope"
printf 'Commits: %s\n' "$commits"
printf -- '---\n'
