//! Theme SDK — create themes for EduShell.
use serde::{Deserialize, Serialize};

/// Complete theme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDefinition {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub dark: ThemeColors,
    pub light: ThemeColors,
    pub accent: String,
    pub typography: TypographyConfig,
    pub radii: RadiiConfig,
    pub shadows: ShadowConfig,
    pub transparency: f64,
    pub blur: bool,
}

/// Color palette for a theme mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_tertiary: String,
    pub fg_primary: String,
    pub fg_secondary: String,
    pub fg_tertiary: String,
    pub border: String,
    pub highlight: String,
    pub danger: String,
    pub success: String,
    pub warning: String,
}

/// Typography configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    pub font_family: String,
    pub font_size_base: f64,
    pub font_size_small: f64,
    pub font_size_large: f64,
    pub font_size_title: f64,
    pub line_height: f64,
}

/// Border radius configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiiConfig {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
    pub full: f64,
}

/// Shadow configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowConfig {
    pub small: String,
    pub medium: String,
    pub large: String,
}

impl ThemeDefinition {
    pub fn default_dark() -> Self {
        Self {
            id: "edushell-dark".into(),
            name: "EduShell Dark".into(),
            author: "EduShell Team".into(),
            version: "1.0.0".into(),
            description: "Default dark theme".into(),
            dark: ThemeColors::default_dark(),
            light: ThemeColors::default_light(),
            accent: "#3584e4".into(),
            typography: TypographyConfig::default(),
            radii: RadiiConfig::default(),
            shadows: ShadowConfig::default(),
            transparency: 0.85,
            blur: true,
        }
    }
}

impl ThemeColors {
    pub fn default_dark() -> Self {
        Self {
            bg_primary: "#1a1a2e".into(),
            bg_secondary: "#16213e".into(),
            bg_tertiary: "#0f3460".into(),
            fg_primary: "#e0e0e0".into(),
            fg_secondary: "#a0a0a0".into(),
            fg_tertiary: "#606060".into(),
            border: "#2a2a3e".into(),
            highlight: "#3584e4".into(),
            danger: "#e43535".into(),
            success: "#35e435".into(),
            warning: "#e4a035".into(),
        }
    }

    pub fn default_light() -> Self {
        Self {
            bg_primary: "#ffffff".into(),
            bg_secondary: "#f5f5f5".into(),
            bg_tertiary: "#e8e8e8".into(),
            fg_primary: "#1a1a1a".into(),
            fg_secondary: "#606060".into(),
            fg_tertiary: "#a0a0a0".into(),
            border: "#d0d0d0".into(),
            highlight: "#3584e4".into(),
            danger: "#e43535".into(),
            success: "#35a035".into(),
            warning: "#e4a035".into(),
        }
    }
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self {
            font_family: "Noto Sans, sans-serif".into(),
            font_size_base: 12.0,
            font_size_small: 10.0,
            font_size_large: 14.0,
            font_size_title: 18.0,
            line_height: 1.5,
        }
    }
}

impl Default for RadiiConfig {
    fn default() -> Self {
        Self {
            small: 4.0,
            medium: 8.0,
            large: 12.0,
            full: 9999.0,
        }
    }
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            small: "0 1px 3px rgba(0,0,0,0.12)".into(),
            medium: "0 4px 6px rgba(0,0,0,0.15)".into(),
            large: "0 10px 25px rgba(0,0,0,0.2)".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dark_theme() {
        let t = ThemeDefinition::default_dark();
        assert_eq!(t.id, "edushell-dark");
        assert!(t.blur);
    }

    #[test]
    fn test_colors_default_dark() {
        let c = ThemeColors::default_dark();
        assert_eq!(c.bg_primary, "#1a1a2e");
    }

    #[test]
    fn test_colors_default_light() {
        let c = ThemeColors::default_light();
        assert_eq!(c.bg_primary, "#ffffff");
    }

    #[test]
    fn test_typography_default() {
        let t = TypographyConfig::default();
        assert!(t.font_size_base > 0.0);
    }

    #[test]
    fn test_theme_serde_roundtrip() {
        let t = ThemeDefinition::default_dark();
        let json = serde_json::to_string(&t).unwrap();
        let deserialized: ThemeDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "edushell-dark");
    }

    #[test]
    fn test_radii_default() {
        let r = RadiiConfig::default();
        assert_eq!(r.full, 9999.0);
    }

    #[test]
    fn test_shadow_default() {
        let s = ShadowConfig::default();
        assert!(s.medium.contains("rgba"));
    }
}
