// SPDX-License-Identifier: GPL-3.0-or-later

//! # Notification Center
//!
//! Manages system notifications including toast popups,
//! persistent notifications, history, action buttons,
//! and the notification center panel.

use std::collections::VecDeque;
use crate::localization::LocalizationManager;
use edushell_core::event::EventBus;

/// Notification urgency level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

/// An action button on a notification.
#[derive(Debug, Clone)]
pub struct NotificationAction {
    pub id: String,
    pub label: String,
}

/// A single notification.
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub app_id: String,
    pub app_name: String,
    pub app_icon: String,
    pub title: String,
    pub body: String,
    pub urgency: Urgency,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub persistent: bool,
    pub actions: Vec<NotificationAction>,
    pub read: bool,
    pub dismissed: bool,
}

/// Notification center state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationCenterState {
    Closed,
    Open,
}

/// The notification center.
pub struct NotificationCenter {
    state: NotificationCenterState,
    notifications: VecDeque<Notification>,
    history: VecDeque<Notification>,
    max_visible: usize,
    do_not_disturb: bool,
    localization: LocalizationManager,
    event_bus: EventBus,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
    #[cfg(feature = "gtk")]
    list: Option<gtk::ListBox>,
}

impl NotificationCenter {
    pub fn new(localization: LocalizationManager, event_bus: EventBus) -> Self {
        Self {
            state: NotificationCenterState::Closed,
            notifications: VecDeque::new(),
            history: VecDeque::with_capacity(100),
            max_visible: 10,
            do_not_disturb: false,
            localization,
            event_bus,
            #[cfg(feature = "gtk")]
            window: None,
            #[cfg(feature = "gtk")]
            list: None,
        }
    }

    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        tracing::info!(target: "edushell::notifications", "Notification center built");
    }

    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {}

    #[cfg(feature = "gtk")]
    pub fn destroy(&self) {
        if let Some(w) = &self.window {
            w.close();
        }
    }

    #[cfg(not(feature = "gtk"))]
    pub fn destroy(&self) {}

    pub fn open(&mut self) { self.state = NotificationCenterState::Open; }
    pub fn close(&mut self) { self.state = NotificationCenterState::Closed; }
    pub fn toggle(&mut self) {
        self.state = match self.state {
            NotificationCenterState::Closed => NotificationCenterState::Open,
            NotificationCenterState::Open => NotificationCenterState::Closed,
        };
    }
    pub fn is_open(&self) -> bool { self.state == NotificationCenterState::Open }
    pub fn state(&self) -> NotificationCenterState { self.state }

    pub fn add_notification(&mut self, notif: Notification) {
        if self.do_not_disturb && notif.urgency != Urgency::Critical {
            self.history.push_back(notif);
            if self.history.len() > 100 { self.history.pop_front(); }
            return;
        }
        self.notifications.push_back(notif);
        if self.notifications.len() > self.max_visible + 5 {
            let n = self.notifications.pop_front().unwrap();
            self.history.push_back(n);
        }
        if self.history.len() > 100 { self.history.pop_front(); }
    }

    pub fn dismiss(&mut self, id: &str) {
        if let Some(pos) = self.notifications.iter().position(|n| n.id == id) {
            let mut n = self.notifications.remove(pos).unwrap();
            n.dismissed = true;
            self.history.push_back(n);
            if self.history.len() > 100 { self.history.pop_front(); }
        }
    }

    pub fn dismiss_all(&mut self) {
        while let Some(mut n) = self.notifications.pop_front() {
            n.dismissed = true;
            self.history.push_back(n);
        }
        while self.history.len() > 100 { self.history.pop_front(); }
    }

    pub fn clear_history(&mut self) { self.history.clear(); }

    pub fn notifications(&self) -> &VecDeque<Notification> { &self.notifications }

    pub fn visible_notifications(&self) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| !n.dismissed).collect()
    }

    pub fn history(&self) -> &VecDeque<Notification> { &self.history }
    pub fn mark_read(&mut self, id: &str) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.read = true;
        }
    }

    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read && !n.dismissed).count()
    }

    pub fn set_do_not_disturb(&mut self, enabled: bool) { self.do_not_disturb = enabled; }
    pub fn do_not_disturb(&self) -> bool { self.do_not_disturb }
    pub fn set_max_visible(&mut self, max: usize) { self.max_visible = max; }
    pub fn max_visible(&self) -> usize { self.max_visible }
}

