# Tasks: Jobs (Apalis)
- [ ] T401 Add `apalis` dependency to workspace; verify license/maintenance
- [ ] T402 Create job table + PostgreSQL pool setup (`crates/infrastructure` or `crates/jobs`)
- [ ] T403 Enqueue / retry / scheduling / heartbeat / orphan recovery
- [ ] T404 Integration test: worker restart → retry → no duplicate idempotent actions
- [ ] T405 Tenant-scoped job access + audit event
