// SPDX-License-Identifier: GPL-3.0-or-later

//! # Error Framework
//!
//! Unified error types for all EduShell components.
//! Every module produces typed errors that chain to
//! provide context while remaining user-friendly.

use std::fmt;
use std::path::PathBuf;

/// Top-level error for all EduShell operations.
#[derive(Debug)]
pub enum EduError {
    /// Configuration-related error.
    Config(ConfigErrorKind),
    /// I/O operation failed.
    Io(std::io::Error),
    /// Theme engine error.
    Theme(ThemeErrorKind),
    /// Application registry error.
    AppRegistry(AppRegistryErrorKind),
    /// Search engine error.
    Search(SearchErrorKind),
    /// Session management error.
    Session(SessionErrorKind),
    /// Window tracker error.
    Window(WindowErrorKind),
    /// Resource loading error.
    Resource(ResourceErrorKind),
    /// Service lifecycle error.
    Service(ServiceErrorKind),
    /// Serialization/deserialization error.
    Serialization(SerializationErrorKind),
    /// D-Bus communication error (only with `dbus` feature).
    #[cfg(feature = "dbus")]
    DBus(zbus::Error),
    /// Unknown or unexpected error.
    Unknown(String),
}

/// Configuration error variants.
#[derive(Debug)]
pub enum ConfigErrorKind {
    /// Failed to read or parse config file.
    Parse { path: PathBuf, detail: String },
    /// Schema version migration failed.
    Migrate { from: String, to: String, detail: String },
    /// Failed to write config to disk.
    Write { path: PathBuf, source: std::io::Error },
    /// Validation of a config field failed.
    Validate { field: String, reason: String },
    /// Config file not found at expected path.
    NotFound { path: PathBuf },
    /// Failed to watch config file for changes.
    Watch { path: PathBuf, source: String },
}

/// Theme engine error variants.
#[derive(Debug)]
pub enum ThemeErrorKind {
    /// Theme not found in search paths.
    NotFound { name: String },
    /// Theme manifest parsing failed.
    InvalidManifest { name: String, detail: String },
    /// Invalid color value.
    InvalidColor { value: String },
    /// CSS processing failed.
    CssError { detail: String },
    /// Icon theme not found.
    IconThemeNotFound { name: String },
    /// GTK CSS provider error (only with `gtk` feature).
    #[cfg(feature = "gtk")]
    GtkCssProvider(gtk4::CssParserError),
}

/// Application registry error variants.
#[derive(Debug)]
pub enum AppRegistryErrorKind {
    /// Desktop entry file parsing failed.
    ParseDesktopEntry { path: PathBuf, detail: String },
    /// Desktop entry not found.
    NotFound { app_id: String },
    /// Invalid icon specified in desktop entry.
    InvalidIcon { app_id: String, icon: String },
}

/// Search engine error variants.
#[derive(Debug)]
pub enum SearchErrorKind {
    /// Index initialization failed.
    IndexInit { detail: String },
    /// Query parsing failed.
    QueryError { detail: String },
    /// Index not ready yet.
    IndexNotReady,
}

/// Session management error variants.
#[derive(Debug)]
pub enum SessionErrorKind {
    /// Cinnamon session D-Bus call failed.
    CinnamonSession(String),
    /// Lock screen integration failed.
    LockScreen(String),
    /// Power management action failed.
    PowerAction(String),
}

/// Window tracker error variants.
#[derive(Debug)]
pub enum WindowErrorKind {
    /// D-Bus proxy connection failed.
    DBusConnection(String),
    /// Window info query failed.
    QueryFailed(String),
}

/// Resource loading error variants.
#[derive(Debug)]
pub enum ResourceErrorKind {
    /// File not found in search paths.
    NotFound { path: PathBuf, searched: Vec<PathBuf> },
    /// Unsupported format.
    UnsupportedFormat { path: PathBuf, format: String },
    /// Failed to decode resource.
    Decode { path: PathBuf, detail: String },
}

