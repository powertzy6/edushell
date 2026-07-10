// SPDX-License-Identifier: GPL-3.0-or-later

//! # Session Manager
//!
//! Manages the user session lifecycle: login, logout, lock,
//! suspend, and shutdown. Integrates with Cinnamon's session
//! manager via D-Bus and provides a state machine for
//! session transitions.

use std::fmt;
use std::sync::Arc;

use crate::error::{EduResult, SessionErrorKind};

/// Session state machine states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// No active session.
    Inactive,
    /// Session is starting up.
    Starting,
    /// Active session running.
    Active,
    /// Session is locked.
    Locked,
    /// Session is being suspended.
    Suspending,
    /// Session is shutting down.
    ShuttingDown,
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "inactive"),
            Self::Starting => write!(f, "starting"),
            Self::Active => write!(f, "active"),
            Self::Locked => write!(f, "locked"),
            Self::Suspending => write!(f, "suspending"),
            Self::ShuttingDown => write!(f, "shutting-down"),
        }
    }
}

impl SessionState {
    /// Check if transition to target state is valid.
    pub fn can_transition_to(&self, target: SessionState) -> bool {
        use SessionState::*;
        matches!((*self, target),
            (Inactive, Starting) |
            (Starting, Active) |
            (Starting, Inactive) |
            (Active, Locked) |
            (Active, Suspending) |
            (Active, ShuttingDown) |
            (Locked, Active) |
            (Suspending, Active) |
            (ShuttingDown, Inactive)
        )
    }
}

/// Trait for lock screen integration.
pub trait LockScreenHandle: Send + Sync {
    /// Show the lock screen.
    fn lock(&self) -> EduResult<()>;
    /// Hide the lock screen.
    fn unlock(&self) -> EduResult<()>;
    /// Check if lock screen is visible.
    fn is_locked(&self) -> bool;
}

/// Session event type.
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// User login.
    Login { username: String },
    /// User logout.
    Logout,
    /// Lock session.
    Lock,
    /// Unlock session.
    Unlock,
    /// Suspend system.
    Suspend,
    /// System resumed.
    Resume,
    /// Shutdown system.
    Shutdown,
}

/// Thread-safe session manager.
#[derive(Clone)]
pub struct SessionManager {
    state: Arc<std::sync::RwLock<SessionState>>,
    current_user: Arc<std::sync::RwLock<Option<String>>>,
    lock_screen: Arc<std::sync::RwLock<Option<Box<dyn LockScreenHandle>>>>,
}

