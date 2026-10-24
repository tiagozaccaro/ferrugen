# Repository instructions

Instruction hierarchy (precedence high → low): platform/system policies > constitution > root `AGENTS.md` > directory `AGENTS.md` > skills (`docs/skills/`) > automated checks.

See `docs/index.md` for the pre-change read order (AGENTS, architecture, roadmap, specs, constitution, speckit, templates). Read before any edit.

## Mission
Build a reusable Rust/Ferrugen platform that is MIT-licensed at the project level, self-hostable, and does not require paid services for core features. This workspace is intended as a base-template: reusable `crates/` and `apps/` packages that a real application (e.g., a content creator / YouTube-style SaaS) can reference via `path` dependencies rather than copying source.

## Before changing code
- Read `docs/architecture/overview.md`, `docs/architecture/dependency-policy.md`, and the relevant crate README.
- Read the issue/spec and implement only the requested bounded task.
- Follow dependency direction: platform-core -> contracts/domain -> application -> infrastructure/adapters -> app entry points. Avoid dependency cycles.
- One issue per branch. Never silently broaden scope.
- Before any implementation: generate a failing test that captures the acceptance criteria; implement only until the test passes; never disable the failing test to make CI green.

## Rust standards
- Rust 2024, workspace dependency versions, workspace lints.
- No `unsafe`; no `unwrap`, `expect`, or panic in production paths.
- Prefer explicit errors (`thiserror`), typed IDs/DTOs, small modules and tests.
- Avoid `anyhow` in reusable domain crates; use it only at application boundaries.
- Keep I/O behind traits where that makes tests and provider replacement materially easier.
- Never put secrets in source, logs, fixtures, examples or committed `.env` files.

## SaaS security
- A user can belong to multiple tenants; a tenant can contain multiple users.
- Resolve tenant context from a verified membership/session, not from a client-supplied tenant ID alone.
- Every tenant-scoped query must be tenant-scoped at the repository boundary.
- Deny by default; authorize each operation server-side.
- Add tests for cross-tenant read/write denial for every tenant-scoped module.
- Audit privileged actions and credential/security events without logging secrets.

## Self-hosting and license policy
- Core operation must not require hosted OAuth, hosted email, paid AI APIs, managed database or commercial deployment service.
- External providers are optional adapters and must be disabled when unconfigured.
- Review direct and transitive dependency licenses with `cargo deny`; review advisories with `cargo audit`.
- Runtime and model weights have separate licenses. Never claim a model is open source without checking its license.

## Required checks
Run applicable commands:
- `cargo fmt --all -- --check`
- `cargo check -p <changed-package>`
- `cargo test -p <changed-package>`
- `cargo clippy -p <changed-package> --all-targets -- -D warnings`
- `cargo deny check`
- `cargo audit`

## Agent instruction system
Canonical policies live in `docs/` (`rust-engineering.md`, `security-policy.md`, `testing-policy.md`) and reusable skills in `docs/skills/`. Directory-specific adapters go in `crates/<name>/AGENTS.md` or `apps/<name>/AGENTS.md`.

- Repository overview: `crates/*` (platform, contracts/domain, application, infrastructure/adapters) + `apps/*` (web, desktop, mobile, worker, admin-cli, api). See `docs/architecture/overview.md`.
- How to discover guidance: root `AGENTS.md` → `docs/rust-engineering.md`, `docs/security-policy.md`, `docs/testing-policy.md`, then directory `AGENTS.md`, then skills.
- Supported Rust toolchain: edition 2024, Rust ≥1.85 (MSRV). Identify the authoritative version via `rustc --version` and Cargo config; do not upgrade blindly.
- Build/validate commands (use exact workspace): `cargo fmt --all -- --check`, `cargo check -p <crate>`, `cargo clippy -p <crate> --all-targets -- -D warnings`, `cargo test -p <crate>` / `cargo test --workspace`, `cargo deny check`, `cargo audit`.
- Dependency changes: review direct + transitive licenses with `cargo deny`; advisories with `cargo audit`; prefer permissive licences (MIT/Apache-2.0/BSD family); never ignore vulnerabilities without documented justification.
- API design: strong types, explicit invariants, documented public surface; domain errors via `thiserror`; no `anyhow` in domain crates.
- Errors/observability: meaningful `Result` types; no `unwrap`/`expect`/panic in production paths unless invariant-justified; log secrets never.
- Async: Tokio model follows project conventions; no blocking executor calls; bounded concurrency/queues.
- Security/privacy: server-side authorisation; tenant scoped at boundary; secrets only in env/session; cryptography via established libraries.
- Database/migrations: where a database is used, follow `persistence` crate conventions; migrate in dependency order.
- Testing: unit/integration/regression/contract/auth/concurrency/failure-path; validate behaviour not implementation; never weaken tests to pass.
- Documentation: keep inline concise; move workflows to skills/policies.
- Git: one issue per branch; small commits; PR title describes behaviour; body includes `Closes #N`; request code+security review.
- Prohibited shortcuts: no bypassing branch protection/CI; no disabling security controls to make builds pass; no masking failures; no fabricated test results.
- Definition of done: change smallest coherent unit; tests pass; fmt/clippy/check green; diff reviewed for scope/regressions/secrets; report executed commands and actual results.
- Reporting completed work and unresolved limitations is mandatory.

