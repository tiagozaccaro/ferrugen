# Feature Specification: Dioxus Multi-Platform UI (Reusability & Offline Ready)

**Feature Branch**: `006-dioxus-saas-ui`

**Created**: 2026-10-24

**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### P1 — Shared navigation and dashboard shell (P1)
**Independent Test**: Launch Dioxus app (web/desktop); navigate between Home, Tenants, Members, Profile; all routes share `NavBar` layout; no broken links.

### P2 — Offline data model and conflict policy (P2)
**Independent Test**: Disconnect network; view cached dashboard; sync upon reconnection; conflicts resolved by timestamp + user authority.

---

## Requirements *(mandatory)*
- Shared components (`NavBar`, `DashboardShell`, `MemberTable`) in `crates/ui` or `templates/ferrugen/`.
- Routing via `dioxus 0.7` (`Router`, `Route`, `Outlet`); no `cx`/`Scope`.
- Secure token/session storage (platform-specific: browser `localStorage`/cookie, desktop `keyring`, mobile secure storage).
- Offline sync protocol + conflict policy defined in docs; data model includes `offline_changes`, `sync_status`, `conflict_resolution`.
- Multi-platform packaging validated (`docs/plans/implementation-roadmap.md` Phase 7).
- Reusable across apps: `path` dependencies reference `platform-core`/`contracts`/`auth`/`ai` without copying source.

---

## Key Entities / References
- `dioxus` full features (`router`, `fullstack`) in workspace `Cargo.toml`.
- `OFFLINE.md` / `sync.md` planned; `offline` capability optional.
- Security: session storage uses `zeroize` where applicable.

---

## Assumptions
- Desktop/mobile targets added to workspace members (`apps/desktop`, `apps/mobile`) when ready.
- Shared UI design system (`design-system`) provides consistent components.
- Offline model does not expose secrets in `localStorage`; encrypted or secure storage preferred.
