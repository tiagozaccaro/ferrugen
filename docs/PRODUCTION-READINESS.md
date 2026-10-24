# Production readiness gate

Do not deploy publicly until every applicable item is verified:

- [ ] Workspace builds on documented target toolchains.
- [ ] Database migrations, backup and restore are tested.
- [ ] Local authentication is complete and security-reviewed.
- [ ] Session rotation, logout, recovery, throttling and CSRF strategy are tested.
- [ ] Tenant isolation tests cover every tenant-scoped endpoint/repository.
- [ ] Authorization is enforced on the server.
- [ ] Upload path traversal and size limits are tested.
- [ ] Rate limiting, request limits and error redaction are configured.
- [ ] HTTPS/reverse proxy and secure headers are documented.
- [ ] Secrets are not stored in source or logged.
- [ ] `cargo audit` and `cargo deny check` pass or documented exceptions are approved.
- [ ] Data retention, deletion and audit access policy is documented.
- [ ] AI model/provider data flow and model licenses are reviewed.
- [ ] Monitoring, logs, restore procedure and incident response are documented.
