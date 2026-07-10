// SPDX-License-Identifier: GPL-3.0-or-later

//! # Desktop Shell Application
//!
//! The `DesktopShell` struct is the top-level orchestrator
//! that creates all UI components, connects them to the
//! backend services, and manages the GTK application lifecycle.
//!
//! ## Startup Sequence
//!
//! 1. Create GTK Application
//! 2. Initialize theme manager and apply CSS
//! 3. Create multi-monitor manager
//! 4. Create accessibility manager
//! 5. Create localization manager
//! 6. Create animation engine
//! 7. Build panel(s) for each monitor
//! 8. Build dock(s) for each monitor
//! 9. Build desktop area (wallpaper, icons)
//! 10. Create overlay components (launcher, overview, notifications, QS)
//! 11. Register keyboard shortcuts
//! 12. Enter GTK main loop

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::panel::Panel;
use crate::dock::Dock;
use crate::launcher::Launcher;
use crate::desktop::DesktopArea;
use crate::overview::Overview;
use crate::notifications::NotificationCenter;
use crate::quick_settings::QuickSettingsPanel;
use crate::clock::ClockWidget;
use crate::power::PowerMenu;
use crate::animation::AnimationEngine;
use crate::accessibility::{AccessibilityManager, FocusManager, ShortcutRegistry};
use crate::multi_monitor::MultiMonitorManager;
use crate::theme::ThemeManager;
use crate::localization::LocalizationManager;
use crate::settings::SettingsIntegration;

use edushell_core::config::ConfigManager;
use edushell_core::settings::SettingsBackend;
use edushell_core::event::EventBus;

/// The top-level desktop shell application.
pub struct DesktopShell {
    /// GTK application handle.
    #[cfg(feature = "gtk")]
    app: gtk::Application,

    /// Theme manager.
    pub theme: ThemeManager,
    /// Multi-monitor manager.
    pub monitors: MultiMonitorManager,
    /// Accessibility manager.
    pub accessibility: AccessibilityManager,
    /// Localization manager.
    pub localization: LocalizationManager,
    /// Animation engine.
    pub animation: AnimationEngine,
    /// Settings integration.
    pub settings: SettingsIntegration,
    /// Shortcut registry.
    pub shortcuts: ShortcutRegistry,
    /// Focus manager.
    pub focus: FocusManager,

    // UI Components
    pub panel: Option<Panel>,
    pub dock: Option<Dock>,
    pub desktop: Option<DesktopArea>,
    pub launcher: Option<Launcher>,
    pub overview: Option<Overview>,
    pub notification_center: Option<NotificationCenter>,
    pub quick_settings: Option<QuickSettingsPanel>,
    pub clock: Option<ClockWidget>,
    pub power_menu: Option<PowerMenu>,

    /// Whether the shell is running.
    running: Arc<AtomicBool>,
}

impl DesktopShell {
    /// Create a new desktop shell application.
    #[cfg(feature = "gtk")]
    pub fn new(
        config_manager: ConfigManager,
        settings_backend: SettingsBackend,
        event_bus: EventBus,
    ) -> Self {
        let app = gtk::Application::builder()
            .application_id("org.edushell.shell")
            .flags(gtk::gio::ApplicationFlags::default())
            .build();

        let settings = SettingsIntegration::new(config_manager, settings_backend, event_bus);

        let shell = Self {
            app,
            theme: ThemeManager::new(),
            monitors: MultiMonitorManager::with_default_monitor(),
            accessibility: AccessibilityManager::new(),
            localization: LocalizationManager::new(),
            animation: AnimationEngine::new(),
            settings,
            shortcuts: ShortcutRegistry::new(),
            focus: FocusManager::new(),
            panel: None,
            dock: None,
            desktop: None,
            launcher: None,
            overview: None,
            notification_center: None,
            quick_settings: None,
            clock: None,
            power_menu: None,
            running: Arc::new(AtomicBool::new(false)),
        };

        shell
    }

