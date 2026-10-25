# Phase 1 Data Model: Foundation and Tenancy Slice

**Feature**: `001-platform-foundation` | **Date**: 2026-10-25

This model covers P001-P010. `orgs`, normalized roles/permissions, audit persistence and jobs are explicitly deferred.

## Entities

### `users`

| Column | Type | Constraints |
|---|---|---|
| `id` | `UUID` | Primary key; generated in Rust |
| `email` | `TEXT` | `NOT NULL`, `UNIQUE`; stored normalized to lowercase |
| `display_name` | `TEXT` | Nullable |
| `password_hash` | `TEXT` | `NOT NULL`; Argon2id PHC string |
| `created_at` | `TIMESTAMPTZ` | `NOT NULL DEFAULT now()` |
| `updated_at` | `TIMESTAMPTZ` | `NOT NULL DEFAULT now()` |

Rules:
- Email is normalized (trimmed, lowercased) before insert; uniqueness is case-insensitive by normalization.
- No plaintext password or password-derived value is stored outside `password_hash`.
- A user may exist without any tenant membership.

Indexes:
- `users_email_key` unique on `email`.
- Primary key on `id`.

### `tenants`

| Column | Type | Constraints |
|---|---|---|
| `id` | `UUID` | Primary key; generated in Rust |
| `name` | `TEXT` | `NOT NULL`, `CHECK (char_length(name) BETWEEN 1 AND 120)` |
| `created_at` | `TIMESTAMPTZ` | `NOT NULL DEFAULT now()` |
| `updated_at` | `TIMESTAMPTZ` | `NOT NULL DEFAULT now()` |

Rules:
- No `org_id` in this slice; org hierarchy is deferred to spec 003.
- No `created_by_user_id` or ownership column; ownership is the `owner` membership (FR-016).
- Tenant creation inserts the tenant and owner membership in one transaction.

Indexes:
- Primary key on `id`.

### `tenant_memberships`

| Column | Type | Constraints |
|---|---|---|
| `user_id` | `UUID` | `NOT NULL REFERENCES users(id) ON DELETE CASCADE` |
| `tenant_id` | `UUID` | `NOT NULL REFERENCES tenants(id) ON DELETE CASCADE` |
| `role` | `TEXT` | `NOT NULL CHECK (role IN ('owner','admin','member','viewer'))` |
| `status` | `TEXT` | `NOT NULL CHECK (status IN ('invited','active','removed'))` |
| `created_at` | `TIMESTAMPTZ` | `NOT NULL DEFAULT now()` |
| `updated_at` | `TIMESTAMPTZ` | `NOT NULL DEFAULT now()` |

Primary key: `(user_id, tenant_id)`.

Rules:
- At most one row per user/tenant, across all statuses (FR-019).
- `removed` memberships are retained; access is denied.
- Re-adding a removed user updates the existing row to an active status and the requested role.
- `invited` is a valid stored status for future invitation flows; P005 adds existing users as `active` unless a later spec introduces invitations.
- Role changes and removals must preserve at least one active `owner` per tenant.
- A user may have memberships in multiple tenants.

Indexes:
- Primary key `(user_id, tenant_id)`.
- `tenant_memberships_tenant_status_idx` on `(tenant_id, status)`.
- Foreign-key indexes on `user_id` and `tenant_id` are covered by the primary key prefix and the tenant/status index.

### `sessions`

| Column | Type | Constraints |
|---|---|---|
| `id` | `UUID` | Primary key; generated in Rust |
| `user_id` | `UUID` | `NOT NULL REFERENCES users(id) ON DELETE CASCADE` |
| `token_hash` | `BYTEA` | `NOT NULL`, `UNIQUE`; 32-byte SHA-256 digest |
| `created_at` | `TIMESTAMPTZ` | `NOT NULL DEFAULT now()` |
| `expires_at` | `TIMESTAMPTZ` | `NOT NULL`, `CHECK (expires_at > created_at)` |
| `revoked_at` | `TIMESTAMPTZ` | Nullable; non-null means revoked |

