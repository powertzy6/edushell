// SPDX-License-Identifier: GPL-3.0-or-later

//! # Application Launcher
//!
//! Overlay window for launching applications. Provides
//! search, category filtering, recent and pinned apps.
//! Opens with Alt+Space or Super key.
//!
//! ## Layout
//!
//! ```text
//! +---------------------------------------------+
//! |  Search...                          Ctrl+Q |
//! +---------------------------------------------+
//! | All Apps | Frequently Used | Recent         |
//! +------+---------------------------------------+
//! | Cat1 |  app1  app2  app3                    |
//! | Cat2 |  app4  app5  app6                    |
//! | Cat3 |  app7  app8  app9                    |
//! | ...  |                                      |
//! +------+---------------------------------------+
//! ```

use crate::localization::LocalizationManager;
use crate::accessibility::AccessibilityManager;
use edushell_core::event::EventBus;

/// Launcher visibility state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LauncherState {
    Closed,
    Opening,
    Open,
    Closing,
}

/// Category filter for the launcher.
#[derive(Debug, Clone, PartialEq)]
pub struct AppCategory {
    pub id: String,
    pub name: String,
    pub icon: String,
}

/// An application entry in the launcher.
#[derive(Debug, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub category: String,
    pub exec: String,
}

/// The application launcher.
pub struct Launcher {
    state: LauncherState,
    search_query: String,
    selected_category: Option<String>,
    categories: Vec<AppCategory>,
    all_apps: Vec<AppEntry>,
    filtered_apps: Vec<AppEntry>,
    recent_apps: Vec<String>,
    pinned_apps: Vec<String>,
    selected_index: usize,
    localization: LocalizationManager,
    accessibility: AccessibilityManager,
    event_bus: EventBus,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
    #[cfg(feature = "gtk")]
    search_entry: Option<gtk::SearchEntry>,
    #[cfg(feature = "gtk")]
    app_flow: Option<gtk::FlowBox>,
}

