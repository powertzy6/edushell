// SPDX-License-Identifier: GPL-3.0-or-later

//! # Overview Mode
//!
//! Fullscreen overlay showing workspace thumbnails and
//! running windows. Accessible via Super+Tab or hot corner.
//! Includes integrated search and launcher access.
//!
//! ## Layout
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │  🔍 Search...                       ✕ close │
//! ├─────────────────────────────────────────────┤
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐       │
//! │  │WS 1     │ │WS 2     │ │WS 3     │       │
//! │  │ ┌───┐   │ │ ┌───┐   │ │         │       │
//! │  │ │Win│   │ │ │Win│   │ │         │       │
//! │  │ └───┘   │ │ └───┘   │ │         │       │
//! │  └─────────┘ └─────────┘ └─────────┘       │
//! └─────────────────────────────────────────────┘

use crate::accessibility::AccessibilityManager;
use crate::animation::AnimationEngine;
use crate::multi_monitor::MultiMonitorManager;

/// Overview state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverviewState {
    Closed,
    Opening,
    Open,
    Closing,
}

/// A window thumbnail in overview mode.
#[derive(Debug, Clone)]
pub struct WindowThumbnail {
    pub id: String,
    pub title: String,
    pub app_id: String,
    pub workspace: u32,
    pub minimized: bool,
}

/// Overview mode manager.
pub struct Overview {
    state: OverviewState,
    workspaces: u32,
    active_workspace: u32,
    windows: Vec<WindowThumbnail>,
    search_query: String,
    blur_active: bool,
    monitors: MultiMonitorManager,
    accessibility: AccessibilityManager,
    animation: AnimationEngine,
    animation_progress: f64,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
}

impl Overview {
    /// Create a new overview manager.
    pub fn new(
        monitors: MultiMonitorManager,
        accessibility: AccessibilityManager,
        animation: AnimationEngine,
    ) -> Self {
        Self {
            state: OverviewState::Closed,
            workspaces: 4,
            active_workspace: 0,
            windows: Vec::new(),
            search_query: String::new(),
            blur_active: false,
            monitors,
            accessibility,
            animation,
            animation_progress: 0.0,
            #[cfg(feature = "gtk")]
            window: None,
        }
    }

    /// Build GTK widgets for the overview.
    pub fn build(&self) {
        #[cfg(feature = "gtk")]
        if self.window.is_some() {
            tracing::debug!("Overview GTK window already built");
        }
    }

    /// Destroy GTK widgets and resources.
    pub fn destroy(&self) {
        #[cfg(feature = "gtk")]
        if let Some(ref win) = self.window {
            win.close();
        }
    }

    /// Open the overview overlay.
    pub fn open(&mut self) {
        if self.state == OverviewState::Closed || self.state == OverviewState::Closing {
            self.state = OverviewState::Opening;
            self.animation_progress = 0.0;
            #[cfg(feature = "gtk")]
            if let Some(ref win) = self.window {
                win.present();
            }
            tracing::debug!("Overview opening");
        }
    }

    /// Close the overview overlay.
    pub fn close(&mut self) {
        if self.state == OverviewState::Open || self.state == OverviewState::Opening {
            self.state = OverviewState::Closing;
            self.animation_progress = 0.0;
            #[cfg(feature = "gtk")]
            if let Some(ref win) = self.window {
                win.hide();
            }
            tracing::debug!("Overview closing");
        }
    }

    /// Toggle the overview open/closed.
    pub fn toggle(&mut self) {
        match self.state {
            OverviewState::Closed | OverviewState::Closing => self.open(),
            OverviewState::Open | OverviewState::Opening => self.close(),
        }
    }

    /// Check whether the overview is fully open.
    pub fn is_open(&self) -> bool {
        self.state == OverviewState::Open
    }

    /// Get the current overview state.
    pub fn state(&self) -> OverviewState {
        self.state
    }

