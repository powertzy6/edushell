// SPDX-License-Identifier: GPL-3.0-or-later

//! # Quick Settings Panel
//!
//! Drop-down panel with toggles for WiFi, Bluetooth, Volume,
//! Brightness, Dark Mode, Night Light, Battery, Power Profile,
//! Airplane Mode, Microphone, Screenshot, and Settings shortcut.

use crate::localization::LocalizationManager;
use crate::settings::SettingsIntegration;
use edushell_core::event::EventBus;

/// Quick Settings panel state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuickSettingsState {
    Closed,
    Open,
}

/// Power profile options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

/// A single quick toggle control.
#[derive(Debug, Clone)]
pub struct QsControl {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub active: bool,
    pub enabled: bool,
}

/// The Quick Settings panel.
pub struct QuickSettingsPanel {
    state: QuickSettingsState,
    controls: Vec<QsControl>,
    volume: f64,
    brightness: f64,
    power_profile: PowerProfile,
    battery_percent: u8,
    battery_charging: bool,
    dark_mode: bool,
    night_light: bool,
    localization: LocalizationManager,
    settings: SettingsIntegration,
    event_bus: EventBus,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
    #[cfg(feature = "gtk")]
    controls_box: Option<gtk::Box>,
}

impl QuickSettingsPanel {
    pub fn new(
        localization: LocalizationManager,
        settings: SettingsIntegration,
        event_bus: EventBus,
    ) -> Self {
        let mut panel = Self {
            state: QuickSettingsState::Closed,
            controls: Vec::new(),
            volume: 75.0,
            brightness: 80.0,
            power_profile: PowerProfile::Balanced,
            battery_percent: 85,
            battery_charging: true,
            dark_mode: settings.dark_mode(),
            night_light: false,
            localization,
            settings,
            event_bus,
            #[cfg(feature = "gtk")]
            window: None,
            #[cfg(feature = "gtk")]
            controls_box: None,
        };
        panel.init_controls();
        panel
    }

    fn init_controls(&mut self) {
        self.controls = vec![
            QsControl { id: "wifi".into(), label: self.localization.translate("wifi"), icon: "network-wireless".into(), active: true, enabled: true },
            QsControl { id: "bluetooth".into(), label: self.localization.translate("bluetooth"), icon: "bluetooth".into(), active: false, enabled: true },
            QsControl { id: "dark-mode".into(), label: self.localization.translate("dark_mode"), icon: "weather-clear-night".into(), active: self.dark_mode, enabled: true },
            QsControl { id: "night-light".into(), label: self.localization.translate("night_light"), icon: "redshift".into(), active: false, enabled: true },
            QsControl { id: "airplane".into(), label: self.localization.translate("airplane_mode"), icon: "network-offline".into(), active: false, enabled: true },
            QsControl { id: "microphone".into(), label: self.localization.translate("microphone"), icon: "audio-input-microphone".into(), active: true, enabled: true },
            QsControl { id: "screenshot".into(), label: self.localization.translate("screenshot"), icon: "camera-photo".into(), active: false, enabled: true },
            QsControl { id: "screen-recording".into(), label: self.localization.translate("screen_recording"), icon: "media-record".into(), active: false, enabled: false },
        ];
    }

    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        tracing::info!(target: "edushell::qs", "Quick Settings panel built");
    }

    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {}

    #[cfg(feature = "gtk")]
    pub fn destroy(&self) {
        if let Some(w) = &self.window { w.close(); }
    }

    #[cfg(not(feature = "gtk"))]
    pub fn destroy(&self) {}

    pub fn open(&mut self) { self.state = QuickSettingsState::Open; }
    pub fn close(&mut self) { self.state = QuickSettingsState::Closed; }
    pub fn toggle(&mut self) {
        self.state = match self.state {
            QuickSettingsState::Closed => QuickSettingsState::Open,
            QuickSettingsState::Open => QuickSettingsState::Closed,
        };
    }
    pub fn is_open(&self) -> bool { self.state == QuickSettingsState::Open }

    pub fn toggle_control(&mut self, id: &str) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.id == id) {
            control.active = !control.active;
            match id {
                "dark-mode" => {
                    self.dark_mode = control.active;
                    self.settings.set_dark_mode(self.dark_mode);
                }
                "night-light" => self.night_light = control.active,
                _ => {}
            }
        }
    }

    pub fn set_control_active(&mut self, id: &str, active: bool) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.id == id) {
            control.active = active;
        }
    }

    pub fn is_control_active(&self, id: &str) -> bool {
        self.controls.iter().find(|c| c.id == id).map(|c| c.active).unwrap_or(false)
    }

    pub fn controls(&self) -> &[QsControl] { &self.controls }

    pub fn set_volume(&mut self, vol: f64) { self.volume = vol.clamp(0.0, 100.0); }
    pub fn volume(&self) -> f64 { self.volume }
    pub fn set_brightness(&mut self, bri: f64) { self.brightness = bri.clamp(0.0, 100.0); }
    pub fn brightness(&self) -> f64 { self.brightness }

    pub fn set_power_profile(&mut self, profile: PowerProfile) { self.power_profile = profile; }
    pub fn power_profile(&self) -> PowerProfile { self.power_profile }

    pub fn set_battery(&mut self, percent: u8, charging: bool) {
        self.battery_percent = percent;
        self.battery_charging = charging;
    }
    pub fn battery_percent(&self) -> u8 { self.battery_percent }
    pub fn battery_charging(&self) -> bool { self.battery_charging }
}

