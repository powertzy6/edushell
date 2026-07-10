// SPDX-License-Identifier: GPL-3.0-or-later

//! # Dock
//!
//! Application dock showing pinned and running applications.
//! Supports auto-hide, drag-and-drop reorder, badges,
//! window indicators, and workspace awareness.

pub mod dock_item;

use crate::theme::ThemeManager;
use crate::multi_monitor::MultiMonitorManager;
use crate::localization::LocalizationManager;
use crate::settings::SettingsIntegration;
use crate::accessibility::AccessibilityManager;
use crate::dock::dock_item::DockItem;

/// Dock position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DockPosition {
    Bottom,
    Left,
    Right,
}

/// Dock state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DockState {
    Visible,
    AutoHidden,
    Hidden,
}

/// The main dock container.
pub struct Dock {
    position: DockPosition,
    state: DockState,
    icon_size: i32,
    auto_hide: bool,
    magnification: bool,
    pinned_apps: Vec<String>,
    running_apps: Vec<String>,
    items: Vec<DockItem>,
    theme: ThemeManager,
    monitors: MultiMonitorManager,
    localization: LocalizationManager,
    settings: SettingsIntegration,
    accessibility: AccessibilityManager,
    // GTK fields
    #[cfg(feature = "gtk")]
    widget: Option<gtk::Box>,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
}

impl Dock {
    pub fn new(
        theme: ThemeManager,
        monitors: MultiMonitorManager,
        localization: LocalizationManager,
        settings: SettingsIntegration,
        accessibility: AccessibilityManager,
    ) -> Self {
        Dock {
            position: DockPosition::Bottom,
            state: DockState::Visible,
            icon_size: 48,
            auto_hide: false,
            magnification: false,
            pinned_apps: Vec::new(),
            running_apps: Vec::new(),
            items: Vec::new(),
            theme,
            monitors,
            localization,
            settings,
            accessibility,
            #[cfg(feature = "gtk")]
            widget: None,
            #[cfg(feature = "gtk")]
            window: None,
        }
    }

    pub fn build(&self) {
        #[cfg(feature = "gtk")]
        {
            if let Some(ref w) = self.window {
                w.show_all();
            }
        }
    }

    pub fn destroy(&self) {
        #[cfg(feature = "gtk")]
        {
            if let Some(ref w) = self.window {
                w.close();
            }
        }
    }

    pub fn add_pinned(&mut self, app_id: &str) {
        if self.pinned_apps.iter().any(|a| a == app_id) {
            return;
        }
        self.pinned_apps.push(app_id.to_string());
        if let Some(item) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            item.pinned = true;
        } else {
            let mut item = DockItem::new(app_id, app_id);
            item.pinned = true;
            item.running = self.running_apps.contains(&app_id.to_string());
            self.items.push(item);
        }
    }

    pub fn remove_pinned(&mut self, app_id: &str) {
        self.pinned_apps.retain(|a| a != app_id);
        if let Some(item) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            item.pinned = false;
        }
    }

    pub fn add_running(&mut self, app_id: &str) {
        if self.running_apps.iter().any(|a| a == app_id) {
            return;
        }
        self.running_apps.push(app_id.to_string());
        if let Some(item) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            item.running = true;
            item.add_window();
        } else {
            let mut item = DockItem::new(app_id, app_id);
            item.running = true;
            item.pinned = self.pinned_apps.contains(&app_id.to_string());
            item.add_window();
            self.items.push(item);
        }
    }

    pub fn remove_running(&mut self, app_id: &str) {
        self.running_apps.retain(|a| a != app_id);
        if let Some(item) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            item.running = false;
            item.windows = 0;
            item.focused = false;
        }
    }

    pub fn is_pinned(&self, app_id: &str) -> bool {
        self.pinned_apps.contains(&app_id.to_string())
    }

    pub fn is_running(&self, app_id: &str) -> bool {
        self.running_apps.contains(&app_id.to_string())
    }

    pub fn pin_app(&mut self, app_id: &str) {
        self.add_pinned(app_id);
    }

    pub fn unpin_app(&mut self, app_id: &str) {
        self.remove_pinned(app_id);
    }

    pub fn set_badge(&mut self, app_id: &str, count: u32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            item.set_badge(count);
        }
    }

    pub fn reorder(&mut self, from: usize, to: usize) {
        let len = self.items.len();
        if from >= len || to >= len || from == to {
            return;
        }
        let item = self.items.remove(from);
        self.items.insert(to, item);
    }

    pub fn set_position(&mut self, pos: DockPosition) {
        self.position = pos;
    }

    pub fn set_auto_hide(&mut self, enabled: bool) {
        self.auto_hide = enabled;
        if enabled && self.state == DockState::Visible {
            self.state = DockState::AutoHidden;
        } else if !enabled && self.state == DockState::AutoHidden {
            self.state = DockState::Visible;
        }
    }

    pub fn set_magnification(&mut self, enabled: bool) {
        self.magnification = enabled;
    }

    pub fn set_icon_size(&mut self, size: i32) {
        if size >= 16 && size <= 128 {
            self.icon_size = size;
        }
    }
}

