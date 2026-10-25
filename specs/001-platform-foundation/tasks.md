# Tasks: Reusable SaaS Platform Foundation

**Input**: Design documents from `/specs/001-platform-foundation/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Required by the repository testing policy and `plan.md`. Write each test task first, confirm it fails, then implement until it passes. Do not weaken or skip a failing test.

**Organization**: The spec's three user stories are expressed as independently testable delivery stories after the shared setup and foundational phases:

- **US1** — local auth and sessions (spec US1; plan P007-P008)
- **US2** — tenant context and tenant creation (spec US2; plan P009)
- **US3** — RBAC and membership administration (spec US3; plan P010)

Setup and foundational phases have no story label. Tasks use the repository-relative paths from `plan.md`.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with other `[P]` tasks in the same phase (different files, no incomplete dependency)
- **[Story]**: `[US1]`, `[US2]` or `[US3]` for delivery-story phases only
- Every task includes an exact file path

## Path Conventions

Rust workspace: `crates/<crate>/src/`, `crates/<crate>/tests/`, `apps/api/src/`, `migrations/`, `scripts/`, `.github/workflows/`.

## Plan Task Mapping

| Original plan task | New tasks |
|---|---|
| P001 workspace and host builds | T006, T052, T053 |
| P002 dependency boundaries | T001, T002, T007 |
| P003 IDs, errors, configuration | T003, T004, T005 |
| P004 SQLx pool and migration runner | T008, T009, T010, T019 |
| P005 users/tenants/memberships migrations | T011, T012 |
| P006 repository integration tests | T013, T014, T015, T016, T017, T018 |
| P007 rustauth spike and ADR | T024 |
| P008 local auth/session lifecycle | T020-T032 |
| P009 membership-verified tenant context | T033-T039 |
| P010 RBAC evaluation and negative tests | T040-T048 |
| P001-P010 final validation | T049-T056 |

P011-P019 from the original plan are **delegated** and are not acceptance criteria for spec 001. See [Delegated Work](#delegated-work).

`Pxxx` = original plan task; `Txxx` = execution task in this file. Delegated work uses `P011-P019`.

Plan phases 0-6 map to task phases 1-6: plan Phase 0 → task Phase 1, plan Phase 1 → task Phase 2, plan Phase 2 → task Phase 3 (US1 includes the rustauth gate), plan Phases 3-5 → task Phases 3-5, plan Phase 6 → task Phase 6.

| Spec story | FRs | SCs |
|---|---|---|
| US1 | FR-013, FR-015, FR-017, FR-018 | SC-004, SC-005 |
| US2 | FR-004, FR-014, FR-016, FR-019 | SC-003, SC-006 |
| US3 | FR-005, FR-018 | SC-003 |

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Validate the workspace, enforce dependency boundaries, add shared primitives and prepare CI.

**Independent Test**: `bash scripts/tests/check-dependency-boundaries-test.sh` fails before the script exists and passes after; `cargo test -p platform-core` passes; the workspace still checks on the host and for wasm32.

- [ ] T001 Create a failing boundary fixture test in `scripts/tests/check-dependency-boundaries-test.sh` proving an upward crate dependency and a misplaced heavy dependency are rejected.
- [ ] T002 Implement `scripts/check-dependency-boundaries.sh` using `cargo metadata --no-deps` and `jq`; enforce L0 `platform-core`, L1 `contracts`/`domain`, L2 `application`, L3 capability crates, L4 `infrastructure`/apps, plus `sqlx` only in `persistence`/`infrastructure`, `dioxus` only in `ui`/apps, `axum` only in `apps/api`.
- [ ] T003 [P] Add `SessionId` and re-exports in `crates/platform-core/src/ids.rs` and `crates/platform-core/src/lib.rs`; add distinctness coverage in `crates/platform-core/tests/ids.rs`.
- [ ] T004 [P] Add validated `AppConfig` value objects in `crates/platform-core/src/config.rs`; tests in `crates/platform-core/tests/config.rs` must cover missing `DATABASE_URL`, invalid port, non-positive `SESSION_TTL_SECONDS`/`LOGIN_WINDOW_SECONDS`/`LOGIN_LOCKOUT_SECONDS`, and valid defaults.
- [ ] T005 [P] Add `.env.example` with `DATABASE_URL`, `APP_HOST=127.0.0.1`, `APP_PORT=8080`, `SESSION_TTL_SECONDS=2592000`, `LOGIN_MAX_ATTEMPTS=5`, `LOGIN_WINDOW_SECONDS=900`, `LOGIN_LOCKOUT_SECONDS=900`; contain no real secret.
- [ ] T006 Add `rustup target add wasm32-unknown-unknown` and `cargo check -p web --target wasm32-unknown-unknown` to `.github/workflows/ci.yml`; do not add desktop/mobile packaging claims.
- [ ] T007 Add `bash scripts/check-dependency-boundaries.sh` to `.github/workflows/ci.yml` after the host checks; remove `continue-on-error: true` from the `rustsec/audit-check@v2` step so `cargo audit` is blocking; add a CI guard that fails if the audit step regains `continue-on-error`; keep `cargo deny` blocking.

**Checkpoint**: Boundary and primitive checks are green; Phase 2 can start.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: PostgreSQL pool, forward-only migrations, the four-table schema and repository implementations. Every delivery story depends on this phase.

**⚠️ CRITICAL**: No US1/US2/US3 implementation starts until T008-T019 are complete.

**Independent Test**: `cargo test -p persistence` with `DATABASE_URL` set passes fresh-migration, re-run, uniqueness, atomic tenant creation, session validity and cross-tenant denial tests.

- [ ] T008 Add a `postgres:17` service with a `pg_isready` health check and `DATABASE_URL=postgres://saas:saas_dev_only@127.0.0.1:5432/saas` to the `rust` job in `.github/workflows/ci.yml`.
- [ ] T009 [P] Implement `PgPool` construction with a bounded acquire timeout in `crates/persistence/src/pool.rs`; map configuration/connection failures to safe errors.
- [ ] T010 [P] Implement the migration runner in `crates/persistence/src/migrations.rs` using `sqlx::migrate!` from the repository `migrations/` directory.
- [ ] T011 [P] Write `migrations/0001_foundation.sql` with `users`, `tenants`, `tenant_memberships` and `sessions`, including: `users.email` `NOT NULL UNIQUE` stored lowercase; `tenants.name` `CHECK (char_length(name) BETWEEN 1 AND 120)`; membership role `CHECK (role IN ('owner','admin','member','viewer'))`, status `CHECK (status IN ('invited','active','removed'))`, primary key `(user_id, tenant_id)`; `sessions.token_hash` 32-byte `BYTEA NOT NULL UNIQUE`, `expires_at CHECK (expires_at > created_at)`, nullable `revoked_at`; indexes from `data-model.md`.
- [ ] T012 [P] Update `migrations/README.md` to describe the four-table slice and mark `orgs`, roles/permissions tables, `audit_events` and `jobs` as deferred.
- [ ] T013 Add `sqlx::test` migration coverage in `crates/persistence/tests/migrations.rs`: a fresh database applies `0001_foundation.sql`, and a second migration run is a no-op.
- [ ] T014 [P] Implement `PgUserRepository` in `crates/persistence/src/users.rs` with `create`, `find_by_email` and `find_by_id`; duplicate email maps to a persistence `EmailTaken` error; tests in `crates/persistence/tests/users.rs`. The auth trait adapter is T032.
- [ ] T015 [P] Implement `PgTenantRepository` in `crates/persistence/src/tenants.rs`; `create_tenant_with_owner` must insert the tenant and an active `owner` membership in one transaction; tests in `crates/persistence/tests/tenants.rs`. The application trait adapter is T039.
- [ ] T016 [P] Implement `PgMembershipRepository` in `crates/persistence/src/memberships.rs`; every tenant-scoped query includes `tenant_id`; `change_role`/`remove_member` use a transaction with `SELECT ... FOR UPDATE`; enforce the last-owner guard and `(user_id, tenant_id)` uniqueness across statuses; tests in `crates/persistence/tests/memberships.rs`. The application trait adapter is T039.
- [ ] T017 [P] Implement `PgSessionRepository` in `crates/persistence/src/sessions.rs` with `create`, `find_by_token_hash`, `revoke` and `delete_expired`; tests in `crates/persistence/tests/sessions.rs` cover expired and revoked sessions. The auth trait adapter is T032.
- [ ] T018 Add cross-tenant read/write denial tests in `crates/persistence/tests/tenant_isolation.rs`, including an inactive/removed membership denial.
- [ ] T019 Export the persistence repositories and construct `PgPool`-backed repository values in `crates/persistence/src/lib.rs` and `crates/infrastructure/src/wiring/mod.rs`; add a compile-time wiring smoke test in `crates/infrastructure/tests/wiring.rs`; trait adapters are T032/T039.

