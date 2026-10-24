#!/usr/bin/env bash
# Prepare a release: generate the changelog entry from commits with git-cliff
# and bump the workspace version. This script does not commit, tag or push.
#
# Usage: scripts/release.sh <version> [--date YYYY-MM-DD]
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

if [ "$#" -lt 1 ]; then
  printf 'usage: %s <version> [--date YYYY-MM-DD]\n' "$0" >&2
  exit 2
fi

exec python3 "$ROOT/scripts/release.py" prepare "$1" --repo "$ROOT" "${@:2}"
