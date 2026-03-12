#!/bin/bash
# discover_copybooks.sh -- Discover copybook locations per repo and write .cobol2rust.toml
#
# Scans each repo under <repos_dir>/<source>/<repo>/ and identifies copybook
# directories using multiple heuristics:
#   1. Directories with well-known names (copy, copylib, cpy, copybooks, includes)
#   2. Directories containing .cpy files
#   3. Directories containing .cbl/.cob files that lack PROCEDURE DIVISION
#      (i.e., they are data-only copybooks, not full programs)
#   4. Directories referenced by COPY statements in the source files
#
# Produces a .cobol2rust.toml in each repo root with discovered copy_paths.
#
# Usage:
#   ./scripts/discover_copybooks.sh <repos_dir> [--dry-run] [--force]
#
# Example:
#   ./scripts/discover_copybooks.sh repos/
#   ./scripts/discover_copybooks.sh repos/ --dry-run
#   ./scripts/discover_copybooks.sh repos/ --force   # overwrite existing .cobol2rust.toml

set -euo pipefail

REPOS_DIR="${1:?Usage: $0 <repos_dir> [--dry-run] [--force]}"
DRY_RUN=false
FORCE=false

shift
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
        --force)   FORCE=true ;;
        *)         echo "Unknown arg: $arg"; exit 1 ;;
    esac
done

# Well-known copybook directory names (case-insensitive matching)
KNOWN_DIRS="copy|copylib|cpy|copybooks|includes|copybook|cpylib|cpybook"

# Counters
TOTAL_REPOS=0
REPOS_WITH_COPYBOOKS=0
REPOS_SKIPPED=0
REPOS_NO_COPYBOOKS=0

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*"
}

# Check if a .cbl/.cob file is a copybook (no PROCEDURE DIVISION)
is_copybook_file() {
    local file="$1"
    # If file has no PROCEDURE DIVISION, it's likely a copybook
    if ! grep -qi 'PROCEDURE[[:space:]]\+DIVISION' "$file" 2>/dev/null; then
        return 0
    fi
    return 1
}

