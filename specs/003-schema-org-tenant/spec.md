# Feature Specification: Multi-Tenant Schema (Users / Orgs / Tenants / Memberships)

**Feature Branch**: `003-schema-org-tenant`

**Created**: 2026-10-24

**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Independent user identity with optional org (P1)
**Independent Test**: Create user without org; register; view profile. User can later join org.

### User Story 2 - Multi-org / multi-tenant memberships (P1)
**Independent Test**: User joins org A and tenant T1 under A; access scoped to T1; no cross-tenant leakage.

---

## Requirements *(mandatory)*
- `users`: independent identity; can exist standalone.
- `orgs`: grouping; `created_by_user_id`; can contain multiple tenants.
- `tenants`: workspace; `org_id` NULL = standalone (solo user); under org when set.
- `memberships`: `user_id`, `org_id?`, `tenant_id?`, `role`, `status` (invited/active/removed); user can have multiple orgs/tenants.
- Every tenant-scoped query uses verified session context from `memberships`, never client-supplied ID.
- Cross-tenant denial tested before release (negative tests).

---

## Key Entities
- `users` / `orgs` / `tenants` / `memberships`
- `sessions` (server-side; verified user → session token; tenant scope resolved from session/membership)
- `audit_events` (privileged actions; no secret logging)

---

## Assumptions
- Schema supports GCP/GitHub-style model (independent user + group memberships).
- Migration framework uses SQLx.
