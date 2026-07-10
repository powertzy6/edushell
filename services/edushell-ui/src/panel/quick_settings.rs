// SPDX-License-Identifier: GPL-3.0-or-later

//! # Panel Quick Settings Button
//!
//! Button in the panel that toggles the Quick Settings panel.

/// A toggle button in the panel that opens or closes the Quick Settings panel.
pub struct PanelQuickSettings {
    active: bool,
    #[cfg(feature = "gtk")]
    button: Option<gtk::ToggleButton>,
}

impl PanelQuickSettings {
    /// Create a new quick settings toggle button.
    pub fn new() -> Self {
        Self {
            active: false,
            #[cfg(feature = "gtk")]
            button: None,
        }
    }

    /// Build the GTK widget for this component.
    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        let toggle = gtk::ToggleButton::new();
        toggle.set_label("Quick Settings");
        toggle.add_css_class("qs-button");
        toggle.set_active(self.active);
        toggle.set_has_frame(false);
    }

    /// Build stub for non-GTK mode.
    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {
        tracing::info!(target: "edushell::ui::panel::quick_settings", "Quick settings button stub: active={}", self.active);
    }

    /// Set the active state of the button.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
        #[cfg(feature = "gtk")]
        if let Some(button) = &self.button {
            button.set_active(active);
        }
    }

    /// Check if the button is active (quick settings is open).
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Toggle the active state.
    pub fn toggle(&mut self) {
        self.active = !self.active;
        #[cfg(feature = "gtk")]
        if let Some(button) = &self.button {
            button.set_active(self.active);
        }
    }
}

impl Default for PanelQuickSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for PanelQuickSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PanelQuickSettings")
            .field("active", &self.active)
            .finish()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_qs() -> PanelQuickSettings {
        PanelQuickSettings::new()
    }

    #[test]
    fn test_new_is_inactive() {
        let qs = make_qs();
        assert!(!qs.is_active());
    }

    #[test]
    fn test_set_active() {
        let mut qs = make_qs();
        qs.set_active(true);
        assert!(qs.is_active());
        qs.set_active(false);
        assert!(!qs.is_active());
    }

    #[test]
    fn test_toggle() {
        let mut qs = make_qs();
        qs.toggle();
        assert!(qs.is_active());
        qs.toggle();
        assert!(!qs.is_active());
    }

    #[test]
    fn test_multiple_toggles() {
        let mut qs = make_qs();
        for _ in 0..10 {
            qs.toggle();
        }
        // Even number of toggles (10) from false -> false
        assert!(!qs.is_active());
    }

    #[test]
    fn test_odd_toggles() {
        let mut qs = make_qs();
        for _ in 0..7 {
            qs.toggle();
        }
        // 7 toggles from false -> true (odd)
        assert!(qs.is_active());
    }

    #[test]
    fn test_set_active_twice() {
        let mut qs = make_qs();
        qs.set_active(true);
        qs.set_active(true);
        assert!(qs.is_active());
    }

    #[test]
    fn test_build_does_not_panic() {
        let qs = make_qs();
        qs.build();
    }

    #[test]
    fn test_default_trait() {
        let qs = PanelQuickSettings::default();
        assert!(!qs.is_active());
    }

    #[test]
    fn test_debug_output() {
        let mut qs = make_qs();
        qs.set_active(true);
        let d = format!("{qs:?}");
        assert!(d.contains("PanelQuickSettings"));
        assert!(d.contains("active: true"));
    }

    #[test]
    fn test_toggle_from_active() {
        let mut qs = make_qs();
        qs.set_active(true);
        qs.toggle();
        assert!(!qs.is_active());
    }

    #[test]
    fn test_state_after_build() {
        let mut qs = make_qs();
        qs.set_active(true);
        qs.build();
        assert!(qs.is_active());
    }
}
