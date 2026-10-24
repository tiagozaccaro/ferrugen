# Skill: rust-security
Trigger: auth, tenancy, secrets, vulnerability, or AI-generated code changes.
Prerequisites: `docs/security-policy.md`.
Steps:
1. Confirm server-side validation; resolve tenant from session.
2. Add denial tests for cross-tenant access.
3. Audit event logging without secrets.
4. Review dependency changes with `cargo deny`/`audit`.
5. Validate path/file inputs before use.
Completion: security checks passed; no secrets exposed.
