// SPDX-License-Identifier: GPL-3.0-or-later

//! # Calendar Popover
//!
//! Calendar widget shown when clicking the clock in the panel.
//! Supports month/year navigation and shows the current date.

use chrono::Datelike;
use crate::localization::LocalizationManager;

/// Calendar navigation state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalendarView {
    Month,
    Year,
}

/// Calendar popover data (no GTK widgets since it's embedded).
pub struct Calendar {
    view: CalendarView,
    display_month: u8,
    display_year: i32,
    selected_day: Option<u8>,
    selected_month: u8,
    selected_year: i32,
    localization: LocalizationManager,
}

impl Calendar {
    pub fn new(localization: LocalizationManager) -> Self {
        let now = chrono::Local::now();
        Self {
            view: CalendarView::Month,
            display_month: now.month() as u8,
            display_year: now.year(),
            selected_day: Some(now.day() as u8),
            selected_month: now.month() as u8,
            selected_year: now.year(),
            localization,
        }
    }

    pub fn set_display_month(&mut self, month: u8) {
        if month == 0 {
            self.display_month = 12;
            self.display_year -= 1;
        } else if month > 12 {
            self.display_month = 1;
            self.display_year += 1;
        } else {
            self.display_month = month;
        }
    }

    pub fn set_display_year(&mut self, year: i32) { self.display_year = year; }
    pub fn display_month(&self) -> u8 { self.display_month }
    pub fn display_year(&self) -> i32 { self.display_year }

    pub fn previous_month(&mut self) {
        if self.display_month == 1 {
            self.display_month = 12;
            self.display_year -= 1;
        } else {
            self.display_month -= 1;
        }
    }

    pub fn next_month(&mut self) {
        if self.display_month == 12 {
            self.display_month = 1;
            self.display_year += 1;
        } else {
            self.display_month += 1;
        }
    }

    pub fn previous_year(&mut self) { self.display_year -= 1; }
    pub fn next_year(&mut self) { self.display_year += 1; }

    pub fn select_day(&mut self, day: u8) {
        self.selected_day = Some(day);
        self.selected_month = self.display_month;
        self.selected_year = self.display_year;
    }

    pub fn selected_day(&self) -> Option<u8> { self.selected_day }
    pub fn selected_month(&self) -> u8 { self.selected_month }
    pub fn selected_year(&self) -> i32 { self.selected_year }
    pub fn set_view(&mut self, view: CalendarView) { self.view = view; }
    pub fn view(&self) -> CalendarView { self.view }
    pub fn toggle_view(&mut self) {
        self.view = match self.view {
            CalendarView::Month => CalendarView::Year,
            CalendarView::Year => CalendarView::Month,
        };
    }

    pub fn month_name(&self) -> &'static str {
        self.localization.month_name(self.display_month)
    }

    pub fn month_name_for(&self, month: u8) -> &'static str {
        self.localization.month_name(month)
    }

    pub fn day_name_for(&self, day: u8) -> &'static str {
        self.localization.day_name(day)
    }

    pub fn short_day_name_for(&self, day: u8) -> &'static str {
        self.localization.short_day_name(day)
    }

    pub fn first_day_of_week(&self) -> u8 {
        self.localization.first_day_of_week()
    }

    /// Get calendar data for the display month.
    /// Returns (start_day_of_week, days_in_month, selected_day_option)
    pub fn calendar_data(&self) -> (u8, u8, Option<u8>) {
        let first = chrono::NaiveDate::from_ymd_opt(self.display_year, self.display_month as u32, 1);
        let start_wday = first.map(|d| d.weekday().num_days_from_sunday() as u8).unwrap_or(0);

        let next_month = if self.display_month == 12 {
            chrono::NaiveDate::from_ymd_opt(self.display_year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(self.display_year, self.display_month as u32 + 1, 1)
        };

        let days = if let Some(next) = next_month {
            (next - chrono::Duration::days(1)).day() as u8
        } else {
            31
        };

        let selected = if self.selected_month == self.display_month && self.selected_year == self.display_year {
            self.selected_day
        } else {
            None
        };

        (start_wday, days, selected)
    }

    /// Check if a date is today.
    pub fn is_today(&self, day: u8) -> bool {
        let now = chrono::Local::now();
        day == now.day() as u8
            && self.display_month == now.month() as u8
            && self.display_year == now.year()
    }

    /// Get today's date.
    pub fn today() -> (u8, u8, i32) {
        let now = chrono::Local::now();
        (now.day() as u8, now.month() as u8, now.year())
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new(LocalizationManager::new())
    }
}

