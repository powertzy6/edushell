#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeComponent {
    WindowBorder,
    Shadow,
    CornerRadius,
    Opacity,
    Blur,
    Decoration,
    Titlebar,
    AccentColor,
    Background,
    Foreground,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TitlebarStyle {
    Default,
    Compact,
    Hidden,
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecorationsSide {
    Left,
    Right,
    Top,
    Bottom,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderStyle {
    Solid,
    Raised,
    Sunken,
    Gradient,
    Rounded,
    None,
}

#[derive(Debug, Clone)]
pub struct WindowBorderConfig {
    pub width: i32,
    pub color: String,
    pub active_color: String,
    pub inactive_color: String,
    pub style: BorderStyle,
}

#[derive(Debug, Clone)]
pub struct ShadowConfig {
    pub blur_radius: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub color: String,
    pub opacity: f64,
    pub spread: f64,
    pub inset: bool,
}

#[derive(Debug, Clone)]
pub struct CornerRadiusConfig {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl CornerRadiusConfig {
    pub fn uniform(radius: f64) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeDecoration {
    pub show_titlebar: bool,
    pub titlebar_height: i32,
    pub titlebar_style: TitlebarStyle,
    pub show_buttons: bool,
    pub button_layout: String,
    pub decorations_side: DecorationsSide,
    pub border: WindowBorderConfig,
    pub shadow: ShadowConfig,
    pub corner_radius: CornerRadiusConfig,
}

#[derive(Debug, Clone)]
pub struct ButtonLayoutConfig {
    pub close: bool,
    pub minimize: bool,
    pub maximize: bool,
    pub menu: bool,
    pub sticky: bool,
    pub shade: bool,
}

impl Default for ButtonLayoutConfig {
    fn default() -> Self {
        Self {
            close: true,
            minimize: true,
            maximize: true,
            menu: false,
            sticky: false,
            shade: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpacityConfig {
    pub normal: f64,
    pub focused: f64,
    pub inactive: f64,
    pub background: f64,
}

impl Default for OpacityConfig {
    fn default() -> Self {
        Self {
            normal: 1.0,
            focused: 1.0,
            inactive: 0.85,
            background: 0.95,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlurConfig {
    pub enabled: bool,
    pub radius: f64,
    pub passes: u32,
    pub brightness: f64,
}

impl Default for BlurConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            radius: 8.0,
            passes: 3,
            brightness: 0.8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub surface: String,
    pub surface_variant: String,
    pub outline: String,
    pub error: String,
    pub on_accent: String,
    pub on_background: String,
    pub on_surface: String,
    pub on_error: String,
}

pub struct ThemeLayer {
    pub name: String,
    pub is_dark: bool,
    pub decoration: ThemeDecoration,
    pub button_layout: ButtonLayoutConfig,
    pub opacity: OpacityConfig,
    pub blur: BlurConfig,
    pub colors: ColorScheme,
    pub animations_enabled: bool,
    pub animation_duration_ms: u64,
}

impl ThemeLayer {
    pub fn new(name: String, is_dark: bool) -> Self {
        let (bg, fg, surface, accent) = if is_dark {
            ("#1a1a1a", "#ffffff", "#2a2a2a", "#5294e2")
        } else {
            ("#f5f5f5", "#1a1a1a", "#ffffff", "#3584e4")
        };
        Self {
            name,
            is_dark,
            decoration: ThemeDecoration {
                show_titlebar: true,
                titlebar_height: 32,
                titlebar_style: TitlebarStyle::Default,
                show_buttons: true,
                button_layout: ":minimize,maximize,close:".to_string(),
                decorations_side: DecorationsSide::All,
                border: WindowBorderConfig {
                    width: 1,
                    color: accent.to_string(),
                    active_color: accent.to_string(),
                    inactive_color: if is_dark { "#555555" } else { "#cccccc" }.to_string(),
                    style: BorderStyle::Solid,
                },
                shadow: ShadowConfig {
                    blur_radius: 12.0,
                    offset_x: 0.0,
                    offset_y: 2.0,
                    color: "#000000".to_string(),
                    opacity: 0.4,
                    spread: 0.0,
                    inset: false,
                },
                corner_radius: CornerRadiusConfig::uniform(8.0),
            },
            button_layout: ButtonLayoutConfig::default(),
            opacity: OpacityConfig::default(),
            blur: BlurConfig::default(),
            colors: ColorScheme {
                accent: accent.to_string(),
                background: bg.to_string(),
                foreground: fg.to_string(),
                surface: surface.to_string(),
                surface_variant: if is_dark { "#3a3a3a" } else { "#e6e6e6" }.to_string(),
                outline: if is_dark { "#555555" } else { "#cccccc" }.to_string(),
                error: "#e74c3c".to_string(),
                on_accent: "#ffffff".to_string(),
                on_background: fg.to_string(),
                on_surface: fg.to_string(),
                on_error: "#ffffff".to_string(),
            },
            animations_enabled: true,
            animation_duration_ms: 200,
        }
    }

    pub fn set_decoration(&mut self, decoration: ThemeDecoration) {
        self.decoration = decoration;
    }

    pub fn get_decoration(&self) -> &ThemeDecoration {
        &self.decoration
    }

    pub fn set_button_layout(&mut self, layout: ButtonLayoutConfig) {
        self.button_layout = layout;
    }

    pub fn get_button_layout(&self) -> &ButtonLayoutConfig {
        &self.button_layout
    }

    pub fn set_opacity(&mut self, opacity: OpacityConfig) {
        self.opacity = opacity;
    }

    pub fn set_blur(&mut self, blur: BlurConfig) {
        self.blur = blur;
    }

    pub fn set_colors(&mut self, colors: ColorScheme) {
        self.colors = colors;
    }

    pub fn get_colors(&self) -> &ColorScheme {
        &self.colors
    }

    pub fn set_animation(&mut self, enabled: bool, duration_ms: u64) {
        self.animations_enabled = enabled;
        self.animation_duration_ms = duration_ms;
    }

    pub fn get_active_border_color(&self, is_focused: bool) -> &str {
        if is_focused {
            &self.decoration.border.active_color
        } else {
            &self.decoration.border.inactive_color
        }
    }

    pub fn render_css(&self) -> String {
        format!(
            r#"/* EduWM Theme: {name} */
:root {{
  --wm-accent: {accent};
  --wm-background: {bg};
  --wm-foreground: {fg};
  --wm-surface: {surface};
  --wm-surface-variant: {surface_variant};
  --wm-outline: {outline};
  --wm-error: {error};
  --wm-on-accent: {on_accent};
  --wm-on-background: {on_bg};
  --wm-on-surface: {on_surface};
  --wm-on-error: {on_error};
  --wm-border-width: {bw}px;
  --wm-border-color: {bc};
  --wm-border-active: {bac};
  --wm-border-inactive: {bic};
  --wm-border-style: {bs};
  --wm-shadow-blur: {sb}px;
  --wm-shadow-offset-x: {sox}px;
  --wm-shadow-offset-y: {soy}px;
  --wm-shadow-color: {sc};
  --wm-shadow-opacity: {so};
  --wm-shadow-spread: {ssp}px;
  --wm-radius-top-left: {rtl}px;
  --wm-radius-top-right: {rtr}px;
  --wm-radius-bottom-right: {rbr}px;
  --wm-radius-bottom-left: {rbl}px;
  --wm-titlebar-height: {th}px;
  --wm-opacity-normal: {on};
  --wm-opacity-focused: {of};
  --wm-opacity-inactive: {oi};
  --wm-opacity-background: {ob};
  --wm-blur-radius: {br}px;
  --wm-blur-passes: {bp};
  --wm-blur-brightness: {bb};
  --wm-animation-duration: {ad}ms;
  --wm-dark: {dark};
}}
"#,
            name = self.name,
            accent = self.colors.accent,
            bg = self.colors.background,
            fg = self.colors.foreground,
            surface = self.colors.surface,
            surface_variant = self.colors.surface_variant,
            outline = self.colors.outline,
            error = self.colors.error,
            on_accent = self.colors.on_accent,
            on_bg = self.colors.on_background,
            on_surface = self.colors.on_surface,
            on_error = self.colors.on_error,
            bw = self.decoration.border.width,
            bc = self.decoration.border.color,
            bac = self.decoration.border.active_color,
            bic = self.decoration.border.inactive_color,
            bs = format!("{:?}", self.decoration.border.style).to_lowercase(),
            sb = self.decoration.shadow.blur_radius,
            sox = self.decoration.shadow.offset_x,
            soy = self.decoration.shadow.offset_y,
            sc = self.decoration.shadow.color,
            so = self.decoration.shadow.opacity,
            ssp = self.decoration.shadow.spread,
            rtl = self.decoration.corner_radius.top_left,
            rtr = self.decoration.corner_radius.top_right,
            rbr = self.decoration.corner_radius.bottom_right,
            rbl = self.decoration.corner_radius.bottom_left,
            th = self.decoration.titlebar_height,
            on = self.opacity.normal,
            of = self.opacity.focused,
            oi = self.opacity.inactive,
            ob = self.opacity.background,
            br = self.blur.radius,
            bp = self.blur.passes,
            bb = self.blur.brightness,
            ad = self.animation_duration_ms,
            dark = if self.is_dark { "true" } else { "false" },
        )
    }
}

pub struct ThemeManager {
    pub themes: Vec<ThemeLayer>,
    pub active: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            themes: vec![ThemeLayer::new("Cinnamon Light".to_string(), false)],
            active: 0,
        }
    }

    pub fn add_theme(&mut self, theme: ThemeLayer) {
        self.themes.push(theme);
    }

    pub fn remove_theme(&mut self, idx: usize) -> bool {
        if idx >= self.themes.len() || self.themes.len() <= 1 {
            return false;
        }
        self.themes.remove(idx);
        if self.active >= self.themes.len() {
            self.active = self.themes.len().saturating_sub(1);
        } else if self.active > idx || (self.active == idx && self.active > 0) {
            self.active -= 1;
        }
        true
    }

    pub fn activate_theme(&mut self, idx: usize) -> bool {
        if idx < self.themes.len() {
            self.active = idx;
            true
        } else {
            false
        }
    }

    pub fn get_active_theme(&self) -> &ThemeLayer {
        &self.themes[self.active]
    }

    pub fn get_active_theme_mut(&mut self) -> &mut ThemeLayer {
        &mut self.themes[self.active]
    }

    pub fn get_theme(&self, idx: usize) -> Option<&ThemeLayer> {
        self.themes.get(idx)
    }

    pub fn list_themes(&self) -> &[ThemeLayer] {
        &self.themes
    }

    pub fn count(&self) -> usize {
        self.themes.len()
    }

    pub fn switch_dark_mode(&mut self, is_dark: bool) {
        if self.get_active_theme().is_dark == is_dark {
            return;
        }
        for (i, theme) in self.themes.iter().enumerate() {
            if theme.is_dark == is_dark {
                self.active = i;
                return;
            }
        }
        let theme = ThemeLayer::new(
            if is_dark {
                "Dark".to_string()
            } else {
                "Light".to_string()
            },
            is_dark,
        );
        self.themes.push(theme);
        self.active = self.themes.len() - 1;
    }

    pub fn render_current_css(&self) -> String {
        self.get_active_theme().render_css()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme_creation() {
        let theme = ThemeLayer::new("Test".to_string(), false);
        assert_eq!(theme.name, "Test");
        assert!(!theme.is_dark);
        assert_eq!(theme.colors.background, "#f5f5f5");
        assert_eq!(theme.colors.foreground, "#1a1a1a");
    }

    #[test]
    fn test_dark_mode() {
        let theme = ThemeLayer::new("Dark Test".to_string(), true);
        assert!(theme.is_dark);
        assert_eq!(theme.colors.background, "#1a1a1a");
        assert_eq!(theme.colors.foreground, "#ffffff");
    }

    #[test]
    fn test_border_config() {
        let mut theme = ThemeLayer::new("Test".to_string(), false);
        theme.decoration.border.width = 2;
        theme.decoration.border.active_color = "#ff0000".to_string();
        assert_eq!(theme.decoration.border.width, 2);
        assert_eq!(theme.get_active_border_color(true), "#ff0000");
        assert_eq!(theme.get_active_border_color(false), "#cccccc");
    }

    #[test]
    fn test_shadow_config() {
        let mut theme = ThemeLayer::new("Test".to_string(), false);
        theme.decoration.shadow.blur_radius = 20.0;
        theme.decoration.shadow.opacity = 0.5;
        assert_eq!(theme.decoration.shadow.blur_radius, 20.0);
        assert_eq!(theme.decoration.shadow.opacity, 0.5);
    }

    #[test]
    fn test_corner_radius_uniform() {
        let cr = CornerRadiusConfig::uniform(10.0);
        assert_eq!(cr.top_left, 10.0);
        assert_eq!(cr.top_right, 10.0);
        assert_eq!(cr.bottom_right, 10.0);
        assert_eq!(cr.bottom_left, 10.0);
    }

    #[test]
    fn test_css_rendering() {
        let theme = ThemeLayer::new("Test".to_string(), false);
        let css = theme.render_css();
        assert!(css.contains("Cinnamon Light") || css.contains("Test"));
        assert!(css.contains("--wm-accent"));
        assert!(css.contains("--wm-background"));
        assert!(css.contains("--wm-border-width"));
        assert!(css.contains("--wm-shadow-blur"));
    }

    #[test]
    fn test_theme_manager_add_remove() {
        let mut mgr = ThemeManager::new();
        let initial = mgr.count();
        mgr.add_theme(ThemeLayer::new("Extra".to_string(), false));
        assert_eq!(mgr.count(), initial + 1);
        assert!(mgr.remove_theme(initial));
        assert_eq!(mgr.count(), initial);
        assert!(!mgr.remove_theme(99));
    }

    #[test]
    fn test_theme_manager_activate() {
        let mut mgr = ThemeManager::new();
        mgr.add_theme(ThemeLayer::new("Dark".to_string(), true));
        assert!(mgr.activate_theme(1));
        assert_eq!(mgr.get_active_theme().name, "Dark");
        assert!(!mgr.activate_theme(5));
    }

    #[test]
    fn test_switch_dark_mode() {
        let mut mgr = ThemeManager::new();
        assert!(!mgr.get_active_theme().is_dark);
        mgr.switch_dark_mode(true);
        assert!(mgr.get_active_theme().is_dark);
        mgr.switch_dark_mode(false);
        assert!(!mgr.get_active_theme().is_dark);
    }

    #[test]
    fn test_button_layout() {
        let layout = ButtonLayoutConfig::default();
        assert!(layout.close);
        assert!(layout.minimize);
        assert!(layout.maximize);
        assert!(!layout.menu);
        let mut theme = ThemeLayer::new("Test".to_string(), false);
        theme.button_layout.close = false;
        assert!(!theme.button_layout.close);
    }
}
