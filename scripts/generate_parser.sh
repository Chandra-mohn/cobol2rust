#!/usr/bin/env bash
# generate_parser.sh -- Generate Rust parser from COBOL ANTLR4 grammars.
#
# Uses the project-local ANTLR4 fork JAR (tools/antlr4-rust.jar) which
# supports the Rust target. This does NOT use or affect the system antlr4
# installation.
#
# Usage:
#   ./scripts/generate_parser.sh          # from project root
#   ./scripts/generate_parser.sh --check  # verify JAR exists without generating

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

JAR="$PROJECT_ROOT/tools/antlr4-rust.jar"
GRAMMAR_DIR="$PROJECT_ROOT/grammar"
OUTPUT_DIR="$PROJECT_ROOT/crates/cobol-transpiler/src/generated"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

# Verify prerequisites
check_prerequisites() {
    if ! command -v java &>/dev/null; then
        error "Java not found. ANTLR4 requires Java to run."
        exit 1
    fi

    if [ ! -f "$JAR" ]; then
        error "ANTLR4 Rust JAR not found at: $JAR"
        echo ""
        echo "Download it with:"
        echo "  curl -L -o $JAR \\"
        echo "    https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar"
        exit 1
    fi

    info "Java: $(java -version 2>&1 | head -1)"
    info "ANTLR4 JAR: $JAR"
}

# Generate Rust code from a grammar file
generate() {
    local grammar="$1"
    local name
    name="$(basename "$grammar" .g4)"

    info "Generating Rust from $name.g4 ..."

    java -jar "$JAR" \
        -Dlanguage=Rust \
        -visitor \
        -o "$OUTPUT_DIR" \
        "$grammar"

    info "$name -- done"
}

# Main
main() {
    if [ "${1:-}" = "--check" ]; then
        check_prerequisites
        info "All prerequisites OK."
        exit 0
    fi

    check_prerequisites

    # Clean previous generated .rs files (but not mod.rs)
    info "Cleaning previous generated files ..."
    find "$OUTPUT_DIR" -name '*.rs' ! -name 'mod.rs' -delete
    find "$OUTPUT_DIR" -name '*.interp' -delete
    find "$OUTPUT_DIR" -name '*.tokens' -delete

    # Generate from both grammars
    generate "$GRAMMAR_DIR/Cobol85.g4"
    generate "$GRAMMAR_DIR/Cobol85Preprocessor.g4"

    # Summary
    echo ""
    info "Generated files:"
    ls -lh "$OUTPUT_DIR"/*.rs | while read -r line; do
        echo "  $line"
    done

    echo ""
    info "Parser generation complete. Run 'cargo build -p cobol-transpiler' to verify."
}

main "$@"
