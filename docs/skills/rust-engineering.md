# Skill: rust-engineering

Trigger: implementing, refactoring or reviewing Rust code.
Prerequisites: `AGENTS.md`, `docs/rust-engineering.md`, workspace `Cargo.toml`.
Steps:
1. Identify crate scope (platform-core -> contracts/domain -> application -> infrastructure -> app).
2. Read crate `README` and `Cargo.toml`; check workspace lints.
3. Implement smallest coherent change with typed IDs/DTOs, explicit errors.
4. Avoid `unsafe`; no `unwrap`/`expect`/panic in production.
5. Run `cargo fmt`, `cargo check -p <crate>`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
6. Add/update tests; never disable failing tests.
Completion: command results shown; diff reviewed; no secrets/regressions.
