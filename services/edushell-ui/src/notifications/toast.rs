// SPDX-License-Identifier: GPL-3.0-or-later

//! # Toast Notifications
//!
//! Temporary popup notifications that appear at the top
//! of the screen, auto-dismiss after a timeout, and
//! can have action buttons.

use crate::notifications::{Notification, Urgency};

/// Toast position on screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    TopCenter,
}

/// Toast display state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastState {
    Entering,
    Visible,
    Leaving,
    Gone,
}

/// A toast notification popup.
#[derive(Debug, Clone)]
pub struct Toast {
    pub notification: Notification,
    pub position: ToastPosition,
    pub state: ToastState,
    pub timeout_ms: u64,
    pub elapsed_ms: u64,
}

/// Manager for toast popups.
pub struct ToastManager {
    toasts: Vec<Toast>,
    max_visible: usize,
    default_timeout_ms: u64,
    position: ToastPosition,
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            max_visible: 5,
            default_timeout_ms: 5000,
            position: ToastPosition::TopRight,
        }
    }

    pub fn show(&mut self, notification: Notification) {
        let urgency = notification.urgency;
        let timeout = if urgency == Urgency::Critical {
            0 // never auto-dismiss
        } else {
            self.default_timeout_ms
        };

        let toast = Toast {
            notification,
            position: self.position,
            state: ToastState::Entering,
            timeout_ms: timeout,
            elapsed_ms: 0,
        };

        self.toasts.push(toast);

        if self.toasts.len() > self.max_visible {
            self.toasts.remove(0);
        }
    }

    pub fn dismiss(&mut self, id: &str) {
        if let Some(toast) = self.toasts.iter_mut().find(|t| t.notification.id == id) {
            toast.state = ToastState::Leaving;
        }
    }

    pub fn dismiss_all(&mut self) {
        for toast in &mut self.toasts {
            toast.state = ToastState::Leaving;
        }
    }

    pub fn update(&mut self, dt_ms: u64) {
        self.toasts.retain_mut(|toast| {
            match toast.state {
                ToastState::Entering => {
                    toast.elapsed_ms += dt_ms;
                    if toast.elapsed_ms > 300 {
                        toast.state = ToastState::Visible;
                        toast.elapsed_ms = 0;
                    }
                }
                ToastState::Visible => {
                    toast.elapsed_ms += dt_ms;
                    if toast.timeout_ms > 0 && toast.elapsed_ms >= toast.timeout_ms {
                        toast.state = ToastState::Leaving;
                        toast.elapsed_ms = 0;
                    }
                }
                ToastState::Leaving => {
                    toast.elapsed_ms += dt_ms;
                    if toast.elapsed_ms > 300 {
                        toast.state = ToastState::Gone;
                    }
                }
                ToastState::Gone => return false,
            }
            true
        });
    }

    pub fn toasts(&self) -> &[Toast] { &self.toasts }

    pub fn visible_toasts(&self) -> Vec<&Toast> {
        self.toasts.iter().filter(|t| t.state != ToastState::Gone).collect()
    }

    pub fn set_max_visible(&mut self, max: usize) { self.max_visible = max; }
    pub fn set_default_timeout(&mut self, ms: u64) { self.default_timeout_ms = ms; }
    pub fn set_position(&mut self, pos: ToastPosition) {
        self.position = pos;
        for toast in &mut self.toasts {
            toast.position = pos;
        }
    }
    pub fn position(&self) -> ToastPosition { self.position }

    pub fn default_timeout_for(urgency: Urgency) -> u64 {
        match urgency {
            Urgency::Low => 3000,
            Urgency::Normal => 5000,
            Urgency::Critical => 0,
        }
    }
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::{Notification, NotificationAction, Urgency};

    fn test_notification(id: &str, urgency: Urgency) -> Notification {
        Notification {
            id: id.to_string(),
            app_id: "test".into(), app_name: "Test".into(),
            app_icon: "test".into(), title: "Title".into(),
            body: "Body".into(), urgency,
            timestamp: chrono::Local::now(),
            persistent: false, actions: vec![],
            read: false, dismissed: false,
        }
    }

    #[test]
    fn test_creation() {
        let tm = ToastManager::new();
        assert_eq!(tm.toasts().len(), 0);
        assert_eq!(tm.position(), ToastPosition::TopRight);
    }

    #[test]
    fn test_show_toast() {
        let mut tm = ToastManager::new();
        tm.show(test_notification("n1", Urgency::Normal));
        assert_eq!(tm.toasts().len(), 1);
        assert_eq!(tm.toasts()[0].state, ToastState::Entering);
    }

    #[test]
    fn test_dismiss() {
        let mut tm = ToastManager::new();
        tm.show(test_notification("n1", Urgency::Normal));
        tm.dismiss("n1");
        assert_eq!(tm.toasts()[0].state, ToastState::Leaving);
    }

    #[test]
    fn test_dismiss_all() {
        let mut tm = ToastManager::new();
        tm.show(test_notification("n1", Urgency::Normal));
        tm.show(test_notification("n2", Urgency::Low));
        tm.dismiss_all();
        assert!(tm.toasts().iter().all(|t| t.state == ToastState::Leaving));
    }

    #[test]
    fn test_update_entering_to_visible() {
        let mut tm = ToastManager::new();
        tm.show(test_notification("n1", Urgency::Normal));
        tm.update(350);
        assert_eq!(tm.toasts()[0].state, ToastState::Visible);
    }

    #[test]
    fn test_auto_dismiss_after_timeout() {
        let mut tm = ToastManager::new();
        tm.set_default_timeout(100);
        tm.show(test_notification("n1", Urgency::Normal));
        tm.update(350); // entering
        tm.update(150); // visible + timeout
        assert_eq!(tm.toasts()[0].state, ToastState::Leaving);
    }

    #[test]
    fn test_critical_never_auto_dismiss() {
        let mut tm = ToastManager::new();
        tm.set_default_timeout(100);
        tm.show(test_notification("n1", Urgency::Critical));
        tm.update(350); // entering
        tm.update(5000); // long wait
        assert_eq!(tm.toasts()[0].state, ToastState::Visible);
    }

    #[test]
    fn test_max_visible() {
        let mut tm = ToastManager::new();
        tm.set_max_visible(2);
        for i in 0..5 {
            tm.show(test_notification(&format!("n{i}"), Urgency::Normal));
        }
        assert_eq!(tm.toasts().len(), 2);
    }

    #[test]
    fn test_visible_toasts() {
        let mut tm = ToastManager::new();
        tm.show(test_notification("n1", Urgency::Normal));
        tm.update(350); // visible
        tm.show(test_notification("n2", Urgency::Normal)); // entering
        assert_eq!(tm.visible_toasts().len(), 2);
        tm.dismiss("n1");
        tm.update(350); // gone
        assert_eq!(tm.visible_toasts().len(), 1);
    }

    #[test]
    fn test_set_position() {
        let mut tm = ToastManager::new();
        tm.set_position(ToastPosition::BottomLeft);
        assert_eq!(tm.position(), ToastPosition::BottomLeft);
        tm.show(test_notification("n1", Urgency::Normal));
        assert_eq!(tm.toasts()[0].position, ToastPosition::BottomLeft);
    }

    #[test]
    fn test_default_timeout_for_urgency() {
        assert_eq!(ToastManager::default_timeout_for(Urgency::Low), 3000);
        assert_eq!(ToastManager::default_timeout_for(Urgency::Normal), 5000);
        assert_eq!(ToastManager::default_timeout_for(Urgency::Critical), 0);
    }

    #[test]
    fn test_update_removes_gone() {
        let mut tm = ToastManager::new();
        tm.set_default_timeout(1);
        tm.show(test_notification("n1", Urgency::Normal));
        tm.update(400); // entering -> visible, timeout
        tm.update(400); // leaving -> gone
        assert!(tm.toasts().is_empty());
    }
}
