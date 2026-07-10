// SPDX-License-Identifier: GPL-3.0-or-later

//! # Category Sidebar
//!
//! Shows application categories for filtering in the launcher.
//! Includes categories like All, Development, Games, Graphics,
//! Internet, Office, Sound & Video, System, Utilities.

use crate::launcher::AppCategory;

/// Default categories for EduShell.
pub fn default_categories() -> Vec<AppCategory> {
    vec![
        AppCategory { id: "all".into(), name: "All Applications".into(), icon: "application-x-executable".into() },
        AppCategory { id: "Development".into(), name: "Development".into(), icon: "applications-development".into() },
        AppCategory { id: "Education".into(), name: "Education".into(), icon: "applications-education".into() },
        AppCategory { id: "Game".into(), name: "Games".into(), icon: "applications-games".into() },
        AppCategory { id: "Graphics".into(), name: "Graphics".into(), icon: "applications-graphics".into() },
        AppCategory { id: "Network".into(), name: "Internet".into(), icon: "applications-internet".into() },
        AppCategory { id: "Office".into(), name: "Office".into(), icon: "applications-office".into() },
        AppCategory { id: "AudioVideo".into(), name: "Sound & Video".into(), icon: "applications-multimedia".into() },
        AppCategory { id: "System".into(), name: "System".into(), icon: "applications-system".into() },
        AppCategory { id: "Utility".into(), name: "Utilities".into(), icon: "applications-utilities".into() },
        AppCategory { id: "Learning".into(), name: "Learning".into(), icon: "applications-education".into() },
    ]
}

pub struct CategorySidebar {
    categories: Vec<AppCategory>,
    selected: Option<String>,
    #[cfg(feature = "gtk")]
    list_box: Option<gtk::ListBox>,
}

impl CategorySidebar {
    pub fn new() -> Self {
        Self {
            categories: default_categories(),
            selected: Some("all".to_string()),
            #[cfg(feature = "gtk")]
            list_box: None,
        }
    }

    pub fn build(&self) {
        #[cfg(feature = "gtk")]
        if let Some(ref lb) = self.list_box {
            for cat in &self.categories {
                let row = gtk::ListBoxRow::new();
                let label = gtk::Label::new(Some(&cat.name));
                let icon = gtk::Image::from_icon_name(&cat.icon, gtk::IconSize::Menu);
                let box_ = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                box_.append(&icon);
                box_.append(&label);
                row.set_child(Some(&box_));
                lb.append(&row);
            }
        }
    }

    pub fn set_categories(&mut self, categories: Vec<AppCategory>) {
        self.categories = categories;
        if !self.categories.iter().any(|c| Some(&c.id) == self.selected.as_ref()) {
            self.selected = self.categories.first().map(|c| c.id.clone());
        }
    }

    pub fn select(&mut self, category_id: &str) {
        if self.categories.iter().any(|c| c.id == category_id) {
            self.selected = Some(category_id.to_string());
        }
    }

    pub fn selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    pub fn categories(&self) -> &[AppCategory] {
        &self.categories
    }

    pub fn select_next(&mut self) {
        let current = self.selected.as_deref();
        let pos = self.categories.iter().position(|c| Some(c.id.as_str()) == current);
        match pos {
            Some(i) if i + 1 < self.categories.len() => {
                self.selected = Some(self.categories[i + 1].id.clone());
            }
            _ => {}
        }
    }

    pub fn select_prev(&mut self) {
        let current = self.selected.as_deref();
        let pos = self.categories.iter().position(|c| Some(c.id.as_str()) == current);
        match pos {
            Some(i) if i > 0 => {
                self.selected = Some(self.categories[i - 1].id.clone());
            }
            _ => {}
        }
    }
}