impl Launcher {
    pub fn new(
        localization: LocalizationManager,
        accessibility: AccessibilityManager,
        event_bus: EventBus,
    ) -> Self {
        let categories = vec![
            AppCategory { id: "all".into(), name: localization.translate("All Applications"), icon: "application-x-executable".into() },
            AppCategory { id: "Development".into(), name: localization.translate("Development"), icon: "applications-development".into() },
            AppCategory { id: "Education".into(), name: localization.translate("Education"), icon: "applications-education".into() },
            AppCategory { id: "Game".into(), name: localization.translate("Games"), icon: "applications-games".into() },
            AppCategory { id: "Graphics".into(), name: localization.translate("Graphics"), icon: "applications-graphics".into() },
            AppCategory { id: "Network".into(), name: localization.translate("Internet"), icon: "applications-internet".into() },
            AppCategory { id: "Office".into(), name: localization.translate("Office"), icon: "applications-office".into() },
            AppCategory { id: "AudioVideo".into(), name: localization.translate("Sound & Video"), icon: "applications-multimedia".into() },
            AppCategory { id: "System".into(), name: localization.translate("System"), icon: "applications-system".into() },
            AppCategory { id: "Utility".into(), name: localization.translate("Utilities"), icon: "applications-utilities".into() },
        ];
        Self {
            state: LauncherState::Closed,
            search_query: String::new(),
            selected_category: None,
            categories,
            all_apps: Vec::new(),
            filtered_apps: Vec::new(),
            recent_apps: Vec::new(),
            pinned_apps: Vec::new(),
            selected_index: 0,
            localization,
            accessibility,
            event_bus,
            #[cfg(feature = "gtk")]
            window: None,
            #[cfg(feature = "gtk")]
            search_entry: None,
            #[cfg(feature = "gtk")]
            app_flow: None,
        }
    }

    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {}

    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        // GTK widgets are constructed here when the feature is enabled.
    }

    pub fn destroy(&self) {
        #[cfg(feature = "gtk")]
        if let Some(ref w) = self.window {
            w.close();
        }
    }

    pub fn open(&mut self) {
        if self.state == LauncherState::Closed || self.state == LauncherState::Closing {
            self.state = LauncherState::Opening;
            self.search_query.clear();
            self.selected_index = 0;
            self.selected_category = None;
            self.perform_search();
            self.state = LauncherState::Open;
        }
    }

    pub fn close(&mut self) {
        if self.state == LauncherState::Open || self.state == LauncherState::Opening {
            self.state = LauncherState::Closing;
            self.search_query.clear();
            self.state = LauncherState::Closed;
        }
    }

    pub fn toggle(&mut self) {
        if self.is_open() {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn is_open(&self) -> bool {
        self.state == LauncherState::Open || self.state == LauncherState::Opening
    }

    pub fn state(&self) -> LauncherState {
        self.state
    }

    pub fn set_search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.selected_index = 0;
        self.perform_search();
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn select_category(&mut self, category_id: &str) {
        if category_id == "all" {
            self.selected_category = None;
        } else {
            self.selected_category = Some(category_id.to_string());
        }
        self.selected_index = 0;
        self.perform_search();
    }

    pub fn selected_category(&self) -> Option<&str> {
        self.selected_category.as_deref()
    }

    pub fn categories(&self) -> &[AppCategory] {
        &self.categories
    }

    pub fn set_apps(&mut self, apps: Vec<AppEntry>) {
        self.all_apps = apps;
        self.perform_search();
    }

    pub fn filtered_apps(&self) -> &[AppEntry] {
        &self.filtered_apps
    }

    pub fn add_recent(&mut self, app_id: &str) {
        self.recent_apps.retain(|a| a != app_id);
        self.recent_apps.insert(0, app_id.to_string());
        if self.recent_apps.len() > 50 {
            self.recent_apps.truncate(50);
        }
    }

    pub fn pin_app(&mut self, app_id: &str) {
        if !self.pinned_apps.contains(&app_id.to_string()) {
            self.pinned_apps.push(app_id.to_string());
        }
    }

    pub fn unpin_app(&mut self, app_id: &str) {
        self.pinned_apps.retain(|a| a != app_id);
    }

    pub fn select_next(&mut self) {
        let len = self.filtered_apps.len();
        if len == 0 {
            return;
        }
        self.selected_index = (self.selected_index + 1) % len;
    }

    pub fn select_prev(&mut self) {
        let len = self.filtered_apps.len();
        if len == 0 {
            return;
        }
        self.selected_index = if self.selected_index == 0 {
            len - 1
        } else {
            self.selected_index - 1
        };
    }

    pub fn selected_app(&self) -> Option<&AppEntry> {
        self.filtered_apps.get(self.selected_index)
    }

    pub fn launch_selected(&self) -> Option<&AppEntry> {
        self.selected_app()
    }

    fn perform_search(&mut self) {
        let mut apps: Vec<AppEntry> = self.all_apps.clone();

        // Apply category filter
        if let Some(ref cat) = self.selected_category {
            if cat != "all" {
                apps = self.filter_by_category(&apps);
            }
        }

        // Apply search query
        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            let query_words: Vec<&str> = query.split_whitespace().collect();
            apps = apps
                .into_iter()
                .filter(|app| {
                    let name_lower = app.name.to_lowercase();
                    let desc_lower = app.description.to_lowercase();
                    if query_words.is_empty() {
                        return true;
                    }
                    if query_words.len() == 1 && query_words[0].len() <= 3 {
                        return name_lower.contains(query_words[0])
                            || desc_lower.contains(query_words[0]);
                    }
                    query_words.iter().all(|w| {
                        Self::fuzzy_match(w, &name_lower)
                            || Self::fuzzy_match(w, &desc_lower)
                    })
                })
                .collect();
        }

        // Sort: pinned apps first, then by name
        let pinned = &self.pinned_apps;
        apps.sort_by(|a, b| {
            let a_pinned = pinned.contains(&a.id);
            let b_pinned = pinned.contains(&b.id);
            if a_pinned != b_pinned {
                return b_pinned.cmp(&a_pinned);
            }
            a.name.cmp(&b.name)
        });

        self.filtered_apps = apps;
        if self.selected_index >= self.filtered_apps.len() && !self.filtered_apps.is_empty() {
            self.selected_index = 0;
        }
    }

    fn filter_by_category(&self, apps: &[AppEntry]) -> Vec<AppEntry> {
        match self.selected_category {
            Some(ref cat) => apps
                .iter()
                .filter(|a| a.category == *cat)
                .cloned()
                .collect(),
            None => apps.to_vec(),
        }
    }

    fn fuzzy_match(query: &str, text: &str) -> bool {
        let query = query.to_lowercase();
        let text = text.to_lowercase();
        if query.is_empty() {
            return true;
        }
        let q_chars: Vec<char> = query.chars().collect();
        let t_chars: Vec<char> = text.chars().collect();
        let mut qi = 0;
        for &tc in &t_chars {
            if qi < q_chars.len() && tc == q_chars[qi] {
                qi += 1;
            }
        }
        qi == q_chars.len()
    }
}

