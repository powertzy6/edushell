//! Design System — tokens, spacing, color, typography, elevation.
use serde::{Deserialize, Serialize};

/// Spacing scale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingScale {
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub xxl: u32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xs: 4,
            sm: 8,
            md: 16,
            lg: 24,
            xl: 32,
            xxl: 48,
        }
    }
}

/// Color palette.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
    pub bg_primary: String,
    pub bg_secondary: String,
    pub fg_primary: String,
    pub fg_secondary: String,
    pub border: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: "#3584e4".into(),
            secondary: "#7c4dff".into(),
            success: "#35e435".into(),
            warning: "#e4a035".into(),
            danger: "#e43535".into(),
            info: "#35b5e4".into(),
            bg_primary: "#ffffff".into(),
            bg_secondary: "#f5f5f5".into(),
            fg_primary: "#1a1a1a".into(),
            fg_secondary: "#606060".into(),
            border: "#d0d0d0".into(),
        }
    }
}

/// Elevation levels (shadows).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elevation {
    pub level_0: String,
    pub level_1: String,
    pub level_2: String,
    pub level_3: String,
    pub level_4: String,
}

impl Default for Elevation {
    fn default() -> Self {
        Self {
            level_0: "none".into(),
            level_1: "0 1px 3px rgba(0,0,0,0.12)".into(),
            level_2: "0 4px 6px rgba(0,0,0,0.15)".into(),
            level_3: "0 10px 25px rgba(0,0,0,0.2)".into(),
            level_4: "0 20px 50px rgba(0,0,0,0.3)".into(),
        }
    }
}

/// Typography tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyTokens {
    pub font_family_base: String,
    pub font_family_mono: String,
    pub font_size_caption: f64,
    pub font_size_body: f64,
    pub font_size_subtitle: f64,
    pub font_size_title: f64,
    pub font_size_h1: f64,
    pub font_size_h2: f64,
    pub font_weight_normal: u32,
    pub font_weight_bold: u32,
    pub line_height: f64,
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            font_family_base: "Noto Sans, sans-serif".into(),
            font_family_mono: "Noto Sans Mono, monospace".into(),
            font_size_caption: 10.0,
            font_size_body: 12.0,
            font_size_subtitle: 14.0,
            font_size_title: 18.0,
            font_size_h1: 28.0,
            font_size_h2: 22.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        }
    }
}

/// Corner radius tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CornerRadii {
    pub none: f64,
    pub sm: f64,
    pub md: f64,
    pub lg: f64,
    pub full: f64,
}

impl Default for CornerRadii {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            full: 9999.0,
        }
    }
}

/// Motion/animation tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionTokens {
    pub duration_fast_ms: u64,
    pub duration_normal_ms: u64,
    pub duration_slow_ms: u64,
    pub easing_default: String,
    pub easing_decelerate: String,
    pub easing_accelerate: String,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            duration_fast_ms: 100,
            duration_normal_ms: 200,
            duration_slow_ms: 400,
            easing_default: "cubic-bezier(0.4, 0.0, 0.2, 1)".into(),
            easing_decelerate: "cubic-bezier(0.0, 0.0, 0.2, 1)".into(),
            easing_accelerate: "cubic-bezier(0.4, 0.0, 1, 1)".into(),
        }
    }
}

/// Complete design system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSystem {
    pub name: String,
    pub version: String,
    pub spacing: SpacingScale,
    pub colors: ColorPalette,
    pub elevation: Elevation,
    pub typography: TypographyTokens,
    pub radii: CornerRadii,
    pub motion: MotionTokens,
}

impl Default for DesignSystem {
    fn default() -> Self {
        Self {
            name: "EduShell Design System".into(),
            version: "2.0.0".into(),
            spacing: SpacingScale::default(),
            colors: ColorPalette::default(),
            elevation: Elevation::default(),
            typography: TypographyTokens::default(),
            radii: CornerRadii::default(),
            motion: MotionTokens::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_system_default() {
        let ds = DesignSystem::default();
        assert_eq!(ds.name, "EduShell Design System");
        assert_eq!(ds.version, "2.0.0");
    }

    #[test]
    fn test_spacing_default() {
        let s = SpacingScale::default();
        assert_eq!(s.md, 16);
    }

    #[test]
    fn test_color_palette_default() {
        let c = ColorPalette::default();
        assert_eq!(c.primary, "#3584e4");
    }

    #[test]
    fn test_elevation_default() {
        let e = Elevation::default();
        assert_eq!(e.level_0, "none");
        assert!(e.level_4.contains("20px"));
    }

    #[test]
    fn test_typography_default() {
        let t = TypographyTokens::default();
        assert!(t.font_size_body > 0.0);
        assert_eq!(t.font_weight_normal, 400);
    }

    #[test]
    fn test_corner_radii_default() {
        let r = CornerRadii::default();
        assert_eq!(r.full, 9999.0);
    }

    #[test]
    fn test_motion_default() {
        let m = MotionTokens::default();
        assert!(m.duration_normal_ms > 0);
        assert!(m.easing_default.contains("cubic-bezier"));
    }

    #[test]
    fn test_design_system_serde() {
        let ds = DesignSystem::default();
        let json = serde_json::to_string_pretty(&ds).unwrap();
        let d: DesignSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(d.name, "EduShell Design System");
    }
}
