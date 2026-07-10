# ADR-001: Vala & GTK4 sebagai Bahasa dan Toolkit Utama

**Status**: Proposed
**Version**: v1
**Date**: 2026-07

## Context
EduShell membutuhkan bahasa pemrograman dan UI toolkit untuk membangun komponen shell. Pilihan harus:
1. Kompatibel dengan ecosystem GNOME/Cinnamon
2. Produktif untuk satu pengembang
3. Cukup cepat untuk desktop shell
4. Mendapatkan GTK4 (modern toolkit)
5. Mendukung translasi, aksesibilitas, dan theming

## Decision
Gunakan **Vala** sebagai bahasa utama dan **GTK4** sebagai UI toolkit.

### Rationale
1. **Vala kompatibel dengan Cinnamon**: Cinnamon menggunakan GJS (JavaScript) dan C, tetapi Vala menghasilkan kode C yang kompatibel dengan GLib/GTK4.
2. **Produktivitas**: Vala lebih produktif daripada C murni — memiliki OOP, signal/slot, manajemen memori otomatis (reference counting).
3. **GTK4 native binding**: Vala memiliki binding GTK4 yang first-class. Tidak perlu menulis binding manual.
4. **Translasi**: gettext terintegrasi di Vala.
5. **Aksesibilitas**: GTK4 memiliki dukungan AT-SPI2 built-in.
6. **Ecosystem**: Banyak contoh kode shell di Vala (Budgie, Pantheon).

### Consequences
**Positive**:
- Produktivitas lebih tinggi daripada C
- Kompatibel dengan Cinnamon ecosystem tanpa rewrite
- GTK4 memberikan modern UI toolkit
- Komunitas Vala masih aktif (meskipun kecil)

**Negative**:
- Vala bukan bahasa mainstream — lebih sulit cari kontributor
- Dokumentasi Vala terbatas
- Beberapa library mungkin tidak memiliki binding Vala
- Tooling lebih terbatas daripada C, C++, atau Rust

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| **C + GTK4** | Terlalu verbose untuk satu pengembang. Produktivitas rendah. |
| **Python + GTK4** | Terlalu lambat untuk desktop shell. GIL membatasi performance. |
| **Rust + GTK4** | Terlalu kompleks untuk v1. Rust akan digunakan di v3+ untuk komponen tertentu. |
| **JavaScript + GTK4 (GJS)** | Performance tidak cukup untuk shell. Cinnamon menggunakan GJS dan sering lambat. |
| **C++ + Qt6** | Tidak kompatibel dengan Cinnamon ecosystem. Qt tidak digunakan di Cinnamon. |
