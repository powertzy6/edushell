// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::config::schema::EduConfig;
use crate::config::migration::migrate_config;
use crate::error::{ConfigErrorKind, EduError, EduResult};

/// Manages loading, saving, watching, and migrating configuration.
///
/// Thread-safe: uses `Arc<RwLock<>>` for interior mutability.
#[derive(Clone)]
pub struct ConfigManager {
    path: PathBuf,
    current: Arc<RwLock<EduConfig>>,
    dirty: Arc<RwLock<bool>>,
}

impl ConfigManager {
    /// Create a new config manager at the standard config path.
    pub fn new() -> Self {
        let path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("edushell")
            .join("config.toml");

        Self {
            path,
            current: Arc::new(RwLock::new(EduConfig::default())),
            dirty: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a config manager at a custom path.
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            path,
            current: Arc::new(RwLock::new(EduConfig::default())),
            dirty: Arc::new(RwLock::new(false)),
        }
    }

    /// Load configuration from the file.
    ///
    /// If the file doesn't exist, creates it with defaults.
    /// If the file has an older schema version, migrates it.
    pub fn load(&self) -> EduResult<EduConfig> {
        if !self.path.exists() {
            // Ensure parent directory exists
            if let Some(parent) = self.path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let defaults = EduConfig::default();
            let toml_str = toml::to_string_pretty(&defaults)?;
            std::fs::write(&self.path, &toml_str)?;

            tracing::info!(
                target: "edushell::config",
                path = %self.path.display(),
                "Created default configuration"
            );

            let mut current = self.current.write().map_err(|e| {
                EduError::Unknown(format!("Config lock poisoned: {e}"))
            })?;
            *current = defaults.clone();
            *self.dirty.write().map_err(|e| {
                EduError::Unknown(format!("Config lock poisoned: {e}"))
            })? = false;

            return Ok(defaults);
        }

        let content = std::fs::read_to_string(&self.path).map_err(|e| {
            ConfigErrorKind::Parse {
                path: self.path.clone(),
                detail: e.to_string(),
            }
        })?;

        let mut config: EduConfig = toml::from_str(&content).map_err(|e| {
            ConfigErrorKind::Parse {
                path: self.path.clone(),
                detail: e.to_string(),
            }
        })?;

        // Run migration if needed
        let current_version = config.version.clone();
        if current_version != env!("CARGO_PKG_VERSION") {
            migrate_config(&mut config, &current_version, env!("CARGO_PKG_VERSION"))?;
            // Save migrated config
            self.write_config(&config)?;
        }

        let mut current = self.current.write().map_err(|e| {
            EduError::Unknown(format!("Config lock poisoned: {e}"))
        })?;
        *current = config.clone();
        *self.dirty.write().map_err(|e| {
            EduError::Unknown(format!("Config lock poisoned: {e}"))
        })? = false;

        tracing::info!(
            target: "edushell::config",
            path = %self.path.display(),
            version = %config.version,
            "Configuration loaded"
        );

        Ok(config)
    }

    /// Get a snapshot of the current configuration.
    pub fn current(&self) -> EduResult<EduConfig> {
        self.current.read().map(|c| c.clone()).map_err(|e| {
            EduError::Unknown(format!("Config lock poisoned: {e}"))
        })
    }

    /// Update the configuration and save to disk.
    pub fn update<F>(&self, updater: F) -> EduResult<EduConfig>
    where
        F: FnOnce(&mut EduConfig),
    {
        let mut current = self.current.write().map_err(|e| {
            EduError::Unknown(format!("Config lock poisoned: {e}"))
        })?;

        updater(&mut current);

        // Write to file
        let toml_str = toml::to_string_pretty(&*current)?;
        std::fs::write(&self.path, &toml_str)?;

        *self.dirty.write().map_err(|e| {
            EduError::Unknown(format!("Config lock poisoned: {e}"))
        })? = true;

        tracing::debug!(target: "edushell::config", "Configuration updated");

        Ok(current.clone())
    }

    /// Write config to disk.
    fn write_config(&self, config: &EduConfig) -> EduResult<()> {
        let toml_str = toml::to_string_pretty(config)?;
        std::fs::write(&self.path, &toml_str)?;
        Ok(())
    }

