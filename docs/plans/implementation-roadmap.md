# Implementation roadmap

## Phase 0 — Workspace and governance
- [x] Define workspace, reusable crate boundaries and app entry points.
- [x] Add MIT license, OpenCode agents, Spec Kit memory, CI, dependency policy and self-hosting docs.
- [ ] Run `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` and resolve compile issues on supported host.
- [ ] Replace repository URL and contributor copyright holder.

## Phase 1 — Foundation
- [ ] Stabilize platform-core error, ID, clock and configuration APIs.
- [ ] Define contracts and domain entity IDs.
- [ ] Define application ports and dependency direction.
- [ ] Add test-support helpers and architecture boundary checks.

## Phase 2 — Persistence and migrations
- [ ] Add SQLx PostgreSQL connection pool and migration runner.
- [ ] Create users, tenants, memberships, roles, permissions, sessions and audit tables.
- [ ] Add indexes, foreign keys, unique constraints and migration rollback/recovery notes.
- [ ] Add integration tests using local PostgreSQL.

## Phase 3 — Authentication and tenant security
- [x] Lock auth: `rustauth` (local + OAuth) — dependency record added.
- [x] Lock jobs: `apalis` + PostgreSQL — dependency record added.
- [x] Lock AI: `rig` (chat + agent) — dependency record added.
- [x] Lock plugins: native traits + Wasmtime + WIT — dependency record added.
- [x] Lock hierarchy: GCP/GitHub-style users/orgs/tenants/memberships.
- [ ] Implement local registration/login/logout, secure sessions, recovery and rate limits.
- [ ] Implement membership-validated tenant selection.
- [ ] Implement RBAC and resource checks.
- [ ] Add cross-tenant access regression suite.

## Phase 4 — API and web vertical slice
- [ ] Wire API routes to application services and PostgreSQL.
- [ ] Implement sign-in, tenant switcher, dashboard shell, profile and member management.
- [ ] Add typed API client and contract tests.
- [ ] Implement consistent errors, tracing, request IDs and health/readiness.

## Phase 5 — Platform capabilities
- [ ] Audit trail, localization, file storage, background jobs and settings.
- [ ] Plans/entitlements and usage limits without requiring a payment provider.
- [ ] Email interface with local development sink and optional SMTP.
- [ ] Webhook signature verification and idempotency.

## Phase 6 — AI and extensions
- [x] AI = `rig` (chat + agent); plugin model = native traits + Wasmtime + WIT (architecture locked in `docs/ARCHITECTURE-CHOICES.md`).
- [x] Provider-neutral streaming contracts adapted to rig.
- [x] Tool permission model, prompt/version tracking and telemetry planned.
- [x] Background jobs = `apalis`; auth = `rustauth`. Spikes complete; see `docs/ARCHITECTURE-CHOICES.md`.

## Phase 7 — Desktop and mobile
- [ ] Validate Dioxus target setup on Linux/macOS/Windows and Android/iOS.
- [ ] Shared navigation and UI, platform-specific packaging, secure token/session storage.
- [ ] Offline data model, sync protocol, conflict policy and device tests.

## Phase 8 — Operations and release
- [ ] Docker Compose for API, worker, PostgreSQL and optional local AI.
- [ ] Backup/restore scripts, retention, observability and upgrade documentation.
- [ ] Threat model, dependency/license audit, SBOM and release checklist.
- [ ] CI matrix for backend, web, desktop and mobile where runners/toolchains support them.
- [ ] Complete acceptance criteria before describing the starter as production-ready.