impl std::fmt::Debug for QuickSettingsPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuickSettingsPanel")
            .field("state", &self.state)
            .field("dark_mode", &self.dark_mode)
            .field("volume", &self.volume)
            .field("brightness", &self.brightness)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_panel() -> QuickSettingsPanel {
        let bus = EventBus::new();
        let settings = SettingsIntegration::new(
            edushell_core::config::ConfigManager::new(),
            edushell_core::settings::SettingsBackend::new(),
            bus.clone(),
        );
        QuickSettingsPanel::new(LocalizationManager::new(), settings, bus)
    }

    #[test]
    fn test_creation() {
        let qs = test_panel();
        assert_eq!(qs.state, QuickSettingsState::Closed);
        assert!(!qs.controls.is_empty());
    }

    #[test]
    fn test_state_transitions() {
        let mut qs = test_panel();
        assert!(!qs.is_open());
        qs.open();
        assert!(qs.is_open());
        qs.close();
        assert!(!qs.is_open());
        qs.toggle();
        assert!(qs.is_open());
        qs.toggle();
        assert!(!qs.is_open());
    }

    #[test]
    fn test_toggle_control() {
        let mut qs = test_panel();
        assert!(!qs.is_control_active("dark-mode"));
        qs.toggle_control("dark-mode");
        assert!(qs.is_control_active("dark-mode"));
        qs.toggle_control("dark-mode");
        assert!(!qs.is_control_active("dark-mode"));
    }

    #[test]
    fn test_volume() {
        let mut qs = test_panel();
        qs.set_volume(50.0);
        assert!((qs.volume() - 50.0).abs() < 0.001);
        qs.set_volume(200.0);
        assert!((qs.volume() - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_brightness() {
        let mut qs = test_panel();
        qs.set_brightness(30.0);
        assert!((qs.brightness() - 30.0).abs() < 0.001);
        qs.set_brightness(-10.0);
        assert!((qs.brightness() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_power_profile() {
        let mut qs = test_panel();
        assert_eq!(qs.power_profile(), PowerProfile::Balanced);
        qs.set_power_profile(PowerProfile::Performance);
        assert_eq!(qs.power_profile(), PowerProfile::Performance);
    }

    #[test]
    fn test_battery() {
        let mut qs = test_panel();
        qs.set_battery(50, false);
        assert_eq!(qs.battery_percent(), 50);
        assert!(!qs.battery_charging());
        qs.set_battery(100, true);
        assert_eq!(qs.battery_percent(), 100);
        assert!(qs.battery_charging());
    }

    #[test]
    fn test_set_control_active() {
        let mut qs = test_panel();
        qs.set_control_active("wifi", false);
        assert!(!qs.is_control_active("wifi"));
        qs.set_control_active("wifi", true);
        assert!(qs.is_control_active("wifi"));
    }

    #[test]
    fn test_dark_mode_toggle_syncs_settings() {
        let mut qs = test_panel();
        qs.toggle_control("dark-mode");
        assert!(qs.settings.dark_mode());
        qs.toggle_control("dark-mode");
        assert!(!qs.settings.dark_mode());
    }

    #[test]
    fn test_controls_initialized() {
        let qs = test_panel();
        let ids: Vec<&str> = qs.controls.iter().map(|c| c.id.as_str()).collect();
        assert!(ids.contains(&"wifi"));
        assert!(ids.contains(&"bluetooth"));
        assert!(ids.contains(&"dark-mode"));
        assert!(ids.contains(&"screenshot"));
    }

    #[test]
    fn test_screen_recording_disabled() {
        let qs = test_panel();
        let sr = qs.controls.iter().find(|c| c.id == "screen-recording").unwrap();
        assert!(!sr.enabled);
    }
}