discover_repo() {
    local repo_dir="$1"
    local rel_path="$2"
    local copy_dirs=()

    # Strategy 1: Find directories with well-known copybook names
    while IFS= read -r dir; do
        [ -n "$dir" ] || continue
        copy_dirs+=("$dir")
    done < <(find "$repo_dir" -maxdepth 5 -type d 2>/dev/null | \
        grep -iE "/($KNOWN_DIRS)$" 2>/dev/null || true)

    # Strategy 2: Find directories containing .cpy files
    while IFS= read -r dir; do
        [ -n "$dir" ] || continue
        copy_dirs+=("$dir")
    done < <(find "$repo_dir" -maxdepth 5 -name '*.cpy' -o -name '*.CPY' 2>/dev/null | \
        xargs -r dirname 2>/dev/null | sort -u || true)

    # Strategy 3: Find directories with .cbl/.cob files that lack PROCEDURE DIVISION
    # (These are copybooks stored with program extensions)
    # Only check directories that aren't already the repo root
    while IFS= read -r dir; do
        [ -n "$dir" ] || continue
        # Skip if this is the repo root itself (programs live there too)
        [ "$dir" = "${repo_dir%/}" ] && continue
        # Sample up to 5 files from this dir to check if they're copybooks
        local copybook_count=0
        local program_count=0
        while IFS= read -r f; do
            if is_copybook_file "$f"; then
                copybook_count=$((copybook_count + 1))
            else
                program_count=$((program_count + 1))
            fi
        done < <(find "$dir" -maxdepth 1 \( -name '*.cbl' -o -name '*.cob' -o -name '*.CBL' -o -name '*.COB' \) 2>/dev/null | head -5)
        # If majority are copybooks, include this dir
        if [ "$copybook_count" -gt 0 ] && [ "$copybook_count" -ge "$program_count" ]; then
            copy_dirs+=("$dir")
        fi
    done < <(find "$repo_dir" -maxdepth 4 \( -name '*.cbl' -o -name '*.cob' -o -name '*.CBL' -o -name '*.COB' \) 2>/dev/null | \
        xargs -r dirname 2>/dev/null | sort -u || true)

    # Strategy 4: Extract COPY statement library references from source files
    # Look for: COPY member [OF|IN library].
    # If we find references to directory names that exist, add them
    while IFS= read -r lib; do
        [ -n "$lib" ] || continue
        # Check if this library name matches an actual directory in the repo
        while IFS= read -r dir; do
            [ -n "$dir" ] || continue
            copy_dirs+=("$dir")
        done < <(find "$repo_dir" -maxdepth 4 -type d -iname "$lib" 2>/dev/null || true)
    done < <(grep -rhi 'COPY[[:space:]]\+[A-Za-z0-9_-]\+[[:space:]]\+\(OF\|IN\)[[:space:]]\+' \
        "$repo_dir" --include='*.cbl' --include='*.cob' --include='*.CBL' --include='*.COB' 2>/dev/null | \
        sed -n 's/.*\(OF\|IN\)[[:space:]]\+\([A-Za-z0-9_-]\+\).*/\2/Ip' | \
        sort -u || true)

    # Deduplicate and convert to relative paths
    local unique_dirs=()
    local seen=""
    for dir in "${copy_dirs[@]}"; do
        # Normalize and make relative to repo_dir
        local rel="${dir#"${repo_dir}"}"
        rel="${rel#/}"
        rel="${rel%/}"
        [ -z "$rel" ] && rel="."

        # Skip if already seen
        if echo "$seen" | grep -qF "|${rel}|" 2>/dev/null; then
            continue
        fi
        seen="${seen}|${rel}|"
        unique_dirs+=("$rel")
    done

    # Output
    if [ ${#unique_dirs[@]} -eq 0 ]; then
        return 1
    fi

    # Build TOML content
    local toml="[workspace]\ncopy_paths = ["
    local first=true
    for d in "${unique_dirs[@]}"; do
        if $first; then
            first=false
        else
            toml="$toml, "
        fi
        toml="$toml\"$d\""
    done
    toml="$toml]"

    if $DRY_RUN; then
        echo "  Would write .cobol2rust.toml:"
        echo -e "  $toml"
    else
        echo -e "$toml" > "${repo_dir}/.cobol2rust.toml"
    fi

    return 0
}

log "Starting copybook discovery"
log "  repos_dir: $REPOS_DIR"
log "  dry_run:   $DRY_RUN"
log "  force:     $FORCE"
log ""

# Discover repo directories
for source_dir in "$REPOS_DIR"/*/; do
    [ -d "$source_dir" ] || continue
    for repo_dir in "$source_dir"*/; do
        [ -d "$repo_dir" ] || continue

        TOTAL_REPOS=$((TOTAL_REPOS + 1))
        rel_path="${repo_dir#"$REPOS_DIR"/}"
        rel_path="${rel_path%/}"

        # Skip if .cobol2rust.toml already exists (unless --force)
        if [ -f "${repo_dir}/.cobol2rust.toml" ] && ! $FORCE; then
            REPOS_SKIPPED=$((REPOS_SKIPPED + 1))
            continue
        fi

        # Check if repo has any COBOL files
        if ! find "$repo_dir" -maxdepth 5 -name '*.cbl' -o -name '*.cob' -o -name '*.cobol' \
                -o -name '*.cpy' -o -name '*.CBL' -o -name '*.COB' -o -name '*.CPY' 2>/dev/null | \
                head -1 | grep -q .; then
            continue
        fi

        echo "[$TOTAL_REPOS] $rel_path"

        if discover_repo "$repo_dir" "$rel_path"; then
            REPOS_WITH_COPYBOOKS=$((REPOS_WITH_COPYBOOKS + 1))
        else
            REPOS_NO_COPYBOOKS=$((REPOS_NO_COPYBOOKS + 1))
        fi
    done
done

log ""
log "========================================="
log "Copybook discovery complete"
log "========================================="
log "Total repos:            $TOTAL_REPOS"
log "Repos with copybooks:   $REPOS_WITH_COPYBOOKS"
log "Repos without copybooks: $REPOS_NO_COPYBOOKS"
log "Repos skipped (existing): $REPOS_SKIPPED"
if $DRY_RUN; then
    log ""
    log "(Dry run -- no files written)"
fi
