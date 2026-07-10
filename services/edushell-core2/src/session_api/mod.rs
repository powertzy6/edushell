//! Session API — lifecycle, authentication, user management.
use serde::{Deserialize, Serialize};

/// Session identifier.
pub type SessionId = String;

/// Session state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionState {
    LoggedOut,
    LoggingIn,
    Active,
    Locked,
    LoggingOut,
}

/// User information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub uid: u32,
    pub username: String,
    pub display_name: String,
    pub home_dir: String,
    pub shell: String,
}

/// Session info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: SessionId,
    pub user: UserInfo,
    pub state: SessionState,
    pub display_server: String,
    pub seat: String,
    pub login_time: String,
    pub session_type: String,
}

/// Session API trait.
pub trait SessionApi: Send + Sync {
    fn current_session(&self) -> Option<SessionInfo>;
    fn list_sessions(&self) -> Vec<SessionInfo>;
    fn switch_to(&self, session_id: &SessionId) -> bool;
    fn lock_current_session(&self) -> bool;
    fn logout_current_session(&self) -> bool;
}

/// Stub implementation.
pub struct StubSessionManager;

impl SessionApi for StubSessionManager {
    fn current_session(&self) -> Option<SessionInfo> {
        None
    }
    fn list_sessions(&self) -> Vec<SessionInfo> {
        Vec::new()
    }
    fn switch_to(&self, _id: &SessionId) -> bool {
        true
    }
    fn lock_current_session(&self) -> bool {
        true
    }
    fn logout_current_session(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_session() {
        let s = StubSessionManager;
        assert!(s.current_session().is_none());
        assert!(s.list_sessions().is_empty());
    }

    #[test]
    fn test_session_states() {
        assert_eq!(format!("{:?}", SessionState::Active), "Active");
        assert_eq!(format!("{:?}", SessionState::Locked), "Locked");
    }

    #[test]
    fn test_user_info() {
        let u = UserInfo {
            uid: 1000,
            username: "student".into(),
            display_name: "Student".into(),
            home_dir: "/home/student".into(),
            shell: "/bin/bash".into(),
        };
        assert_eq!(u.username, "student");
    }

    #[test]
    fn test_session_info() {
        let info = SessionInfo {
            id: "s1".into(),
            user: UserInfo {
                uid: 1000,
                username: "student".into(),
                display_name: "Student".into(),
                home_dir: "/home/student".into(),
                shell: "/bin/bash".into(),
            },
            state: SessionState::Active,
            display_server: "wayland".into(),
            seat: "seat0".into(),
            login_time: "2026-01-01T00:00:00Z".into(),
            session_type: "edushell".into(),
        };
        assert_eq!(info.display_server, "wayland");
    }
}
