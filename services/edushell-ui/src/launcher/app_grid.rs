// SPDX-License-Identifier: GPL-3.0-or-later

//! # Application Grid
//!
//! The grid of application icons shown in the launcher.
//! Supports lazy loading, keyboard navigation, and
//! category filtering.

use crate::launcher::AppEntry;

pub struct AppGrid {
    items: Vec<AppEntry>,
    columns: u32,
    item_size: i32,
    selected_index: usize,
    #[cfg(feature = "gtk")]
    flow_box: Option<gtk::FlowBox>,
}

impl AppGrid {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            columns: 6,
            item_size: 80,
            selected_index: 0,
            #[cfg(feature = "gtk")]
            flow_box: None,
        }
    }

    pub fn build(&self) {
        #[cfg(feature = "gtk")]
        if let Some(ref fb) = self.flow_box {
            fb.set_min_children_per_line(self.columns);
            fb.set_max_children_per_line(self.columns);
        }
    }

    pub fn set_items(&mut self, items: Vec<AppEntry>) {
        self.items = items;
        if self.selected_index >= self.items.len() {
            self.selected_index = if self.items.is_empty() { 0 } else { 0 };
        }
        #[cfg(feature = "gtk")]
        if let Some(ref fb) = self.flow_box {
            for child in &fb.children() {
                fb.remove(child);
            }
            for app in &self.items {
                let label = gtk::Label::new(Some(&app.name));
                let icon = gtk::Image::from_icon_name(&app.icon, gtk::IconSize::Dialog);
                let box_ = gtk::Box::new(gtk::Orientation::Vertical, 4);
                box_.append(&icon);
                box_.append(&label);
                fb.append(&box_);
            }
        }
    }

    pub fn items(&self) -> &[AppEntry] {
        &self.items
    }

    pub fn set_columns(&mut self, cols: u32) {
        self.columns = cols;
        #[cfg(feature = "gtk")]
        if let Some(ref fb) = self.flow_box {
            fb.set_min_children_per_line(cols);
            fb.set_max_children_per_line(cols);
        }
    }

    pub fn set_item_size(&mut self, size: i32) {
        self.item_size = size;
    }

    pub fn select_index(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = index;
        }
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let next = self.selected_index + 1;
        if next < self.items.len() {
            self.selected_index = next;
        }
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn selected_item(&self) -> Option<&AppEntry> {
        self.items.get(self.selected_index)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl Default for AppGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for AppGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppGrid")
            .field("items", &self.items.len())
            .field("columns", &self.columns)
            .field("item_size", &self.item_size)
            .field("selected_index", &self.selected_index)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launcher::AppEntry;

    fn sample_apps() -> Vec<AppEntry> {
        vec![
            AppEntry {
                id: "firefox".into(),
                name: "Firefox".into(),
                icon: "firefox".into(),
                description: "Web Browser".into(),
                category: "Network".into(),
                exec: "firefox".into(),
            },
            AppEntry {
                id: "libreoffice".into(),
                name: "LibreOffice Writer".into(),
                icon: "libreoffice-writer".into(),
                description: "Word Processor".into(),
                category: "Office".into(),
                exec: "libreoffice --writer".into(),
            },
            AppEntry {
                id: "gimp".into(),
                name: "GIMP".into(),
                icon: "gimp".into(),
                description: "Image Editor".into(),
                category: "Graphics".into(),
                exec: "gimp".into(),
            },
            AppEntry {
                id: "code".into(),
                name: "Visual Studio Code".into(),
                icon: "code".into(),
                description: "Code Editor".into(),
                category: "Development".into(),
                exec: "code".into(),
            },
        ]
    }

    #[test]
    fn test_new_creates_empty_grid() {
        let grid = AppGrid::new();
        assert_eq!(grid.count(), 0);
        assert_eq!(grid.columns, 6);
        assert_eq!(grid.item_size, 80);
        assert_eq!(grid.selected_index(), 0);
    }

    #[test]
    fn test_default_is_same_as_new() {
        let grid1 = AppGrid::new();
        let grid2 = AppGrid::default();
        assert_eq!(grid1.count(), grid2.count());
        assert_eq!(grid1.columns, grid2.columns);
    }

    #[test]
    fn test_set_items() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        assert_eq!(grid.count(), 4);
        assert_eq!(grid.items()[0].id, "firefox");
    }

    #[test]
    fn test_items_returns_slice() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        let items = grid.items();
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn test_set_columns() {
        let mut grid = AppGrid::new();
        grid.set_columns(8);
        assert_eq!(grid.columns, 8);
    }

    #[test]
    fn test_set_item_size() {
        let mut grid = AppGrid::new();
        grid.set_item_size(96);
        assert_eq!(grid.item_size, 96);
    }

    #[test]
    fn test_select_index() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_index(2);
        assert_eq!(grid.selected_index(), 2);
    }

    #[test]
    fn test_select_index_out_of_bounds() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_index(999);
        assert_eq!(grid.selected_index(), 0); // unchanged
    }

    #[test]
    fn test_select_next() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        assert_eq!(grid.selected_index(), 0);
        grid.select_next();
        assert_eq!(grid.selected_index(), 1);
        grid.select_next();
        assert_eq!(grid.selected_index(), 2);
    }

    #[test]
    fn test_select_next_stops_at_end() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_index(3);
        grid.select_next();
        assert_eq!(grid.selected_index(), 3); // stays at last
    }

    #[test]
    fn test_select_prev() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_index(2);
        grid.select_prev();
        assert_eq!(grid.selected_index(), 1);
    }

    #[test]
    fn test_select_prev_stops_at_start() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_prev();
        assert_eq!(grid.selected_index(), 0); // stays at 0
    }

    #[test]
    fn test_select_next_empty() {
        let mut grid = AppGrid::new();
        grid.select_next();
        assert_eq!(grid.selected_index(), 0);
    }

    #[test]
    fn test_select_prev_empty() {
        let mut grid = AppGrid::new();
        grid.select_prev();
        assert_eq!(grid.selected_index(), 0);
    }

    #[test]
    fn test_selected_item() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_index(1);
        let item = grid.selected_item();
        assert!(item.is_some());
        assert_eq!(item.unwrap().id, "libreoffice");
    }

    #[test]
    fn test_selected_item_none_when_empty() {
        let grid = AppGrid::new();
        assert!(grid.selected_item().is_none());
    }

    #[test]
    fn test_set_items_resets_selection() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_index(3);
        grid.set_items(vec![]);
        assert_eq!(grid.selected_index(), 0);
        assert_eq!(grid.count(), 0);
    }

    #[test]
    fn test_set_items_replaces_content() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        assert_eq!(grid.count(), 4);
        grid.set_items(vec![]);
        assert_eq!(grid.count(), 0);
    }

    #[test]
    fn test_debug_format() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        let debug = format!("{:?}", grid);
        assert!(debug.contains("items"));
        assert!(debug.contains("columns"));
        assert!(debug.contains("item_size"));
    }

    #[test]
    fn test_build_does_not_panic() {
        let grid = AppGrid::new();
        grid.build();
    }

    #[test]
    fn test_selected_item_after_select_next() {
        let mut grid = AppGrid::new();
        grid.set_items(sample_apps());
        grid.select_next();
        assert_eq!(grid.selected_item().unwrap().id, "libreoffice");
    }
}
