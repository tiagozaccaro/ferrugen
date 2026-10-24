# ADR 0002: Local-first core with optional external adapters

- Status: Accepted
- Decision: local credentials, PostgreSQL, filesystem storage and optional local AI are the baseline. OAuth, SMTP and hosted AI are adapters disabled unless configured.
- Consequences: optional integrations need explicit config, failure modes and data-flow documentation; core smoke tests must run without internet credentials.
