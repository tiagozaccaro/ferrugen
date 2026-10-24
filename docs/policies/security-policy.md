# Security Engineering Policy

Proportionate to this multi-tenant Rust/Ferrugen workspace.

- Authentication/authorisation server-side; resolve tenant from verified membership/session.
- Tenant-scoped queries must be tenant-scoped at repository boundary.
- Deny by default; authorise each operation server-side.
- Add cross-tenant read/write denial tests for every tenant-scoped module.
- Audit privileged actions and credential/security events without logging secrets.
- Use `thiserror` for typed errors; never expose stack traces/secrets to clients.
- Dependency vulnerability management: `cargo deny` + `cargo audit`; review advisories.
- File/path handling: validate paths before file operations (`InvalidPath` error type exists).
- AI-generated code risk: review untrusted input/prompt-injection paths; no arbitrary plugin execution.
- Secure logging: no secrets in logs/fixtures/committed files.
