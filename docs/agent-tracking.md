# Agent Tracking & Audit Policy

Since this project is mainly developed by AI agents, every change must be traceable and reversible.

## Tracking Requirements

### Commit Messages
Every commit by an AI agent must include:
```
[agent:<harness-name>] <bounded scope>: <action>

Agent: <agent-id / harness>
Issue: <closes #N>
Scope: <what was edited, no broadening>
```

Example:
```
[agent:opencode] add audit workflow: create revert.yml and backup-tag.yml

Agent: opencode-session
Issue: Closes #N
Scope: .github/workflows/ only; no crate edits
```

### PR Template (enforced)
Every PR must fill:
- Agent / harness used
- Issue / spec link
- Change scope (bounded)
- Independent reviews completed (code, security, test when relevant)

### Audit Artifact
The CI audit-log workflow (`.github/workflows/audit-agent-log.yml`) creates an artifact mapping PR number → agent actions for the session.

## Fix / Revert Capability

### Revert Workflow
`.github/workflows/revert.yml` provides a manual trigger to revert a bad PR via `github.ref`.

### Recovery Procedure
See `docs/recovery.md` for `git reflog`, `git bisect`, tag rollback, and manual restoration steps.

## Fixing Mistakes
1. Identify the bad commit / PR from audit artifact or git log (`[agent:...]` tags help).
2. Create a revert PR referencing the original issue (`Reverts #N`).
3. If already merged to `main`: use `revert.yml` or apply `git revert <sha>` directly.
4. Re-run full CI (`cargo fmt/check/test/clippy`, `cargo deny check`, `cargo audit`) before re-merge.
5. Update this doc if the mistake reveals a gap in protection rules.
