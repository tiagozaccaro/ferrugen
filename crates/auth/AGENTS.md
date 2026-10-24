# `auth` package — local rules
Tenant membership/session resolution; server-side verified only.
- Every operation resolves tenant from verified session, never client-only ID.
- Cross-tenant denial tests required.
- Secret/credential events audited without logging secrets.
- Refer to `docs/security-policy.md` and root `AGENTS.md`.
