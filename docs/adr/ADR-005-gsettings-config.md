# ADR-005: GSettings/GSchema untuk Configuration Management

**Status**: Proposed
**Version**: v1
**Date**: 2026-07

## Context
EduShell perlu menyimpan konfigurasi: panel position, theme, language, shortcuts, dll. Harus:
1. Berbasis file (bukan database)
2. Mendukung default values
3. Key/value dengan tipe data
4. Kompatibel dengan Cinnamon
5. Monitor perubahan real-time

## Decision
Gunakan **GSettings** dengan **GSchema XML** sebagai sistem konfigurasi.

### Namespace Convention
```
org.edushell.shell        # Panel, launcher, workspace config
org.edushell.launcher     # Launcher-specific config
org.edushell.settings     # EduSettings app config
org.edushell.theme        # Theme configuration
org.edushell.accessibility # Accessibility settings
```

### Storage Location
- Schema definitions: `/usr/share/glib-2.0/schemas/org.edushell.*.gschema.xml`
- User config: `~/.config/glib-2.0/settings/keyfile` (via dconf)
- Override: `~/.config/edushell/overrides.ini`

### Monitoring
- Gunakan `g_settings_bind()` untuk binding langsung ke widget
- Gunakan `g_settings_signal_changed()` untuk custom reactions

### Consequences
**Positive**:
- Standard di ecosystem GNOME/Cinnamon
- Type safety (boolean, int, double, string, tuple, variant)
- Change notification via signals
- Default values built-in
- CLI access via `gsettings` command

**Negative**:
- Tidak portable ke non-GNOME system
- Depends on dconf for runtime storage
- Schema changes require recompilation

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| **JSON files** | Tidak ada schema validation built-in, change notification manual. |
| **INI files** | Tipe data terbatas (string only), tidak ada hierarki. |
| **YAML** | Dependency eksternal, tidak native di ecosystem Linux. |
| **SQLite** | Overkill untuk konfigurasi shell sederhana. |
