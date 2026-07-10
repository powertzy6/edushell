// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Root configuration for all EduShell components.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EduConfig {
    /// Schema version for migration tracking.
    pub version: String,

    #[serde(default)]
    pub core: CoreConfig,

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

    #[serde(default)]
    pub search: SearchConfig,

    #[serde(default)]
    pub session: SessionConfig,

    #[serde(default)]
    pub power: PowerConfig,

    #[serde(default)]
    pub performance: PerformanceConfig,
}

// ── Core Configuration ──────────────────────────────────────────

/// Core system configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CoreConfig {
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_true")]
    pub enable_crash_reports: bool,
    pub log_file: Option<PathBuf>,
}

fn default_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("edushell")
}

fn default_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("edushell")
}

fn default_log_level() -> String {
    if cfg!(debug_assertions) {
        "debug".into()
    } else {
        "info".into()
    }
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            data_dir: default_data_dir(),
            cache_dir: default_cache_dir(),
            log_level: default_log_level(),
            enable_crash_reports: true,
            log_file: None,
        }
    }
}

// ── Shell Configuration ─────────────────────────────────────────

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
    pub pinned_apps: Vec<String>,
}

fn default_panel_position() -> PanelPosition {
    PanelPosition::Bottom
}
fn default_autohide_delay() -> u32 {
    500
}
fn default_opacity() -> f64 {
    0.95
}
fn default_workspace_count() -> u32 {
    4
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            panel_position: default_panel_position(),
            panel_autohide: true,
            panel_autohide_delay_ms: default_autohide_delay(),
            panel_opacity: default_opacity(),
            workspace_count: default_workspace_count(),
            show_favorites: true,
            pinned_apps: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PanelPosition {
    Bottom,
    Top,
    Left,
    Right,
}

// ── Launcher Configuration ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LauncherConfig {
    #[serde(default = "default_launcher_icon_size")]
    pub icon_size: u32,
    #[serde(default = "default_launcher_columns")]
    pub grid_columns: u32,
    #[serde(default = "default_true")]
    pub show_recent: bool,
    #[serde(default = "default_true")]
    pub show_categories: bool,
}

fn default_launcher_icon_size() -> u32 {
    48
}
fn default_launcher_columns() -> u32 {
    6
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            icon_size: default_launcher_icon_size(),
            grid_columns: default_launcher_columns(),
            show_recent: true,
            show_categories: true,
        }
    }
}

// ── Workspace Configuration ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceConfig {
    #[serde(default = "default_workspace_count")]
    pub count: u32,
    #[serde(default = "default_true")]
    pub show_thumbnails: bool,
    #[serde(default = "default_true")]
    pub wrap_around: bool,
    #[serde(default = "default_false")]
    pub static_workspaces: bool,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            count: default_workspace_count(),
            show_thumbnails: true,
            wrap_around: true,
            static_workspaces: false,
        }
    }
}

// ── Theme Configuration ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ThemeConfig {
    #[serde(default = "default_theme_name")]
    pub name: String,
    #[serde(default = "default_mode")]
    pub mode: ThemeMode,
    #[serde(default = "default_accent")]
    pub accent_color: String,
    #[serde(default = "default_icon_theme")]
    pub icon_theme: String,
    #[serde(default = "default_cursor_theme")]
    pub cursor_theme: String,
    #[serde(default = "default_font")]
    pub font: String,
    #[serde(default = "default_mono_font")]
    pub monospace_font: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_scale")]
    pub scale_factor: f64,
    #[serde(default = "default_true")]
    pub enable_animations: bool,
    #[serde(default = "default_true")]
    pub enable_blur: bool,
    #[serde(default = "default_true")]
    pub enable_transparency: bool,
    pub wallpaper: Option<String>,
}

