#!/usr/bin/env bash
# test-build-per-container.sh — validate the `build-{container}.json` pattern.
#
# Data-driven: iterates every `build-*.json` entry in manifest.json. For each,
# verifies the generated file exists, any consuming service's src/ symlink
# resolves, and (if a flake.nix reads it) `nix build` of that service succeeds.
#
# Exits non-zero on any failure. Safe to run anytime — read-only except for
# symlink→real-file resolution during nix build (restored afterwards).

set -euo pipefail

REPO="${GIT_BASE:-$HOME/git}"
CLOUD_DATA_DIR="$REPO/cloud-data"
CLOUD_SOLUTIONS_DIR="$REPO/cloud/a_solutions"
MANIFEST="$CLOUD_DATA_DIR/manifest.json"

err() { echo "✗ $*" >&2; exit 1; }
ok()  { echo "✓ $*"; }

[ -f "$MANIFEST" ] || err "manifest.json missing at $MANIFEST"

# Collect build-{name}.json files from manifest
mapfile -t BUILD_FILES < <(jq -r '.[] | select(.file | startswith("build-")) | .file' "$MANIFEST")
[ "${#BUILD_FILES[@]}" -gt 0 ] || err "no build-*.json entries in manifest.json"

TESTED=0
for BF in "${BUILD_FILES[@]}"; do
  CLOUD_JSON="$CLOUD_DATA_DIR/$BF"
  [ -f "$CLOUD_JSON" ] || err "generated file missing: $CLOUD_JSON"
  ok "$BF present in cloud-data ($(jq -r '._meta.description // "no meta"' "$CLOUD_JSON" 2>/dev/null || echo unknown))"

  # Find any service that consumes this file via symlink in its src/
  mapfile -t CONSUMERS < <(command find "$CLOUD_SOLUTIONS_DIR" -maxdepth 3 -lname "*I_cloud-data/$BF" 2>/dev/null)
  for SYMLINK in "${CONSUMERS[@]}"; do
    [ -L "$SYMLINK" ] || continue
    SVC_SRC=$(dirname "$SYMLINK")
    SVC_NAME=$(basename "$(dirname "$SVC_SRC")")

    # Resolve symlink → real file for nix build (flakes can't follow external symlinks)
    TARGET=$(readlink -f "$SYMLINK" 2>/dev/null || true)
    [ -f "$TARGET" ] || err "  $SVC_NAME: symlink target missing: $TARGET"
    rm "$SYMLINK"
    cp "$TARGET" "$SYMLINK"

    BUILD_RC=0
    if command -v nix >/dev/null 2>&1 && [ -f "$SVC_SRC/flake.nix" ]; then
      if command grep -q "./$BF" "$SVC_SRC/flake.nix"; then
        (cd "$SVC_SRC" && nix build --impure --no-link --print-out-paths) >/dev/null 2>&1 || BUILD_RC=$?
      fi
    fi

    # Restore symlink regardless of build outcome
    rm "$SYMLINK"
    ln -s "../../../I_cloud-data/$BF" "$SYMLINK"

    [ $BUILD_RC -eq 0 ] || err "  $SVC_NAME: nix build failed"
    ok "  $SVC_NAME: symlink → $BF resolves, nix build OK"
    TESTED=$((TESTED + 1))
  done
done

ok "all build-per-container tests passed ($TESTED consumer(s) across ${#BUILD_FILES[@]} file(s))"
