#!/bin/bash
# transpile_corpus.sh -- Transpile a multi-repo COBOL corpus
#
# Walks repos/{github,software_heritage}/*/ treating each subdirectory
# as an independent workspace. Runs cobol2rust transpile --workspace on
# each, with parallel transpilation within each repo.
#
# Produces:
#   <output_root>/<source>/<repo>/   -- Rust workspace per repo
#   <output_root>/reports/           -- Merged NDJSON report across all repos
#
# Usage:
#   ./scripts/transpile_corpus.sh <repos_dir> <output_root> [jobs]
#
# Example:
#   ./scripts/transpile_corpus.sh repos/ /data/rust-output 24

set -euo pipefail

REPOS_DIR="${1:?Usage: $0 <repos_dir> <output_root> [jobs]}"
OUTPUT_ROOT="${2:?Usage: $0 <repos_dir> <output_root> [jobs]}"
JOBS="${3:-$(nproc)}"

# Locate cobol2rust binary (prefer local build, fall back to PATH)
COBOL2RUST="${COBOL2RUST:-$(command -v cobol2rust 2>/dev/null || echo "")}"
if [ -z "$COBOL2RUST" ]; then
    echo "ERROR: cobol2rust not found. Set COBOL2RUST env var or add to PATH."
    exit 1
fi

# Merged reports directory
MERGED_REPORTS="${OUTPUT_ROOT}/reports"
mkdir -p "$MERGED_REPORTS"

# Counters
TOTAL_REPOS=0
SUCCEEDED_REPOS=0
FAILED_REPOS=0
SKIPPED_REPOS=0
TOTAL_FILES=0
TOTAL_SUCCEEDED=0
TOTAL_FAILED=0

# Log file
LOG_FILE="${OUTPUT_ROOT}/transpile_corpus.log"
: > "$LOG_FILE"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

log "Starting corpus transpile"
log "  repos_dir:   $REPOS_DIR"
log "  output_root: $OUTPUT_ROOT"
log "  jobs:        $JOBS"
log "  cobol2rust:  $COBOL2RUST"
log ""

# Discover repo directories (any dir under repos_dir that contains .cbl/.cob files)
REPO_DIRS=()
for source_dir in "$REPOS_DIR"/*/; do
    [ -d "$source_dir" ] || continue
    for repo_dir in "$source_dir"*/; do
        [ -d "$repo_dir" ] || continue
        # Check if this repo has any COBOL files (quick check)
        if find "$repo_dir" -maxdepth 5 -name '*.cbl' -o -name '*.cob' -o -name '*.cobol' 2>/dev/null | head -1 | grep -q .; then
            REPO_DIRS+=("$repo_dir")
        fi
    done
done

