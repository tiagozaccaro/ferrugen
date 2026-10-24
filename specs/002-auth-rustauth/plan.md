# Plan: Auth with rustauth (Local + OAuth)

## Boundaries
- `crates/auth` (new / existing): auth trait and rustauth integration.
- `domain` / `contracts`: session DTOs, membership resolution.
- `infrastructure`: PostgreSQL session store, OAuth adapter config.
- `apps/api`: Axum middleware / handler wiring only; business rules stay in application.

## Dependency records (required per policy)
- `rustauth`: verify MIT/Apache license, maintenance status, session model, Axum compatibility.
- Local OAuth adapters: optional trait implementations, disabled when unconfigured.

## Validation sequence
1. `cargo check -p auth`; `cargo test -p auth`
2. Integration tests with PostgreSQL: register → session → session revocation → cross-tenant deny.
3. Dependency audit: `cargo deny check`; `cargo audit`.
