# Plan: Dioxus Multi-Platform UI
- `crates/ui`: shared components (`NavBar`, `DashboardShell`, `MemberTable`, `Route` definitions).
- `apps/web`: Dioxus fullstack entry; `Router` with `Outlet`; `NavBar` layout.
- `crates/design-system`: design tokens / components reference.
- `offline` sync: sync protocol design; conflict resolution; secure session storage; multi-platform packaging (Phase 7).
- Dependency audit: `dioxus` features (`router`, `fullstack`); `subtle`, `zeroize` for session storage.
