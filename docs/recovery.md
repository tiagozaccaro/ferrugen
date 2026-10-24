# Recovery & Fix Procedures

Because AI agents can make mistakes at scale, recovery must be scripted and documented.

## Before Any Merge (Prevention)
- `main` is protected: direct pushes blocked; PR required; CI required.
- Every PR has bounded scope in description.

## When a Mistake Is Found

### 1. Identify
```bash
git log --oneline --grep="\[agent:" -n 20
git log --oneline --decorate --graph --all | head -30
```
Cross-reference with PR # and audit artifact (`audit-agent-log.yml`).

### 2. Immediate Rollback Options

**Revert PR (preferred — keeps history):**
```bash
git revert <bad-sha>
git push origin main
```
Or use `.github/workflows/revert.yml` (manual dispatch with PR URL or SHA).

**Restore from a known-good tag (destructive — use only if revert fails):**
```bash
git fetch --tags
git reset --hard <known-good-tag>  # only on a recovery branch
```
Then force-push only if authorized and documented.

**Git bisect (find first bad commit):**
```bash
git bisect start
git bisect bad HEAD
git bisect good <last-known-good-tag>
# ... test at each step ...
git bisect reset
```

### 3. Verify Fix
Run all required checks from `AGENTS.md`:
```bash
cargo fmt --all -- --check
cargo check -p <package>
cargo test -p <package>
cargo clippy -p <package> --all-targets -- -D warnings
cargo deny check
cargo audit
```

### 4. Document
Update PR description or `docs/agent-tracking.md` with root cause and protection gap fixed.
