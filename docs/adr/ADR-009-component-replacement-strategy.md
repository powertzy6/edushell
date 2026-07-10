# ADR-009: Component Replacement Strategy

**Status**: Proposed
**Version**: v1+
**Date**: 2026-07

## Context
EduShell dibangun di atas Cinnamon dengan rencana mengganti setiap komponen secara bertahap (v1→v5). Diperlukan strategi yang jelas agar transisi berjalan mulus tanpa rewrite besar.

## Decision

### Replacement Principles
1. **Abstraction First**: Setiap komponen Cinnamon yang digunakan harus memiliki abstraction layer (libcinnamon-adapter) sejak v1.
2. **Parallel Run**: Komponen baru dan komponen Cinnamon harus bisa berjalan paralel selama transisi.
3. **Feature Parity**: Komponen baru harus memiliki minimal feature parity dengan komponen yang diganti sebelum Cinnamon version dihapus.
4. **Backward Compatibility**: Jangan break existing user configuration saat migrasi.
5. **Incremental Replacement**: Ganti satu komponen per minor/major version.

### Replacement Pipeline
```
Phase 1: Wrap (v1)
  ┌─────────────────────────────┐
  │ EduShell code → Cinnamon    │
  │ via abstraction layer       │
  └─────────────────────────────┘

Phase 2: Replace (v2-v4)
  ┌─────────────────────────────┐
  │ EduShell code → Direct impl │
  │ (Cinnamon bypassed)         │
  └─────────────────────────────┘

Phase 3: Remove (v3-v5)
  ┌─────────────────────────────┐
  │ Cinnamon dependency removed │
  │ from session                 │
  └─────────────────────────────┘
```

### Replacement Order
```
Priority 1 (v2): User-facing components
  - Background → EduWallpaper
  - Screensaver → EduLock
  - Network/applet → EduNetwork
  - Audio applet → EduAudio
  - Tray icons → EduTray (complete)

Priority 2 (v3): Session & files
  - cinnamon-session → EduSession
  - Nemo → EduFiles
  - Cinnamon OSD → EduOSD (complete)

Priority 3 (v4): Window management
  - Muffin → EduWM
  - Mutter → EduCompositor

Priority 4 (v5): Complete independence
  - SDDM/LightDM → EduGreeter
  - GNOME Control Center → EduSystemSettings
```

### Fallback Plan
If replacement N+1 fails or is delayed:
- Previous version continues to work (backward compatible)
- Cinnamon component remains as fallback
- User can switch between old and new via EduSettings

### Risk Mitigation
| Risk | Mitigation |
|------|------------|
| Replacement too complex | Delay to next version; keep abstraction |
| Cinnamon API removed | Detected by CI test; accelerate replacement |
| User resistance to change | Provide Cinnamon fallback option |
| Performance regression | Benchmark before/after each replacement |

### Consequences
**Positive**:
- Roadmap jelas dan predictable
- Setiap versi memiliki nilai tambah
- Tidak ada big-bang rewrite
- User dapat mengikuti transisi secara bertahap

**Negative**:
- Abstraction layer menambah kompleksitas
- Beberapa komponen memiliki dual implementasi sementara
- Memori dan CPU lebih tinggi selama transisi (parallel components)
