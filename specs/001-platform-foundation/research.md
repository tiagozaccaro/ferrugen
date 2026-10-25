# Phase 0 Research: Reusable SaaS Platform Foundation

**Feature**: `001-platform-foundation` | **Date**: 2026-10-25

All decisions below are scoped to P001-P010. P011-P019 remain delegated to their owning specs.

## 1. Build and target matrix

**Decision**: Validate host workspace formatting, check, test and clippy, plus `cargo check -p web --target wasm32-unknown-unknown`. Do not claim desktop or mobile packaging.

**Rationale**: The web app’s deployment target is wasm32, so a host-only check can miss wasm-specific failures. Desktop/mobile packaging requires platform runners and belongs to spec 016/P018.

**Alternatives considered**:
- Host-only checks: cheaper, but leaves the web app’s real target unvalidated.
- Full host/wasm/desktop/mobile matrix: correct long-term, but blocks the foundation on unavailable mobile toolchains.

## 2. Dependency-boundary enforcement

**Decision**: Add `scripts/check-dependency-boundaries.sh` using `cargo metadata --no-deps` and `jq`. Enforce layer direction and heavy-dependency placement.

Layers:
- L0: `platform-core`
- L1: `contracts`, `domain`
- L2: `application`
- L3: capability crates (`auth`, `tenancy`, `audit`, `persistence`, `storage`, `jobs`, `ai`, `plugins`, `ui`, `rate-limit`, `cache`, `backup`, `billing`, `i18n`, `sync`, `api-client`)
- L4: `infrastructure`, `apps/*`, `templates/ferrugen`

Rules:
- A crate may depend only on the same or a lower layer; no upward edges.
- `sqlx` may appear only in `persistence` and `infrastructure`.
- `dioxus` may appear only in `ui` and the web/desktop/mobile apps.
- `axum` may appear only in `apps/api`.

**Rationale**: Layer direction alone would still allow `domain` to import `sqlx` or `dioxus`. The heavy-dependency rules make the existing architecture comments enforceable.

**Alternatives considered**:
- Prose-only policy: already exists and has drifted.
- Full per-crate allowlist: precise but high-maintenance and likely to block legitimate same-layer edges.
- Cargo-deny bans: useful for external crates but not for workspace layer direction.

**Verification**: A fixture-based test runs the script against synthetic metadata containing an upward edge and a misplaced heavy dependency, and asserts a non-zero exit. The real workspace must pass.

## 3. IDs, errors and configuration

**Decision**: Keep `UserId` and `TenantId` newtypes in `platform-core`; add `SessionId`. Add validated configuration value objects in `platform-core`; load environment variables in `infrastructure` with `dotenvy` and `std::env`. No TOML configuration in this slice.

Required settings:
- `DATABASE_URL` — required; must start with `postgres://` or `postgresql://`.
- `APP_HOST` — default `127.0.0.1`.
- `APP_PORT` — default `8080`; must be 1-65535.
- `SESSION_TTL_SECONDS` — default 2,592,000 (30 days); must be positive.
- `LOGIN_MAX_ATTEMPTS` — default 5; must be positive.
- `LOGIN_WINDOW_SECONDS` — default 900; must be positive.
- `LOGIN_LOCKOUT_SECONDS` — default 900; must be positive.

**Rationale**: Environment variables are the simplest self-hosting interface and avoid a second configuration format. Validated value objects keep invalid settings out of the application.

**Alternatives considered**:
- `config` crate with TOML/environment layering: more flexible, but unnecessary for the foundation.
- Raw `std::env` reads in handlers: hard to test and easy to misuse.
- Defaulting `DATABASE_URL` in production: hides misconfiguration; use an explicit local quickstart value instead.

## 4. PostgreSQL and migrations

**Decision**: Use SQLx 0.8 with `runtime-tokio-rustls`, `postgres`, `uuid`, `chrono` and `migrate`. Run forward-only migrations from the repository `migrations/` directory. Use `sqlx::test` for integration tests against PostgreSQL 17.

**Rationale**: SQLx is already the workspace persistence dependency. Forward-only migrations are simpler and safer for a foundation; rollback is restore-based.

**Alternatives considered**:
- Diesel/SeaORM: additional abstraction and dependencies.
- Down migrations: operational complexity and rarely tested.
- In-memory SQLite for tests: would not validate PostgreSQL-specific constraints or `sqlx::test` behavior.

## 5. Auth library

**Decision**: Adopt `rustauth` 0.3 + `rustauth-sqlx` 0.3 only if T024 verifies license, maintenance, local password auth, session creation/revocation, Axum integration, SQLx/PostgreSQL compatibility and multi-tenant suitability. If the gate fails, stop and request a constitution amendment.

