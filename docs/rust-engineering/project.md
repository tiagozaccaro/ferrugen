<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Project

# Project Guidelines



## Common settings come from the workspace Cargo.toml (M-CARGO-WORKSPACE) { #M-CARGO-WORKSPACE }

<why>consistent, maintainable project configuration.</why>

Any repo with two or more crates that somehow belong together should unify these crates with a workspace `Cargo.toml`. Members then inherit shared metadata and dependency versions from the workspace root via `[workspace.dependencies]`, `[workspace.lints]`, ... rather than duplicating these values in each crate.

Where a dependency is crate-specific, it should still be defined in the workspace. Workspace definitions should generally not enable dependency features (except basic ones such as `["std"]`), and should instead use `default-features = false`.



## All crates are siblings in one folder (M-CRATES-FLAT-FOLDER) { #M-CRATES-FLAT-FOLDER }

<why>simple project navigation and a standard Rust layout.</why>

A repository should contain a single workspace `Cargo.toml`, and all Rust crates should be subordinate to it. All crates should then live in a single, direct subdirectory (e.g., `crates/`) below the workspace (for up to 1-2 dozen of crates), beyond that some folder hierarchy should be used (e.g., `common/`, `server/`, `client/`) to organize siblings.

```bash
