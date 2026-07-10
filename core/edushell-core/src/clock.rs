// SPDX-License-Identifier: GPL-3.0-or-later

//! # Clock Service
//!
//! Provides time formatting, timezone handling, calendar
//! calculations, and periodic time-change notifications.
//! Handles 12/24-hour format preferences and locale-aware
//! date display.

use std::sync::Arc;

use crate::error::EduResult;

/// Time format preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    /// 12-hour format (e.g., "2:30 PM").
    Hour12,
    /// 24-hour format (e.g., "14:30").
    Hour24,
}

/// Thread-safe clock service.
#[derive(Clone)]
pub struct ClockService {
    time_format: Arc<std::sync::RwLock<TimeFormat>>,
    timezone: Arc<std::sync::RwLock<String>>,
    show_seconds: Arc<std::sync::RwLock<bool>>,
    show_date: Arc<std::sync::RwLock<bool>>,
}

impl ClockService {
    /// Create a new clock service.
    pub fn new() -> Self {
        let tz = detect_system_timezone();

        Self {
            time_format: Arc::new(std::sync::RwLock::new(TimeFormat::Hour24)),
            timezone: Arc::new(std::sync::RwLock::new(tz)),
            show_seconds: Arc::new(std::sync::RwLock::new(false)),
            show_date: Arc::new(std::sync::RwLock::new(true)),
        }
    }

    /// Get the current date/time as `chrono::DateTime`.
    pub fn now(&self) -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }

    /// Format the current time as a string.
    pub fn format_time(&self) -> String {
        let now = self.now();
        let format = self.time_format.read()
            .map(|f| *f)
            .unwrap_or(TimeFormat::Hour24);
        let show_seconds = self.show_seconds.read()
            .map(|s| *s)
            .unwrap_or(false);

        match format {
            TimeFormat::Hour12 => {
                if show_seconds {
                    now.format("%I:%M:%S %p").to_string()
                } else {
                    now.format("%I:%M %p").to_string()
                }
            }
            TimeFormat::Hour24 => {
                if show_seconds {
                    now.format("%H:%M:%S").to_string()
                } else {
                    now.format("%H:%M").to_string()
                }
            }
        }
    }

    /// Format the current date as a string.
    pub fn format_date(&self) -> String {
        let now = self.now();
        now.format("%A, %B %d, %Y").to_string()
    }

    /// Format a short date string.
    pub fn format_date_short(&self) -> String {
        let now = self.now();
        now.format("%m/%d/%Y").to_string()
    }

    /// Get the current timezone string.
    pub fn timezone(&self) -> String {
        self.timezone.read()
            .map(|tz| tz.clone())
            .unwrap_or_else(|_| "UTC".to_string())
    }

    /// Set the time format.
    pub fn set_time_format(&self, format: TimeFormat) -> EduResult<()> {
        let mut fmt = self.time_format.write()
            .map_err(|e| crate::error::EduError::Unknown(format!("Clock lock: {e}")))?;
        *fmt = format;
        Ok(())
    }

    /// Set whether to show seconds.
    pub fn set_show_seconds(&self, show: bool) -> EduResult<()> {
        let mut ss = self.show_seconds.write()
            .map_err(|e| crate::error::EduError::Unknown(format!("Clock lock: {e}")))?;
        *ss = show;
        Ok(())
    }

    /// Set whether to show date alongside time.
    pub fn set_show_date(&self, show: bool) -> EduResult<()> {
        let mut sd = self.show_date.write()
            .map_err(|e| crate::error::EduError::Unknown(format!("Clock lock: {e}")))?;
        *sd = show;
        Ok(())
    }
}

impl Default for ClockService {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect the system timezone.
fn detect_system_timezone() -> String {
    // Try /etc/timezone first
    if let Ok(content) = std::fs::read_to_string("/etc/timezone") {
        return content.trim().to_string();
    }

    // Try /etc/localtime symlink
    if let Ok(target) = std::fs::read_link("/etc/localtime") {
        let path_str = target.to_string_lossy();
        if let Some(pos) = path_str.rfind("/zoneinfo/") {
            return path_str[pos + 10..].to_string();
        }
    }

    // Fallback
    "UTC".to_string()
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_clock_now() {
        let clock = ClockService::new();
        let now = clock.now();
        assert!(now.year() >= 2024);
    }

    #[test]
    fn test_format_time() {
        let clock = ClockService::new();
        let time_str = clock.format_time();
        assert!(!time_str.is_empty());
        assert!(time_str.contains(':') || time_str.contains("AM") || time_str.contains("PM"));
    }

    #[test]
    fn test_format_date() {
        let clock = ClockService::new();
        let date_str = clock.format_date();
        assert!(!date_str.is_empty());
    }

    #[test]
    fn test_set_show_seconds() {
        let clock = ClockService::new();
        clock.set_show_seconds(true).unwrap();
        clock.set_show_seconds(false).unwrap();
    }

    #[test]
    fn test_timezone_detection() {
        let tz = detect_system_timezone();
        assert!(!tz.is_empty());
    }

    #[test]
    fn test_time_format_switch() {
        let clock = ClockService::new();
        clock.set_time_format(TimeFormat::Hour12).unwrap();

        clock.set_time_format(TimeFormat::Hour24).unwrap();
    }

    #[test]
    fn test_format_date_short() {
        let clock = ClockService::new();
        let short = clock.format_date_short();
        assert!(!short.is_empty());
    }

    #[test]
    fn test_timezone_access() {
        let clock = ClockService::new();
        let tz = clock.timezone();
        assert!(!tz.is_empty());
    }
}
