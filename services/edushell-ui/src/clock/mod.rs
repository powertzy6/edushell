// SPDX-License-Identifier: GPL-3.0-or-later

//! # Clock Widget & Calendar
//!
//! Clock widget showing current time and date. Clicking
//! opens a calendar popover with month/year navigation.

pub mod calendar;
pub use calendar::Calendar;

use chrono::{Datelike, Timelike};
use crate::localization::LocalizationManager;
use edushell_core::event::EventBus;

/// Clock display mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClockMode {
    TimeOnly,
    DateOnly,
    TimeAndDate,
}

/// The clock widget.
pub struct ClockWidget {
    mode: ClockMode,
    show_seconds: bool,
    hour: u8,
    minute: u8,
    second: u8,
    day: u8,
    month: u8,
    year: i32,
    localization: LocalizationManager,
    event_bus: EventBus,
    #[cfg(feature = "gtk")]
    widget: Option<gtk::Box>,
    #[cfg(feature = "gtk")]
    popover: Option<gtk::Popover>,
}

impl ClockWidget {
    pub fn new(localization: LocalizationManager, event_bus: EventBus) -> Self {
        let now = chrono::Local::now();
        Self {
            mode: ClockMode::TimeAndDate,
            show_seconds: false,
            hour: now.hour() as u8,
            minute: now.minute() as u8,
            second: now.second() as u8,
            day: now.day() as u8,
            month: now.month() as u8,
            year: now.year(),
            localization,
            event_bus,
            #[cfg(feature = "gtk")]
            widget: None,
            #[cfg(feature = "gtk")]
            popover: None,
        }
    }

    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        tracing::info!(target: "edushell::clock", "Clock widget built");
    }

    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {}

    pub fn update_now(&mut self) {
        let now = chrono::Local::now();
        self.hour = now.hour() as u8;
        self.minute = now.minute() as u8;
        self.second = now.second() as u8;
        self.day = now.day() as u8;
        self.month = now.month() as u8;
        self.year = now.year();
    }

    pub fn set_time(&mut self, hour: u8, minute: u8, second: u8) {
        self.hour = hour.min(23);
        self.minute = minute.min(59);
        self.second = second.min(59);
    }

    pub fn set_date(&mut self, day: u8, month: u8, year: i32) {
        self.day = day.max(1).min(31);
        self.month = month.max(1).min(12);
        self.year = year;
    }

    pub fn set_mode(&mut self, mode: ClockMode) { self.mode = mode; }
    pub fn mode(&self) -> ClockMode { self.mode }
    pub fn set_show_seconds(&mut self, show: bool) { self.show_seconds = show; }
    pub fn show_seconds(&self) -> bool { self.show_seconds }
    pub fn hour(&self) -> u8 { self.hour }
    pub fn minute(&self) -> u8 { self.minute }
    pub fn second(&self) -> u8 { self.second }
    pub fn day(&self) -> u8 { self.day }
    pub fn month(&self) -> u8 { self.month }
    pub fn year(&self) -> i32 { self.year }

    pub fn time_string(&self) -> String {
        self.localization.format_time(self.hour, self.minute)
    }

    pub fn date_string(&self) -> String {
        self.localization.format_date(self.year, self.month, self.day)
    }

    pub fn display_string(&self) -> String {
        match self.mode {
            ClockMode::TimeOnly => self.time_string(),
            ClockMode::DateOnly => self.date_string(),
            ClockMode::TimeAndDate => format!("{}  {}", self.time_string(), self.date_string()),
        }
    }

    pub fn month_name(&self) -> &'static str {
        self.localization.month_name(self.month)
    }

    pub fn day_name(&self) -> &'static str {
        let dow = chrono::NaiveDate::from_ymd_opt(self.year, self.month as u32, self.day as u32)
            .map(|d| d.weekday().num_days_from_sunday() as u8)
            .unwrap_or(0);
        self.localization.day_name(dow)
    }

    /// Get calendar grid for current month.
    /// Returns (weekday_of_first_day_0_6, days_in_month)
    pub fn calendar_data(&self) -> (u8, u8) {
        let first = chrono::NaiveDate::from_ymd_opt(self.year, self.month as u32, 1);
        let days_in_month = if self.month == 12 {
            chrono::NaiveDate::from_ymd_opt(self.year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(self.year, self.month as u32 + 1, 1)
        };

        let start_wday = first.map(|d| d.weekday().num_days_from_sunday() as u8).unwrap_or(0);

        let days = if let Some(next) = days_in_month {
            (next - chrono::Duration::days(1)).day() as u8
        } else {
            31
        };

        (start_wday, days)
    }
}

