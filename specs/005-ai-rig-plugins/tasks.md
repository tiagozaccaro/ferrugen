# Tasks: AI Agents and Plugins (rig + WASM)
- [ ] T501 Add `rig` / `fastembed` / `pgvector` to workspace/crates; verify licenses/maintenance
- [ ] T502 Design `FerrugenPlugin` trait + manifest + WIT contract; implement native + WASM hosts
- [ ] T503 Retrieval authorization (tenant + document permissions before LLM context)
- [ ] T504 Tool allowlist / approval / audit / timeouts / typed inputs/outputs
- [ ] T505 Durable agent state recovery / resume / retry (no non-idempotent duplicates)
- [ ] T506 MCP adapter trait + SDK evaluation; isolate behind adapter
- [ ] T507 CI: mock provider + recorded test cases; observability metrics (latency/token/errors)
- [ ] T508 Security audit of plugin host capabilities; resource limits; audit event coverage
