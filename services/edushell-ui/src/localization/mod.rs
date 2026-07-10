// SPDX-License-Identifier: GPL-3.0-or-later

//! # Localization (i18n)
//!
//! Internationalization support for the desktop shell UI.
//! Provides string translation, locale detection, and
//! formatting for dates, times, numbers, and currencies.

use std::collections::HashMap;

/// Manager for all localization features.
#[derive(Clone)]
pub struct LocalizationManager {
    /// Current locale (e.g. "en-US", "id-ID").
    locale: String,
    /// Available locales.
    available: Vec<String>,
    /// Translation map: (locale, key) -> translated string.
    translations: HashMap<String, HashMap<String, String>>,
    /// Time format: 12h or 24h.
    time_format: TimeFormat,
    /// Date format style.
    date_format: DateFormat,
    /// First day of week (0=Sunday, 1=Monday).
    first_day_of_week: u8,
    /// Measurement unit system.
    measurement: MeasurementSystem,
}

/// Time display format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeFormat {
    /// 12-hour clock (e.g. 2:30 PM).
    Hour12,
    /// 24-hour clock (e.g. 14:30).
    Hour24,
}

/// Date display format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateFormat {
    /// Month/Day/Year (US).
    MDY,
    /// Day/Month/Year (EU/Indonesia).
    DMY,
    /// Year/Month/Day (ISO).
    YMD,
}

/// Measurement system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementSystem {
    /// Metric (meters, Celsius).
    Metric,
    /// Imperial (feet, Fahrenheit).
    Imperial,
}

impl LocalizationManager {
    /// Create a new localization manager.
    pub fn new() -> Self {
        let mut mgr = Self {
            locale: "en-US".to_string(),
            available: vec![
                "en-US".to_string(),
                "id-ID".to_string(),
                "es-ES".to_string(),
                "fr-FR".to_string(),
                "de-DE".to_string(),
                "ja-JP".to_string(),
                "zh-CN".to_string(),
                "ar-SA".to_string(),
            ],
            translations: HashMap::new(),
            time_format: TimeFormat::Hour24,
            date_format: DateFormat::DMY,
            first_day_of_week: 1, // Monday
            measurement: MeasurementSystem::Metric,
        };
        mgr.load_default_translations();
        mgr
    }

    /// Detect system locale.
    pub fn detect_system_locale() -> String {
        std::env::var("LANG")
            .or_else(|_| std::env::var("LC_ALL"))
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .unwrap_or_else(|_| "en-US".to_string())
            .split('.')
            .next()
            .unwrap_or("en-US")
            .to_string()
    }

    /// Set locale.
    pub fn set_locale(&mut self, locale: &str) {
        if self.available.contains(&locale.to_string()) || locale.len() == 5 {
            self.locale = locale.to_string();
            // Update format preferences based on locale
            self.update_formats_for_locale();
        }
    }

    /// Get current locale.
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Get available locales.
    pub fn available_locales(&self) -> &[String] {
        &self.available
    }

