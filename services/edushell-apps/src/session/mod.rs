use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SessionState {
    pub user: String,
    pub display: String,
    pub seat: String,
    pub login_time: String,
    pub session_type: SessionType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionType {
    Tty,
    X11,
    Wayland,
}

pub struct EduShellSession {
    active: bool,
    current: Option<SessionState>,
    history: Vec<SessionState>,
    environment: HashMap<String, String>,
}

impl EduShellSession {
    pub fn new() -> Self {
        Self {
            active: false,
            current: None,
            history: Vec::new(),
            environment: HashMap::new(),
        }
    }

    pub fn login(&mut self, user: &str, display: &str, seat: &str, session_type: SessionType) {
        let state = SessionState {
            user: user.to_string(),
            display: display.to_string(),
            seat: seat.to_string(),
            login_time: Utc::now().to_rfc3339(),
            session_type,
        };
        self.active = true;
        self.current = Some(state.clone());
        self.history.push(state);
    }

    pub fn logout(&mut self) {
        self.active = false;
        self.current = None;
    }

    pub fn is_active(&self) -> bool { self.active }
    pub fn current(&self) -> Option<&SessionState> { self.current.as_ref() }
    pub fn history(&self) -> &[SessionState] { &self.history }

    pub fn set_env(&mut self, key: &str, value: &str) {
        self.environment.insert(key.to_string(), value.to_string());
    }

    pub fn get_env(&self, key: &str) -> Option<&str> {
        self.environment.get(key).map(|s| s.as_str())
    }
}

impl Default for EduShellSession {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_create() {
        let s = EduShellSession::new();
        assert!(!s.is_active());
        assert!(s.current().is_none());
    }

    #[test]
    fn test_login_logout() {
        let mut s = EduShellSession::new();
        s.login("student", ":0", "seat0", SessionType::X11);
        assert!(s.is_active());
        assert_eq!(s.current().unwrap().user, "student");
        s.logout();
        assert!(!s.is_active());
    }

    #[test]
    fn test_history_tracked() {
        let mut s = EduShellSession::new();
        s.login("user1", ":0", "seat0", SessionType::Wayland);
        assert_eq!(s.history().len(), 1);
        s.logout();
        s.login("user2", ":1", "seat0", SessionType::Tty);
        assert_eq!(s.history().len(), 2);
    }

    #[test]
    fn test_environment() {
        let mut s = EduShellSession::new();
        s.set_env("DESKTOP_SESSION", "edushell");
        assert_eq!(s.get_env("DESKTOP_SESSION"), Some("edushell"));
    }
}
