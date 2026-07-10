# Compatibility Guide — EduShell v1.0 ↔ v2.0

## Cinnamon Compat
The `CompatibilityLayer` provides shims for these Cinnamon components:

| Cinnamon Name | EduShell v2.0 Equivalent | Status |
|---------------|--------------------------|--------|
| `cinnamon-settings` | `edushell-settings` | Shimmed |
| `cinnamon-menu` | `edushell-launcher` | Shimmed |
| `cinnamon-workspace` | `edushell-workspace` | Shimmed |
| `muffin` | `window_api::StubWindowManager` | Partial (v3.0 full) |
| `nemo` | `file_manager::FileManagerIntegration` | Direct |

## Plugin Compatibility
- v1.0 plugins using `edushell_sdk::plugin` continue to work.
- v2.0 plugins should use `edushell_core2::extension_framework`.
- Both can coexist in the same installation.

## Theme Compatibility
- v1.0 CSS themes load via Compatibility Layer.
- v2.0 JSON theme format loads into `ThemeEngine`.
- System auto-detects format from file extension.

## Config Compatibility
- `ConfigEngine::load_json()` reads v1.0 format.
- `MigrationEngine` provides one-shot upgrade.
- Rollback via `MigrationEngine::rollback()`.

## Known Incompatibilities
1. **Muffin window effects**: Not replicated in v2.0 stub. Full support in v3.0.
2. **Cinnamon JS applets**: Must be rewritten as Rust extensions. No JS runtime in v2.0.
3. **Cinnamon desklet API**: Not ported. Use Widget SDK instead.
