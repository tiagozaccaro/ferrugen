# Ferrugen

A reusable, self-hostable Rust platform for multi-tenant SaaS products. The workspace separates **reusable platform crates** from **runnable applications** and an example **Ferrugen template**. Products consume the platform through `path` dependencies rather than copying source.

- **Version:** `0.1.0` (foundation milestone, not production-ready)
- **Language/toolchain:** Rust 2024, MSRV 1.85, pinned toolchain in `rust-toolchain.toml`
- **License:** MIT (`LICENSE`)

## Non-negotiable principles

- MIT-licensed project code; audit every dependency and model license separately.
- Self-hostable core: no paid SaaS account is required for local accounts, PostgreSQL persistence, local file storage, API operation, or administration.
- Optional external integrations (OAuth, SMTP, hosted AI APIs) are opt-in, disabled when unconfigured, and must be documented.
- Tenant membership is many-to-many: one user may belong to multiple tenants, and each tenant may have many users.
- Server-side authorization and tenant isolation; never trust tenant IDs supplied by a client without membership verification.
- Rust, Dioxus, Axum, Tokio, SQLx, PostgreSQL and `pnpm` only where a JS tool is later introduced.
- OpenCode is the primary coding agent. Spec Kit artifacts remain tool-independent.

## Workspace layout

```text
apps/
  api/              Axum HTTP API (health endpoints)
  web/              Dioxus 0.7 web starter (router)
  desktop/          Dioxus 0.7 desktop starter (router)
  mobile/           Dioxus mobile scaffold
  worker/           Background worker scaffold
  admin-cli/        Local administration CLI (`version`, `doctor`)

crates/
  Foundation        platform-core, contracts, domain, application
  Capabilities      auth, tenancy, audit, billing, cache, rate-limit, backup,
                    jobs, ai, plugins, persistence, storage, sync, api-client, i18n
  Presentation      ui
  Adapters          infrastructure

templates/ferrugen/ Example product composition
migrations/         Database migration home (no migrations yet)
docs/               Architecture, policies, guides and plans
specs/              Spec Kit feature specs (001-016)
scripts/            Repository and instruction scripts
tests/              GitHub-governance regression tests
```

## Quick start

Requirements: the Rust toolchain from `rust-toolchain.toml`, Cargo, Docker Compose, and (for UI work) the Dioxus CLI. Web, desktop and mobile builds also need platform-specific system libraries; see `docs/architecture/platform-matrix.md`.

```bash
cp .env.example .env
docker compose up -d postgres

cargo check --workspace
cargo test --workspace
cargo run -p api
```

The API starts on `127.0.0.1:8080` by default (override with `APP_HOST` / `APP_PORT`).

- `GET /health/live` — liveness; does not require a database.
- `GET /health/ready` — readiness; currently returns `503` with `not_ready_database_not_wired` until persistence is wired.

Run the Dioxus starters with the Dioxus CLI (`dx serve`) from `apps/web` or `apps/desktop`. Dioxus 0.7 selects targets with flags such as `dx serve --web` and `dx serve --desktop`; see the Dioxus 0.7 documentation for platform-specific setup.

## Honest implementation status

This repository is a **foundation**, not a production-ready SaaS. Do not expose it publicly until the release gate in `SECURITY.md` is satisfied.

**Working today**

- Rust 2024 workspace with centralized dependency versions and lints (forbid `unsafe`; deny `unwrap`, `expect`, `panic`).
- Typed `UserId` / `TenantId`, shared errors, tenant/membership domain invariants.
- `TenantContext` derived only from a verified membership, never from a client-supplied tenant ID.
- API liveness/readiness endpoints; readiness intentionally stays `503` until the database is wired.
- Dioxus 0.7 web/desktop starters with routing and shared UI components (`Navbar`, `Hero`, `Echo`, `AppShell`, `EmptyState`).
- Language-tag parsing in `i18n`; admin CLI `version` and `doctor` commands.
- Spec Kit specs `001`-`016`, architecture/policy documentation, CI and repository governance.

