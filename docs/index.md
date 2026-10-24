# Agent Index — Read Before Any Change

This directory is organized for any AI agent (`opencode`, other harnesses, or external). Read in this order before editing:

1. `AGENTS.md` (repo root) — mission, security rules, Rust standards, Dioxus 0.7 reference.
2. `docs/guides/architecture-choices.md` — locked architecture (rustauth, apalis, rig, wasmtime, plugin trait, hierarchy, AI design).
3. `docs/architecture/overview.md` + `docs/architecture/dependency-policy.md` — architecture + dependency rules.
4. `docs/implementation.md` (repo root) or `docs/guides/speckit-workflow.md` — implementation protocol + speckit commands.
5. `docs/plans/implementation-roadmap.md` — phases (0-8) with current status.
6. `docs/plans/phase-2-schema.md` — multi-tenant schema design.
7. `docs/plans/production-readiness.md` — readiness criteria.
8. `specs/` — feature specs per phase (`001` foundation through `005` AI/plugins). Each has `spec.md` + `plan.md` + `tasks.md`. See `specs/` directly.
9. `.specify/memory/constitution.md` — hard constraints (open-source, tenant isolation, module reuse, deny-by-default, dependency discipline).
10. `.specify/templates/` — spec/plan/task/constitution templates.
11. `.specify/workflows/speckit/` — speckit workflow definition.
12. `docs/plans/README.md` + `docs/guides/README.md` + `docs/architecture/README.md` — subfolder navigation.

Agent rules (from `AGENTS.md`): read spec before edit; confirm acceptance criteria; implement smallest vertical slice; never silently broaden scope; add/update tests; run `cargo fmt`/`check`/`test`/`clippy`; run `cargo deny check` and `cargo audit`; never merge/deploy without authorization.

Compatible with `opencode`, Claude, GPT, and other agents: all instructions are plain markdown with file paths; no proprietary syntax required.

### Instruction system references (added by agent instruction implementation)
- docs/rust-engineering.md / docs/security-policy.md / docs/testing-policy.md
- docs/skills/ (rust-engineering, rust-security, etc.)
- docs/branch-protection.md / scripts/instruction-integrity.sh
