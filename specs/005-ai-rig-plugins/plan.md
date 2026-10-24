# Plan: AI Agents and Plugins (rig + WASM)
- `crates/ai`: rig adapter, retrieval contracts (`pgvector`), tool model (`allowlist`, approval, timeouts).
- `crates/plugins`: `FerrugenPlugin` trait + manifest + WASM adapter (`Wasmtime`+`WIT`); host capability enforcement; tenant-scoped.
- MCP adapter behind internal trait; evaluate Rust MCP SDK; isolation only.
- Observability metrics + mock provider tests in CI.
- Integration: durable agent workflow recovery test.
- Dependency audit (`rig`, `fastembed`, `pgvector`, `wasmtime`).
