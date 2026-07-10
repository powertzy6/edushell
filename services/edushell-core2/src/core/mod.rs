//! Core types, identifiers, versioning, and base errors.
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// EduShell Core v2 version.
pub const CORE_VERSION: &str = "2.0.0";

/// Minimum supported Rust version.
pub const MSRV: &str = "1.75.0";

/// Unique identifier (UUID v4).
pub type Id = String;

/// Timestamp in RFC 3339 format.
pub type Timestamp = String;

/// A semantic version string.
pub type SemVer = String;

/// Generic result for core operations.
pub type CoreResult<T> = Result<T, CoreError>;

/// Core error enumeration.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("already exists: {0}")]
    AlreadyExists(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("version mismatch: expected {expected}, got {got}")]
    VersionMismatch { expected: String, got: String },
    #[error("configuration error: {0}")]
    ConfigError(String),
    #[error("internal error: {0}")]
    Internal(String),
}

/// Generate a new UUID v4 ID.
pub fn new_id() -> Id {
    uuid::Uuid::new_v4().to_string()
}

/// Generate current timestamp.
pub fn now_iso() -> Timestamp {
    chrono::Utc::now().to_rfc3339()
}

/// Component identification for registry/lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentId {
    pub name: String,
    pub version: SemVer,
    pub kind: ComponentKind,
}

/// Type of component.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentKind {
    Core,
    Service,
    Extension,
    Widget,
    Theme,
    Plugin,
    Application,
}

/// Feature flags for capability-based access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub experimental: bool,
    pub developer_mode: bool,
    pub accessibility: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            experimental: false,
            developer_mode: false,
            accessibility: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_version() {
        assert_eq!(CORE_VERSION, "2.0.0");
    }

    #[test]
    fn test_new_id_is_non_empty() {
        let id = new_id();
        assert!(!id.is_empty());
        assert_eq!(id.len(), 36);
    }

    #[test]
    fn test_now_iso_format() {
        let ts = now_iso();
        assert!(ts.contains('T'));
        assert!(ts.ends_with('Z') || ts.contains('+'));
    }

    #[test]
    fn test_component_id_serde() {
        let cid = ComponentId {
            name: "test-core".into(),
            version: "2.0.0".into(),
            kind: ComponentKind::Core,
        };
        let json = serde_json::to_string(&cid).unwrap();
        let d: ComponentId = serde_json::from_str(&json).unwrap();
        assert_eq!(d.name, "test-core");
        assert_eq!(d.kind, ComponentKind::Core);
    }

    #[test]
    fn test_feature_flags_default() {
        let f = FeatureFlags::default();
        assert!(!f.experimental);
        assert!(f.accessibility);
    }

    #[test]
    fn test_core_error_not_found() {
        let e = CoreError::NotFound("file".into());
        assert_eq!(e.to_string(), "not found: file");
    }

    #[test]
    fn test_core_error_version_mismatch() {
        let e = CoreError::VersionMismatch {
            expected: "2.0".into(),
            got: "1.0".into(),
        };
        assert!(e.to_string().contains("2.0"));
    }

    #[test]
    fn test_component_kind_variant() {
        assert_eq!(format!("{:?}", ComponentKind::Theme), "Theme");
    }
}