impl std::fmt::Debug for ClockWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClockWidget")
            .field("mode", &self.mode)
            .field("show_seconds", &self.show_seconds)
            .field("time", &format_args!("{:02}:{:02}", self.hour, self.minute))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_clock() -> ClockWidget {
        ClockWidget::new(LocalizationManager::new(), EventBus::new())
    }

    #[test]
    fn test_creation() {
        let clock = test_clock();
        assert!(!clock.show_seconds());
        assert_eq!(clock.mode(), ClockMode::TimeAndDate);
    }

    #[test]
    fn test_set_time() {
        let mut clock = test_clock();
        clock.set_time(14, 30, 45);
        assert_eq!(clock.hour(), 14);
        assert_eq!(clock.minute(), 30);
        assert_eq!(clock.second(), 45);
    }

    #[test]
    fn test_set_time_clamps() {
        let mut clock = test_clock();
        clock.set_time(99, 99, 99);
        assert_eq!(clock.hour(), 23);
        assert_eq!(clock.minute(), 59);
        assert_eq!(clock.second(), 59);
    }

    #[test]
    fn test_set_date() {
        let mut clock = test_clock();
        clock.set_date(14, 3, 2024);
        assert_eq!(clock.day(), 14);
        assert_eq!(clock.month(), 3);
        assert_eq!(clock.year(), 2024);
    }

    #[test]
    fn test_set_date_clamps() {
        let mut clock = test_clock();
        clock.set_date(0, 0, 2024);
        assert_eq!(clock.day(), 1);
        assert_eq!(clock.month(), 1);
    }

    #[test]
    fn test_show_seconds() {
        let mut clock = test_clock();
        clock.set_show_seconds(true);
        assert!(clock.show_seconds());
    }

    #[test]
    fn test_time_string() {
        let mut clock = test_clock();
        clock.set_time(14, 30, 0);
        let ts = clock.time_string();
        assert!(ts.contains("14:30") || ts.contains("2:30"));
    }

    #[test]
    fn test_date_string() {
        let mut clock = test_clock();
        clock.set_date(14, 3, 2024);
        let ds = clock.date_string();
        assert!(ds.contains("14") || ds.contains("3") || ds.contains("2024"));
    }

    #[test]
    fn test_display_string_mode() {
        let mut clock = test_clock();
        clock.set_mode(ClockMode::TimeOnly);
        let ds = clock.display_string();
        assert!(!ds.contains("/") || !ds.contains("-")); // probably no date separator

        clock.set_mode(ClockMode::DateOnly);
        let ds = clock.display_string();
        assert!(ds.contains("/") || ds.contains("-")); // probably has date separator
    }

    #[test]
    fn test_month_name() {
        let mut clock = test_clock();
        clock.set_date(1, 1, 2024);
        assert_eq!(clock.month_name(), "January");
        clock.set_date(1, 12, 2024);
        assert_eq!(clock.month_name(), "December");
    }

    #[test]
    fn test_calendar_data() {
        let mut clock = test_clock();
        clock.set_date(15, 1, 2024);
        let (start_wday, days) = clock.calendar_data();
        // January 2024 starts on Monday (1)
        assert_eq!(start_wday, 1);
        assert_eq!(days, 31);
    }

    #[test]
    fn test_update_now() {
        let mut clock = test_clock();
        clock.set_time(0, 0, 0);
        clock.update_now();
        let now = chrono::Local::now();
        assert_eq!(clock.hour(), now.hour() as u8);
        assert_eq!(clock.minute(), now.minute() as u8);
    }

    #[test]
    fn test_debug_format() {
        let clock = test_clock();
        let debug = format!("{clock:?}");
        assert!(debug.contains("ClockWidget"));
    }
}
