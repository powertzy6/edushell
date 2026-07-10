# Developer Migration Guide — v1.0 → v2.0

## Overview
EduShell v2.0 introduces a new core (`edushell-core2`) with zero Cinnamon dependency.
Existing v1.0 plugins, themes, and extensions continue to work via the Compatibility Layer.

## For Plugin Developers

### If you use the v1 SDK (`edushell-sdk`):
- Your code continues to compile.
- New v2 APIs are additive: `edushell-core2` coexists alongside `edushell-apps`.
- Migrate at your own pace.

### Migration steps:
1. Add `edushell-core2 = "2.0"` to Cargo.toml
2. Replace `use edushell_sdk::plugin::*` with `use edushell_core2::extension_framework::*`
3. Update `PluginManifest` to `ExtensionManifest`
4. Recompile — the trait signatures are intentionally similar

### Key API changes:
| v1.0 | v2.0 |
|------|------|
| `PluginManifest` | `ExtensionManifest` |
| `Plugin` trait | `ExtensionFramework` |
| `PluginPermission` | `ExtensionPermission` |
| `PluginState` | `ExtensionState` |
| `SearchProvider` | `SearchEngine` + `SearchProvider` |
| `WidgetCategory` | `ui_kit::component` |

## For Theme Developers
- v1.0 CSS themes continue to work via Compatibility Layer.
- New v2.0 theme format uses structured JSON tokens (`ThemeDefinition`).
- Use `edushell-cli new-theme` to generate a v2 theme scaffold.

## For Config Migration
- Run `edushell migrate` (available in v2.0 CLI).
- ConfigEngine auto-detects v1.0 `settings.json` and converts to v2.0 format.
- Rollback available.

## Backward Compatibility Guarantee
- All v1.0 plugins compiled with `edushell-sdk` v1.0 will work on v2.0.
- The `CompatibilityLayer` module intercepts Cinnamon-specific calls.
- No breaking changes without a major version bump and migration tool.