**Contracts and interfaces only** (no production adapters)

`auth`, `audit`, `billing`, `jobs`, `storage`, `sync`, `ai`, `plugins`, `rate-limit`, `api-client`.

**Scaffolds**

`cache`, `backup`, `persistence`, `infrastructure`.

**Not implemented yet**

Authentication persistence and session flows, SQLx repositories, database migrations, tenant-scoped API endpoints, durable job execution, local inference, plugin sandboxing, file-upload handling, offline sync, mobile packaging, end-to-end UI flows, and backup/restore.

## Validation

Run the applicable checks before proposing a change:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check
cargo audit
bash tests/github-governance/workflow-injection-test.sh
bash tests/release/release-test.sh
```

CI runs formatting, check, tests, Clippy, `cargo deny`, a best-effort `cargo audit`, and the workflow-injection regression test. Linux desktop builds require GTK/WebKit system packages, which CI installs.

## Documentation

Read in this order before editing:

1. `AGENTS.md` — mission, security rules, Rust standards and Dioxus 0.7 reference.
2. `docs/index.md` — complete pre-change read order.
3. `docs/architecture/overview.md` and `docs/architecture/dependency-policy.md`.
4. `docs/policies/` — security, testing and branch-protection policies.
5. `docs/plans/implementation-roadmap.md` — phases and current status.
6. `specs/` — feature specs for each phase.
7. `CHANGELOG.md` — release history; created by the first release (see `docs/guides/release.md`).

Additional references: `docs/guides/architecture-choices.md`, `docs/guides/speckit-workflow.md`, `docs/guides/release.md`, `docs/plans/production-readiness.md`, `IMPLEMENTATION.md`, `TESTING.md`, `SECURITY.md`, `review.md`.

## Development workflow

1. One GitHub issue per bounded task; one branch per issue.
2. Use Spec Kit: `/speckit.specify`, `/speckit.plan`, `/speckit.tasks`, then `/speckit.implement` in OpenCode.
3. Write a failing test that captures the acceptance criteria before implementation; never disable a failing test to make CI green.
4. Keep changes small and add tests with behaviour changes.
5. Run formatting, linting, tests and dependency/license checks.
6. Request independent code review, plus security review for security-sensitive work and test review for behaviour changes.
7. PR title describes the behaviour and the body includes `Closes #N`.

Repository governance lives in `.github/`: CODEOWNERS, issue/PR templates, Dependabot, `merge-guard`, `audit-agent-log` and `revert` workflows, and the `main` ruleset reference in `.github/repo-rules.yml`. Agent commits use the `[agent:<harness>]` scope format described in `docs/agent-tracking.md`. Never merge, push, release or deploy without explicit authorization.

## Releases

Changelog entries are generated from commits with `git-cliff` when a release is prepared. There is no `[Unreleased]` section: the first release creates `CHANGELOG.md`, and later releases append to it.

```bash
scripts/release.sh 0.1.0          # first release: creates CHANGELOG.md and bumps the workspace version
python3 scripts/release.py verify 0.1.0
```

Include the generated files in a release pull request, merge it, then push the `v0.1.0` tag. The `Release Validation` workflow rejects a tag when the changelog entry and `Cargo.toml` version do not match. Full process: `docs/guides/release.md`.

## Self-hosting and licensing

Core operation does not require hosted OAuth, hosted email, paid AI APIs, a managed database or a commercial deployment service. External providers are optional adapters and must be disabled when unconfigured. PostgreSQL is exposed only on loopback by default in `compose.yaml`.

Dependency licenses are checked with `cargo deny`; advisories with `cargo audit`. Runtime and model weights have separate licenses — never claim a model is open source without checking its license.

## Security

This starter is not production-ready. The minimum release gate is documented in `SECURITY.md`. Do not publish exploitable reports with working credentials or personal data; use the repository owner's private security reporting channel for private deployments.

## License

MIT. See `LICENSE`.
