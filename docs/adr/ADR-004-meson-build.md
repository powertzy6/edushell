# ADR-004: Meson sebagai Build System

**Status**: Proposed
**Version**: v1
**Date**: 2026-07

## Context
EduShell membutuhkan build system yang:
1. Mendukung Vala, C, Python, (dan Rust di masa depan)
2. Digunakan secara luas di GNOME/Cinnamon ecosystem
3. Cross-platform
4. Cepat dan reliable

## Decision
Gunakan **Meson** sebagai build system utama.

### Rationale
1. **Standar GNOME/Cinnamon**: Meson adalah build system resmi GNOME, GTK, dan Cinnamon.
2. **Multi-language support**: Meson mendukung Vala, C, C++, Rust via subprojects.
3. **Cross-compilation**: Meson memiliki cross-compilation support yang baik.
4. **Dependency management**: Meson dapat menangani dependency kompleks.
5. **Ninja backend**: Build sangat cepat.

### Consequences
**Positive**:
- Kompatibel dengan ecosystem Cinnamon/GNOME
- Build cepat dengan Ninja
- Dukungan Vala yang baik
- Mudah diintegrasikan dengan GNOME Builder

**Negative**:
- Meson memiliki learning curve
- Beberapa fitur niche mungkin belum ada

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| **CMake** | Tidak umum di GNOME ecosystem. Dukungan Vala kurang baik. |
| **Autotools** | Legacy. Terlalu kompleks dan lambat. |
| **Makefile manual** | Tidak scalable untuk project multi-module. |
