// SPDX-License-Identifier: GPL-3.0-or-later

//! # Desktop Icons
//!
//! Manages icons shown on the desktop. Supports selection,
//! rubber-band selection, drag and drop, and grid alignment.
//!
//! ## Grid Layout
//!
//! Icons are arranged in a grid from top-left to bottom-right.
//! Grid spacing, icon size, and column count are configurable.
//!
//! ```text
//! ┌──────┐  ┌──────┐  ┌──────┐
//! │ Icon │  │ Icon │  │ Icon │
//! │  1   │  │  2   │  │  3   │
//! └──────┘  └──────┘  └──────┘
//! ┌──────┐  ┌──────┐
//! │ Icon │  │ Icon │
//! │  4   │  │  5   │
//! └──────┘  └──────┘
//! ```

use crate::desktop::DesktopIcon;

/// Manages desktop icon placement, selection, and layout.
#[derive(Debug, Clone)]
pub struct DesktopIconManager {
    icons: Vec<DesktopIcon>,
    grid_spacing: i32,
    icon_size: i32,
    grid_columns: u32,
    show_home: bool,
    show_trash: bool,
    show_mounts: bool,
}

impl DesktopIconManager {
    /// Create a new desktop icon manager with default settings.
    pub fn new() -> Self {
        let icons = Self::default_icons_internal();
        Self {
            icons,
            grid_spacing: 80,
            icon_size: 64,
            grid_columns: 7,
            show_home: true,
            show_trash: true,
            show_mounts: true,
        }
    }

    /// Replace all icons with a new set.
    pub fn set_icons(&mut self, icons: Vec<DesktopIcon>) {
        self.icons = icons;
    }

    /// Get a reference to all icons.
    pub fn icons(&self) -> &[DesktopIcon] {
        &self.icons
    }

    /// Get a mutable reference to all icons.
    pub fn icons_mut(&mut self) -> &mut [DesktopIcon] {
        &mut self.icons
    }

    /// Add an icon. Returns `false` if an icon with the same `id` already exists.
    pub fn add_icon(&mut self, icon: DesktopIcon) -> bool {
        if self.icons.iter().any(|i| i.id == icon.id) {
            return false;
        }
        self.icons.push(icon);
        true
    }

    /// Remove an icon by ID. Returns `true` if the icon was found and removed.
    pub fn remove_icon(&mut self, id: &str) -> bool {
        let len = self.icons.len();
        self.icons.retain(|i| i.id != id);
        self.icons.len() < len
    }

    /// Remove all icons.
    pub fn clear(&mut self) {
        self.icons.clear();
    }

    /// Select a specific icon by ID. Deselects all others.
    pub fn select(&mut self, id: &str) {
        for icon in &mut self.icons {
            icon.selected = icon.id == id;
        }
    }

    /// Select a range of icons by index (inclusive).
    pub fn select_range(&mut self, from: usize, to: usize) {
        let (lo, hi) = if from <= to { (from, to) } else { (to, from) };
        for (i, icon) in self.icons.iter_mut().enumerate() {
            icon.selected = i >= lo && i <= hi;
        }
    }

    /// Deselect all icons.
    pub fn deselect_all(&mut self) {
        for icon in &mut self.icons {
            icon.selected = false;
        }
    }

    /// Toggle the selection state of an icon by ID.
    pub fn toggle_select(&mut self, id: &str) {
        if let Some(icon) = self.icons.iter_mut().find(|i| i.id == id) {
            icon.selected = !icon.selected;
        }
    }

    /// Get a list of references to all selected icons.
    pub fn selected(&self) -> Vec<&DesktopIcon> {
        self.icons.iter().filter(|i| i.selected).collect()
    }

    /// Count of selected icons.
    pub fn selected_count(&self) -> usize {
        self.icons.iter().filter(|i| i.selected).count()
    }

    /// Total number of icons.
    pub fn count(&self) -> usize {
        self.icons.len()
    }

