# Configuration, Theme & Localization Systems — EduShell

---

## 1. Configuration System Design

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Configuration System                      │
│                                                              │
│  ┌─────────────────┐    ┌──────────────────┐                │
│  │  ConfigManager   │    │  Schema (serde)   │               │
│  │                  │    │                   │               │
│  │  - load()        │    │  EduConfig        │               │
│  │  - save()        │───►│    ├─ shell       │               │
│  │  - watch()       │    │    ├─ launcher    │               │
│  │  - migrate()     │    │    ├─ theme       │               │
│  │  - export()      │    │    ├─ workspace   │               │
│  │  - import()      │    │    ├─ accessibility│              │
│  │  - reset()       │    │    ├─ language    │               │
│  │  - backup()      │    │    └─ shortcuts   │               │
│  └────────┬─────────┘    └──────────────────┘                │
│           │                                                  │
│           │ TOML file                                        │
│           ▼                                                  │
│  ┌──────────────────────────────────────────────────┐        │
│  │  ~/.config/edushell/config.toml                   │        │
│  └──────────────────────────────────────────────────┘        │
│           │                                                  │
│           │ notify (file watcher)                            │
│           ▼                                                  │
│  ┌─────────────────┐                                         │
│  │  Broadcast      │  ConfigEvent::Changed(key)              │
│  │  Channel        │─────────────────────────────────►       │
│  │                 │  Subscribers: Panel, Daemon, etc.       │
│  └─────────────────┘                                         │
└─────────────────────────────────────────────────────────────┘
```

### File Layout

```
~/.config/edushell/
├── config.toml           # Main configuration
├── keybindings.toml      # Keyboard shortcuts
├── state.toml            # Session state (window positions, etc.)
├── backups/
│   ├── config.20260701_120000.toml  # Auto backup before migration
│   └── config.20260702_080000.toml
└── overrides/
    └── local.toml         # Local overrides (not tracked)
```

### Schema Design

```rust
// core/edushell-core/src/config/schema.rs

use serde::{Deserialize, Serialize};

/// Main configuration schema
/// version field enables automatic migration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EduConfig {
    /// Schema version for migration
    pub version: SemVer,

    #[serde(default)]
    pub shell: ShellConfig,

    #[serde(default)]
    pub launcher: LauncherConfig,

    #[serde(default)]
    pub workspace: WorkspaceConfig,

    #[serde(default)]
    pub theme: ThemeConfig,

    #[serde(default)]
    pub accessibility: AccessibilityConfig,

    #[serde(default)]
    pub language: LanguageConfig,

    #[serde(default)]
    pub shortcuts: ShortcutsConfig,

    #[serde(default)]
    pub notifications: NotificationsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ShellConfig {
    #[serde(default = "default_panel_position")]
    pub panel_position: PanelPosition,

    #[serde(default = "default_true")]
    pub panel_autohide: bool,

    #[serde(default = "default_autohide_delay")]
    pub panel_autohide_delay_ms: u32,

    #[serde(default = "default_opacity")]
    pub panel_opacity: f64,

    #[serde(default = "default_workspace_count")]
    pub workspace_count: u32,

    #[serde(default = "default_true")]
    pub show_favorites: bool,

    #[serde(default)]
    pub pinned_apps: Vec<String>,  // desktop file IDs
}

// Default values ensure zero-config startup
fn default_panel_position() -> PanelPosition { PanelPosition::Bottom }
fn default_autohide_delay() -> u32 { 500 }
fn default_opacity() -> f64 { 0.95 }
fn default_workspace_count() -> u32 { 4 }
fn default_true() -> bool { true }
```

### Migration System

```rust
// core/edushell-core/src/config/migration.rs

use semver::Version;

/// Configuration migration registry
pub struct MigrationRegistry {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationRegistry {
    pub fn new() -> Self {
        let mut reg = Self { migrations: Vec::new() };
        reg.register(v1_0_to_v1_1);
        reg.register(v1_1_to_v1_2);
        reg
    }