Rules:
- The raw token is never stored; only the SHA-256 hash is stored.
- A session is valid only when `revoked_at IS NULL` and `expires_at > now()`.
- Logout sets `revoked_at`; expired sessions are rejected and may be cleaned up later.
- Token transport (cookie/header) is a P011/spec 002 concern.

Indexes:
- Primary key on `id`.
- Unique index on `token_hash`.
- `sessions_user_id_idx` on `user_id`.
- `sessions_expires_at_idx` on `expires_at` for cleanup.

## Relationships

```text
users 1 ─── * tenant_memberships * ─── 1 tenants
users 1 ─── * sessions
```

- Deleting a user cascades to memberships and sessions.
- Deleting a tenant cascades to memberships. Tenant deletion is not exposed in P001-P010; the RBAC permission is defined, but the HTTP operation is P011/spec 003.
- No org relationship in this slice.

## Role and permission model

`Role` values: `owner`, `admin`, `member`, `viewer`.

| Permission | Owner | Admin | Member | Viewer |
|---|---:|---:|---:|---:|
| `TenantRead` | yes | yes | yes | yes |
| `TenantUpdate` | yes | yes | no | no |
| `TenantDelete` | yes | no | no | no |
| `TenantTransfer` | yes | no | no | no |
| `MemberRead` | yes | yes | no | no |
| `MemberAdd` | yes | yes | no | no |
| `MemberRemove` | yes | yes | no | no |
| `MemberRoleChange` | yes | yes (non-owner roles) | no | no |
| `SettingsManage` | yes | yes | no | no |
| `DataRead` | yes | yes | yes | yes |
| `DataWrite` | yes | yes | yes | no |

Rules:
- Authorization is deny-by-default: an unknown role or permission is denied.
- Only an owner may grant or remove the `owner` role.
- Admin cannot delete or transfer the tenant and cannot remove the last owner.
- Every tenant must retain at least one active `owner`; the last-owner guard is enforced inside a transaction with row locks.
- The matrix is compiled in code; it is not stored in database tables in this slice.

## Membership lifecycle

```text
(no row) --add--> active
(no row) --invite--> invited          (future invitation flow; not implemented in 001)
invited --accept--> active            (future)
active --remove--> removed
removed --re-add--> active
removed --re-invite--> invited        (future)
```

- `removed` rows are retained and denied access.
- Re-adding a removed membership updates the existing `(user_id, tenant_id)` row; it does not insert a duplicate.
- `invited` rows are denied access until activated.
- A role change does not change status.

## Tenant context

`TenantContext` contains:
- `user_id: UserId`
- `tenant_id: TenantId`
- `role: Role`

Construction rules:
- Only `resolve_tenant_context(reader, user_id, requested_tenant)` constructs it.
- The membership lookup must return an `active` membership for that exact user/tenant pair.
- Missing and inactive memberships both return `ApplicationError::NotMember` to avoid leaking membership status.
- The requested tenant ID is accepted only after the lookup succeeds; it is never trusted on its own.

## Session and token model

- Token: 32 random bytes from a CSPRNG, encoded for transport (hex or base64url).
- Storage: `SHA-256(token)` as 32 bytes in `sessions.token_hash`.
- Expiry: `now + SESSION_TTL_SECONDS` (default 30 days).
- Revocation: `revoked_at` set by logout or administrative revocation.
- Validation: hash the presented token, look up the hash, reject if missing, revoked or expired.
- Logging: never log the raw token or the hash; `SessionToken` uses a redacted `Debug` implementation.

## Migration and test notes

- Migration file: `migrations/0001_foundation.sql`.
- Use PostgreSQL 17; `gen_random_uuid()` is available if needed, but IDs are generated in Rust.
- `sqlx::test` applies migrations from `../../migrations` relative to `crates/persistence`.
- Integration tests must cover:
  - Fresh migration succeeds; re-running the migrator is a no-op.
  - Membership uniqueness across statuses.
  - Tenant creation writes tenant + owner membership atomically.
  - Cross-tenant read/write denial.
  - Inactive/removed membership denial.
  - Last-owner demotion/removal denial.
  - Expired/revoked session denial.
