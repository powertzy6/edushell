# EDUSHELL FIX — jalankan baris satu per satu
# ============================================

cd ~/Documents/DS
cargo build --release -p edushell-session

# Password akan diminta di sini
sudo install -Dm755 target/release/edushell-session /usr/local/bin/
sudo install -Dm755 packaging/edushell-session.sh /usr/local/bin/
sudo mkdir -p /usr/share/xsessions
sudo cp packaging/edushell-session.desktop /usr/share/xsessions/edushell.desktop
sudo rm -f /usr/local/share/xsessions/edushell.desktop
sudo rm -f /usr/share/wayland-sessions/edushell.desktop 2>/dev/null

# Test dulu — kalo work, baru logout
/usr/local/bin/edushell-session
