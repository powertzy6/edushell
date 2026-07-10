//! Theme engine — CSS generation, token resolution, theme loading.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

/// Design token value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignToken {
    pub name: String,
    pub value: String,
    pub category: String,
}

/// Complete theme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDefinition {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub mode: ThemeMode,
    pub tokens: HashMap<String, String>,
    pub css_overrides: Option<String>,
}

/// Theme engine.
pub struct ThemeEngine {
    current: Option<ThemeDefinition>,
    themes: HashMap<String, ThemeDefinition>,
}

impl ThemeEngine {
    pub fn new() -> Self {
        Self {
            current: None,
            themes: HashMap::new(),
        }
    }

    pub fn register(&mut self, theme: ThemeDefinition) {
        self.themes.insert(theme.id.clone(), theme);
    }

    pub fn activate(&mut self, id: &str) -> bool {
        if let Some(theme) = self.themes.get(id) {
            self.current = Some(theme.clone());
            true
        } else {
            false
        }
    }

    pub fn current(&self) -> Option<&ThemeDefinition> {
        self.current.as_ref()
    }
    pub fn list(&self) -> Vec<&ThemeDefinition> {
        self.themes.values().collect()
    }

    pub fn resolve_token(&self, name: &str) -> Option<&str> {
        self.current.as_ref()?.tokens.get(name).map(|s| s.as_str())
    }

    pub fn generate_css(&self) -> String {
        let mut css = String::new();
        if let Some(theme) = &self.current {
            for (name, value) in &theme.tokens {
                css.push_str(&format!("  --{}: {};\n", name, value));
            }
            if let Some(overrides) = &theme.css_overrides {
                css.push('\n');
                css.push_str(overrides);
            }
        }
        format!(":root {{\n{}}}", css)
    }

    pub fn default_dark() -> ThemeDefinition {
        ThemeDefinition {
            id: "edushell-dark".into(),
            name: "EduShell Dark".into(),
            version: "2.0.0".into(),
            author: "EduShell Team".into(),
            mode: ThemeMode::Dark,
            tokens: HashMap::from([
                ("bg-primary".into(), "#1a1a2e".into()),
                ("bg-secondary".into(), "#16213e".into()),
                ("fg-primary".into(), "#e0e0e0".into()),
                ("accent".into(), "#3584e4".into()),
            ]),
            css_overrides: None,
        }
    }

    pub fn default_light() -> ThemeDefinition {
        ThemeDefinition {
            id: "edushell-light".into(),
            name: "EduShell Light".into(),
            version: "2.0.0".into(),
            author: "EduShell Team".into(),
            mode: ThemeMode::Light,
            tokens: HashMap::from([
                ("bg-primary".into(), "#ffffff".into()),
                ("bg-secondary".into(), "#f5f5f5".into()),
                ("fg-primary".into(), "#1a1a1a".into()),
                ("accent".into(), "#3584e4".into()),
            ]),
            css_overrides: None,
        }
    }
}

impl Default for ThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_engine_new() {
        let te = ThemeEngine::new();
        assert!(te.current().is_none());
    }

    #[test]
    fn test_register_and_activate() {
        let mut te = ThemeEngine::new();
        te.register(ThemeEngine::default_dark());
        assert!(te.activate("edushell-dark"));
        assert!(te.current().is_some());
    }

    #[test]
    fn test_activate_nonexistent() {
        let mut te = ThemeEngine::new();
        assert!(!te.activate("nonexistent"));
    }

    #[test]
    fn test_resolve_token() {
        let mut te = ThemeEngine::new();
        te.register(ThemeEngine::default_dark());
        te.activate("edushell-dark");
        assert_eq!(te.resolve_token("bg-primary"), Some("#1a1a2e"));
        assert!(te.resolve_token("nonexistent").is_none());
    }

    #[test]
    fn test_generate_css() {
        let mut te = ThemeEngine::new();
        te.register(ThemeEngine::default_dark());
        te.activate("edushell-dark");
        let css = te.generate_css();
        assert!(css.contains("--bg-primary: #1a1a2e"));
        assert!(css.contains(":root"));
    }

    #[test]
    fn test_list_themes() {
        let mut te = ThemeEngine::new();
        te.register(ThemeEngine::default_dark());
        te.register(ThemeEngine::default_light());
        assert_eq!(te.list().len(), 2);
    }

    #[test]
    fn test_theme_mode_variants() {
        assert_eq!(format!("{:?}", ThemeMode::Auto), "Auto");
    }

    #[test]
    fn test_theme_definition_serde() {
        let t = ThemeEngine::default_dark();
        let json = serde_json::to_string(&t).unwrap();
        let d: ThemeDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(d.id, "edushell-dark");
    }
}
