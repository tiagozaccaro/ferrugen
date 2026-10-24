# Rust Engineering Policy

Adopts Microsoft Pragmatic Rust Guidelines (https://microsoft.github.io/rust-guidelines/) and Rust API Guidelines (https://rust-lang.github.io/api-guidelines/). Preserve attribution; do not claim endorsement.

## Types and APIs
- Prefer strong types and explicit invariants.
- Design small, coherent public APIs; minimise unnecessary public surface.
- Document public APIs and invariants.
- Use generics/traits when they provide genuine value, not speculative extensibility.
- Avoid unnecessary abstraction layers.

## Ownership and memory
- Prefer borrowing over ownership transfer when unnecessary.
- Clone only when justified by ownership or performance.
- Avoid unnecessary allocation/copying.
- No `unsafe` without demonstrated need, documented safety argument, targeted tests.
- Encapsulate unavoidable `unsafe` behind small, reviewed interfaces.

## Errors and correctness
- Use `Result` and meaningful error types (`thiserror` in domain crates).
- Avoid `unwrap`/`expect`/`panic!` in production paths unless invariant-justified.
- Preserve useful error context; never expose secrets in errors/logs.
- Distinguish programmer errors, validation failures, operational failures.

## Async and concurrency
- Follow the project's Tokio runtime model (established in workspace).
- Do not block async executors with blocking operations.
- Handle cancellation, timeouts, retries, lock contention deliberately.
- Avoid unbounded spawning/queues.
- Justify shared mutable state and complex synchronisation.

## Code quality
- Follow Rust naming/formatting conventions (`rustfmt`, `clippy`).
- Prefer simple, readable, maintainable implementations.
- Avoid unnecessary dependencies, macros, generated code.
- Do not suppress warnings globally to conceal problems.
- Workspace lints (`workspace.lints`) enforce this; do not weaken them.

## Justified exceptions
- Any exception requires documented justification and targeted review.
- `unwrap`/`expect` allowed only at test boundaries or invariant-proven paths with comments.
