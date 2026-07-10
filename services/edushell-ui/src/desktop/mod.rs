// SPDX-License-Identifier: GPL-3.0-or-later

//! # Desktop Area
//!
//! The main desktop area providing wallpaper rendering,
//! desktop icons, right-click context menu, drag & drop,
//! and selection.
//!
//! ## Layout
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  Wallpaper (background)                  │
//! │  ┌────┐ ┌────┐                          │
//! │  │Icon│ │Icon│                          │
//! │  └────┘ └────┘                          │
//! │  ┌────────────────────────────────────┐ │
//! │  │ Context Menu (right-click)         │ │
//! │  │  New Folder                        │ │
//! │  │  Open Terminal                     │ │
//! │  │  Change Wallpaper                  │ │
//! │  │  Display Settings                  │ │
//! │  └────────────────────────────────────┘ │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Submodules
//!
//! | Module | Description |
//! |--------|-------------|
//! | `wallpaper` | Wallpaper rendering engine |
//! | `context_menu` | Right-click context menu |
//! | `icons` | Desktop icon manager |

pub mod wallpaper;
pub mod context_menu;
pub mod icons;

use crate::multi_monitor::MultiMonitorManager;
use crate::settings::SettingsIntegration;
use crate::localization::LocalizationManager;

/// A desktop icon (file, folder, or application shortcut).
#[derive(Debug, Clone)]
pub struct DesktopIcon {
    /// Unique identifier for the icon.
    pub id: String,
    /// Display label shown beneath the icon.
    pub label: String,
    /// Named icon from the icon theme.
    pub icon_name: String,
    /// X position in desktop coordinates.
    pub x: i32,
    /// Y position in desktop coordinates.
    pub y: i32,
    /// Whether this icon is currently selected.
    pub selected: bool,
}

impl DesktopIcon {
    /// Create a new desktop icon.
    pub fn new(id: &str, label: &str, icon_name: &str, x: i32, y: i32) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            icon_name: icon_name.to_string(),
            x,
            y,
            selected: false,
        }
    }
}

/// Desktop area view mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DesktopViewMode {
    /// Show desktop icons normally.
    Icons,
    /// Hide all desktop icons.
    Clean,
    /// Show only a minimal set of icons.
    Minimal,
}

impl DesktopViewMode {
    /// All available view modes.
    pub fn all() -> &'static [Self] {
        &[Self::Icons, Self::Clean, Self::Minimal]
    }
}

/// The main desktop area widget and state manager.
///
/// Coordinates wallpaper rendering, icon management, context menus,
/// selection, and multi-monitor layout.
pub struct DesktopArea {
    wallpaper_path: Option<String>,
    wallpaper_color: String,
    icons: Vec<DesktopIcon>,
    view_mode: DesktopViewMode,
    show_icons: bool,
    context_menu_open: bool,
    selection_active: bool,
    selection_rect: Option<(i32, i32, i32, i32)>,
    monitors: MultiMonitorManager,
    settings: SettingsIntegration,
    localization: LocalizationManager,
    #[cfg(feature = "gtk")]
    widget: Option<gtk::Overlay>,
    #[cfg(feature = "gtk")]
    wallpaper_widget: Option<gtk::Picture>,
}

impl DesktopArea {
    /// Create a new desktop area.
    pub fn new(
        monitors: MultiMonitorManager,
        settings: SettingsIntegration,
        localization: LocalizationManager,
    ) -> Self {
        Self {
            wallpaper_path: None,
            wallpaper_color: "#2e3436".to_string(),
            icons: Vec::new(),
            view_mode: DesktopViewMode::Icons,
            show_icons: true,
            context_menu_open: false,
            selection_active: false,
            selection_rect: None,
            monitors,
            settings,
            localization,
            #[cfg(feature = "gtk")]
            widget: None,
            #[cfg(feature = "gtk")]
            wallpaper_widget: None,
        }
    }

    /// Build the GTK widget tree (no-op without `gtk` feature).
    pub fn build(&self) {
        #[cfg(feature = "gtk")]
        {
            // GTK widget construction would create an GtkOverlay with
            // a GtkPicture for the wallpaper and GtkFlowBox for icons.
        }
    }

    /// Set the wallpaper to an image file.
    pub fn set_wallpaper(&mut self, path: &str) {
        self.wallpaper_path = Some(path.to_string());
    }

    /// Set the wallpaper to a solid color.
    pub fn set_wallpaper_color(&mut self, color: &str) {
        self.wallpaper_color = color.to_string();
    }

    /// Get the current wallpaper path, if set.
    pub fn wallpaper_path(&self) -> Option<&str> {
        self.wallpaper_path.as_deref()
    }

    /// Get the current wallpaper color.
    pub fn wallpaper_color(&self) -> &str {
        &self.wallpaper_color
    }

    /// Set the desktop view mode.
    pub fn set_view_mode(&mut self, mode: DesktopViewMode) {
        self.view_mode = mode;
        match mode {
            DesktopViewMode::Icons => self.show_icons = true,
            DesktopViewMode::Clean => self.show_icons = false,
            DesktopViewMode::Minimal => self.show_icons = true,
        }
    }

