# Plan: Jobs (Apalis)
- `crates/jobs` (already exists): Apalis worker + PostgreSQL connection.
- Job table: `job_id`, `kind`, `payload`, `status`, `retry_count`, `scheduled_at`, `created_by_tenant`.
- Integration test: enqueue → restart worker → verify retry without duplication; tenant-scoped access control on job payload/retries.
- Dependency audit (`apalis`).
