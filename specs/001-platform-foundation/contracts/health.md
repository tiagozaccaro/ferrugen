# Health Contract

**Feature**: `001-platform-foundation` | **Date**: 2026-10-25

The API exposes two unauthenticated health endpoints. They are the only HTTP behavior owned by P001-P010; auth endpoints are delegated to P011/spec 002.

## `GET /health/live`

Returns 200 while the HTTP process can serve requests. It does not check PostgreSQL.

**Response** `200 OK`:

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

**Contract rules**:
- Always 200 when the handler is reachable.
- No database call.
- No authentication.
- `version` is `env!("CARGO_PKG_VERSION")`.
- Response media type: `application/json`.

## `GET /health/ready`

Returns 200 only when PostgreSQL is reachable and migrations have been applied. Otherwise returns 503.

**Response** `200 OK`:

```json
{
  "status": "ready",
  "version": "0.1.0"
}
```

**Response** `503 Service Unavailable`:

```json
{
  "status": "not_ready_database_unavailable",
  "version": "0.1.0"
}
```

or

```json
{
  "status": "not_ready_migrations_failed",
  "version": "0.1.0"
}
```

**Contract rules**:
- Database check: bounded `SELECT 1` with a 2-second timeout.
- Migration check: the application records migration success at startup; readiness returns 200 only when the recorded state is `applied`.
- If the database is unreachable or the check times out: 503 with `not_ready_database_unavailable`.
- If the database is reachable but the migration runner failed: 503 with `not_ready_migrations_failed`.
- No authentication.
- Response media type: `application/json`.
- The endpoint must not expose database URLs, credentials, SQL errors or stack traces.
- A missing `DATABASE_URL` is a startup configuration error; the process must not start with an invalid configuration.

## Startup behavior

1. Load and validate configuration.
2. Build the PostgreSQL pool (lazy connections; acquire timeout 5 seconds).
3. Start the HTTP server.
4. Attempt `persistence::migrations::run(&pool)` (the runner created by T010).
5. Record `MigrationState::Applied` on success or `MigrationState::Failed` on error.
6. Readiness reflects the recorded state plus the live `SELECT 1` check.

Starting the HTTP server before migrations finish lets liveness and readiness report the real state instead of hiding a broken deployment.

## Test obligations

- `GET /health/live` returns 200 with `status = "ok"`.
- `GET /health/ready` returns 200 with `status = "ready"` when the database is reachable and migrations are applied.
- `GET /health/ready` returns 503 with `not_ready_database_unavailable` when the database check fails.
- `GET /health/ready` returns 503 with `not_ready_migrations_failed` when migrations failed.
- Readiness completes within the 2-second bound.
- Responses contain no secrets or raw SQL errors.
