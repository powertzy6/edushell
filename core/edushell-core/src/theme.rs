// SPDX-License-Identifier: GPL-3.0-or-later

//! # Theme Engine
//!
//! Manages loading, switching, and applying themes.
//! Supports light/dark/auto modes, accent colors, icon themes,
//! cursor themes, wallpapers, fonts, and scaling.
//!
//! When compiled with the `gtk` feature, the engine also
//! applies CSS to GTK widgets via `CssProvider`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Timelike;

use crate::config::{ThemeConfig, ThemeMode};
use crate::error::EduError;
use crate::error::{EduResult, ThemeErrorKind};

/// Information about a loaded theme.
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    /// Theme identifier.
    pub name: String,
    /// Current mode.
    pub mode: ThemeMode,
    /// Accent color as hex string.
    pub accent_color: String,
    /// Icon theme name.
    pub icon_theme: String,
    /// Cursor theme name.
    pub cursor_theme: String,
    /// Wallpaper path.
    pub wallpaper: Option<PathBuf>,
    /// Font name with size.
    pub font: String,
    /// Monospace font name with size.
    pub monospace_font: String,
    /// Font size in points.
    pub font_size: u32,
    /// Display scale factor.
    pub scale_factor: f64,
}

/// RGB color representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaColor {
    /// Red (0-255).
    pub r: u8,
    /// Green (0-255).
    pub g: u8,
    /// Blue (0-255).
    pub b: u8,
    /// Alpha (0.0-1.0).
    pub a: f64,
}

impl RgbaColor {
    /// Parse a hex color string (e.g., "#1A237E" or "#1A237EFF").
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        let len = hex.len();

        let (r, g, b, a) = match len {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|e| e.to_string())?;
                (r, g, b, 1.0)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                (r, g, b, 1.0)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?;
                (r, g, b, a as f64 / 255.0)
            }
            _ => return Err(format!("Invalid hex color length: {len}")),
        };

        Ok(Self { r, g, b, a })
    }

    /// Convert to CSS hex string.
    pub fn to_css(&self) -> String {
        if (self.a - 1.0).abs() < f64::EPSILON {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!(
                "rgba({}, {}, {}, {})",
                self.r, self.g, self.b, self.a
            )
        }
    }

    /// Create a semi-transparent variant.
    pub fn with_alpha(&self, alpha: f64) -> Self {
        Self {
            a: alpha.min(1.0).max(0.0),
            ..*self
        }
    }
}

/// A complete color palette for a theme variant.
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: RgbaColor,
    pub on_primary: RgbaColor,
    pub primary_container: RgbaColor,
    pub secondary: RgbaColor,
    pub on_secondary: RgbaColor,
    pub background: RgbaColor,
    pub on_background: RgbaColor,
    pub surface: RgbaColor,
    pub on_surface: RgbaColor,
    pub error: RgbaColor,
    pub on_error: RgbaColor,
    pub accent: RgbaColor,
}

impl ColorPalette {
    /// Create a default light palette.
    pub fn light() -> Self {
        Self {
            primary: RgbaColor::from_hex("#1A237E").unwrap(),
            on_primary: RgbaColor::from_hex("#FFFFFF").unwrap(),
            primary_container: RgbaColor::from_hex("#E8EAF6").unwrap(),
            secondary: RgbaColor::from_hex("#00897B").unwrap(),
            on_secondary: RgbaColor::from_hex("#FFFFFF").unwrap(),
            background: RgbaColor::from_hex("#FAFAFA").unwrap(),
            on_background: RgbaColor::from_hex("#212121").unwrap(),
            surface: RgbaColor::from_hex("#FFFFFF").unwrap(),
            on_surface: RgbaColor::from_hex("#212121").unwrap(),
            error: RgbaColor::from_hex("#C62828").unwrap(),
            on_error: RgbaColor::from_hex("#FFFFFF").unwrap(),
            accent: RgbaColor::from_hex("#00897B").unwrap(),
        }
    }

    /// Create a default dark palette.
    pub fn dark() -> Self {
        Self {
            primary: RgbaColor::from_hex("#7986CB").unwrap(),
            on_primary: RgbaColor::from_hex("#1A237E").unwrap(),
            primary_container: RgbaColor::from_hex("#283593").unwrap(),
            secondary: RgbaColor::from_hex("#80CBC4").unwrap(),
            on_secondary: RgbaColor::from_hex("#004D40").unwrap(),
            background: RgbaColor::from_hex("#121212").unwrap(),
            on_background: RgbaColor::from_hex("#E0E0E0").unwrap(),
            surface: RgbaColor::from_hex("#1E1E1E").unwrap(),
            on_surface: RgbaColor::from_hex("#E0E0E0").unwrap(),
            error: RgbaColor::from_hex("#EF5350").unwrap(),
            on_error: RgbaColor::from_hex("#212121").unwrap(),
            accent: RgbaColor::from_hex("#80CBC4").unwrap(),
        }
    }
}

