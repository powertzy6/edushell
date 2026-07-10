// SPDX-License-Identifier: GPL-3.0-or-later

//! # Panel Clock Widget
//!
//! Displays the current time and date in the panel.
//! Clicking opens the calendar popover.

/// A panel widget showing the current time and optionally the date.
pub struct PanelClock {
    show_date: bool,
    show_seconds: bool,
    #[cfg(feature = "gtk")]
    widget: Option<gtk::Box>,
}

impl PanelClock {
    /// Create a new panel clock widget.
    pub fn new() -> Self {
        Self {
            show_date: false,
            show_seconds: false,
            #[cfg(feature = "gtk")]
            widget: None,
        }
    }

    /// Build the GTK widget for this component.
    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        let box_ = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        box_.add_css_class("panel-clock");
        let label = gtk::Label::new(Some("00:00"));
        label.add_css_class("clock-label");
        box_.append(&label);
    }

    /// Build stub for non-GTK mode.
    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {
        tracing::info!(target: "edushell::ui::panel::clock", "Panel clock stub: show_date={}, show_seconds={}",
            self.show_date, self.show_seconds);
    }

    /// Set whether to show the date alongside the time.
    pub fn set_show_date(&mut self, show: bool) {
        self.show_date = show;
    }

    /// Set whether to show seconds.
    pub fn set_show_seconds(&mut self, show: bool) {
        self.show_seconds = show;
    }

    /// Update the displayed time.
    pub fn update_time(&self, hour: u8, minute: u8, second: u8) {
        let _ = second;
        tracing::debug!(target: "edushell::ui::panel::clock", "Time updated: {:02}:{:02}:{:02}", hour, minute, second);
    }

    /// Update the displayed date.
    pub fn update_date(&self, day: u8, month: u8, year: i32) {
        let _ = (day, month, year);
        tracing::debug!(target: "edushell::ui::panel::clock", "Date updated: {}-{:02}-{:02}", year, month, day);
    }

    /// Check if date is shown.
    pub fn shows_date(&self) -> bool {
        self.show_date
    }

    /// Check if seconds are shown.
    pub fn shows_seconds(&self) -> bool {
        self.show_seconds
    }
}

impl Default for PanelClock {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for PanelClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PanelClock")
            .field("show_date", &self.show_date)
            .field("show_seconds", &self.show_seconds)
            .finish()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_clock() -> PanelClock {
        PanelClock::new()
    }

    #[test]
    fn test_defaults() {
        let clock = make_clock();
        assert!(!clock.shows_date());
        assert!(!clock.shows_seconds());
    }

    #[test]
    fn test_show_date() {
        let mut clock = make_clock();
        clock.set_show_date(true);
        assert!(clock.shows_date());
        clock.set_show_date(false);
        assert!(!clock.shows_date());
    }

    #[test]
    fn test_show_seconds() {
        let mut clock = make_clock();
        clock.set_show_seconds(true);
        assert!(clock.shows_seconds());
        clock.set_show_seconds(false);
        assert!(!clock.shows_seconds());
    }

    #[test]
    fn test_update_time_does_not_panic() {
        let clock = make_clock();
        clock.update_time(14, 30, 45);
    }

    #[test]
    fn test_update_date_does_not_panic() {
        let clock = make_clock();
        clock.update_date(15, 3, 2024);
    }

    #[test]
    fn test_build_does_not_panic() {
        let clock = make_clock();
        clock.build();
    }

    #[test]
    fn test_show_both_date_and_seconds() {
        let mut clock = make_clock();
        clock.set_show_date(true);
        clock.set_show_seconds(true);
        assert!(clock.shows_date());
        assert!(clock.shows_seconds());
    }

    #[test]
    fn test_default_trait() {
        let clock = PanelClock::default();
        assert!(!clock.shows_date());
        assert!(!clock.shows_seconds());
    }

    #[test]
    fn test_debug_output() {
        let mut clock = make_clock();
        clock.set_show_date(true);
        let d = format!("{clock:?}");
        assert!(d.contains("PanelClock"));
        assert!(d.contains("show_date: true"));
    }

    #[test]
    fn test_update_time_edge_cases() {
        let clock = make_clock();
        clock.update_time(0, 0, 0);
        clock.update_time(23, 59, 59);
    }

    #[test]
    fn test_update_date_edge_cases() {
        let clock = make_clock();
        clock.update_date(1, 1, 1970);
        clock.update_date(31, 12, 2099);
    }
}