    /// Reset to factory defaults.
    pub fn reset(&self) -> EduResult<EduConfig> {
        let defaults = EduConfig::default();
        self.write_config(&defaults)?;

        let mut current = self.current.write().map_err(|e| {
            EduError::Unknown(format!("Config lock poisoned: {e}"))
        })?;
        *current = defaults.clone();

        tracing::info!(target: "edushell::config", "Configuration reset to defaults");
        Ok(defaults)
    }

    /// Export configuration as a TOML string.
    pub fn export(&self) -> EduResult<String> {
        let config = self.current()?;
        toml::to_string_pretty(&config).map_err(EduError::from)
    }

    /// Import configuration from a TOML string.
    pub fn import(&self, data: &str) -> EduResult<EduConfig> {
        let config: EduConfig = toml::from_str(data).map_err(|e| {
            ConfigErrorKind::Parse {
                path: self.path.clone(),
                detail: e.to_string(),
            }
        })?;

        self.write_config(&config)?;

        let mut current = self.current.write().map_err(|e| {
            EduError::Unknown(format!("Config lock poisoned: {e}"))
        })?;
        *current = config.clone();

        tracing::info!(target: "edushell::config", "Configuration imported");
        Ok(config)
    }

    /// Get the path of the config file.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Check if the configuration has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty.read().map(|d| *d).unwrap_or(false)
    }

    /// Reload configuration from disk.
    pub fn reload(&self) -> EduResult<EduConfig> {
        self.load()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_config_path() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        (dir, path)
    }

    #[test]
    fn test_create_default_config() {
        let (_dir, path) = test_config_path();
        let mgr = ConfigManager::with_path(path.clone());
        let config = mgr.load().unwrap();

        assert_eq!(config.shell.panel_position, crate::config::PanelPosition::Bottom);
        assert_eq!(config.theme.mode, crate::config::ThemeMode::Auto);
        assert!(path.exists());
    }

    #[test]
    fn test_load_existing_config() {
        let (_dir, path) = test_config_path();

        // Create config file first
        let defaults = EduConfig::default();
        let toml_str = toml::to_string_pretty(&defaults).unwrap();
        std::fs::write(&path, &toml_str).unwrap();

        let mgr = ConfigManager::with_path(path);
        let config = mgr.load().unwrap();
        assert_eq!(config.shell.workspace_count, 4);
    }

    #[test]
    fn test_update_config() {
        let (_dir, path) = test_config_path();
        let mgr = ConfigManager::with_path(path);
        mgr.load().unwrap();

        mgr.update(|config| {
            config.shell.panel_position = crate::config::PanelPosition::Top;
            config.theme.mode = crate::config::ThemeMode::Dark;
        })
        .unwrap();

        let updated = mgr.current().unwrap();
        assert_eq!(updated.shell.panel_position, crate::config::PanelPosition::Top);
        assert_eq!(updated.theme.mode, crate::config::ThemeMode::Dark);
    }

    #[test]
    fn test_reset_config() {
        let (_dir, path) = test_config_path();
        let mgr = ConfigManager::with_path(path);
        mgr.load().unwrap();

        mgr.update(|config| {
            config.shell.workspace_count = 8;
        })
        .unwrap();

        let reset = mgr.reset().unwrap();
        assert_eq!(reset.shell.workspace_count, 4); // Default
    }

    #[test]
    fn test_export_import() {
        let (_dir, path) = test_config_path();
        let mgr = ConfigManager::with_path(path);
        mgr.load().unwrap();

        let exported = mgr.export().unwrap();
        assert!(exported.contains("panel-position"));

        mgr.update(|config| {
            config.shell.workspace_count = 6;
        })
        .unwrap();

        // Import back to original
        mgr.import(&exported).unwrap();
        let restored = mgr.current().unwrap();
        assert_eq!(restored.shell.workspace_count, 4);
    }

    #[test]
    fn test_dirty_flag() {
        let (_dir, path) = test_config_path();
        let mgr = ConfigManager::with_path(path);
        mgr.load().unwrap();
        assert!(!mgr.is_dirty());

        mgr.update(|config| {
            config.shell.workspace_count = 3;
        })
        .unwrap();
        assert!(mgr.is_dirty());
    }
}
