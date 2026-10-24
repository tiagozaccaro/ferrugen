# Security policy

This project is a starter and is not production-ready until the hardening roadmap is complete.

## Reporting
Do not publish exploitable security reports with working credentials or personal data. For a private deployment, use the repository owner's private security reporting channel.

## Minimum release gate
- Local authentication tested and rate-limited.
- Secure password hashing, session rotation, CSRF strategy and account recovery designed.
- Tenant isolation enforced server-side and covered by tests.
- Secrets loaded from environment or mounted secret files, never committed.
- TLS/reverse proxy deployment documented.
- PostgreSQL not exposed publicly by default.
- Backups and restore tested.
- `cargo audit` and `cargo deny check` clean or exceptions documented.
- Threat model and data retention policy reviewed.