    pub fn migrate(&self, config: &mut EduConfig) -> Result<(), ConfigError> {
        let current = config.version.clone();
        for migration in &self.migrations {
            if migration.applies_to(&current) {
                log::info!("Running migration: {}", migration.name());
                migration.apply(config)?;
            }
        }
        Ok(())
    }
}

/// A single migration step
trait Migration: Send + Sync {
    fn name(&self) -> &'static str;
    fn from_version(&self) -> Version;
    fn to_version(&self) -> Version;
    fn applies_to(&self, current: &Version) -> bool {
        current >= &self.from_version() && current < &self.to_version()
    }
    fn apply(&self, config: &mut EduConfig) -> Result<(), ConfigError>;
}
```

### Export / Import / Reset

```rust
impl ConfigManager {
    /// Export config as TOML string (for backup)
    pub fn export(&self) -> Result<String, ConfigError> {
        let config = self.current.read().unwrap();
        toml::to_string_pretty(&*config)
            .map_err(|e| ConfigError::Serialize { detail: e.to_string() })
    }

    /// Import config from TOML string
    pub fn import(&mut self, data: &str) -> Result<(), ConfigError> {
        let mut config: EduConfig = toml::from_str(data)
            .map_err(|e| ConfigError::Parse {
                path: self.path.clone(),
                detail: e.to_string(),
            })?;

        // Run migration chain on imported config
        self.migration_registry.migrate(&mut config)?;

        // Backup current before overwrite
        self.backup()?;

        // Write new config
        let mut file = std::fs::File::create(&self.path)
            .map_err(|e| ConfigError::Write {
                path: self.path.clone(),
                source: e,
            })?;
        file.write_all(toml::to_string_pretty(&config).unwrap().as_bytes())?;

        // Update in-memory state
        *self.current.write().unwrap() = config;

        // Notify subscribers
        let _ = self.change_tx.send(ConfigEvent::Imported);

        Ok(())
    }

    /// Reset to factory defaults
    pub fn reset(&mut self) -> Result<(), ConfigError> {
        // Load embedded defaults
        let defaults = EduConfig::default();

        // Backup current
        self.backup()?;

        // Write defaults
        self.write_config(&defaults)?;

        // Update in-memory
        *self.current.write().unwrap() = defaults;

        // Notify
        let _ = self.change_tx.send(ConfigEvent::Reset);

        Ok(())
    }

    /// Create timestamped backup
    fn backup(&self) -> Result<(), ConfigError> {
        let backup_dir = self.path.parent().unwrap().join("backups");
        std::fs::create_dir_all(&backup_dir)
            .map_err(|e| ConfigError::Io { source: e })?;

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("config.{}.toml", timestamp));

        std::fs::copy(&self.path, &backup_path)
            .map_err(|e| ConfigError::Io { source: e })?;

        Ok(())
    }
}
```

### Change Notification Flow

```
User changes setting in EduSettings
    │
    ▼
EduSettings writes to ~/.config/edushell/config.toml
    │
    ▼
notify crate detects file change
    │
    ▼
ConfigManager::on_file_changed()
    │
    ├── Re-read file
    ├── Validate schema
    ├── Update in-memory cache
    └── Broadcast ConfigEvent::Changed(key)
           │
           ▼
    Panel subscriber → Update panel position
    Theme subscriber → Reload CSS
    Daemon subscriber → Update behavior
```

---

## 2. Theme Engine Design

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Theme Manager                          │
│                                                              │
│  ┌──────────────────┐      ┌──────────────────┐             │
│  │  Theme Loader     │      │  CSS Processor    │            │
│  │                   │      │                   │            │
│  │  - scan dirs      │      │  - parse CSS vars │            │
│  │  - read manifest  │      │  - substitute vars│            │
│  │  - validate       │      │  - minify         │            │
│  └────────┬─────────┘      └────────┬──────────┘             │
│           │                         │                        │
│           ▼                         ▼                        │
│  ┌──────────────────┐      ┌──────────────────┐             │
│  │  Theme Repository │      │  GTK CSS Provider │            │
│  │                   │      │                   │            │
│  │  - active theme   │      │  - load to GTK    │            │
│  │  - available list │      │  - switch at runtime          │
│  │  - user themes    │      │  - priority stack │            │
│  └──────────────────┘      └──────────────────┘             │
└─────────────────────────────────────────────────────────────┘
```

