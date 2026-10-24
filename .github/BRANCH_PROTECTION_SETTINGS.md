# GitHub Branch Protection — `main` (Apply Manually or via Admin API)

> **Solo-maintainer exception (current)** — Only one maintainer on this project; approvals reduced to 1 and self-approval permitted. Apply the same exception on the GitHub repo settings / ruleset.

Required settings for `main` (all must be enabled before any agent-driven merge is permitted):

1. **Require a pull request before merging**
   - Require approvals: 1 (solo-maintainer exception — only one maintainer; self-approval permitted)
   - Dismiss stale PR approvals when new commits are pushed
   - Require review from CODEOWNERS (if defined) — sole maintainer in CODEOWNERS may approve own PR

2. **Require status checks to pass before merging**
   - `rust` (format/check/test/clippy)
   - `dependency-policy` (deny + audit)
   - `audit-agent-log` (optional enforcement)
   - `backup-tag` (optional enforcement)

3. **Restrict who can push to matching branches**
   - Restrict push access to roles: maintainers / admins only
   - Prevent direct pushes; require PR for all changes

4. **Require conversation resolution before merging**
5. **Require signed commits** (optional but recommended for agent attribution)
6. **Require linear history** (optional — prevents merge commits that obscure agent traces)
7. **Merge method restrictions** (squash/merge/rebase)
   - Require squash merge for all PRs (preserves linear history, keeps agent tags visible, prevents merge-commit noise).
   - Disable merge commits; optionally allow rebase-merge for hotfixes only with admin approval.

8. **Require branches to be up to date before merging**

Also configure:
- **Branch ruleset** (if using rulesets instead of classic protection): require PR + CI + 1 approval, restrict push.
- **Auto-link issues** enabled.
- **Require issues to be linked** in PRs (align with `Closes #N` in PR template).

## Why
Since AI agents will write code autonomously, direct pushes must be impossible and every change must carry an audit trail (commit tags + PR fields + CI artifacts).
