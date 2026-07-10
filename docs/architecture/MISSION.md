# EduShell Mission Document

## Mission Statement
Membangun, memelihara, dan terus mengembangkan Desktop Shell Linux yang:

1. **Sederhana** — sehingga pelajar SMP dapat langsung menggunakannya tanpa pelatihan.
2. **Modern** — dengan desain UI yang relevan di tahun 2026 dan seterusnya.
3. **Edukatif** — menampilkan Learning Hub, alat bantu tugas, dan akses cepat ke sumber belajar.
4. **Ringan** — tetap responsif di laptop berspesifikasi rendah.
5. **Stabil** — siap digunakan untuk kegiatan belajar harian tanpa crash.
6. **Open Source** — transparan, dapat diaudit, dan dikembangkan bersama komunitas.
7. **Berbahasa Indonesia** — antarmuka sepenuhnya dalam Bahasa Indonesia secara default.

## Scope

### In Scope (v1)
- Custom Desktop Shell (panel, app launcher, workspace switcher, system tray)
- Cinnamon sebagai backend window management dan session management
- Edu Settings (panel pengaturan khusus EduShell)
- Learning Hub (portal konten edukasi)
- Theme & visual identity EduShell
- Wayland + X11 compatibility
- Bahasa Indonesia + English translation system
- Keyboard navigation & screen reader readiness

### Out of Scope (v1)
- Window Manager sendiri (masih pakai Muffin dari Cinnamon)
- Display server sendiri (masih pakai Wayland/X11 dari Cinnamon)
- Application suite sendiri (masih pakai aplikasi GNOME/GTK)
- File manager sendiri (masih pakai Nemo)
- Screen recorder / screenshot built-in

### Future Scope (v2–v5)
- Aplikasi edukasi bawaan (EduApps)
- Fork komponen Cinnamon tertentu
- Custom Window Manager
- Custom Display Server
- Desktop Environment mandiri

## Core Activities
1. Merancang dan mengimplementasikan shell UI yang bersih dan modern
2. Membangun sistem plugin/ekstensi untuk shell
3. Membangun Learning Hub sebagai portal konten edukasi
4. Membangun Edu Settings sebagai panel kontrol terpadu
5. Membangun pipeline translation dan CI/CD
6. Menulis dokumentasi pengguna dan pengembang
7. Membangun komunitas pengguna dan kontributor di Indonesia