**Checkpoint**: Schema and repositories are ready; US1 and US2 can start in parallel.

---

## Phase 3: US1 — Local Auth and Sessions (spec US1, Priority: P1)

**Goal**: A service-layer auth and session lifecycle with the rustauth gate, Argon2id passwords, opaque hashed tokens, login throttling and audit events. No HTTP routes.

**Independent Test**: `cargo test -p auth` with in-memory fakes proves registration, login, logout, token authentication, duplicate email, weak password, lockout, expired/revoked session, login/logout audit events and no secret logging.

### Tests for US1 (write first; confirm failure) ⚠️

- [ ] T020 [P] [US1] Write failing service tests in `crates/auth/tests/auth_service.rs` for register/login/logout/authenticate, duplicate email `EmailTaken`, password length 12-128 `WeakPassword`, invalid credentials, expired session and revoked session; record a non-gating login p95 benchmark; do not fail CI on wall-clock timing.
- [ ] T021 [P] [US1] Write failing throttle tests in `crates/auth/tests/login_throttle.rs` for 5 failures per 15 minutes, 15-minute lockout, successful-login reset and deterministic `Clock` behavior.
- [ ] T022 [P] [US1] Write failing audit tests in `crates/auth/tests/audit_events.rs` proving login success, login failure and logout events reach a recording application-owned `AuditSink` without secrets.
- [ ] T023 [P] [US1] Write failing redaction tests in `crates/auth/tests/redaction.rs` proving raw tokens, token hashes and password hashes never appear in `Debug` output or logs.