    /// Get the current view mode.
    pub fn view_mode(&self) -> DesktopViewMode {
        self.view_mode
    }

    /// Check if desktop icons are currently shown.
    pub fn show_icons(&self) -> bool {
        self.show_icons
    }

    /// Manually control whether icons are shown.
    pub fn set_show_icons(&mut self, show: bool) {
        self.show_icons = show;
    }

    /// Get a reference to all desktop icons.
    pub fn icons(&self) -> &[DesktopIcon] {
        &self.icons
    }

    /// Add an icon to the desktop.
    pub fn add_icon(&mut self, icon: DesktopIcon) {
        self.icons.push(icon);
    }

    /// Remove an icon by ID.
    pub fn remove_icon(&mut self, id: &str) {
        self.icons.retain(|i| i.id != id);
    }

    /// Remove all icons.
    pub fn clear_icons(&mut self) {
        self.icons.clear();
    }

    /// Select a specific icon by ID, deselecting all others.
    pub fn select_icon(&mut self, id: &str) {
        for icon in &mut self.icons {
            icon.selected = icon.id == id;
        }
    }

    /// Deselect all icons.
    pub fn deselect_all(&mut self) {
        for icon in &mut self.icons {
            icon.selected = false;
        }
    }

    /// Get a list of references to all currently selected icons.
    pub fn selected_icons(&self) -> Vec<&DesktopIcon> {
        self.icons.iter().filter(|i| i.selected).collect()
    }

    /// Get the number of selected icons.
    pub fn selected_count(&self) -> usize {
        self.icons.iter().filter(|i| i.selected).count()
    }

    /// Open the context menu at the given screen coordinates.
    pub fn open_context_menu(&mut self, x: f64, y: f64) {
        self.context_menu_open = true;
        let _ = x;
        let _ = y;
    }

    /// Close the context menu.
    pub fn close_context_menu(&mut self) {
        self.context_menu_open = false;
    }

    /// Check if the context menu is currently open.
    pub fn context_menu_open(&self) -> bool {
        self.context_menu_open
    }

    /// Get a reference to the multi-monitor manager.
    pub fn monitors(&self) -> &MultiMonitorManager {
        &self.monitors
    }

    /// Get a reference to the settings integration.
    pub fn settings(&self) -> &SettingsIntegration {
        &self.settings
    }

    /// Get a reference to the localization manager.
    pub fn localization(&self) -> &LocalizationManager {
        &self.localization
    }

    /// Check if a selection rubber-band is active.
    pub fn selection_active(&self) -> bool {
        self.selection_active
    }

    /// Start a rubber-band selection.
    pub fn start_selection(&mut self, x: i32, y: i32) {
        self.selection_active = true;
        self.selection_rect = Some((x, y, 0, 0));
    }

    /// Update the rubber-band selection rectangle.
    pub fn update_selection(&mut self, x: i32, y: i32) {
        if let Some((sx, sy, _, _)) = self.selection_rect {
            let w = (x - sx).abs();
            let h = (y - sy).abs();
            let nx = x.min(sx);
            let ny = y.min(sy);
            self.selection_rect = Some((nx, ny, w, h));
        }
    }

    /// Finish the rubber-band selection.
    pub fn end_selection(&mut self) {
        self.selection_active = false;
        self.selection_rect = None;
    }

    /// Get the current selection rectangle, if any.
    pub fn selection_rect(&self) -> Option<(i32, i32, i32, i32)> {
        self.selection_rect
    }
}