### Theme Structure

```
/usr/share/edushell/themes/
└── edushell-default/
    ├── theme.toml              # Theme manifest
    ├── gtk/
    │   ├── gtk.css             # Base GTK styles
    │   ├── gtk-dark.css        # Dark variant
    │   └── assets/             # Images, sprites
    ├── shell/
    │   ├── panel.css           # Panel styling
    │   ├── launcher.css        # Launcher styling
    │   ├── notifications.css   # Notification styling
    │   └── osd.css             # OSD styling
    ├── colors/
    │   ├── light.toml          # Light color palette
    │   └── dark.toml           # Dark color palette
    └── cursors/
        └── (symlink to system)

~/.local/share/edushell/themes/
└── (user-installed themes, same structure)
```

### Theme Manifest

```toml
# theme.toml
[manifest]
name = "EduShell Default"
id = "edushell-default"
version = "1.0.0"
author = "EduShell Team"
license = "GPL-3.0"
description = "Default EduShell theme — clean, modern, educational"

[compatibility]
edushell-version = ">=1.0.0"
gtk-version = ">=4.12"

[features]
supports-dark = true
supports-accent = true
supports-blur = true
supports-transparency = true

[fonts]
default = "Inter 10"
monospace = "JetBrains Mono 10"
title = "Inter 12"

[scaling]
base-size = 14
scale-factor = 1.0
```

### Color System

```toml
# colors/light.toml
[palette]
primary = "#1A237E"
on-primary = "#FFFFFF"
primary-container = "#E8EAF6"
on-primary-container = "#1A237E"

secondary = "#00897B"
on-secondary = "#FFFFFF"
secondary-container = "#E0F2F1"
on-secondary-container = "#004D40"

background = "#FAFAFA"
on-background = "#212121"
surface = "#FFFFFF"
on-surface = "#212121"
surface-variant = "#F5F5F5"

error = "#C62828"
on-error = "#FFFFFF"
error-container = "#FFEBEE"

outline = "#BDBDBD"
outline-variant = "#E0E0E0"

shadow = "#00000020"
scrim = "#00000080"
```

### Runtime Theme Switching

```rust
// core/theme-engine/src/manager.rs

pub struct ThemeManager {
    current: RwLock<ThemeState>,
    css_provider: CssProvider,
    gtk_settings: glib::Settings,
    event_tx: broadcast::Sender<ThemeEvent>,
}

impl ThemeManager {
    /// Switch between light/dark mode
    pub fn set_mode(&self, mode: ThemeMode) -> Result<(), ThemeError> {
        let mut state = self.current.write().unwrap();

        // 1. Load color palette for this mode
        let colors = ColorPalette::load(&state.theme_path, mode)?;

        // 2. Process CSS templates with color substitution
        let css = self.process_css(&state.theme_path, &colors, mode)?;

        // 3. Apply to GTK
        self.css_provider.load_from_string(&css);
        gtk4::StyleContext::add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &self.css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // 4. Update icon theme
        gtk4::IconTheme::for_display(&gdk::Display::default().unwrap())
            .set_theme_name(Some(state.icon_theme.as_str()));

        // 5. Update state
        state.current_mode = mode;

        // 6. Notify
        let _ = self.event_tx.send(ThemeEvent::ModeChanged(mode));

        log::info!("Theme switched to {:?}", mode);
        Ok(())
    }

    /// Set accent color dynamically
    pub fn set_accent(&self, color: &str) -> Result<(), ThemeError> {
        let parsed = css_color::Rgba::from_html(color)
            .map_err(|_| ThemeError::InvalidColor(color.to_string()))?;

        let mut state = self.current.write().unwrap();
        state.accent_color = parsed;

        // Regenerate CSS with new accent
        let colors = ColorPalette::from_base(parsed);
        let css = self.process_css(&state.theme_path, &colors, state.current_mode)?;
        self.css_provider.load_from_string(&css);

        let _ = self.event_tx.send(ThemeEvent::AccentChanged(parsed));
        Ok(())
    }

    /// Process CSS template: substitute variables with actual values
    fn process_css(
        &self,
        theme_path: &Path,
        colors: &ColorPalette,
        mode: ThemeMode,
    ) -> Result<String, ThemeError> {
        let base_css = std::fs::read_to_string(
            theme_path.join("shell/panel.css")
        )?;
        let dark_css = std::fs::read_to_string(
            theme_path.join("shell/panel-dark.css")
        ).unwrap_or_default();

        // Substitute @color@ variables
        let css = if mode == ThemeMode::Dark {
            format!("{}\n{}", base_css, dark_css)
        } else {
            base_css
        };

        // Replace color variables
        let css = css
            .replace("@primary@", &colors.primary.to_css())
            .replace("@surface@", &colors.surface.to_css())
            .replace("@accent@", &colors.accent.to_css());

        Ok(css)
    }
}
```

