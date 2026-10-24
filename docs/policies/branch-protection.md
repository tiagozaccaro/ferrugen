# CI and Branch Protection — Ruleset

Migrated from classic branch protection to GitHub Ruleset (`main` branch).

Required checks (Ruleset → Status checks):
- `rust` (format, check, test, clippy)
- `dependency-policy` (deny, audit)

Push restrictions: `main` protected.
Bypass / self-approval: configured via Ruleset bypass actors (manual admin step).

Manual steps (admin permissions required):
1. Configure Ruleset for `main`: required checks, restrict pushes, set bypass for self-approval if needed.
2. Verify `docs/index.md` references ruleset (not classic settings).
3. Use least-privilege `permissions:` in `.github/workflows`.
4. Never bypass protection to merge agent-generated work.

Local equivalent for development (before opening PR):
- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`
- `cargo audit`
- `./scripts/instruction-integrity.sh`
