// SPDX-License-Identifier: GPL-3.0-or-later

//! # Desktop Context Menu
//!
//! Right-click menu on the desktop area providing file
//! operations, display settings, wallpaper change, and
//! terminal access.
//!
//! ## Default Menu Structure
//!
//! ```text
//! New Folder
//! Open Terminal
//! ─────────────
//! Change Wallpaper
//! Display Settings
//! ─────────────
//! Desktop Icons  >  Show Icons
//!                   Clean Desktop
//!                   Minimal Mode
//! ```

use crate::localization::LocalizationManager;

/// A menu item in the context menu.
#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    /// Unique identifier for this item.
    pub id: String,
    /// Display label (already translated).
    pub label: String,
    /// Optional icon name (icon theme lookup).
    pub icon: Option<String>,
    /// Whether the item is enabled.
    pub enabled: bool,
    /// Whether this item is a separator.
    pub separator: bool,
    /// Submenu items, if any.
    pub submenu: Vec<ContextMenuItem>,
}

impl ContextMenuItem {
    /// Create a new menu item.
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            icon: None,
            enabled: true,
            separator: false,
            submenu: Vec::new(),
        }
    }

    /// Create a separator item.
    pub fn separator() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            icon: None,
            enabled: false,
            separator: true,
            submenu: Vec::new(),
        }
    }

    /// Set an icon for this item.
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// Set enabled state.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add a submenu item.
    pub fn with_submenu(mut self, items: Vec<ContextMenuItem>) -> Self {
        self.submenu = items;
        self
    }
}

/// Desktop right-click context menu.
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    visible: bool,
    x: f64,
    y: f64,
    localization: LocalizationManager,
    #[cfg(feature = "gtk")]
    popover: Option<gtk::Popover>,
    #[cfg(feature = "gtk")]
    menu: Option<gtk::Box>,
}

impl ContextMenu {
    /// Create a new context menu with default items.
    pub fn new(localization: LocalizationManager) -> Self {
        let items = Self::default_items_internal(&localization);
        Self {
            items,
            visible: false,
            x: 0.0,
            y: 0.0,
            localization,
            #[cfg(feature = "gtk")]
            popover: None,
            #[cfg(feature = "gtk")]
            menu: None,
        }
    }

    /// Build the GTK widget tree (no-op without `gtk` feature).
    pub fn build(&self) {
        #[cfg(feature = "gtk")]
        {
            // GTK widget construction would happen here.
            // In a real implementation, this would create a GtkPopover
            // with GtkButton children for each item.
        }
    }

    /// Show the menu at the given screen coordinates.
    pub fn show(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.visible = true;
        #[cfg(feature = "gtk")]
        if let Some(ref popover) = self.popover {
            popover.popup();
        }
    }

    /// Hide the menu.
    pub fn hide(&mut self) {
        self.visible = false;
        #[cfg(feature = "gtk")]
        if let Some(ref popover) = self.popover {
            popover.popdown();
        }
    }

    /// Toggle the menu at the given position.
    pub fn toggle(&mut self, x: f64, y: f64) {
        if self.visible {
            self.hide();
        } else {
            self.show(x, y);
        }
    }

    /// Check if the menu is currently visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Replace all menu items.
    pub fn set_items(&mut self, items: Vec<ContextMenuItem>) {
        self.items = items;
    }

    /// Get the current menu items.
    pub fn items(&self) -> &[ContextMenuItem] {
        &self.items
    }

    /// Get the current position of the menu.
    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Generate the default set of menu items (translated).
    pub fn default_items(&self) -> Vec<ContextMenuItem> {
        Self::default_items_internal(&self.localization)
    }

