# ADR-003: Display Server Strategy — Wayland Primer, X11 Fallback

**Status**: Proposed
**Version**: v1
**Date**: 2026-07

## Context
Desktop shell membutuhkan display server. Di Cinnamon v6+, Wayland session tersedia tetapi masih dalam transisi dari X11. EduShell harus memutuskan strategi display server.

## Decision
**Wayland** sebagai display server primer. **X11** sebagai fallback.

### Wayland Features Used
- `wlr-layer-shell` protocol untuk panel positioning
- `wlr-foreign-toplevel` untuk window management
- GTK4 Wayland backend (built-in)
- Per-monitor scaling

### X11 Fallback
- Jika Wayland session Cinnamon tidak stabil
- Jika GPU/driver tidak support Wayland
- Menggunakan GDK X11 backend

### Detection Logic
```
if (WAYLAND_DISPLAY is set && wlr-layer-shell available)
    → Wayland mode (full features)
else if (WAYLAND_DISPLAY is set)
    → Wayland mode (basic features, no layer-shell)
else if (DISPLAY is set)
    → X11 fallback mode
```

### Consequences
**Positive**:
- Masa depan Wayland — investasi di protokol yang benar
- GTK4 memiliki Wayland backend native
- X11 fallback memberikan safety net

**Negative**:
- wlr-layer-shell tidak terstandarisasi di semua Wayland compositor
- Cinnamon Wayland belum siap produksi di semua hardware
- X11 memiliki keterbatasan (no per-monitor scaling, no fractional scaling)
- Dua code path untuk beberapa fitur

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| **X11 only** | Tidak future-proof. Semua distribusi migrasi ke Wayland. |
| **Wayland only** | Terlalu berisiko untuk hardware lama dan NVIDIA. |
| **Mir** | Hampir tidak digunakan di ecosystem Linux desktop. |