### Implementation for US1

- [ ] T024 [US1] Complete the rustauth spike and record the decision in `docs/architecture/adr/0001-rustauth.md`; verify `rustauth` 0.3 + `rustauth-sqlx` 0.3 license, maintenance, local password auth, session creation/revocation, Axum integration, SQLx/PostgreSQL compatibility and multi-tenant suitability. If the gate fails, stop and request a constitution amendment.
- [ ] T025 [US1] Add auth ports in `crates/auth/src/ports.rs`: `UserRepository`, `SessionRepository`, `Clock`, `PasswordHasher`, `TokenCodec`, `LoginThrottle`, plus `NewUser`/`User`/`NewSession`/`Session`; add `application`, `chrono` and `async-trait` dependencies to `crates/auth/Cargo.toml` and import the application-owned `AuditSink` from `crates/application/src/audit.rs`; add `sha2`, `rand` (or `getrandom`) and `base64`/`hex` dependencies for T027. T029 must land first; T025 must precede T026-T028.
- [ ] T026 [P] [US1] Implement the Argon2id `PasswordHasher` in `crates/auth/src/password.rs`, using `tokio::task::spawn_blocking`; never log passwords or hashes.
- [ ] T027 [P] [US1] Implement the opaque token codec in `crates/auth/src/token.rs`: 32 CSPRNG bytes, transport encoding, SHA-256 storage hash and redacted `Debug`.
- [ ] T028 [P] [US1] Implement the in-memory fixed-window `LoginThrottle` in `crates/auth/src/throttle.rs`, keyed by normalized email and driven by `Clock`.
- [ ] T029 [US1] Implement `AuditAction`, `AuditEvent`, `AuditOutcome`, `AuditSink` and `AuditError` in `crates/application/src/audit.rs`; add `chrono` and `async-trait` to `crates/application/Cargo.toml`; implement tracing and recording sinks in `crates/audit/src/lib.rs` and add `application` and `async-trait` to `crates/audit/Cargo.toml`; no passwords, raw tokens or token hashes. This application-owned port must land before T025.
- [ ] T030 [US1] Add `AuthError` variants `EmailTaken`, `WeakPassword`, `TooManyAttempts`, `SessionExpired`, `SessionRevoked`, `Repository(String)` and `Hashing(String)` in `crates/auth/src/lib.rs`.
- [ ] T031 [US1] Implement `AuthService` in `crates/auth/src/service.rs`: normalize email; register with password length 12-128; login with throttle and audit; logout revokes idempotently; authenticate rejects missing/revoked/expired sessions.
- [ ] T032 [US1] Implement the auth `UserRepository`/`SessionRepository` trait adapters over `PgUserRepository`/`PgSessionRepository` in `crates/infrastructure/src/wiring/auth.rs`; map persistence errors to `AuthError` (`EmailTaken`, `Repository`); add `application`, `auth`, `domain`, `tokio` and `async-trait` to `crates/infrastructure/Cargo.toml`; add integration coverage in `crates/infrastructure/tests/wiring.rs`; keep `crates/auth` independent of SQLx.

