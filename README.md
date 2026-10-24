# Ferrugen

A reusable Rust workspace and self-hostable library base. The repository separates **reusable platform crates** from **runnable applications** and an example **Ferrugen template**.

## Non-negotiable principles

- MIT-licensed project code; audit every dependency and model license separately.
- Self-hostable core: no paid SaaS account is required for local accounts, PostgreSQL persistence, local file storage, API operation, or administration.
- Optional external integrations (OAuth, SMTP, hosted AI APIs) are opt-in and must be documented.
- Tenant membership is many-to-many: one user may belong to multiple tenants, and each tenant may have many users.
- Server-side authorization and tenant isolation; never trust tenant IDs supplied by a client without membership verification.
- Rust, Dioxus, Axum, Tokio, SQLx, PostgreSQL and `pnpm` only where a JS tool is later introduced.
- OpenCode is the primary coding agent. Spec Kit artifacts remain tool-independent.

## Workspace layout

```text
apps/
  api/              Axum HTTP API
  web/              Dioxus web frontend
  desktop/          Dioxus desktop frontend
  mobile/           Dioxus mobile frontend
  worker/           Background worker entry point
  admin-cli/        Local administration CLI
crates/
  platform-core/ contracts/ domain/ application/
  auth/ tenancy/ authorization/ audit/ billing/ i18n/
  persistence/ storage/ jobs/ sync/ integrations/ api-client/
  ai/ plugins/ ui/ design-system/ infrastructure/ test-support/
templates/ferrugen/
docs/
.specify/
.opencode/
```

## Quick start

Requirements: Rust toolchain from `rust-toolchain.toml`, Cargo, Docker Compose, and OpenCode. Web/desktop/mobile toolchains are platform-specific; see `docs/architecture/platform-matrix.md`.

```bash
cp .env.example .env
docker compose up -d postgres
cargo check -p platform-core -p contracts -p domain -p tenancy -p authorization
cargo test -p platform-core -p contracts -p domain -p tenancy -p authorization
cargo run -p api
```

The API starts on `127.0.0.1:8080` by default. `/health/live` is a liveness check and does not require a database. `/health/ready` reports whether required dependencies are ready; the starter currently keeps readiness conservative until DB wiring is implemented.

## Honest implementation status

This repository contains a wired workspace and starter interfaces, not a production-ready SaaS. The foundation crates contain initial implementations; authentication persistence, SQLx repositories, migrations, end-to-end tenant isolation, local AI execution, durable jobs, mobile packaging and full UI flows remain explicit implementation work. Do not deploy publicly until the security checklist and acceptance tests pass.

## Development workflow

1. One GitHub issue per bounded task; one branch per issue.
2. Use Spec Kit: `/speckit.specify`, `/speckit.plan`, `/speckit.tasks`, then `/speckit.implement` in OpenCode.
3. Keep changes small; add tests with behavior changes.
4. Run formatting, linting, tests and dependency/license checks.
5. Request independent code and security review.
6. PR must state `Closes #N`; squash merge only after CI success and approval.

See `docs/IMPLEMENTATION-ROADMAP.md`, `AGENTS.md`, `review.md`, `SECURITY.md` and `.opencode/agents/`.
