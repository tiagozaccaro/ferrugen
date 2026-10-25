#!/usr/bin/env bash
# Failing test: the CI/CD pipeline set is complete with respect to the governance target plan.
# Before implementation this script exits non-zero; pass after the missing workflows exist
# and are wired into the main branch ruleset reference.
set -euo pipefail
cd "$(dirname "$0")/../.."

workflow=1
fail() { echo "FAIL: $1" >&2; exit 1; }
ok()   { echo "OK: $1"; }

# Files that the governance target plan expects but are currently absent.
MISSING=()
for wf in security.yml agent-compliance.yml ai-review-evaluation.yml; do
  if [[ ! -f ".github/workflows/$wf" ]]; then MISSING+=("$wf"); fi
done
if (( ${#MISSING[@]} )); then
  fail "Missing workflow(s): ${MISSING[*]}"
fi

# Ruleset must record all mandatory checks that block merge on `main`.
REPO_RULES=".github/repo-rules.yml"
MANDATORY=(rust dependency-policy security agent-compliance ai-review-evaluation release-tooling workflow-safety)
for required in "${MANDATORY[@]}"; do
  if ! grep -q "$required" "$REPO_RULES"; then
    fail "repo-rules.yml missing mandatory check '$required'"
  fi
done

ok "Pipeline set complete: missing workflows removed and ruleset references updated."
