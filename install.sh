#!/bin/bash
# EduShell v1.0 Installer
set -e

VERSION="1.0.0"
PREFIX="${PREFIX:-/usr/local}"

echo "EduShell v${VERSION} Installer"
echo "============================="

# Check for Rust toolchain
if ! command -v cargo &>/dev/null; then
    echo "ERROR: Rust toolchain not found."
    echo "Install it from https://rustup.rs"
    exit 1
fi

echo "[1/4] Building EduShell..."
cargo build --release --workspace

echo "[2/4] Installing binaries..."
install -Dm755 target/release/edushell-daemon "${PREFIX}/bin/edushell-daemon"
install -Dm755 target/release/edushell-cli "${PREFIX}/bin/edushell-cli"
install -Dm755 target/release/edushell-session "${PREFIX}/bin/edushell-session" 2>/dev/null || true

echo "[3/4] Installing desktop files..."
install -Dm644 packaging/edushell.desktop "${PREFIX}/share/applications/edushell.desktop"
install -Dm644 packaging/edushell-session.desktop "${PREFIX}/share/xsessions/edushell.desktop"

echo "[4/4] Installing data files..."
install -Dm644 branding/logo.svg "${PREFIX}/share/icons/hicolor/scalable/apps/edushell.svg" 2>/dev/null || true

echo ""
echo "EduShell v${VERSION} installed successfully!"
echo "Log out and select EduShell from your display manager."