impl std::fmt::Debug for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Calendar")
            .field("display", &format_args!("{}-{:02}", self.display_year, self.display_month))
            .field("selected", &self.selected_day)
            .field("view", &self.view)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cal() -> Calendar {
        Calendar::new(LocalizationManager::new())
    }

    #[test]
    fn test_creation() {
        let cal = test_cal();
        assert_eq!(cal.view(), CalendarView::Month);
        assert!(cal.selected_day().is_some());
    }

    #[test]
    fn test_previous_month() {
        let mut cal = test_cal();
        cal.set_display_month(1);
        cal.set_display_year(2024);
        cal.previous_month();
        assert_eq!(cal.display_month(), 12);
        assert_eq!(cal.display_year(), 2023);
    }

    #[test]
    fn test_next_month() {
        let mut cal = test_cal();
        cal.set_display_month(12);
        cal.set_display_year(2024);
        cal.next_month();
        assert_eq!(cal.display_month(), 1);
        assert_eq!(cal.display_year(), 2025);
    }

    #[test]
    fn test_previous_year() {
        let mut cal = test_cal();
        cal.set_display_year(2024);
        cal.previous_year();
        assert_eq!(cal.display_year(), 2023);
    }

    #[test]
    fn test_next_year() {
        let mut cal = test_cal();
        cal.set_display_year(2024);
        cal.next_year();
        assert_eq!(cal.display_year(), 2025);
    }

    #[test]
    fn test_select_day() {
        let mut cal = test_cal();
        cal.set_display_month(3);
        cal.set_display_year(2024);
        cal.select_day(14);
        assert_eq!(cal.selected_day(), Some(14));
        assert_eq!(cal.selected_month(), 3);
        assert_eq!(cal.selected_year(), 2024);
    }

    #[test]
    fn test_calendar_data() {
        let mut cal = test_cal();
        cal.set_display_month(1);
        cal.set_display_year(2024);
        cal.select_day(15);
        let (start_wday, days, selected) = cal.calendar_data();
        assert_eq!(start_wday, 1); // Jan 2024 starts Monday
        assert_eq!(days, 31);
        assert!(selected.is_some());
    }

    #[test]
    fn test_toggle_view() {
        let mut cal = test_cal();
        assert_eq!(cal.view(), CalendarView::Month);
        cal.toggle_view();
        assert_eq!(cal.view(), CalendarView::Year);
        cal.toggle_view();
        assert_eq!(cal.view(), CalendarView::Month);
    }

    #[test]
    fn test_today() {
        let (day, month, year) = Calendar::today();
        let now = chrono::Local::now();
        assert_eq!(day, now.day() as u8);
        assert_eq!(month, now.month() as u8);
        assert_eq!(year, now.year());
    }

    #[test]
    fn test_is_today() {
        let mut cal = test_cal();
        let now = chrono::Local::now();
        cal.set_display_month(now.month() as u8);
        cal.set_display_year(now.year());
        assert!(cal.is_today(now.day() as u8));
        assert!(!cal.is_today(1)); // probably not day 1
    }

    #[test]
    fn test_month_name_for() {
        let cal = test_cal();
        assert_eq!(cal.month_name_for(1), "January");
        assert_eq!(cal.month_name_for(12), "December");
    }

    #[test]
    fn test_day_names() {
        let cal = test_cal();
        assert_eq!(cal.day_name_for(0), "Sunday");
        assert_eq!(cal.short_day_name_for(0), "Sun");
    }

    #[test]
    fn test_set_display_month_wrap_low() {
        let mut cal = test_cal();
        cal.set_display_year(2024);
        cal.set_display_month(0);
        assert_eq!(cal.display_month(), 12);
        assert_eq!(cal.display_year(), 2023);
    }

    #[test]
    fn test_debug_format() {
        let cal = test_cal();
        let debug = format!("{cal:?}");
        assert!(debug.contains("Calendar"));
    }
}