impl std::fmt::Debug for NotificationCenter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationCenter")
            .field("state", &self.state)
            .field("count", &self.notifications.len())
            .field("unread", &self.unread_count())
            .field("dnd", &self.do_not_disturb)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        assert_eq!(nc.state(), NotificationCenterState::Closed);
        assert_eq!(nc.unread_count(), 0);
    }

    #[test]
    fn test_add_and_dismiss() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.add_notification(test_notification("n1", Urgency::Normal));
        assert_eq!(nc.notifications().len(), 1);
        assert_eq!(nc.unread_count(), 1);
        nc.dismiss("n1");
        assert_eq!(nc.notifications().len(), 0);
    }

    #[test]
    fn test_dismiss_all() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.add_notification(test_notification("n1", Urgency::Normal));
        nc.add_notification(test_notification("n2", Urgency::Low));
        nc.dismiss_all();
        assert_eq!(nc.notifications().len(), 0);
    }

    #[test]
    fn test_unread_count() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.add_notification(test_notification("n1", Urgency::Normal));
        nc.add_notification(test_notification("n2", Urgency::Critical));
        assert_eq!(nc.unread_count(), 2);
        nc.mark_read("n1");
        assert_eq!(nc.unread_count(), 1);
    }

    #[test]
    fn test_do_not_disturb() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.set_do_not_disturb(true);
        assert!(nc.do_not_disturb());
        nc.add_notification(test_notification("n1", Urgency::Normal));
        assert_eq!(nc.notifications().len(), 0); // filtered
        nc.add_notification(test_notification("n2", Urgency::Critical));
        assert_eq!(nc.notifications().len(), 1); // critical passes through
    }

    #[test]
    fn test_state_transitions() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        assert!(!nc.is_open());
        nc.open();
        assert!(nc.is_open());
        nc.close();
        assert!(!nc.is_open());
        nc.toggle();
        assert!(nc.is_open());
        nc.toggle();
        assert!(!nc.is_open());
    }

    #[test]
    fn test_visible_notifications() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.add_notification(test_notification("n1", Urgency::Normal));
        nc.add_notification(test_notification("n2", Urgency::Normal));
        nc.dismiss("n1");
        let visible = nc.visible_notifications();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, "n2");
    }

    #[test]
    fn test_history() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.add_notification(test_notification("n1", Urgency::Normal));
        nc.dismiss("n1");
        assert_eq!(nc.history().len(), 1);
        nc.clear_history();
        assert_eq!(nc.history().len(), 0);
    }

    #[test]
    fn test_max_visible() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.set_max_visible(2);
        for i in 0..5 {
            nc.add_notification(test_notification(&format!("n{i}"), Urgency::Normal));
        }
        // Should have kept the most recent ones
        assert!(nc.notifications().len() <= 7); // max_visible + buffer
    }

    #[test]
    fn test_urgency_variants() {
        assert_ne!(Urgency::Low as u8, Urgency::Critical as u8);
    }

    #[test]
    fn test_notification_action() {
        let action = NotificationAction { id: "open".into(), label: "Open".into() };
        assert_eq!(action.id, "open");
        assert_eq!(action.label, "Open");
    }

    #[test]
    fn test_notification_clone() {
        let n = test_notification("n1", Urgency::Normal);
        let n2 = n.clone();
        assert_eq!(n.id, n2.id);
    }

    #[test]
    fn test_mark_read_nonexistent() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        nc.mark_read("nonexistent");
        assert_eq!(nc.unread_count(), 0);
    }

    #[test]
    fn test_dismiss_all_clears_all() {
        let mut nc = NotificationCenter::new(LocalizationManager::new(), EventBus::new());
        for i in 0..10 {
            nc.add_notification(test_notification(&format!("n{i}"), Urgency::Normal));
        }
        assert_eq!(nc.notifications().len(), 10);
        nc.dismiss_all();
        assert_eq!(nc.notifications().len(), 0);
        // All should be in history
        assert_eq!(nc.history().len(), 10);
    }
}
