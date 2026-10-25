# Quickstart: Foundation and Tenancy Slice

**Feature**: `001-platform-foundation` | **Date**: 2026-10-25

This guide validates P001-P010 locally. It does not include HTTP auth endpoints, UI, audit persistence, storage, jobs, AI, backup or packaging; those are P011-P019.

## Prerequisites

- Rust 1.89.0 via `rustup` (the repository pins it in `rust-toolchain.toml`).
- Docker with Compose.
- `jq` for the dependency-boundary script.
- Optional: `cargo-deny` and `cargo-audit` for local policy checks.

```bash
rustc --version
docker compose version
jq --version
```

## 1. Start PostgreSQL

```bash
docker compose up -d postgres
docker compose ps
```

The Compose service binds PostgreSQL to `127.0.0.1:5432` only. It is not publicly exposed.

## 2. Configure the environment

```bash
export DATABASE_URL='postgres://saas:saas_dev_only@127.0.0.1:5432/saas'
export APP_HOST='127.0.0.1'
export APP_PORT='8080'
```

Optional auth/throttle settings (defaults shown):

```bash
export SESSION_TTL_SECONDS='2592000'   # 30 days
export LOGIN_MAX_ATTEMPTS='5'
export LOGIN_WINDOW_SECONDS='900'
export LOGIN_LOCKOUT_SECONDS='900'
```

`DATABASE_URL` is required. Do not commit a real database URL or password; `.env.example` contains local-development placeholders only.

## 3. Run the API and health checks

```bash
cargo run -p api
```

The API starts on `http://127.0.0.1:8080`, runs the SQLx migrations, and exposes:

```bash
curl -i http://127.0.0.1:8080/health/live
curl -i http://127.0.0.1:8080/health/ready
```

Expected:
- `/health/live` -> `200` with `{"status":"ok","version":"..."}`.
- `/health/ready` -> `200` with `{"status":"ready","version":"..."}` when PostgreSQL is reachable and migrations are applied.
- If PostgreSQL is unavailable, `/health/live` stays `200` and `/health/ready` returns `503` with `not_ready_database_unavailable`.
- If migrations failed, `/health/ready` returns `503` with `not_ready_migrations_failed`.

There are no `/auth/register`, `/auth/login` or `/auth/logout` HTTP endpoints in this slice. Auth is service-layer only; P011/spec 002 wires HTTP.

## 4. Run the required checks

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Database tests use `sqlx::test` and require the `DATABASE_URL` above. The Compose `saas` user can create the temporary databases that `sqlx::test` needs.

## 5. Validate the web target

```bash
rustup target add wasm32-unknown-unknown
cargo check -p web --target wasm32-unknown-unknown
```

Desktop and mobile packaging are deliberately not validated here; they are deferred to spec 016/P018.

## 6. Validate dependency boundaries

```bash
bash scripts/check-dependency-boundaries.sh
```

The script must pass for the current workspace. It rejects:
- An upward dependency between architecture layers.
- `sqlx` outside `persistence`/`infrastructure`.
- `dioxus` outside `ui`/web/desktop/mobile apps.
- `axum` outside `apps/api`.

A fixture-based test under `scripts/tests/` proves the script fails on a synthetic violation.

## 7. Run policy checks

```bash
cargo deny check
cargo audit
```

If the tools are not installed:

```bash
cargo install cargo-deny cargo-audit
```

These checks are blocking in CI for this slice.

## 8. Run the cross-tenant security tests

```bash
cargo test -p persistence --test tenant_isolation
cargo test -p auth --test auth_flow
cargo test -p application --test authorization
```

Expected negative coverage:
- A tenant-A context cannot read or write tenant-B data.
- A non-member or inactive/removed membership is rejected.
- The last active owner cannot be demoted or removed.
- An expired or revoked session is rejected.
- Repeated login failures trigger the configured lockout.

## 9. Stop PostgreSQL

```bash
docker compose down
```

Add `-v` only when you intend to delete the local database volume.

## Troubleshooting

- **`DATABASE_URL` is missing**: the API fails fast with a configuration error. Export the URL from step 2.
- **Port 5432 is already in use**: stop the conflicting PostgreSQL instance or change the Compose port and `DATABASE_URL` together.
- **`sqlx::test` cannot create a database**: ensure the configured user has `CREATEDB`; the Compose `saas` user is the PostgreSQL superuser.
- **wasm check fails with a missing target**: run `rustup target add wasm32-unknown-unknown`.
- **Boundary script fails with `jq: command not found`**: install `jq`.
- **Desktop host check fails on Linux**: install the GTK/system libraries listed in `.github/workflows/ci.yml`; this is a host build dependency, not a mobile packaging claim.

## Scope reminder

P001-P010 covers the workspace, boundaries, primitives, PostgreSQL schema, rustauth spike, local auth/session service, tenant context and RBAC. P011-P019 covers API auth routes, UI, audit persistence, storage, jobs, AI, backup and packaging.
