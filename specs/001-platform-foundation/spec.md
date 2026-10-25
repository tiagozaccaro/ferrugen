# Feature Specification: Reusable SaaS Platform Foundation

## Goal
Provide a self-hostable, MIT-licensed Rust workspace with reusable SaaS capability crates (platform-core, contracts, domain, application, auth, tenancy, audit, billing, persistence, storage, jobs, ai, plugins, ui, i18n, sync, api-client, infrastructure, cache, rate-limit, backup) and Dioxus 0.7 multi-platform starter apps (api, web, desktop, mobile, worker, admin-cli). Architecture choices locked (rustauth, apalis, rig, wasmtime + WIT, pgvector, fastembed, tantivy); specs 002-016 defined; docs organized (`docs/index.md` + subfolders); OpenCode / Spec Kit artifacts define repeatable implementation.

**Completion scope:** spec 001 is complete when P001-P010 are implemented and verified. P011-P019 are delegated to their owning specs (002-016).

Task-ID convention: `Pxxx` refers to the original plan task; `Txxx` refers to the regenerated execution task in `tasks.md`.

## Clarifications

### Session 2026-10-25

First clarification pass.

- Q: What must be true for spec 001 to count as complete? → A: Foundation + tenancy slice: P001-P010 complete; P011-P019 delegated to later specs.
- Q: How should authorization grants be modeled for P010? → A: Single role enum per membership (owner/admin/member/viewer) with permissions compiled in code; deny by default.
- Q: What should `/health/ready` report as ready in the foundation slice? → A: 200 only when PostgreSQL is reachable and migrations are applied; 503 otherwise.
- Q: Which builds must P001 validate before spec 001 is complete? → A: Host workspace checks plus `cargo check -p web --target wasm32-unknown-unknown`; desktop/mobile packaging deferred to spec 016/P018.
- Q: What should P002's dependency-boundary check enforce? → A: Layer direction plus heavy-dependency placement: `sqlx` only in persistence/infrastructure, `dioxus` only in ui/apps, `axum` only in apps/api.

### Session 2026-10-25

Second clarification pass.

- Q: Should spec 001 include the `orgs` table and org memberships? → A: Tenants + memberships only; `orgs` and org membership are deferred to spec 003.
- Q: What should P008 deliver: service-layer auth primitives or working HTTP registration/login endpoints? → A: Service-layer auth and session persistence only; HTTP auth endpoints remain P011/spec 002.
- Q: Should tenant context come from a tenant ID supplied per request and verified against membership, or from an active tenant stored in the session? → A: Per-request tenant ID, always verified against session membership; no active tenant stored in the session.
- Q: How should tenant isolation be enforced in the foundation: repository-level scoping, PostgreSQL row-level security, or both? → A: Repository-level scoping; every tenant-scoped method requires a verified context and includes `tenant_id` in SQL, with cross-tenant integration tests.
- Q: How should local session tokens be represented and stored? → A: Opaque random token, stored as a SHA-256 hash in `sessions`, with expiry and revocation; transport is a P011 concern.

### Session 2026-10-25

Third clarification pass.

- Q: What should each foundation role be allowed to do? → A: Owner > admin > member > viewer: owner has full tenant control including deletion/transfer; admin manages members and settings; member reads/writes tenant data; viewer is read-only.
- Q: How should a tenant record its creator and owner? → A: Owner membership only; tenant creation writes the tenant and owner membership in one transaction, with no separate ownership column.
- Q: Should P008 include login throttling, or should rate limiting stay out of the foundation slice? → A: In scope; P008 adds configurable login-attempt throttling/lockout with tests.
- Q: Should P008 and P010 emit audit events for login, logout and role changes, or should auditing wait for P013? → A: Audit port + events; emit login/logout/role-change events, with persistence remaining P013.
- Q: Should a user have at most one membership per tenant? → A: Unique across all statuses; one `(user_id, tenant_id)` row, and re-inviting reactivates the existing membership.

## Users
- Application developer assembling new products from platform crates via `path` dependencies.
- Self-hosting operator deploying API/worker/PostgreSQL (self-hosted, no mandatory paid SaaS).
- Agent (opencode / other harness): reads `docs/index.md` first, then architecture/roadmap/specs/constitution before editing.

## User Scenarios & Testing

### US1 — Local auth and sessions (P1)
**Independent Test**: `cargo test -p auth` proves register/login/logout/authenticate, duplicate email, weak password, throttling, expiry/revocation and audit events at the service layer.
**Acceptance Scenarios**:
1. Given a new email/password, When the user registers and logs in, Then a session token is issued and only its SHA-256 hash is stored.
2. Given five failed logins within 15 minutes, When a sixth attempt occurs, Then it is denied with `TooManyAttempts`.
3. Given a revoked or expired session, When authenticate is called, Then it is rejected.

### US2 — Tenant context and creation (P1)
**Independent Test**: `cargo test -p application -p persistence` proves active membership resolution, missing/inactive `NotMember`, atomic tenant+owner creation and cross-tenant denial.
**Acceptance Scenarios**:
1. Given an active membership, When resolving context for that tenant, Then the context carries the correct role.
2. Given no membership or a removed membership, When resolving that tenant, Then `NotMember` is returned.
3. Given tenant creation, When it succeeds, Then the tenant and owner membership are committed atomically.

### US3 — RBAC and membership administration (P1)
**Independent Test**: `cargo test -p domain -p application -p persistence` proves the full role matrix, last-owner guard, owner-only owner grants/removals, admin limits, removed-membership denial/re-invite and audit events.
**Acceptance Scenarios**:
1. Given a viewer, When the viewer attempts a write, Then it is denied.
2. Given the last active owner, When demotion/removal is attempted, Then it is denied.
3. Given a removed membership, When the user is re-invited, Then the same `(user_id, tenant_id)` row becomes active.

