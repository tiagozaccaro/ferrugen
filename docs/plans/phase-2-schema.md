# Phase 2 — Multi-tenant schema (locked hierarchy)

Based on GCP / GitHub model: users are independent; orgs are groups; tenants are scoped workspaces; memberships link them.

## Tables (conceptual — to become SQLx migrations)

- `users` (id, email, display_name, created_at, updated_at) — independent identity; can exist without org/tenant.
- `orgs` (id, name, slug, created_by_user_id, created_at) — grouping / billing container.
- `tenants` (id, org_id?, name, slug, created_by_user_id, created_at) — workspace; `org_id` NULL = standalone (solo user). Tenant bounded by org when present.
- `memberships` (id, user_id, org_id?, tenant_id?, role, status, invited_by, invited_at, joined_at, removed_at) — links user to org and/or tenant. `org_id` / `tenant_id` independently nullable so a user can be in an org but not its tenants, or in a tenant directly.
- `roles` / `permissions` — RBAC; resource checks server-side; deny by default.
- `sessions` — secure session storage linked to verified user + resolved tenant context (not client-supplied tenant ID alone).
- `audit_events` — privileged actions and credential events without secrets.

## Security rules (from AGENTS.md / overview.md)
- Resolve tenant context from verified membership/session, not client-supplied tenant ID.
- Every tenant-scoped query tenant-scoped at repository boundary.
- Cross-tenant read/write denial tests required.

## Next vertical slice
- Create migration skeleton (`crates/infrastructure` or `apps/`) with these tables.
- Add `rustauth` local registration / login / OAuth adapter wiring (Phase 3 overlap).
