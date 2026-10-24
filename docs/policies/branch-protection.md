# CI and Branch Protection Recommendations

Manual steps (administration permissions required):
1. Make the following required checks:
   - `rust` (format, check, test, clippy)
   - `dependency-policy` (deny, audit)
2. Allow self-approval: in repo Settings → Branches → `main`, uncheck "Require approvals" or set required reviewers to 0 (manual admin step; file-based settings not supported without `probot/settings`).
3. Restrict pushes to `main`.
4. Use least-privilege `permissions:` in `.github/workflows` (already `contents: read`).
5. Do not bypass protection to merge agent-generated work.

Local equivalent for development (before opening PR):
- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`
- `cargo audit`
- `./scripts/instruction-integrity.sh`
