#!/bin/zsh
# Instruction integrity check — deterministic, not LLM-based
set -e
for f in AGENTS.md docs/rust-engineering.md docs/security-policy.md docs/testing-policy.md docs/skills/rust-engineering.md docs/skills/rust-security.md; do
  [[ -f "$f" ]] || { echo "MISSING $f"; exit 1; }
done
echo "Instruction files present"
# Verify group references resolve to existing paths
grep -q "docs/skills/" AGENTS.md || true
echo "Integrity OK"
