#!/usr/bin/env bash
# Regression tests for the release/changelog tooling.
#
# The changelog is created on the first release and must never contain an
# [Unreleased] section. The test stubs `git-cliff` so it runs without the real
# binary and exercises deterministic changelog creation, appending, version
# bumping and verification.
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
SCRIPT="$ROOT/scripts/release.py"
TMP=$(mktemp -d "${TMPDIR:-/tmp}/ferrugen-release-test.XXXXXX")
trap 'rm -rf "$TMP"' EXIT

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

assert_contains() {
  grep -qF "$2" "$1" || fail "expected '$2' in $1"
}

assert_not_contains() {
  if grep -qF "$2" "$1"; then
    fail "did not expect '$2' in $1"
  fi
}

assert_equals() {
  [ "$2" = "$3" ] || fail "expected '$3', got '$2' ($1)"
}

make_stub() {
  local out="$TMP/bin/git-cliff"
  mkdir -p "$TMP/bin"
  cat > "$out" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
output=""
tag=""
while [ $# -gt 0 ]; do
  case "$1" in
    --output|-o) output=$2; shift 2 ;;
    --tag|-t) tag=$2; shift 2 ;;
    *) shift ;;
  esac
done
[ -n "$output" ] || { echo "missing --output" >&2; exit 2; }
version="${tag#v}"
cat > "$output" <<'EOF'
# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Dates use ISO 8601 (`YYYY-MM-DD`).

<!-- git-cliff: end of header -->

## [__VERSION__] - 2026-10-24

### Added

- Generated feature from commits.
EOF
# Fill in the version after writing the heredoc.
python3 - "$output" "$version" <<'PY'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
path.write_text(path.read_text().replace("__VERSION__", sys.argv[2]))
PY
STUB
  chmod +x "$out"
  printf '%s\n' "$out"
}

write_repo() {
  local repo=$1
  mkdir -p "$repo"
  cd "$repo"
  git init -q
  git config user.email test@example.com
  git config user.name Test

  cat > Cargo.toml <<'EOF'
[workspace]
members = []

[workspace.package]
version = "0.1.0"
repository = "https://example.com/ferrugen"
EOF

  cat > cliff.toml <<'EOF'
# Stub configuration; the test replaces git-cliff with a fixture.
EOF

  # Deliberately no CHANGELOG.md: the first release creates it.
  git add Cargo.toml cliff.toml
  git commit -qm 'chore: initial'
}

main() {
  [ -f "$SCRIPT" ] || fail "scripts/release.py is missing"

  local repo="$TMP/repo"
  write_repo "$repo"
  local stub
  stub=$(make_stub)

  cd "$repo"
  [ ! -e CHANGELOG.md ] || fail "test setup should not create CHANGELOG.md"

  GIT_CLIFF="$stub" python3 "$SCRIPT" prepare 0.1.0 --date 2026-10-24 --repo "$repo"

  [ -f CHANGELOG.md ] || fail "the first release should create CHANGELOG.md"
  assert_contains CHANGELOG.md '# Changelog'
  assert_contains CHANGELOG.md '## [0.1.0] - 2026-10-24'
  assert_contains CHANGELOG.md '### Added'
  assert_contains CHANGELOG.md 'Generated feature from commits.'
  assert_contains CHANGELOG.md '[0.1.0]: https://example.com/ferrugen/releases/tag/v0.1.0'
  assert_not_contains CHANGELOG.md '## [Unreleased]'
  assert_not_contains CHANGELOG.md '[Unreleased]:'
  assert_contains Cargo.toml 'version = "0.1.0"'

  python3 "$SCRIPT" verify 0.1.0 --repo "$repo"

  GIT_CLIFF="$stub" python3 "$SCRIPT" prepare 0.2.0 --date 2026-10-25 --repo "$repo"

  assert_contains CHANGELOG.md '## [0.2.0] - 2026-10-25'
  assert_contains CHANGELOG.md '## [0.1.0] - 2026-10-24'
  assert_contains CHANGELOG.md '[0.2.0]: https://example.com/ferrugen/releases/tag/v0.2.0'
  assert_contains CHANGELOG.md '[0.1.0]: https://example.com/ferrugen/releases/tag/v0.1.0'
  assert_not_contains CHANGELOG.md '## [Unreleased]'
  assert_not_contains CHANGELOG.md '[Unreleased]:'
  assert_contains Cargo.toml 'version = "0.2.0"'

  local count
  count=$(grep -c '^## \[0.2.0\]' CHANGELOG.md)
  assert_equals "duplicate release heading" "$count" "1"
  count=$(grep -c '^## \[0.1.0\]' CHANGELOG.md)
  assert_equals "previous release heading" "$count" "1"

  python3 "$SCRIPT" verify 0.2.0 --repo "$repo"

  if python3 "$SCRIPT" verify 0.3.0 --repo "$repo" >/dev/null 2>&1; then
    fail "verify should reject a missing changelog entry"
  fi

  if GIT_CLIFF="$stub" python3 "$SCRIPT" prepare 0.2.0 --date 2026-10-25 --repo "$repo" >/dev/null 2>&1; then
    fail "prepare should reject a duplicate release version"
  fi

  printf '\n## [Unreleased]\n\n- stale note.\n' >> CHANGELOG.md

  if python3 "$SCRIPT" verify 0.2.0 --repo "$repo" >/dev/null 2>&1; then
    fail "verify should reject a pre-release section"
  fi

  if GIT_CLIFF="$stub" python3 "$SCRIPT" prepare 0.3.0 --date 2026-10-26 --repo "$repo" >/dev/null 2>&1; then
    fail "prepare should reject a pre-release section"
  fi

  printf 'PASS: release tooling\n'
}

main "$@"