impl Default for CategorySidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CategorySidebar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CategorySidebar")
            .field("categories", &self.categories.len())
            .field("selected", &self.selected)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launcher::AppCategory;

    #[test]
    fn test_new_has_default_categories() {
        let sidebar = CategorySidebar::new();
        assert_eq!(sidebar.categories().len(), 11);
        assert_eq!(sidebar.selected(), Some("all"));
    }

    #[test]
    fn test_default_is_same_as_new() {
        let s1 = CategorySidebar::new();
        let s2 = CategorySidebar::default();
        assert_eq!(s1.categories().len(), s2.categories().len());
        assert_eq!(s1.selected(), s2.selected());
    }

    #[test]
    fn test_default_categories_contains_known() {
        let cats = default_categories();
        let ids: Vec<&str> = cats.iter().map(|c| c.id.as_str()).collect();
        assert!(ids.contains(&"Development"));
        assert!(ids.contains(&"Game"));
        assert!(ids.contains(&"Graphics"));
        assert!(ids.contains(&"Network"));
        assert!(ids.contains(&"Office"));
        assert!(ids.contains(&"AudioVideo"));
        assert!(ids.contains(&"System"));
        assert!(ids.contains(&"Utility"));
        assert!(ids.contains(&"Learning"));
    }

    #[test]
    fn test_select() {
        let mut sidebar = CategorySidebar::new();
        sidebar.select("Game");
        assert_eq!(sidebar.selected(), Some("Game"));
    }

    #[test]
    fn test_select_invalid_does_nothing() {
        let mut sidebar = CategorySidebar::new();
        sidebar.select("NonExistent");
        assert_eq!(sidebar.selected(), Some("all"));
    }

    #[test]
    fn test_set_categories_replaces() {
        let mut sidebar = CategorySidebar::new();
        let new_cats = vec![
            AppCategory { id: "custom1".into(), name: "Custom 1".into(), icon: "icon1".into() },
            AppCategory { id: "custom2".into(), name: "Custom 2".into(), icon: "icon2".into() },
        ];
        sidebar.set_categories(new_cats);
        assert_eq!(sidebar.categories().len(), 2);
        assert_eq!(sidebar.categories()[0].id, "custom1");
        assert_eq!(sidebar.selected(), Some("custom1"));
    }

    #[test]
    fn test_set_categories_preserves_selection() {
        let mut sidebar = CategorySidebar::new();
        sidebar.select("Development");
        let new_cats = vec![
            AppCategory { id: "Development".into(), name: "Development".into(), icon: "dev".into() },
            AppCategory { id: "Other".into(), name: "Other".into(), icon: "other".into() },
        ];
        sidebar.set_categories(new_cats);
        assert_eq!(sidebar.selected(), Some("Development"));
    }

    #[test]
    fn test_select_next() {
        let mut sidebar = CategorySidebar::new();
        assert_eq!(sidebar.selected(), Some("all"));

        sidebar.select_next();
        assert_eq!(sidebar.selected(), Some("Development"));

        sidebar.select_next();
        assert_eq!(sidebar.selected(), Some("Education"));
    }

    #[test]
    fn test_select_next_stays_at_end() {
        let mut sidebar = CategorySidebar::new();
        // Navigate to the last category
        for _ in 0..20 {
            sidebar.select_next();
        }
        let cats = sidebar.categories();
        assert_eq!(sidebar.selected(), Some(cats.last().unwrap().id.as_str()));
        // Another next should stay
        sidebar.select_next();
        assert_eq!(sidebar.selected(), Some(cats.last().unwrap().id.as_str()));
    }

    #[test]
    fn test_select_prev() {
        let mut sidebar = CategorySidebar::new();
        sidebar.select("Network");
        sidebar.select_prev();
        assert_eq!(sidebar.selected(), Some("Graphics"));
    }

    #[test]
    fn test_select_prev_stays_at_start() {
        let mut sidebar = CategorySidebar::new();
        sidebar.select_prev();
        assert_eq!(sidebar.selected(), Some("all"));
    }

    #[test]
    fn test_categories_returns_correct_slice() {
        let sidebar = CategorySidebar::new();
        let cats = sidebar.categories();
        assert_eq!(cats.len(), 11);
        assert_eq!(cats[0].name, "All Applications");
    }

    #[test]
    fn test_build_does_not_panic() {
        let sidebar = CategorySidebar::new();
        sidebar.build();
    }

    #[test]
    fn test_debug_format() {
        let sidebar = CategorySidebar::new();
        let debug = format!("{:?}", sidebar);
        assert!(debug.contains("categories"));
        assert!(debug.contains("selected"));
    }

    #[test]
    fn test_select_all() {
        let mut sidebar = CategorySidebar::new();
        sidebar.select("all");
        assert_eq!(sidebar.selected(), Some("all"));
    }

    #[test]
    fn test_select_next_from_all() {
        let mut sidebar = CategorySidebar::new();
        assert_eq!(sidebar.selected(), Some("all"));
        sidebar.select_next();
        assert_eq!(sidebar.selected(), Some("Development"));
    }

    #[test]
    fn test_select_prev_from_all() {
        let mut sidebar = CategorySidebar::new();
        // Should stay at "all" since it's first
        sidebar.select_prev();
        assert_eq!(sidebar.selected(), Some("all"));
    }
}