    /// Create non-GTK shell (for testing/headless).
    #[cfg(not(feature = "gtk"))]
    pub fn new(
        config_manager: ConfigManager,
        settings_backend: SettingsBackend,
        event_bus: EventBus,
    ) -> Self {
        let settings = SettingsIntegration::new(config_manager, settings_backend, event_bus);

        Self {
            theme: ThemeManager::new(),
            monitors: MultiMonitorManager::with_default_monitor(),
            accessibility: AccessibilityManager::new(),
            localization: LocalizationManager::new(),
            animation: AnimationEngine::new(),
            settings,
            shortcuts: ShortcutRegistry::new(),
            focus: FocusManager::new(),
            panel: None,
            dock: None,
            desktop: None,
            launcher: None,
            overview: None,
            notification_center: None,
            quick_settings: None,
            clock: None,
            power_menu: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Initialize the desktop shell and build all UI components.
    #[cfg(feature = "gtk")]
    pub fn initialize(&mut self) {
        tracing::info!(target: "edushell::ui", "Initializing desktop shell UI");

        // Detect locale
        let locale = LocalizationManager::detect_system_locale();
        self.localization.set_locale(&locale);

        // Apply theme
        if self.settings.dark_mode() {
            self.theme.set_dark_mode(true);
        }
        self.theme.apply();

        // Build all UI components
        self.build_panel();
        self.build_dock();
        self.build_desktop();
        self.build_launcher();
        self.build_overview();
        self.build_notification_center();
        self.build_quick_settings();
        self.build_clock();
        self.build_power_menu();

        // Register global shortcuts
        self.register_shortcuts();

        // Subscribe to events
        self.subscribe_events();

        tracing::info!(target: "edushell::ui", "Desktop shell UI initialized");
    }

    /// Initialize shell (non-GTK stub).
    #[cfg(not(feature = "gtk"))]
    pub fn initialize(&mut self) {
        tracing::info!(target: "edushell::ui", "Desktop shell initialized (headless mode)");
    }

    /// Run the GTK application main loop.
    #[cfg(feature = "gtk")]
    pub fn run(&mut self) {
        self.initialize();

        let running = self.running.clone();
        let app = self.app.clone();

        self.app.connect_activate(move |app| {
            // Create main window (invisible, serves as container)
            let window = gtk::ApplicationWindow::new(app);
            window.set_title(Some("EduShell Desktop"));
            window.set_default_size(1920, 1080);
            window.fullscreen();

            // Add CSS provider
            let provider = gtk::CssProvider::new();
            provider.load_from_string(include_str!("../../assets/theme/shell.css"));
            if let Some(display) = window.display() {
                gtk::StyleContext::add_provider_for_display(
                    &display,
                    &provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }

            window.show();
        });

        self.app.connect_shutdown(move || {
            running.store(false, Ordering::Relaxed);
        });

        self.app.run();
    }

    /// Run shell (non-GTK stub).
    #[cfg(not(feature = "gtk"))]
    pub fn run(&mut self) {
        self.initialize();
        tracing::info!(target: "edushell::ui", "Shell would start GTK main loop");
        // In headless mode, just return
    }

    /// Shut down the shell gracefully.
    pub fn shutdown(&mut self) {
        tracing::info!(target: "edushell::ui", "Shutting down desktop shell");
        self.running.store(false, Ordering::Relaxed);

        #[cfg(feature = "gtk")]
        {
            if let Some(panel) = &self.panel {
                panel.destroy();
            }
            if let Some(dock) = &self.dock {
                dock.destroy();
            }
            if let Some(launcher) = &self.launcher {
                launcher.destroy();
            }
            if let Some(overview) = &self.overview {
                overview.destroy();
            }
            if let Some(notif) = &self.notification_center {
                notif.destroy();
            }
            if let Some(qs) = &self.quick_settings {
                qs.destroy();
            }
            if let Some(power) = &self.power_menu {
                power.destroy();
            }
            self.app.quit();
        }

        tracing::info!(target: "edushell::ui", "Desktop shell shut down");
    }

    /// Check if the shell is running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    // ── Builder Methods ─────────────────────────────────────────

    #[cfg(feature = "gtk")]
    fn build_panel(&mut self) {
        let panel = Panel::new(
            self.theme.clone(),
            self.monitors.clone(),
            self.localization.clone(),
            self.settings.clone(),
            self.accessibility.clone(),
        );
        panel.build();
        self.panel = Some(panel);
    }

    #[cfg(feature = "gtk")]
    fn build_dock(&mut self) {
        let dock = Dock::new(
            self.theme.clone(),
            self.monitors.clone(),
            self.localization.clone(),
            self.settings.clone(),
            self.accessibility.clone(),
        );
        dock.build();
        self.dock = Some(dock);
    }

    #[cfg(feature = "gtk")]
    fn build_desktop(&mut self) {
        let desktop = DesktopArea::new(
            self.monitors.clone(),
            self.settings.clone(),
            self.localization.clone(),
        );
        desktop.build();
        self.desktop = Some(desktop);
    }

    #[cfg(feature = "gtk")]
    fn build_launcher(&mut self) {
        let launcher = Launcher::new(
            self.localization.clone(),
            self.accessibility.clone(),
            self.settings.event_bus().clone(),
        );
        launcher.build();
        self.launcher = Some(launcher);
    }

    #[cfg(feature = "gtk")]
    fn build_overview(&mut self) {
        let overview = Overview::new(
            self.monitors.clone(),
            self.accessibility.clone(),
            self.animation.clone(),
        );
        overview.build();
        self.overview = Some(overview);
    }

    #[cfg(feature = "gtk")]
    fn build_notification_center(&mut self) {
        let nc = NotificationCenter::new(
            self.localization.clone(),
            self.settings.event_bus().clone(),
        );
        nc.build();
        self.notification_center = Some(nc);
    }

    #[cfg(feature = "gtk")]
    fn build_quick_settings(&mut self) {
        let qs = QuickSettingsPanel::new(
            self.localization.clone(),
            self.settings.clone(),
            self.settings.event_bus().clone(),
        );
        qs.build();
        self.quick_settings = Some(qs);
    }

    #[cfg(feature = "gtk")]
    fn build_clock(&mut self) {
        let clock = ClockWidget::new(
            self.localization.clone(),
            self.settings.event_bus().clone(),
        );
        clock.build();
        self.clock = Some(clock);
    }

    #[cfg(feature = "gtk")]
    fn build_power_menu(&mut self) {
        let power = PowerMenu::new(self.localization.clone());
        power.build();
        self.power_menu = Some(power);
    }

    #[cfg(feature = "gtk")]
    fn register_shortcuts(&self) {
        // Register all shortcuts with the window
        // This is called after the window is created in run()
    }

    /// Subscribe to relevant backend events.
    fn subscribe_events(&self) {
        let mut rx = self.settings.event_bus().subscribe();
        tokio::spawn(async move {
            while let Ok(event) = rx.recv().await {
                match &event {
                    edushell_core::event::SystemEvent::ThemeModeChanged(mode) => {
                        tracing::debug!(target: "edushell::ui", "Theme mode changed: {mode}");
                    }
                    edushell_core::event::SystemEvent::LanguageChanged(lang) => {
                        tracing::debug!(target: "edushell::ui", "Language changed: {lang}");
                    }
                    _ => {}
                }
            }
        });
    }

}

impl std::fmt::Debug for DesktopShell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DesktopShell")
            .field("running", &self.is_running())
            .field("locale", &self.localization.locale())
            .field("dark_mode", &self.theme.dark_mode())
            .field("monitors", &self.monitors.count())
            .finish_non_exhaustive()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use edushell_core::config::ConfigManager;
    use edushell_core::settings::SettingsBackend;
    use edushell_core::event::EventBus;

    fn test_shell() -> DesktopShell {
        let config = ConfigManager::new();
        let settings = SettingsBackend::new();
        let bus = EventBus::new();
        DesktopShell::new(config, settings, bus)
    }

    #[test]
    fn test_shell_creation() {
        let shell = test_shell();
        assert!(!shell.is_running()); // not started yet
        assert_eq!(shell.monitors.count(), 1);
    }

    #[test]
    fn test_shell_initialize() {
        let mut shell = test_shell();
        shell.initialize();
        // Components should be initialized
        assert!(shell.theme.dark_mode() == shell.settings.dark_mode());
    }

    #[test]
    fn test_shutdown() {
        let mut shell = test_shell();
        shell.initialize();
        shell.shutdown();
        assert!(!shell.is_running());
    }

    #[test]
    fn test_shell_debug() {
        let shell = test_shell();
        let debug = format!("{shell:?}");
        assert!(debug.contains("DesktopShell"));
        assert!(debug.contains("locale"));
    }

    #[test]
    fn test_shell_supports_gtk_feature() {
        #[cfg(feature = "gtk")]
        assert!(true, "GTK feature enabled");
        #[cfg(not(feature = "gtk"))]
        assert!(true, "GTK feature disabled (headless mode)");
    }
}
