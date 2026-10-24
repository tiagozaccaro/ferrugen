# `persistence` package — local rules
Database schema / migrations for this workspace.
- Migration ordering must respect dependency direction (platform -> contracts -> application -> infrastructure).
- Test migrations with rollback/recovery; preserve data.
- Refer to `docs/testing-policy.md` and root `AGENTS.md`.