fn default_theme_name() -> String {
    "edushell-default".into()
}
fn default_mode() -> ThemeMode {
    ThemeMode::Auto
}
fn default_accent() -> String {
    "#1A237E".into()
}
fn default_icon_theme() -> String {
    "Papirus".into()
}
fn default_cursor_theme() -> String {
    "default".into()
}
fn default_font() -> String {
    "Inter 10".into()
}
fn default_mono_font() -> String {
    "JetBrains Mono 10".into()
}
fn default_font_size() -> u32 {
    10
}
fn default_scale() -> f64 {
    1.0
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: default_theme_name(),
            mode: default_mode(),
            accent_color: default_accent(),
            icon_theme: default_icon_theme(),
            cursor_theme: default_cursor_theme(),
            font: default_font(),
            monospace_font: default_mono_font(),
            font_size: default_font_size(),
            scale_factor: default_scale(),
            enable_animations: true,
            enable_blur: true,
            enable_transparency: true,
            wallpaper: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

// ── Accessibility Configuration ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AccessibilityConfig {
    #[serde(default)]
    pub high_contrast: bool,
    #[serde(default)]
    pub large_font: bool,
    #[serde(default = "default_font_scale")]
    pub font_scale: f64,
    #[serde(default)]
    pub screen_reader: bool,
    #[serde(default)]
    pub reduce_motion: bool,
    #[serde(default)]
    pub sticky_keys: bool,
    #[serde(default)]
    pub slow_keys: bool,
    #[serde(default)]
    pub bounce_keys: bool,
    #[serde(default)]
    pub on_screen_keyboard: bool,
}

fn default_font_scale() -> f64 {
    1.0
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            high_contrast: false,
            large_font: false,
            font_scale: default_font_scale(),
            screen_reader: false,
            reduce_motion: false,
            sticky_keys: false,
            slow_keys: false,
            bounce_keys: false,
            on_screen_keyboard: false,
        }
    }
}

// ── Language Configuration ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LanguageConfig {
    #[serde(default = "default_language")]
    pub language: String,
    pub override_lang: Option<String>,
}

fn default_language() -> String {
    "id".into()
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            override_lang: None,
        }
    }
}

// ── Shortcuts Configuration ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ShortcutsConfig {
    #[serde(default = "default_launcher_shortcut")]
    pub launcher: String,
    #[serde(default = "default_workspace_left")]
    pub workspace_left: String,
    #[serde(default = "default_workspace_right")]
    pub workspace_right: String,
    #[serde(default = "default_quick_settings")]
    pub quick_settings: String,
    #[serde(default = "default_lock_screen")]
    pub lock_screen: String,
    #[serde(default = "default_terminal")]
    pub terminal: String,
}

fn default_launcher_shortcut() -> String {
    "<Super>space".into()
}
fn default_workspace_left() -> String {
    "<Super>Left".into()
}
fn default_workspace_right() -> String {
    "<Super>Right".into()
}
fn default_quick_settings() -> String {
    "<Super>s".into()
}
fn default_lock_screen() -> String {
    "<Super>l".into()
}
fn default_terminal() -> String {
    "<Ctrl><Alt>t".into()
}

impl Default for ShortcutsConfig {
    fn default() -> Self {
        Self {
            launcher: default_launcher_shortcut(),
            workspace_left: default_workspace_left(),
            workspace_right: default_workspace_right(),
            quick_settings: default_quick_settings(),
            lock_screen: default_lock_screen(),
            terminal: default_terminal(),
        }
    }
}

// ── Notifications Configuration ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NotificationsConfig {
    #[serde(default = "default_true")]
    pub show_popups: bool,
    #[serde(default)]
    pub do_not_disturb: bool,
    #[serde(default = "default_notif_history")]
    pub max_history: u32,
    #[serde(default)]
    pub per_app_settings: Vec<String>,
}

fn default_notif_history() -> u32 {
    100
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            show_popups: true,
            do_not_disturb: false,
            max_history: default_notif_history(),
            per_app_settings: Vec::new(),
        }
    }
}

// ── Search Configuration ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SearchConfig {
    #[serde(default = "default_true")]
    pub index_apps: bool,
    #[serde(default)]
    pub index_files: bool,
    #[serde(default = "default_true")]
    pub index_settings: bool,
    #[serde(default)]
    pub index_learning: bool,
    #[serde(default = "default_search_debounce")]
    pub debounce_ms: u64,
}

fn default_search_debounce() -> u64 {
    150
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            index_apps: true,
            index_files: false,
            index_settings: true,
            index_learning: false,
            debounce_ms: default_search_debounce(),
        }
    }
}

