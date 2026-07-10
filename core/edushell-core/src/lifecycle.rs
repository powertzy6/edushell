// SPDX-License-Identifier: GPL-3.0-or-later

//! # Lifecycle Management
//!
//! Defines the standard lifecycle states and transitions
//! for all EduShell components. Every module follows
//! this lifecycle to ensure predictable behaviour.

use std::fmt;

/// Standard lifecycle state for EduShell components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleState {
    /// Component created but not yet initialized.
    Uninitialized,
    /// Initialization in progress.
    Initializing,
    /// Configuration loaded.
    Configured,
    /// Services registered.
    Registered,
    /// Resources loaded.
    ResourceLoaded,
    /// Component is ready and running.
    Ready,
    /// Component is active and processing.
    Running,
    /// Component suspended (power management).
    Suspended,
    /// Shutdown in progress.
    ShuttingDown,
    /// Component has been shut down.
    Shutdown,
    /// Component encountered a fatal error.
    Error,
}

impl LifecycleState {
    /// Check if this state allows transition to `target`.
    ///
    /// Returns `Ok(())` if the transition is valid.
    pub fn can_transition_to(&self, target: LifecycleState) -> Result<(), LifecycleError> {
        use LifecycleState::*;

        let valid = match (*self, target) {
            (Uninitialized, Initializing) => true,
            (Initializing, Configured) => true,
            (Initializing, Error) => true,
            (Configured, Registered) => true,
            (Configured, Error) => true,
            (Registered, ResourceLoaded) => true,
            (Registered, Error) => true,
            (ResourceLoaded, Ready) => true,
            (ResourceLoaded, Error) => true,
            (Ready, Running) => true,
            (Running, Suspended) => true,
            (Running, Ready) => true, // pause
            (Running, ShuttingDown) => true,
            (Running, Error) => true,
            (Suspended, Running) => true, // resume
            (ShuttingDown, Shutdown) => true,
            (ShuttingDown, Error) => true,
            (Error, Initializing) => true, // retry
            (Shutdown, Uninitialized) => true,
            _ => false,
        };

        if valid {
            Ok(())
        } else {
            Err(LifecycleError::InvalidTransition {
                from: *self,
                to: target,
            })
        }
    }

    /// Returns `true` if the component is in a runnable state.
    pub fn is_runnable(&self) -> bool {
        matches!(self, LifecycleState::Ready | LifecycleState::Running)
    }

    /// Returns `true` if the component has been shut down.
    pub fn is_shutdown(&self) -> bool {
        matches!(self, LifecycleState::Shutdown)
    }

    /// Returns `true` if the component is in an error state.
    pub fn is_error(&self) -> bool {
        matches!(self, LifecycleState::Error)
    }
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uninitialized => write!(f, "uninitialized"),
            Self::Initializing => write!(f, "initializing"),
            Self::Configured => write!(f, "configured"),
            Self::Registered => write!(f, "registered"),
            Self::ResourceLoaded => write!(f, "resource-loaded"),
            Self::Ready => write!(f, "ready"),
            Self::Running => write!(f, "running"),
            Self::Suspended => write!(f, "suspended"),
            Self::ShuttingDown => write!(f, "shutting-down"),
            Self::Shutdown => write!(f, "shutdown"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// Error type for lifecycle state transitions.
#[derive(Debug, Clone)]
pub enum LifecycleError {
    /// Transition not allowed between the given states.
    InvalidTransition { from: LifecycleState, to: LifecycleState },
    /// The operation requires a different state.
    InvalidState { required: LifecycleState, actual: LifecycleState },
}

impl fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { from, to } => {
                write!(f, "cannot transition from {from} to {to}")
            }
            Self::InvalidState { required, actual } => {
                write!(f, "required state {required} but was {actual}")
            }
        }
    }
}

impl std::error::Error for LifecycleError {}

/// Trait for components that follow the standard lifecycle.
pub trait Lifecycle: Send + Sync {
    /// Get the current lifecycle state.
    fn state(&self) -> LifecycleState;

    /// Initialize the component (loads config, registers resources).
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Load configuration.
    fn load_config(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Register services or hooks.
    fn register(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Load resources (themes, icons, sounds).
    fn load_resources(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Make the component ready for operation.
    fn ready(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Start the component (begin processing).
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Suspend the component (power management).
    fn suspend(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Resume from suspension.
    fn resume(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Shutdown the component gracefully.
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Clean up resources.
    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(
            LifecycleState::Uninitialized
                .can_transition_to(LifecycleState::Initializing)
                .is_ok()
        );
        assert!(
            LifecycleState::Running
                .can_transition_to(LifecycleState::ShuttingDown)
                .is_ok()
        );
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(
            LifecycleState::Uninitialized
                .can_transition_to(LifecycleState::Shutdown)
                .is_err()
        );
        assert!(
            LifecycleState::Running
                .can_transition_to(LifecycleState::Uninitialized)
                .is_err()
        );
    }

    #[test]
    fn test_error_retry() {
        assert!(
            LifecycleState::Error
                .can_transition_to(LifecycleState::Initializing)
                .is_ok()
        );
    }

    #[test]
    fn test_is_runnable() {
        assert!(LifecycleState::Ready.is_runnable());
        assert!(LifecycleState::Running.is_runnable());
        assert!(!LifecycleState::Shutdown.is_runnable());
    }
}
