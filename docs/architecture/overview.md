# Architecture overview

## Shape
A modular monolith is the default: one API, one PostgreSQL database, one worker process and optional local AI runtime. Crates are modular boundaries, not separate network services unless a later measured need justifies it.

## Layers
- `platform-core`: IDs, errors, clocks and configuration primitives.
- `contracts`: API input/output types shared with clients.
- `domain`: entities and invariants, no HTTP/SQL/UI dependencies.
- `application`: use cases and ports/traits.
- capability crates: auth, tenancy, authorization, audit, billing, jobs, AI, storage, sync, plugins and integrations.
- `infrastructure`: concrete database, filesystem and provider adapters.
- `apps/*`: runtime entry points.
- `templates/ferrugen`: example product composition.

## Multi-tenancy
Users and tenants are separate records linked through memberships. A user can belong to many tenants; each tenant can have many users. Tenant selection is validated against membership. Repositories receive a verified tenant context for scoped operations. Consider PostgreSQL row-level security as defense in depth after application-level tenant scoping and transaction/session behavior are tested.

## Authentication
Local credentials are the baseline. External OAuth/OIDC are optional. RustAuth is not assumed until a spike verifies current license, maintenance, session model, storage support, Axum compatibility and multi-tenant integration. Do not duplicate a mature auth library unnecessarily, but keep application authorization and tenant policy owned by this project.

## AI
`ai` exposes provider-neutral interfaces. Local runtime adapters are the baseline path; hosted APIs are optional. A runtime library being permissively licensed does not make downloaded model weights permissively licensed.

## Background jobs
Use a durable PostgreSQL-backed job implementation. Evaluate Apalis against requirements (retry policy, uniqueness/idempotency, scheduling, observability, license and maintenance) before adopting it. Do not create a queue crate that is only an in-memory channel and call it durable.

## API
Axum handlers validate transport input and call application use cases. Do not put business rules directly in handlers. Keep a versioning strategy for public API contracts.
