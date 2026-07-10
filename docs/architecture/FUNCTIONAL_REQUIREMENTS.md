# Functional Requirements — EduShell v1

## F1. Desktop Shell Core

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F1.1 | Panel utama di bagian bawah layar (default) | P0 | Bisa dipindahkan ke atas/kiri/kanan via Settings |
| F1.2 | Panel menampilkan: menu Edu, running apps, system tray, jam | P0 | |
| F1.3 | Menu Edu (App Launcher) dengan search | P0 | Seperti GNOME Activities tetapi lebih ringan |
| F1.4 | App grid dengan kategori default: Education, Office, Internet, Development, Settings | P0 | |
| F1.5 | Favorite apps (bisa pin/unpin) | P0 | |
| F1.6 | Recent apps section | P1 | |
| F1.7 | Search bar di launcher mencakup: aplikasi, file, settings | P0 | Menggunakan tracker/miner sebagai backend |
| F1.8 | Running apps indicator di panel | P0 | |
| F1.9 | Workspace switcher di panel | P0 | Minimal 4 workspace |
| F1.10 | Workspace preview saat switching | P1 | |
| F1.11 | System tray: volume, jaringan, baterai, notifikasi, user menu | P0 | |
| F1.12 | Quick settings panel (toggle WiFi, Bluetooth, Dark Mode, DND) | P0 | |
| F1.13 | Notification center (riwayat notifikasi) | P0 | |
| F1.14 | User menu: Account settings, Lock, Logout, Suspend, Restart, Shutdown | P0 | |
| F1.15 | Clock/calendar dropdown di panel | P0 | Menampilkan tanggal, agenda (jika ada) |
| F1.16 | Autohide panel (opsi) | P2 | |
| F1.17 | Panel transparency / style customization | P1 | |

## F2. Edu Settings

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F2.1 | Settings app khusus EduShell | P0 | Bukan Cinnamon Settings |
| F2.2 | Pengaturan panel: posisi, autohide, style | P0 | |
| F2.3 | Pengaturan workspace: jumlah, shortcut | P0 | |
| F2.4 | Pengaturan launcher: categories, favorites | P0 | |
| F2.5 | Pengaturan tampilan: tema, font, scaling | P0 | |
| F2.6 | Pengaturan bahasa: Indonesia / English | P0 | |
| F2.7 | Pengaturan aksesibilitas: high contrast, large font, screen reader | P0 | |
| F2.8 | Pengaturan notifikasi: DND, per-app | P1 | |
| F2.9 | Pengaturan shortcut keyboard | P0 | |
| F2.10 | Pengaturan akun: avatar, password, auto-login | P1 | |
| F2.11 | Pengaturan Learning Hub | P0 | |
| F2.12 | Pengaturan update | P1 | |
| F2.13 | Factory reset EduShell settings | P2 | |

## F3. Learning Hub

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F3.1 | Portal edukasi bawaan di panel | P0 | Ikon buku terbuka |
| F3.2 | Halaman: "Mulai Belajar Linux" (panduan dasar) | P0 | Konten statis HTML/markdown |
| F3.3 | Halaman: "Tips & Trik Desktop" | P0 | |
| F3.4 | Halaman: "Aplikasi Edukasi" (rekomendasi dan link instalasi) | P0 | |
| F3.5 | Halaman: "Shortcut Keyboard" | P0 | |
| F3.6 | Halaman: "Komunitas" (link forum Indonesia) | P1 | |
| F3.7 | Learning Hub dapat diperbarui secara independen | P1 | |
| F3.8 | Konten Learning Hub dalam Bahasa Indonesia | P0 | |
| F3.9 | Konten Learning Hub dalam English | P1 | |

## F4. Theme & Visual

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F4.1 | GTK theme EduShell (light + dark variant) | P0 | |
| F4.2 | Shell theme (panel, launcher, OSD) | P0 | |
| F4.3 | Icon theme EduShell | P0 | Based on Papirus atau custom |
| F4.4 | Cursor theme EduShell | P1 | |
| F4.5 | Wallpaper EduShell default | P0 | |
| F4.6 | Font rendering optimization | P0 | |
| F4.7 | Sound theme (subtle) | P2 | |

## F5. Accessibility

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F5.1 | Full keyboard navigation with visible focus | P0 | |
| F5.2 | High contrast theme variant | P0 | |
| F5.3 | Large font mode (1.25x, 1.5x, 2x) | P0 | |
| F5.4 | Screen reader compatible (Orca) | P0 | ARIA labels, accessible names |
| F5.5 | Respect prefers-reduced-motion | P0 | |
| F5.6 | Respect prefers-contrast | P1 | |
| F5.7 | On-screen keyboard (optional toggle) | P2 | |

## F6. Security

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F6.1 | Tidak ada service yang berjalan sebagai root | P0 | |
| F6.2 | Semua konfigurasi di user space | P0 | |
| F6.3 | Sesuai XDG Base Directory Specification | P0 | |
| F6.4 | Sanitasi input untuk IPC dan file config | P0 | |
| F6.5 | Tidak ada hardcoded credentials | P0 | |

## F7. Translation & Internationalization

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F7.1 | Semua string menggunakan gettext | P0 | |
| F7.2 | Bahasa Indonesia sebagai default | P0 | |
| F7.3 | English sebagai fallback | P0 | |
| F7.4 | Translation files di direktori terpisah | P0 | |
| F7.5 | Potensi untuk bahasa daerah | P2 | |

## F8. Integration

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F8.1 | Integrasi dengan Cinnamon session manager | P0 | |
| F8.2 | Integrasi dengan NetworkManager (via Cinnamon) | P0 | |
| F8.3 | Integrasi dengan PulseAudio/WirePlumber (via Cinnamon) | P0 | |
| F8.4 | Integrasi dengan UPower (baterai) | P0 | |
| F8.5 | Dukungan untuk Cinnamon Applets/Extensions (plugin system) | P1 | |

## F9. Performance

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| F9.1 | Panel idle CPU < 1% pada hardware target | P0 | |
| F9.2 | Launcher search response < 200ms | P0 | |
| F9.3 | Startup time < 10 detik ke desktop | P0 | Dari login manager ke shell siap pakai |
| F9.4 | Notification handling tidak mengganggu aplikasi fullscreen | P0 | |
| F9.5 | Zero unnecessary background processes | P0 | |