    /// Calculate the screen position for an icon at the given grid index.
    pub fn grid_position(&self, index: usize) -> (i32, i32) {
        let col = index as u32 % self.grid_columns;
        let row = index as u32 / self.grid_columns;
        let x = (col as i32) * (self.icon_size + self.grid_spacing);
        let y = (row as i32) * (self.icon_size + self.grid_spacing);
        (x, y)
    }

    /// Set the spacing between grid cells.
    pub fn set_grid_spacing(&mut self, spacing: i32) {
        self.grid_spacing = spacing;
    }

    /// Get the grid spacing.
    pub fn grid_spacing(&self) -> i32 {
        self.grid_spacing
    }

    /// Set the icon size in pixels.
    pub fn set_icon_size(&mut self, size: i32) {
        self.icon_size = size;
    }

    /// Get the icon size.
    pub fn icon_size(&self) -> i32 {
        self.icon_size
    }

    /// Set the number of columns in the grid.
    pub fn set_grid_columns(&mut self, cols: u32) {
        self.grid_columns = cols.max(1);
    }

    /// Get the number of grid columns.
    pub fn grid_columns(&self) -> u32 {
        self.grid_columns
    }

    /// Arrange all icons into grid positions based on their index.
    pub fn arrange_grid(&mut self) {
        let positions: Vec<(i32, i32)> = (0..self.icons.len())
            .map(|i| self.grid_position(i))
            .collect();
        for (icon, (x, y)) in self.icons.iter_mut().zip(positions) {
            icon.x = x;
            icon.y = y;
        }
    }

    /// Check if the Home icon is shown.
    pub fn show_home(&self) -> bool {
        self.show_home
    }

    /// Set whether the Home icon is shown.
    pub fn set_show_home(&mut self, show: bool) {
        self.show_home = show;
    }

    /// Check if the Trash icon is shown.
    pub fn show_trash(&self) -> bool {
        self.show_trash
    }

    /// Set whether the Trash icon is shown.
    pub fn set_show_trash(&mut self, show: bool) {
        self.show_trash = show;
    }

    /// Check if mounted volumes are shown.
    pub fn show_mounts(&self) -> bool {
        self.show_mounts
    }

    /// Set whether mounted volumes are shown.
    pub fn set_show_mounts(&mut self, show: bool) {
        self.show_mounts = show;
    }

    /// Get the default set of desktop icons.
    pub fn default_icons(&self) -> Vec<DesktopIcon> {
        Self::default_icons_internal()
    }

    fn default_icons_internal() -> Vec<DesktopIcon> {
        vec![
            DesktopIcon {
                id: "home".into(),
                label: "Home".into(),
                icon_name: "user-home".into(),
                x: 16,
                y: 16,
                selected: false,
            },
            DesktopIcon {
                id: "trash".into(),
                label: "Trash".into(),
                icon_name: "user-trash".into(),
                x: 16,
                y: 96,
                selected: false,
            },
        ]
    }
}