/// Service lifecycle error variants.
#[derive(Debug)]
pub enum ServiceErrorKind {
    /// Service not found in registry.
    NotFound { name: String },
    /// Service already registered.
    AlreadyRegistered { name: String },
    /// Service failed to initialize.
    InitFailed { name: String, detail: String },
    /// Service failed to start.
    StartFailed { name: String, detail: String },
    /// Dependency cycle detected.
    DependencyCycle { names: Vec<String> },
    /// Dependency not satisfied.
    UnsatisfiedDependency { service: String, depends_on: String },
    /// Service in invalid state for requested operation.
    InvalidState { name: String, expected: String, actual: String },
}

/// Serialization/deserialization error variants.
#[derive(Debug)]
pub enum SerializationErrorKind {
    /// TOML serialization failed.
    TomlSerialize(String),
    /// TOML deserialization failed.
    TomlDeserialize(String),
    /// JSON serialization failed.
    JsonSerialize(String),
    /// JSON deserialization failed.
    JsonDeserialize(String),
}

// ── Display implementations ──────────────────────────────────────

impl fmt::Display for EduError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EduError::Config(e) => write!(f, "Config error: {e}"),
            EduError::Io(e) => write!(f, "I/O error: {e}"),
            EduError::Theme(e) => write!(f, "Theme error: {e}"),
            EduError::AppRegistry(e) => write!(f, "App registry error: {e}"),
            EduError::Search(e) => write!(f, "Search error: {e}"),
            EduError::Session(e) => write!(f, "Session error: {e}"),
            EduError::Window(e) => write!(f, "Window error: {e}"),
            EduError::Resource(e) => write!(f, "Resource error: {e}"),
            EduError::Service(e) => write!(f, "Service error: {e}"),
            EduError::Serialization(e) => write!(f, "Serialization error: {e}"),
            #[cfg(feature = "dbus")]
            EduError::DBus(e) => write!(f, "D-Bus error: {e}"),
            EduError::Unknown(e) => write!(f, "Unknown error: {e}"),
        }
    }
}

impl fmt::Display for ConfigErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { path, detail } => {
                write!(f, "failed to parse {path:?}: {detail}")
            }
            Self::Migrate { from, to, detail } => {
                write!(f, "migration {from} → {to} failed: {detail}")
            }
            Self::Write { path, source } => {
                write!(f, "failed to write {path:?}: {source}")
            }
            Self::Validate { field, reason } => {
                write!(f, "validation failed for `{field}`: {reason}")
            }
            Self::NotFound { path } => {
                write!(f, "not found at {path:?}")
            }
            Self::Watch { path, source } => {
                write!(f, "failed to watch {path:?}: {source}")
            }
        }
    }
}

impl fmt::Display for ThemeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { name } => write!(f, "theme `{name}` not found"),
            Self::InvalidManifest { name, detail } => {
                write!(f, "theme `{name}` manifest invalid: {detail}")
            }
            Self::InvalidColor { value } => write!(f, "invalid color `{value}`"),
            Self::CssError { detail } => write!(f, "CSS error: {detail}"),
            Self::IconThemeNotFound { name } => write!(f, "icon theme `{name}` not found"),
            #[cfg(feature = "gtk")]
            Self::GtkCssProvider(e) => write!(f, "GTK CSS provider error: {e:?}"),
        }
    }
}

impl fmt::Display for AppRegistryErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseDesktopEntry { path, detail } => {
                write!(f, "failed to parse desktop entry {path:?}: {detail}")
            }
            Self::NotFound { app_id } => write!(f, "application `{app_id}` not found"),
            Self::InvalidIcon { app_id, icon } => {
                write!(f, "invalid icon `{icon}` for `{app_id}`")
            }
        }
    }
}

impl fmt::Display for SearchErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexInit { detail } => write!(f, "search index init failed: {detail}"),
            Self::QueryError { detail } => write!(f, "search query error: {detail}"),
            Self::IndexNotReady => write!(f, "search index not ready"),
        }
    }
}

impl fmt::Display for SessionErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CinnamonSession(e) => write!(f, "cinnamon session error: {e}"),
            Self::LockScreen(e) => write!(f, "lock screen error: {e}"),
            Self::PowerAction(e) => write!(f, "power action error: {e}"),
        }
    }
}

impl fmt::Display for WindowErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DBusConnection(e) => write!(f, "D-Bus connection error: {e}"),
            Self::QueryFailed(e) => write!(f, "window query failed: {e}"),
        }
    }
}