**Rationale**: The constitution locks rustauth, but the architecture overview explicitly says it is not assumed until the spike passes. A hard gate prevents a silent stack substitution.

**Alternatives considered**:
- `argon2` + custom session store: simplest fallback, but would require a constitution amendment because rustauth is locked.
- `axum-login`: session middleware, not a full auth service.
- `axum_session_auth`: session/auth integration, but still requires user/password storage decisions.

## 6. Password hashing

**Decision**: Use Argon2id with the `argon2` 0.5 crate and its default parameters unless the rustauth spike requires a specific PHC configuration. Store only the PHC hash string.

**Rationale**: Argon2id is the current password-hashing recommendation and is already a workspace dependency.

**Alternatives considered**: bcrypt (older, 72-byte limit), scrypt (fine but less common in Rust auth stacks), PBKDF2 (not preferred for new systems).

### Password policy

**Decision**: Reject passwords shorter than 12 characters or longer than 128 characters, returning `WeakPassword`; do not impose composition rules. This is a foundation baseline, not a product-specific policy.

**Rationale**: A length floor blocks trivially weak credentials without creating arbitrary character-class rules that harm usability.

## 7. Session tokens

**Decision**: Use opaque random tokens. Return the token to the client; store only a SHA-256 hash in `sessions.token_hash`, with `expires_at` and `revoked_at`. Default TTL 30 days, configurable. Transport (cookie or header) is a P011/spec 002 concern.

**Rationale**: Opaque server-side sessions support immediate revocation without JWT key rotation. SHA-256 is appropriate for high-entropy random tokens (not for passwords).

**Alternatives considered**:
- Signed JWT plus revocation list: stateless but adds key management and a revocation store anyway.
- Plaintext tokens at rest: simpler lookup, unacceptable secret handling.
- Argon2 for token hashes: unnecessary cost for 256-bit random tokens.

**Open implementation detail**: T024 must confirm whether rustauth’s `SessionStore` can store the hash/expiry/revocation fields or whether a thin adapter is required. The logical contract does not change.

## 8. Initial schema

**Decision**: Four tables: `users`, `tenants`, `tenant_memberships`, `sessions`.

- No `orgs`, `roles`, `permissions`, `role_permissions` or `membership_roles` tables in T011.
- `tenant_memberships` has one row per `(user_id, tenant_id)` across all statuses.
- Removed memberships are retained with status `removed`; re-adding reactivates the row.
- Role and status are constrained by `CHECK` constraints.
- Tenant ownership is an `owner` membership; no separate ownership column.

**Rationale**: The spec’s completion scope and clarifications explicitly defer orgs and normalized RBAC tables. A single role per membership is sufficient for the foundation.

**Alternatives considered**:
- Include orgs now: expands scope and duplicates spec 003.
- Normalized roles/permissions: flexible but unnecessary before a second use case.
- Separate ownership column: duplicates the owner membership and creates two sources of truth.

## 9. Tenant isolation

**Decision**: Enforce isolation at the repository boundary. Every tenant-scoped method requires a verified `TenantContext` and includes `tenant_id` in SQL. Do not enable PostgreSQL RLS in this slice.

**Rationale**: The security policy and AGENTS.md already require repository-boundary scoping. RLS adds session-variable and connection-reset complexity that is not justified for the foundation.

**Alternatives considered**:
- RLS only: strong defense but harder to test and easy to bypass if the session variable is not set.
- Both: best long-term, but deferred as hardening.

**Verification**: Integration tests create two tenants and assert that a context for tenant A cannot read or write tenant B data; membership resolution denies non-members and inactive memberships.

## 10. Tenant context selection

**Decision**: Accept a per-request tenant ID, but only after server-side membership verification. Do not store an active tenant in the session.

**Rationale**: This matches the existing `resolve_tenant_context` design and supports multiple tenants and browser tabs. The client ID is never trusted without a membership lookup.

**Alternatives considered**:
- Session-stored active tenant: simpler for a single-tab UI, but requires a switch endpoint and reduces flexibility.
- Both session default and request override: more state and more test cases than the foundation needs.

## 11. RBAC

**Decision**: A single role per membership: `owner`, `admin`, `member`, `viewer`. Permissions are compiled in code; deny by default.

Matrix:
- Owner: tenant read/update/delete/transfer, member management, settings, data read/write.
- Admin: member management, settings, data read/write; cannot grant/remove owner or delete/transfer the tenant.
- Member: data read/write.
- Viewer: data read only.

Additional rules:
- A tenant must always retain at least one active owner.
- Role changes and removals run in a transaction with `SELECT ... FOR UPDATE` on tenant memberships.
- Removed memberships are denied and retained; re-adding reactivates the same row.