**Checkpoint**: US1 is independently testable at the service layer; US2 can proceed.

---

## Phase 4: US2 — Tenant Context and Tenant Creation (spec US2, Priority: P1)

**Goal**: A role-bearing `TenantContext` constructed only from an active membership, plus atomic tenant creation and repository-level tenant scoping.

**Independent Test**: `cargo test -p application -p persistence` proves active membership resolution, missing/inactive `NotMember`, role population, atomic tenant + owner creation and cross-tenant read/write denial.

### Tests for US2 (write first; confirm failure) ⚠️

- [ ] T033 [P] [US2] Write failing tenant-context tests in `crates/application/tests/tenant_context.rs`: active membership resolves with the correct role; missing, invited and removed memberships return `NotMember`.
- [ ] T034 [P] [US2] Write failing tenant-service tests in `crates/application/tests/tenant_service.rs`: tenant name bounds, atomic creation call, cross-tenant resolution denial and no tenant trust without membership.

### Implementation for US2

- [ ] T035 [P] [US2] Add `Role` and `MembershipStatus` enums in `crates/domain/src/role.rs` and `crates/domain/src/membership.rs`; extend `TenantContext` with `role: Role` in `crates/application/src/tenant_context.rs`; keep fields private and expose getters.
- [ ] T036 [US2] Update `crates/domain/src/lib.rs` so `Membership` carries `Role`/`MembershipStatus`; update `crates/application/src/ports.rs` so tenant-scoped methods require a verified `TenantContext`.
- [ ] T037 [US2] Implement `TenantService` in `crates/application/src/tenant_service.rs` with `create_tenant`, `resolve_context`, `change_role` and `remove_member`; delegate atomicity/locking to repositories.
- [ ] T038 [US2] Update `resolve_tenant_context` to accept the requested tenant ID only after an active membership lookup and return `ApplicationError::NotMember` for missing/inactive memberships.
- [ ] T039 [US2] Implement the application `MembershipReader`/`TenantRepository`/`MembershipRepository` adapters in `crates/infrastructure/src/wiring/tenancy.rs`; map persistence errors to `ApplicationError`; add `application`, `domain`, `tokio` and `async-trait` to `crates/infrastructure/Cargo.toml` if not already present; update `crates/tenancy/src/lib.rs` re-exports for `Role`, `TenantContext` and `TenantService`; extend `crates/infrastructure/tests/wiring.rs` and `crates/persistence/tests/tenant_isolation.rs` with tenant-context cross-tenant read/write cases.

**Checkpoint**: US2 is independently testable; US3 can start.

---

## Phase 5: US3 — RBAC and Membership Administration (spec US3, Priority: P1)

**Goal**: A deny-by-default `Role`/`Permission` matrix, owner/admin/member/viewer rules, last-owner protection, role changes, member removal and re-invite reactivation.

**Independent Test**: `cargo test -p domain -p application -p persistence` proves the full permission matrix, owner-only owner grants/removals, admin restrictions, last-owner denial, retained removed memberships, re-add reactivation and cross-tenant role-change denial.

### Tests for US3 (write first; confirm failure) ⚠️

- [ ] T040 [P] [US3] Write failing matrix tests in `crates/domain/tests/role_matrix.rs` for all 11 permissions × 4 roles; unknown roles/permissions are denied.
- [ ] T041 [P] [US3] Write failing authorization tests in `crates/application/tests/authorize.rs`: deny by default; only owner grants/removes owner; admin cannot delete/transfer or remove the last owner.
- [ ] T042 [P] [US3] Write failing membership-admin tests in `crates/persistence/tests/membership_admin.rs`: last-owner demotion/removal denial, re-add reactivation, removed-membership denial and cross-tenant role change denial.

### Implementation for US3

