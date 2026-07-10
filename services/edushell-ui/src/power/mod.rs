// SPDX-License-Identifier: GPL-3.0-or-later

//! # Power Menu
//!
//! Power/shutdown menu with options to Lock, Log Out,
//! Suspend, Hibernate, Restart, and Shut Down.
//! Includes confirmation dialogs for destructive actions.

use crate::localization::LocalizationManager;

/// Power action type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerAction {
    Lock,
    Logout,
    Suspend,
    Hibernate,
    Restart,
    Shutdown,
    SwitchUser,
}

/// Power menu state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMenuState {
    Closed,
    Open,
    Confirming(PowerAction),
}

/// Power menu component.
pub struct PowerMenu {
    state: PowerMenuState,
    localization: LocalizationManager,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
    #[cfg(feature = "gtk")]
    confirm_dialog: Option<gtk::MessageDialog>,
}

impl PowerMenu {
    pub fn new(localization: LocalizationManager) -> Self {
        Self {
            state: PowerMenuState::Closed,
            localization,
            #[cfg(feature = "gtk")]
            window: None,
            #[cfg(feature = "gtk")]
            confirm_dialog: None,
        }
    }

    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        tracing::info!(target: "edushell::power", "Power menu built");
    }

    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {}

    #[cfg(feature = "gtk")]
    pub fn destroy(&self) {
        if let Some(w) = &self.window { w.close(); }
        if let Some(d) = &self.confirm_dialog { d.close(); }
    }

    #[cfg(not(feature = "gtk"))]
    pub fn destroy(&self) {}

    pub fn open(&mut self) { self.state = PowerMenuState::Open; }
    pub fn close(&mut self) { self.state = PowerMenuState::Closed; }
    pub fn toggle(&mut self) {
        self.state = match self.state {
            PowerMenuState::Closed => PowerMenuState::Open,
            _ => PowerMenuState::Closed,
        };
    }
    pub fn is_open(&self) -> bool { self.state != PowerMenuState::Closed }
    pub fn state(&self) -> PowerMenuState { self.state }

    /// Initiate a power action. Returns true if confirmation is needed.
    pub fn execute(&mut self, action: PowerAction) -> bool {
        match action {
            PowerAction::Lock | PowerAction::Suspend
            | PowerAction::Hibernate | PowerAction::SwitchUser => {
                // No confirmation needed
                self.state = PowerMenuState::Closed;
                tracing::info!(target: "edushell::power", "Executing: {action:?}");
                false
            }
            PowerAction::Logout | PowerAction::Restart | PowerAction::Shutdown => {
                // Confirmation needed
                self.state = PowerMenuState::Confirming(action);
                true
            }
        }
    }

    /// Confirm a pending power action.
    pub fn confirm(&mut self) -> Option<PowerAction> {
        if let PowerMenuState::Confirming(action) = self.state {
            self.state = PowerMenuState::Closed;
            tracing::info!(target: "edushell::power", "Confirmed: {action:?}");
            Some(action)
        } else {
            None
        }
    }

    /// Cancel a pending confirmation.
    pub fn cancel(&mut self) {
        if let PowerMenuState::Confirming(_) = self.state {
            self.state = PowerMenuState::Open;
            tracing::info!(target: "edushell::power", "Cancelled power action");
        }
    }

    /// Get the action pending confirmation, if any.
    pub fn pending_action(&self) -> Option<PowerAction> {
        if let PowerMenuState::Confirming(action) = self.state {
            Some(action)
        } else {
            None
        }
    }

    /// Get the confirmation message for an action.
    pub fn confirmation_message(&self, action: PowerAction) -> String {
        match action {
            PowerAction::Shutdown => self.localization.translate("confirm_shutdown"),
            PowerAction::Restart => self.localization.translate("confirm_restart"),
            PowerAction::Logout => "Are you sure you want to log out?".to_string(),
            _ => String::new(),
        }
    }

    pub fn action_label(&self, action: PowerAction) -> String {
        self.localization.translate(match action {
            PowerAction::Lock => "lock",
            PowerAction::Logout => "logout",
            PowerAction::Suspend => "suspend",
            PowerAction::Hibernate => "hibernate",
            PowerAction::Restart => "restart",
            PowerAction::Shutdown => "shutdown",
            PowerAction::SwitchUser => "switch_user",
        })
    }

    /// Get all available power actions.
    pub fn available_actions() -> Vec<PowerAction> {
        vec![
            PowerAction::Lock,
            PowerAction::SwitchUser,
            PowerAction::Logout,
            PowerAction::Suspend,
            PowerAction::Hibernate,
            PowerAction::Restart,
            PowerAction::Shutdown,
        ]
    }
}

