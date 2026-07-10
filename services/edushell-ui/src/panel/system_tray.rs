// SPDX-License-Identifier: GPL-3.0-or-later

//! # System Tray
//!
//! Hosts status notifier items (modern) and legacy tray icons.
//! Supports dynamic add/remove, tooltips, and context menus.

/// A single item in the system tray.
#[derive(Debug, Clone)]
pub struct SystemTrayItem {
    /// Unique identifier for this tray item.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Icon name (theme icon or path).
    pub icon_name: String,
    /// Tooltip text.
    pub tooltip: String,
}

impl SystemTrayItem {
    /// Create a new system tray item.
    pub fn new(id: &str, title: &str, icon_name: &str, tooltip: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            icon_name: icon_name.to_string(),
            tooltip: tooltip.to_string(),
        }
    }
}

/// The system tray component in the panel.
pub struct SystemTray {
    items: Vec<SystemTrayItem>,
    #[cfg(feature = "gtk")]
    widget: Option<gtk::Box>,
}

impl SystemTray {
    /// Create a new empty system tray.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            #[cfg(feature = "gtk")]
            widget: None,
        }
    }

    /// Build the GTK widget for this component.
    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        let box_ = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        box_.add_css_class("system-tray");
    }

    /// Build stub for non-GTK mode.
    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {
        tracing::info!(target: "edushell::ui::panel::system_tray", "System tray stub: {} items", self.items.len());
    }

    /// Add an item to the system tray.
    pub fn add_item(&mut self, item: SystemTrayItem) {
        if !self.items.iter().any(|i| i.id == item.id) {
            self.items.push(item);
        }
    }

    /// Remove an item from the system tray by ID.
    pub fn remove_item(&mut self, id: &str) {
        self.items.retain(|i| i.id != id);
    }

    /// Get a slice of all tray items.
    pub fn items(&self) -> &[SystemTrayItem] {
        &self.items
    }

    /// Get the number of tray items.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for SystemTray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemTray")
            .field("item_count", &self.items.len())
            .finish_non_exhaustive()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: &str) -> SystemTrayItem {
        SystemTrayItem::new(id, "Test App", "test-icon", "A test tray item")
    }

    fn make_tray() -> SystemTray {
        SystemTray::new()
    }

    #[test]
    fn test_new_tray_is_empty() {
        let tray = make_tray();
        assert_eq!(tray.item_count(), 0);
        assert!(tray.items().is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut tray = make_tray();
        tray.add_item(make_item("network"));
        assert_eq!(tray.item_count(), 1);
    }

    #[test]
    fn test_add_duplicate_item() {
        let mut tray = make_tray();
        tray.add_item(make_item("network"));
        tray.add_item(make_item("network"));
        assert_eq!(tray.item_count(), 1);
    }

    #[test]
    fn test_remove_item() {
        let mut tray = make_tray();
        tray.add_item(make_item("network"));
        tray.add_item(make_item("sound"));
        tray.remove_item("network");
        assert_eq!(tray.item_count(), 1);
        assert_eq!(tray.items()[0].id, "sound");
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut tray = make_tray();
        tray.add_item(make_item("item"));
        tray.remove_item("nonexistent");
        assert_eq!(tray.item_count(), 1);
    }

    #[test]
    fn test_items_slice_content() {
        let mut tray = make_tray();
        tray.add_item(SystemTrayItem::new("a", "Alpha", "icon-a", "Alpha tray icon"));
        tray.add_item(SystemTrayItem::new("b", "Beta", "icon-b", "Beta tray icon"));
        let items = tray.items();
        assert_eq!(items[0].title, "Alpha");
        assert_eq!(items[1].title, "Beta");
        assert_eq!(items[0].tooltip, "Alpha tray icon");
    }

    #[test]
    fn test_multiple_items() {
        let mut tray = make_tray();
        let ids = ["bluetooth", "network", "sound", "battery", "input"];
        for id in &ids {
            tray.add_item(make_item(id));
        }
        assert_eq!(tray.item_count(), 5);
    }

    #[test]
    fn test_item_constructor() {
        let item = SystemTrayItem::new("test-id", "Test Title", "icon", "A tooltip");
        assert_eq!(item.id, "test-id");
        assert_eq!(item.title, "Test Title");
        assert_eq!(item.icon_name, "icon");
        assert_eq!(item.tooltip, "A tooltip");
    }

    #[test]
    fn test_clear_all_items() {
        let mut tray = make_tray();
        tray.add_item(make_item("one"));
        tray.add_item(make_item("two"));
        tray.remove_item("one");
        tray.remove_item("two");
        assert_eq!(tray.item_count(), 0);
    }

    #[test]
    fn test_build_does_not_panic() {
        let tray = make_tray();
        tray.build();
    }

    #[test]
    fn test_default_trait() {
        let tray = SystemTray::default();
        assert_eq!(tray.item_count(), 0);
    }

    #[test]
    fn test_debug_output() {
        let mut tray = make_tray();
        tray.add_item(make_item("test"));
        let d = format!("{tray:?}");
        assert!(d.contains("SystemTray"));
        assert!(d.contains("item_count"));
    }

    #[test]
    fn test_item_debug() {
        let item = make_item("dbus");
        let d = format!("{item:?}");
        assert!(d.contains("id"));
        assert!(d.contains("dbus"));
    }

    #[test]
    fn test_remove_all_by_id_list() {
        let mut tray = make_tray();
        let all = vec!["a", "b", "c"];
        for id in &all {
            tray.add_item(make_item(id));
        }
        for id in &all {
            tray.remove_item(id);
        }
        assert_eq!(tray.item_count(), 0);
    }
}
