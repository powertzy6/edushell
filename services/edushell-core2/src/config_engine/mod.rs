//! Configuration engine — schema-driven config persistence.
use crate::core::CoreResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single config entry with schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub schema: ConfigSchema,
    pub description: String,
    pub group: String,
}

/// Schema type for validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigSchema {
    String,
    Integer,
    Float,
    Boolean,
    Color,
    Enum(Vec<String>),
}

/// Configuration engine.
pub struct ConfigEngine {
    entries: HashMap<String, ConfigEntry>,
    dirty: bool,
}

impl ConfigEngine {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            dirty: false,
        }
    }

    pub fn set(&mut self, key: &str, value: &str, schema: ConfigSchema, desc: &str, group: &str) {
        self.entries.insert(
            key.to_string(),
            ConfigEntry {
                key: key.to_string(),
                value: value.to_string(),
                schema,
                description: desc.to_string(),
                group: group.to_string(),
            },
        );
        self.dirty = true;
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|e| e.value.as_str())
    }

    pub fn get_entry(&self, key: &str) -> Option<&ConfigEntry> {
        self.entries.get(key)
    }

    pub fn all(&self) -> Vec<&ConfigEntry> {
        self.entries.values().collect()
    }

    pub fn by_group(&self, group: &str) -> Vec<&ConfigEntry> {
        self.entries.values().filter(|e| e.group == group).collect()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn validate(&self, key: &str, value: &str) -> bool {
        match self.entries.get(key) {
            Some(entry) => match &entry.schema {
                ConfigSchema::String => true,
                ConfigSchema::Integer => value.parse::<i64>().is_ok(),
                ConfigSchema::Float => value.parse::<f64>().is_ok(),
                ConfigSchema::Boolean => matches!(value, "true" | "false"),
                ConfigSchema::Color => value.starts_with('#') && value.len() == 7,
                ConfigSchema::Enum(variants) => variants.contains(&value.to_string()),
            },
            None => true,
        }
    }

    pub fn load_json(&mut self, json: &str) -> CoreResult<()> {
        let entries: HashMap<String, ConfigEntry> = serde_json::from_str(json)
            .map_err(|e| crate::core::CoreError::ConfigError(e.to_string()))?;
        self.entries = entries;
        self.dirty = false;
        Ok(())
    }

    pub fn export_json(&self) -> CoreResult<String> {
        serde_json::to_string(&self.entries)
            .map_err(|e| crate::core::CoreError::Internal(e.to_string()))
    }
}

impl Default for ConfigEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let ce = ConfigEngine::new();
        assert!(ce.all().is_empty());
        assert!(!ce.is_dirty());
    }

    #[test]
    fn test_config_set_get() {
        let mut ce = ConfigEngine::new();
        ce.set(
            "theme.mode",
            "dark",
            ConfigSchema::String,
            "Theme mode",
            "theme",
        );
        assert_eq!(ce.get("theme.mode"), Some("dark"));
        assert!(ce.is_dirty());
    }

    #[test]
    fn test_config_validate_integer() {
        let mut ce = ConfigEngine::new();
        ce.set(
            "font.size",
            "12",
            ConfigSchema::Integer,
            "Font size",
            "appearance",
        );
        assert!(ce.validate("font.size", "14"));
        assert!(!ce.validate("font.size", "abc"));
    }

    #[test]
    fn test_config_validate_boolean() {
        let mut ce = ConfigEngine::new();
        ce.set(
            "animations",
            "true",
            ConfigSchema::Boolean,
            "Animations",
            "theme",
        );
        assert!(ce.validate("animations", "false"));
        assert!(!ce.validate("animations", "yes"));
    }

    #[test]
    fn test_config_validate_color() {
        let mut ce = ConfigEngine::new();
        ce.set(
            "accent",
            "#3584e4",
            ConfigSchema::Color,
            "Accent color",
            "theme",
        );
        assert!(ce.validate("accent", "#ff0000"));
        assert!(!ce.validate("accent", "red"));
    }

    #[test]
    fn test_config_validate_enum() {
        let mut ce = ConfigEngine::new();
        ce.set(
            "lang",
            "id-ID",
            ConfigSchema::Enum(vec!["id-ID".into(), "en-US".into()]),
            "Language",
            "locale",
        );
        assert!(ce.validate("lang", "en-US"));
        assert!(!ce.validate("lang", "fr-FR"));
    }

    #[test]
    fn test_config_by_group() {
        let mut ce = ConfigEngine::new();
        ce.set("a", "1", ConfigSchema::String, "", "g1");
        ce.set("b", "2", ConfigSchema::String, "", "g2");
        ce.set("c", "3", ConfigSchema::String, "", "g1");
        assert_eq!(ce.by_group("g1").len(), 2);
    }

    #[test]
    fn test_config_json_roundtrip() {
        let mut ce = ConfigEngine::new();
        ce.set("theme.mode", "dark", ConfigSchema::String, "", "theme");
        let json = ce.export_json().unwrap();
        let mut ce2 = ConfigEngine::new();
        ce2.load_json(&json).unwrap();
        assert_eq!(ce2.get("theme.mode"), Some("dark"));
    }
}