impl std::fmt::Debug for PowerMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PowerMenu")
            .field("state", &self.state)
            .field("pending", &self.pending_action())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_menu() -> PowerMenu {
        PowerMenu::new(LocalizationManager::new())
    }

    #[test]
    fn test_creation() {
        let pm = test_menu();
        assert_eq!(pm.state(), PowerMenuState::Closed);
        assert!(!pm.is_open());
    }

    #[test]
    fn test_state_transitions() {
        let mut pm = test_menu();
        pm.open();
        assert!(pm.is_open());
        pm.close();
        assert!(!pm.is_open());
        pm.toggle();
        assert!(pm.is_open());
        pm.toggle();
        assert!(!pm.is_open());
    }

    #[test]
    fn test_lock_no_confirmation() {
        let mut pm = test_menu();
        let needs_confirm = pm.execute(PowerAction::Lock);
        assert!(!needs_confirm);
        assert!(!pm.is_open());
    }

    #[test]
    fn test_shutdown_needs_confirmation() {
        let mut pm = test_menu();
        let needs_confirm = pm.execute(PowerAction::Shutdown);
        assert!(needs_confirm);
        assert!(pm.pending_action() == Some(PowerAction::Shutdown));
    }

    #[test]
    fn test_confirm_action() {
        let mut pm = test_menu();
        pm.execute(PowerAction::Restart);
        let action = pm.confirm();
        assert_eq!(action, Some(PowerAction::Restart));
    }

    #[test]
    fn test_cancel_action() {
        let mut pm = test_menu();
        pm.execute(PowerAction::Shutdown);
        pm.cancel();
        assert!(pm.pending_action().is_none());
        assert!(pm.is_open());
    }

    #[test]
    fn test_cancel_when_not_confirming() {
        let mut pm = test_menu();
        pm.cancel(); // should not panic
        assert!(!pm.is_open());
    }

    #[test]
    fn test_confirm_when_not_confirming() {
        let mut pm = test_menu();
        let action = pm.confirm();
        assert!(action.is_none());
    }

    #[test]
    fn test_confirmation_message() {
        let pm = test_menu();
        let msg = pm.confirmation_message(PowerAction::Shutdown);
        assert!(!msg.is_empty());
        let empty = pm.confirmation_message(PowerAction::Lock);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_action_labels() {
        let pm = test_menu();
        assert_eq!(pm.action_label(PowerAction::Shutdown), "Shut Down");
        assert_eq!(pm.action_label(PowerAction::Lock), "Lock Screen");
    }

    #[test]
    fn test_available_actions() {
        let actions = PowerMenu::available_actions();
        assert!(actions.contains(&PowerAction::Shutdown));
        assert!(actions.contains(&PowerAction::Lock));
        assert!(actions.contains(&PowerAction::Restart));
        assert_eq!(actions.len(), 7);
    }

    #[test]
    fn test_execute_logout() {
        let mut pm = test_menu();
        assert!(pm.execute(PowerAction::Logout));
    }

    #[test]
    fn test_execute_suspend() {
        let mut pm = test_menu();
        assert!(!pm.execute(PowerAction::Suspend));
    }

    #[test]
    fn test_execute_hibernate() {
        let mut pm = test_menu();
        assert!(!pm.execute(PowerAction::Hibernate));
    }

    #[test]
    fn test_execute_switch_user() {
        let mut pm = test_menu();
        assert!(!pm.execute(PowerAction::SwitchUser));
    }

    #[test]
    fn test_debug_format() {
        let pm = test_menu();
        let debug = format!("{pm:?}");
        assert!(debug.contains("PowerMenu"));
    }
}
