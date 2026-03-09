#!/bin/bash
# ================================================================
# rust-dev-setup.sh -- Prepare development environment for Rust
#
# Run this before starting a Rust development session.
# Creates a RAM disk for build artifacts and installs/verifies
# build acceleration tools.
#
# Usage:
#   ./scripts/rust-dev-setup.sh          # Full setup
#   ./scripts/rust-dev-setup.sh ramdisk  # RAM disk only
#   ./scripts/rust-dev-setup.sh tools    # Tool install only
#   ./scripts/rust-dev-setup.sh status   # Check current state
#   ./scripts/rust-dev-setup.sh teardown # Remove RAM disk
# ================================================================

set -euo pipefail

RAMDISK_NAME="RustBuild"
RAMDISK_SIZE_GB=8
RAMDISK_MOUNT="/Volumes/${RAMDISK_NAME}"
TARGET_SUBDIR="cobol2rust-target"

# Colors (ASCII-safe)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info()  { echo -e "${CYAN}[INFO]${NC} $1"; }
ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
fail()  { echo -e "${RED}[FAIL]${NC} $1"; }

# ----------------------------------------------------------------
# RAM Disk
# ----------------------------------------------------------------
create_ramdisk() {
    if mount | grep -q "${RAMDISK_MOUNT}"; then
        ok "RAM disk already mounted at ${RAMDISK_MOUNT}"
        mkdir -p "${RAMDISK_MOUNT}/${TARGET_SUBDIR}"
        return 0
    fi

    info "Creating ${RAMDISK_SIZE_GB}GB RAM disk..."
    local sectors=$(( RAMDISK_SIZE_GB * 1024 * 2048 ))
    local device
    device=$(hdiutil attach -nomount "ram://${sectors}")
    device=$(echo "${device}" | xargs)  # trim whitespace

    if [ -z "${device}" ]; then
        fail "Failed to create RAM disk"
        return 1
    fi

    diskutil erasevolume HFS+ "${RAMDISK_NAME}" "${device}" > /dev/null 2>&1

    if mount | grep -q "${RAMDISK_MOUNT}"; then
        ok "RAM disk created: ${RAMDISK_MOUNT} (${RAMDISK_SIZE_GB}GB)"
        mkdir -p "${RAMDISK_MOUNT}/${TARGET_SUBDIR}"
        ok "Target directory: ${RAMDISK_MOUNT}/${TARGET_SUBDIR}"
    else
        fail "RAM disk creation failed"
        return 1
    fi
}

teardown_ramdisk() {
    if mount | grep -q "${RAMDISK_MOUNT}"; then
        info "Ejecting RAM disk..."
        diskutil eject "${RAMDISK_MOUNT}" > /dev/null 2>&1
        ok "RAM disk ejected. Build artifacts are gone."
    else
        warn "No RAM disk mounted at ${RAMDISK_MOUNT}"
    fi
}

# ----------------------------------------------------------------
# Tool installation
# ----------------------------------------------------------------
install_tools() {
    info "Checking Rust build acceleration tools..."
    echo ""

    # 1. sccache -- compiler cache
    if command -v sccache > /dev/null 2>&1; then
        ok "sccache $(sccache --version 2>/dev/null | head -1)"
    else
        info "Installing sccache (compiler cache)..."
        cargo install sccache --locked
        ok "sccache installed"
    fi

    # 2. cargo-nextest -- faster test runner
    if command -v cargo-nextest > /dev/null 2>&1; then
        ok "cargo-nextest installed"
    else
        info "Installing cargo-nextest (parallel test runner)..."
        cargo install cargo-nextest --locked
        ok "cargo-nextest installed"
    fi

    # 3. cargo-watch -- auto-rebuild on file change
    if command -v cargo-watch > /dev/null 2>&1; then
        ok "cargo-watch installed"
    else
        info "Installing cargo-watch (auto-rebuild)..."
        cargo install cargo-watch --locked
        ok "cargo-watch installed"
    fi

    echo ""
    ok "All tools verified"
}

# ----------------------------------------------------------------
# Status check
# ----------------------------------------------------------------
show_status() {
    echo ""
    echo "=== Rust Development Environment Status ==="
    echo ""

    # RAM disk
    if mount | grep -q "${RAMDISK_MOUNT}"; then
        local used
        used=$(df -h "${RAMDISK_MOUNT}" | tail -1 | awk '{print $3}')
        local avail
        avail=$(df -h "${RAMDISK_MOUNT}" | tail -1 | awk '{print $4}')
        ok "RAM disk: ${RAMDISK_MOUNT} (used: ${used}, free: ${avail})"
    else
        warn "RAM disk: not mounted"
    fi

    # Rust
    if command -v rustc > /dev/null 2>&1; then
        ok "rustc: $(rustc --version)"
    else
        fail "rustc: not installed"
    fi

    if command -v cargo > /dev/null 2>&1; then
        ok "cargo: $(cargo --version)"
    else
        fail "cargo: not installed"
    fi

    # Acceleration tools
    if command -v sccache > /dev/null 2>&1; then
        ok "sccache: $(sccache --version 2>/dev/null | head -1)"
    else
        warn "sccache: not installed (run: cargo install sccache)"
    fi

    if command -v cargo-nextest > /dev/null 2>&1; then
        ok "cargo-nextest: installed"
    else
        warn "cargo-nextest: not installed (run: cargo install cargo-nextest)"
    fi

    if command -v cargo-watch > /dev/null 2>&1; then
        ok "cargo-watch: installed"
    else
        warn "cargo-watch: not installed (run: cargo install cargo-watch)"
    fi

    # System info
    echo ""
    local ram
    ram=$(sysctl -n hw.memsize | awk '{printf "%.0f", $1/1073741824}')
    local cores
    cores=$(sysctl -n hw.ncpu)
    info "System: ${ram}GB RAM, ${cores} CPU cores"
    echo ""
}

# ----------------------------------------------------------------
# Main
# ----------------------------------------------------------------
main() {
    local cmd="${1:-all}"

    echo ""
    echo "========================================"
    echo "  Rust Development Environment Setup"
    echo "  (cobol2rust project)"
    echo "========================================"
    echo ""

    case "${cmd}" in
        ramdisk)
            create_ramdisk
            ;;
        tools)
            install_tools
            ;;
        status)
            show_status
            ;;
        teardown)
            teardown_ramdisk
            ;;
        all)
            create_ramdisk
            install_tools
            echo ""
            echo "========================================"
            info "Setup complete. Summary:"
            show_status
            ;;
        *)
            echo "Usage: $0 [all|ramdisk|tools|status|teardown]"
            echo ""
            echo "  all       Full setup (default)"
            echo "  ramdisk   Create RAM disk only"
            echo "  tools     Install build tools only"
            echo "  status    Show current state"
            echo "  teardown  Remove RAM disk"
            exit 1
            ;;
    esac
}

main "$@"
