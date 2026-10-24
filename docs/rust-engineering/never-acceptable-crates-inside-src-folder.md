<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Never Acceptable Crates Inside Src Folder

# Never acceptable, crates inside `src/` folder
Cargo.toml
crates/
  foo/Cargo.toml 
    src/lib.rs
       deps/bar/Cargo.toml 
```

Rare exceptions to this rule can occur if your crate is in the business of processing workspaces and has a collection of UI tests or similar it relies on; but even then these are usually dummy crates in nature.



## The workspace lists and versions all crates (M-CRATES-IN-WORKSPACE) { #M-CRATES-IN-WORKSPACE }

<why>simple inter-crate dependencies and debugging.</why>

Every crate produced by the project should be listed as a workspace member, and its version should be declared in `[workspace.dependencies]` so that intra-workspace dependencies resolve to a single canonical version.

```toml