    /// Translate a key.
    pub fn translate(&self, key: &str) -> String {
        self.translations
            .get(&self.locale)
            .and_then(|t| t.get(key))
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to English or key itself
                self.translations
                    .get("en-US")
                    .and_then(|t| t.get(key))
                    .cloned()
                    .unwrap_or_else(|| key.to_string())
            })
    }

    /// Format a time string (HH:MM).
    pub fn format_time(&self, hour: u8, minute: u8) -> String {
        match self.time_format {
            TimeFormat::Hour12 => {
                let period = if hour < 12 { "AM" } else { "PM" };
                let h12 = if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour };
                format!("{}:{:02} {}", h12, minute, period)
            }
            TimeFormat::Hour24 => {
                format!("{:02}:{:02}", hour, minute)
            }
        }
    }

    /// Get time format.
    pub fn time_format(&self) -> TimeFormat {
        self.time_format
    }

    /// Set time format.
    pub fn set_time_format(&mut self, format: TimeFormat) {
        self.time_format = format;
    }

    /// Get date format.
    pub fn date_format(&self) -> DateFormat {
        self.date_format
    }

    /// Set date format.
    pub fn set_date_format(&mut self, format: DateFormat) {
        self.date_format = format;
    }

    /// Format a date string.
    pub fn format_date(&self, year: i32, month: u8, day: u8) -> String {
        match self.date_format {
            DateFormat::MDY => format!("{}/{}/{}", month, day, year),
            DateFormat::DMY => format!("{}/{}/{}", day, month, year),
            DateFormat::YMD => format!("{}-{:02}-{:02}", year, month, day),
        }
    }

    /// Get first day of week.
    pub fn first_day_of_week(&self) -> u8 {
        self.first_day_of_week
    }

    /// Get measurement system.
    pub fn measurement(&self) -> MeasurementSystem {
        self.measurement
    }

    /// Get month name.
    pub fn month_name(&self, month: u8) -> &'static str {
        match month {
            1 => "January", 2 => "February", 3 => "March",
            4 => "April", 5 => "May", 6 => "June",
            7 => "July", 8 => "August", 9 => "September",
            10 => "October", 11 => "November", 12 => "December",
            _ => "Unknown",
        }
    }

    /// Get short month name.
    pub fn short_month_name(&self, month: u8) -> &'static str {
        &self.month_name(month)[..3]
    }

    /// Get day name.
    pub fn day_name(&self, day: u8) -> &'static str {
        match day {
            0 => "Sunday", 1 => "Monday", 2 => "Tuesday",
            3 => "Wednesday", 4 => "Thursday", 5 => "Friday",
            6 => "Saturday", _ => "Unknown",
        }
    }

    /// Get short day name.
    pub fn short_day_name(&self, day: u8) -> &'static str {
        match day {
            0 => "Sun", 1 => "Mon", 2 => "Tue",
            3 => "Wed", 4 => "Thu", 5 => "Fri",
            6 => "Sat", _ => "Unk",
        }
    }

    fn update_formats_for_locale(&mut self) {
        match &self.locale[..2] {
            "en" => {
                self.time_format = TimeFormat::Hour12;
                self.date_format = DateFormat::MDY;
                self.first_day_of_week = 0; // Sunday
                self.measurement = MeasurementSystem::Imperial;
            }
            "id" | "es" | "fr" | "de" => {
                self.time_format = TimeFormat::Hour24;
                self.date_format = DateFormat::DMY;
                self.first_day_of_week = 1; // Monday
                self.measurement = MeasurementSystem::Metric;
            }
            "ja" | "zh" => {
                self.time_format = TimeFormat::Hour24;
                self.date_format = DateFormat::YMD;
                self.first_day_of_week = 1;
                self.measurement = MeasurementSystem::Metric;
            }
            "ar" => {
                self.time_format = TimeFormat::Hour24;
                self.date_format = DateFormat::DMY;
                self.first_day_of_week = 6; // Saturday
                self.measurement = MeasurementSystem::Metric;
            }
            _ => {
                self.time_format = TimeFormat::Hour24;
                self.date_format = DateFormat::DMY;
                self.first_day_of_week = 1;
                self.measurement = MeasurementSystem::Metric;
            }
        }
    }

    fn load_default_translations(&mut self) {
        let mut en = HashMap::new();
        en.insert("app_menu", "Applications");
        en.insert("places", "Places");
        en.insert("recent", "Recent");
        en.insert("search", "Type to search...");
        en.insert("settings", "Settings");
        en.insert("lock", "Lock Screen");
        en.insert("logout", "Log Out");
        en.insert("suspend", "Suspend");
        en.insert("hibernate", "Hibernate");
        en.insert("restart", "Restart");
        en.insert("shutdown", "Shut Down");
        en.insert("switch_user", "Switch User");
        en.insert("confirm_shutdown", "Are you sure you want to shut down?");
        en.insert("confirm_restart", "Are you sure you want to restart?");
        en.insert("cancel", "Cancel");
        en.insert("confirm", "Confirm");
        en.insert("yes", "Yes");
        en.insert("no", "No");
        en.insert("empty_trash", "Empty Trash");
        en.insert("new_folder", "New Folder");
        en.insert("open_terminal", "Open Terminal");
        en.insert("change_wallpaper", "Change Wallpaper");
        en.insert("display_settings", "Display Settings");
        en.insert("wifi", "WiFi");
        en.insert("bluetooth", "Bluetooth");
        en.insert("volume", "Volume");
        en.insert("brightness", "Brightness");
        en.insert("dark_mode", "Dark Mode");
        en.insert("night_light", "Night Light");
        en.insert("airplane_mode", "Airplane Mode");
        en.insert("microphone", "Microphone");
        en.insert("screenshot", "Screenshot");
        en.insert("screen_recording", "Screen Recording");
        en.insert("battery", "Battery");
        en.insert("power_profile", "Power Profile");
        en.insert("performance", "Performance");
        en.insert("balanced", "Balanced");
        en.insert("power_saver", "Power Saver");
        en.insert("notifications", "Notifications");
        en.insert("clear_all", "Clear All");
        en.insert("no_notifications", "No notifications");
        en.insert("workspace", "Workspace");
        en.insert("workspaces", "Workspaces");
        en.insert("add_workspace", "Add Workspace");
        en.insert("remove_workspace", "Remove Workspace");
        en.insert("overview", "Overview");
        en.insert("window_preview", "Window Preview");
        en.insert("close", "Close");
        en.insert("minimize", "Minimize");
        en.insert("maximize", "Maximize");
        en.insert("restore", "Restore");
        en.insert("unpin", "Unpin");
        en.insert("pin", "Pin");
        en.insert("remove", "Remove");
        en.insert("show_desktop", "Show Desktop");
        en.insert("all_apps", "All Applications");
        en.insert("frequently_used", "Frequently Used");
        en.insert("recent_apps", "Recent Apps");
        en.insert("search_results", "Search Results");
        en.insert("no_results", "No results found");
        en.insert("type_to_search", "Type to search...");
        en.insert("learning", "Learning");
        en.insert("powered_by", "Powered by EduShell");
        self.translations.insert(
            "en-US".to_string(),
            en.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        );

        let mut id = HashMap::new();
        id.insert("app_menu", "Aplikasi");
        id.insert("places", "Tempat");
        id.insert("recent", "Terbaru");
        id.insert("search", "Ketik untuk mencari...");
        id.insert("settings", "Pengaturan");
        id.insert("lock", "Kunci Layar");
        id.insert("logout", "Keluar");
        id.insert("suspend", "Tidur");
        id.insert("hibernate", "Hibernasi");
        id.insert("restart", "Mulai Ulang");
        id.insert("shutdown", "Matikan");
        id.insert("switch_user", "Ganti Pengguna");
        id.insert("confirm_shutdown", "Anda yakin ingin mematikan komputer?");
        id.insert("confirm_restart", "Anda yakin ingin memulai ulang?");
        id.insert("cancel", "Batal");
        id.insert("confirm", "Konfirmasi");
        id.insert("yes", "Ya");
        id.insert("no", "Tidak");
        id.insert("empty_trash", "Kosongkan Sampah");
        id.insert("new_folder", "Folder Baru");
        id.insert("open_terminal", "Buka Terminal");
        id.insert("change_wallpaper", "Ganti Wallpaper");
        id.insert("display_settings", "Pengaturan Tampilan");
        id.insert("wifi", "WiFi");
        id.insert("bluetooth", "Bluetooth");
        id.insert("volume", "Volume");
        id.insert("brightness", "Kecerahan");
        id.insert("dark_mode", "Mode Gelap");
        id.insert("night_light", "Lampu Malam");
        id.insert("airplane_mode", "Mode Pesawat");
        id.insert("microphone", "Mikrofon");
        id.insert("screenshot", "Tangkapan Layar");
        id.insert("screen_recording", "Rekam Layar");
        id.insert("battery", "Baterai");
        id.insert("power_profile", "Profil Daya");
        id.insert("performance", "Performa");
        id.insert("balanced", "Seimbang");
        id.insert("power_saver", "Hemat Daya");
        id.insert("notifications", "Notifikasi");
        id.insert("clear_all", "Hapus Semua");
        id.insert("no_notifications", "Tidak ada notifikasi");
        id.insert("workspace", "Ruang Kerja");
        id.insert("workspaces", "Ruang Kerja");
        id.insert("add_workspace", "Tambah Ruang Kerja");
        id.insert("remove_workspace", "Hapus Ruang Kerja");
        id.insert("overview", "Ikhtisar");
        id.insert("window_preview", "Pratinjau Jendela");
        id.insert("close", "Tutup");
        id.insert("minimize", "Minimalkan");
        id.insert("maximize", "Maksimalkan");
        id.insert("restore", "Kembalikan");
        id.insert("unpin", "Lepas Sematan");
        id.insert("pin", "Sematkan");
        id.insert("remove", "Hapus");
        id.insert("show_desktop", "Tampilkan Desktop");
        id.insert("all_apps", "Semua Aplikasi");
        id.insert("frequently_used", "Sering Digunakan");
        id.insert("recent_apps", "Aplikasi Terbaru");
        id.insert("search_results", "Hasil Pencarian");
        id.insert("no_results", "Hasil tidak ditemukan");
        id.insert("type_to_search", "Ketik untuk mencari...");
        id.insert("learning", "Pembelajaran");
        id.insert("powered_by", "Didukung oleh EduShell");
        self.translations.insert(
            "id-ID".to_string(),
            id.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        );
    }
}

