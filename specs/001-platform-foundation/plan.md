# Implementation Plan: Reusable SaaS Platform Foundation

**Branch**: `001-platform-foundation` | **Date**: 2026-10-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-platform-foundation/spec.md`

**Task IDs**: `Pxxx` = original plan task; `Txxx` = execution task in `tasks.md`.

## Summary

Implement the foundation and tenancy slice (P001-P010) of the reusable SaaS platform: validate the Rust workspace and target matrix, enforce dependency boundaries, add stable IDs/errors/configuration, wire PostgreSQL with SQLx migrations, create the initial users/tenants/memberships/sessions schema, complete the rustauth spike, implement local auth and session lifecycle at the service layer, resolve membership-verified tenant context, and enforce deny-by-default RBAC with cross-tenant negative tests.

The API remains a thin Axum entry point with honest liveness/readiness endpoints. HTTP auth endpoints, UI, audit persistence, storage, jobs, AI, backup and packaging remain delegated to P011-P019 and their owning specs.

## Technical Context

**Language/Version**: Rust 1.89.0 (pinned toolchain), edition 2024, MSRV 1.85

**Primary Dependencies**: axum 0.8, tokio 1, sqlx 0.8 (PostgreSQL, migrations), rustauth 0.3 + rustauth-sqlx 0.3 (P007 gate), argon2 0.5, uuid 1, chrono 0.4, thiserror 2, serde 1, tracing 0.1 / tracing-subscriber 0.3, tower-http 0.6, dotenvy 0.15. Existing workspace dependencies are reused; no TOML configuration crate is required for this slice.

**Storage**: PostgreSQL 17 via `compose.yaml`; SQLx migrations from the repository `migrations/` directory; no Redis, object storage or hosted database.

**Testing**: `cargo test` unit tests plus `sqlx::test` PostgreSQL integration tests; `proptest`/`rstest` available for property and table-driven tests; Postgres 17 service in CI.

**Target Platform**: Host targets (Linux x86_64 CI, macOS Apple Silicon development) plus `wasm32-unknown-unknown` check for the web app. Desktop/mobile packaging is deferred to spec 016/P018.

**Project Type**: Rust workspace / modular-monolith backend with reusable crates and thin apps.

**Performance Goals**: Local login p95 under 500 ms on the supported runner (spec 002 SC-001); readiness database check bounded to 2 seconds; no throughput target is claimed for the foundation slice.

**Constraints**: No `unsafe`, `unwrap`, `expect` or panic in production paths; tenant isolation enforced at the repository boundary; no PostgreSQL RLS in this slice; no `orgs` tables; no HTTP auth endpoints; no audit persistence; no mandatory paid or hosted service.

**Scale/Scope**: 21 reusable crates, 6 apps and 1 template package already in the workspace; P001-P010 only; initial schema of four tables; one PostgreSQL instance; horizontal scaling and multi-instance throttling are out of scope.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate | Status |
|---|---|---|
| I. Open-source and self-hosting first | MIT workspace; PostgreSQL runs locally via Compose; no paid service is required for P001-P010 | PASS |
| II. Tenant isolation is mandatory | Repository-boundary scoping with `tenant_id` in SQL; cross-tenant negative tests | PASS |
| III. Reuse before duplication | IDs/errors/config in `platform-core`; tenancy in `domain`/`application`; no app-specific copies | PASS |
| IV. Modular monolith by default | One API entry point and one worker; crate boundaries preserved; no distributed services | PASS |
| V. Secure defaults | Deny-by-default RBAC; Argon2id; opaque hashed session tokens; login throttling; application-owned audit port; no secrets in logs | PASS |
| VI. Provider neutrality | rustauth sits behind the auth service; audit sits behind an application-owned port with `audit` sinks; optional providers are not required | PASS |
| VII. Testable changes | Failing tests first; unit, integration, migration and cross-tenant negative tests | PASS |
| VIII. Dependency discipline | `cargo deny`/`cargo audit`, boundary script, rustauth spike gate, license review for new crates | PASS (with rustauth risk noted) |
| IX. Small, reviewable delivery | P001-P010 only; P011-P019 delegated | PASS |
| X. Honest completion | Readiness reports real database/migration state; desktop/mobile and rustauth claims are gated | PASS |
| Locked architecture choices | rustauth is adopted only if P007 passes; orgs are deferred to spec 003; apalis/rig/Wasmtime are outside this slice | PASS (with explicit deferrals) |

No constitution violations require complexity tracking. The rustauth maturity risk is handled as a stop-and-amend gate in P007, not as a silent substitution.

## Project Structure

### Documentation (this feature)

```text
specs/001-platform-foundation/
├── plan.md              # This file
├── research.md          # Phase 0 decisions and alternatives
├── data-model.md        # Phase 1 schema and state rules
├── quickstart.md        # Phase 1 local validation guide
├── contracts/
│   ├── health.md        # Liveness/readiness contract
│   └── internal-ports.md # Repository, auth, throttle, audit and clock ports
├── spec.md              # Feature specification
└── tasks.md             # Regenerated by /speckit.tasks
```

### Source Code (repository root)

```text
crates/
├── platform-core/src/
│   ├── lib.rs
│   ├── ids.rs           # UserId, TenantId, SessionId
│   ├── errors.rs        # CoreError
│   └── config.rs        # Validated AppConfig value objects
├── contracts/src/lib.rs # HealthResponse, ErrorResponse, TenantSummary
├── domain/src/
│   ├── lib.rs
│   ├── tenant.rs        # Tenant name validation
│   ├── membership.rs    # Membership + MembershipStatus
│   └── role.rs          # Role, Permission, role matrix
├── application/src/
│   ├── lib.rs
│   ├── tenant_context.rs
│   ├── ports.rs         # MembershipReader, repositories
│   ├── audit.rs         # AuditAction, AuditEvent, AuditOutcome, AuditSink, AuditError
│   └── authorize.rs     # Deny-by-default permission checks
├── auth/src/
│   ├── lib.rs
│   ├── service.rs       # Register/login/logout/authenticate
│   ├── ports.rs         # User/session repositories, Clock, LoginThrottle
│   └── throttle.rs      # In-memory fixed-window login throttle
├── tenancy/src/lib.rs   # Re-exports only
├── audit/src/lib.rs     # Tracing/recording AuditSink implementations
├── persistence/src/
│   ├── lib.rs
│   ├── pool.rs          # PgPool construction
│   ├── migrations.rs    # sqlx::migrate! runner
│   ├── users.rs
│   ├── tenants.rs
│   ├── memberships.rs
│   └── sessions.rs
├── infrastructure/src/
│   ├── lib.rs
│   ├── config.rs        # Environment loading and validation
│   └── wiring/          # Concrete repository/service construction (auth.rs, tenancy.rs)
└── rate-limit/src/lib.rs # Remains scaffold; login throttle is auth-specific in 001