impl std::fmt::Debug for Launcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Launcher")
            .field("state", &self.state)
            .field("search_query", &self.search_query)
            .field("selected_category", &self.selected_category)
            .field("filtered_apps", &self.filtered_apps.len())
            .field("selected_index", &self.selected_index)
            .field("all_apps", &self.all_apps.len())
            .field("recent_apps", &self.recent_apps.len())
            .field("pinned_apps", &self.pinned_apps.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::localization::LocalizationManager;
    use crate::accessibility::AccessibilityManager;
    use edushell_core::event::EventBus;

    fn make_locale() -> LocalizationManager {
        LocalizationManager::new()
    }

    fn make_access() -> AccessibilityManager {
        AccessibilityManager::new()
    }

    fn make_bus() -> EventBus {
        EventBus::new()
    }

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
            AppEntry {
                id: "vlc".into(),
                name: "VLC media player".into(),
                icon: "vlc".into(),
                description: "Media Player".into(),
                category: "AudioVideo".into(),
                exec: "vlc".into(),
            },
            AppEntry {
                id: "gnome-terminal".into(),
                name: "Terminal".into(),
                icon: "terminal".into(),
                description: "Command line".into(),
                category: "System".into(),
                exec: "gnome-terminal".into(),
            },
            AppEntry {
                id: "solitaire".into(),
                name: "Solitaire".into(),
                icon: "solitaire".into(),
                description: "Card Game".into(),
                category: "Game".into(),
                exec: "solitaire".into(),
            },
            AppEntry {
                id: "calculator".into(),
                name: "Calculator".into(),
                icon: "calculator".into(),
                description: "Scientific Calculator".into(),
                category: "Utility".into(),
                exec: "gnome-calculator".into(),
            },
        ]
    }

    #[test]
    fn test_creation() {
        let launcher = Launcher::new(make_locale(), make_access(), make_bus());
        assert_eq!(launcher.state(), LauncherState::Closed);
        assert!(launcher.search_query().is_empty());
        assert!(launcher.selected_category().is_none());
        assert!(launcher.filtered_apps().is_empty());
        assert!(!launcher.is_open());
    }

    #[test]
    fn test_state_transitions() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        assert_eq!(launcher.state(), LauncherState::Closed);

        launcher.open();
        assert!(launcher.is_open());
        assert_eq!(launcher.state(), LauncherState::Open);

        launcher.close();
        assert_eq!(launcher.state(), LauncherState::Closed);
        assert!(!launcher.is_open());
    }

    #[test]
    fn test_toggle() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        assert!(!launcher.is_open());

        launcher.toggle();
        assert!(launcher.is_open());

        launcher.toggle();
        assert!(!launcher.is_open());
    }

    #[test]
    fn test_search_filtering() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.set_search("firefox");
        assert_eq!(launcher.filtered_apps().len(), 1);
        assert_eq!(launcher.filtered_apps()[0].name, "Firefox");

        launcher.set_search("editor");
        assert_eq!(launcher.filtered_apps().len(), 2); // GIMP (Image Editor) and VS Code (Code Editor)
    }

    #[test]
    fn test_search_partial_match() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.set_search("calc");
        assert_eq!(launcher.filtered_apps().len(), 1);
        assert_eq!(launcher.filtered_apps()[0].id, "calculator");
    }

    #[test]
    fn test_search_empty_returns_all() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.set_search("");
        assert_eq!(launcher.filtered_apps().len(), sample_apps().len());
    }

    #[test]
    fn test_search_no_match() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.set_search("xyznonexistent");
        assert!(launcher.filtered_apps().is_empty());
    }

    #[test]
    fn test_category_selection() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.select_category("Network");
        assert_eq!(launcher.selected_category(), Some("Network"));
        assert_eq!(launcher.filtered_apps().len(), 1);
        assert_eq!(launcher.filtered_apps()[0].id, "firefox");
    }

    #[test]
    fn test_category_all() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.select_category("all");
        assert_eq!(launcher.selected_category(), None);
        assert_eq!(launcher.filtered_apps().len(), sample_apps().len());
    }

    #[test]
    fn test_select_next_prev_wrap() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());
        assert_eq!(launcher.selected_index, 0);

        launcher.select_next();
        assert_eq!(launcher.selected_index, 1);

        launcher.select_next();
        assert_eq!(launcher.selected_index, 2);

        // Wrap around: go to last then first
        launcher.selected_index = sample_apps().len() - 1;
        launcher.select_next();
        assert_eq!(launcher.selected_index, 0);
    }

    #[test]
    fn test_select_prev_wrap() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        // select_prev from 0 should wrap to last
        launcher.select_prev();
        assert_eq!(launcher.selected_index, sample_apps().len() - 1);
    }

    #[test]
    fn test_selected_app_none_when_empty() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(vec![]);
        assert!(launcher.selected_app().is_none());
    }

    #[test]
    fn test_selected_app() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());
        let selected = launcher.selected_app();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id, "calculator"); // first alphabetically
    }

    #[test]
    fn test_fuzzy_match() {
        assert!(Launcher::fuzzy_match("ff", "firefox"));
        assert!(Launcher::fuzzy_match("fx", "firefox"));
        assert!(Launcher::fuzzy_match("frefox", "firefox"));
        assert!(!Launcher::fuzzy_match("xyz", "firefox"));
        assert!(Launcher::fuzzy_match("", "anything"));
        assert!(!Launcher::fuzzy_match("abc", ""));
    }

    #[test]
    fn test_pin_unpin() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.pin_app("calculator");
        // Pinned apps come first in filtered results
        assert_eq!(launcher.filtered_apps()[0].id, "calculator");

        launcher.unpin_app("calculator");
        // After unpinning, should sort by name again
        assert_eq!(launcher.filtered_apps()[0].id, "calculator"); // calculator is first alphabetically anyway
    }

    #[test]
    fn test_pin_app_idempotent() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.pin_app("firefox");
        launcher.pin_app("firefox");
        // Should only appear once
        let count = launcher.pinned_apps.iter().filter(|&id| id == "firefox").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_recent_apps() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.add_recent("firefox");
        launcher.add_recent("code");
        launcher.add_recent("gimp");

        assert_eq!(launcher.recent_apps.len(), 3);
        assert_eq!(launcher.recent_apps[0], "gimp"); // most recent first

        // Re-adding moves to front
        launcher.add_recent("firefox");
        assert_eq!(launcher.recent_apps[0], "firefox");
        assert_eq!(launcher.recent_apps.len(), 3);
    }

    #[test]
    fn test_recent_apps_capped() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        for i in 0..60 {
            launcher.add_recent(&format!("app-{}", i));
        }
        assert_eq!(launcher.recent_apps.len(), 50);
    }

    #[test]
    fn test_filter_by_category() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        let filtered = launcher.filter_by_category(&sample_apps());
        // With no category selected, returns all
        assert_eq!(filtered.len(), sample_apps().len());

        launcher.select_category("Game");
        let filtered = launcher.filter_by_category(&sample_apps());
        // With category selected, only matches
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "solitaire");
    }

    #[test]
    fn test_launch_selected() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());
        let entry = launcher.launch_selected();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().exec, "gnome-calculator");
    }

    #[test]
    fn test_categories() {
        let launcher = Launcher::new(make_locale(), make_access(), make_bus());
        assert!(launcher.categories().len() >= 10);
        assert_eq!(launcher.categories()[0].id, "all");
    }

    #[test]
    fn test_set_search_multiple_times() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.set_search("firefox");
        assert_eq!(launcher.filtered_apps().len(), 1);

        launcher.set_search("media");
        assert_eq!(launcher.filtered_apps().len(), 1);
        assert_eq!(launcher.filtered_apps()[0].id, "vlc");

        launcher.set_search("");
        assert_eq!(launcher.filtered_apps().len(), sample_apps().len());
    }

    #[test]
    fn test_select_index_reset_on_search() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.select_next();
        launcher.select_next();
        assert_eq!(launcher.selected_index, 2);

        launcher.set_search("firefox");
        assert_eq!(launcher.selected_index, 0);
    }

    #[test]
    fn test_select_index_reset_on_category() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(sample_apps());

        launcher.select_next();
        launcher.select_next();
        assert_eq!(launcher.selected_index, 2);

        launcher.select_category("Office");
        assert_eq!(launcher.selected_index, 0);
    }

    #[test]
    fn test_debug_format() {
        let launcher = Launcher::new(make_locale(), make_access(), make_bus());
        let debug = format!("{:?}", launcher);
        assert!(debug.contains("Closed"));
        assert!(debug.contains("filtered_apps"));
    }

    #[test]
    fn test_open_clears_search() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_search("test");
        launcher.close();
        launcher.open();
        assert!(launcher.search_query().is_empty());
    }

    #[test]
    fn test_select_next_no_op_when_empty() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(vec![]);
        launcher.select_next();
        assert!(launcher.selected_app().is_none());
    }

    #[test]
    fn test_select_prev_no_op_when_empty() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.set_apps(vec![]);
        launcher.select_prev();
        assert!(launcher.selected_app().is_none());
    }

    #[test]
    fn test_destroy() {
        let launcher = Launcher::new(make_locale(), make_access(), make_bus());
        // Should not panic
        launcher.destroy();
    }

    #[test]
    fn test_open_idempotent() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.open();
        launcher.open();
        assert_eq!(launcher.state(), LauncherState::Open);
    }

    #[test]
    fn test_close_idempotent() {
        let mut launcher = Launcher::new(make_locale(), make_access(), make_bus());
        launcher.close();
        launcher.close();
        assert_eq!(launcher.state(), LauncherState::Closed);
    }
}