### Auto Theme (Time-based)

```rust
impl ThemeManager {
    /// Auto mode: switch based on time of day or system preference
    pub fn update_auto_mode(&self) {
        let config = self.config_reader();
        if config.theme.mode != ThemeMode::Auto {
            return;
        }

        // Check system preference first
        if let Some(scheme) = self.detect_color_scheme() {
            self.set_mode(scheme);
            return;
        }

        // Fallback: time-based (sunrise/sunset approximation)
        let hour = chrono::Local::now().hour();
        let mode = if hour >= 6 && hour < 18 {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
        self.set_mode(mode);
    }

    /// Detect system color scheme from settings portal
    fn detect_color_scheme(&self) -> Option<ThemeMode> {
        let settings = glib::Settings::new("org.gnome.desktop.interface");
        let scheme = settings.string("color-scheme");
        match scheme.as_str() {
            "prefer-dark" => Some(ThemeMode::Dark),
            "prefer-light" => Some(ThemeMode::Light),
            _ => None,
        }
    }
}
```

---

## 3. Localization Architecture

### System Design

```
┌─────────────────────────────────────────────────────────────┐
│                   Localization System                        │
│                                                              │
│  ┌──────────────────┐    ┌──────────────────┐               │
│  │  LocaleManager    │    │  Translation      │              │
│  │                   │    │  Repository       │              │
│  │  - detect locale  │    │                   │              │
│  │  - set language   │    │  - gettext .mo    │              │
│  │  - gettext()      │───►│  - fallback chain │              │
│  │  - ngettext()     │    │  - hot-reload     │              │
│  │  - watch changes  │    └──────────────────┘              │
│  └──────────────────┘                                       │
│                                                              │
│  ┌──────────────────┐    ┌──────────────────┐               │
│  │  Language         │    │  Plural Rules     │              │
│  │  Detection        │    │                   │              │
│  │                   │    │  - id: none       │              │
│  │  - $LANG env     │    │  - en: n!=1       │              │
│  │  - locale output │    │  - others...       │              │
│  │  - user setting  │    └──────────────────┘               │
│  └──────────────────┘                                       │
└─────────────────────────────────────────────────────────────┘
```

### Translation Flow

```
Source Code (*.rs)
  │  tr!("Settings")
  │  trn!("{} file", "{} files", count)
  │  tr!("Pengaturan")   // Context: already translated
  │
  ▼
xgettext (xtask task)
  │
  ▼
edushell.pot (template)
  │
  ├──→ id.po  (Indonesian translator)
  └──→ en.po  (English translator)
          │
          ▼
    msgfmt (compile)
          │
          ▼
/usr/share/locale/id/LC_MESSAGES/edushell.mo
/usr/share/locale/en/LC_MESSAGES/edushell.mo
```

### API Design

