# Technical Constraints — EduShell v1

## C1. Cinnamon Dependency Constraints

| Constraint | Detail | Impact |
|------------|--------|--------|
| C1.1 | Wajib berjalan di atas Cinnamon Desktop ≥ 6.x | Semua komponen shell harus kompatibel dengan Cinnamon session lifecycle |
| C1.2 | Window management via Muffin (Cinnamon WM) | Tidak bisa mengubah behavior WM secara fundamental di v1 |
| C1.3 | Display server: apa yang didukung Cinnamon | Wayland (Sessions) atau X11 (legacy) |
| C1.4 | Cinnamon Settings tetap ada sebagai fallback | Edu Settings hanya mengelola pengaturan shell, bukan sistem |
| C1.5 | Cinnamon Applets API tersedia untuk legacy | Boleh digunakan untuk bridging, tidak untuk komponen inti |

## C2. Display Server Constraints

| Constraint | Detail | Impact |
|------------|--------|--------|
| C2.1 | Wayland session: Cinnamon 6.x+ | Shell harus menggunakan Wayland protocols (wlr-layer-shell, etc.) |
| C2.2 | X11 session: fallback | Shell harus tetap berfungsi di X11 via Xlib/XCB, tetapi dengan fitur terbatas |
| C2.3 | No direct DRM/KMS access | Shell tidak mengelola buffer display langsung |
| C2.4 | Fractional scaling di Wayland | Harus handle per-monitor scaling |

## C3. Language & Toolchain Constraints

| Constraint | Detail | Impact |
|------------|--------|--------|
| C3.1 | Bahasa utama: Vala | Kompatibel dengan GTK4, Cinnamon components, GLib ecosystem |
| C3.2 | Bahasa sekunder: Python 3 | Untuk extensions, scripts, Learning Hub content manager |
| C3.3 | Bahasa masa depan: Rust (v3+) | Untuk component replacement yang butuh performa tinggi |
| C3.4 | Build system: Meson | Standar GNOME/Cinnamon ecosystem |
| C3.5 | GTK Version: 4.12+ | Modern GTK toolkit |
| C3.6 | Compiler: valac (Vala), gcc (C), python3 | Sesuai toolchain distribusi |

## C4. Distribution Packaging Constraints

| Constraint | Detail | Impact |
|------------|--------|--------|
| C4.1 | Packaging format: .deb (ubuntu/mint/debian) | Paket harus mengikuti Debian policy |
| C4.2 | Dependency pada paket Cinnamon standar | Tidak boleh memodifikasi paket Cinnamon dari repository |
| C4.3 | Installasi non-destruktif | Tidak boleh menghapus/menonaktifkan komponen Cinnamon default |
| C4.4 | User session switching (Cinnamon ↔ EduShell) | Harus seamless, bisa dipilih di login manager |

## C5. Hardware Constraints

| Constraint | Detail | Impact |
|------------|--------|--------|
| C5.1 | RAM minimum: 4GB | Shell harus idle di 500-650MB, menyisakan RAM untuk aplikasi |
| C5.2 | CPU minimum: Intel Celeron N3060 (2 core, 1.6GHz) | Tidak ada background thread yang berat |
| C5.3 | GPU: Intel HD Graphics (Broadwell, Haswell era) | Tidak ada efek GPU-heavy. Animasi minimal via GTK |
| C5.4 | Storage: minimum 128GB SSD | EduShell sendiri hanya butuh < 50MB |
| C5.5 | Display: 1366x768 minimum | Layout harus responsif |

## C6. Accessibility Constraints

| Constraint | Detail | Impact |
|------------|--------|--------|
| C6.1 | WCAG 2.2 AA compliance target | Semua komponen UI harus accessible |
| C6.2 | AT-SPI2 protocol support | Semua widget harus expose proper accessible interfaces |
| C6.3 | Orca screen reader compatible | Test dengan Orca sebelum rilis |

## C7. Security Constraints

| Constraint | Detail | Impact |
|------------|--------|--------|
| C7.1 | No setuid binaries | Semua berjalan di user space |
| C7.2 | No polkit escalation from shell | Operasi administratif via Settings → Cinnamon → pkexec |
| C7.3 | Config files follow XDG spec | `~/.config/edushell/`, `~/.local/share/edushell/` |
| C7.4 | No network services | EduShell tidak membuka port listening |