**Rationale**: Matches the clarification answers and keeps authorization testable without a permission database.

**Alternatives considered**:
- Normalized roles/permissions: deferred.
- Multiple roles per membership: not needed for the foundation.
- Client-side permission checks: rejected; authorization is server-side.

## 12. Login throttling

**Decision**: Implement an auth-specific `LoginThrottle` port with an in-memory fixed-window implementation. Defaults: 5 failures per 15 minutes, 15-minute lockout, configurable. Key by normalized email; inject a `Clock` for deterministic tests.

**Rationale**: Spec 002 SC-004 requires brute-force protection, and the foundation auth service should not ship without it. An in-memory limiter is sufficient for a single-instance self-hosted deployment.

**Alternatives considered**:
- Database-backed limiter: survives restarts and works across instances, but adds schema and contention.
- Redis: not part of the self-hosted baseline.
- IP-based limiter only: weaker against distributed attempts and needs trusted proxy handling.
- Defer entirely: leaves local auth exposed.

**Limitation**: Per-process only; multi-instance deployments need a shared limiter later. Document this in the ADR/README.

## 13. Audit events

**Decision**: Define an application-owned `AuditSink` port and event types (`AuditAction`, `AuditEvent`, `AuditOutcome`, `AuditError`) in `crates/application`. `crates/auth` emits login/logout events, `application::TenantService` emits role-change/member-removal events, and `crates/audit` provides tracing and recording sinks. Persistence remains P013.

**Rationale**: Security-sensitive actions should be observable now without pulling audit persistence into the foundation slice. Keeping the port in `application` preserves the L2→L3 dependency direction: `auth` and `audit` may depend downward on `application`, but `application` must not depend on either capability crate. No secrets or raw tokens are logged.

**Alternatives considered**:
- Put the port in `auth`: forces `application` to depend on a capability crate and breaks the boundary.
- Put the port in `audit`: same boundary violation and couples application to audit.
- Defer all auditing to P013: leaves auth/role events invisible.
- Persist audit events now: expands schema/scope and duplicates P013.

## 14. Health readiness

**Decision**: `/health/live` always returns 200 while the process is alive. `/health/ready` returns 200 only when a bounded PostgreSQL `SELECT 1` succeeds and the migration runner has recorded success; otherwise 503 with a specific status string.

**Rationale**: Readiness should reflect the ability to serve tenant-scoped traffic, not just process liveness. A bounded check avoids hanging probes.

**Alternatives considered**:
- Process-only readiness: can mark a broken deployment healthy.
- Startup fail-fast only: liveness disappears when the database is down, making diagnosis harder.
- Readiness that always returns 200: violates FR-002.

## 15. CI and verification

**Decision**:
- Add PostgreSQL 17 service and `DATABASE_URL`.
- Add wasm32 target and web check.
- Add the boundary script.
- Make `cargo deny` and `cargo audit` blocking.
- Keep GTK/system libraries for host desktop checks.

**Rationale**: The foundation is not complete unless the checks that define it run in CI. The current audit step is non-blocking, which contradicts the dependency policy.

**Alternatives considered**:
- Keep audit non-blocking: hides advisories.
- Run Postgres only locally: integration tests would not run in CI.
- Skip wasm: leaves the web target unvalidated.

## 16. Test strategy

**Decision**: Failing tests first. Use unit tests for pure rules, `sqlx::test` for database behavior, contract tests for health, and fixture tests for the boundary script.

Required negative tests:
- Cross-tenant read and write denial.
- Inactive/removed membership denial.
- Last-owner demotion/removal denial.
- Expired and revoked session denial.
- Login lockout after the configured failure count.
- Duplicate email rejection.
- Readiness 503 when the database or migrations are unavailable.

**Rationale**: The constitution requires positive and negative tenant-isolation tests and testable behavior. Database constraints alone are not enough; service behavior must be asserted.

**Alternatives considered**:
- Mock-only repository tests: useful for units but insufficient for SQL predicates and constraints.
- Coverage percentage targets: not a substitute for behavior and negative-path tests.

## Deferred and open items

- rustauth schema compatibility and exact session-store API (T024).
- Password-hashing parameter tuning (T024/ADR).
- Session TTL product policy (default 30 days).
- Multi-instance throttling and shared cache (later spec).
- Tenant deletion/transfer HTTP endpoints (P011/spec 003).
- Audit persistence (P013).
- Orgs, RLS, normalized roles/permissions, jobs, AI, storage, backup, packaging (specs 002-016).
- Policy path mismatch in root `AGENTS.md` (`docs/security-policy.md` vs `docs/policies/security-policy.md`) — documentation cleanup outside P001-P010.
