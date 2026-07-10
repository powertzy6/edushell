// SPDX-License-Identifier: GPL-3.0-or-later

//! # Theme Integration
//!
//! Manages GTK CSS theming for the desktop shell. Loads style sheets,
//! applies theme variants (light/dark), and integrates with the
//! core `ThemeEngine` from `edushell-core`.

use std::collections::HashMap;
use std::path::PathBuf;

/// Manager for UI widget styling and CSS theming.
#[derive(Clone)]
pub struct ThemeManager {
    /// Whether dark mode is active.
    dark_mode: bool,
    /// Accent color in hex (#RRGGBB).
    accent_color: String,
    /// Custom CSS snippets by name.
    custom_css: HashMap<String, String>,
    /// Path to theme assets directory.
    assets_path: PathBuf,
    /// Whether to use system theme.
    use_system_theme: bool,
}

impl ThemeManager {
    /// Create a new theme manager.
    pub fn new() -> Self {
        Self {
            dark_mode: false,
            accent_color: "#4A90D9".to_string(),
            custom_css: HashMap::new(),
            assets_path: Self::default_assets_path(),
            use_system_theme: false,
        }
    }

    fn default_assets_path() -> PathBuf {
        std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .ok()
            .or_else(|| {
                std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join(".local").join("share"))
                    .ok()
            })
            .unwrap_or_else(|| PathBuf::from("/usr/share"))
            .join("edushell")
            .join("theme")
    }

    /// Toggle dark mode.
    pub fn set_dark_mode(&mut self, enabled: bool) {
        self.dark_mode = enabled;
    }

    /// Check if dark mode is active.
    pub fn dark_mode(&self) -> bool {
        self.dark_mode
    }

    /// Set accent color (hex format #RRGGBB).
    pub fn set_accent_color(&mut self, color: &str) {
        let color = color.to_string();
        if color.starts_with('#') && (color.len() == 7 || color.len() == 9) {
            self.accent_color = color;
        }
    }

    /// Get accent color.
    pub fn accent_color(&self) -> &str {
        &self.accent_color
    }

    /// Set assets path.
    pub fn set_assets_path(&mut self, path: PathBuf) {
        self.assets_path = path;
    }

    /// Get assets path.
    pub fn assets_path(&self) -> &PathBuf {
        &self.assets_path
    }

    /// Enable/disable system theme.
    pub fn set_use_system_theme(&mut self, enabled: bool) {
        self.use_system_theme = enabled;
    }

    /// Check if using system theme.
    pub fn use_system_theme(&self) -> bool {
        self.use_system_theme
    }

    /// Register a custom CSS snippet.
    pub fn add_custom_css(&mut self, name: &str, css: &str) {
        self.custom_css.insert(name.to_string(), css.to_string());
    }

    /// Remove a custom CSS snippet.
    pub fn remove_custom_css(&mut self, name: &str) {
        self.custom_css.remove(name);
    }

    /// Get all custom CSS snippets concatenated.
    pub fn custom_css(&self) -> String {
        self.custom_css.values().cloned().collect::<Vec<_>>().join("\n")
    }

    /// Generate the complete desktop shell stylesheet.
    pub fn generate_stylesheet(&self) -> String {
        let mode = if self.dark_mode { "dark" } else { "light" };
        let accent = &self.accent_color;
        let _custom = self.custom_css();

        format!(
            r#"
/* EduShell Desktop Theme - {mode} mode */
@import url('resource:///org/edushell/shell/theme/{mode}.css');

/* Accent color overrides */
:root {{
    --edushell-accent-color: {accent};
    --edushell-accent-rgb: {accent_rgb};
}}

/* Panel */
.panel {{
    background-color: rgba(30, 30, 30, 0.85);
    color: #ffffff;
    border: none;
}}

.panel-dark {{
    background-color: rgba(20, 20, 20, 0.90);
}}

/* Dock */
.dock {{
    background-color: rgba(30, 30, 30, 0.80);
    border-radius: 12px;
}}

.dock-item {{
    border-radius: 8px;
    transition: all 200ms ease;
}}

.dock-item:hover {{
    background-color: rgba(255, 255, 255, 0.15);
}}

.dock-item.active {{
    background-color: rgba(255, 255, 255, 0.25);
}}

/* Launcher */
.launcher-window {{
    background-color: rgba(10, 10, 10, 0.70);
}}

.launcher-search {{
    background-color: rgba(255, 255, 255, 0.10);
    color: #ffffff;
    border-radius: 24px;
    padding: 8px 16px;
}}

/* Notifications */
.notification {{
    background-color: rgba(30, 30, 30, 0.95);
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
}}

.notification-center {{
    background-color: rgba(20, 20, 20, 0.95);
    border-left: 1px solid rgba(255, 255, 255, 0.1);
}}

/* Quick Settings */
.quick-settings {{
    background-color: rgba(20, 20, 20, 0.95);
    border-radius: 12px;
}}

.qs-button {{
    border-radius: 8px;
    padding: 8px;
}}

.qs-button.active {{
    background-color: {accent}40;
}}

/* Overview */
.overview {{
    background-color: rgba(10, 10, 10, 0.60);
}}

.overview-workspace {{
    border-radius: 12px;
    background-color: rgba(30, 30, 30, 0.50);
}}

/* Power Menu */
.power-menu {{
    background-color: rgba(20, 20, 20, 0.95);
    border-radius: 12px;
}}

.power-button {{
    border-radius: 8px;
    min-width: 80px;
    min-height: 80px;
}}

.power-button:hover {{
    background-color: rgba(255, 255, 255, 0.1);
}}

/* Context Menu */
.context-menu {{
    background-color: rgba(30, 30, 30, 0.95);
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
}}

/* Workspace Indicator */
.workspace-indicator {{
    padding: 2px 4px;
}}

.workspace-dot {{
    min-width: 8px;
    min-height: 8px;
    border-radius: 4px;
    margin: 1px;
    background-color: rgba(255, 255, 255, 0.3);
}}

.workspace-dot.active {{
    background-color: #ffffff;
    min-width: 24px;
}}

/* App Grid */
.app-grid-item {{
    border-radius: 8px;
    padding: 4px;
}}

.app-grid-item:hover {{
    background-color: rgba(255, 255, 255, 0.1);
}}

.app-grid-item:focus {{
    outline: 2px solid {accent};
}}

/* Toast */
.toast {{
    background-color: rgba(30, 30, 30, 0.95);
    border-radius: 8px;
    margin: 4px;
}}

/* System Tray */
.system-tray-icon {{
    padding: 2px;
    border-radius: 4px;
}}

.system-tray-icon:hover {{
    background-color: rgba(255, 255, 255, 0.1);
}}

/* Accessibility: Focus Ring */
.focus-ring {{
    outline: {ring_width}px solid {accent};
    outline-offset: -{ring_width}px;
}}

/* Accessibility: High Contrast */
.high-contrast .panel,
.high-contrast .dock,
.high-contrast .notification,
.high-contrast .quick-settings,
.high-contrast .context-menu {{
    background-color: #000000;
    border: 1px solid #ffffff;
}}

/* Accessibility: Large Text */
.large-text {{
    font-size: {text_scale}em;
}}

/* reduced motion */
@media (prefers-reduced-motion) {{
    *, *::before, *::after {{
        animation-duration: 0.01ms !important;
        transition-duration: 0.01ms !important;
    }}
}}
"#,
            mode = mode,
            accent = accent,
            accent_rgb = hex_to_rgb(accent),
            ring_width = 2,
            text_scale = 1.0,
        )
    }

    /// Apply the current theme. With GTK feature, provides the CSS provider.
    /// Without GTK, this is a no-op.
    #[cfg(feature = "gtk")]
    pub fn apply(&self) {
        use gtk::gdk::Display;
        use gtk::prelude::*;

        let css = self.generate_stylesheet();
        let provider = gtk::CssProvider::new();
        provider.load_from_string(&css);

        if let Some(display) = Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        tracing::info!(target: "edushell::ui::theme", "Theme applied: {mode}",
            mode = if self.dark_mode { "dark" } else { "light" });
    }

    /// Apply theme (non-GTK stub).
    #[cfg(not(feature = "gtk"))]
    pub fn apply(&self) {
        tracing::info!(target: "edushell::ui::theme", "Theme recorded (GTK not available): {mode}",
            mode = if self.dark_mode { "dark" } else { "light" });
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert hex color (#RRGGBB) to RGB tuple string.
fn hex_to_rgb(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return format!("{r}, {g}, {b}");
        }
    }
    "74, 144, 217".to_string()
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_defaults() {
        let tm = ThemeManager::new();
        assert!(!tm.dark_mode());
        assert_eq!(tm.accent_color(), "#4A90D9");
        assert!(!tm.use_system_theme());
    }

    #[test]
    fn test_dark_mode_toggle() {
        let mut tm = ThemeManager::new();
        tm.set_dark_mode(true);
        assert!(tm.dark_mode());
        tm.set_dark_mode(false);
        assert!(!tm.dark_mode());
    }

    #[test]
    fn test_accent_color_validation() {
        let mut tm = ThemeManager::new();
        tm.set_accent_color("#FF0000");
        assert_eq!(tm.accent_color(), "#FF0000");
        tm.set_accent_color("invalid");
        assert_eq!(tm.accent_color(), "#FF0000"); // unchanged
    }

    #[test]
    fn test_accent_color_with_alpha() {
        let mut tm = ThemeManager::new();
        tm.set_accent_color("#FF000080");
        assert_eq!(tm.accent_color(), "#FF000080");
    }

    #[test]
    fn test_custom_css() {
        let mut tm = ThemeManager::new();
        tm.add_custom_css("test", ".test { color: red; }");
        assert!(tm.custom_css().contains("color: red"));
        tm.remove_custom_css("test");
        assert_eq!(tm.custom_css(), "");
    }

    #[test]
    fn test_generate_stylesheet_includes_mode() {
        let tm = ThemeManager::new();
        let css = tm.generate_stylesheet();
        assert!(css.contains("EduShell Desktop Theme - light mode"));
        assert!(!css.contains("EduShell Desktop Theme - dark mode"));
    }

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FF0000"), "255, 0, 0");
        assert_eq!(hex_to_rgb("#00FF00"), "0, 255, 0");
        assert_eq!(hex_to_rgb("#0000FF"), "0, 0, 255");
    }

    #[test]
    fn test_hex_to_rgb_without_hash() {
        assert_eq!(hex_to_rgb("4A90D9"), "74, 144, 217");
    }

    #[test]
    fn test_hex_to_rgb_invalid() {
        assert_eq!(hex_to_rgb("invalid"), "74, 144, 217"); // fallback
    }

    #[test]
    fn test_use_system_theme() {
        let mut tm = ThemeManager::new();
        tm.set_use_system_theme(true);
        assert!(tm.use_system_theme());
    }

    #[test]
    fn test_assets_path() {
        let tm = ThemeManager::new();
        assert!(tm.assets_path().ends_with("edushell/theme"));
    }

    #[test]
    fn test_generate_stylesheet_dark() {
        let mut tm = ThemeManager::new();
        tm.set_dark_mode(true);
        let css = tm.generate_stylesheet();
        assert!(css.contains("dark"));
    }

    #[test]
    fn test_accent_in_stylesheet() {
        let mut tm = ThemeManager::new();
        tm.set_accent_color("#00FF00");
        let css = tm.generate_stylesheet();
        assert!(css.contains("#00FF00"));
    }

    #[test]
    fn test_multi_custom_css() {
        let mut tm = ThemeManager::new();
        tm.add_custom_css("a", ".a {}");
        tm.add_custom_css("b", ".b {}");
        let css = tm.custom_css();
        assert!(css.contains(".a"));
        assert!(css.contains(".b"));
    }
}
