# Implementation roadmap

## Phase 0 — Workspace and governance
- [x] Define workspace, reusable crate boundaries and app entry points.
- [x] Add MIT license, OpenCode agents, Spec Kit memory, CI, dependency policy and self-hosting docs.
- [ ] Run `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` and resolve compile issues on supported host.
- [ ] Replace repository URL and contributor copyright holder.

## Phase 1a — Foundation (`spec/001`)
- [ ] Platform-core APIs, contracts, domain IDs.

## Phase 2a — Schema and tenancy (`spec/003`)
- [ ] Multi-tenant schema, migrations.

## Phase 3a — Auth (`spec/002`)
- [ ] `rustauth` session, OAuth, tenant isolation.

## Phase 3b — Durable jobs (`spec/004`)
- [ ] `apalis` + PostgreSQL background jobs.

## Phase 4a — AI and plugins (`spec/005`)
- [ ] `rig`, WASM sandbox, native plugin traits.

## Phase 4b — Multi-platform UI (`spec/006`)
- [ ] Dioxus 0.7 navigation, routing, shared shell.

## Phase 5a — Cache and rate limit (`spec/007` `spec/008`)
- [ ] `moka` caching; `tower` rate limits.

## Phase 5b — Observability and events (`spec/009` `spec/012`)
- [ ] Metrics/tracing; event contracts (`contracts/src/events`).

## Phase 5c — Backup, privacy and audit (`spec/010` `spec/011`)
- [ ] Script-based backup; privacy events in audit.

## Phase 5d — Billing, storage, i18n (`spec/013` `spec/014`)
- [ ] Usage limits; file storage; localization.

## Phase 6 — AI extensions (`spec/005` deep)
- [ ] Durable agent recovery; tool allowlist; retrieval authorization.

## Phase 7 — Sync, offline and mobile (`spec/015` `spec/016`)
- [ ] Offline sync protocol; mobile packaging.

## Phase 8 — Operations and release (`spec/009` `spec/010` `spec/011`)
- [ ] Docker Compose; retention; observability; upgrade docs.
