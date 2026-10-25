#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
MANIFEST="$ROOT/Cargo.toml"

if [[ ! -f "$MANIFEST" ]]; then
  echo "Manifest not found: $MANIFEST" >&2
  exit 2
fi

layer_of() {
  case "$1" in
    platform-core) echo 0 ;;
    contracts|domain) echo 1 ;;
    application) echo 2 ;;
    auth|tenancy|audit|billing|i18n|persistence|storage|jobs|ai|plugins|ui|cache|rate-limit|backup|api-client|sync) echo 3 ;;
    infrastructure|api|web|desktop|mobile|worker|admin-cli) echo 4 ;;
    *) echo "" ;;
  esac
}

sqlx_allowed() { [[ "$1" == "persistence" || "$1" == "infrastructure" ]]; }
dioxus_allowed() { [[ "$1" == "ui" || "$1" == "api" || "$1" == "web" || "$1" == "desktop" || "$1" == "mobile" || "$1" == "worker" || "$1" == "admin-cli" ]]; }
axum_allowed() { [[ "$1" == "api" ]]; }

METADATA=$(cargo metadata --no-deps --manifest-path "$MANIFEST" --format-version 1)

FAIL=0

for pkg in $(echo "$METADATA" | jq -r '.packages[] | select(.source == null) | .name'); do
  pkg_layer=$(layer_of "$pkg")
  [[ -z "$pkg_layer" ]] && continue

  # internal dependencies
  while IFS= read -r dep; do
    [[ -z "$dep" ]] && continue
    dep_layer=$(layer_of "$dep")
    [[ -z "$dep_layer" ]] && continue
    if (( dep_layer > pkg_layer )); then
      echo "ERROR: upward dependency: $pkg (layer $pkg_layer) depends on $dep (layer $dep_layer)" >&2
      FAIL=1
    fi
  done < <(echo "$METADATA" | jq -r --arg pkg "$pkg" '.packages[] | select(.name == $pkg) | .dependencies[] | select(.source == null) | .name')

  # heavy dependencies
  while IFS= read -r dep; do
    [[ -z "$dep" ]] && continue
    case "$dep" in
      sqlx)
        if ! sqlx_allowed "$pkg"; then
          echo "ERROR: misplaced heavy dependency: $pkg depends on sqlx" >&2
          FAIL=1
        fi
        ;;
      dioxus)
        if ! dioxus_allowed "$pkg"; then
          echo "ERROR: misplaced heavy dependency: $pkg depends on dioxus" >&2
          FAIL=1
        fi
        ;;
      axum)
        if ! axum_allowed "$pkg"; then
          echo "ERROR: misplaced heavy dependency: $pkg depends on axum" >&2
          FAIL=1
        fi
        ;;
    esac
  done < <(echo "$METADATA" | jq -r --arg pkg "$pkg" '.packages[] | select(.name == $pkg) | .dependencies[] | .name')
done

if [[ $FAIL -ne 0 ]]; then
  exit 1
fi

echo "Dependency boundaries OK"
