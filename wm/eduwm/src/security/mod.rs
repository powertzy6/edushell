use crate::core::WindowId;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Paranoid,
}

#[derive(Debug, Clone)]
pub enum SecurityEvent {
    WindowAccess {
        source: WindowId,
        target: WindowId,
        granted: bool,
    },
    ClipboardRead {
        window: WindowId,
        granted: bool,
    },
    ClipboardWrite {
        window: WindowId,
        granted: bool,
    },
    ScreenCapture {
        window: WindowId,
        granted: bool,
    },
    Screenshot {
        window: WindowId,
        granted: bool,
    },
    InputInject {
        source: WindowId,
        target: WindowId,
        granted: bool,
    },
    IPCAccess {
        source: String,
        target: String,
        granted: bool,
    },
    PermissionDenied {
        window: WindowId,
        action: String,
    },
}

#[derive(Debug, Clone)]
pub struct SecurityAuditEntry {
    pub timestamp: String,
    pub event: SecurityEvent,
    pub severity: String,
    pub details: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    ClipboardRead,
    ClipboardWrite,
    ScreenCapture,
    Screenshot,
    InputInject,
    IPC,
    FileAccess,
    NetworkAccess,
    Notifications,
}

#[derive(Debug, Clone)]
pub struct PermissionGrant {
    pub window_id: WindowId,
    pub permission: Permission,
    pub granted: bool,
    pub temporary: bool,
    pub expires_at: Option<u64>,
}