// ── Session Configuration ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SessionConfig {
    #[serde(default = "default_true")]
    pub restore_apps: bool,
    #[serde(default = "default_true")]
    pub lock_on_suspend: bool,
    #[serde(default = "default_true")]
    pub auto_start_daemon: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            restore_apps: true,
            lock_on_suspend: true,
            auto_start_daemon: true,
        }
    }
}

// ── Power Configuration ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PowerConfig {
    #[serde(default = "default_battery_low")]
    pub battery_low_percent: f64,
    #[serde(default = "default_battery_critical")]
    pub battery_critical_percent: f64,
    #[serde(default = "default_suspend_timeout_ac")]
    pub suspend_timeout_ac_minutes: u32,
    #[serde(default = "default_suspend_timeout_battery")]
    pub suspend_timeout_battery_minutes: u32,
}

fn default_battery_low() -> f64 {
    20.0
}
fn default_battery_critical() -> f64 {
    5.0
}
fn default_suspend_timeout_ac() -> u32 {
    30
}
fn default_suspend_timeout_battery() -> u32 {
    15
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            battery_low_percent: default_battery_low(),
            battery_critical_percent: default_battery_critical(),
            suspend_timeout_ac_minutes: default_suspend_timeout_ac(),
            suspend_timeout_battery_minutes: default_suspend_timeout_battery(),
        }
    }
}

// ── Performance Configuration ───────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PerformanceConfig {
    #[serde(default = "default_true")]
    pub enable_animations: bool,
    #[serde(default)]
    pub enable_blur: bool,
    #[serde(default)]
    pub enable_transparency: bool,
    #[serde(default = "default_true")]
    pub lazy_loading: bool,
    #[serde(default = "default_true")]
    pub background_cache: bool,
    #[serde(default = "default_false")]
    pub reduce_widgets: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_animations: true,
            enable_blur: false,
            enable_transparency: false,
            lazy_loading: true,
            background_cache: true,
            reduce_widgets: false,
        }
    }
}

// ── Default implementation ──────────────────────────────────────

impl Default for EduConfig {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            core: CoreConfig::default(),
            shell: ShellConfig::default(),
            launcher: LauncherConfig::default(),
            workspace: WorkspaceConfig::default(),
            theme: ThemeConfig::default(),
            accessibility: AccessibilityConfig::default(),
            language: LanguageConfig::default(),
            shortcuts: ShortcutsConfig::default(),
            notifications: NotificationsConfig::default(),
            search: SearchConfig::default(),
            session: SessionConfig::default(),
            power: PowerConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_roundtrip() {
        let config = EduConfig::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialized: EduConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.shell.panel_position, deserialized.shell.panel_position);
        assert_eq!(config.theme.mode, deserialized.theme.mode);
        assert_eq!(config.workspace.count, deserialized.workspace.count);
    }

    #[test]
    fn test_config_custom_values() {
        let mut config = EduConfig::default();
        config.shell.panel_position = PanelPosition::Top;
        config.theme.mode = ThemeMode::Dark;
        config.accessibility.high_contrast = true;

        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialized: EduConfig = toml::from_str(&toml_str).unwrap();

        assert_eq!(deserialized.shell.panel_position, PanelPosition::Top);
        assert_eq!(deserialized.theme.mode, ThemeMode::Dark);
        assert!(deserialized.accessibility.high_contrast);
    }

    #[test]
    fn test_serialization_field_names() {
        let config = EduConfig::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();

        // Verify kebab-case serialization
        assert!(toml_str.contains("panel-position"));
        assert!(toml_str.contains("accent-color"));
        assert!(toml_str.contains("icon-theme"));
        assert!(toml_str.contains("scale-factor"));
        assert!(toml_str.contains("high-contrast"));
    }

    #[test]
    fn test_theme_mode_serde() {
        let modes = [
            (ThemeMode::Light, "\"Light\""),
            (ThemeMode::Dark, "\"Dark\""),
            (ThemeMode::Auto, "\"Auto\""),
        ];

        for (mode, expected) in &modes {
            let json = serde_json::to_string(mode).unwrap();
            assert_eq!(json, *expected);
        }
    }
}
