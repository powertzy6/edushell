// SPDX-License-Identifier: GPL-3.0-or-later

//! # Running Applications
//!
//! Shows icons of currently running applications in the panel.
//! Clicking an icon focuses or activates the corresponding window.

/// Tracks and displays currently running applications in the panel.
pub struct RunningApps {
    app_ids: Vec<String>,
    #[cfg(feature = "gtk")]
    widget: Option<gtk::Box>,
}

impl RunningApps {
    /// Create a new running applications tracker.
    pub fn new() -> Self {
        Self {
            app_ids: Vec::new(),
            #[cfg(feature = "gtk")]
            widget: None,
        }
    }

    /// Build the GTK widget for this component.
    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        let box_ = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        box_.add_css_class("running-apps");
    }

    /// Build stub for non-GTK mode.
    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {
        tracing::info!(target: "edushell::ui::panel::running_apps", "Running apps stub: {} apps", self.app_ids.len());
    }

    /// Add a running application by its app ID.
    pub fn add_app(&mut self, app_id: &str) {
        if !self.app_ids.contains(&app_id.to_string()) {
            self.app_ids.push(app_id.to_string());
        }
    }

    /// Remove a running application by its app ID.
    pub fn remove_app(&mut self, app_id: &str) {
        self.app_ids.retain(|id| id != app_id);
    }

    /// Check if an app ID is currently tracked.
    pub fn has_app(&self, app_id: &str) -> bool {
        self.app_ids.contains(&app_id.to_string())
    }

    /// Get the number of tracked running applications.
    pub fn app_count(&self) -> usize {
        self.app_ids.len()
    }

    /// Get a slice of all tracked app IDs.
    pub fn app_ids(&self) -> &[String] {
        &self.app_ids
    }

    /// Remove all running applications.
    pub fn clear(&mut self) {
        self.app_ids.clear();
    }
}

impl Default for RunningApps {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RunningApps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunningApps")
            .field("app_count", &self.app_ids.len())
            .field("app_ids", &self.app_ids)
            .finish()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_apps() -> RunningApps {
        RunningApps::new()
    }

    #[test]
    fn test_new_is_empty() {
        let apps = make_apps();
        assert_eq!(apps.app_count(), 0);
        assert!(apps.app_ids().is_empty());
    }

    #[test]
    fn test_add_app() {
        let mut apps = make_apps();
        apps.add_app("firefox.desktop");
        assert!(apps.has_app("firefox.desktop"));
        assert_eq!(apps.app_count(), 1);
    }

    #[test]
    fn test_add_duplicate_app() {
        let mut apps = make_apps();
        apps.add_app("firefox.desktop");
        apps.add_app("firefox.desktop");
        assert_eq!(apps.app_count(), 1);
    }

    #[test]
    fn test_remove_app() {
        let mut apps = make_apps();
        apps.add_app("firefox.desktop");
        apps.add_app("terminal.desktop");
        apps.remove_app("firefox.desktop");
        assert!(!apps.has_app("firefox.desktop"));
        assert!(apps.has_app("terminal.desktop"));
        assert_eq!(apps.app_count(), 1);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut apps = make_apps();
        apps.add_app("firefox.desktop");
        apps.remove_app("nonexistent.desktop");
        assert_eq!(apps.app_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut apps = make_apps();
        apps.add_app("a.desktop");
        apps.add_app("b.desktop");
        apps.add_app("c.desktop");
        apps.clear();
        assert_eq!(apps.app_count(), 0);
        assert!(apps.app_ids().is_empty());
    }

    #[test]
    fn test_app_ids_slice() {
        let mut apps = make_apps();
        apps.add_app("alpha.desktop");
        apps.add_app("beta.desktop");
        let ids = apps.app_ids();
        assert_eq!(ids.len(), 2);
        assert_eq!(ids[0], "alpha.desktop");
        assert_eq!(ids[1], "beta.desktop");
    }

    #[test]
    fn test_multiple_adds() {
        let mut apps = make_apps();
        let ids = ["a", "b", "c", "d", "e"];
        for id in &ids {
            apps.add_app(id);
        }
        assert_eq!(apps.app_count(), 5);
    }

    #[test]
    fn test_has_app_empty() {
        let apps = make_apps();
        assert!(!apps.has_app("anything"));
    }

    #[test]
    fn test_has_app_after_remove() {
        let mut apps = make_apps();
        apps.add_app("keep.desktop");
        apps.add_app("remove.desktop");
        apps.remove_app("remove.desktop");
        assert!(apps.has_app("keep.desktop"));
        assert!(!apps.has_app("remove.desktop"));
    }

    #[test]
    fn test_default_trait() {
        let apps = RunningApps::default();
        assert_eq!(apps.app_count(), 0);
    }

    #[test]
    fn test_debug_output() {
        let mut apps = make_apps();
        apps.add_app("test.desktop");
        let debug = format!("{apps:?}");
        assert!(debug.contains("RunningApps"));
        assert!(debug.contains("test.desktop"));
    }

    #[test]
    fn test_build_does_not_panic() {
        let apps = make_apps();
        apps.build();
    }

    #[test]
    fn test_order_preserved() {
        let mut apps = make_apps();
        apps.add_app("first");
        apps.add_app("second");
        apps.add_app("third");
        assert_eq!(apps.app_ids()[0], "first");
        assert_eq!(apps.app_ids()[1], "second");
        assert_eq!(apps.app_ids()[2], "third");
    }
}
