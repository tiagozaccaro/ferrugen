<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Ok For Large Workspaces

# Ok for large workspaces
Cargo.toml
crates/
  server/
    main/Cargo.toml 
    routes/Cargo.toml 
  client/
    foo/Cargo.toml
    bar/Cargo.toml
  common/
    error/Cargo.toml
```

Placing crates inside other crates (at or below their `Cargo.toml`), or even inside their `src/` folder is never acceptable. If a crate relationship should be expressed, this is done via common prefixes instead (e.g., `foo`, `foo_util`, `foo_build`).

```bash
