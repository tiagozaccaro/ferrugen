# Feature Specification: Reusable SaaS Platform Foundation

## Goal
Provide a self-hostable, MIT-licensed Rust workspace with reusable SaaS capability crates (platform-core, contracts, domain, application, auth, tenancy, authorization, audit, billing, persistence, storage, jobs, ai, plugins, ui, design-system) and Dioxus 0.7 multi-platform starter apps (api, web, desktop, mobile, worker, admin-cli). Architecture choices locked (rustauth, apalis, rig, wasmtime + WIT, pgvector, fastembed, tantivy); specs 002-006 defined; docs organized (`docs/index.md` + subfolders); OpenCode / Spec Kit artifacts define repeatable implementation.

## Users
- Application developer assembling new products from platform crates via `path` dependencies.
- Self-hosting operator deploying API/worker/PostgreSQL (self-hosted, no mandatory paid SaaS).
- Agent (opencode / other harness): reads `docs/index.md` first, then architecture/roadmap/specs/constitution before editing.

## Functional requirements
- FR-001: Workspace has separate reusable crates and runnable apps.
- FR-002: API exposes liveness and readiness endpoints.
- FR-003: Tenant membership model supports many-to-many user/tenant relationships.
- FR-004: Tenant-scoped operations require verified tenant context.
- FR-005: Authorization defaults to deny when no matching grant exists.
- FR-006: Core local development does not require third-party SaaS credentials.
- FR-007: Optional auth, AI, email and storage providers are replaceable adapters.
- FR-008: OpenCode instructions and Spec Kit artifacts define a repeatable implementation process.
- FR-009: CI runs `cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, `cargo deny check`, `cargo audit`; dependency audit required for new crates (`validator`, `config`, `utoipa`, etc.).

## Non-functional requirements
- NFR-001: MIT project license with dependency license audit.
- NFR-002: No production path uses panic/unwrap/expect.
- NFR-003: Cross-tenant access is covered by negative tests before release.
- NFR-004: PostgreSQL is not exposed publicly by default.
- NFR-005: Unimplemented capabilities clearly marked as scaffolds (`scaffold` / `todo` / `not implemented`).
- NFR-006: All docs agent-compatible (`docs/index.md` + subfolder READMEs); instructions readable by opencode / Claude / GPT.

## Out of scope for foundation
- Complete production authentication flows (002 auth spec defines scope).
- Full database schema implementation (003 schema spec defines design; migrations deferred).
- Billing provider / paid services (specified in 005 / phase 5; optional only).
- Hosted AI dependency (rig provides local adapter baseline; hosted APIs optional behind feature gates; model weight licenses separate).
- Arbitrary in-process untrusted plugin execution (005 defines WASM sandbox with Wasmtime + WIT; native trait plugins for trusted only; process/container isolation deferred for genuinely untrusted).
- Certified mobile/desktop builds for every target (Phase 7; packaging validated when runners/toolchains support).