/// CSS theme generator.
pub struct CssTheme {
    /// Raw CSS content.
    pub css: String,
    /// CSS variable definitions.
    pub variables: HashMap<String, String>,
}

impl CssTheme {
    /// Generate CSS variables from a color palette.
    pub fn from_palette(palette: &ColorPalette) -> Self {
        let mut variables = HashMap::new();

        let pairs = [
            ("--edushell-primary", palette.primary.to_css()),
            ("--edushell-on-primary", palette.on_primary.to_css()),
            ("--edushell-primary-container", palette.primary_container.to_css()),
            ("--edushell-secondary", palette.secondary.to_css()),
            ("--edushell-on-secondary", palette.on_secondary.to_css()),
            ("--edushell-background", palette.background.to_css()),
            ("--edushell-on-background", palette.on_background.to_css()),
            ("--edushell-surface", palette.surface.to_css()),
            ("--edushell-on-surface", palette.on_surface.to_css()),
            ("--edushell-error", palette.error.to_css()),
            ("--edushell-on-error", palette.on_error.to_css()),
            ("--edushell-accent", palette.accent.to_css()),
        ];

        for (key, value) in &pairs {
            variables.insert((*key).to_string(), value.clone());
        }

        let css = build_css_variables(&variables);

        Self { css, variables }
    }
}

fn build_css_variables(vars: &HashMap<String, String>) -> String {
    let mut css = String::from(":root {\n");
    for (key, value) in vars {
        css.push_str(&format!("  {}: {};\n", key, value));
    }
    css.push_str("}\n");
    css
}

/// Theme engine managing all theme-related functionality.
#[derive(Clone)]
pub struct ThemeEngine {
    #[allow(dead_code)]
    config: ThemeConfig,
    current_info: Arc<std::sync::RwLock<ThemeInfo>>,
}

impl ThemeEngine {
    /// Create a new theme engine from configuration.
    pub fn new(config: &ThemeConfig) -> Self {
        let info = ThemeInfo {
            name: config.name.clone(),
            mode: config.mode.clone(),
            accent_color: config.accent_color.clone(),
            icon_theme: config.icon_theme.clone(),
            cursor_theme: config.cursor_theme.clone(),
            wallpaper: config.wallpaper.as_ref().map(PathBuf::from),
            font: config.font.clone(),
            monospace_font: config.monospace_font.clone(),
            font_size: config.font_size,
            scale_factor: config.scale_factor,
        };

        Self {
            config: config.clone(),
            current_info: Arc::new(std::sync::RwLock::new(info)),
        }
    }

    /// Get current theme information.
    pub fn current(&self) -> ThemeInfo {
        self.current_info.read()
            .map(|i| i.clone())
            .unwrap_or_else(|_| {
                ThemeInfo {
                    name: "default".into(),
                    mode: ThemeMode::Light,
                    accent_color: "#1A237E".into(),
                    icon_theme: "Papirus".into(),
                    cursor_theme: "default".into(),
                    wallpaper: None,
                    font: "Inter 10".into(),
                    monospace_font: "JetBrains Mono 10".into(),
                    font_size: 10,
                    scale_factor: 1.0,
                }
            })
    }

    /// Set the theme mode (light/dark/auto).
    pub fn set_mode(&self, mode: ThemeMode) -> EduResult<()> {
        let mut info = self.current_info.write()
            .map_err(|e| EduError::Unknown(format!("Theme lock poisoned: {e}")))?;
        info.mode = mode.clone();

        tracing::info!(
            target: "edushell::theme",
            mode = ?mode,
            "Theme mode changed"
        );

        Ok(())
    }

    /// Set the accent color.
    pub fn set_accent(&self, hex: &str) -> EduResult<()> {
        // Validate color
        RgbaColor::from_hex(hex)
            .map_err(|_| ThemeErrorKind::InvalidColor { value: hex.to_string() })?;

        let mut info = self.current_info.write()
            .map_err(|e| EduError::Unknown(format!("Theme lock poisoned: {e}")))?;
        info.accent_color = hex.to_string();

        tracing::info!(
            target: "edushell::theme",
            accent = hex,
            "Accent color changed"
        );

        Ok(())
    }

    /// Set the icon theme.
    pub fn set_icon_theme(&self, name: &str) -> EduResult<()> {
        let mut info = self.current_info.write()
            .map_err(|e| EduError::Unknown(format!("Theme lock poisoned: {e}")))?;
        info.icon_theme = name.to_string();
        Ok(())
    }

    /// Set the wallpaper.
    pub fn set_wallpaper(&self, path: &Path) -> EduResult<()> {
        if !path.exists() {
            return Err(ThemeErrorKind::NotFound {
                name: path.display().to_string(),
            }
            .into());
        }

        let mut info = self.current_info.write()
            .map_err(|e| EduError::Unknown(format!("Theme lock poisoned: {e}")))?;
        info.wallpaper = Some(path.to_path_buf());

        Ok(())
    }

