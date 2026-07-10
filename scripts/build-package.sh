#!/usr/bin/env bash
set -euo pipefail

# EduShell Build Script — creates Debian package from release build
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"
PKG_NAME="edushell"
PKG_VERSION="${1:-1.0.0}"
PKG_DIR="${BUILD_DIR}/${PKG_NAME}_${PKG_VERSION}-1_amd64"

echo "=== Building EduShell v${PKG_VERSION} package ==="

# Build release binaries
echo "Building release binaries..."
cd "${SCRIPT_DIR}"
cargo build --release -p edushell-core2 -p edushell-sdk -p edushell-cli -p edushell-apps -p eduwm

# Create package directory structure
echo "Creating package structure..."
mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/bin"
mkdir -p "${PKG_DIR}/usr/share/doc/edushell"
mkdir -p "${PKG_DIR}/usr/share/applications"
mkdir -p "${PKG_DIR}/usr/share/wayland-sessions"
mkdir -p "${PKG_DIR}/usr/share/icons/hicolor/scalable/apps"

# Copy binaries
echo "Copying binaries..."
cp target/release/edushell-cli "${PKG_DIR}/usr/bin/"

# Copy desktop files
cp packaging/edushell.desktop "${PKG_DIR}/usr/share/applications/"
cp packaging/edushell-session.desktop "${PKG_DIR}/usr/share/wayland-sessions/"

# Copy branding
cp branding/edushell.svg "${PKG_DIR}/usr/share/icons/hicolor/scalable/apps/" 2>/dev/null || true

# Copy documentation
cp README.md "${PKG_DIR}/usr/share/doc/edushell/" 2>/dev/null || true
cp docs/architecture/ARCHITECTURE_OVERVIEW.md "${PKG_DIR}/usr/share/doc/edushell/" 2>/dev/null || true
cp LICENSE "${PKG_DIR}/usr/share/doc/edushell/" 2>/dev/null || true

# Generate Debian control file
cat > "${PKG_DIR}/DEBIAN/control" << 'CONTROL'
Package: edushell
Version: VERSION_PLACEHOLDER
Section: x11
Priority: optional
Architecture: amd64
Maintainer: EduShell Team <dev@edushell.id>
Description: EduShell — Modern educational desktop environment
 EduShell is a modern, modular desktop environment designed for
 education. Built on Rust, it provides a clean, performant,
 and accessible computing experience.
 .
 This package provides the EduShell desktop session and SDK.
CONTROL

# Generate postinst
cat > "${PKG_DIR}/DEBIAN/postinst" << 'POSTINST'
#!/bin/sh
set -e
echo "EduShell installed. Select 'EduShell' from your login manager."
POSTINST
chmod 755 "${PKG_DIR}/DEBIAN/postinst"

# Build .deb
echo "Building .deb package..."
fakeroot dpkg-deb --build "${PKG_DIR}" 2>/dev/null || {
    # Try without fakeroot
    dpkg-deb --build "${PKG_DIR}"
}

echo ""
echo "Package built: ${BUILD_DIR}/${PKG_NAME}_${PKG_VERSION}-1_amd64.deb"