    fn default_items_internal(localization: &LocalizationManager) -> Vec<ContextMenuItem> {
        let t = |key: &str| localization.translate(key);

        vec![
            ContextMenuItem::new("new-folder", &t("new_folder"))
                .with_icon("folder-new-symbolic"),
            ContextMenuItem::new("open-terminal", &t("open_terminal"))
                .with_icon("utilities-terminal-symbolic"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("change-wallpaper", &t("change_wallpaper"))
                .with_icon("preferences-desktop-wallpaper-symbolic"),
            ContextMenuItem::new("display-settings", &t("display_settings"))
                .with_icon("preferences-system-display-symbolic"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("desktop-icons", "Desktop Icons")
                .with_icon("preferences-desktop-icons-symbolic")
                .with_submenu(vec![
                    ContextMenuItem::new("show-icons", &t("show_desktop")),
                    ContextMenuItem::new("clean-desktop", "Clean Desktop"),
                    ContextMenuItem::new("minimal-mode", "Minimal Mode"),
                ]),
        ]
    }
}

impl std::fmt::Debug for ContextMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextMenu")
            .field("visible", &self.visible)
            .field("position", &(self.x, self.y))
            .field("items", &self.items.len())
            .finish()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_localization() -> LocalizationManager {
        let mut lm = LocalizationManager::new();
        lm.set_locale("en-US");
        lm
    }

    #[test]
    fn test_context_menu_creation() {
        let lm = test_localization();
        let menu = ContextMenu::new(lm);
        assert!(!menu.is_visible());
        assert_eq!(menu.position(), (0.0, 0.0));
    }

    #[test]
    fn test_context_menu_show_hide() {
        let lm = test_localization();
        let mut menu = ContextMenu::new(lm);
        menu.show(100.0, 200.0);
        assert!(menu.is_visible());
        assert_eq!(menu.position(), (100.0, 200.0));
        menu.hide();
        assert!(!menu.is_visible());
    }

    #[test]
    fn test_context_menu_toggle() {
        let lm = test_localization();
        let mut menu = ContextMenu::new(lm);
        menu.toggle(50.0, 75.0);
        assert!(menu.is_visible());
        menu.toggle(0.0, 0.0);
        assert!(!menu.is_visible());
    }

    #[test]
    fn test_context_menu_set_items() {
        let lm = test_localization();
        let mut menu = ContextMenu::new(lm);
        let custom = vec![
            ContextMenuItem::new("custom-1", "Custom 1"),
            ContextMenuItem::new("custom-2", "Custom 2"),
        ];
        menu.set_items(custom.clone());
        assert_eq!(menu.items().len(), 2);
        assert_eq!(menu.items()[0].id, "custom-1");
    }

    #[test]
    fn test_default_items_have_translations() {
        let lm = test_localization();
        let menu = ContextMenu::new(lm);
        let items = menu.default_items();
        assert!(!items.is_empty());
        // Check for known entries
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"New Folder"));
        assert!(labels.contains(&"Open Terminal"));
    }

    #[test]
    fn test_separator_item() {
        let sep = ContextMenuItem::separator();
        assert!(sep.separator);
        assert!(!sep.enabled);
        assert!(sep.icon.is_none());
    }

    #[test]
    fn test_menu_item_with_icon() {
        let item = ContextMenuItem::new("test", "Test")
            .with_icon("test-icon");
        assert_eq!(item.icon.as_deref(), Some("test-icon"));
    }

    #[test]
    fn test_menu_item_with_submenu() {
        let sub = vec![
            ContextMenuItem::new("sub-1", "Sub 1"),
        ];
        let item = ContextMenuItem::new("parent", "Parent")
            .with_submenu(sub);
        assert_eq!(item.submenu.len(), 1);
        assert_eq!(item.submenu[0].id, "sub-1");
    }

    #[test]
    fn test_menu_item_disabled() {
        let item = ContextMenuItem::new("test", "Test")
            .with_enabled(false);
        assert!(!item.enabled);
    }

    #[test]
    fn test_default_items_have_submenu() {
        let lm = test_localization();
        let menu = ContextMenu::new(lm);
        let items = menu.default_items();
        let desktop_icons = items.iter().find(|i| i.id == "desktop-icons");
        assert!(desktop_icons.is_some());
        assert!(!desktop_icons.unwrap().submenu.is_empty());
    }
}
