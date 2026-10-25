#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BOUNDARY_SCRIPT="$SCRIPT_DIR/../../scripts/check-dependency-boundaries.sh"

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/crates/platform-core/src" "$TMP/crates/auth/src"

cat > "$TMP/Cargo.toml" <<'INNER'
[workspace]
resolver = "2"
members = ["crates/platform-core", "crates/auth"]

[workspace.package]
version = "0.1.0"
edition = "2024"
INNER

cat > "$TMP/crates/platform-core/Cargo.toml" <<'INNER'
[package]
name = "platform-core"
version.workspace = true
edition.workspace = true

[dependencies]
INNER

cat > "$TMP/crates/platform-core/src/lib.rs" <<'INNER'
INNER

cat > "$TMP/crates/auth/Cargo.toml" <<'INNER'
[package]
name = "auth"
version.workspace = true
edition.workspace = true

[dependencies]
platform-core = { path = "../platform-core" }
INNER

cat > "$TMP/crates/auth/src/lib.rs" <<'INNER'
INNER

# Test 1: upward dependency (platform-core depends on auth)
cat > "$TMP/crates/platform-core/Cargo.toml" <<'INNER'
[package]
name = "platform-core"
version.workspace = true
edition.workspace = true

[dependencies]
auth = { path = "../auth" }
INNER

if bash "$BOUNDARY_SCRIPT" "$TMP" >/dev/null 2>&1; then
  echo "FAIL: upward dependency not rejected" >&2
  exit 1
fi

# Test 2: misplaced heavy dependency (platform-core depends on sqlx)
cat > "$TMP/crates/platform-core/Cargo.toml" <<'INNER'
[package]
name = "platform-core"
version.workspace = true
edition.workspace = true

[dependencies]
sqlx = "0.8"
INNER

if bash "$BOUNDARY_SCRIPT" "$TMP" >/dev/null 2>&1; then
  echo "FAIL: misplaced heavy dependency not rejected" >&2
  exit 1
fi

echo "Boundary fixture tests passed"
