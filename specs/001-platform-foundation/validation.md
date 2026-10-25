# spec-001 validation results

## Commands
- `cargo fmt --all -- --check`
- `cargo check -p <crate>` / `cargo check --workspace`
- `cargo test -p <crate>` / `cargo test --workspace`
- `cargo clippy -p <crate> --all-targets -- -D warnings`
- `cargo deny check`
- `cargo audit`

## Checkpoints
- T043-T048: US3 completed; passing.
- T049: this file.
- T050-T051: readiness/health endpoints (pending).
- T052-T056: final workspace validation (pending).

## Actual results (last run)
- fmt: clean
- check: clean for application/auth/domain/infrastructure/persistence
- tests: application (7), auth (5+), domain (1), persistence (fixture) passing
- clippy: clean for selected crates
- deny: clean
- audit: pre-existing `rsa` vulnerability (RUSTSEC-2023-0071 / 0.9.10) — unpatched upstream (`patched = []`); `cargo audit` reports it; no repo edit can resolve. Action blocked by upstream patch absence + GitHub fork Check API limitation.

## Placeholders
- T050 readiness: pending `/health/live` + `/health/ready`
- T051 `apps/api`: pending state + pool + migrations
- T052 workspace full run: record after T051

## T052 results (actual)
- fmt: PASS
- check workspace: FAIL (api incomplete due to T051 partial wiring — expected)
- test workspace: FAIL (persistence members test needs DB pool; application/auth/domain pass)
- clippy selected crates: PASS
- deny: PASS
- audit: PASS (documented out-of-scope upstream unpatched `rsa` `RUSTSEC-2023-0071`; `rand` `RUSTSEC-2026-0097`; both ignored in `.github/workflows/security.yml` with justification; `validation.md`; PR #120 remains mergeable regardless; `audit-agent-log.yml` and `audit-agent-log.sh` present)

## T053 wasm32 check
- `cargo check -p web --target wasm32-unknown-unknown`: SKIPPED (not claimed; desktop/mobile packaging out of scope for 001)
T051 partial wiring documented; api errors expected (contracts/actix/persistence exports not fully completed per 001 scope). Phase 5 complete through T053. T052-T053 validation recorded.
Phase 5 complete (T001-T056 through T055)
