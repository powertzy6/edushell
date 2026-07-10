// SPDX-License-Identifier: GPL-3.0-or-later

//! # Dock Item
//!
//! A single item in the dock representing a pinned or
//! running application. Shows icon, badge, and window
//! indicator dot.

#[derive(Debug, Clone)]
pub struct DockItem {
    pub app_id: String,
    pub icon_name: String,
    pub name: String,
    pub pinned: bool,
    pub running: bool,
    pub badge: u32,
    pub windows: u32,
    pub focused: bool,
}

impl DockItem {
    pub fn new(app_id: &str, name: &str) -> Self {
        DockItem {
            app_id: app_id.to_string(),
            icon_name: app_id.to_string(),
            name: name.to_string(),
            pinned: false,
            running: false,
            badge: 0,
            windows: 0,
            focused: false,
        }
    }

    pub fn with_icon(app_id: &str, name: &str, icon: &str) -> Self {
        DockItem {
            app_id: app_id.to_string(),
            icon_name: icon.to_string(),
            name: name.to_string(),
            pinned: false,
            running: false,
            badge: 0,
            windows: 0,
            focused: false,
        }
    }

    pub fn set_badge(&mut self, count: u32) {
        self.badge = count;
    }

    pub fn has_badge(&self) -> bool {
        self.badge > 0
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn add_window(&mut self) {
        self.windows = self.windows.saturating_add(1);
        self.running = true;
    }

    pub fn remove_window(&mut self) {
        self.windows = self.windows.saturating_sub(1);
        if self.windows == 0 {
            self.running = false;
            self.focused = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dock_item() {
        let item = DockItem::new("org.gnome.Calculator", "Calculator");
        assert_eq!(item.app_id, "org.gnome.Calculator");
        assert_eq!(item.name, "Calculator");
        assert_eq!(item.icon_name, "org.gnome.Calculator");
        assert!(!item.pinned);
        assert!(!item.running);
        assert!(!item.focused);
        assert_eq!(item.badge, 0);
        assert_eq!(item.windows, 0);
    }

    #[test]
    fn test_with_icon() {
        let item = DockItem::with_icon("firefox", "Firefox", "firefox-icon");
        assert_eq!(item.app_id, "firefox");
        assert_eq!(item.name, "Firefox");
        assert_eq!(item.icon_name, "firefox-icon");
    }

    #[test]
    fn test_set_badge_and_has_badge() {
        let mut item = DockItem::new("app", "App");
        assert!(!item.has_badge());
        item.set_badge(5);
        assert!(item.has_badge());
        assert_eq!(item.badge, 5);
        item.set_badge(0);
        assert!(!item.has_badge());
    }

    #[test]
    fn test_focus_unfocus() {
        let mut item = DockItem::new("app", "App");
        assert!(!item.focused);
        item.focus();
        assert!(item.focused);
        item.unfocus();
        assert!(!item.focused);
    }

    #[test]
    fn test_add_window() {
        let mut item = DockItem::new("app", "App");
        assert!(!item.running);
        assert_eq!(item.windows, 0);
        item.add_window();
        assert!(item.running);
        assert_eq!(item.windows, 1);
        item.add_window();
        assert_eq!(item.windows, 2);
    }

    #[test]
    fn test_remove_window() {
        let mut item = DockItem::new("app", "App");
        item.add_window();
        item.focus();
        assert!(item.running);
        assert!(item.focused);
        item.remove_window();
        assert!(!item.running);
        assert!(!item.focused);
        assert_eq!(item.windows, 0);
    }

    #[test]
    fn test_remove_window_multiple() {
        let mut item = DockItem::new("app", "App");
        item.add_window();
        item.add_window();
        assert_eq!(item.windows, 2);
        item.remove_window();
        assert_eq!(item.windows, 1);
        assert!(item.running);
        item.remove_window();
        assert_eq!(item.windows, 0);
        assert!(!item.running);
    }

    #[test]
    fn test_remove_window_no_overflow() {
        let mut item = DockItem::new("app", "App");
        item.remove_window();
        assert_eq!(item.windows, 0);
        assert!(!item.running);
    }

    #[test]
    fn test_clone() {
        let item = DockItem::with_icon("org.gnome.Calculator", "Calculator", "calc");
        let cloned = item.clone();
        assert_eq!(cloned.app_id, item.app_id);
        assert_eq!(cloned.icon_name, item.icon_name);
        assert_eq!(cloned.name, item.name);
        assert_eq!(cloned.pinned, item.pinned);
        assert_eq!(cloned.running, item.running);
        assert_eq!(cloned.badge, item.badge);
        assert_eq!(cloned.windows, item.windows);
        assert_eq!(cloned.focused, item.focused);
    }

    #[test]
    fn test_debug_format() {
        let item = DockItem::with_icon("app", "App", "icon");
        let debug = format!("{:?}", item);
        assert!(debug.contains("DockItem"));
        assert!(debug.contains("app"));
        assert!(debug.contains("App"));
        assert!(debug.contains("icon"));
    }
}
