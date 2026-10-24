# Locked architecture choices (Phase 0 → Phase 2)

Recorded per `docs/architecture/dependency-policy.md`. All choices skip open spikes and commit to specific crates.

| Capability | Choice | Why / Replacement | License / Maintenance (to verify) | Replacement if removed |
|---|---|---|---|---|
| Auth (local + OAuth) | `rustauth` + local OAuth adapters | Replaces open RustAuth vs low-level crates spike; supports multi-tenant session model | Verify: MIT/Apache? Active commits? Axum compatibility? | Local credential module + Axum session middleware |
| Background jobs | `apalis` + PostgreSQL | Replaces in-memory channel; durable retries, uniqueness, scheduling, heartbeats, orphan recovery | Verify: MIT/Apache? Maintenance? | Custom SQLx poller (rejected — too complex) |
| AI (chat + agent) | `rig` | Provider-neutral intended; committing to `rig` specifically for chat and agent contracts; local adapter baseline | Verify: license, model weight separation, remote provider gates | Trait-only interface (would drop rig-specific features) |
| Plugins (trusted) | Native Rust traits (`FerrugenPlugin`) | First-party / built-in modules; no sandbox needed for trusted code | Project-owned trait + `crate plugins` | Feature-gated compile-time modules |
| Plugins (untrusted / AI) | Wasmtime + WIT + `wasmtime` crate | Sandbox for third-party / user / AI-generated plugins; typed interface; resource limits; host capability control; manifest (permissions/versioning/lifecycle) required; native traits (`FerrugenPlugin`) for trusted modules; separate process/container deferred | Verify: MIT/Apache? Component Model stability? | Separate worker/container (deferred — more infra) |

Security constraints preserved:
- Multi-tenant queries tenant-scoped at repo boundary.
- Plugin manifest declares permissions; host enforces; manifest alone is not enforcement.
- WASM is not complete isolation; combine with resource limits, restricted host functions, audit.
- No secrets in source/fixtures/logs; audit privileged actions.

Hierarchy model (GCP/GitHub-style):
- `users` = independent identity (solo or multi-org).
- `orgs` = grouping / billing / access container.
- `tenants` = scoped workspace (under org, or standalone for solo users).
- `memberships` = `user_id`, `org_id?`, `tenant_id`, `role`, `status` (invited / active / removed).
- User can belong to multiple orgs and multiple tenants across orgs.

## Plugin design (concrete)

Two channels with explicit interfaces; no unrestricted access to app/database.

- Native: `FerrugenPlugin` trait (`async_trait`, `Send + Sync`) with `manifest()`, `initialize(context)`, `execute(input, context)` → `Result<PluginOutput, PluginError>`, `shutdown()`. Built-in modules via compile-time features; trusted extensions as native dynamic libraries (deferred).
- WASM (Wasmtime + WIT): WIT component contract for third-party / AI agents; typed, versioned, sandboxed, resource-limited.
- Separation: Apalis handles jobs (`PostgreSQL` persistence, retries, heartbeats, orphan recovery); plugin runtime handles safe execution of extensions. Same plugin may enqueue a durable job (Apalis worker → host → WASM sandbox → audit → persist result) for tasks needing retries/scheduling.

Manifest declares: ID, version, API compatibility, permissions/required capabilities, input/output schemas, resource limits/timeouts, network/AI capability flags. **Host enforces permissions regardless of what the manifest states.**

Host services exposed to plugins: `Database` / `Storage` / `AI models` / `Jobs` / `HTTP` / `Audit` — each behind explicit capabilities, tenant-scoped where relevant. No secret logging. Combined with resource limits and restricted host functions for genuine isolation.

---

## AI and agent design (005)

### Embedding / retrieval
- PostgreSQL + `pgvector` (primary): embeddings, conversation history, documents, permissions, metadata in same database. Keeps tenant context and vector records together.
- `Qdrant`: optional only when scale/retrieval needs justify another service.
- `fastembed` (local): generate embeddings locally.
- `tantivy`: full-text search engine in Rust.
- PDF: `lopdf` / `pdf-extract`; HTML extraction: readability-style.
- Retrieval authorization: every vector/FT query must enforce tenant and document permissions **before** context reaches model.

### Durable AI workflows
- `Apalis` + PostgreSQL for indexing, imports, scheduled tasks, long-running agent executions.
- LLM call ≠ durable workflow. Restart/recover/resume/retry required; avoid repeating non-idempotent tool actions.
- State recovery: worker restarts must recover agent state; resume or safely retry without duplicate effects.

### Tools / interoperability (rig-based)
- `rig` for retrieval + LLM contracts; adapter isolated behind internal interface.
- MCP adapter behind internal trait; evaluate Rust MCP SDK; no direct dependency on one SDK.
- Tools: typed inputs/outputs; argument validation; tenant permissions enforced; timeouts; allowlist for executable operations; approval for consequential actions; audit trail maintained.
- Agent execution: WASM sandbox preferred for third-party/user/AI-generated plugins; native traits for trusted first-party.

### Observability / evaluation
- Metrics: latency, token usage, provider errors, tool execution, retrieval quality, approximate costs.
- CI: deterministic mock providers + recorded test cases instead of requiring live LLM.
- Dependency audit (`cargo deny` / `cargo audit`) for `rig`, `fastembed`, `pgvector`, `tantivy`, `lopdf`, `pdf-extract`, `sqlx`, `apalis`.
