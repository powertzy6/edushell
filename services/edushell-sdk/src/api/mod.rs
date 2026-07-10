//! Public API — bindings for interacting with the EduShell shell.
use serde::{Deserialize, Serialize};

/// Shell API version.
pub const API_VERSION: &str = "1.0.0";

/// Shell info returned by get_shell_info().
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellInfo {
    pub version: String,
    pub api_version: String,
    pub sdk_version: String,
    pub session_type: String,
    pub locale: String,
    pub theme_mode: String,
}

/// Window state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Notification priority.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// A shell notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub priority: NotificationPriority,
    pub timestamp: String,
    pub action_id: Option<String>,
}

impl Notification {
    pub fn new(title: &str, body: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            body: body.to_string(),
            icon: None,
            priority: NotificationPriority::Normal,
            timestamp: chrono::Utc::now().to_rfc3339(),
            action_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_version() {
        assert_eq!(API_VERSION, "1.0.0");
    }

    #[test]
    fn test_notification_new() {
        let n = Notification::new("Test", "Body");
        assert_eq!(n.title, "Test");
        assert_eq!(n.body, "Body");
        assert_eq!(n.priority, NotificationPriority::Normal);
    }

    #[test]
    fn test_shell_info() {
        let info = ShellInfo {
            version: "1.0.0".into(),
            api_version: "1.0.0".into(),
            sdk_version: "1.0.0".into(),
            session_type: "wayland".into(),
            locale: "id-ID".into(),
            theme_mode: "dark".into(),
        };
        assert_eq!(info.version, "1.0.0");
    }

    #[test]
    fn test_window_state() {
        assert_eq!(format!("{:?}", WindowState::Fullscreen), "Fullscreen");
    }

    #[test]
    fn test_notification_serde() {
        let n = Notification::new("Alert", "Something happened");
        let json = serde_json::to_string(&n).unwrap();
        let d: Notification = serde_json::from_str(&json).unwrap();
        assert_eq!(d.title, "Alert");
    }
}
