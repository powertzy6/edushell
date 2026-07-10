// SPDX-License-Identifier: GPL-3.0-or-later

//! # Settings Integration
//!
//! Bridges the UI layer with the core `ConfigManager` and
//! `SettingsBackend`. Provides reactive setting watchers,
//! hot-reload support, and a unified settings API for all
//! UI components.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use edushell_core::config::{ConfigManager, ThemeMode};
use edushell_core::settings::SettingsBackend;
use edushell_core::event::EventBus;
use edushell_core::event::SystemEvent;

/// Integrates UI settings with the core configuration engine.
#[derive(Clone)]
pub struct SettingsIntegration {
    /// Core configuration manager.
    config_manager: ConfigManager,
    /// Core settings backend.
    settings_backend: SettingsBackend,
    /// Event bus for reactive updates.
    event_bus: EventBus,
    /// Whether the UI should use dark mode.
    dark_mode: Arc<AtomicBool>,
    /// Panel position (true = bottom, false = top).
    panel_bottom: Arc<AtomicBool>,
    /// Whether the panel auto-hides.
    panel_auto_hide: Arc<AtomicBool>,
    /// Whether the dock auto-hides.
    dock_auto_hide: Arc<AtomicBool>,
}

impl SettingsIntegration {
    /// Create a new settings integration.
    pub fn new(
        config_manager: ConfigManager,
        settings_backend: SettingsBackend,
        event_bus: EventBus,
    ) -> Self {
        let dm = config_manager
            .current()
            .ok()
            .map(|c| c.theme.mode == ThemeMode::Dark)
            .unwrap_or(false);
        let dark_mode = Arc::new(AtomicBool::new(dm));
        let panel_bottom = Arc::new(AtomicBool::new(false));
        let panel_auto_hide = Arc::new(AtomicBool::new(false));
        let dock_auto_hide = Arc::new(AtomicBool::new(false));

        Self {
            config_manager,
            settings_backend,
            event_bus,
            dark_mode,
            panel_bottom,
            panel_auto_hide,
            dock_auto_hide,
        }
    }

    /// Get the configuration manager.
    pub fn config_manager(&self) -> &ConfigManager {
        &self.config_manager
    }

    /// Get the settings backend.
    pub fn settings_backend(&self) -> &SettingsBackend {
        &self.settings_backend
    }

    /// Get the event bus.
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Check if dark mode is active.
    pub fn dark_mode(&self) -> bool {
        self.dark_mode.load(Ordering::Relaxed)
    }

    /// Toggle dark mode.
    pub fn set_dark_mode(&self, enabled: bool) {
        self.dark_mode.store(enabled, Ordering::Relaxed);
        let mode = if enabled { ThemeMode::Dark } else { ThemeMode::Light };
        let _ = self.config_manager.update(|config| {
            config.theme.mode = mode;
        });
        let _ = self.event_bus.publish(
            SystemEvent::ThemeModeChanged(if enabled { "dark".into() } else { "light".into() }),
        );
    }

    /// Check if panel is at bottom.
    pub fn panel_bottom(&self) -> bool {
        self.panel_bottom.load(Ordering::Relaxed)
    }

    /// Set panel position.
    pub fn set_panel_bottom(&self, bottom: bool) {
        self.panel_bottom.store(bottom, Ordering::Relaxed);
    }

    /// Check if panel auto-hides.
    pub fn panel_auto_hide(&self) -> bool {
        self.panel_auto_hide.load(Ordering::Relaxed)
    }

    /// Set panel auto-hide.
    pub fn set_panel_auto_hide(&self, enabled: bool) {
        self.panel_auto_hide.store(enabled, Ordering::Relaxed);
        let _ = self.event_bus.publish(
            SystemEvent::ThemeModeChanged(if enabled { "system".into() } else { "light".into() }),
        );
    }

    /// Check if dock auto-hides.
    pub fn dock_auto_hide(&self) -> bool {
        self.dock_auto_hide.load(Ordering::Relaxed)
    }

    /// Set dock auto-hide.
    pub fn set_dock_auto_hide(&self, enabled: bool) {
        self.dock_auto_hide.store(enabled, Ordering::Relaxed);
        let _ = self.event_bus.publish(
            SystemEvent::ThemeModeChanged(if enabled { "system".into() } else { "light".into() }),
        );
    }

    /// Reload configuration from disk.
    pub fn reload_config(&self) -> edushell_core::error::EduResult<()> {
        let config = self.config_manager.load()?;
        self.dark_mode.store(config.theme.mode == ThemeMode::Dark, Ordering::Relaxed);
        let _ = self.event_bus.publish(
            SystemEvent::ThemeModeChanged(if config.theme.mode == ThemeMode::Dark { "dark".into() } else { "light".into() }),
        );
        Ok(())
    }
}

impl std::fmt::Debug for SettingsIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsIntegration")
            .field("dark_mode", &self.dark_mode())
            .field("panel_bottom", &self.panel_bottom())
            .finish_non_exhaustive()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_starts_with_config() {
        let config = ConfigManager::new();
        let settings = SettingsBackend::new();
        let bus = EventBus::new();
        let integration = SettingsIntegration::new(config, settings, bus);
        assert!(!integration.dark_mode()); // default light mode
    }

    #[test]
    fn test_toggle_dark_mode() {
        let config = ConfigManager::new();
        let settings = SettingsBackend::new();
        let bus = EventBus::new();
        let integration = SettingsIntegration::new(config, settings, bus);
        integration.set_dark_mode(true);
        assert!(integration.dark_mode());
    }

    #[test]
    fn test_panel_auto_hide() {
        let config = ConfigManager::new();
        let settings = SettingsBackend::new();
        let bus = EventBus::new();
        let integration = SettingsIntegration::new(config, settings, bus);
        assert!(!integration.panel_auto_hide());
        integration.set_panel_auto_hide(true);
        assert!(integration.panel_auto_hide());
    }

    #[test]
    fn test_dock_auto_hide() {
        let config = ConfigManager::new();
        let settings = SettingsBackend::new();
        let bus = EventBus::new();
        let integration = SettingsIntegration::new(config, settings, bus);
        assert!(!integration.dock_auto_hide());
        integration.set_dock_auto_hide(true);
        assert!(integration.dock_auto_hide());
    }

    #[test]
    fn test_panel_position() {
        let config = ConfigManager::new();
        let settings = SettingsBackend::new();
        let bus = EventBus::new();
        let integration = SettingsIntegration::new(config, settings, bus);
        assert!(!integration.panel_bottom());
        integration.set_panel_bottom(true);
        assert!(integration.panel_bottom());
    }
}