impl std::fmt::Debug for Dock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dock")
            .field("position", &self.position)
            .field("state", &self.state)
            .field("icon_size", &self.icon_size)
            .field("auto_hide", &self.auto_hide)
            .field("magnification", &self.magnification)
            .field("pinned_count", &self.pinned_apps.len())
            .field("running_count", &self.running_apps.len())
            .field("item_count", &self.items.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_managers() -> (ThemeManager, MultiMonitorManager, LocalizationManager, SettingsIntegration, AccessibilityManager) {
        let config = edushell_core::config::ConfigManager::new();
        let settings = edushell_core::settings::SettingsBackend::new();
        let bus = edushell_core::event::EventBus::new();
        (
            ThemeManager::new(),
            MultiMonitorManager::new(),
            LocalizationManager::new(),
            SettingsIntegration::new(config, settings, bus),
            AccessibilityManager::new(),
        )
    }

    #[test]
    fn test_dock_creation() {
        let (t, m, l, s, a) = make_managers();
        let dock = Dock::new(t, m, l, s, a);
        assert_eq!(dock.position, DockPosition::Bottom);
        assert_eq!(dock.state, DockState::Visible);
        assert_eq!(dock.icon_size, 48);
        assert!(!dock.auto_hide);
        assert!(!dock.magnification);
        assert!(dock.pinned_apps.is_empty());
        assert!(dock.running_apps.is_empty());
        assert!(dock.items.is_empty());
    }

    #[test]
    fn test_add_remove_pinned() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.add_pinned("org.gnome.Calculator");
        assert!(dock.is_pinned("org.gnome.Calculator"));
        assert_eq!(dock.pinned_apps.len(), 1);
        dock.add_pinned("org.gnome.Calculator");
        assert_eq!(dock.pinned_apps.len(), 1);
        dock.remove_pinned("org.gnome.Calculator");
        assert!(!dock.is_pinned("org.gnome.Calculator"));
        assert!(dock.pinned_apps.is_empty());
    }

    #[test]
    fn test_add_remove_running() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.add_running("firefox");
        assert!(dock.is_running("firefox"));
        assert_eq!(dock.running_apps.len(), 1);
        dock.add_running("firefox");
        assert_eq!(dock.running_apps.len(), 1);
        dock.remove_running("firefox");
        assert!(!dock.is_running("firefox"));
        assert!(dock.running_apps.is_empty());
    }

    #[test]
    fn test_pin_unpin() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.pin_app("code");
        assert!(dock.is_pinned("code"));
        dock.unpin_app("code");
        assert!(!dock.is_pinned("code"));
    }

    #[test]
    fn test_is_pinned_is_running() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        assert!(!dock.is_pinned("nonexistent"));
        assert!(!dock.is_running("nonexistent"));
        dock.add_pinned("app1");
        dock.add_running("app2");
        assert!(dock.is_pinned("app1"));
        assert!(dock.is_running("app2"));
        assert!(!dock.is_pinned("app2"));
        assert!(!dock.is_running("app1"));
    }

    #[test]
    fn test_set_badge() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.add_pinned("app");
        dock.set_badge("app", 7);
        let item = dock.items.iter().find(|i| i.app_id == "app").unwrap();
        assert_eq!(item.badge, 7);
        assert!(item.has_badge());
    }

    #[test]
    fn test_reorder() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.add_pinned("alpha");
        dock.add_pinned("beta");
        dock.add_pinned("gamma");
        assert_eq!(dock.items[0].app_id, "alpha");
        assert_eq!(dock.items[2].app_id, "gamma");
        dock.reorder(0, 2);
        assert_eq!(dock.items[0].app_id, "beta");
        assert_eq!(dock.items[2].app_id, "alpha");
    }

    #[test]
    fn test_reorder_bounds() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.add_pinned("only");
        dock.reorder(0, 5);
        dock.reorder(5, 0);
        assert_eq!(dock.items.len(), 1);
        assert_eq!(dock.items[0].app_id, "only");
    }

    #[test]
    fn test_set_position() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        assert_eq!(dock.position, DockPosition::Bottom);
        dock.set_position(DockPosition::Left);
        assert_eq!(dock.position, DockPosition::Left);
        dock.set_position(DockPosition::Right);
        assert_eq!(dock.position, DockPosition::Right);
        dock.set_position(DockPosition::Bottom);
        assert_eq!(dock.position, DockPosition::Bottom);
    }

    #[test]
    fn test_auto_hide_toggle() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        assert!(!dock.auto_hide);
        assert_eq!(dock.state, DockState::Visible);
        dock.set_auto_hide(true);
        assert!(dock.auto_hide);
        assert_eq!(dock.state, DockState::AutoHidden);
        dock.set_auto_hide(false);
        assert!(!dock.auto_hide);
        assert_eq!(dock.state, DockState::Visible);
    }

    #[test]
    fn test_magnification_toggle() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        assert!(!dock.magnification);
        dock.set_magnification(true);
        assert!(dock.magnification);
        dock.set_magnification(false);
        assert!(!dock.magnification);
    }

    #[test]
    fn test_set_icon_size() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        assert_eq!(dock.icon_size, 48);
        dock.set_icon_size(64);
        assert_eq!(dock.icon_size, 64);
        dock.set_icon_size(8);
        assert_eq!(dock.icon_size, 64);
        dock.set_icon_size(256);
        assert_eq!(dock.icon_size, 64);
        dock.set_icon_size(16);
        assert_eq!(dock.icon_size, 16);
        dock.set_icon_size(128);
        assert_eq!(dock.icon_size, 128);
    }

    #[test]
    fn test_debug_format() {
        let (t, m, l, s, a) = make_managers();
        let dock = Dock::new(t, m, l, s, a);
        let debug = format!("{:?}", dock);
        assert!(debug.contains("Dock"));
        assert!(debug.contains("position"));
        assert!(debug.contains("Bottom"));
        assert!(debug.contains("state"));
        assert!(debug.contains("Visible"));
        assert!(debug.contains("icon_size"));
    }

    #[test]
    fn test_dock_build_destroy() {
        let (t, m, l, s, a) = make_managers();
        let dock = Dock::new(t, m, l, s, a);
        dock.build();
        dock.destroy();
    }

    #[test]
    fn test_combined_pinned_and_running() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.add_pinned("app");
        dock.add_running("app");
        assert!(dock.is_pinned("app"));
        assert!(dock.is_running("app"));
        let item = dock.items.iter().find(|i| i.app_id == "app").unwrap();
        assert!(item.pinned);
        assert!(item.running);
        assert_eq!(item.windows, 1);
    }

    #[test]
    fn test_state_remains_visible_when_auto_hide_toggled_without_visible() {
        let (t, m, l, s, a) = make_managers();
        let mut dock = Dock::new(t, m, l, s, a);
        dock.state = DockState::Hidden;
        dock.set_auto_hide(true);
        assert_eq!(dock.state, DockState::Hidden);
        dock.set_auto_hide(false);
        assert_eq!(dock.state, DockState::Hidden);
    }
}
