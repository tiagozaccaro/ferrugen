Backup protocol: script-based (scripts/backup/ or crates/backup module).
Retention policy: see docs/plans/production-readiness.md (data retention, deletion, audit access).
No separate service required; PostgreSQL dumps + encrypted storage adapter (optional SMTP/cloud adapter disabled when unconfigured).
Scaffold: crates/backup/src/lib.rs — not implemented.