    /// Set the display scale factor.
    pub fn set_scale(&self, factor: f64) -> EduResult<()> {
        if factor < 0.5 || factor > 4.0 {
            return Err(EduError::Unknown(format!(
                "Scale factor {} out of range [0.5, 4.0]",
                factor
            )));
        }

        let mut info = self.current_info.write()
            .map_err(|e| EduError::Unknown(format!("Theme lock poisoned: {e}")))?;
        info.scale_factor = factor;

        Ok(())
    }

    /// Get the color palette for the current mode.
    pub fn palette(&self) -> ColorPalette {
        let info = self.current();

        match info.mode {
            ThemeMode::Light => ColorPalette::light(),
            ThemeMode::Dark => ColorPalette::dark(),
            ThemeMode::Auto => {
                // Auto: check time of day
                let hour = chrono::Local::now().hour();
                if hour >= 6 && hour < 18 {
                    ColorPalette::light()
                } else {
                    ColorPalette::dark()
                }
            }
        }
    }

    /// Generate CSS variables for the current theme.
    pub fn generate_css(&self) -> CssTheme {
        let palette = self.palette();
        CssTheme::from_palette(&palette)
    }

    /// Get available themes from search paths.
    pub fn available_themes(&self) -> Vec<String> {
        let mut themes = Vec::new();

        // System themes
        let system_paths = vec![
            PathBuf::from("/usr/share/edushell/themes"),
            PathBuf::from("/usr/local/share/edushell/themes"),
        ];

        // User themes
        let user_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("edushell/themes");

        let search_paths = [system_paths, vec![user_path]].concat();

        for path in &search_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let manifest = entry.path().join("theme.toml");
                        if manifest.exists() {
                            if let Some(name) = entry.file_name().to_str() {
                                themes.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        themes
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_parsing() {
        let color = RgbaColor::from_hex("#1A237E").unwrap();
        assert_eq!(color.r, 0x1A);
        assert_eq!(color.g, 0x23);
        assert_eq!(color.b, 0x7E);
        assert!((color.a - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_hex_color_with_alpha() {
        let color = RgbaColor::from_hex("#1A237E80").unwrap();
        assert_eq!(color.r, 0x1A);
        assert!((color.a - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_short_hex_color() {
        let color = RgbaColor::from_hex("#123").unwrap();
        assert_eq!(color.r, 0x11);
        assert_eq!(color.g, 0x22);
        assert_eq!(color.b, 0x33);
    }

    #[test]
    fn test_invalid_hex() {
        assert!(RgbaColor::from_hex("notacolor").is_err());
        assert!(RgbaColor::from_hex("#GGGGGG").is_err());
    }

    #[test]
    fn test_color_to_css() {
        let color = RgbaColor { r: 26, g: 35, b: 126, a: 1.0 };
        assert_eq!(color.to_css(), "#1A237E");

        let transparent = color.with_alpha(0.5);
        assert!((transparent.a - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_theme_engine_creation() {
        let config = ThemeConfig::default();
        let engine = ThemeEngine::new(&config);
        let info = engine.current();
        assert_eq!(info.name, "edushell-default");
    }

    #[test]
    fn test_theme_mode_switch() {
        let config = ThemeConfig::default();
        let engine = ThemeEngine::new(&config);

        engine.set_mode(ThemeMode::Dark).unwrap();
        let info = engine.current();
        assert_eq!(info.mode, ThemeMode::Dark);

        engine.set_mode(ThemeMode::Light).unwrap();
        let info = engine.current();
        assert_eq!(info.mode, ThemeMode::Light);
    }

    #[test]
    fn test_accent_color_validation() {
        let config = ThemeConfig::default();
        let engine = ThemeEngine::new(&config);

        assert!(engine.set_accent("#FF0000").is_ok());
        assert!(engine.set_accent("invalid").is_err());
    }

    #[test]
    fn test_palette_light_defaults() {
        let palette = ColorPalette::light();
        assert_eq!(palette.primary.to_css(), "#1A237E");
        assert_eq!(palette.on_primary.to_css(), "#FFFFFF");
    }

    #[test]
    fn test_css_generation() {
        let palette = ColorPalette::light();
        let theme = CssTheme::from_palette(&palette);
        assert!(theme.css.contains("--edushell-primary"));
        assert!(theme.css.contains("#1A237E"));
    }

    #[test]
    fn test_set_scale_validation() {
        let config = ThemeConfig::default();
        let engine = ThemeEngine::new(&config);

        assert!(engine.set_scale(1.0).is_ok());
        assert!(engine.set_scale(1.5).is_ok());
        assert!(engine.set_scale(0.25).is_err());
        assert!(engine.set_scale(5.0).is_err());
    }
}