pub struct SecurityManager {
    level: SecurityLevel,
    audit_log: Vec<SecurityAuditEntry>,
    max_log: usize,
    permissions: Vec<PermissionGrant>,
    isolated_windows: HashSet<WindowId>,
    trusted_applications: Vec<String>,
    sandbox_enabled: bool,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            level: SecurityLevel::Medium,
            audit_log: Vec::new(),
            max_log: 1000,
            permissions: Vec::new(),
            isolated_windows: HashSet::new(),
            trusted_applications: Vec::new(),
            sandbox_enabled: true,
        }
    }

    pub fn set_level(&mut self, level: SecurityLevel) {
        self.level = level;
    }

    pub fn get_level(&self) -> &SecurityLevel {
        &self.level
    }

    pub fn request_permission(
        &mut self,
        window_id: WindowId,
        permission: Permission,
        temporary: bool,
    ) -> bool {
        let allowed = match self.level {
            SecurityLevel::Low => true,
            SecurityLevel::Medium => !self.is_isolated(window_id),
            SecurityLevel::High => {
                if self.is_isolated(window_id) {
                    return false;
                }
                !matches!(
                    permission,
                    Permission::ScreenCapture | Permission::Screenshot | Permission::InputInject
                )
            }
            SecurityLevel::Paranoid => {
                if self.is_isolated(window_id) {
                    return false;
                }
                matches!(
                    permission,
                    Permission::ClipboardRead
                        | Permission::ClipboardWrite
                        | Permission::Notifications
                )
            }
        };

        let grant = PermissionGrant {
            window_id,
            permission,
            granted: allowed,
            temporary,
            expires_at: if temporary {
                Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        + 3600,
                )
            } else {
                None
            },
        };
        self.permissions.push(grant);

        let event = match permission {
            Permission::ClipboardRead => SecurityEvent::ClipboardRead {
                window: window_id,
                granted: allowed,
            },
            Permission::ClipboardWrite => SecurityEvent::ClipboardWrite {
                window: window_id,
                granted: allowed,
            },
            Permission::ScreenCapture => SecurityEvent::ScreenCapture {
                window: window_id,
                granted: allowed,
            },
            Permission::Screenshot => SecurityEvent::Screenshot {
                window: window_id,
                granted: allowed,
            },
            Permission::InputInject => SecurityEvent::InputInject {
                source: window_id,
                target: window_id,
                granted: allowed,
            },
            Permission::IPC => SecurityEvent::IPCAccess {
                source: String::new(),
                target: String::new(),
                granted: allowed,
            },
            _ => return allowed,
        };
        self.log_event(event, if allowed { "info" } else { "warning" });

        allowed
    }

    pub fn revoke_permission(&mut self, window_id: WindowId, permission: Permission) -> bool {
        let before = self.permissions.len();
        self.permissions
            .retain(|p| !(p.window_id == window_id && p.permission == permission));
        before != self.permissions.len()
    }

    pub fn check_permission(&self, window_id: WindowId, permission: Permission) -> bool {
        self.permissions
            .iter()
            .any(|p| p.window_id == window_id && p.permission == permission && p.granted)
    }

    pub fn list_permissions(&self, window_id: WindowId) -> Vec<&PermissionGrant> {
        self.permissions
            .iter()
            .filter(|p| p.window_id == window_id)
            .collect()
    }

    pub fn isolate_window(&mut self, window_id: WindowId) -> bool {
        self.isolated_windows.insert(window_id)
    }

    pub fn unisolate_window(&mut self, window_id: WindowId) -> bool {
        self.isolated_windows.remove(&window_id)
    }

    pub fn is_isolated(&self, window_id: WindowId) -> bool {
        self.isolated_windows.contains(&window_id)
    }

    pub fn isolated_count(&self) -> usize {
        self.isolated_windows.len()
    }

    pub fn add_trusted(&mut self, app_id: String) {
        if !self.trusted_applications.contains(&app_id) {
            self.trusted_applications.push(app_id);
        }
    }

    pub fn remove_trusted(&mut self, app_id: &str) -> bool {
        let before = self.trusted_applications.len();
        self.trusted_applications.retain(|a| a != app_id);
        before != self.trusted_applications.len()
    }

    pub fn is_trusted(&self, app_id: &str) -> bool {
        self.trusted_applications.iter().any(|a| a == app_id)
    }

    pub fn check_clipboard_access(&self, _src: WindowId, _dst: WindowId) -> bool {
        match self.level {
            SecurityLevel::Low => true,
            SecurityLevel::Medium => true,
            SecurityLevel::High => false,
            SecurityLevel::Paranoid => false,
        }
    }

    pub fn check_screen_capture(&self, window_id: WindowId) -> bool {
        if self.is_isolated(window_id) {
            return false;
        }
        match self.level {
            SecurityLevel::Low => true,
            SecurityLevel::Medium => true,
            SecurityLevel::High => false,
            SecurityLevel::Paranoid => false,
        }
    }

    pub fn check_input_inject(&self, src: WindowId, dst: WindowId) -> bool {
        if self.is_isolated(src) || self.is_isolated(dst) {
            return false;
        }
        match self.level {
            SecurityLevel::Low => true,
            SecurityLevel::Medium => src == dst,
            SecurityLevel::High => false,
            SecurityLevel::Paranoid => false,
        }
    }

    pub fn check_ipc_access(&self, source: &str, target: &str) -> bool {
        if self.is_trusted(source) && self.is_trusted(target) {
            return true;
        }
        match self.level {
            SecurityLevel::Low => true,
            SecurityLevel::Medium => self.is_trusted(source),
            SecurityLevel::High => false,
            SecurityLevel::Paranoid => false,
        }
    }

    pub fn log_event(&mut self, event: SecurityEvent, severity: &str) {
        let details = match &event {
            SecurityEvent::WindowAccess {
                source,
                target,
                granted,
            } => format!(
                "Window access: {} -> {}, granted: {}",
                source, target, granted
            ),
            SecurityEvent::ClipboardRead { window, granted } => {
                format!("Clipboard read by {}: granted={}", window, granted)
            }
            SecurityEvent::ClipboardWrite { window, granted } => {
                format!("Clipboard write by {}: granted={}", window, granted)
            }
            SecurityEvent::ScreenCapture { window, granted } => {
                format!("Screen capture by {}: granted={}", window, granted)
            }
            SecurityEvent::Screenshot { window, granted } => {
                format!("Screenshot by {}: granted={}", window, granted)
            }
            SecurityEvent::InputInject {
                source,
                target,
                granted,
            } => format!("Input inject {} -> {}: granted={}", source, target, granted),
            SecurityEvent::IPCAccess {
                source,
                target,
                granted,
            } => format!("IPC access {} -> {}: granted={}", source, target, granted),
            SecurityEvent::PermissionDenied { window, action } => {
                format!("Permission denied for {}: {}", window, action)
            }
        };

        let entry = SecurityAuditEntry {
            timestamp: crate::core::now(),
            event,
            severity: severity.to_string(),
            details,
        };

        if self.audit_log.len() >= self.max_log {
            self.audit_log.remove(0);
        }
        self.audit_log.push(entry);
    }

    pub fn get_audit_log(&self) -> &[SecurityAuditEntry] {
        &self.audit_log
    }

    pub fn clear_audit_log(&mut self) {
        self.audit_log.clear();
    }

    pub fn set_sandbox(&mut self, enabled: bool) {
        self.sandbox_enabled = enabled;
    }

    pub fn is_sandbox_enabled(&self) -> bool {
        self.sandbox_enabled
    }

    pub fn record_access_denied(&mut self, window_id: WindowId, action: &str) {
        self.log_event(
            SecurityEvent::PermissionDenied {
                window: window_id,
                action: action.to_string(),
            },
            "error",
        );
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_default() {
        let sm = SecurityManager::new();
        assert_eq!(*sm.get_level(), SecurityLevel::Medium);
        assert!(sm.is_sandbox_enabled());
    }

    #[test]
    fn test_permission_request_grant_deny() {
        let mut sm = SecurityManager::new();
        let wid = WindowId::new();
        assert!(sm.request_permission(wid, Permission::ClipboardRead, false));
        assert!(sm.check_permission(wid, Permission::ClipboardRead));
        assert!(!sm.check_permission(wid, Permission::ScreenCapture));
        sm.set_level(SecurityLevel::Paranoid);
        assert!(!sm.request_permission(wid, Permission::ScreenCapture, false));
    }

    #[test]
    fn test_window_isolation() {
        let mut sm = SecurityManager::new();
        let wid = WindowId::new();
        assert!(sm.isolate_window(wid));
        assert!(sm.is_isolated(wid));
        assert_eq!(sm.isolated_count(), 1);
        assert!(sm.unisolate_window(wid));
        assert!(!sm.is_isolated(wid));
        assert_eq!(sm.isolated_count(), 0);
    }

    #[test]
    fn test_trusted_applications() {
        let mut sm = SecurityManager::new();
        sm.add_trusted("org.gnome.Files".to_string());
        assert!(sm.is_trusted("org.gnome.Files"));
        assert!(!sm.is_trusted("org.malicious.App"));
        assert!(sm.remove_trusted("org.gnome.Files"));
        assert!(!sm.is_trusted("org.gnome.Files"));
    }

    #[test]
    fn test_audit_log() {
        let mut sm = SecurityManager::new();
        assert!(sm.get_audit_log().is_empty());
        let wid = WindowId::new();
        sm.request_permission(wid, Permission::ClipboardRead, false);
        assert_eq!(sm.get_audit_log().len(), 1);
        sm.clear_audit_log();
        assert_eq!(sm.get_audit_log().len(), 0);
    }

    #[test]
    fn test_clipboard_access() {
        let sm = SecurityManager::new();
        let a = WindowId::new();
        let b = WindowId::new();
        assert!(sm.check_clipboard_access(a, b));
    }

    #[test]
    fn test_screen_capture_check() {
        let mut sm = SecurityManager::new();
        let wid = WindowId::new();
        assert!(sm.check_screen_capture(wid));
        sm.isolate_window(wid);
        assert!(!sm.check_screen_capture(wid));
    }

    #[test]
    fn test_sandbox_enable_disable() {
        let mut sm = SecurityManager::new();
        assert!(sm.is_sandbox_enabled());
        sm.set_sandbox(false);
        assert!(!sm.is_sandbox_enabled());
        sm.set_sandbox(true);
        assert!(sm.is_sandbox_enabled());
    }

    #[test]
    fn test_revoke_permission() {
        let mut sm = SecurityManager::new();
        let wid = WindowId::new();
        assert!(sm.request_permission(wid, Permission::ClipboardRead, false));
        assert!(sm.check_permission(wid, Permission::ClipboardRead));
        assert!(sm.revoke_permission(wid, Permission::ClipboardRead));
        assert!(!sm.check_permission(wid, Permission::ClipboardRead));
    }
}
