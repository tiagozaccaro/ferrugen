# Feature Specification: AI Agents and Plugins (rig + WASM)

**Feature Branch**: `005-ai-rig-plugins`

**Created**: 2026-10-24

**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### User Story 1 - AI agent with retrieval and safe tool execution (P1)
**Why this priority**: Converts a chatbot into a platform; retrieval must enforce tenant and document permissions before context reaches model.
**Independent Test**: Agent receives input; retrieves permitted documents; validates context; executes approved tool; audits event; returns result.

### User Story 2 - Durable agent workflow recovery (P1)
**Why this priority**: Agent execution must survive worker restart; non-idempotent tool actions must not repeat.
**Independent Test**: Enqueue long-running agent; restart worker; resume/retry safely without duplicate effects.

### User Story 3 - WASM sandbox plugin execution (P2)
**Why this priority**: Third-party/user AI agents must run isolated.
**Independent Test**: Load WASM plugin (Wasmtime + WIT); enforce manifest capabilities; resource limits applied; no unrestricted DB/app access.

---

## Requirements *(mandatory)*
- `rig` (chat + agent contracts) via adapter interface; not direct dependency on one SDK.
- Embedding/retrieval: PostgreSQL + `pgvector` (primary); `fastembed` (local embeddings); `tantivy` (FTS); `Qdrant` only when scale requires another service.
- Retrieval authorization: every query checks tenant + document permissions before context reaches LLM.
- Plugin architecture: native traits (`FerrugenPlugin`) for trusted; WASM (`Wasmtime` + `WIT`) for untrusted/AI/user plugins.
- Manifest: plugin ID, version, API compatibility, permissions/required capabilities, schemas, resource/time limits, network/AI flags. **Host enforces; manifest alone is not enforcement.**
- MCP adapter behind internal trait; evaluate Rust MCP SDK; isolation required.
- Tools: typed inputs/outputs; argument validation; allowlist for executable operations; approval for consequential actions; audit trail; timeouts; tenant permissions enforced.
- Durable agent workflow (`apalis` + PostgreSQL): index/import/scheduled/agent runs recover after restart; resume/retry; no non-idempotent repeats.
- Observability: latency, token usage, provider errors, tool executions, retrieval quality, approximate costs; deterministic mock providers for CI.
- Security: no secret logging; audit privileged/security events.

---

## Key Entities / Interfaces
- `rig` adapter (`AI models` capability), `WIT` plugin contract (`plugins` crate), manifest (`PluginManifest`), host capabilities (`DB`/`Storage`/`AI`/`Jobs`/`HTTP`/`Audit`).
- Plugin execution: request-level or durable job (`Apalis` worker → host → WASM sandbox → audit).

---

## Assumptions
- `crates/ai` (exists) receives `rig` adapter; `crates/plugins` (exists) receives native trait + WASM adapter.
- `fastembed`, `pgvector`, `tantivy`, `lopdf`, `pdf-extract` added as needed.
- Plugin sandboxing combined with resource limits and restricted host functions; for genuinely untrusted plugins, process/container isolation remains deferred.
