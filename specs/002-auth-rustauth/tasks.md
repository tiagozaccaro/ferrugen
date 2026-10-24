# Tasks: Auth with rustauth (Local + OAuth)

- [ ] T201 Create auth crate structure (`rustauth` adapter trait + session interface).
- [ ] T202 Add PostgreSQL session table (server-side session storage, revocation, expiry).
- [ ] T203 Implement local registration / login / logout / rate limit.
- [ ] T204 Implement OAuth adapter trait; provide one provider adapter as reference.
- [ ] T205 Resolve tenant context from verified membership (not client-supplied).
- [ ] T206 Cross-tenant negative test suite (regression before release).
- [ ] T207 Dependency record and license check (`cargo deny` / `cargo audit`).
- [ ] T208 Wire Axum middleware in `apps/api`; keep business rules in application layer.
