## Summary
Foundation spec 001 (P001–P010) complete: workspace, dependency boundaries, IDs/config, PostgreSQL migrations (4-table schema), rustauth spike, service-layer auth/sessions (US1), membership-verified tenant context (US2), deny-by-default RBAC (US3), readiness endpoint with DB/migration checks, full validation documented, cross-tenant isolation tests expanded.

## Issue
Closes #001 (spec 001 — platform foundation).

## Acceptance criteria
- [x] T001–T056 completed; Phase 7 convergence (T057–T061) completed.
- [x] `cargo fmt/check/test/clippy/deny` pass; `cargo audit` documented (upstream `rsa` unpatched).
- [x] Dependency boundary script passes.
- [x] Readiness endpoint (`/health/ready`) returns 200 only when DB reachable + migrations applied; 503 otherwise.
- [x] No delegated P011–P019 work implemented (deferred to specs 002–016).

## Independent review
Requested from build/security reviewer; PR stays open until review approved.
