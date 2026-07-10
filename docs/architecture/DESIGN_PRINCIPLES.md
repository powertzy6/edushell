# EduShell Design Principles

## UI Design Principles

### P1. Clarity First
- Setiap elemen UI harus langsung dapat dipahami dalam 2 detik
- Gunakan label teks yang jelas — jangan hanya ikon
- Hindari ambiguity visual

### P2. Visual Hierarchy
- Informasi paling penting harus paling menonjol
- Gunakan ukuran, warna, dan spacing secara konsisten
- Kontras tinggi antara foreground dan background

### P3. Consistency
- Gunakan design tokens yang konsisten (spacing, radius, color, typography)
- Pola interaksi yang sama untuk fungsi yang sama
- Paddle/stik navigasi yang konsisten di seluruh shell

### P4. Responsive Layout
- Mendukung berbagai ukuran layar (1366x768 hingga 3840x2160)
- Scaling fraction (100%, 125%, 150%, 200%)
- Panel dan elemen harus menyesuaikan secara proporsional

### P5. Gentle Animations
- Animasi < 200ms untuk feedback instan
- Animasi < 400ms untuk transisi
- Animasi < 600ms untuk notifikasi
- Respect `prefers-reduced-motion`

### P6. Color System
- Primary: Biru laut dalam (#1A237E variasi) — melambangkan kepercayaan, pengetahuan
- Accent: Hijau toska (#00897B variasi) — melambangkan pertumbuhan, edukasi
- Warning: Oranye (#E65100)
- Error: Merah (#C62828)
- Success: Hijau (#2E7D32)
- Background netral: Putih/abu-abu sangat terang (light mode), abu-abu sangat gelap (dark mode)
- Mode gelap wajib ada sejak v1

### P7. Typography
- Font utama: Inter (sans-serif, legible di layar)
- Font monospace: JetBrains Mono (untuk terminal/code)
- Base size: 14px
- Scale: 1.25 (Major Third)
- Line height: 1.5 minimum

### P8. Spacing System
- Base unit: 4px
- Scale: 4, 8, 12, 16, 24, 32, 48, 64, 96
- Padding konsisten di semua komponen

### P9. Iconography
- Gunakan symbolic/icons outline style
- Set ikon: Papirus atau ikon buatan sendiri
- Ukuran ikon konsisten: 16px (panel), 24px (button), 32px (list item), 48px (app icon)

### P10. Dark Mode
- Wajib tersedia sejak v1
- Ikuti sistem preference
- Toggle manual di Settings

## UX Design Principles

### UX1. Zero Learning Curve for Basic Tasks
- Membuka aplikasi: 1 klik (launcher) atau Super+Space
- Mengganti workspace: 1 klik atau Super+Arrow
- Mengatur volume/wifi: 1 klik di system tray
- Shutdown: 1 klik di menu

### UX2. Progressive Complexity
- Interface default: hanya fitur dasar terlihat
- Fitur lanjutan: tersedia di Settings atau menu konteks
- Pengguna "tumbuh" bersama desktop

### UX3. Forgiving Design
- Setiap aksi destruktif memiliki konfirmasi atau undo
- Dialog konfirmasi: "Apakah Anda yakin ingin mematikan komputer?"
- Tidak ada aksi yang tidak bisa dikembalikan

### UX4. Feedback Loop
- Setiap aksi pengguna harus mendapat feedback visual < 100ms
- Loading state wajib untuk operasi > 300ms
- Error state harus informatif, bukan technical

### UX5. Discoverability
- Semua fitur utama dapat ditemukan dalam 2 klik
- Learning Hub muncul di panel default
- Search bar di launcher dapat menemukan aplikasi, file, settings

### UX6. Accessibility-first Navigation
- Seluruh fungsi shell dapat diakses via keyboard
- Focus indicator jelas dan kontras
- Tab order logis
- Skip-to-content untuk screen reader
