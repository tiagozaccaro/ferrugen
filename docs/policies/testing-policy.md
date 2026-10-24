# Testing Policy

Distinguish:
- Unit tests (per crate).
- Integration tests (`tests/` or crate-level).
- Regression tests (capture acceptance criteria; never delete to make CI green).
- Contract/API tests (contracts/domain crates).
- Security/auth tests (cross-tenant denial, tenant isolation).
- Migration tests where database used (persistence/infrastructure crates).
- Failure-path/concurrency tests where applicable (async, sync crates).

Requirements:
- Before implementation: generate failing test that captures criteria; implement until test passes.
- Tests must validate meaningful behaviour, not implementation details (avoid brittle mocks).
- Isolate fixtures; do not depend on external services by default.
- No arbitrary coverage percentages; measure baseline meaningfully.
- Commands: `cargo test -p <crate>`; `cargo test --workspace`.