    /// Set the number of workspaces.
    pub fn set_workspaces(&mut self, count: u32) {
        self.workspaces = count;
        if self.active_workspace >= count && count > 0 {
            self.active_workspace = count - 1;
        }
    }

    /// Get the number of workspaces.
    pub fn workspaces(&self) -> u32 {
        self.workspaces
    }

    /// Set the active workspace index.
    pub fn set_active_workspace(&mut self, index: u32) {
        if index < self.workspaces {
            self.active_workspace = index;
        }
    }

    /// Get the active workspace index.
    pub fn active_workspace(&self) -> u32 {
        self.active_workspace
    }

    /// Set the list of window thumbnails.
    pub fn set_windows(&mut self, windows: Vec<WindowThumbnail>) {
        self.windows = windows;
    }

    /// Get the list of window thumbnails.
    pub fn windows(&self) -> &[WindowThumbnail] {
        &self.windows
    }

    /// Update the search query.
    pub fn set_search(&mut self, query: &str) {
        self.search_query = query.to_string();
        tracing::debug!(query = %self.search_query, "Overview search updated");
    }

    /// Get the current search query.
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// Navigate to a specific workspace and close overview.
    pub fn go_to_workspace(&mut self, index: u32) {
        if index < self.workspaces {
            self.active_workspace = index;
            self.close();
            tracing::debug!(workspace = index, "Navigated to workspace");
        }
    }

    /// Request focus for a specific window.
    pub fn focus_window(&self, window_id: &str) {
        tracing::debug!(window_id, "Focus window requested");
        #[cfg(feature = "gtk")]
        self.focus_window_gtk(window_id);
    }

    #[cfg(feature = "gtk")]
    fn focus_window_gtk(&self, window_id: &str) {
        let _ = window_id;
        tracing::debug!("GTK window focus not yet implemented");
    }

    /// Get the current animation progress [0.0, 1.0].
    pub fn animation_progress(&self) -> f64 {
        self.animation_progress
    }

    /// Enable or disable background blur.
    pub fn set_blur(&mut self, enabled: bool) {
        self.blur_active = enabled;
        tracing::debug!(blur = enabled, "Overview blur toggled");
    }
}

