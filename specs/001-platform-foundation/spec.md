# Feature Specification: Reusable SaaS Platform Foundation

## Goal
Provide a self-hostable, permissively licensed Rust workspace with reusable SaaS capability crates and a Dioxus multi-platform starter.

## Users
- Application developer assembling new products from platform crates.
- Self-hosting operator deploying the API, worker and PostgreSQL.
- Tenant administrator managing a tenant and memberships.
- Member who can belong to multiple tenants.

## Functional requirements
- FR-001: Workspace has separate reusable crates and runnable apps.
- FR-002: API exposes liveness and readiness endpoints.
- FR-003: Tenant membership model supports many-to-many user/tenant relationships.
- FR-004: Tenant-scoped operations require verified tenant context.
- FR-005: Authorization defaults to deny when no matching grant exists.
- FR-006: Core local development does not require third-party SaaS credentials.
- FR-007: Optional auth, AI, email and storage providers are replaceable adapters.
- FR-008: OpenCode instructions and Spec Kit artifacts define a repeatable implementation process.
- FR-009: CI runs format, check, tests, clippy and dependency policy checks.

## Non-functional requirements
- NFR-001: MIT project license with dependency license audit.
- NFR-002: No production path uses panic/unwrap/expect.
- NFR-003: Cross-tenant access is covered by negative tests before release.
- NFR-004: PostgreSQL is not exposed publicly by default.
- NFR-005: Unimplemented capabilities are clearly marked as scaffolds.

## Out of scope for foundation
Production authentication flows, complete database schema, billing provider, hosted AI dependency, arbitrary in-process plugin execution, and certified builds for every mobile target.
