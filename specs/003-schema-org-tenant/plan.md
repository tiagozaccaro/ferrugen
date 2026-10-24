# Plan: Schema
- Migration skeleton (`migrations/`) for `users`, `orgs`, `tenants`, `memberships`, `sessions`, `audit_events`.
- `platform-core`: typed IDs (`UserId`, `OrgId`, `TenantId`).
- `domain`: membership invariants (one active membership per user/org/tenant combo).
- Tests: cross-tenant denial (negative); membership creation/lookup.