- [ ] T043 [P] [US3] Add `Permission` and `Role::allows` in `crates/domain/src/role.rs` using the matrix in `data-model.md`; `Role` is defined in T035.
- [ ] T044 [P] [US3] Add the deny-by-default `authorize` helper in `crates/application/src/authorize.rs`, returning `ApplicationError::Forbidden`.
- [ ] T045 [US3] Implement role change and member removal in `crates/application/src/tenant_service.rs` with permission checks and audit events through the application-owned `AuditSink`.
- [ ] T046 [US3] Implement the transaction, `SELECT ... FOR UPDATE` and last-owner guard in `crates/persistence/src/memberships.rs`.
- [ ] T047 [US3] Ensure removed memberships are retained and denied, and re-adding updates the same `(user_id, tenant_id)` row to active with the requested role; cover in `crates/persistence/tests/memberships.rs`.
- [ ] T048 [US3] Write failing audit tests in `crates/application/tests/audit_events.rs` for role change and member removal; then emit those events through the application-owned `AuditSink`; extend the existing application audit tests.

**Checkpoint**: All three delivery stories are independently functional.

---

## Phase 6: API Readiness, Polish & Cross-Cutting Concerns

**Purpose**: API readiness first, then full validation, honest reporting and delegated-work bookkeeping.

- [ ] T049 [P] Create `specs/001-platform-foundation/validation.md` with the exact commands and placeholders for results.
- [ ] T050 Write failing readiness tests in `apps/api/tests/health.rs`: `/health/live` is always 200; `/health/ready` is 200 only when the readiness port reports database reachable and migrations applied; 503 with `not_ready_database_unavailable` or `not_ready_migrations_failed` otherwise; a delayed readiness check returns 503 within the 2-second bound.
- [ ] T051 Implement `apps/api/src/state.rs` and `apps/api/src/health.rs`, and wire `apps/api/src/main.rs` to load config, build the pool through `persistence`, run the `persistence` migration runner, record migration state, and serve both endpoints per `contracts/health.md`; add `persistence` to `apps/api/Cargo.toml`; keep `sqlx` out of `apps/api`; make T050 pass.
- [ ] T052 Run `cargo fmt --all -- --check`, `cargo check --workspace`, `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings`; record actual results in `specs/001-platform-foundation/validation.md`.
- [ ] T053 Run `cargo check -p web --target wasm32-unknown-unknown`; record the result in `specs/001-platform-foundation/validation.md`; do not claim desktop/mobile packaging.
- [ ] T054 Run `cargo deny check`, `cargo audit` and `bash scripts/check-dependency-boundaries.sh`; confirm `.github/workflows/ci.yml` has no `continue-on-error` on the audit step; record results in `specs/001-platform-foundation/validation.md`.
- [ ] T055 Final scope review: confirm no delegated `P011-P019` work (API/UI/audit persistence/storage/jobs/AI/backup/packaging) was implemented; Phase 2 `T011-T019` schema/repository tasks are in scope. Confirm no desktop/mobile claims, no secrets or raw tokens in logs, and no unresolved `unwrap`/`expect`/panic in production paths; record in `specs/001-platform-foundation/validation.md`.
- [ ] T056 Update `docs/plans/implementation-roadmap.md` Phase 1a status and mark P011-P019 as delegated in `specs/001-platform-foundation/tasks.md`; do this only after T055 passes.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies; starts immediately.
- **Foundational (Phase 2)**: Depends on Phase 1; blocks all delivery stories.
- **US1 (Phase 3)** and **US2 (Phase 4)**: Both depend on Phase 2 and can run in parallel.
- **US3 (Phase 5)**: Depends on US1 (application-owned audit port) and US2 (`TenantContext`/`TenantService`).
- **Polish (Phase 6)**: T049 can start early; T050→T051; T052-T054 depend on T051 and all three stories; T055 depends on T052-T054; T056 depends on T055.

### Within Each Story

- Test tasks first; confirm they fail.
- Ports/types before implementations.
- Services before repository adapters.
- Repository transaction/locking before membership-admin integration tests.
- Do not mark a story complete until its independent test command passes.

### Parallel Opportunities

- Setup: T003, T004 and T005 can run in parallel; T001→T002 and T006→T007 are sequential.
- Foundational: T009-T012 can run in parallel; T014-T017 can run in parallel after T011; T013 depends on T010/T011; T018 depends on T014-T017; T019 depends on all repositories.
- US1: T020-T023 can run in parallel; T029 must precede T025; T025 must precede T026-T028; T026-T028 can run in parallel after T025; T030 follows T024/T025; T031 depends on T025-T030; T032 follows T031.
- US2: T033-T035 can run in parallel; T036 follows T035; T037/T038 follow; T039 follows T037/T038.
- US3: T040-T042 can run in parallel; T043/T044 can run in parallel; T045/T046 follow; T047/T048 follow.
- Polish: T049 can start early; T050→T051; T052-T054 after T051; T055→T056.

