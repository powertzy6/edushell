# ADR-007: gettext untuk Translation System

**Status**: Proposed
**Version**: v1
**Date**: 2026-07

## Context
EduShell wajib mendukung Bahasa Indonesia dan English. Semua string harus melalui sistem translasi. Tidak boleh ada hardcode string.

## Decision
Gunakan **gettext** (`GLib.gettext()`) sebagai sistem translasi.

### Implementation
1. **Source code**: Semua string user-facing menggunakan `_("string")` macro
2. **POT file**: Generate dari source dengan `xgettext`
3. **PO files**: `id.po`, `en.po` di direktori `po/`
4. **MO files**: Compiled dan diinstall ke `/usr/share/locale/`
5. **Default**: Bahasa Indonesia (`id_ID`)
6. **Fallback**: English (`en_US`)

### Translation Workflow
```
Source code (*.vala)
    │ (xgettext)
    ▼
edushell.pot (template)
    │ (msginit)
    ▼
id.po → id.mo → /usr/share/locale/id/LC_MESSAGES/edushell.mo
en.po → en.mo → /usr/share/locale/en/LC_MESSAGES/edushell.mo
```

### Plural Forms
- Bahasa Indonesia: tidak ada plural forms
- English: `nplurals=2; plural=(n != 1)`

### Integration with Weblate/Crowdin (v1.x+)
1. PO files push ke Weblate
2. Community translators translate via web UI
3. Automated PR on translation update
4. CI checks translation completeness (> 80% required for release)

### Consequences
**Positive**:
- Standard di ecosystem GNOME/Linux
- Terintegrasi dengan Vala (GLib.gettext)
- Tooling mature (Poedit, Weblate, Crowdin)
- Tidak ada dependency tambahan

**Negative**:
- Tidak mendukung translation di runtime tanpa restart
- Harus compile MO files untuk setiap bahasa

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| **Qt Linguist** | Qt stack, tidak kompatibel dengan GTK. |
| **Fluent/gettext (Rust)** | Untuk v3+, saat Rust component sudah ada. |
| **Custom JSON translation** | Tidak standard, tooling terbatas. |
