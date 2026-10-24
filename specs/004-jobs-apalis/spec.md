# Feature Specification: Durable Background Jobs (Apalis)

**Feature Branch**: `004-jobs-apalis`

**Created**: 2026-10-24

**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### P1 - Durable job enqueue / retry / orphan recovery
**Independent Test**: Enqueue job; restart worker; verify retry/recovery without duplication (idempotency required).

---

## Requirements
- PostgreSQL-backed durable job store (`apalis` or equivalent); retries, scheduling, heartbeats, orphan recovery.
- Jobs separate from plugin runtime (plugin execution handled by host/plugin layer; durable scheduling handled by Apalis).
- AI durable workflows: index/import/scheduled/agent executions recover state after restart, avoid non-idempotent repeats.
- Dependency record for `apalis`; verify MIT/Apache, active maintenance.

---

## Assumptions
- Apalis supports PostgreSQL persistence, retries, scheduling, heartbeats, and orphan recovery.
- Durable job table separate from session/member tables.
