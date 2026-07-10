use crate::localization::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettingsCategory {
    Appearance,
    Dock,
    Panel,
    Theme,
    Wallpaper,
    Font,
    Animation,
    Accessibility,
    Language,
    Keyboard,
    Mouse,
    Touchpad,
    Monitor,
    Power,
    Battery,
    Bluetooth,
    Wifi,
    Privacy,
    Notification,
    Update,
    About,
    Learning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingsValue {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Color(String),
    Choice(String, Vec<String>),
    Path(String),
    List(Vec<String>),
}

impl SettingsValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            SettingsValue::String(s) => Some(s.as_str()),
            SettingsValue::Color(s) => Some(s.as_str()),
            SettingsValue::Path(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SettingsValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            SettingsValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            SettingsValue::Float(f) => Some(*f),
            SettingsValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_choice(&self) -> Option<(&str, &[String])> {
        match self {
            SettingsValue::Choice(selected, options) => Some((selected.as_str(), options.as_slice())),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            SettingsValue::List(v) => Some(v.as_slice()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsEntry {
    pub key: String,
    pub name_key: String,
    pub description_key: String,
    pub category: SettingsCategory,
    pub value: SettingsValue,
    pub default_value: SettingsValue,
    pub requires_restart: bool,
    pub requires_reload: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub advanced: bool,
}

impl SettingsEntry {
    pub fn is_default(&self) -> bool {
        values_equal(&self.value, &self.default_value)
    }

    pub fn reset(&mut self) {
        self.value = self.default_value.clone();
    }

    pub fn display_name(&self, l10n: &LocalizationManager) -> String {
        l10n.get(&self.name_key).to_string()
    }

    pub fn display_description(&self, l10n: &LocalizationManager) -> String {
        l10n.get(&self.description_key).to_string()
    }
}

fn values_equal(a: &SettingsValue, b: &SettingsValue) -> bool {
    match (a, b) {
        (SettingsValue::String(a), SettingsValue::String(b)) => a == b,
        (SettingsValue::Bool(a), SettingsValue::Bool(b)) => a == b,
        (SettingsValue::Int(a), SettingsValue::Int(b)) => a == b,
        (SettingsValue::Float(a), SettingsValue::Float(b)) => (a - b).abs() < f64::EPSILON,
        (SettingsValue::Color(a), SettingsValue::Color(b)) => a == b,
        (SettingsValue::Choice(a, _), SettingsValue::Choice(b, _)) => a == b,
        (SettingsValue::Path(a), SettingsValue::Path(b)) => a == b,
        (SettingsValue::List(a), SettingsValue::List(b)) => a == b,
        _ => false,
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("setting not found: {0}")]
    NotFound(String),
    #[error("type mismatch for setting: {0}")]
    TypeMismatch(String),
    #[error("value out of bounds for setting: {0}")]
    OutOfBounds(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SettingsError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SettingsData {
    entries: HashMap<String, SettingsEntry>,
}

pub struct SettingsCenter {
    entries: HashMap<String, SettingsEntry>,
    categories: Vec<SettingsCategory>,
    localization: LocalizationManager,
    config_dir: PathBuf,
    changed_entries: Vec<String>,
    search_index: HashMap<String, Vec<String>>,
}

impl SettingsCenter {
    pub fn new(config_dir: Option<PathBuf>) -> Self {
        let cfg = config_dir.unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".config/edushell")
        });
        let localization = LocalizationManager::new();
        let mut sc = Self {
            entries: HashMap::new(),
            categories: vec![],
            localization,
            config_dir: cfg,
            changed_entries: vec![],
            search_index: HashMap::new(),
        };
        sc.init_categories();
        sc.register_all_defaults();
        sc.build_search_index();
        sc
    }

    fn init_categories(&mut self) {
        self.categories = vec![
            SettingsCategory::Appearance,
            SettingsCategory::Dock,
            SettingsCategory::Panel,
            SettingsCategory::Theme,
            SettingsCategory::Wallpaper,
            SettingsCategory::Font,
            SettingsCategory::Animation,
            SettingsCategory::Accessibility,
            SettingsCategory::Language,
            SettingsCategory::Keyboard,
            SettingsCategory::Mouse,
            SettingsCategory::Touchpad,
            SettingsCategory::Monitor,
            SettingsCategory::Power,
            SettingsCategory::Battery,
            SettingsCategory::Bluetooth,
            SettingsCategory::Wifi,
            SettingsCategory::Privacy,
            SettingsCategory::Notification,
            SettingsCategory::Update,
            SettingsCategory::About,
            SettingsCategory::Learning,
        ];
    }

    fn register(&mut self, entry: SettingsEntry) {
        let key = entry.key.clone();
        self.entries.insert(key, entry);
    }

    fn se(
        key: &str, name_key: &str, desc_key: &str, cat: SettingsCategory,
        value: SettingsValue, default: SettingsValue,
        restart: bool, reload: bool, min: Option<f64>, max: Option<f64>,
        step: Option<f64>, advanced: bool,
    ) -> SettingsEntry {
        SettingsEntry {
            key: key.to_string(),
            name_key: name_key.to_string(),
            description_key: desc_key.to_string(),
            category: cat,
            value: value.clone(),
            default_value: default,
            requires_restart: restart,
            requires_reload: reload,
            min,
            max,
            step,
            advanced,
        }
    }

    fn register_all_defaults(&mut self) {
        // Appearance (12)
        self.register(Self::se("appearance.theme_mode", "settings.appearance.theme_mode", "settings.appearance.theme_mode.desc", SettingsCategory::Appearance, SettingsValue::Choice("auto".into(), vec!["light".into(), "dark".into(), "auto".into()]), SettingsValue::Choice("auto".into(), vec!["light".into(), "dark".into(), "auto".into()]), false, true, None, None, None, false));
        self.register(Self::se("appearance.accent_color", "settings.appearance.accent_color", "settings.appearance.accent_color.desc", SettingsCategory::Appearance, SettingsValue::Color("#4A90D9".into()), SettingsValue::Color("#4A90D9".into()), false, true, None, None, None, false));
        self.register(Self::se("appearance.rounded_corners", "settings.appearance.rounded_corners", "settings.appearance.rounded_corners.desc", SettingsCategory::Appearance, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("appearance.animations_enabled", "settings.appearance.animations_enabled", "settings.appearance.animations_enabled.desc", SettingsCategory::Appearance, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("appearance.transparency_effects", "settings.appearance.transparency_effects", "settings.appearance.transparency_effects.desc", SettingsCategory::Appearance, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("appearance.wallpaper", "settings.appearance.wallpaper", "settings.appearance.wallpaper.desc", SettingsCategory::Appearance, SettingsValue::Path("".into()), SettingsValue::Path("".into()), false, true, None, None, None, false));
        self.register(Self::se("appearance.font_display", "settings.appearance.font_display", "settings.appearance.font_display.desc", SettingsCategory::Appearance, SettingsValue::String("Noto Sans".into()), SettingsValue::String("Noto Sans".into()), false, true, None, None, None, false));
        self.register(Self::se("appearance.font_mono", "settings.appearance.font_mono", "settings.appearance.font_mono.desc", SettingsCategory::Appearance, SettingsValue::String("Noto Sans Mono".into()), SettingsValue::String("Noto Sans Mono".into()), false, true, None, None, None, false));
        self.register(Self::se("appearance.font_size", "settings.appearance.font_size", "settings.appearance.font_size.desc", SettingsCategory::Appearance, SettingsValue::Float(10.0), SettingsValue::Float(10.0), false, true, Some(6.0), Some(24.0), Some(1.0), false));
        self.register(Self::se("appearance.icon_theme", "settings.appearance.icon_theme", "settings.appearance.icon_theme.desc", SettingsCategory::Appearance, SettingsValue::String("edushell".into()), SettingsValue::String("edushell".into()), false, true, None, None, None, false));
        self.register(Self::se("appearance.cursor_theme", "settings.appearance.cursor_theme", "settings.appearance.cursor_theme.desc", SettingsCategory::Appearance, SettingsValue::String("default".into()), SettingsValue::String("default".into()), false, true, None, None, None, false));
        self.register(Self::se("appearance.cursor_size", "settings.appearance.cursor_size", "settings.appearance.cursor_size.desc", SettingsCategory::Appearance, SettingsValue::Int(24), SettingsValue::Int(24), false, true, Some(16.0), Some(64.0), Some(2.0), false));

        // Dock (8)
        self.register(Self::se("dock.position", "settings.dock.position", "settings.dock.position.desc", SettingsCategory::Dock, SettingsValue::Choice("bottom".into(), vec!["bottom".into(), "left".into(), "right".into()]), SettingsValue::Choice("bottom".into(), vec!["bottom".into(), "left".into(), "right".into()]), true, false, None, None, None, false));
        self.register(Self::se("dock.auto_hide", "settings.dock.auto_hide", "settings.dock.auto_hide.desc", SettingsCategory::Dock, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("dock.icon_size", "settings.dock.icon_size", "settings.dock.icon_size.desc", SettingsCategory::Dock, SettingsValue::Int(48), SettingsValue::Int(48), true, false, Some(24.0), Some(96.0), Some(4.0), false));
        self.register(Self::se("dock.magnification", "settings.dock.magnification", "settings.dock.magnification.desc", SettingsCategory::Dock, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("dock.magnification_size", "settings.dock.magnification_size", "settings.dock.magnification_size.desc", SettingsCategory::Dock, SettingsValue::Int(64), SettingsValue::Int(64), false, true, Some(32.0), Some(128.0), Some(8.0), true));
        self.register(Self::se("dock.opacity", "settings.dock.opacity", "settings.dock.opacity.desc", SettingsCategory::Dock, SettingsValue::Float(0.8), SettingsValue::Float(0.8), false, true, Some(0.0), Some(1.0), Some(0.05), false));
        self.register(Self::se("dock.show_running", "settings.dock.show_running", "settings.dock.show_running.desc", SettingsCategory::Dock, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("dock.show_trash", "settings.dock.show_trash", "settings.dock.show_trash.desc", SettingsCategory::Dock, SettingsValue::Bool(true), SettingsValue::Bool(true), false, false, None, None, None, false));

        // Panel (6)
        self.register(Self::se("panel.position", "settings.panel.position", "settings.panel.position.desc", SettingsCategory::Panel, SettingsValue::Choice("top".into(), vec!["top".into(), "bottom".into()]), SettingsValue::Choice("top".into(), vec!["top".into(), "bottom".into()]), true, false, None, None, None, false));
        self.register(Self::se("panel.auto_hide", "settings.panel.auto_hide", "settings.panel.auto_hide.desc", SettingsCategory::Panel, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("panel.height", "settings.panel.height", "settings.panel.height.desc", SettingsCategory::Panel, SettingsValue::Int(36), SettingsValue::Int(36), true, false, Some(24.0), Some(64.0), Some(2.0), false));
        self.register(Self::se("panel.opacity", "settings.panel.opacity", "settings.panel.opacity.desc", SettingsCategory::Panel, SettingsValue::Float(0.85), SettingsValue::Float(0.85), false, true, Some(0.0), Some(1.0), Some(0.05), false));
        self.register(Self::se("panel.show_clock", "settings.panel.show_clock", "settings.panel.show_clock.desc", SettingsCategory::Panel, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("panel.show_tray", "settings.panel.show_tray", "settings.panel.show_tray.desc", SettingsCategory::Panel, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));

        // Theme (6)
        self.register(Self::se("theme.variant", "settings.theme.variant", "settings.theme.variant.desc", SettingsCategory::Theme, SettingsValue::Choice("auto".into(), vec!["light".into(), "dark".into(), "auto".into()]), SettingsValue::Choice("auto".into(), vec!["light".into(), "dark".into(), "auto".into()]), false, true, None, None, None, false));
        self.register(Self::se("theme.accent", "settings.theme.accent", "settings.theme.accent.desc", SettingsCategory::Theme, SettingsValue::Color("#4A90D9".into()), SettingsValue::Color("#4A90D9".into()), false, true, None, None, None, false));
        self.register(Self::se("theme.contrast", "settings.theme.contrast", "settings.theme.contrast.desc", SettingsCategory::Theme, SettingsValue::Choice("default".into(), vec!["default".into(), "high".into()]), SettingsValue::Choice("default".into(), vec!["default".into(), "high".into()]), false, true, None, None, None, false));
        self.register(Self::se("theme.rounded", "settings.theme.rounded", "settings.theme.rounded.desc", SettingsCategory::Theme, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("theme.animations", "settings.theme.animations", "settings.theme.animations.desc", SettingsCategory::Theme, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("theme.wallpaper_blur", "settings.theme.wallpaper_blur", "settings.theme.wallpaper_blur.desc", SettingsCategory::Theme, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, true));

        // Wallpaper (4)
        self.register(Self::se("wallpaper.current", "settings.wallpaper.current", "settings.wallpaper.current.desc", SettingsCategory::Wallpaper, SettingsValue::String(String::new()), SettingsValue::String(String::new()), false, true, None, None, None, false));
        self.register(Self::se("wallpaper.fit", "settings.wallpaper.fit", "settings.wallpaper.fit.desc", SettingsCategory::Wallpaper, SettingsValue::Choice("fill".into(), vec!["fill".into(), "fit".into(), "stretch".into(), "center".into(), "tile".into()]), SettingsValue::Choice("fill".into(), vec!["fill".into(), "fit".into(), "stretch".into(), "center".into(), "tile".into()]), false, true, None, None, None, false));
        self.register(Self::se("wallpaper.color", "settings.wallpaper.color", "settings.wallpaper.color.desc", SettingsCategory::Wallpaper, SettingsValue::Color("#2d2d2d".into()), SettingsValue::Color("#2d2d2d".into()), false, true, None, None, None, false));
        self.register(Self::se("wallpaper.shuffle", "settings.wallpaper.shuffle", "settings.wallpaper.shuffle.desc", SettingsCategory::Wallpaper, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));

        // Font (6)
        self.register(Self::se("font.display", "settings.font.display", "settings.font.display.desc", SettingsCategory::Font, SettingsValue::String("Noto Sans".into()), SettingsValue::String("Noto Sans".into()), false, true, None, None, None, false));
        self.register(Self::se("font.mono", "settings.font.mono", "settings.font.mono.desc", SettingsCategory::Font, SettingsValue::String("Noto Sans Mono".into()), SettingsValue::String("Noto Sans Mono".into()), false, true, None, None, None, false));
        self.register(Self::se("font.size", "settings.font.size", "settings.font.size.desc", SettingsCategory::Font, SettingsValue::Float(10.0), SettingsValue::Float(10.0), false, true, Some(6.0), Some(24.0), Some(0.5), false));
        self.register(Self::se("font.scale", "settings.font.scale", "settings.font.scale.desc", SettingsCategory::Font, SettingsValue::Float(1.0), SettingsValue::Float(1.0), false, true, Some(0.5), Some(3.0), Some(0.1), false));
        self.register(Self::se("font.hinting", "settings.font.hinting", "settings.font.hinting.desc", SettingsCategory::Font, SettingsValue::Choice("full".into(), vec!["none".into(), "light".into(), "full".into()]), SettingsValue::Choice("full".into(), vec!["none".into(), "light".into(), "full".into()]), true, true, None, None, None, true));
        self.register(Self::se("font.antialiasing", "settings.font.antialiasing", "settings.font.antialiasing.desc", SettingsCategory::Font, SettingsValue::Choice("grayscale".into(), vec!["grayscale".into(), "subpixel".into()]), SettingsValue::Choice("grayscale".into(), vec!["grayscale".into(), "subpixel".into()]), true, true, None, None, None, true));

        // Animation (4)
        self.register(Self::se("anim.speed", "settings.anim.speed", "settings.anim.speed.desc", SettingsCategory::Animation, SettingsValue::Float(1.0), SettingsValue::Float(1.0), false, true, Some(0.1), Some(5.0), Some(0.1), false));
        self.register(Self::se("anim.reduce_motion", "settings.anim.reduce_motion", "settings.anim.reduce_motion.desc", SettingsCategory::Animation, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("anim.open_close", "settings.anim.open_close", "settings.anim.open_close.desc", SettingsCategory::Animation, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("anim.workspace_switch", "settings.anim.workspace_switch", "settings.anim.workspace_switch.desc", SettingsCategory::Animation, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));

        // Accessibility (8)
        self.register(Self::se("a11y.large_text", "settings.a11y.large_text", "settings.a11y.large_text.desc", SettingsCategory::Accessibility, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("a11y.high_contrast", "settings.a11y.high_contrast", "settings.a11y.high_contrast.desc", SettingsCategory::Accessibility, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("a11y.screen_reader", "settings.a11y.screen_reader", "settings.a11y.screen_reader.desc", SettingsCategory::Accessibility, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("a11y.sticky_keys", "settings.a11y.sticky_keys", "settings.a11y.sticky_keys.desc", SettingsCategory::Accessibility, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("a11y.bounce_keys", "settings.a11y.bounce_keys", "settings.a11y.bounce_keys.desc", SettingsCategory::Accessibility, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("a11y.mouse_keys", "settings.a11y.mouse_keys", "settings.a11y.mouse_keys.desc", SettingsCategory::Accessibility, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("a11y.slow_keys", "settings.a11y.slow_keys", "settings.a11y.slow_keys.desc", SettingsCategory::Accessibility, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("a11y.focus_ring", "settings.a11y.focus_ring", "settings.a11y.focus_ring.desc", SettingsCategory::Accessibility, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));

        // Language (4)
        self.register(Self::se("locale.language", "settings.locale.language", "settings.locale.language.desc", SettingsCategory::Language, SettingsValue::Choice("id".into(), vec!["id".into(), "en".into()]), SettingsValue::Choice("id".into(), vec!["id".into(), "en".into()]), false, true, None, None, None, false));
        self.register(Self::se("locale.measurement", "settings.locale.measurement", "settings.locale.measurement.desc", SettingsCategory::Language, SettingsValue::Choice("metric".into(), vec!["metric".into(), "imperial".into()]), SettingsValue::Choice("metric".into(), vec!["metric".into(), "imperial".into()]), false, true, None, None, None, false));
        self.register(Self::se("locale.first_day", "settings.locale.first_day", "settings.locale.first_day.desc", SettingsCategory::Language, SettingsValue::Choice("monday".into(), vec!["monday".into(), "sunday".into()]), SettingsValue::Choice("monday".into(), vec!["monday".into(), "sunday".into()]), false, true, None, None, None, false));
        self.register(Self::se("locale.time_format", "settings.locale.time_format", "settings.locale.time_format.desc", SettingsCategory::Language, SettingsValue::Choice("24h".into(), vec!["24h".into(), "12h".into()]), SettingsValue::Choice("24h".into(), vec!["24h".into(), "12h".into()]), false, true, None, None, None, false));

        // Keyboard (6)
        self.register(Self::se("kb.layout", "settings.kb.layout", "settings.kb.layout.desc", SettingsCategory::Keyboard, SettingsValue::String("us".into()), SettingsValue::String("us".into()), true, true, None, None, None, false));
        self.register(Self::se("kb.variant", "settings.kb.variant", "settings.kb.variant.desc", SettingsCategory::Keyboard, SettingsValue::String(String::new()), SettingsValue::String(String::new()), true, true, None, None, None, true));
        self.register(Self::se("kb.options", "settings.kb.options", "settings.kb.options.desc", SettingsCategory::Keyboard, SettingsValue::String(String::new()), SettingsValue::String(String::new()), true, true, None, None, None, true));
        self.register(Self::se("kb.repeat_rate", "settings.kb.repeat_rate", "settings.kb.repeat_rate.desc", SettingsCategory::Keyboard, SettingsValue::Int(30), SettingsValue::Int(30), true, false, Some(15.0), Some(60.0), Some(5.0), false));
        self.register(Self::se("kb.repeat_delay", "settings.kb.repeat_delay", "settings.kb.repeat_delay.desc", SettingsCategory::Keyboard, SettingsValue::Int(500), SettingsValue::Int(500), true, false, Some(200.0), Some(2000.0), Some(50.0), false));
        self.register(Self::se("kb.numlock", "settings.kb.numlock", "settings.kb.numlock.desc", SettingsCategory::Keyboard, SettingsValue::Bool(true), SettingsValue::Bool(true), false, false, None, None, None, false));

        // Mouse (6)
        self.register(Self::se("mouse.speed", "settings.mouse.speed", "settings.mouse.speed.desc", SettingsCategory::Mouse, SettingsValue::Float(1.0), SettingsValue::Float(1.0), false, true, Some(-5.0), Some(5.0), Some(0.1), false));
        self.register(Self::se("mouse.acceleration", "settings.mouse.acceleration", "settings.mouse.acceleration.desc", SettingsCategory::Mouse, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("mouse.natural_scroll", "settings.mouse.natural_scroll", "settings.mouse.natural_scroll.desc", SettingsCategory::Mouse, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("mouse.primary_button", "settings.mouse.primary_button", "settings.mouse.primary_button.desc", SettingsCategory::Mouse, SettingsValue::Choice("left".into(), vec!["left".into(), "right".into()]), SettingsValue::Choice("left".into(), vec!["left".into(), "right".into()]), false, true, None, None, None, false));
        self.register(Self::se("mouse.double_click", "settings.mouse.double_click", "settings.mouse.double_click.desc", SettingsCategory::Mouse, SettingsValue::Int(400), SettingsValue::Int(400), false, true, Some(100.0), Some(1000.0), Some(50.0), true));
        self.register(Self::se("mouse.scroll_speed", "settings.mouse.scroll_speed", "settings.mouse.scroll_speed.desc", SettingsCategory::Mouse, SettingsValue::Int(2), SettingsValue::Int(2), false, true, Some(1.0), Some(10.0), Some(1.0), true));

        // Touchpad (6)
        self.register(Self::se("touchpad.enabled", "settings.touchpad.enabled", "settings.touchpad.enabled.desc", SettingsCategory::Touchpad, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("touchpad.tap_to_click", "settings.touchpad.tap_to_click", "settings.touchpad.tap_to_click.desc", SettingsCategory::Touchpad, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("touchpad.two_finger_scroll", "settings.touchpad.two_finger_scroll", "settings.touchpad.two_finger_scroll.desc", SettingsCategory::Touchpad, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("touchpad.natural_scroll", "settings.touchpad.natural_scroll", "settings.touchpad.natural_scroll.desc", SettingsCategory::Touchpad, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("touchpad.pinch_zoom", "settings.touchpad.pinch_zoom", "settings.touchpad.pinch_zoom.desc", SettingsCategory::Touchpad, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("touchpad.disable_while_typing", "settings.touchpad.disable_while_typing", "settings.touchpad.disable_while_typing.desc", SettingsCategory::Touchpad, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));

        // Monitor (4)
        self.register(Self::se("monitor.brightness", "settings.monitor.brightness", "settings.monitor.brightness.desc", SettingsCategory::Monitor, SettingsValue::Int(80), SettingsValue::Int(80), false, true, Some(0.0), Some(100.0), Some(5.0), false));
        self.register(Self::se("monitor.night_light", "settings.monitor.night_light", "settings.monitor.night_light.desc", SettingsCategory::Monitor, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("monitor.night_temp", "settings.monitor.night_temp", "settings.monitor.night_temp.desc", SettingsCategory::Monitor, SettingsValue::Int(4000), SettingsValue::Int(4000), false, true, Some(1000.0), Some(8000.0), Some(100.0), true));
        self.register(Self::se("monitor.scaling", "settings.monitor.scaling", "settings.monitor.scaling.desc", SettingsCategory::Monitor, SettingsValue::Float(1.0), SettingsValue::Float(1.0), true, true, Some(0.5), Some(3.0), Some(0.25), false));

        // Power (6)
        self.register(Self::se("power.sleep_inactive", "settings.power.sleep_inactive", "settings.power.sleep_inactive.desc", SettingsCategory::Power, SettingsValue::Int(15), SettingsValue::Int(15), false, true, Some(0.0), Some(120.0), Some(5.0), false));
        self.register(Self::se("power.suspend_on_close", "settings.power.suspend_on_close", "settings.power.suspend_on_close.desc", SettingsCategory::Power, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("power.screen_off", "settings.power.screen_off", "settings.power.screen_off.desc", SettingsCategory::Power, SettingsValue::Int(5), SettingsValue::Int(5), false, true, Some(0.0), Some(60.0), Some(1.0), false));
        self.register(Self::se("power.auto_suspend", "settings.power.auto_suspend", "settings.power.auto_suspend.desc", SettingsCategory::Power, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("power.battery_low_warning", "settings.power.battery_low_warning", "settings.power.battery_low_warning.desc", SettingsCategory::Power, SettingsValue::Int(15), SettingsValue::Int(15), false, true, Some(5.0), Some(30.0), Some(5.0), false));
        self.register(Self::se("power.performance_mode", "settings.power.performance_mode", "settings.power.performance_mode.desc", SettingsCategory::Power, SettingsValue::Choice("balanced".into(), vec!["balanced".into(), "powersave".into(), "performance".into()]), SettingsValue::Choice("balanced".into(), vec!["balanced".into(), "powersave".into(), "performance".into()]), false, true, None, None, None, false));

        // Battery (4)
        self.register(Self::se("battery.show_percentage", "settings.battery.show_percentage", "settings.battery.show_percentage.desc", SettingsCategory::Battery, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("battery.show_time", "settings.battery.show_time", "settings.battery.show_time.desc", SettingsCategory::Battery, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("battery.low_threshold", "settings.battery.low_threshold", "settings.battery.low_threshold.desc", SettingsCategory::Battery, SettingsValue::Int(20), SettingsValue::Int(20), false, true, Some(5.0), Some(50.0), Some(5.0), false));
        self.register(Self::se("battery.critical_threshold", "settings.battery.critical_threshold", "settings.battery.critical_threshold.desc", SettingsCategory::Battery, SettingsValue::Int(5), SettingsValue::Int(5), false, true, Some(1.0), Some(20.0), Some(1.0), false));

        // Bluetooth (4)
        self.register(Self::se("bluetooth.enabled", "settings.bluetooth.enabled", "settings.bluetooth.enabled.desc", SettingsCategory::Bluetooth, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("bluetooth.discoverable", "settings.bluetooth.discoverable", "settings.bluetooth.discoverable.desc", SettingsCategory::Bluetooth, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("bluetooth.power_save", "settings.bluetooth.power_save", "settings.bluetooth.power_save.desc", SettingsCategory::Bluetooth, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("bluetooth.auto_connect", "settings.bluetooth.auto_connect", "settings.bluetooth.auto_connect.desc", SettingsCategory::Bluetooth, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));

        // Wi-Fi (4)
        self.register(Self::se("wifi.enabled", "settings.wifi.enabled", "settings.wifi.enabled.desc", SettingsCategory::Wifi, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("wifi.auto_connect", "settings.wifi.auto_connect", "settings.wifi.auto_connect.desc", SettingsCategory::Wifi, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("wifi.power_save", "settings.wifi.power_save", "settings.wifi.power_save.desc", SettingsCategory::Wifi, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("wifi.hotspot", "settings.wifi.hotspot", "settings.wifi.hotspot.desc", SettingsCategory::Wifi, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));

        // Privacy (6)
        self.register(Self::se("privacy.screenshots", "settings.privacy.screenshots", "settings.privacy.screenshots.desc", SettingsCategory::Privacy, SettingsValue::Bool(true), SettingsValue::Bool(true), false, false, None, None, None, false));
        self.register(Self::se("privacy.usage_history", "settings.privacy.usage_history", "settings.privacy.usage_history.desc", SettingsCategory::Privacy, SettingsValue::Bool(true), SettingsValue::Bool(true), false, false, None, None, None, false));
        self.register(Self::se("privacy.search_history", "settings.privacy.search_history", "settings.privacy.search_history.desc", SettingsCategory::Privacy, SettingsValue::Bool(true), SettingsValue::Bool(true), false, false, None, None, None, false));
        self.register(Self::se("privacy.location", "settings.privacy.location", "settings.privacy.location.desc", SettingsCategory::Privacy, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("privacy.crash_reports", "settings.privacy.crash_reports", "settings.privacy.crash_reports.desc", SettingsCategory::Privacy, SettingsValue::Bool(false), SettingsValue::Bool(false), false, false, None, None, None, true));
        self.register(Self::se("privacy.diagnostics", "settings.privacy.diagnostics", "settings.privacy.diagnostics.desc", SettingsCategory::Privacy, SettingsValue::Bool(false), SettingsValue::Bool(false), false, false, None, None, None, true));

        // Notification (4)
        self.register(Self::se("notif.do_not_disturb", "settings.notif.do_not_disturb", "settings.notif.do_not_disturb.desc", SettingsCategory::Notification, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("notif.show_banners", "settings.notif.show_banners", "settings.notif.show_banners.desc", SettingsCategory::Notification, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("notif.show_in_lock", "settings.notif.show_in_lock", "settings.notif.show_in_lock.desc", SettingsCategory::Notification, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("notif.sound", "settings.notif.sound", "settings.notif.sound.desc", SettingsCategory::Notification, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));

        // Update (4)
        self.register(Self::se("update.auto_check", "settings.update.auto_check", "settings.update.auto_check.desc", SettingsCategory::Update, SettingsValue::Bool(true), SettingsValue::Bool(true), false, false, None, None, None, false));
        self.register(Self::se("update.auto_download", "settings.update.auto_download", "settings.update.auto_download.desc", SettingsCategory::Update, SettingsValue::Bool(false), SettingsValue::Bool(false), false, false, None, None, None, false));
        self.register(Self::se("update.channel", "settings.update.channel", "settings.update.channel.desc", SettingsCategory::Update, SettingsValue::Choice("stable".into(), vec!["stable".into(), "beta".into(), "dev".into()]), SettingsValue::Choice("stable".into(), vec!["stable".into(), "beta".into(), "dev".into()]), false, true, None, None, None, false));
        self.register(Self::se("update.interval", "settings.update.interval", "settings.update.interval.desc", SettingsCategory::Update, SettingsValue::Int(24), SettingsValue::Int(24), false, false, Some(1.0), Some(168.0), Some(1.0), false));

        // About (4)
        self.register(Self::se("about.version", "settings.about.version", "settings.about.version.desc", SettingsCategory::About, SettingsValue::String("1.0.0-alpha.1".into()), SettingsValue::String("1.0.0-alpha.1".into()), false, false, None, None, None, false));
        self.register(Self::se("about.os", "settings.about.os", "settings.about.os.desc", SettingsCategory::About, SettingsValue::String("Linux".into()), SettingsValue::String("Linux".into()), false, false, None, None, None, false));
        self.register(Self::se("about.desktop", "settings.about.desktop", "settings.about.desktop.desc", SettingsCategory::About, SettingsValue::String("EduShell".into()), SettingsValue::String("EduShell".into()), false, false, None, None, None, false));
        self.register(Self::se("about.license", "settings.about.license", "settings.about.license.desc", SettingsCategory::About, SettingsValue::String("GPL-3.0-or-later".into()), SettingsValue::String("GPL-3.0-or-later".into()), false, false, None, None, None, false));

        // Learning (4)
        self.register(Self::se("learning.recommendations", "settings.learning.recommendations", "settings.learning.recommendations.desc", SettingsCategory::Learning, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("learning.reminders", "settings.learning.reminders", "settings.learning.reminders.desc", SettingsCategory::Learning, SettingsValue::Bool(false), SettingsValue::Bool(false), false, true, None, None, None, false));
        self.register(Self::se("learning.notifications", "settings.learning.notifications", "settings.learning.notifications.desc", SettingsCategory::Learning, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
        self.register(Self::se("learning.streak_tracking", "settings.learning.streak_tracking", "settings.learning.streak_tracking.desc", SettingsCategory::Learning, SettingsValue::Bool(true), SettingsValue::Bool(true), false, true, None, None, None, false));
    }

    fn build_search_index(&mut self) {
        self.search_index.clear();
        for entry in self.entries.values() {
            let words: Vec<String> = vec![
                entry.key.clone(),
                entry.name_key.clone(),
                entry.description_key.clone(),
                format!("{:?}", entry.category),
            ];
            for w in &words {
                let lower = w.to_lowercase();
                self.search_index
                    .entry(lower)
                    .or_default()
                    .push(entry.key.clone());
            }
            // index individual tokens
            for w in &words {
                for token in w.split(&['.', '_', ' ', '-'][..]) {
                    if !token.is_empty() {
                        let t = token.to_lowercase();
                        self.search_index
                            .entry(t)
                            .or_default()
                            .push(entry.key.clone());
                    }
                }
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&SettingsEntry> {
        self.entries.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut SettingsEntry> {
        self.entries.get_mut(key)
    }

    pub fn set(&mut self, key: &str, value: SettingsValue) -> Result<()> {
        let entry = self.entries.get_mut(key).ok_or_else(|| SettingsError::NotFound(key.to_string()))?;
        if std::mem::discriminant(&entry.value) != std::mem::discriminant(&value) {
            return Err(SettingsError::TypeMismatch(key.to_string()));
        }
        // Check bounds for numeric types
        if let (Some(min), Some(max)) = (entry.min, entry.max) {
            match &value {
                SettingsValue::Int(v) => {
                    let fv = *v as f64;
                    if fv < min || fv > max {
                        return Err(SettingsError::OutOfBounds(key.to_string()));
                    }
                }
                SettingsValue::Float(v) => {
                    if *v < min || *v > max {
                        return Err(SettingsError::OutOfBounds(key.to_string()));
                    }
                }
                _ => {}
            }
        }
        entry.value = value;
        if !self.changed_entries.contains(&key.to_string()) {
            self.changed_entries.push(key.to_string());
        }
        Ok(())
    }

    pub fn set_string(&mut self, key: &str, value: &str) -> Result<()> {
        self.set(key, SettingsValue::String(value.to_string()))
    }

    pub fn set_bool(&mut self, key: &str, value: bool) -> Result<()> {
        self.set(key, SettingsValue::Bool(value))
    }

    pub fn set_int(&mut self, key: &str, value: i64) -> Result<()> {
        self.set(key, SettingsValue::Int(value))
    }

    pub fn reset(&mut self, key: &str) -> Result<()> {
        let entry = self.entries.get_mut(key).ok_or_else(|| SettingsError::NotFound(key.to_string()))?;
        if !entry.is_default() {
            entry.reset();
            if !self.changed_entries.contains(&key.to_string()) {
                self.changed_entries.push(key.to_string());
            }
        }
        Ok(())
    }

    pub fn reset_category(&mut self, cat: SettingsCategory) -> Result<()> {
        let keys: Vec<String> = self.entries.values()
            .filter(|e| e.category == cat)
            .map(|e| e.key.clone())
            .collect();
        for k in keys {
            self.reset(&k)?;
        }
        Ok(())
    }

    pub fn reset_all(&mut self) -> Result<()> {
        let keys: Vec<String> = self.entries.keys().cloned().collect();
        for k in keys {
            self.reset(&k)?;
        }
        Ok(())
    }

    pub fn entries_by_category(&self, cat: SettingsCategory) -> Vec<&SettingsEntry> {
        self.entries.values().filter(|e| e.category == cat).collect()
    }

    pub fn entries_by_categories(&self, cats: &[SettingsCategory]) -> Vec<&SettingsEntry> {
        self.entries.values().filter(|e| cats.contains(&e.category)).collect()
    }

    pub fn all_entries(&self) -> Vec<&SettingsEntry> {
        self.entries.values().collect()
    }

    pub fn search(&self, query: &str) -> Vec<&SettingsEntry> {
        if query.is_empty() {
            return self.all_entries();
        }
        let q = query.to_lowercase();
        let mut matched_keys: Vec<String> = vec![];

        // direct key match
        if self.entries.contains_key(&q) {
            matched_keys.push(q.clone());
        }

        // linear scan (primary)
        let mut keys_from_scan: Vec<String> = vec![];
        for entry in self.entries.values() {
            if entry.key.to_lowercase().contains(&q)
                || entry.name_key.to_lowercase().contains(&q)
                || entry.description_key.to_lowercase().contains(&q)
                || format!("{:?}", entry.category).to_lowercase().contains(&q)
            {
                if !keys_from_scan.contains(&entry.key) {
                    keys_from_scan.push(entry.key.clone());
                }
            }
        }

        // search index to catch word-boundary matches
        for (word, keys) in &self.search_index {
            if word.contains(&q) {
                for k in keys {
                    if !matched_keys.contains(k) && !keys_from_scan.contains(k) {
                        keys_from_scan.push(k.clone());
                    }
                }
            }
        }

        for k in keys_from_scan {
            if !matched_keys.contains(&k) {
                matched_keys.push(k);
            }
        }

        matched_keys.iter()
            .filter_map(|k| self.entries.get(k))
            .collect()
    }

    pub fn changed_entries(&self) -> &[String] {
        &self.changed_entries
    }

    pub fn has_changes(&self) -> bool {
        !self.changed_entries.is_empty()
    }

    pub fn requires_restart(&self) -> Vec<&SettingsEntry> {
        self.entries.values().filter(|e| e.requires_restart).collect()
    }

    pub fn requires_reload(&self) -> Vec<&SettingsEntry> {
        self.entries.values().filter(|e| e.requires_reload).collect()
    }

    pub fn category_name(&self, cat: SettingsCategory) -> String {
        let key = match cat {
            SettingsCategory::Appearance => "settings.appearance",
            SettingsCategory::Dock => "settings.dock",
            SettingsCategory::Panel => "settings.panel",
            SettingsCategory::Theme => "settings.theme",
            SettingsCategory::Wallpaper => "settings.wallpaper",
            SettingsCategory::Font => "settings.font",
            SettingsCategory::Animation => "settings.animation",
            SettingsCategory::Accessibility => "settings.accessibility",
            SettingsCategory::Language => "settings.language",
            SettingsCategory::Keyboard => "settings.keyboard",
            SettingsCategory::Mouse => "settings.mouse",
            SettingsCategory::Touchpad => "settings.touchpad",
            SettingsCategory::Monitor => "settings.monitor",
            SettingsCategory::Power => "settings.power",
            SettingsCategory::Battery => "settings.battery",
            SettingsCategory::Bluetooth => "settings.bluetooth",
            SettingsCategory::Wifi => "settings.wifi",
            SettingsCategory::Privacy => "settings.privacy",
            SettingsCategory::Notification => "settings.notification",
            SettingsCategory::Update => "settings.update",
            SettingsCategory::About => "settings.about",
            SettingsCategory::Learning => "settings.learning",
        };
        self.localization.get(key).to_string()
    }

    pub fn category_icon(cat: SettingsCategory) -> &'static str {
        match cat {
            SettingsCategory::Appearance => "edushell-appearance",
            SettingsCategory::Dock => "edushell-dock",
            SettingsCategory::Panel => "edushell-panel",
            SettingsCategory::Theme => "edushell-theme",
            SettingsCategory::Wallpaper => "edushell-wallpaper",
            SettingsCategory::Font => "edushell-font",
            SettingsCategory::Animation => "edushell-animation",
            SettingsCategory::Accessibility => "edushell-accessibility",
            SettingsCategory::Language => "edushell-language",
            SettingsCategory::Keyboard => "edushell-keyboard",
            SettingsCategory::Mouse => "edushell-mouse",
            SettingsCategory::Touchpad => "edushell-touchpad",
            SettingsCategory::Monitor => "edushell-monitor",
            SettingsCategory::Power => "edushell-power",
            SettingsCategory::Battery => "edushell-battery",
            SettingsCategory::Bluetooth => "edushell-bluetooth",
            SettingsCategory::Wifi => "edushell-wifi",
            SettingsCategory::Privacy => "edushell-privacy",
            SettingsCategory::Notification => "edushell-notification",
            SettingsCategory::Update => "edushell-update",
            SettingsCategory::About => "edushell-about",
            SettingsCategory::Learning => "edushell-learning",
        }
    }

    pub fn category_description(&self, cat: SettingsCategory) -> String {
        let key = match cat {
            SettingsCategory::Appearance => "settings.appearance.desc",
            SettingsCategory::Dock => "settings.dock.desc",
            SettingsCategory::Panel => "settings.panel.desc",
            SettingsCategory::Theme => "settings.theme.desc",
            SettingsCategory::Wallpaper => "settings.wallpaper.desc",
            SettingsCategory::Font => "settings.font.desc",
            SettingsCategory::Animation => "settings.animation.desc",
            SettingsCategory::Accessibility => "settings.accessibility.desc",
            SettingsCategory::Language => "settings.language.desc",
            SettingsCategory::Keyboard => "settings.keyboard.desc",
            SettingsCategory::Mouse => "settings.mouse.desc",
            SettingsCategory::Touchpad => "settings.touchpad.desc",
            SettingsCategory::Monitor => "settings.monitor.desc",
            SettingsCategory::Power => "settings.power.desc",
            SettingsCategory::Battery => "settings.battery.desc",
            SettingsCategory::Bluetooth => "settings.bluetooth.desc",
            SettingsCategory::Wifi => "settings.wifi.desc",
            SettingsCategory::Privacy => "settings.privacy.desc",
            SettingsCategory::Notification => "settings.notification.desc",
            SettingsCategory::Update => "settings.update.desc",
            SettingsCategory::About => "settings.about.desc",
            SettingsCategory::Learning => "settings.learning.desc",
        };
        self.localization.get(key).to_string()
    }

    pub fn categories(&self) -> &[SettingsCategory] {
        &self.categories
    }

    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    pub fn save(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        let path = self.config_dir.join("settings.json");
        let data = SettingsData {
            entries: self.entries.clone(),
        };
        let json = serde_json::to_string_pretty(&data)?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let path = self.config_dir.join("settings.json");
        if !path.exists() {
            return Ok(());
        }
        let json = std::fs::read_to_string(&path)?;
        let data: SettingsData = serde_json::from_str(&json)?;
        // Merge loaded entries into current ones, preserving types
        for (key, loaded_entry) in data.entries {
            if let Some(existing) = self.entries.get_mut(&key) {
                if std::mem::discriminant(&existing.value) == std::mem::discriminant(&loaded_entry.value) {
                    existing.value = loaded_entry.value;
                }
            }
        }
        self.changed_entries.clear();
        self.build_search_index();
        Ok(())
    }

    pub fn export_to_json(&self) -> String {
        let data = SettingsData {
            entries: self.entries.clone(),
        };
        serde_json::to_string_pretty(&data).unwrap_or_default()
    }

    pub fn import_from_json(&mut self, json: &str) -> Result<()> {
        let data: SettingsData = serde_json::from_str(json)?;
        for (key, loaded_entry) in data.entries {
            if let Some(existing) = self.entries.get_mut(&key) {
                if std::mem::discriminant(&existing.value) == std::mem::discriminant(&loaded_entry.value) {
                    existing.value = loaded_entry.value;
                    if !self.changed_entries.contains(&key) {
                        self.changed_entries.push(key);
                    }
                }
            }
        }
        self.build_search_index();
        Ok(())
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = match locale {
            "id-ID" | "id" => Lang::Indonesian,
            _ => Lang::English,
        };
        self.localization.set_language(lang);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_config_dir() -> PathBuf {
        let n = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("edushell_test_settings_{}", n));
        let _ = std::fs::create_dir_all(&dir);
        dir
    }

    fn new_center() -> SettingsCenter {
        SettingsCenter::new(Some(temp_config_dir()))
    }

    // --- Creation & Category Tests ---

    #[test]
    fn test_new_creates_all_entries() {
        let sc = new_center();
        assert!(sc.entries.len() >= 80, "expected >=80 entries, got {}", sc.entries.len());
    }

    #[test]
    fn test_all_22_categories_present() {
        let sc = new_center();
        assert_eq!(sc.category_count(), 22);
        let cats = sc.categories();
        assert!(cats.contains(&SettingsCategory::Appearance));
        assert!(cats.contains(&SettingsCategory::Dock));
        assert!(cats.contains(&SettingsCategory::Panel));
        assert!(cats.contains(&SettingsCategory::Theme));
        assert!(cats.contains(&SettingsCategory::Wallpaper));
        assert!(cats.contains(&SettingsCategory::Font));
        assert!(cats.contains(&SettingsCategory::Animation));
        assert!(cats.contains(&SettingsCategory::Accessibility));
        assert!(cats.contains(&SettingsCategory::Language));
        assert!(cats.contains(&SettingsCategory::Keyboard));
        assert!(cats.contains(&SettingsCategory::Mouse));
        assert!(cats.contains(&SettingsCategory::Touchpad));
        assert!(cats.contains(&SettingsCategory::Monitor));
        assert!(cats.contains(&SettingsCategory::Power));
        assert!(cats.contains(&SettingsCategory::Battery));
        assert!(cats.contains(&SettingsCategory::Bluetooth));
        assert!(cats.contains(&SettingsCategory::Wifi));
        assert!(cats.contains(&SettingsCategory::Privacy));
        assert!(cats.contains(&SettingsCategory::Notification));
        assert!(cats.contains(&SettingsCategory::Update));
        assert!(cats.contains(&SettingsCategory::About));
        assert!(cats.contains(&SettingsCategory::Learning));
    }

    #[test]
    fn test_each_category_has_entries() {
        let sc = new_center();
        for cat in sc.categories() {
            let entries = sc.entries_by_category(*cat);
            assert!(!entries.is_empty(), "category {:?} has no entries", cat);
        }
    }

    #[test]
    fn test_appearance_has_12_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Appearance).len(), 12);
    }

    #[test]
    fn test_dock_has_8_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Dock).len(), 8);
    }

    #[test]
    fn test_panel_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Panel).len(), 6);
    }

    #[test]
    fn test_theme_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Theme).len(), 6);
    }

    #[test]
    fn test_wallpaper_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Wallpaper).len(), 4);
    }

    #[test]
    fn test_font_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Font).len(), 6);
    }

    #[test]
    fn test_animation_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Animation).len(), 4);
    }

    #[test]
    fn test_accessibility_has_8_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Accessibility).len(), 8);
    }

    #[test]
    fn test_language_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Language).len(), 4);
    }

    #[test]
    fn test_keyboard_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Keyboard).len(), 6);
    }

    #[test]
    fn test_mouse_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Mouse).len(), 6);
    }

    #[test]
    fn test_touchpad_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Touchpad).len(), 6);
    }

    #[test]
    fn test_monitor_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Monitor).len(), 4);
    }

    #[test]
    fn test_power_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Power).len(), 6);
    }

    #[test]
    fn test_battery_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Battery).len(), 4);
    }

    #[test]
    fn test_bluetooth_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Bluetooth).len(), 4);
    }

    #[test]
    fn test_wifi_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Wifi).len(), 4);
    }

    #[test]
    fn test_privacy_has_6_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Privacy).len(), 6);
    }

    #[test]
    fn test_notification_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Notification).len(), 4);
    }

    #[test]
    fn test_update_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Update).len(), 4);
    }

    #[test]
    fn test_about_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::About).len(), 4);
    }

    #[test]
    fn test_learning_has_4_entries() {
        let sc = new_center();
        assert_eq!(sc.entries_by_category(SettingsCategory::Learning).len(), 4);
    }

    // --- get / set / reset ---

    #[test]
    fn test_get_existing() {
        let sc = new_center();
        let entry = sc.get("dock.position").unwrap();
        assert_eq!(entry.key, "dock.position");
        assert_eq!(entry.category, SettingsCategory::Dock);
    }

    #[test]
    fn test_get_nonexistent() {
        let sc = new_center();
        assert!(sc.get("nonexistent.setting").is_none());
    }

    #[test]
    fn test_set_bool() {
        let mut sc = new_center();
        assert!(sc.get("dock.auto_hide").unwrap().value.as_bool() == Some(false));
        sc.set_bool("dock.auto_hide", true).unwrap();
        assert_eq!(sc.get("dock.auto_hide").unwrap().value.as_bool(), Some(true));
    }

    #[test]
    fn test_set_int() {
        let mut sc = new_center();
        sc.set_int("dock.icon_size", 64).unwrap();
        assert_eq!(sc.get("dock.icon_size").unwrap().value.as_int(), Some(64));
    }

    #[test]
    fn test_set_string() {
        let mut sc = new_center();
        sc.set_string("font.display", "Liberation Sans").unwrap();
        assert_eq!(sc.get("font.display").unwrap().value.as_string(), Some("Liberation Sans"));
    }

    #[test]
    fn test_set_type_mismatch_error() {
        let mut sc = new_center();
        let result = sc.set("dock.position", SettingsValue::Bool(true));
        assert!(result.is_err());
        match result {
            Err(SettingsError::TypeMismatch(_)) => {}
            _ => panic!("expected TypeMismatch error"),
        }
    }

    #[test]
    fn test_set_nonexistent_error() {
        let mut sc = new_center();
        let result = sc.set("nope.nope", SettingsValue::Bool(true));
        assert!(result.is_err());
        match result {
            Err(SettingsError::NotFound(_)) => {}
            _ => panic!("expected NotFound error"),
        }
    }

    #[test]
    fn test_set_int_out_of_bounds_high() {
        let mut sc = new_center();
        let result = sc.set_int("monitor.brightness", 200);
        assert!(result.is_err());
        match result {
            Err(SettingsError::OutOfBounds(_)) => {}
            _ => panic!("expected OutOfBounds error"),
        }
    }

    #[test]
    fn test_set_int_out_of_bounds_low() {
        let mut sc = new_center();
        let result = sc.set_int("monitor.brightness", -10);
        assert!(result.is_err());
        match result {
            Err(SettingsError::OutOfBounds(_)) => {}
            _ => panic!("expected OutOfBounds error"),
        }
    }

    #[test]
    fn test_set_float_out_of_bounds() {
        let mut sc = new_center();
        let result = sc.set("mouse.speed", SettingsValue::Float(10.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_set_float_at_boundary() {
        let mut sc = new_center();
        sc.set("mouse.speed", SettingsValue::Float(5.0)).unwrap();
        assert!((sc.get("mouse.speed").unwrap().value.as_float().unwrap() - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_reset_individual() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        assert_eq!(sc.get("dock.auto_hide").unwrap().value.as_bool(), Some(true));
        sc.reset("dock.auto_hide").unwrap();
        assert_eq!(sc.get("dock.auto_hide").unwrap().value.as_bool(), Some(false));
    }

    #[test]
    fn test_reset_nonexistent_error() {
        let mut sc = new_center();
        let result = sc.reset("nope");
        assert!(result.is_err());
    }

    #[test]
    fn test_reset_category() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        sc.set_int("dock.icon_size", 64).unwrap();
        sc.reset_category(SettingsCategory::Dock).unwrap();
        let entries = sc.entries_by_category(SettingsCategory::Dock);
        for e in &entries {
            assert!(e.is_default(), "entry {} not default after category reset", e.key);
        }
    }

    #[test]
    fn test_reset_all() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        sc.set_bool("anim.reduce_motion", true).unwrap();
        sc.set_string("font.display", "Custom").unwrap();
        sc.reset_all().unwrap();
        for e in sc.entries.values() {
            assert!(e.is_default(), "entry {} not default after reset_all", e.key);
        }
    }

    // --- Entries by category ---

    #[test]
    fn test_entries_by_categories_multiple() {
        let sc = new_center();
        let result = sc.entries_by_categories(&[SettingsCategory::Appearance, SettingsCategory::Dock]);
        let total = result.len();
        assert_eq!(total, 12 + 8);
    }

    #[test]
    fn test_all_entries_count() {
        let sc = new_center();
        let all = sc.all_entries();
        assert_eq!(all.len(), sc.entries.len());
    }

    // --- Category utility methods ---

    #[test]
    fn test_category_name_returns_string() {
        let sc = new_center();
        let name = sc.category_name(SettingsCategory::Appearance);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_category_icon_returns_static_str() {
        let icon = SettingsCenter::category_icon(SettingsCategory::Appearance);
        assert!(!icon.is_empty());
    }

    #[test]
    fn test_category_description_returns_string() {
        let sc = new_center();
        let desc = sc.category_description(SettingsCategory::Appearance);
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_category_icon_all_unique() {
        let icons: Vec<&str> = vec![
            SettingsCenter::category_icon(SettingsCategory::Appearance),
            SettingsCenter::category_icon(SettingsCategory::Dock),
            SettingsCenter::category_icon(SettingsCategory::Panel),
            SettingsCenter::category_icon(SettingsCategory::Theme),
            SettingsCenter::category_icon(SettingsCategory::Wallpaper),
            SettingsCenter::category_icon(SettingsCategory::Font),
            SettingsCenter::category_icon(SettingsCategory::Animation),
            SettingsCenter::category_icon(SettingsCategory::Accessibility),
            SettingsCenter::category_icon(SettingsCategory::Language),
            SettingsCenter::category_icon(SettingsCategory::Keyboard),
            SettingsCenter::category_icon(SettingsCategory::Mouse),
            SettingsCenter::category_icon(SettingsCategory::Touchpad),
            SettingsCenter::category_icon(SettingsCategory::Monitor),
            SettingsCenter::category_icon(SettingsCategory::Power),
            SettingsCenter::category_icon(SettingsCategory::Battery),
            SettingsCenter::category_icon(SettingsCategory::Bluetooth),
            SettingsCenter::category_icon(SettingsCategory::Wifi),
            SettingsCenter::category_icon(SettingsCategory::Privacy),
            SettingsCenter::category_icon(SettingsCategory::Notification),
            SettingsCenter::category_icon(SettingsCategory::Update),
            SettingsCenter::category_icon(SettingsCategory::About),
            SettingsCenter::category_icon(SettingsCategory::Learning),
        ];
        let mut uniq = icons.clone();
        uniq.sort();
        uniq.dedup();
        assert_eq!(uniq.len(), icons.len(), "category icons are not unique");
    }

    // --- Search ---

    #[test]
    fn test_search_by_key() {
        let sc = new_center();
        let results = sc.search("dock.position");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.key == "dock.position"));
    }

    #[test]
    fn test_search_by_name_key() {
        let sc = new_center();
        let results = sc.search("appearance.accent_color");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_by_category() {
        let sc = new_center();
        let results = sc.search("dock");
        assert!(results.len() >= 8);
    }

    #[test]
    fn test_search_empty_query_returns_all() {
        let sc = new_center();
        let results = sc.search("");
        assert_eq!(results.len(), sc.entries.len());
    }

    #[test]
    fn test_search_partial_key() {
        let sc = new_center();
        let results = sc.search("accent");
        assert!(results.iter().any(|e| e.key.contains("accent")));
    }

    #[test]
    fn test_search_no_match() {
        let sc = new_center();
        let results = sc.search("zzzznonexistentzzzz");
        assert!(results.is_empty());
    }

    // --- Changed entries ---

    #[test]
    fn test_changed_entries_tracking() {
        let mut sc = new_center();
        assert!(!sc.has_changes());
        assert!(sc.changed_entries().is_empty());
        sc.set_bool("dock.auto_hide", true).unwrap();
        assert!(sc.has_changes());
        assert_eq!(sc.changed_entries().len(), 1);
        assert_eq!(sc.changed_entries()[0], "dock.auto_hide");
    }

    #[test]
    fn test_changed_entries_after_reset() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        assert_eq!(sc.changed_entries().len(), 1);
        sc.reset("dock.auto_hide").unwrap();
        // resetting keeps it tracked (already in changed list)
        assert_eq!(sc.changed_entries().len(), 1);
    }

    #[test]
    fn test_changed_entries_deduplicates() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        sc.set_bool("dock.auto_hide", false).unwrap();
        assert_eq!(sc.changed_entries().len(), 1);
    }

    // --- requires_restart / requires_reload ---

    #[test]
    fn test_requires_restart() {
        let sc = new_center();
        let restart = sc.requires_restart();
        assert!(!restart.is_empty());
        assert!(restart.iter().any(|e| e.key == "dock.position"));
    }

    #[test]
    fn test_requires_reload() {
        let sc = new_center();
        let reload = sc.requires_reload();
        assert!(!reload.is_empty());
        assert!(reload.iter().any(|e| e.key == "appearance.theme_mode"));
    }

    // --- Save / Load round-trip ---

    #[test]
    fn test_save_creates_file() {
        let dir = temp_config_dir();
        let mut sc = SettingsCenter::new(Some(dir.clone()));
        sc.set_bool("dock.auto_hide", true).unwrap();
        sc.save().unwrap();
        let path = dir.join("settings.json");
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("dock.auto_hide"));
    }

    #[test]
    fn test_save_load_round_trip() {
        let dir = temp_config_dir();
        let mut sc1 = SettingsCenter::new(Some(dir.clone()));
        sc1.set_bool("dock.auto_hide", true).unwrap();
        sc1.set_int("dock.icon_size", 72).unwrap();
        sc1.set_string("font.display", "Test Font").unwrap();
        sc1.save().unwrap();

        let mut sc2 = SettingsCenter::new(Some(dir.clone()));
        sc2.load().unwrap();

        assert_eq!(sc2.get("dock.auto_hide").unwrap().value.as_bool(), Some(true));
        assert_eq!(sc2.get("dock.icon_size").unwrap().value.as_int(), Some(72));
        assert_eq!(sc2.get("font.display").unwrap().value.as_string(), Some("Test Font"));
    }

    #[test]
    fn test_load_nonexistent_file() {
        let dir = temp_config_dir();
        let mut sc = SettingsCenter::new(Some(dir));
        sc.load().unwrap(); // should not error
    }

    #[test]
    fn test_load_ignores_type_mismatch() {
        let dir = temp_config_dir();
        let sc1 = SettingsCenter::new(Some(dir.clone()));
        sc1.save().unwrap();

        // Manually corrupt the file to change type
        let path = dir.join("settings.json");
        let content = std::fs::read_to_string(&path).unwrap();
        let corrupted = content.replace("\"Bool\": true", "\"String\": \"oops\"");
        std::fs::write(&path, corrupted).unwrap();

        let mut sc2 = SettingsCenter::new(Some(dir));
        sc2.load().unwrap();
        // The entry should retain its original type/value
        assert!(sc2.get("dock.auto_hide").unwrap().value.as_bool().is_some());
    }

    // --- Export / Import JSON ---

    #[test]
    fn test_export_to_json() {
        let sc = new_center();
        let json = sc.export_to_json();
        assert!(!json.is_empty());
        assert!(json.contains("dock.auto_hide"));
    }

    #[test]
    fn test_import_from_json() {
        let mut sc1 = new_center();
        sc1.set_bool("dock.auto_hide", true).unwrap();
        let json = sc1.export_to_json();
        assert!(json.contains("\"Bool\": true") || json.contains("\"Bool\":true"));

        let mut sc2 = new_center();
        assert_eq!(sc2.get("dock.auto_hide").unwrap().value.as_bool(), Some(false));
        sc2.import_from_json(&json).unwrap();
        assert_eq!(sc2.get("dock.auto_hide").unwrap().value.as_bool(), Some(true));
    }

    #[test]
    fn test_import_invalid_json() {
        let mut sc = new_center();
        let result = sc.import_from_json("not valid json");
        assert!(result.is_err());
    }

    // --- Locale ---

    #[test]
    fn test_set_locale_id() {
        let mut sc = new_center();
        sc.set_locale("id-ID");
        // just verify no crash; the name will be in Indonesian
        let name = sc.category_name(SettingsCategory::Appearance);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_set_locale_en() {
        let mut sc = new_center();
        sc.set_locale("en-US");
        let name = sc.category_name(SettingsCategory::Appearance);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_set_locale_id_short() {
        let mut sc = new_center();
        sc.set_locale("id");
        let name = sc.category_name(SettingsCategory::Appearance);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_set_locale_unknown_falls_back_to_english() {
        let mut sc = new_center();
        sc.set_locale("jp-JP");
        let name = sc.category_name(SettingsCategory::Appearance);
        assert!(!name.is_empty());
    }

    // --- Advanced filtering ---

    #[test]
    fn test_advanced_entries_marked() {
        let sc = new_center();
        let advanced: Vec<&SettingsEntry> = sc.all_entries().into_iter().filter(|e| e.advanced).collect();
        assert!(!advanced.is_empty());
        assert!(advanced.iter().any(|e| e.key == "kb.variant"));
        assert!(advanced.iter().any(|e| e.key == "kb.options"));
    }

    // --- Edge cases ---

    #[test]
    fn test_get_mut_and_modify() {
        let mut sc = new_center();
        {
            let entry = sc.get_mut("dock.position").unwrap();
            entry.value = SettingsValue::Choice("left".into(), vec!["bottom".into(), "left".into(), "right".into()]);
        }
        let val = sc.get("dock.position").unwrap();
        let (selected, _) = val.value.as_choice().unwrap();
        assert_eq!(selected, "left");
    }

    #[test]
    fn test_get_mut_nonexistent() {
        let mut sc = new_center();
        assert!(sc.get_mut("nope").is_none());
    }

    #[test]
    fn test_settings_value_as_methods() {
        let s = SettingsValue::String("hello".into());
        assert_eq!(s.as_string(), Some("hello"));
        assert!(s.as_bool().is_none());
        assert!(s.as_int().is_none());

        let b = SettingsValue::Bool(true);
        assert_eq!(b.as_bool(), Some(true));

        let i = SettingsValue::Int(42);
        assert_eq!(i.as_int(), Some(42));

        let f = SettingsValue::Float(3.14);
        assert!(f.as_float().is_some());

        let c = SettingsValue::Color("#fff".into());
        assert_eq!(c.as_string(), Some("#fff"));

        let p = SettingsValue::Path("/tmp".into());
        assert_eq!(p.as_string(), Some("/tmp"));

        let ch = SettingsValue::Choice("a".into(), vec!["a".into(), "b".into()]);
        let (sel, opts) = ch.as_choice().unwrap();
        assert_eq!(sel, "a");
        assert_eq!(opts.len(), 2);

        let l = SettingsValue::List(vec!["x".into(), "y".into()]);
        assert_eq!(l.as_list().unwrap().len(), 2);
    }

    #[test]
    fn test_float_as_int_conversion() {
        let i = SettingsValue::Int(10);
        assert!((i.as_float().unwrap() - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_is_default_true_on_creation() {
        let sc = new_center();
        for e in sc.entries.values() {
            assert!(e.is_default(), "entry {} should be default on creation", e.key);
        }
    }

    #[test]
    fn test_is_default_after_set() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        assert!(!sc.get("dock.auto_hide").unwrap().is_default());
    }

    #[test]
    fn test_display_name_and_description() {
        let sc = new_center();
        let entry = sc.get("appearance.theme_mode").unwrap();
        let name = entry.display_name(&sc.localization);
        let desc = entry.display_description(&sc.localization);
        assert!(!name.is_empty());
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_entries_by_categories_empty_slice() {
        let sc = new_center();
        let result = sc.entries_by_categories(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_search_case_insensitive() {
        let sc = new_center();
        let r1 = sc.search("DOCK");
        let r2 = sc.search("dock");
        assert_eq!(r1.len(), r2.len());
    }

    #[test]
    fn test_requires_restart_entries_count() {
        let sc = new_center();
        let rr = sc.requires_restart();
        // dock.position, panel.position, kb.layout, kb.variant, kb.options require restart
        assert!(rr.len() >= 5);
    }

    #[test]
    fn test_requires_reload_entries_count() {
        let sc = new_center();
        let rl = sc.requires_reload();
        assert!(rl.len() >= 10);
    }

    #[test]
    fn test_save_preserves_directory() {
        let dir = temp_config_dir();
        let sc = SettingsCenter::new(Some(dir.clone()));
        sc.save().unwrap();
        assert!(dir.join("settings.json").exists());
    }

    #[test]
    fn test_default_config_dir_uses_home() {
        // when no config_dir is provided, uses $HOME/.config/edushell
        let sc = SettingsCenter::new(None);
        let home = std::env::var("HOME").unwrap();
        assert!(sc.config_dir.to_string_lossy().contains(&home));
        assert!(sc.config_dir.to_string_lossy().contains(".config/edushell"));
    }

    #[test]
    fn test_reset_category_all_categories() {
        let mut sc = new_center();
        for cat in sc.categories.clone() {
            sc.reset_category(cat).unwrap();
        }
        for e in sc.entries.values() {
            assert!(e.is_default());
        }
    }

    #[test]
    fn test_changed_entries_cleared_on_load() {
        let dir = temp_config_dir();
        let mut sc1 = SettingsCenter::new(Some(dir.clone()));
        sc1.set_bool("dock.auto_hide", true).unwrap();
        sc1.save().unwrap();

        let mut sc2 = SettingsCenter::new(Some(dir.clone()));
        sc2.set_bool("dock.show_running", false).unwrap();
        assert_eq!(sc2.changed_entries().len(), 1);
        sc2.load().unwrap();
        assert_eq!(sc2.changed_entries().len(), 0);
    }

    #[test]
    fn test_export_import_round_trip_all() {
        let mut sc1 = new_center();
        sc1.set_bool("dock.auto_hide", true).unwrap();
        sc1.set_int("dock.icon_size", 72).unwrap();
        sc1.set_string("font.display", "CustomFont").unwrap();
        let json = sc1.export_to_json();

        let mut sc2 = new_center();
        sc2.import_from_json(&json).unwrap();
        assert_eq!(sc2.get("dock.auto_hide").unwrap().value.as_bool(), Some(true));
        assert_eq!(sc2.get("dock.icon_size").unwrap().value.as_int(), Some(72));
        assert_eq!(sc2.get("font.display").unwrap().value.as_string(), Some("CustomFont"));

        // entries not in import should remain default
        assert_eq!(sc2.get("dock.position").unwrap().value.as_choice().unwrap().0, "bottom");
    }

    #[test]
    fn test_category_name_all_categories() {
        let sc = new_center();
        for cat in sc.categories() {
            let name = sc.category_name(*cat);
            assert!(!name.is_empty(), "category_name empty for {:?}", cat);
        }
    }

    #[test]
    fn test_category_description_all_categories() {
        let sc = new_center();
        for cat in sc.categories() {
            let desc = sc.category_description(*cat);
            assert!(!desc.is_empty(), "category_description empty for {:?}", cat);
        }
    }

    #[test]
    fn test_search_respects_category_filter_combined() {
        let sc = new_center();
        let results = sc.search("dock");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.key.contains("dock")));
    }

    #[test]
    fn test_set_same_value_twice() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        sc.set_bool("dock.auto_hide", true).unwrap();
        assert_eq!(sc.changed_entries().len(), 1);
    }

    #[test]
    fn test_total_entry_count_exact() {
        let sc = new_center();
        let total = sc.entries.len();
        // Appearance(12) + Dock(8) + Panel(6) + Theme(6) + Wallpaper(4) + Font(6)
        // + Animation(4) + Accessibility(8) + Language(4) + Keyboard(6) + Mouse(6)
        // + Touchpad(6) + Monitor(4) + Power(6) + Battery(4) + Bluetooth(4) + Wifi(4)
        // + Privacy(6) + Notification(4) + Update(4) + About(4) + Learning(4)
        // = 12+8+6+6+4+6+4+8+4+6+6+6+4+6+4+4+4+6+4+4+4+4 = 120
        assert_eq!(total, 120);
    }

    #[test]
    fn test_settings_entry_debug() {
        let entry = SettingsEntry {
            key: "test.key".into(),
            name_key: "test.name".into(),
            description_key: "test.desc".into(),
            category: SettingsCategory::Appearance,
            value: SettingsValue::Bool(true),
            default_value: SettingsValue::Bool(false),
            requires_restart: false,
            requires_reload: true,
            min: None,
            max: None,
            step: None,
            advanced: false,
        };
        let debug = format!("{:?}", entry);
        assert!(debug.contains("test.key"));
    }

    #[test]
    fn test_categories_slice_matches_enum() {
        let sc = new_center();
        assert_eq!(sc.categories().len(), 22);
        // ensure all enum variants appear, no dupes
        let mut seen = std::collections::HashSet::new();
        for cat in sc.categories() {
            assert!(seen.insert(*cat), "duplicate category {:?}", cat);
        }
    }

    #[test]
    fn test_reset_all_clears_changed() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        sc.set_bool("anim.reduce_motion", true).unwrap();
        assert!(sc.has_changes());
        sc.reset_all().unwrap();
        // changed entries list will have the reset entries too
        assert!(sc.has_changes());
    }

    #[test]
    fn test_get_mut_preserves_default() {
        let mut sc = new_center();
        {
            let entry = sc.get_mut("dock.auto_hide").unwrap();
            entry.value = SettingsValue::Bool(true);
        }
        assert!(!sc.get("dock.auto_hide").unwrap().is_default());
    }

    #[test]
    fn test_values_equal_float() {
        assert!(values_equal(&SettingsValue::Float(1.0), &SettingsValue::Float(1.0)));
        assert!(!values_equal(&SettingsValue::Float(1.0), &SettingsValue::Float(2.0)));
        assert!(values_equal(&SettingsValue::Float(0.1 + 0.2), &SettingsValue::Float(0.3)));
    }

    #[test]
    fn test_values_equal_across_types() {
        assert!(!values_equal(&SettingsValue::Bool(true), &SettingsValue::Int(1)));
        assert!(!values_equal(&SettingsValue::String("a".into()), &SettingsValue::Bool(true)));
    }

    #[test]
    fn test_search_partial_category_name() {
        let sc = new_center();
        let results = sc.search("appear");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.category == SettingsCategory::Appearance));
    }

    #[test]
    fn test_search_token_boundary() {
        let sc = new_center();
        let results = sc.search("theme_mode");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.key == "appearance.theme_mode"));
    }

    #[test]
    fn test_import_tracks_changes() {
        let mut sc1 = new_center();
        sc1.set_bool("dock.auto_hide", true).unwrap();
        let json = sc1.export_to_json();

        let mut sc2 = new_center();
        assert!(!sc2.has_changes());
        sc2.import_from_json(&json).unwrap();
        assert!(sc2.has_changes());
    }

    #[test]
    fn test_monitor_brightness_bounds_0_to_100() {
        let mut sc = new_center();
        assert!(sc.set_int("monitor.brightness", 0).is_ok());
        assert!(sc.set_int("monitor.brightness", 100).is_ok());
        assert!(sc.set_int("monitor.brightness", -1).is_err());
        assert!(sc.set_int("monitor.brightness", 101).is_err());
    }

    #[test]
    fn test_power_sleep_inactive_bounds() {
        let mut sc = new_center();
        assert!(sc.set_int("power.sleep_inactive", 0).is_ok());
        assert!(sc.set_int("power.sleep_inactive", 120).is_ok());
        assert!(sc.set_int("power.sleep_inactive", 130).is_err());
    }

    #[test]
    fn test_anim_speed_bounds() {
        let mut sc = new_center();
        assert!(sc.set("anim.speed", SettingsValue::Float(0.1)).is_ok());
        assert!(sc.set("anim.speed", SettingsValue::Float(5.0)).is_ok());
        assert!(sc.set("anim.speed", SettingsValue::Float(0.0)).is_err());
        assert!(sc.set("anim.speed", SettingsValue::Float(5.1)).is_err());
    }

    #[test]
    fn test_dock_icon_size_bounds() {
        let mut sc = new_center();
        assert!(sc.set_int("dock.icon_size", 24).is_ok());
        assert!(sc.set_int("dock.icon_size", 96).is_ok());
        assert!(sc.set_int("dock.icon_size", 20).is_err());
        assert!(sc.set_int("dock.icon_size", 100).is_err());
    }

    #[test]
    fn test_about_entries_are_not_advanced() {
        let sc = new_center();
        for e in sc.entries_by_category(SettingsCategory::About) {
            assert!(!e.advanced, "about entry {} should not be advanced", e.key);
        }
    }

    #[test]
    fn test_bluetooth_defaults() {
        let sc = new_center();
        assert_eq!(sc.get("bluetooth.enabled").unwrap().value.as_bool(), Some(true));
        assert_eq!(sc.get("bluetooth.discoverable").unwrap().value.as_bool(), Some(false));
    }

    #[test]
    fn test_wifi_defaults() {
        let sc = new_center();
        assert_eq!(sc.get("wifi.enabled").unwrap().value.as_bool(), Some(true));
        assert_eq!(sc.get("wifi.hotspot").unwrap().value.as_bool(), Some(false));
    }

    #[test]
    fn test_privacy_defaults() {
        let sc = new_center();
        assert_eq!(sc.get("privacy.screenshots").unwrap().value.as_bool(), Some(true));
        assert_eq!(sc.get("privacy.location").unwrap().value.as_bool(), Some(false));
        assert_eq!(sc.get("privacy.crash_reports").unwrap().value.as_bool(), Some(false));
    }

    #[test]
    fn test_learning_defaults() {
        let sc = new_center();
        assert_eq!(sc.get("learning.recommendations").unwrap().value.as_bool(), Some(true));
        assert_eq!(sc.get("learning.reminders").unwrap().value.as_bool(), Some(false));
        assert_eq!(sc.get("learning.notifications").unwrap().value.as_bool(), Some(true));
    }

    #[test]
    fn test_update_defaults() {
        let sc = new_center();
        assert_eq!(sc.get("update.auto_check").unwrap().value.as_bool(), Some(true));
        assert_eq!(sc.get("update.auto_download").unwrap().value.as_bool(), Some(false));
        assert_eq!(sc.get("update.channel").unwrap().value.as_choice().unwrap().0, "stable");
        assert_eq!(sc.get("update.interval").unwrap().value.as_int(), Some(24));
    }

    #[test]
    fn test_keyboard_defaults() {
        let sc = new_center();
        assert_eq!(sc.get("kb.layout").unwrap().value.as_string(), Some("us"));
        assert_eq!(sc.get("kb.repeat_rate").unwrap().value.as_int(), Some(30));
        assert_eq!(sc.get("kb.repeat_delay").unwrap().value.as_int(), Some(500));
    }

    #[test]
    fn test_notification_defaults() {
        let sc = new_center();
        assert_eq!(sc.get("notif.do_not_disturb").unwrap().value.as_bool(), Some(false));
        assert_eq!(sc.get("notif.show_banners").unwrap().value.as_bool(), Some(true));
    }

    #[test]
    fn test_export_json_parses_back() {
        let mut sc = new_center();
        sc.set_bool("dock.auto_hide", true).unwrap();
        let json = sc.export_to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
        let entries = parsed.get("entries").unwrap().as_object().unwrap();
        let dock_entry = entries.get("dock.auto_hide").unwrap();
        assert!(dock_entry.to_string().contains("Bool"));
    }

    #[test]
    fn test_search_returns_no_duplicates() {
        let sc = new_center();
        let results = sc.search("dock");
        let mut keys: Vec<&str> = results.iter().map(|e| e.key.as_str()).collect();
        let original_len = keys.len();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), original_len);
    }

    #[test]
    fn test_category_name_not_empty_for_all_locales() {
        let mut sc = new_center();
        sc.set_locale("id-ID");
        for cat in sc.categories() {
            let n = sc.category_name(*cat);
            assert!(!n.is_empty(), "ID name empty for {:?}", cat);
        }
        sc.set_locale("en-US");
        for cat in sc.categories() {
            let n = sc.category_name(*cat);
            assert!(!n.is_empty(), "EN name empty for {:?}", cat);
        }
    }

    #[test]
    fn test_save_load_maintains_types() {
        let dir = temp_config_dir();
        let mut sc1 = SettingsCenter::new(Some(dir.clone()));
        sc1.set_bool("dock.auto_hide", true).unwrap();
        sc1.set_int("dock.icon_size", 64).unwrap();
        sc1.save().unwrap();

        let mut sc2 = SettingsCenter::new(Some(dir));
        sc2.load().unwrap();
        assert!(sc2.get("dock.auto_hide").unwrap().value.as_bool().is_some());
        assert!(sc2.get("dock.icon_size").unwrap().value.as_int().is_some());
    }
}
