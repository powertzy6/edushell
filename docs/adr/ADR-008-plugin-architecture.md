# ADR-008: Plugin Architecture untuk Ekstensi (v3+)

**Status**: Deferred (v3+)
**Version**: v3
**Date**: 2026-07

## Context
EduShell membutuhkan sistem ekstensi agar pengembang dapat menambahkan fungsionalitas tanpa mengubah kode inti. Cinnamon memiliki Applet/Extension API, tetapi EduShell akan menggantinya di v3+.

## Decision (Deferred)
Keputusan ditunda hingga v3. Untuk v1, gunakan Cinnamon Applet API via applet-bridge.

### For v3+ (Proposed Direction)
1. **Plugin types**: Applets (panel widgets), Extensions (system-level), Themes
2. **Language**: Vala (native), Python (scripting), JavaScript (lightweight)
3. **Isolation**: Setiap plugin berjalan di proses terpisah (sandboxed)
4. **API**: DBus-based communication
5. **Security**: Plugin memerlukan user approval untuk akses sistem
6. **Package format**: `.edushugin` (zip with metadata)

### Notes for v1
- Jangan over-engineer plugin system di v1
- Cinnamon applet-bridge cukup untuk kompatibilitas mundur
- Desain API plugin dimulai di v2, implementasi di v3

## Alternatives (for v3+)

| Alternative | Notes |
|-------------|-------|
| **Cinnamon Applet API** | Legacy, akan diganti |
| **GNOME Shell Extension** | Terlalu terikat GNOME |
| **Plasma Widget** | Qt stack |
| **Web-based plugins** | Terlalu berat untuk target hardware |
