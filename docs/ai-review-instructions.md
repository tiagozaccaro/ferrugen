# AI Code Review Instructions

Every AI agent must complete this review checklist before committing or opening a PR.

## Mandatory checks (must pass before any commit)
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo deny check` passes (advisories, bans, licenses, sources)
- [ ] `cargo check --workspace` passes
- [ ] `cargo test --workspace` passes (or failing test is explicitly required by spec/test-first policy)
- [ ] Pre-commit hook `.github/hooks/pre-commit` passes

## Scope / governance checks
- [ ] Change is bounded to the issue / spec referenced in PR body (`Closes #N`)
- [ ] No silent broadening (no changes to auth/tenancy/persistence/AI/plugin/crates unless spec requires)
- [ ] Agent tracking fields present: `Agent / harness`, `Scope`, `Issue / spec link`
- [ ] Commit tags include `[agent:<harness>]` for AI-generated commits

## Security / quality checks
- [ ] No secrets, tokens, or credentials in source / logs / fixtures / examples
- [ ] No `unsafe` code; no `unwrap` / `expect` / `panic` in production paths
- [ ] Every tenant-scoped query is tenant-scoped at repository boundary (deny by default; authorize server-side)
- [ ] Cross-tenant read/write denial tests exist for tenant-scoped modules
- [ ] Dependency direction respected: platform-core -> contracts/domain -> application -> infrastructure -> app
- [ ] No dependency cycles introduced
- [ ] License impact reviewed (`cargo deny` clean; `MPL-2.0` allowed only via explicit policy)

## Review workflow
- [ ] Independent code / security / test review mentioned in PR body
- [ ] PR uses `.github/pull_request_template.md` sections (`## Summary`, `## Issue`, `## Acceptance criteria`, `## Independent reviews`)
- [ ] Never push directly to `main`; never merge without authorization
- [ ] Solo-maintainer exception: sole maintainer may approve own PR when `CODEOWNERS` / ruleset permits
