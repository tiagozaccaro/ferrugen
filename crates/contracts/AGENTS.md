# `contracts` package — local rules
Public contract APIs (`ErrorResponse`, `HealthResponse`, `TenantSummary`).
- Strong types only; no raw primitives for IDs/status codes.
- `Display` implemented for error responses (`docs/rust-engineering.md`).
- Serialization (`serde`) is required; no extra public methods.
- Refer to root `AGENTS.md` for dependency/license/security rules.
