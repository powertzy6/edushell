// SPDX-License-Identifier: GPL-3.0-or-later

//! # Settings Backend
//!
//! Key-value settings storage with type-safe access,
//! change notifications, and per-key schema metadata.
//! Uses a TOML-based file for persistence alongside
//! the main configuration.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::error::EduResult;

/// A typed setting value.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SettingsValue {
    /// Boolean setting.
    Bool(bool),
    /// Integer setting.
    Int(i64),
    /// Float setting.
    Float(f64),
    /// String setting.
    String(String),
    /// List of strings.
    StringList(Vec<String>),
}

/// A unique settings key.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SettingsKey(pub String);

impl From<&str> for SettingsKey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Change notification for a setting.
#[derive(Debug, Clone)]
pub struct SettingsChange {
    /// The key that changed.
    pub key: SettingsKey,
    /// The new value.
    pub new_value: SettingsValue,
    /// The previous value, if any.
    pub old_value: Option<SettingsValue>,
}

/// Schema metadata for a setting.
#[derive(Debug, Clone)]
pub struct SettingsMeta {
    /// Display name.
    pub name: String,
    /// Description of what this setting does.
    pub description: String,
    /// Category for grouping in settings UI.
    pub category: String,
    /// Default value.
    pub default: SettingsValue,
    /// Minimum value (for numeric types).
    pub min: Option<f64>,
    /// Maximum value (for numeric types).
    pub max: Option<f64>,
    /// Possible values (for enum settings).
    pub options: Vec<String>,
}

/// Settings backend with file persistence.
#[derive(Clone)]
pub struct SettingsBackend {
    path: PathBuf,
    store: Arc<std::sync::RwLock<HashMap<String, SettingsValue>>>,
    schema: Arc<std::sync::RwLock<HashMap<String, SettingsMeta>>>,
    dirty: Arc<std::sync::RwLock<bool>>,
}

impl SettingsBackend {
    /// Create a new settings backend.
    pub fn new() -> Self {
        let path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("edushell")
            .join("settings.toml");

        Self {
            path,
            store: Arc::new(std::sync::RwLock::new(HashMap::new())),
            schema: Arc::new(std::sync::RwLock::new(HashMap::new())),
            dirty: Arc::new(std::sync::RwLock::new(false)),
        }
    }

    /// Create a settings backend at a custom path.
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            path,
            store: Arc::new(std::sync::RwLock::new(HashMap::new())),
            schema: Arc::new(std::sync::RwLock::new(HashMap::new())),
            dirty: Arc::new(std::sync::RwLock::new(false)),
        }
    }

    /// Register a setting with its schema metadata.
    pub fn register(&self, key: &str, meta: SettingsMeta) {
        if let Ok(mut schema) = self.schema.write() {
            schema.insert(key.to_string(), meta);
        }
    }

    /// Load settings from disk.
    pub fn load(&self) -> EduResult<()> {
        if !self.path.exists() {
            self.save()?;
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.path)?;
        let parsed: toml::Value = toml::from_str(&content)
            .map_err(|e| crate::error::EduError::Unknown(format!("Settings parse: {e}")))?;

        let mut store = self.store.write()
            .map_err(|e| crate::error::EduError::Unknown(format!("Settings lock: {e}")))?;

        flatten_toml_into(&mut store, "", &parsed);

        *self.dirty.write()
            .map_err(|e| crate::error::EduError::Unknown(format!("Settings lock: {e}")))? = false;

        tracing::info!(target: "edushell::settings", path = %self.path.display(), "Settings loaded");
        Ok(())
    }

    /// Save settings to disk.
    pub fn save(&self) -> EduResult<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let store = self.store.read()
            .map_err(|e| crate::error::EduError::Unknown(format!("Settings lock: {e}")))?;

        let mut map = toml::map::Map::new();
        for (key, value) in store.iter() {
            if let Some(tv) = settings_value_to_toml(value) {
                map.insert(key.clone(), tv);
            }
        }

        let toml_str = toml::to_string_pretty(&toml::Value::Table(map))?;
        std::fs::write(&self.path, &toml_str)?;

        *self.dirty.write()
            .map_err(|e| crate::error::EduError::Unknown(format!("Settings lock: {e}")))? = false;

        Ok(())
    }

    /// Get a setting value.
    pub fn get(&self, key: &str) -> Option<SettingsValue> {
        let store = self.store.read().ok()?;
        store.get(key).cloned()
    }

    /// Set a setting value.
    pub fn set(&self, key: &str, value: SettingsValue) -> EduResult<SettingsChange> {
        let old_value = {
            let mut store = self.store.write()
                .map_err(|e| crate::error::EduError::Unknown(format!("Settings lock: {e}")))?;
            store.insert(key.to_string(), value.clone())
        };

        *self.dirty.write()
            .map_err(|e| crate::error::EduError::Unknown(format!("Settings lock: {e}")))? = true;

        Ok(SettingsChange {
            key: SettingsKey(key.to_string()),
            new_value: value,
            old_value,
        })
    }

    /// Reset a setting to its default value.
    pub fn reset(&self, key: &str) -> EduResult<Option<SettingsChange>> {
        let schema = self.schema.read()
            .map_err(|e| crate::error::EduError::Unknown(format!("Settings lock: {e}")))?;

        if let Some(meta) = schema.get(key) {
            let value = meta.default.clone();
            drop(schema);
            let change = self.set(key, value)?;
            Ok(Some(change))
        } else {
            Ok(None)
        }
    }

    /// Get all settings.
    pub fn all(&self) -> HashMap<String, SettingsValue> {
        self.store.read()
            .map(|s| s.clone())
            .unwrap_or_default()
    }

    /// Get all registered keys.
    pub fn keys(&self) -> Vec<String> {
        self.schema.read()
            .map(|s| s.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Get schema metadata for a key.
    pub fn meta(&self, key: &str) -> Option<SettingsMeta> {
        self.schema.read().ok().and_then(|s| s.get(key).cloned())
    }

    /// Check if there are unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty.read().map(|d| *d).unwrap_or(false)
    }

    /// Reload from disk.
    pub fn reload(&self) -> EduResult<()> {
        self.load()
    }
}

