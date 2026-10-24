# Database migrations

SQLx migrations will live here as numbered SQL files. The database schema is intentionally not claimed complete yet.

Planned initial tables:
- `users`
- `tenants`
- `tenant_memberships` (unique `(tenant_id, user_id)`)
- `roles`, `permissions`, `role_permissions`, `membership_roles`
- `sessions` or session storage tables, depending on the selected auth implementation
- `audit_events`
- `jobs`

Every tenant-owned table needs `tenant_id`, indexes and foreign keys appropriate to its access patterns. Add integration tests proving users cannot read or modify another tenant's rows.