impl SessionManager {
    /// Create a new session manager.
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::RwLock::new(SessionState::Inactive)),
            current_user: Arc::new(std::sync::RwLock::new(None)),
            lock_screen: Arc::new(std::sync::RwLock::new(None)),
        }
    }

    /// Set the lock screen handle.
    pub fn set_lock_screen(&self, handle: Box<dyn LockScreenHandle>) {
        if let Ok(mut ls) = self.lock_screen.write() {
            *ls = Some(handle);
        }
    }

    /// Get current session state.
    pub fn state(&self) -> SessionState {
        self.state.read().map(|s| *s).unwrap_or(SessionState::Inactive)
    }

    /// Get the current user.
    pub fn current_user(&self) -> Option<String> {
        self.current_user.read().ok().and_then(|u| u.clone())
    }

    /// Process a session event.
    pub fn handle_event(&self, event: SessionEvent) -> EduResult<()> {
        let current = self.state();
        let next = match &event {
            SessionEvent::Login { .. } => SessionState::Starting,
            SessionEvent::Logout => SessionState::ShuttingDown,
            SessionEvent::Lock => SessionState::Locked,
            SessionEvent::Unlock => SessionState::Active,
            SessionEvent::Suspend => SessionState::Suspending,
            SessionEvent::Resume => SessionState::Active,
            SessionEvent::Shutdown => SessionState::ShuttingDown,
        };

        if !current.can_transition_to(next) {
            return Err(SessionErrorKind::CinnamonSession(
                format!("Cannot transition from {current} to {next}")
            ).into());
        }

        match &event {
            SessionEvent::Login { username } => {
                tracing::info!(
                    target: "edushell::session",
                    user = username,
                    "User login"
                );
                if let Ok(mut user) = self.current_user.write() {
                    *user = Some(username.clone());
                }
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::Starting;
                }
                // Transition to Active
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::Active;
                }
            }
            SessionEvent::Logout => {
                tracing::info!(target: "edushell::session", "User logout");
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::ShuttingDown;
                }
                if let Ok(mut user) = self.current_user.write() {
                    user.take();
                }
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::Inactive;
                }
            }
            SessionEvent::Lock => {
                if let Ok(ls) = self.lock_screen.read() {
                    if let Some(handle) = ls.as_ref() {
                        handle.lock()?;
                    }
                }
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::Locked;
                }
            }
            SessionEvent::Unlock => {
                if let Ok(ls) = self.lock_screen.read() {
                    if let Some(handle) = ls.as_ref() {
                        handle.unlock()?;
                    }
                }
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::Active;
                }
            }
            SessionEvent::Suspend => {
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::Suspending;
                }
            }
            SessionEvent::Resume => {
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::Active;
                }
            }
            SessionEvent::Shutdown => {
                tracing::info!(target: "edushell::session", "Shutdown requested");
                if let Ok(mut state) = self.state.write() {
                    *state = SessionState::ShuttingDown;
                }
            }
        }

        Ok(())
    }

    /// Check if the session is active.
    pub fn is_active(&self) -> bool {
        self.state() == SessionState::Active
    }

    /// Check if the session is locked.
    pub fn is_locked(&self) -> bool {
        self.state() == SessionState::Locked
    }

    /// Reset session state.
    pub fn reset(&self) {
        if let Ok(mut state) = self.state.write() {
            *state = SessionState::Inactive;
        }
        if let Ok(mut user) = self.current_user.write() {
            user.take();
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let sm = SessionManager::new();
        assert_eq!(sm.state(), SessionState::Inactive);
        assert!(!sm.is_active());
    }

    #[test]
    fn test_login_logout() {
        let sm = SessionManager::new();
        sm.handle_event(SessionEvent::Login { username: "test".into() }).unwrap();
        assert!(sm.is_active());
        assert_eq!(sm.current_user(), Some("test".into()));

        sm.handle_event(SessionEvent::Logout).unwrap();
        assert_eq!(sm.state(), SessionState::Inactive);
        assert_eq!(sm.current_user(), None);
    }

    #[test]
    fn test_lock_unlock() {
        let sm = SessionManager::new();
        sm.handle_event(SessionEvent::Login { username: "test".into() }).unwrap();
        assert!(sm.is_active());

        sm.handle_event(SessionEvent::Lock).unwrap();
        assert!(sm.is_locked());

        sm.handle_event(SessionEvent::Unlock).unwrap();
        assert!(sm.is_active());
    }

    #[test]
    fn test_invalid_transition() {
        let sm = SessionManager::new();
        // Can't lock when inactive
        let result = sm.handle_event(SessionEvent::Lock);
        assert!(result.is_err());
    }

    #[test]
    fn test_suspend_resume() {
        let sm = SessionManager::new();
        sm.handle_event(SessionEvent::Login { username: "test".into() }).unwrap();

        sm.handle_event(SessionEvent::Suspend).unwrap();
        assert_eq!(sm.state(), SessionState::Suspending);

        sm.handle_event(SessionEvent::Resume).unwrap();
        assert!(sm.is_active());
    }

    #[test]
    fn test_state_display() {
        assert_eq!(SessionState::Active.to_string(), "active");
        assert_eq!(SessionState::Locked.to_string(), "locked");
        assert_eq!(SessionState::Inactive.to_string(), "inactive");
    }
}
