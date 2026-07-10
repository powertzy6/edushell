// SPDX-License-Identifier: GPL-3.0-or-later

//! # Workspace Indicator
//!
//! Shows workspace dots in the panel. Click to switch workspaces.
//! Shows which workspace is active.

/// An indicator showing workspace dots in the panel.
pub struct WorkspaceIndicator {
    total: u32,
    active: u32,
    #[cfg(feature = "gtk")]
    widget: Option<gtk::Box>,
}

impl WorkspaceIndicator {
    /// Create a new workspace indicator with default settings.
    pub fn new() -> Self {
        Self {
            total: 1,
            active: 0,
            #[cfg(feature = "gtk")]
            widget: None,
        }
    }

    /// Build the GTK widget for this component.
    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        let box_ = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        box_.add_css_class("workspace-indicator");
        for i in 0..self.total {
            let dot = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            dot.add_css_class("workspace-dot");
            if i == self.active {
                dot.add_css_class("active");
            }
            dot.set_size_request(8, 8);
            box_.append(&dot);
        }
    }

    /// Build stub for non-GTK mode.
    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {
        tracing::info!(target: "edushell::ui::panel::workspace", "Workspace indicator stub: {}/{}", self.active + 1, self.total);
    }

    /// Set the active workspace index.
    pub fn set_active(&mut self, index: u32) {
        if index < self.total {
            self.active = index;
        }
    }

    /// Set the total number of workspaces.
    pub fn set_total(&mut self, count: u32) {
        self.total = count.max(1);
        if self.active >= self.total {
            self.active = self.total - 1;
        }
    }

    /// Get the active workspace index.
    pub fn active(&self) -> u32 {
        self.active
    }

    /// Get the total number of workspaces.
    pub fn total(&self) -> u32 {
        self.total
    }
}

impl Default for WorkspaceIndicator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WorkspaceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkspaceIndicator")
            .field("active", &self.active)
            .field("total", &self.total)
            .finish()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_indicator() -> WorkspaceIndicator {
        WorkspaceIndicator::new()
    }

    #[test]
    fn test_defaults() {
        let ind = make_indicator();
        assert_eq!(ind.total(), 1);
        assert_eq!(ind.active(), 0);
    }

    #[test]
    fn test_set_active_valid() {
        let mut ind = make_indicator();
        ind.set_total(4);
        ind.set_active(2);
        assert_eq!(ind.active(), 2);
    }

    #[test]
    fn test_set_active_out_of_bounds() {
        let mut ind = make_indicator();
        ind.set_total(3);
        ind.set_active(5);
        assert_eq!(ind.active(), 0); // unchanged
    }

    #[test]
    fn test_set_total_increases() {
        let mut ind = make_indicator();
        ind.set_total(5);
        assert_eq!(ind.total(), 5);
    }

    #[test]
    fn test_set_total_minimum() {
        let mut ind = make_indicator();
        ind.set_total(0);
        assert_eq!(ind.total(), 1);
    }

    #[test]
    fn test_active_clamped_when_total_reduced() {
        let mut ind = make_indicator();
        ind.set_total(10);
        ind.set_active(7);
        ind.set_total(3);
        assert_eq!(ind.active(), 2);
        assert_eq!(ind.total(), 3);
    }

    #[test]
    fn test_active_stays_when_total_increased() {
        let mut ind = make_indicator();
        ind.set_total(3);
        ind.set_active(2);
        ind.set_total(5);
        assert_eq!(ind.active(), 2);
    }

    #[test]
    fn test_multiple_workspaces() {
        let mut ind = make_indicator();
        ind.set_total(8);
        assert_eq!(ind.total(), 8);
        for i in 0..8 {
            ind.set_active(i);
            assert_eq!(ind.active(), i);
        }
    }

    #[test]
    fn test_default_trait() {
        let ind = WorkspaceIndicator::default();
        assert_eq!(ind.total(), 1);
        assert_eq!(ind.active(), 0);
    }

    #[test]
    fn test_build_does_not_panic() {
        let ind = make_indicator();
        ind.build();
    }

    #[test]
    fn test_set_active_first() {
        let mut ind = make_indicator();
        ind.set_total(3);
        ind.set_active(0);
        assert_eq!(ind.active(), 0);
    }

    #[test]
    fn test_set_active_last() {
        let mut ind = make_indicator();
        ind.set_total(3);
        ind.set_active(2);
        assert_eq!(ind.active(), 2);
    }

    #[test]
    fn test_debug_output() {
        let mut ind = make_indicator();
        ind.set_total(4);
        ind.set_active(1);
        let d = format!("{ind:?}");
        assert!(d.contains("active: 1"));
        assert!(d.contains("total: 4"));
    }
}