## Git workflow
- One GitHub issue per branch; small commits.
- PR title describes the behavior, body includes `Closes #N`.
- Request independent code and security review.
- Never merge, push, publish releases, or deploy without explicit user authorization.

---

# Dioxus 0.7 Development Reference

This workspace uses Dioxus 0.7 (`dioxus = "0.7"`). Only use up-to-date 0.7 documentation.
`cx`, `Scope`, and `use_state` are gone.

## Dependency
```toml
[dependencies]
dioxus = { version = "0.7.1" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

## Launch
```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "Hello, Dioxus!" }
}
```
Serve with `dx serve`: `curl -sSL http://dioxus.dev/install.sh | sh` then `dx serve`.

## UI with RSX
```rust
rsx! {
    div {
        class: "container",
        color: "red",
        width: if condition { "100%" },
        "Hello, Dioxus!"
    }
    for i in 0..5 {
        div { "{i}" }
    }
    if condition {
        div { "Condition is true!" }
    }
    {children}
    {(0..5).map(|i| rsx! { span { "Item {i}" } })}
}
```

## Assets
Links start with `/` relative to root.
```rust
rsx! {
    img { src: asset!("/assets/image.png"), alt: "An image" }
}
```

## Styles
```rust
rsx! {
    document::Stylesheet { href: asset!("/assets/styles.css") }
}
```

## Components
- `#[component]`; function starts with capital letter or contains underscore.
- Re-renders when props change (`PartialEq`) or reactive state updates.
- Props owned (`String`, `Vec<T>`), implement `PartialEq` + `Clone`.
- Wrap in `ReadOnlySignal` for reactive copies.

## State
Signals wrap values; changing causes re-run.

### Local state
```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    let mut doubled = use_memo(move || count() * 2);
    rsx! {
        h1 { "Count: {count}" }
        button { onclick: move |_| *count.write() += 1, "Increment" }
        button { onclick: move |_| count.with_mut(|c| *c += 1), "Increment with with_mut" }
    }
}
```

### Context
```rust
#[component]
fn App() -> Element {
    let mut theme = use_signal(|| "light".to_string());
    use_context_provider(|| theme);
    rsx! { Child {} }
}
#[component]
fn Child() -> Element {
    let theme = use_context::<Signal<String>>();
    rsx! { div { "Current theme: {theme}" } }
}
```

## Async
```rust
let mut dog = use_resource(move || async move { /* api request */ });
match dog() {
    Some(dog_info) => rsx! { Dog { dog_info } },
    None => rsx! { "Loading..." },
}
```

## Routing
```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
    rsx! {
        a { href: "/", "Home" }
        Outlet<Route> {}
    }
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}
```
Features: `dioxus = { version = "0.7.1", features = ["router"] }`

## Fullstack
```toml
dioxus = { version = "0.7.1", features = ["fullstack"] }
```

## Server Functions
```rust
#[post("/api/double/:path/&query")]
async fn double_server(number: i32, path: String, query: i32) -> Result<i32, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(number * 2)
}
```

## Hydration
Server-rendered HTML made interactive on client. Client initial UI must match server exactly. Use `use_server_future` instead of `use_resource`. Browser-specific APIs after hydration in `use_effect`.
