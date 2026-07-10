# Architecture Decision Records — EduShell

## Purpose
Architecture Decision Records (ADRs) adalah dokumentasi keputusan arsitektur penting yang memengaruhi arah proyek. Setiap ADR mencatat konteks, keputusan, konsekuensi, dan alternatif yang dipertimbangkan.

## How to Use
1. Sebelum membuat keputusan arsitektur besar, buka ADR baru
2. Diskusikan sebagai issue GitHub terlebih dahulu
3. Setelah konsensus, tulis ADR dan merged ke `main`
4. Jika keputusan berubah, buat ADR baru yang menimpa yang lama

## ADR Format
Setiap ADR mengikuti template [Michael Nygard](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions):

- **Title**: Nomor dan judul
- **Status**: Proposed | Accepted | Deprecated | Superseded
- **Context**: Mengapa keputusan ini perlu dibuat
- **Decision**: Keputusan yang diambil
- **Consequences**: Dampak positif dan negatif
- **Alternatives**: Alternatif yang dipertimbangkan dan alasan ditolak

---

## ADR Index

| Number | Title | Status | Version | Date |
|--------|-------|--------|---------|------|
| ADR-001 | [Vala & GTK4 sebagai bahasa dan toolkit utama](ADR-001-vala-gtk4.md) | **Proposed** | v1 | 2026-07 |
| ADR-002 | [Cinnamon Desktop sebagai base system](ADR-002-cinnamon-base.md) | **Proposed** | v1 | 2026-07 |
| ADR-003 | [Display server strategy: Wayland primer, X11 fallback](ADR-003-wayland-x11.md) | **Proposed** | v1 | 2026-07 |
| ADR-004 | [Meson sebagai build system](ADR-004-meson-build.md) | **Proposed** | v1 | 2026-07 |
| ADR-005 | [GSettings/GSchema untuk configuration management](ADR-005-gsettings-config.md) | **Proposed** | v1 | 2026-07 |
| ADR-006 | [Session lifecycle melalui cinnamon-session](ADR-006-session-lifecycle.md) | **Proposed** | v1 | 2026-07 |
| ADR-007 | [gettext untuk translation system](ADR-007-translation-system.md) | **Proposed** | v1 | 2026-07 |
| ADR-008 | [Plugin architecture untuk ekstensi (v3+)](ADR-008-plugin-architecture.md) | **Deferred** | v3+ | 2026-07 |
| ADR-009 | [Component replacement strategy](ADR-009-component-replacement-strategy.md) | **Proposed** | v1+ | 2026-07 |

---

## ADR Lifecycle

```
[Idea] → [Issue/Discussion] → [Draft ADR] → [Review] → [Accepted]
                                                              ↓
                                                   [Superseded] ← [New ADR]
```

## ADR Creation Checklist
- [ ] Apakah keputusan ini berdampak jangka panjang?
- [ ] Apakah keputusan ini sulit diubah nanti?
- [ ] Apakah ada alternatif signifikan yang perlu didokumentasikan?
- [ ] Apakah konsekuensinya sudah dipertimbangkan?

Jika jawaban "ya" untuk salah satu di atas, ADR perlu dibuat.