---

## Parallel Example: US1

```bash
# Tests first (different files):
Task: "Write failing service tests in crates/auth/tests/auth_service.rs"
Task: "Write failing throttle tests in crates/auth/tests/login_throttle.rs"
Task: "Write failing audit tests in crates/auth/tests/audit_events.rs and crates/application/tests/audit_events.rs"
Task: "Write failing redaction tests in crates/auth/tests/redaction.rs"

# Application audit port first, then auth ports, then independent implementations:
Task: "Implement application audit port and sinks in crates/application/src/audit.rs and crates/audit/src/lib.rs"
Task: "Add auth ports in crates/auth/src/ports.rs"
# After T025:
Task: "Implement Argon2id hasher in crates/auth/src/password.rs"
Task: "Implement token codec in crates/auth/src/token.rs"
Task: "Implement throttle in crates/auth/src/throttle.rs"
```

## Parallel Example: US2

```bash
Task: "Write failing tenant-context tests in crates/application/tests/tenant_context.rs"
Task: "Write failing tenant-service tests in crates/application/tests/tenant_service.rs"
Task: "Add Role/MembershipStatus and extend TenantContext in crates/domain/src and crates/application/src/tenant_context.rs"
# T036 follows T035:
Task: "Update Membership and application ports in crates/domain/src/lib.rs and crates/application/src/ports.rs"
```

## Parallel Example: US3

```bash
Task: "Write failing role matrix tests in crates/domain/tests/role_matrix.rs"
Task: "Write failing authorize tests in crates/application/tests/authorize.rs"
Task: "Write failing membership-admin tests in crates/persistence/tests/membership_admin.rs"
Task: "Add Role/Permission matrix in crates/domain/src/role.rs"
Task: "Add authorize helper in crates/application/src/authorize.rs"
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 (setup and boundaries).
2. Complete Phase 2 (schema and repositories).
3. Complete US1 (local auth and sessions).
4. **STOP and VALIDATE**: run `cargo test -p auth` and confirm the service-layer auth/session behavior.
5. Do not add HTTP auth endpoints; those remain P011/spec 002.

### Incremental Delivery

1. Setup + Foundational → workspace and schema ready.
2. US1 → local auth/session service, independently testable.
3. US2 → tenant context and atomic tenant creation.
4. US3 → RBAC and membership administration.
5. Polish → readiness, full validation and honest reporting.

US1 and US2 may run in parallel after Phase 2; US3 starts only after both, because it uses the application-owned audit port and `TenantService`.

---

## Delegated Work

The following original plan tasks are **not** part of spec 001 acceptance and must not be implemented in this branch:

- P011 Wire API handlers to application services.
- P012 Build sign-in, tenant switcher and dashboard shell.
- P013 Add audit events and sensitive action coverage.
- P014 Implement safe local file storage.
- P015 Evaluate and wire durable background jobs.
- P016 Implement local AI provider adapter and license docs.
- P017 Add backup/restore scripts and smoke test.
- P018 Validate web/desktop/mobile packaging on supported runners.
- P019 Add threat model and production readiness checklist.

Their owning specs and sequencing belong to P011-P019 / specs 002-016. P013 in this list refers to **audit persistence**; the application-owned audit port and tracing/recording sinks are in T022/T029/T048 and are part of 001.

- Email delivery adapter (verification/recovery) — owned by spec 002; no email provider is required for 001.

## Notes

- `[P]` means different files and no incomplete dependency; do not parallelize tasks that edit the same file.
- Every behavior change starts with a failing test and ends with the applicable `cargo test -p <crate>` green.
- No `unsafe`, `unwrap`, `expect` or panic in production paths.
- Never log passwords, raw session tokens or token hashes.
- Repository-level tenant scoping is mandatory; do not add RLS or `orgs` tables in this slice.
- The API remains a thin Axum entry point with health endpoints only; auth HTTP routes are P011/spec 002.
- Run the full validation suite after T051 (readiness implementation), not before; rerun it after any readiness fix.