TOTAL_REPOS=${#REPO_DIRS[@]}
log "Found $TOTAL_REPOS repos with COBOL files"
log ""

if [ "$TOTAL_REPOS" -eq 0 ]; then
    log "No repos found. Check that $REPOS_DIR has subdirectories with .cbl files."
    exit 0
fi

# Process each repo
COUNTER=0
START_TIME=$(date +%s)

for repo_dir in "${REPO_DIRS[@]}"; do
    COUNTER=$((COUNTER + 1))

    # Derive relative path for output structure
    # e.g., repos/github/foo -> github/foo
    rel_path="${repo_dir#"$REPOS_DIR"/}"
    rel_path="${rel_path%/}"

    repo_output="${OUTPUT_ROOT}/${rel_path}"
    repo_name="$rel_path"

    # Count .cbl files
    file_count=$(find "$repo_dir" -name '*.cbl' -o -name '*.cob' -o -name '*.cobol' 2>/dev/null | wc -l)
    file_count=$(echo "$file_count" | tr -d ' ')

    log "[$COUNTER/$TOTAL_REPOS] $repo_name ($file_count files)"

    if [ "$file_count" -eq 0 ]; then
        SKIPPED_REPOS=$((SKIPPED_REPOS + 1))
        log "  Skipped (no COBOL files)"
        continue
    fi

    TOTAL_FILES=$((TOTAL_FILES + file_count))

    # Run transpile (capture stderr for error reporting)
    REPO_ERR=$(mktemp)
    if "$COBOL2RUST" transpile "$repo_dir" \
        --workspace \
        --output "$repo_output" \
        -j "$JOBS" \
        --continue-on-error \
        2>"$REPO_ERR"; then

        SUCCEEDED_REPOS=$((SUCCEEDED_REPOS + 1))

        # Merge NDJSON results into the unified reports directory
        repo_reports="${repo_output}/reports"
        if [ -d "$repo_reports" ]; then
            # Append transpile results with repo_name prefix
            if [ -f "$repo_reports/transpile_results.ndjson" ]; then
                cat "$repo_reports/transpile_results.ndjson" >> "$MERGED_REPORTS/transpile_results.ndjson"
            fi
            if [ -f "$repo_reports/files.ndjson" ]; then
                cat "$repo_reports/files.ndjson" >> "$MERGED_REPORTS/files.ndjson"
            fi
            if [ -f "$repo_reports/diagnostics.ndjson" ]; then
                cat "$repo_reports/diagnostics.ndjson" >> "$MERGED_REPORTS/diagnostics.ndjson"
            fi
            if [ -f "$repo_reports/coverage.ndjson" ]; then
                cat "$repo_reports/coverage.ndjson" >> "$MERGED_REPORTS/coverage.ndjson"
            fi
        fi

        # Extract counts from the repo's meta
        if [ -f "$repo_reports/scan_meta.json" ]; then
            repo_succeeded=$(grep -o '"processed_files":[0-9]*' "$repo_reports/scan_meta.json" 2>/dev/null | cut -d: -f2 || echo 0)
            repo_failed=$(grep -o '"failed_files":[0-9]*' "$repo_reports/scan_meta.json" 2>/dev/null | cut -d: -f2 || echo 0)
            TOTAL_SUCCEEDED=$((TOTAL_SUCCEEDED + repo_succeeded))
            TOTAL_FAILED=$((TOTAL_FAILED + repo_failed))
        fi
    else
        FAILED_REPOS=$((FAILED_REPOS + 1))
        log "  FAILED:"
        if [ -s "$REPO_ERR" ]; then
            # Show last 10 lines of error output
            tail -10 "$REPO_ERR" | while IFS= read -r line; do
                log "    $line"
            done
        fi
    fi

    rm -f "$REPO_ERR"

    # Progress estimate
    ELAPSED=$(($(date +%s) - START_TIME))
    if [ "$COUNTER" -gt 0 ] && [ "$ELAPSED" -gt 0 ]; then
        RATE=$(echo "scale=1; $COUNTER / $ELAPSED" | bc 2>/dev/null || echo "?")
        REMAINING=$(echo "scale=0; ($TOTAL_REPOS - $COUNTER) * $ELAPSED / $COUNTER" | bc 2>/dev/null || echo "?")
        log "  Progress: $COUNTER/$TOTAL_REPOS repos, ${RATE} repos/sec, ETA: ${REMAINING}s"
    fi
done

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

# Write merged scan_meta.json
cat > "$MERGED_REPORTS/scan_meta.json" << METAEOF
{
  "run_id": 1,
  "started_at": "$(date -d @$START_TIME '+%Y-%m-%dT%H:%M:%SZ' 2>/dev/null || date -r $START_TIME '+%Y-%m-%dT%H:%M:%SZ')",
  "finished_at": "$(date -d @$END_TIME '+%Y-%m-%dT%H:%M:%SZ' 2>/dev/null || date -r $END_TIME '+%Y-%m-%dT%H:%M:%SZ')",
  "root_dir": "$REPOS_DIR",
  "phase": "transpile",
  "status": "completed",
  "total_files": $TOTAL_FILES,
  "processed_files": $TOTAL_SUCCEEDED,
  "skipped_files": 0,
  "failed_files": $TOTAL_FAILED,
  "worker_count": $JOBS,
  "batch_size": 0,
  "incremental": false
}
METAEOF

# Ensure empty NDJSON files exist for DuckDB loading
touch "$MERGED_REPORTS/parse_results.ndjson"
touch "$MERGED_REPORTS/copybooks.ndjson"

log ""
log "========================================="
log "Corpus transpile complete"
log "========================================="
log "Duration:        ${DURATION}s"
log "Repos:           $TOTAL_REPOS total, $SUCCEEDED_REPOS succeeded, $FAILED_REPOS failed, $SKIPPED_REPOS skipped"
log "Files:           $TOTAL_FILES total, $TOTAL_SUCCEEDED succeeded, $TOTAL_FAILED failed"
log "Output:          $OUTPUT_ROOT"
log "Merged reports:  $MERGED_REPORTS"
log ""
log "To view reports:"
log "  cobol2rust report --scan-dir $MERGED_REPORTS --type transpile"
log "  cobol2rust report --scan-dir $MERGED_REPORTS --type transpile --format json"
