# Implementation Plan

1. Establish Cargo workspace, crate dependency direction and CI.
2. Implement platform primitives and domain types.
3. Define membership and permission boundaries.
4. Add PostgreSQL migrations and persistence adapters.
5. Implement local auth and tenant authorization after an explicit auth-library spike.
6. Wire API, UI, worker and CLI.
7. Add local AI/storage/jobs adapters.
8. Validate license, security, self-hosting and target build matrix.

## Validation
Run formatting, workspace check/test/clippy, cargo-deny and cargo-audit. Use PostgreSQL integration tests for persistence and tenant isolation. Platform-specific UI targets must be checked on their supported toolchains; do not infer mobile readiness from a host build.