impl std::fmt::Debug for Overview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Overview")
            .field("state", &self.state)
            .field("workspaces", &self.workspaces)
            .field("active_workspace", &self.active_workspace)
            .field("windows", &self.windows)
            .field("search_query", &self.search_query)
            .field("blur_active", &self.blur_active)
            .field("animation_progress", &self.animation_progress)
            .finish_non_exhaustive()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_monitors() -> MultiMonitorManager {
        MultiMonitorManager::with_default_monitor()
    }

    fn test_accessibility() -> AccessibilityManager {
        AccessibilityManager::new()
    }

    fn test_animation() -> AnimationEngine {
        AnimationEngine::new()
    }

    fn create_overview() -> Overview {
        Overview::new(test_monitors(), test_accessibility(), test_animation())
    }

    fn sample_windows() -> Vec<WindowThumbnail> {
        vec![
            WindowThumbnail {
                id: "win-1".into(),
                title: "Terminal".into(),
                app_id: "org.gnome.Terminal".into(),
                workspace: 0,
                minimized: false,
            },
            WindowThumbnail {
                id: "win-2".into(),
                title: "Files".into(),
                app_id: "org.gnome.Nautilus".into(),
                workspace: 0,
                minimized: true,
            },
            WindowThumbnail {
                id: "win-3".into(),
                title: "Browser".into(),
                app_id: "org.mozilla.firefox".into(),
                workspace: 1,
                minimized: false,
            },
        ]
    }

    #[test]
    fn test_creation() {
        let ov = create_overview();
        assert_eq!(ov.state(), OverviewState::Closed);
        assert!(!ov.is_open());
        assert_eq!(ov.workspaces(), 4);
        assert_eq!(ov.active_workspace(), 0);
        assert!(ov.windows().is_empty());
        assert!(ov.search_query().is_empty());
        assert!((ov.animation_progress() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_state_transitions_open() {
        let mut ov = create_overview();
        ov.open();
        assert_eq!(ov.state(), OverviewState::Opening);
        assert!(!ov.is_open());
        assert!((ov.animation_progress() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_state_transitions_close() {
        let mut ov = create_overview();
        // Force state to Open
        ov.state = OverviewState::Open;
        ov.close();
        assert_eq!(ov.state(), OverviewState::Closing);
        assert!((ov.animation_progress() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_state_transitions_toggle() {
        let mut ov = create_overview();
        // Closed -> Opening
        ov.toggle();
        assert_eq!(ov.state(), OverviewState::Opening);

        // Opening -> Closing
        ov.toggle();
        assert_eq!(ov.state(), OverviewState::Closing);

        // Closing -> Opening
        ov.toggle();
        assert_eq!(ov.state(), OverviewState::Opening);

        // Force Open, then toggle to Closing
        ov.state = OverviewState::Open;
        ov.toggle();
        assert_eq!(ov.state(), OverviewState::Closing);
    }

    #[test]
    fn test_workspace_management() {
        let mut ov = create_overview();
        assert_eq!(ov.workspaces(), 4);

        ov.set_workspaces(6);
        assert_eq!(ov.workspaces(), 6);

        // Active workspace should be clamped
        ov.set_active_workspace(5);
        assert_eq!(ov.active_workspace(), 5);
        ov.set_workspaces(3);
        assert_eq!(ov.active_workspace(), 2);

        ov.set_workspaces(2);
        assert_eq!(ov.workspaces(), 2);
        assert_eq!(ov.active_workspace(), 1); // clamped to max index
    }

    #[test]
    fn test_active_workspace() {
        let mut ov = create_overview();
        assert_eq!(ov.active_workspace(), 0);

        ov.set_active_workspace(2);
        assert_eq!(ov.active_workspace(), 2);

        // Out of bounds should be ignored
        ov.set_active_workspace(10);
        assert_eq!(ov.active_workspace(), 2);
    }

    #[test]
    fn test_window_management() {
        let mut ov = create_overview();
        assert!(ov.windows().is_empty());

        let windows = sample_windows();
        ov.set_windows(windows.clone());
        assert_eq!(ov.windows().len(), 3);

        let first = &ov.windows()[0];
        assert_eq!(first.id, "win-1");
        assert_eq!(first.title, "Terminal");
        assert_eq!(first.app_id, "org.gnome.Terminal");
        assert_eq!(first.workspace, 0);
        assert!(!first.minimized);

        // Clear windows
        ov.set_windows(Vec::new());
        assert!(ov.windows().is_empty());
    }

    #[test]
    fn test_search_integration() {
        let mut ov = create_overview();
        assert!(ov.search_query().is_empty());

        ov.set_search("firefox");
        assert_eq!(ov.search_query(), "firefox");

        ov.set_search("");
        assert!(ov.search_query().is_empty());

        ov.set_search("terminal emulator");
        assert_eq!(ov.search_query(), "terminal emulator");
    }

    #[test]
    fn test_animation_progress() {
        let mut ov = create_overview();
        assert!((ov.animation_progress() - 0.0).abs() < 1e-6);

        // Simulate external progress update
        ov.animation_progress = 0.5;
        assert!((ov.animation_progress() - 0.5).abs() < 1e-6);

        ov.animation_progress = 1.0;
        assert!((ov.animation_progress() - 1.0).abs() < 1e-6);

        // Opening resets progress
        ov.open();
        assert!((ov.animation_progress() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_blur_toggle() {
        let mut ov = create_overview();
        assert!(!ov.blur_active); // private field, checked via behavior

        ov.set_blur(true);
        assert!(ov.blur_active);

        ov.set_blur(false);
        assert!(!ov.blur_active);
    }

    #[test]
    fn test_go_to_workspace() {
        let mut ov = create_overview();
        ov.state = OverviewState::Open;
        assert_eq!(ov.active_workspace(), 0);

        ov.go_to_workspace(2);
        assert_eq!(ov.active_workspace(), 2);
        assert_eq!(ov.state(), OverviewState::Closing);

        // Out of bounds should be ignored
        let mut ov = create_overview();
        ov.state = OverviewState::Open;
        ov.go_to_workspace(10);
        assert_eq!(ov.active_workspace(), 0);
        assert_eq!(ov.state(), OverviewState::Open);
    }

    #[test]
    fn test_debug_format() {
        let mut ov = create_overview();
        ov.set_search("test");
        ov.set_blur(true);

        let debug = format!("{ov:?}");
        assert!(debug.contains("Overview"));
        assert!(debug.contains("Closed"));
        assert!(debug.contains("test"));
        assert!(debug.contains("true"));
        assert!(debug.contains("animation_progress"));
    }

    #[test]
    fn test_is_open() {
        let mut ov = create_overview();
        assert!(!ov.is_open());

        ov.state = OverviewState::Opening;
        assert!(!ov.is_open());

        ov.state = OverviewState::Open;
        assert!(ov.is_open());

        ov.state = OverviewState::Closing;
        assert!(!ov.is_open());
    }

    #[test]
    fn test_open_idempotent() {
        let mut ov = create_overview();
        ov.open();
        assert_eq!(ov.state(), OverviewState::Opening);
        // Second open should be a no-op
        ov.open();
        assert_eq!(ov.state(), OverviewState::Opening);
    }

    #[test]
    fn test_close_idempotent() {
        let mut ov = create_overview();
        ov.state = OverviewState::Closing;
        ov.close();
        assert_eq!(ov.state(), OverviewState::Closing);
    }

    #[test]
    fn test_close_from_closed() {
        let mut ov = create_overview();
        assert_eq!(ov.state(), OverviewState::Closed);
        ov.close();
        // Should remain closed
        assert_eq!(ov.state(), OverviewState::Closed);
    }

    #[test]
    fn test_go_to_workspace_zero() {
        let mut ov = create_overview();
        ov.state = OverviewState::Open;
        ov.go_to_workspace(0);
        assert_eq!(ov.active_workspace(), 0);
        assert_eq!(ov.state(), OverviewState::Closing);
    }

    #[test]
    fn test_window_thumbnail_debug() {
        let w = WindowThumbnail {
            id: "w1".into(),
            title: "Test".into(),
            app_id: "app.test".into(),
            workspace: 1,
            minimized: true,
        };
        let debug = format!("{w:?}");
        assert!(debug.contains("WindowThumbnail"));
        assert!(debug.contains("w1"));
        assert!(debug.contains("Test"));
        assert!(debug.contains("true"));
    }

    #[test]
    fn test_window_thumbnail_clone() {
        let w = WindowThumbnail {
            id: "w1".into(),
            title: "Test".into(),
            app_id: "app.test".into(),
            workspace: 1,
            minimized: false,
        };
        let _ = w.clone();
    }

    #[test]
    fn test_focus_window() {
        let ov = create_overview();
        // Should not panic
        ov.focus_window("win-1");
        ov.focus_window("");
    }

    #[test]
    fn test_set_windows_empty() {
        let mut ov = create_overview();
        ov.set_windows(vec![]);
        assert!(ov.windows().is_empty());

        ov.set_windows(sample_windows());
        assert_eq!(ov.windows().len(), 3);

        ov.set_windows(vec![]);
        assert!(ov.windows().is_empty());
    }

    #[test]
    fn test_workspace_count_zero() {
        let mut ov = create_overview();
        ov.set_workspaces(0);
        assert_eq!(ov.workspaces(), 0);
        ov.set_active_workspace(0);
        // Should not panic when clamping
        assert_eq!(ov.active_workspace(), 0);
    }

    #[test]
    fn test_search_clears_on_open() {
        let mut ov = create_overview();
        ov.set_search("something");
        assert_eq!(ov.search_query(), "something");

        ov.open();
        // Search should persist after open
        assert_eq!(ov.search_query(), "something");
    }
}