impl Default for DesktopIconManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_icon(id: &str, label: &str) -> DesktopIcon {
        DesktopIcon {
            id: id.into(),
            label: label.into(),
            icon_name: "unknown".into(),
            x: 0, y: 0,
            selected: false,
        }
    }

    #[test]
    fn test_icon_manager_creation() {
        let mgr = DesktopIconManager::new();
        assert_eq!(mgr.count(), 2); // home + trash by default
        assert_eq!(mgr.grid_spacing(), 80);
        assert_eq!(mgr.icon_size(), 64);
        assert_eq!(mgr.grid_columns(), 7);
    }

    #[test]
    fn test_set_icons() {
        let mut mgr = DesktopIconManager::new();
        let icons = vec![
            make_icon("a", "A"),
            make_icon("b", "B"),
            make_icon("c", "C"),
        ];
        mgr.set_icons(icons);
        assert_eq!(mgr.count(), 3);
    }

    #[test]
    fn test_add_icon() {
        let mut mgr = DesktopIconManager::new();
        assert!(mgr.add_icon(make_icon("new-icon", "New")));
        assert_eq!(mgr.count(), 3);
    }

    #[test]
    fn test_add_duplicate_icon() {
        let mut mgr = DesktopIconManager::new();
        assert!(!mgr.add_icon(make_icon("home", "Duplicate")));
        assert_eq!(mgr.count(), 2);
    }

    #[test]
    fn test_remove_icon() {
        let mut mgr = DesktopIconManager::new();
        assert!(mgr.remove_icon("home"));
        assert_eq!(mgr.count(), 1);
        assert!(!mgr.remove_icon("nonexistent"));
    }

    #[test]
    fn test_clear_icons() {
        let mut mgr = DesktopIconManager::new();
        mgr.clear();
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_select_icon() {
        let mut mgr = DesktopIconManager::new();
        mgr.select("home");
        assert!(mgr.icons()[0].selected);
        assert!(!mgr.icons()[1].selected);
    }

    #[test]
    fn test_select_range() {
        let mut mgr = DesktopIconManager::new();
        mgr.add_icon(make_icon("c", "C"));
        mgr.select_range(0, 1);
        assert!(mgr.icons()[0].selected);
        assert!(mgr.icons()[1].selected);
        assert!(!mgr.icons()[2].selected);
    }

    #[test]
    fn test_deselect_all() {
        let mut mgr = DesktopIconManager::new();
        mgr.select("home");
        assert!(mgr.selected_count() > 0);
        mgr.deselect_all();
        assert_eq!(mgr.selected_count(), 0);
    }

    #[test]
    fn test_toggle_select() {
        let mut mgr = DesktopIconManager::new();
        mgr.toggle_select("home");
        assert!(mgr.icons()[0].selected);
        mgr.toggle_select("home");
        assert!(!mgr.icons()[0].selected);
    }

    #[test]
    fn test_selected_icons() {
        let mut mgr = DesktopIconManager::new();
        mgr.select("home");
        let sel = mgr.selected();
        assert_eq!(sel.len(), 1);
        assert_eq!(sel[0].id, "home");
    }

    #[test]
    fn test_grid_position() {
        let mgr = DesktopIconManager::new();
        let (x0, y0) = mgr.grid_position(0);
        assert_eq!(x0, 0);
        assert_eq!(y0, 0);
        let (x1, y1) = mgr.grid_position(1);
        assert_eq!(x1, 64 + 80);
        assert_eq!(y1, 0);
    }

    #[test]
    fn test_arrange_grid() {
        let mut mgr = DesktopIconManager::new();
        mgr.add_icon(make_icon("c", "C"));
        mgr.add_icon(make_icon("d", "D"));
        mgr.arrange_grid();
        let (x0, y0) = (mgr.icons()[0].x, mgr.icons()[0].y);
        let (x3, _y3) = (mgr.icons()[3].x, mgr.icons()[3].y);
        assert_eq!(x0, 0);
        assert_eq!(y0, 0);
        assert_ne!(x3, 0); // should be positioned in the grid
    }

    #[test]
    fn test_set_grid_columns() {
        let mut mgr = DesktopIconManager::new();
        mgr.set_grid_columns(5);
        assert_eq!(mgr.grid_columns(), 5);
        mgr.set_grid_columns(0);
        assert_eq!(mgr.grid_columns(), 1); // minimum 1
    }

    #[test]
    fn test_default_icons_list() {
        let mgr = DesktopIconManager::new();
        let defaults = mgr.default_icons();
        assert_eq!(defaults.len(), 2);
        assert_eq!(defaults[0].id, "home");
        assert_eq!(defaults[1].id, "trash");
    }

    #[test]
    fn test_show_home_trash_mounts() {
        let mut mgr = DesktopIconManager::new();
        assert!(mgr.show_home());
        mgr.set_show_home(false);
        assert!(!mgr.show_home());
        assert!(mgr.show_trash());
        mgr.set_show_trash(false);
        assert!(!mgr.show_trash());
        assert!(mgr.show_mounts());
        mgr.set_show_mounts(false);
        assert!(!mgr.show_mounts());
    }
}