impl std::fmt::Debug for DesktopArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DesktopArea")
            .field("wallpaper_path", &self.wallpaper_path)
            .field("wallpaper_color", &self.wallpaper_color)
            .field("view_mode", &self.view_mode)
            .field("show_icons", &self.show_icons)
            .field("context_menu_open", &self.context_menu_open)
            .field("selection_active", &self.selection_active)
            .field("icons", &self.icons.len())
            .field("monitors", &self.monitors.count())
            .finish()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_monitors() -> MultiMonitorManager {
        let mut m = MultiMonitorManager::new();
        m.set_monitors(vec![
            crate::multi_monitor::MonitorInfo {
                id: 0, name: "eDP-1".into(),
                x: 0, y: 0, width: 1920, height: 1080,
                scale: 1.0, primary: true, refresh_rate: 60.0,
            },
        ]);
        m
    }

    fn test_settings() -> SettingsIntegration {
        use edushell_core::config::ConfigManager;
        use edushell_core::settings::SettingsBackend;
        use edushell_core::event::EventBus;
        SettingsIntegration::new(
            ConfigManager::new(),
            SettingsBackend::new(),
            EventBus::new(),
        )
    }

    fn test_localization() -> LocalizationManager {
        let mut lm = LocalizationManager::new();
        lm.set_locale("en-US");
        lm
    }

    fn make_area() -> DesktopArea {
        DesktopArea::new(test_monitors(), test_settings(), test_localization())
    }

    #[test]
    fn test_desktop_area_creation() {
        let da = make_area();
        assert!(da.wallpaper_path().is_none());
        assert_eq!(da.wallpaper_color(), "#2e3436");
        assert_eq!(da.view_mode(), DesktopViewMode::Icons);
        assert!(da.show_icons());
        assert!(!da.context_menu_open());
        assert!(!da.selection_active());
    }

    #[test]
    fn test_set_wallpaper_path() {
        let mut da = make_area();
        da.set_wallpaper("/path/to/wall.jpg");
        assert_eq!(da.wallpaper_path(), Some("/path/to/wall.jpg"));
    }

    #[test]
    fn test_set_wallpaper_color() {
        let mut da = make_area();
        da.set_wallpaper_color("#ff0000");
        assert_eq!(da.wallpaper_color(), "#ff0000");
    }

    #[test]
    fn test_view_mode_icons() {
        let mut da = make_area();
        da.set_view_mode(DesktopViewMode::Clean);
        assert_eq!(da.view_mode(), DesktopViewMode::Clean);
        assert!(!da.show_icons());
        da.set_view_mode(DesktopViewMode::Icons);
        assert!(da.show_icons());
    }

    #[test]
    fn test_view_mode_minimal() {
        let mut da = make_area();
        da.set_view_mode(DesktopViewMode::Minimal);
        assert_eq!(da.view_mode(), DesktopViewMode::Minimal);
        assert!(da.show_icons());
    }

    #[test]
    fn test_set_show_icons() {
        let mut da = make_area();
        da.set_show_icons(false);
        assert!(!da.show_icons());
        da.set_show_icons(true);
        assert!(da.show_icons());
    }

    #[test]
    fn test_add_and_remove_icon() {
        let mut da = make_area();
        assert_eq!(da.icons().len(), 0);
        da.add_icon(DesktopIcon::new("test", "Test", "unknown", 10, 20));
        assert_eq!(da.icons().len(), 1);
        da.remove_icon("test");
        assert_eq!(da.icons().len(), 0);
    }

    #[test]
    fn test_clear_icons() {
        let mut da = make_area();
        da.add_icon(DesktopIcon::new("a", "A", "a", 0, 0));
        da.add_icon(DesktopIcon::new("b", "B", "b", 0, 0));
        da.clear_icons();
        assert_eq!(da.icons().len(), 0);
    }

    #[test]
    fn test_select_and_deselect() {
        let mut da = make_area();
        da.add_icon(DesktopIcon::new("a", "A", "a", 0, 0));
        da.add_icon(DesktopIcon::new("b", "B", "b", 0, 0));
        da.select_icon("a");
        assert_eq!(da.selected_icons().len(), 1);
        assert_eq!(da.selected_icons()[0].id, "a");
        assert_eq!(da.selected_count(), 1);
        da.deselect_all();
        assert_eq!(da.selected_count(), 0);
    }

    #[test]
    fn test_context_menu() {
        let mut da = make_area();
        assert!(!da.context_menu_open());
        da.open_context_menu(100.0, 200.0);
        assert!(da.context_menu_open());
        da.close_context_menu();
        assert!(!da.context_menu_open());
    }

    #[test]
    fn test_selection_rubber_band() {
        let mut da = make_area();
        assert!(!da.selection_active());
        assert!(da.selection_rect().is_none());
        da.start_selection(50, 50);
        assert!(da.selection_active());
        da.update_selection(100, 100);
        assert!(da.selection_rect().is_some());
        da.end_selection();
        assert!(!da.selection_active());
        assert!(da.selection_rect().is_none());
    }

    #[test]
    fn test_desktop_icon_new() {
        let icon = DesktopIcon::new("my-id", "My Label", "my-icon", 32, 64);
        assert_eq!(icon.id, "my-id");
        assert_eq!(icon.label, "My Label");
        assert_eq!(icon.icon_name, "my-icon");
        assert_eq!(icon.x, 32);
        assert_eq!(icon.y, 64);
        assert!(!icon.selected);
    }

    #[test]
    fn test_view_mode_all() {
        let modes = DesktopViewMode::all();
        assert_eq!(modes.len(), 3);
        assert!(modes.contains(&DesktopViewMode::Icons));
        assert!(modes.contains(&DesktopViewMode::Clean));
        assert!(modes.contains(&DesktopViewMode::Minimal));
    }

    #[test]
    fn test_selection_update_ordering() {
        let mut da = make_area();
        da.start_selection(100, 100);
        da.update_selection(50, 150);
        let rect = da.selection_rect().unwrap();
        assert_eq!(rect.0, 50); // min x
        assert_eq!(rect.1, 100); // min y
        assert_eq!(rect.2, 50); // width
        assert_eq!(rect.3, 50); // height
    }

    #[test]
    fn test_localization_and_settings_access() {
        let da = make_area();
        assert_eq!(da.localization().locale(), "en-US");
        assert!(!da.settings().dark_mode());
        assert_eq!(da.monitors().count(), 1);
    }
}