```rust
// core/localization/src/lib.rs

use std::sync::OnceLock;

static LOCALE_MANAGER: OnceLock<LocaleManager> = OnceLock::new();

/// Initialize localization system
pub fn init(lang: Option<&str>) {
    let manager = LocaleManager::new(lang.unwrap_or("id"));
    LOCALE_MANAGER.set(manager).ok();
}

/// Translate a string (macro for ergonomics)
#[macro_export]
macro_rules! tr {
    ($msgid:expr) => {
        $crate::gettext($msgid)
    };
    ($msgid:expr, $($arg:expr),+) => {
        format!($crate::gettext($msgid), $($arg),+)
    };
}

/// Translate with plural forms
#[macro_export]
macro_rules! trn {
    ($singular:expr, $plural:expr, $count:expr) => {
        $crate::ngettext($singular, $plural, $count)
    };
}

/// Core gettext function
pub fn gettext(msgid: &str) -> String {
    LOCALE_MANAGER
        .get()
        .map(|m| m.translate(msgid))
        .unwrap_or_else(|| msgid.to_string())
}

/// Plural form translation
pub fn ngettext(singular: &str, plural: &str, count: u32) -> String {
    LOCALE_MANAGER
        .get()
        .map(|m| m.translate_plural(singular, plural, count))
        .unwrap_or_else(|| {
            if count == 1 { singular.to_string() } else { plural.to_string() }
        })
}

pub struct LocaleManager {
    current_lang: String,
    translations: HashMap<String, String>,  // msgid → translation
    plural_forms: PluralForms,
}

impl LocaleManager {
    pub fn new(lang: &str) -> Self {
        let translations = Self::load_translations(lang);
        let plural_forms = PluralForms::for_language(lang);
        Self {
            current_lang: lang.to_string(),
            translations,
            plural_forms,
        }
    }

    fn load_translations(lang: &str) -> HashMap<String, String> {
        // 1. Try ~/.local/share/edushell/locale/{lang}/edushell.mo
        // 2. Fallback to /usr/share/locale/{lang}/LC_MESSAGES/edushell.mo
        // 3. Fallback to empty (English as base)
        let paths = vec![
            dirs::data_dir().unwrap().join("edushell/locale").join(lang).join("edushell.mo"),
            PathBuf::from("/usr/share/locale").join(lang).join("LC_MESSAGES/edushell.mo"),
        ];

        for path in &paths {
            if path.exists() {
                match read_mo_file(path) {
                    Ok(translations) => return translations,
                    Err(e) => log::warn!("Failed to load .mo file {:?}: {}", path, e),
                }
            }
        }

        log::info!("No .mo file found for '{}', using English defaults", lang);
        HashMap::new()
    }

    pub fn translate(&self, msgid: &str) -> String {
        self.translations
            .get(msgid)
            .cloned()
            .unwrap_or_else(|| msgid.to_string())
    }

    pub fn translate_plural(&self, singular: &str, plural: &str, count: u32) -> String {
        let form = self.plural_forms.evaluate(count);
        let key = if form == 0 { singular } else { plural };
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}
```

### Language Detection

```rust
pub fn detect_language() -> String {
    // Priority:
    // 1. EduShell config setting
    // 2. $LANG environment variable
    // 3. locale command output
    // 4. Default: Indonesian

    // Check config
    if let Ok(config) = edushell_core::config::load() {
        if config.language.override_lang.is_some() {
            return config.language.override_lang.unwrap();
        }
    }

    // Check $LANG
    if let Ok(lang) = std::env::var("LANG") {
        let lang = lang.split('.').next().unwrap_or("id_ID");
        let lang = lang.split('_').next().unwrap_or("id");
        return lang.to_string();
    }

    // Default
    "id".to_string()
}
```

### Translation Workflow

1. **Developer**: Write code with `tr!()` macros
2. **Developer**: Run `cargo xtask gen-pot` to regenerate `.pot`
3. **Translator**: Use Poedit or Weblate to translate `.po` files
4. **CI**: Build `.mo` files and package
5. **User**: Language auto-detected or set in EduSettings

### Supported Languages (v1)

| Code | Language | Priority | Notes |
|------|----------|----------|-------|
| `id` | Bahasa Indonesia | Primary | Default, full coverage |
| `en` | English | Fallback | Full coverage |

### Future Languages (v2+)

| Code | Language | Region |
|------|----------|--------|
| `jv` | Bahasa Jawa | Indonesia |
| `su` | Bahasa Sunda | Indonesia |
| `ms` | Bahasa Melayu | Malaysia |
| `zh` | Chinese | International |
| `ar` | Arabic | International |
