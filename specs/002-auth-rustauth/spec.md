# Feature Specification: Multi-Tenant Auth with rustauth (Local + OAuth)

**Feature Branch**: `002-auth-rustauth`

**Created**: 2026-10-24

**Status**: Draft

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP that delivers value.
-->

### User Story 1 - Registered local account (Priority: P1)

**Why this priority**: Core baseline — a user signs up locally and logs in to access the platform alone.

**Independent Test**: A tester can complete registration → login → profile view and delivers personal access without org or tenant context.

**Acceptance Scenarios**:

1. **Given** an email/password pair, **When** the user registers and logs in, **Then** the server returns a session token valid for tenant scoping and cross-tenant denial applies.
2. **Given** a second user with the same email domain, **When** both register, **Then** the first login succeeds only after the second is rejected at uniqueness check.

### User Story 2 - Multi-org memberships resolved from verified session (Priority: P1)

**Why this priority**: Tenant isolation correctness depends on server-resolved membership, not client-supplied tenant IDs.

**Independent Test**: Given a logged-in user who belongs to two orgs, `GET /api/memberships` returns both memberships; a query scoped to one tenant returns only that tenant's records.

**Acceptance Scenarios**:

1. **Given** a session for a user in org A and org B, **When** listing members of org B, **Then** only org B members are returned — no cross-org leak.
2. **Given** an unauthenticated request with a client-supplied tenant ID, **When** fetching data for that tenant, **Then** 403 without verified session membership.

### User Story 3 - OAuth provider sign-in (Priority: P2)

**Why this priority**: External identities connect the same RBAC/tenant model as local accounts.

**Independent Test**: Configure an OAuth provider; openid callback → claimed identity registered or linked to existing user → issued session token.

**Acceptance Scenarios**:

1. **Given** enabled provider config, **When** the user redirects and authorizes, **Then** the server issues a local session without leaking provider scopes.
2. **Given** an unverified email claiming the same identity, **When** linking, **Then** require confirmation before existing account attachment.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Local registration creates a unique `users` row with hashed credentials; login verifies credential + password with rate limiting.
- **FR-002**: Session is stored server-side keyed by verified `users.id`; tenant scope resolves from session membership, never client-supplied.
- **FR-003**: Membership lookup (`users` × `orgs`, `users` × `tenants`) is verified before tenant-scoped query.
- **FR-004**: OAuth providers are optional adapters behind trait; deny core operation when unconfigured.
- **FR-005**: Password recovery, reset tokens with expiry, and session revocation (logout).
- **FR-006**: Cross-tenant access denied by default; negative test suite required before release.

### Key Entities

- **users**: independent identity; can exist standalone or via membership.
- **sessions**: verified user → server-issued token; per-session expiry and revocation.
- **tenants** / **orgs** / **memberships**: tenant context resolved from verified membership (see Phase 3 spec).

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Registration/login latency under 500ms for local flow at p95 on supported runner.
- **SC-002**: Zero cross-tenant leakage under randomized multi-user sessions in integration tests.
- **SC-003**: OAuth adapter can be added behind trait without touching auth crate.
- **SC-004**: Rate limit blocks brute-force lockout within configured window.

---

## Assumptions

- Login uses transport input already validated by Axum middleware.
- OAuth adapters implemented only when provider configured; no secret storage beyond managed env.