impl Default for LocalizationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localization_default() {
        let lm = LocalizationManager::new();
        assert_eq!(lm.locale(), "en-US");
        assert_eq!(lm.translate("settings"), "Settings");
    }

    #[test]
    fn test_set_locale() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("id-ID");
        assert_eq!(lm.locale(), "id-ID");
        assert_eq!(lm.translate("settings"), "Pengaturan");
    }

    #[test]
    fn test_invalid_locale() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("invalid");
        assert_eq!(lm.locale(), "en-US"); // unchanged
    }

    #[test]
    fn test_fallback_to_key() {
        let lm = LocalizationManager::new();
        assert_eq!(lm.translate("nonexistent_key"), "nonexistent_key");
    }

    #[test]
    fn test_time_format_12h() {
        let mut lm = LocalizationManager::new();
        lm.set_time_format(TimeFormat::Hour12);
        assert_eq!(lm.format_time(14, 30), "2:30 PM");
        assert_eq!(lm.format_time(0, 0), "12:00 AM");
    }

    #[test]
    fn test_time_format_24h() {
        let mut lm = LocalizationManager::new();
        lm.set_time_format(TimeFormat::Hour24);
        assert_eq!(lm.format_time(14, 30), "14:30");
        assert_eq!(lm.format_time(0, 5), "00:05");
    }

    #[test]
    fn test_date_formats() {
        let mut lm = LocalizationManager::new();
        lm.set_date_format(DateFormat::MDY);
        assert_eq!(lm.format_date(2024, 3, 14), "3/14/2024");
        lm.set_date_format(DateFormat::DMY);
        assert_eq!(lm.format_date(2024, 3, 14), "14/3/2024");
        lm.set_date_format(DateFormat::YMD);
        assert_eq!(lm.format_date(2024, 3, 14), "2024-03-14");
    }

    #[test]
    fn test_locale_format_detection() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("en-US");
        assert_eq!(lm.time_format(), TimeFormat::Hour12);
        assert_eq!(lm.date_format(), DateFormat::MDY);
        assert_eq!(lm.first_day_of_week(), 0);

        lm.set_locale("id-ID");
        assert_eq!(lm.time_format(), TimeFormat::Hour24);
        assert_eq!(lm.date_format(), DateFormat::DMY);
        assert_eq!(lm.first_day_of_week(), 1);
    }

    #[test]
    fn test_month_day_names() {
        let lm = LocalizationManager::new();
        assert_eq!(lm.month_name(1), "January");
        assert_eq!(lm.short_month_name(1), "Jan");
        assert_eq!(lm.day_name(0), "Sunday");
        assert_eq!(lm.short_day_name(0), "Sun");
    }

    #[test]
    fn test_measurement_system() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("en-US");
        assert_eq!(lm.measurement(), MeasurementSystem::Imperial);
        lm.set_locale("id-ID");
        assert_eq!(lm.measurement(), MeasurementSystem::Metric);
    }

    #[test]
    fn test_detect_locale() {
        let locale = LocalizationManager::detect_system_locale();
        assert!(!locale.is_empty());
        assert!(locale.len() >= 2);
    }

    #[test]
    fn test_short_day_names() {
        let lm = LocalizationManager::new();
        for i in 0..7 {
            assert!(!lm.short_day_name(i).is_empty());
        }
    }

    #[test]
    fn test_id_translations_complete() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("id-ID");
        assert_eq!(lm.translate("shutdown"), "Matikan");
        assert_eq!(lm.translate("search"), "Ketik untuk mencari...");
        assert_eq!(lm.translate("learning"), "Pembelajaran");
    }

    #[test]
    fn test_available_locales() {
        let lm = LocalizationManager::new();
        assert!(lm.available_locales().contains(&"en-US".to_string()));
        assert!(lm.available_locales().contains(&"id-ID".to_string()));
        assert!(lm.available_locales().contains(&"ja-JP".to_string()));
    }
}
