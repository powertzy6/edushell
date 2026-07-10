# ADR-002: Cinnamon Desktop sebagai Base System

**Status**: Proposed
**Version**: v1
**Date**: 2026-07

## Context
EduShell membutuhkan foundation untuk:
1. Window management
2. Session management
3. Display server integration
4. System service access (NetworkManager, PulseAudio, UPower)

Opsi:
1. Build everything from scratch
2. Gunakan Cinnamon Desktop sebagai base
3. Gunakan GNOME Shell sebagai base
4. Gunakan Budgie sebagai base

## Decision
Gunakan **Cinnamon Desktop v6+** sebagai base system.

### Rationale
1. **Cinnamon sudah matang**: Produksi selama 10+ tahun, stable, komunitas besar.
2. **Stack yang tepat**: Cinnamon menggunakan Muffin (WM) + GTK — sesuai dengan tech stack EduShell.
3. **Linux Mint base**: Distribusi target (Mint, Ubuntu) semuanya mendukung Cinnamon.
4. **Wayland support**: Cinnamon 6.x sudah memiliki Wayland session.
5. **Modular**: Cinnamon memisahkan panel, WM, session — mudah diganti bertahap.
6. **Familiar**: Tim dan kontributor potensial sudah familiar dengan Cinnamon.

### Consequences
**Positive**:
- Tidak perlu build WM, compositor, session manager dari awal
- Stabilitas terjamin oleh Cinnamon team
- Ecosystem besar (applets, extensions, themes)
- Distribusi target sudah mendukung

**Negative**:
- Terikat dengan Cinnamon release cycle
- Beberapa bug Cinnamon akan terasa di EduShell
- Tidak bisa mengubah behavior WM secara fundamental di v1
- Cinnamon Wayland masih baru — ada risiko ketidakstabilan

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| **Build from scratch** | Terlalu ambisius. WM + compositor butuh tahunan. |
| **GNOME Shell** | Tidak kompatibel dengan distribusi target utama (Linux Mint). |
| **Budgie** | Basis Budgie (GNOME stack) kurang cocok untuk Mint/Ubuntu. |
| **KDE Plasma** | Qt stack, tidak kompatibel dengan GTK ecosystem target. |
| **XFCE** | GTK2/GTK3, tidak mendukung Wayland dengan baik. |
