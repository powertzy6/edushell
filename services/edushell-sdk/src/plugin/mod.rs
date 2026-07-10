//! Plugin API v1 — traits, manifest, lifecycle, permissions.
use serde::{Deserialize, Serialize};

/// Unique identifier for a plugin.
pub type PluginId = String;

/// SDK version constant.
pub const SDK_VERSION: &str = "1.0.0";

/// Minimum shell version required.
pub const MIN_SHELL_VERSION: &str = "1.0.0";

/// Plugin manifest — required metadata for every plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: PluginId,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub min_sdk_version: String,
    pub min_shell_version: String,
    pub permissions: Vec<PluginPermission>,
    pub category: PluginCategory,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
}

/// Categories for plugins.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    Panel,
    Dock,
    Launcher,
    Widget,
    SearchProvider,
    Theme,
    IconPack,
    WallpaperPack,
    Notification,
    QuickSettings,
    Learning,
    Project,
    Localization,
    Service,
    Other(String),
}

/// Permissions a plugin can request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginPermission {
    ReadSettings,
    WriteSettings,
    ReadFiles,
    WriteFiles,
    ExecuteCommand,
    NetworkAccess,
    Notifications,
    SearchProvider,
    ThemeOverride,
    WidgetApi,
    SystemInfo,
}

/// Plugin lifecycle state.
#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    Loaded,
    Enabled,
    Disabled,
    Crashed(String),
}

/// Plugin trait — implement this to create an EduShell plugin.
pub trait Plugin: Send + Sync {
    fn manifest(&self) -> &PluginManifest;
    fn on_load(&mut self) -> Result<(), PluginError>;
    fn on_enable(&mut self) -> Result<(), PluginError>;
    fn on_disable(&mut self) -> Result<(), PluginError>;
    fn on_unload(&mut self) -> Result<(), PluginError>;
    fn state(&self) -> PluginState;
}

/// Plugin errors.
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("manifest validation failed: {0}")]
    ManifestError(String),
    #[error("initialization failed: {0}")]
    InitError(String),
    #[error("runtime error: {0}")]
    RuntimeError(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("dependency missing: {0}")]
    DependencyMissing(String),
    #[error("version mismatch: required {required}, found {found}")]
    VersionMismatch { required: String, found: String },
}

/// Validate a plugin manifest.
pub fn validate_manifest(manifest: &PluginManifest) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if manifest.id.is_empty() {
        errors.push("plugin id is empty".into());
    }
    if manifest.name.is_empty() {
        errors.push("plugin name is empty".into());
    }
    if manifest.version.is_empty() {
        errors.push("plugin version is empty".into());
    }
    if manifest.author.is_empty() {
        errors.push("plugin author is empty".into());
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> PluginManifest {
        PluginManifest {
            id: "com.example.test".into(),
            name: "Test Plugin".into(),
            version: "1.0.0".into(),
            author: "Test Author".into(),
            description: "A test plugin".into(),
            min_sdk_version: "1.0.0".into(),
            min_shell_version: "1.0.0".into(),
            permissions: vec![PluginPermission::ReadSettings],
            category: PluginCategory::Panel,
            dependencies: vec![],
            icon: None,
            homepage: None,
            repository: None,
        }
    }

    #[test]
    fn test_validate_manifest_ok() {
        let m = sample_manifest();
        assert!(validate_manifest(&m).is_ok());
    }

    #[test]
    fn test_validate_manifest_empty_id() {
        let mut m = sample_manifest();
        m.id = "".into();
        assert!(validate_manifest(&m).is_err());
    }

    #[test]
    fn test_plugin_category_debug() {
        let c = PluginCategory::Panel;
        assert_eq!(format!("{:?}", c), "Panel");
    }

    #[test]
    fn test_plugin_permission_debug() {
        let p = PluginPermission::NetworkAccess;
        assert_eq!(format!("{:?}", p), "NetworkAccess");
    }

    #[test]
    fn test_plugin_state_crashed() {
        let s = PluginState::Crashed("oom".into());
        assert_eq!(format!("{:?}", s), "Crashed(\"oom\")");
    }

    #[test]
    fn test_plugin_error_version_mismatch() {
        let e = PluginError::VersionMismatch {
            required: "1.0.0".into(),
            found: "0.9.0".into(),
        };
        assert!(e.to_string().contains("1.0.0"));
    }

    #[test]
    fn test_manifest_serde_roundtrip() {
        let m = sample_manifest();
        let json = serde_json::to_string(&m).unwrap();
        let deserialized: PluginManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "com.example.test");
    }

    #[test]
    fn test_manifest_empty_name() {
        let mut m = sample_manifest();
        m.name = "".into();
        assert!(validate_manifest(&m).is_err());
    }

    #[test]
    fn test_sdk_version_constant() {
        assert_eq!(SDK_VERSION, "1.0.0");
    }
}
