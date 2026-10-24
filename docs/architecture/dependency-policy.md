# Dependency and license policy

## Policy
- Project-owned code is MIT-licensed.
- Prefer MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib and other approved permissive licenses.
- `deny.toml` is a first pass, not a legal determination; review licenses and notices for every release.
- Reject dependencies that require a paid hosted plan for core functionality.
- Do not make a commercial service account necessary for build, test, local development or production core flows.
- Pin application dependency resolution in `Cargo.lock`.
- Run `cargo deny check` and `cargo audit` in CI.
- Review transitive dependencies and duplicate versions; document approved exceptions.
- Treat AI model weights, datasets, fonts, icons and sample content as separately licensed assets.
- Avoid vendoring dependencies without provenance, license files and an update process.

## Required record for new dependencies
Record the capability, why standard library/existing crates are insufficient, license, maintenance status, transitive concerns, whether it phones home, and the replacement/removal path.

## Dependency records (new)
- `sqlx`: PostgreSQL connection pool + migrations; MIT/Apache; active; replacement: custom SQLx poller (rejected)
- `pgvector`: vector storage in PostgreSQL; MIT/Apache; replacement: Qdrant (optional only)
- `rig`: provider-neutral chat/agent contracts; check license / model weight separation; adapter isolated
- `apalis`: PostgreSQL-backed durable jobs; MIT/Apache; replacement: custom poller (rejected — too complex)
- `rustauth`: local + OAuth; verify license / Axum compat; replacement: local session middleware
- `fastembed`: local embeddings; verify license / model weights separate; optional adapter
- `tantivy`: FTS; MIT/Apache; replacement: Elasticsearch (deferred — paid)
- `moka`: in-memory cache; MIT/Apache; replacement: Redis (deferred — separate service)
- `wasmtime`: WASM sandbox; MIT/Apache; verify Component Model stability

## No blanket claim
A MIT project license does not relicense dependencies. The project is designed for permissive dependencies, but each release must pass the actual dependency/license audit.
