#!/bin/bash
set -e

# ---------------------------------------------------------------------------
# EduShell Session Fix — one-shot installer
# ---------------------------------------------------------------------------
cd "$(dirname "$(readlink -f "$0")")"

echo "=== 1. Build edushell-session ==="
cargo build --release -p edushell-session

echo ""
echo "=== 2. Install binary ==="
sudo install -Dm755 target/release/edushell-session /usr/local/bin/

echo ""
echo "=== 3. Install shell script (backup) ==="
sudo install -Dm755 packaging/edushell-session.sh /usr/local/bin/

echo ""
echo "=== 4. Copy desktop file ==="
sudo mkdir -p /usr/share/xsessions
sudo cp packaging/edushell-session.desktop /usr/share/xsessions/edushell.desktop
sudo rm -f /usr/local/share/xsessions/edushell.desktop
sudo rm -f /usr/share/wayland-sessions/edushell.desktop 2>/dev/null || true

echo ""
echo "=== 5. Verify ==="
echo "    desktop : /usr/share/xsessions/edushell.desktop"
cat /usr/share/xsessions/edushell.desktop
echo ""
echo "    binary  : /usr/local/bin/edushell-session"
file /usr/local/bin/edushell-session
echo ""
echo "    script  : /usr/local/bin/edushell-session.sh"
file /usr/local/bin/edushell-session.sh

echo ""
echo "=== 6. Test from terminal ==="
echo "Run: /usr/local/bin/edushell-session"
echo ""
echo "DONE — logout & select EduShell"
