# Release and changelog workflow

Release entries are generated from commits with [git-cliff](https://git-cliff.org/) when a release is prepared. `CHANGELOG.md` does not exist until the first release: the helper creates it, and later releases append a dated section. A tag-time CI job verifies that `CHANGELOG.md` and `Cargo.toml` agree with the release tag.

The changelog has no pre-release bucket. Entries are dated release sections only.

## Prerequisites

Install the pinned generator:

```bash
cargo install git-cliff --locked --version 2.14.2
```

You also need Python 3 (used by `scripts/release.py`) and Git.

## Commit messages

Use Conventional Commits for the clearest output:

```text
feat(api): add tenant-scoped project endpoint
fix(auth): reject expired sessions
docs: document backup restore
```

`feat` maps to **Added**, `fix` to **Fixed**, `refactor` to **Changed**, and so on. See `cliff.toml` for the full mapping. Non-conventional commits are not dropped; they appear under **Other** using the first line of the commit message. Merge commits and `chore(release):` commits are skipped.

## First release

`CHANGELOG.md` is created by the first release. Run the helper with the first version:

```bash
scripts/release.sh 0.1.0
```

Because there is no earlier tag, git-cliff includes the full history in this entry. Review the generated file before committing. If an earlier release is already tagged, the generated entry starts after that tag.

## Prepare a release

1. Ensure the release scope is merged and CI is green.
2. Create a release branch and run the helper from it. The helper edits files but does not commit or push:

   ```bash
   git switch -c release/v0.2.0
   scripts/release.sh 0.2.0
   ```

   Use `--date YYYY-MM-DD` only when backfilling a historical release.
3. Review the diff. The helper:
   - generates the commit-derived section with `git-cliff`;
   - creates `CHANGELOG.md` on the first release and appends later releases;
   - keeps the previous releases;
   - updates the release links;
   - bumps `[workspace.package].version` in `Cargo.toml`.
4. Commit and open the release pull request:

   ```bash
   git add CHANGELOG.md Cargo.toml
   git commit -m "chore(release): v0.2.0"
   git push -u origin release/v0.2.0
   ```

5. After the pull request is merged, create and push the annotated tag:

   ```bash
   git switch main
   git pull --ff-only
   git tag -a v0.2.0 -m v0.2.0
   git push origin v0.2.0
   ```

The tag push runs `Release Validation`, which executes:

```bash
python3 scripts/release.py verify 0.2.0
```

The release fails if the tag version, workspace version and changelog entry disagree, or if the changelog entry is not dated.

## Verify locally

```bash
python3 scripts/release.py verify 0.2.0
```

Verification also rejects a changelog that still contains a pre-release section; remove it before preparing the release.

## Manual fallback

If `git-cliff` is unavailable, do not tag a release with a missing entry. Install it and rerun the helper. The verification gate is intentionally strict: it does not generate content in CI, so released changelogs are reviewable in the pull request before tagging.

## Files

- `cliff.toml` — commit grouping, templates and tag matching.
- `scripts/release.sh` — local release preparation wrapper.
- `scripts/release.py` — changelog creation/editing, version bump and verification.
- `tests/release/release-test.sh` — regression tests for the release tooling.
- `.github/workflows/release-validation.yml` — tag and manual validation gate.