apps/api/src/
├── main.rs              # Config, pool, migrations, router, bind
├── state.rs             # AppState + readiness state
└── health.rs            # /health/live and /health/ready

apps/api/tests/
└── health.rs            # Live/readiness contract tests

migrations/
└── 0001_foundation.sql  # users, tenants, tenant_memberships, sessions

scripts/
├── check-dependency-boundaries.sh
└── tests/
    └── check-dependency-boundaries-test.sh

.env.example             # DATABASE_URL and auth/throttle settings
.github/workflows/ci.yml # Postgres service, wasm check, boundary check
```

**Structure Decision**: Keep the existing workspace and crate boundaries. `platform-core` owns IDs, errors and configuration value objects; `domain` owns tenant/membership/role rules; `application` owns tenant context, authorization ports and the audit port/event types; `auth` owns local auth, sessions and throttling and emits audit events through the application-owned port; `audit` provides tracing and recording sinks; `persistence` is the only crate that owns SQLx row access; `infrastructure` wires concrete adapters; `apps/api` remains a thin HTTP entry point. No new crates are introduced. `application` must not depend on `auth` or `audit`; both capability crates may depend downward on `application`.

## Phased Delivery

### Phase 0 — Workspace, boundaries, primitives (P001-P003)

- Add a failing boundary-script fixture test that proves an upward dependency or misplaced heavy dependency is rejected.
- Add `scripts/check-dependency-boundaries.sh` using `cargo metadata --no-deps` and `jq`.
- Add `SessionId`, extend `CoreError` only where needed, and add validated `AppConfig` value objects in `platform-core`.
- Add config tests for missing `DATABASE_URL`, invalid port, non-positive TTL/window/lockout and valid defaults.
- Add `.env.example`; document that `DATABASE_URL` is required and contains no committed secret.
- Add the wasm32 target check to the plan/CI; do not claim desktop/mobile packaging.

### Phase 1 — PostgreSQL and schema (P004-P006)

- Add `PgPool` construction with bounded acquire timeout and `sqlx::migrate!` from `migrations/`.
- Add `migrations/0001_foundation.sql` for `users`, `tenants`, `tenant_memberships` and `sessions`.
- Update `migrations/README.md` to match the four-table slice and mark roles/permissions/orgs/jobs as deferred.
- Add repository implementations for users, tenants, memberships and sessions.
- Add `sqlx::test` integration tests for fresh migration, membership uniqueness, tenant creation atomicity and cross-tenant denial.
- Add the Postgres 17 service and `DATABASE_URL` to CI.

### Phase 2 — rustauth spike and ADR (P007)

- Verify rustauth 0.3 + rustauth-sqlx 0.3 against license, maintenance, local password auth, session creation/revocation, Axum integration, SQLx/PostgreSQL compatibility and multi-tenant suitability.
- Record the decision in an ADR under `docs/architecture/adr/`.
- If the gate fails, stop and request a constitution amendment; do not silently replace rustauth with a custom stack.
- The logical contract remains: Argon2id password hashes, opaque session tokens, SHA-256 token hashes at rest, expiry and revocation.

### Phase 3 — Local auth and sessions (P008)

- Define the application-owned `AuditSink` port and audit event types first in `crates/application/src/audit.rs`; then define the auth ports (`UserRepository`, `SessionRepository`, `LoginThrottle`, `Clock`) in `crates/auth`.
- Only after the port interfaces are fixed, implement registration, login, logout and token authentication; normalize email; reject duplicate email and passwords outside the 12-128 character policy.
- Implement in-memory fixed-window login throttling (default 5 failures per 15 minutes, 15-minute lockout, configurable) with a `Clock` for deterministic tests.
- Emit audit events for login success/failure and logout through the application-owned `AuditSink`; Phase 5 emits role-change and member-removal events. `crates/audit` provides tracing and recording sinks. Persistence remains P013.
- Add tests for success, invalid credentials, duplicate email, expired session, revoked session, lockout and audit events.

### Phase 4 — Tenant context and tenant creation (P009)

- Extend `TenantContext` with `role`; construct it only from an active membership.
- Treat missing and inactive memberships as `NotMember` to avoid leaking membership status.
- Implement `create_tenant_with_owner` in a transaction: insert tenant and owner membership atomically; no separate ownership column.
- Implement repository-level tenant scoping: every tenant-scoped method takes a verified `TenantContext` and includes `tenant_id` in SQL.
- Add tests for tenant creation atomicity, inactive membership denial, and cross-tenant read/write denial.

### Phase 5 — RBAC and membership administration (P010)

- Implement the `Role`/`Permission` matrix: owner > admin > member > viewer; deny by default.
- Owner: full tenant control including deletion/transfer, member management and settings.
- Admin: member management and settings, but cannot grant/remove owner or delete/transfer the tenant.
- Member: tenant data read/write. Viewer: tenant data read only.
- Implement role change and member removal at the service layer with a transaction and `SELECT ... FOR UPDATE` on tenant memberships.
- Enforce a last-owner guard: a tenant must always retain at least one active owner; reject demotion/removal of the last owner.
- Retain removed memberships with status `removed`; deny access; re-adding a removed member reactivates the same `(user_id, tenant_id)` row.
- Emit audit events for role changes and member removal.
- Add permission-matrix, last-owner, cross-tenant role-change and re-invite tests.

### Phase 6 — API readiness and final validation (P001-P010 completion)

- Implement and verify `/health/live` and `/health/ready` per `contracts/health.md` first; ready is 200 only with a reachable database and applied migrations, otherwise 503 within 2 seconds.
- After readiness is implemented, run the full validation suite: `cargo fmt --all -- --check`, `cargo check --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`.
- Run `cargo check -p web --target wasm32-unknown-unknown`.
- Run `cargo deny check`, `cargo audit` and `bash scripts/check-dependency-boundaries.sh`.
- Rerun the full suite after any readiness fix; record actual command output in the PR. Do not claim desktop/mobile packaging.

## Testing Strategy

- **Unit**: ID distinctness, tenant name bounds, role/permission matrix, config validation, token hashing, throttle window/lockout.
- **Integration**: `sqlx::test` for migrations, users, tenants, memberships and sessions; repository tenant scoping; tenant creation transaction.
- **Security/negative**: cross-tenant read/write denial, inactive/removed membership denial, last-owner protection, expired/revoked session, throttled login, duplicate email.
- **Contract**: liveness always 200; readiness 200 only when database and migrations are ready; readiness 503 with bounded timeout when not.
- **Migration**: fresh database applies `0001_foundation.sql`; a second migration run is a no-op; schema version is current.
- **Boundary**: fixture test proves the script rejects an upward dependency and a misplaced heavy dependency; current workspace passes.
- **Concurrency**: membership uniqueness is enforced by the primary key; role-change/last-owner checks run inside a transaction with row locks.
- **Success criteria coverage**: SC-001 → migration tests; SC-002 → readiness contract tests; SC-003 → tenant-isolation/negative tests; SC-004 → throttle tests; SC-005 → non-gating login benchmark; SC-006 → membership uniqueness/re-invite tests; SC-007 → CI validation job.

## CI Changes

- Add a `postgres:17` service to the `rust` job with a `pg_isready` health check and `DATABASE_URL=postgres://saas:saas_dev_only@127.0.0.1:5432/saas`.
- Add `rustup target add wasm32-unknown-unknown` and `cargo check -p web --target wasm32-unknown-unknown`.
- Add `bash scripts/check-dependency-boundaries.sh`.
- Remove `continue-on-error` from the audit check so `cargo audit` is blocking; add a CI assertion/grep so the audit step cannot regress; keep `cargo deny` blocking.
- Keep the GTK/system libraries needed by host desktop checks.
- Keep the existing fmt/check/test/clippy steps and the workflow-safety/release-tooling jobs.

## Risks and Deferred Work

- **rustauth maturity**: low adoption and a recent release; P007 is a hard gate. If it fails, stop and request a constitution amendment rather than silently substituting.
- **rustauth schema compatibility**: the logical model is four tables; P007 must map rustauth's session/user storage or document a compatible adaptation.
- **Single-instance throttle**: in-memory throttling is per process; distributed/multi-instance throttling is deferred. Document the limitation.
- **Session TTL**: default 30 days, configurable; exact product policy can be tuned later.
- **Tenant deletion/transfer**: the RBAC matrix grants the permission, but HTTP endpoints and destructive operations remain P011/spec 003.
- **Audit persistence**: only the application-owned port and tracing/recording sinks are in 001; persistence is P013.
- **Orgs, RLS, roles/permissions tables, jobs, AI, storage, backup and packaging**: deferred to specs 002-016.
- **Policy path gap**: root `AGENTS.md` references `docs/security-policy.md` and `docs/testing-policy.md`, while the files live under `docs/policies/`; fix as a documentation cleanup outside P001-P010.

## Complexity Tracking

No constitution violations. No entries required.
