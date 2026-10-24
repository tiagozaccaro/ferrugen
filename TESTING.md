# Testing strategy

- Unit tests: domain invariants, permissions, parsers and policy decisions.
- Integration tests: PostgreSQL repositories, migrations, sessions and job persistence.
- Tenant isolation: positive access for members and negative access for non-members; test two tenants and shared users.
- API tests: validation, status codes, auth/session boundaries, error shapes and rate limiting.
- UI tests: critical navigation, form validation, empty/loading/error states and accessibility.
- Contract tests: API DTO serialization and client/server compatibility.
- Security tests: IDOR, CSRF where cookie sessions apply, session fixation, password reset abuse, webhook verification and secret redaction.
- Reliability tests: retries, idempotency, duplicate job delivery, transaction rollback and backup restore.
- Dependency tests: `cargo deny check`, `cargo audit`, lockfile review.

Do not rely on mocks alone for tenant isolation, migrations or persistence correctness. Every security-sensitive endpoint needs at least one denied-access test.