impl fmt::Display for ResourceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { path, searched } => {
                write!(f, "{path:?} not found (searched: {searched:?})")
            }
            Self::UnsupportedFormat { path, format } => {
                write!(f, "unsupported format `{format}` for {path:?}")
            }
            Self::Decode { path, detail } => {
                write!(f, "failed to decode {path:?}: {detail}")
            }
        }
    }
}

impl fmt::Display for ServiceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { name } => write!(f, "service `{name}` not found"),
            Self::AlreadyRegistered { name } => {
                write!(f, "service `{name}` already registered")
            }
            Self::InitFailed { name, detail } => {
                write!(f, "service `{name}` init failed: {detail}")
            }
            Self::StartFailed { name, detail } => {
                write!(f, "service `{name}` start failed: {detail}")
            }
            Self::DependencyCycle { names } => {
                write!(
                    f,
                    "dependency cycle detected: {}",
                    names.join(" → ")
                )
            }
            Self::UnsatisfiedDependency { service, depends_on } => {
                write!(
                    f,
                    "service `{service}` depends on `{depends_on}` which is not registered"
                )
            }
            Self::InvalidState { name, expected, actual } => {
                write!(
                    f,
                    "service `{name}` expected state `{expected}` but was `{actual}`"
                )
            }
        }
    }
}

impl fmt::Display for SerializationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TomlSerialize(e) => write!(f, "TOML serialize: {e}"),
            Self::TomlDeserialize(e) => write!(f, "TOML deserialize: {e}"),
            Self::JsonSerialize(e) => write!(f, "JSON serialize: {e}"),
            Self::JsonDeserialize(e) => write!(f, "JSON deserialize: {e}"),
        }
    }
}

// ── std::error::Error impl ──────────────────────────────────────

impl std::error::Error for EduError {}

// ── From impls ──────────────────────────────────────────────────

impl From<std::io::Error> for EduError {
    fn from(e: std::io::Error) -> Self {
        EduError::Io(e)
    }
}

impl From<ConfigErrorKind> for EduError {
    fn from(e: ConfigErrorKind) -> Self {
        EduError::Config(e)
    }
}

impl From<ThemeErrorKind> for EduError {
    fn from(e: ThemeErrorKind) -> Self {
        EduError::Theme(e)
    }
}

impl From<AppRegistryErrorKind> for EduError {
    fn from(e: AppRegistryErrorKind) -> Self {
        EduError::AppRegistry(e)
    }
}

impl From<SearchErrorKind> for EduError {
    fn from(e: SearchErrorKind) -> Self {
        EduError::Search(e)
    }
}

impl From<SessionErrorKind> for EduError {
    fn from(e: SessionErrorKind) -> Self {
        EduError::Session(e)
    }
}

impl From<WindowErrorKind> for EduError {
    fn from(e: WindowErrorKind) -> Self {
        EduError::Window(e)
    }
}

impl From<ResourceErrorKind> for EduError {
    fn from(e: ResourceErrorKind) -> Self {
        EduError::Resource(e)
    }
}

impl From<ServiceErrorKind> for EduError {
    fn from(e: ServiceErrorKind) -> Self {
        EduError::Service(e)
    }
}

impl From<SerializationErrorKind> for EduError {
    fn from(e: SerializationErrorKind) -> Self {
        EduError::Serialization(e)
    }
}

impl From<toml::ser::Error> for EduError {
    fn from(e: toml::ser::Error) -> Self {
        EduError::Serialization(SerializationErrorKind::TomlSerialize(e.to_string()))
    }
}

impl From<toml::de::Error> for EduError {
    fn from(e: toml::de::Error) -> Self {
        EduError::Serialization(SerializationErrorKind::TomlDeserialize(e.to_string()))
    }
}

// ── Result type ─────────────────────────────────────────────────

/// Convenience result alias for EduShell operations.
pub type EduResult<T> = Result<T, EduError>;

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = EduError::Config(ConfigErrorKind::NotFound {
            path: PathBuf::from("/tmp/config.toml"),
        });
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let edu_err: EduError = io_err.into();
        assert!(matches!(edu_err, EduError::Io(_)));
    }
}
