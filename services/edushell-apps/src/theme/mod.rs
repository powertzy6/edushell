use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::Timelike;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppThemeMode {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppThemeConfig {
    pub mode: AppThemeMode,
    pub accent_color: String,
    pub rounded_corners: bool,
    pub animations_enabled: bool,
    pub wallpaper: String,
    pub font_size: f64,
    pub custom_css: HashMap<String, String>,
}

impl Default for AppThemeConfig {
    fn default() -> Self {
        Self {
            mode: AppThemeMode::Light,
            accent_color: "#3584e4".to_string(),
            rounded_corners: true,
            animations_enabled: true,
            wallpaper: "default".to_string(),
            font_size: 12.0,
            custom_css: HashMap::new(),
        }
    }
}

pub struct AppThemeManager {
    config: AppThemeConfig,
}

impl AppThemeManager {
    pub fn new(config: AppThemeConfig) -> Self {
        Self { config }
    }

    pub fn default() -> Self {
        Self {
            config: AppThemeConfig::default(),
        }
    }

    pub fn config(&self) -> &AppThemeConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut AppThemeConfig {
        &mut self.config
    }

    pub fn set_mode(&mut self, mode: AppThemeMode) {
        self.config.mode = mode;
    }

    pub fn set_accent(&mut self, color: &str) {
        self.config.accent_color = color.to_string();
    }

    pub fn set_rounded(&mut self, enabled: bool) {
        self.config.rounded_corners = enabled;
    }

    pub fn set_animations(&mut self, enabled: bool) {
        self.config.animations_enabled = enabled;
    }

    #[cfg(feature = "gtk")]
    pub fn apply(&self) {
        let css = self.generate_app_css();
        let provider = gtk4::CssProvider::new();
        provider.load_from_string(&css);
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    #[cfg(not(feature = "gtk"))]
    pub fn apply(&self) {}

    pub fn generate_app_css(&self) -> String {
        let mut css = String::new();
        css.push_str(&self.generate_root_variables());
        css.push('\n');

        let bg = if self.is_dark() { "#1a1a2e" } else { "#f5f5f5" };
        let fg = if self.is_dark() { "#e0e0e0" } else { "#2d2d2d" };
        let card_bg = if self.is_dark() { "#16213e" } else { "#ffffff" };
        let border = if self.is_dark() { "#2a2a4a" } else { "#d0d0d0" };
        let hover_bg = if self.is_dark() { "#1e2a4a" } else { "#e8e8e8" };
        let radius = if self.config.rounded_corners {
            "8px"
        } else {
            "0px"
        };

        css.push_str(&format!(
            r#":root {{
    --edu-bg: {bg};
    --edu-fg: {fg};
    --edu-card-bg: {card_bg};
    --edu-border: {border};
    --edu-hover-bg: {hover_bg};
    --edu-radius: {radius};
    --edu-transition-duration: {anim_dur};
}}
"#,
            bg = bg,
            fg = fg,
            card_bg = card_bg,
            border = border,
            hover_bg = hover_bg,
            radius = radius,
            anim_dur = if self.config.animations_enabled {
                "200ms"
            } else {
                "0ms"
            },
        ));

        if self.config.mode == AppThemeMode::Auto {
            css.push_str(
                r#"@media (prefers-color-scheme: dark) {
    :root {
        --edu-bg: #1a1a2e;
        --edu-fg: #e0e0e0;
        --edu-card-bg: #16213e;
        --edu-border: #2a2a4a;
        --edu-hover-bg: #1e2a4a;
    }
}
"#,
            );
        }

        for (selector, rules) in &self.config.custom_css {
            css.push_str(&format!("{} {{\n{}\n}}\n", selector, rules));
        }

        css
    }

    pub fn accent_color_rgb(&self) -> String {
        let hex = self.config.accent_color.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            format!("{}, {}, {}", r, g, b)
        } else if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0);
            format!("{}, {}, {}", r, g, b)
        } else {
            "53, 132, 228".to_string()
        }
    }

    pub fn is_dark(&self) -> bool {
        match self.config.mode {
            AppThemeMode::Dark => true,
            AppThemeMode::Light => false,
            AppThemeMode::Auto => {
                let now = chrono::Local::now();
                let hour = now.hour();
                hour >= 18 || hour < 6
            }
        }
    }

    pub fn generate_root_variables(&self) -> String {
        let rgb = self.accent_color_rgb();
        let accent = &self.config.accent_color;
        let radius = if self.config.rounded_corners {
            "8px"
        } else {
            "0px"
        };
        let dur = if self.config.animations_enabled {
            "200ms"
        } else {
            "0ms"
        };

        format!(
            r#":root {{
    --edu-accent: {accent};
    --edu-accent-rgb: {rgb};
    --edu-accent-bg: color-mix(in srgb, {accent} 20%, transparent);
    --edu-accent-fg: {accent_fg};
    --edu-radius: {radius};
    --edu-transition-duration: {dur};
    --edu-font-size: {fs}pt;
}}
"#,
            accent = accent,
            rgb = rgb,
            accent_fg = if self.is_dark() { "#ffffff" } else { "#ffffff" },
            radius = radius,
            dur = dur,
            fs = self.config.font_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let mgr = AppThemeManager::default();
        assert_eq!(mgr.config().mode, AppThemeMode::Light);
        assert_eq!(mgr.config().accent_color, "#3584e4");
        assert!(mgr.config().rounded_corners);
        assert!(mgr.config().animations_enabled);
        assert_eq!(mgr.config().font_size, 12.0);
    }

    #[test]
    fn test_mode_switching() {
        let mut mgr = AppThemeManager::default();
        assert!(!mgr.is_dark());
        mgr.set_mode(AppThemeMode::Dark);
        assert!(mgr.is_dark());
        mgr.set_mode(AppThemeMode::Light);
        assert!(!mgr.is_dark());
    }

    #[test]
    fn test_set_accent() {
        let mut mgr = AppThemeManager::default();
        mgr.set_accent("#ff0000");
        assert_eq!(mgr.config().accent_color, "#ff0000");
    }

    #[test]
    fn test_set_rounded() {
        let mut mgr = AppThemeManager::default();
        mgr.set_rounded(false);
        assert!(!mgr.config().rounded_corners);
    }

    #[test]
    fn test_set_animations() {
        let mut mgr = AppThemeManager::default();
        mgr.set_animations(false);
        assert!(!mgr.config().animations_enabled);
    }

    #[test]
    fn test_hex_conversion_full() {
        let mut mgr = AppThemeManager::default();
        mgr.set_accent("#ff8800");
        assert_eq!(mgr.accent_color_rgb(), "255, 136, 0");
    }

    #[test]
    fn test_hex_conversion_shorthand() {
        let mut mgr = AppThemeManager::default();
        mgr.set_accent("#f80");
        assert_eq!(mgr.accent_color_rgb(), "255, 136, 0");
    }

    #[test]
    fn test_hex_conversion_invalid() {
        let mut mgr = AppThemeManager::default();
        mgr.set_accent("invalid");
        assert_eq!(mgr.accent_color_rgb(), "53, 132, 228");
    }

    #[test]
    fn test_generate_root_variables() {
        let mgr = AppThemeManager::default();
        let vars = mgr.generate_root_variables();
        assert!(vars.contains("--edu-accent: #3584e4"));
        assert!(vars.contains("--edu-accent-rgb: 53, 132, 228"));
        assert!(vars.contains("--edu-radius: 8px"));
        assert!(vars.contains("--edu-transition-duration: 200ms"));
        assert!(vars.contains("--edu-font-size: 12pt"));
    }

    #[test]
    fn test_generate_app_css_light() {
        let mgr = AppThemeManager::default();
        let css = mgr.generate_app_css();
        assert!(css.contains("--edu-bg: #f5f5f5"));
        assert!(css.contains("--edu-fg: #2d2d2d"));
        assert!(css.contains("--edu-card-bg: #ffffff"));
    }

    #[test]
    fn test_generate_app_css_dark() {
        let mut mgr = AppThemeManager::default();
        mgr.set_mode(AppThemeMode::Dark);
        let css = mgr.generate_app_css();
        assert!(css.contains("--edu-bg: #1a1a2e"));
        assert!(css.contains("--edu-fg: #e0e0e0"));
        assert!(css.contains("--edu-card-bg: #16213e"));
    }

    #[test]
    fn test_generate_app_css_auto_has_media_query() {
        let mut mgr = AppThemeManager::default();
        mgr.set_mode(AppThemeMode::Auto);
        let css = mgr.generate_app_css();
        assert!(css.contains("@media (prefers-color-scheme: dark)"));
    }

    #[test]
    fn test_generate_app_css_rounded_off() {
        let mut mgr = AppThemeManager::default();
        mgr.set_rounded(false);
        let css = mgr.generate_app_css();
        assert!(css.contains("--edu-radius: 0px"));
    }

    #[test]
    fn test_generate_app_css_animations_off() {
        let mut mgr = AppThemeManager::default();
        mgr.set_animations(false);
        let css = mgr.generate_app_css();
        assert!(css.contains("--edu-transition-duration: 0ms"));
    }

    #[test]
    fn test_config_mut() {
        let mut mgr = AppThemeManager::default();
        mgr.config_mut().font_size = 14.0;
        assert_eq!(mgr.config().font_size, 14.0);
    }

    #[test]
    fn test_apply_noop_without_gtk() {
        let mgr = AppThemeManager::default();
        mgr.apply();
    }

    #[test]
    fn test_dark_detection_auto() {
        let mut mgr = AppThemeManager::default();
        mgr.set_mode(AppThemeMode::Auto);
        let _ = mgr.is_dark();
    }

    #[test]
    fn test_custom_css_in_output() {
        let mut mgr = AppThemeManager::default();
        mgr.config_mut()
            .custom_css
            .insert(".my-class".to_string(), "color: red;".to_string());
        let css = mgr.generate_app_css();
        assert!(css.contains(".my-class"));
        assert!(css.contains("color: red;"));
    }

    #[test]
    fn test_custom_config() {
        let config = AppThemeConfig {
            mode: AppThemeMode::Dark,
            accent_color: "#00ff00".to_string(),
            rounded_corners: false,
            animations_enabled: false,
            wallpaper: "custom".to_string(),
            font_size: 16.0,
            custom_css: HashMap::new(),
        };
        let mgr = AppThemeManager::new(config);
        assert!(mgr.is_dark());
        assert!(!mgr.config().rounded_corners);
        assert_eq!(mgr.config().accent_color, "#00ff00");
    }
}