impl Default for SettingsBackend {
    fn default() -> Self {
        Self::new()
    }
}

// ── Conversion helpers ─────────────────────────────────────────

fn toml_value_to_settings(value: toml::Value) -> Option<SettingsValue> {
    match value {
        toml::Value::Boolean(b) => Some(SettingsValue::Bool(b)),
        toml::Value::Integer(i) => Some(SettingsValue::Int(i)),
        toml::Value::Float(f) => Some(SettingsValue::Float(f)),
        toml::Value::String(s) => Some(SettingsValue::String(s)),
        toml::Value::Array(arr) => {
            let strings: Vec<String> = arr.into_iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
            Some(SettingsValue::StringList(strings))
        }
        _ => None,
    }
}

fn flatten_toml_into(
    store: &mut std::collections::HashMap<String, SettingsValue>,
    prefix: &str,
    value: &toml::Value,
) {
    match value {
        toml::Value::Table(table) => {
            for (k, v) in table {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_toml_into(store, &key, v);
            }
        }
        _ => {
            if let Some(sv) = toml_value_to_settings(value.clone()) {
                store.insert(prefix.to_string(), sv);
            }
        }
    }
}

fn settings_value_to_toml(value: &SettingsValue) -> Option<toml::Value> {
    match value {
        SettingsValue::Bool(b) => Some(toml::Value::Boolean(*b)),
        SettingsValue::Int(i) => Some(toml::Value::Integer(*i)),
        SettingsValue::Float(f) => Some(toml::Value::Float(*f)),
        SettingsValue::String(s) => Some(toml::Value::String(s.clone())),
        SettingsValue::StringList(list) => {
            let arr: Vec<toml::Value> = list.iter()
                .map(|s| toml::Value::String(s.clone()))
                .collect();
            Some(toml::Value::Array(arr))
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_settings_path() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("settings.toml");
        (dir, path)
    }

    #[test]
    fn test_set_get() {
        let (_dir, path) = test_settings_path();
        let backend = SettingsBackend::with_path(path);

        backend.register("theme.mode", SettingsMeta {
            name: "Theme Mode".into(),
            description: "Light or dark theme".into(),
            category: "appearance".into(),
            default: SettingsValue::String("light".into()),
            min: None,
            max: None,
            options: vec!["light".into(), "dark".into()],
        });

        backend.set("theme.mode", SettingsValue::String("dark".into())).unwrap();
        let value = backend.get("theme.mode");
        assert!(value.is_some());
        match value.unwrap() {
            SettingsValue::String(s) => assert_eq!(s, "dark"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_load_save() {
        let (_dir, path) = test_settings_path();
        let backend = SettingsBackend::with_path(path.clone());
        backend.set("test.key", SettingsValue::Int(42)).unwrap();
        backend.save().unwrap();

        let backend2 = SettingsBackend::with_path(path);
        backend2.load().unwrap();
        let value = backend2.get("test.key");
        assert!(value.is_some());
        match value.unwrap() {
            SettingsValue::Int(i) => assert_eq!(i, 42),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_reset_to_default() {
        let (_dir, path) = test_settings_path();
        let backend = SettingsBackend::with_path(path);

        backend.register("volume", SettingsMeta {
            name: "Volume".into(),
            description: "System volume".into(),
            category: "audio".into(),
            default: SettingsValue::Int(50),
            min: Some(0.0),
            max: Some(100.0),
            options: vec![],
        });

        backend.set("volume", SettingsValue::Int(75)).unwrap();
        assert_eq!(backend.get("volume").map(|v| format!("{v:?}")), Some("Int(75)".to_string()));

        backend.reset("volume").unwrap();
        match backend.get("volume").unwrap() {
            SettingsValue::Int(i) => assert_eq!(i, 50),
            _ => panic!("Expected Int 50"),
        }
    }

    #[test]
    fn test_dirty_flag() {
        let (_dir, path) = test_settings_path();
        let backend = SettingsBackend::with_path(path);

        assert!(!backend.is_dirty());
        backend.set("key", SettingsValue::Bool(true)).unwrap();
        assert!(backend.is_dirty());
        backend.save().unwrap();
        assert!(!backend.is_dirty());
    }

    #[test]
    fn test_keys() {
        let (_dir, path) = test_settings_path();
        let backend = SettingsBackend::with_path(path);

        backend.register("a", SettingsMeta {
            name: "A".into(), description: "".into(), category: "".into(),
            default: SettingsValue::Bool(false), min: None, max: None, options: vec![],
        });
        backend.register("b", SettingsMeta {
            name: "B".into(), description: "".into(), category: "".into(),
            default: SettingsValue::Int(0), min: None, max: None, options: vec![],
        });

        let keys = backend.keys();
        assert!(keys.contains(&"a".to_string()));
        assert!(keys.contains(&"b".to_string()));
    }
}
