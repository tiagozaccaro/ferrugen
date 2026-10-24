# GitHub governance target plan

Reference: `docs/index.md`, `AGENTS.md` (test-first instruction added). Does not change application architecture (`docs/architecture/overview.md`, `docs/guides/architecture-choices.md`).

Implementation phases (from user-provided plan):
- Phase 0: Baseline audit; Phase 1-4: CI/security/agent/AI; Phase 5-8: Reliability/advanced/release/enforcement.

Target inventory (not all created): `.github/workflows/ci.yml`, `security.yml`, `agent-compliance.yml`, `ai-review-evaluation.yml`, `release-validation.yml`; `tests/github-governance/`; `scripts/github/`.

Enforcement: required PR gate (`PR Gate / Required`) depends on applicable mandatory checks; merge protected default branch; agent changes must include issue link and test results.

Keep benchmark fixtures out of public agent prompts.