## Functional requirements
- FR-001: Workspace has separate reusable crates and runnable apps.
- FR-002: API exposes liveness and readiness endpoints; readiness returns 200 only when PostgreSQL is reachable and migrations are applied, otherwise 503.
- FR-003: Tenant membership model supports many-to-many user/tenant relationships, with one role per membership; membership status is one of `invited`, `active`, `removed` (inactive means `invited` or `removed`); `orgs` and org membership are deferred to spec 003.
- FR-004: Tenant-scoped operations require verified tenant context; a client-supplied tenant ID is accepted only after server-side membership verification, and no active tenant is stored in the session.
- FR-005: Authorization defaults to deny when no matching grant exists; foundation roles are `owner`, `admin`, `member` and `viewer`, mapped to permissions in code. Owner has full tenant control including deletion/transfer; admin manages members and settings; member reads/writes tenant data; viewer is read-only.
- FR-006: Core local development does not require third-party SaaS credentials.
- FR-007: Optional auth, AI, email and storage providers are disabled when unconfigured and sit behind replaceable ports; spec 001 implements the auth provider boundary (optional provider disabled when unconfigured) and the application-owned audit port (FR-018). AI, email and storage delivery remain delegated to their owning specs, and no optional provider is required for local core operation.
- FR-008: OpenCode instructions and Spec Kit artifacts define a repeatable implementation process.
- FR-009: CI runs `cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, `cargo deny check`, `cargo audit`; new crates/dependencies require license and advisory review and must not introduce heavy dependencies outside their designated crates.
- FR-010: Spec 001 completion covers P001-P010 only; P011-P019 are tracked by their owning specs and are not 001 acceptance criteria.
- FR-011: P001 validates host workspace formatting/check/test/clippy plus `cargo check -p web --target wasm32-unknown-unknown`; desktop and mobile packaging validation is deferred to spec 016/P018.
- FR-012: P002 enforces layer direction plus heavy-dependency placement: `sqlx` only in persistence/infrastructure, `dioxus` only in ui/apps, and `axum` only in apps/api.
- FR-013: P008 delivers application/service-layer auth and session persistence; registration requires a password of 12-128 characters; HTTP auth endpoints and route wiring remain P011/spec 002.
- FR-014: Tenant isolation is enforced at the repository boundary: every tenant-scoped method requires a verified tenant context and includes `tenant_id` in its SQL; cross-tenant integration tests prove denial.
- FR-015: Local sessions use opaque random tokens; only a SHA-256 hash is stored in `sessions`, with expiry and revocation; token transport is a P011 concern.
- FR-016: Tenant ownership is represented solely by an `owner` membership; tenant creation writes the tenant and owner membership atomically; tenant names are 1-120 characters.
- FR-017: P008 includes configurable login-attempt throttling/lockout, tested at the service layer.
- FR-018: P008 and P010 emit audit events for login, logout, role changes and member removal through an audit port; persistence remains P013.
- FR-019: A user has at most one membership per tenant, unique across all statuses (`invited`, `active`, `removed`); removed memberships are retained with status `removed` and denied access, and re-inviting reactivates the existing row.

## Success criteria
- SC-001: A fresh database applies `0001_foundation.sql`; a second run is a no-op.
- SC-002: `/health/live` returns 200 in all tests; `/health/ready` returns 200 only with a reachable PostgreSQL and applied migrations, and 503 within 2 seconds otherwise.
- SC-003: 100% of cross-tenant read/write negative cases are denied; no cross-tenant row is returned. Negative cases include read, write, role change, member removal and tenant-context resolution.
- SC-004: The sixth login attempt within 15 minutes is blocked; the account unlocks after 15 minutes with a deterministic Clock.
- SC-005: Local login p95 is under 500 ms on the supported runner; if CI timing is unstable, record the measurement as an informational benchmark and gate on correctness tests only.
- SC-006: A user has at most one membership per tenant across all statuses; re-invite reactivates the same row.
- SC-007: `cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, wasm web check, boundary script, `cargo deny` and `cargo audit` pass in CI.

## Non-functional requirements
- NFR-001: MIT project license with dependency license audit.
- NFR-002: No production path uses panic/unwrap/expect.
- NFR-003: Cross-tenant access is covered by negative tests before release.
- NFR-004: PostgreSQL is not exposed publicly by default.
- NFR-005: Unimplemented capabilities clearly marked as scaffolds (`scaffold` / `todo` / `not implemented`).
- NFR-006: All docs agent-compatible (`docs/index.md` + subfolder READMEs); instructions readable by opencode / Claude / GPT.

## Out of scope for foundation
- Complete production authentication flows beyond the local session lifecycle in P008 (OAuth, recovery, email verification, MFA — 002 auth spec defines scope).
- Full production schema beyond the initial foundation tables in P005 (003 schema spec defines the complete design; later migrations deferred).
- Billing provider / paid services (specified in 005 / phase 5; optional only).
- Hosted AI dependency (rig provides local adapter baseline; hosted APIs optional behind feature gates; model weight licenses separate).
- Arbitrary in-process untrusted plugin execution (005 defines WASM sandbox with Wasmtime + WIT; native trait plugins for trusted only; process/container isolation deferred for genuinely untrusted).
- Certified mobile/desktop builds for every target (Phase 7; packaging validated when runners/toolchains support).
- Implementing P011-P019 (API wiring, UI shell, audit persistence, storage, jobs, AI adapter, backup, packaging) — delegated to specs 002-016; spec 001 covers P001-P010.
- Organization hierarchy (`orgs`, org membership, org-level roles) — deferred to spec 003; 001 covers standalone tenants and tenant memberships.
