// SPDX-License-Identifier: GPL-3.0-or-later

//! # Top Panel
//!
//! The main desktop panel providing application menu, running
//! applications indicator, workspace switcher, clock, system
//! tray, and quick settings access.
//!
//! Position: configurable (top/bottom)
//! Behavior: auto-hide, transparency, blur

pub mod app_menu;
pub mod running_apps;
pub mod workspace;
pub mod clock_widget;
pub mod system_tray;
pub mod quick_settings;

use crate::accessibility::AccessibilityManager;
use crate::localization::LocalizationManager;
use crate::multi_monitor::MultiMonitorManager;
use crate::settings::SettingsIntegration;
use crate::theme::ThemeManager;

/// Panel position on screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelPosition {
    Top,
    Bottom,
}

/// Panel visibility state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelState {
    Visible,
    AutoHidden,
    Hidden,
}

/// The main desktop panel.
pub struct Panel {
    position: PanelPosition,
    state: PanelState,
    height: i32,
    auto_hide: bool,
    transparent: bool,
    blur: bool,
    theme: ThemeManager,
    monitors: MultiMonitorManager,
    localization: LocalizationManager,
    settings: SettingsIntegration,
    accessibility: AccessibilityManager,

    #[cfg(feature = "gtk")]
    widget: Option<gtk::Box>,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
}

impl Panel {
    /// Create a new panel with all required dependencies.
    pub fn new(
        theme: ThemeManager,
        monitors: MultiMonitorManager,
        localization: LocalizationManager,
        settings: SettingsIntegration,
        accessibility: AccessibilityManager,
    ) -> Self {
        Self {
            position: PanelPosition::Top,
            state: PanelState::Visible,
            height: 36,
            auto_hide: false,
            transparent: false,
            blur: false,
            theme,
            monitors,
            localization,
            settings,
            accessibility,
            #[cfg(feature = "gtk")]
            widget: None,
            #[cfg(feature = "gtk")]
            window: None,
        }
    }

    /// Create a panel with custom settings via builder pattern.
    pub fn builder() -> PanelBuilder {
        PanelBuilder::default()
    }

    /// Set panel position.
    pub fn set_position(&mut self, position: PanelPosition) {
        self.position = position;
    }

    /// Get panel position.
    pub fn position(&self) -> PanelPosition {
        self.position
    }

    /// Set panel height in pixels.
    pub fn set_height(&mut self, height: i32) {
        self.height = height.max(24).min(96);
    }

    /// Get panel height in pixels.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Enable or disable auto-hide.
    pub fn set_auto_hide(&mut self, enabled: bool) {
        self.auto_hide = enabled;
    }

    /// Check if auto-hide is enabled.
    pub fn auto_hide(&self) -> bool {
        self.auto_hide
    }

    /// Enable or disable transparency.
    pub fn set_transparent(&mut self, enabled: bool) {
        self.transparent = enabled;
    }

    /// Check if transparency is enabled.
    pub fn transparent(&self) -> bool {
        self.transparent
    }

    /// Enable or disable background blur.
    pub fn set_blur(&mut self, enabled: bool) {
        self.blur = enabled;
    }

    /// Check if blur is enabled.
    pub fn blur(&self) -> bool {
        self.blur
    }

    /// Get current panel state.
    pub fn state(&self) -> PanelState {
        self.state
    }

    /// Set panel visibility state.
    pub fn set_state(&mut self, state: PanelState) {
        self.state = state;
    }

    /// Toggle panel visibility.
    pub fn toggle_visibility(&mut self) {
        self.state = match self.state {
            PanelState::Visible => PanelState::Hidden,
            PanelState::AutoHidden => PanelState::Visible,
            PanelState::Hidden => PanelState::Visible,
        };
    }

    /// Get a reference to the theme manager.
    pub fn theme(&self) -> &ThemeManager {
        &self.theme
    }

    /// Get a reference to the multi-monitor manager.
    pub fn monitors(&self) -> &MultiMonitorManager {
        &self.monitors
    }

    /// Get a reference to the localization manager.
    pub fn localization(&self) -> &LocalizationManager {
        &self.localization
    }

    /// Get a reference to the settings integration.
    pub fn settings(&self) -> &SettingsIntegration {
        &self.settings
    }

    /// Get a reference to the accessibility manager.
    pub fn accessibility(&self) -> &AccessibilityManager {
        &self.accessibility
    }

    /// Build the app menu sub-component.
    pub fn app_menu(&self) -> app_menu::AppMenuButton {
        app_menu::AppMenuButton::new(self.localization.clone())
    }

    /// Build the running apps sub-component.
    pub fn running_apps(&self) -> running_apps::RunningApps {
        running_apps::RunningApps::new()
    }

    /// Build the workspace indicator sub-component.
    pub fn workspace_indicator(&self) -> workspace::WorkspaceIndicator {
        workspace::WorkspaceIndicator::new()
    }

    /// Build the clock sub-component.
    pub fn clock(&self) -> clock_widget::PanelClock {
        clock_widget::PanelClock::new()
    }

    /// Build the system tray sub-component.
    pub fn system_tray(&self) -> system_tray::SystemTray {
        system_tray::SystemTray::new()
    }

    /// Build the quick settings button sub-component.
    pub fn quick_settings_button(&self) -> quick_settings::PanelQuickSettings {
        quick_settings::PanelQuickSettings::new()
    }

    /// Build the complete panel UI (GTK-enabled).
    #[cfg(feature = "gtk")]
    pub fn build(&mut self) {
        let panel_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        panel_box.add_css_class("panel");

        if self.transparent {
            panel_box.add_css_class("panel-transparent");
        }
        if self.blur {
            panel_box.add_css_class("panel-blur");
        }
        if self.position == PanelPosition::Bottom {
            panel_box.add_css_class("panel-bottom");
        } else {
            panel_box.add_css_class("panel-top");
        }

        panel_box.set_height_request(self.height);

        let window = gtk::Window::new(gtk::WindowType::Popup);
        window.set_child(Some(&panel_box));
        window.set_accept_focus(false);

        self.widget = Some(panel_box);
        self.window = Some(window);
    }

    /// Build the panel UI (non-GTK stub).
    #[cfg(not(feature = "gtk"))]
    pub fn build(&mut self) {
        tracing::info!(target: "edushell::ui::panel", "Panel configuration recorded (GTK not available): pos={pos:?}, height={h}, auto_hide={ah}",
            pos = self.position, h = self.height, ah = self.auto_hide);
    }

    /// Show the panel (GTK-enabled).
    #[cfg(feature = "gtk")]
    pub fn show(&mut self) {
        if let Some(window) = &self.window {
            window.present();
        }
        self.state = PanelState::Visible;
    }

    /// Show the panel (non-GTK stub).
    #[cfg(not(feature = "gtk"))]
    pub fn show(&mut self) {
        self.state = PanelState::Visible;
    }

    /// Hide the panel (GTK-enabled).
    #[cfg(feature = "gtk")]
    pub fn hide(&mut self) {
        if let Some(window) = &self.window {
            window.hide();
        }
        self.state = PanelState::Hidden;
    }

    /// Hide the panel (non-GTK stub).
    #[cfg(not(feature = "gtk"))]
    pub fn hide(&mut self) {
        self.state = PanelState::Hidden;
    }

    /// Destroy the panel and free resources (GTK-enabled).
    #[cfg(feature = "gtk")]
    pub fn destroy(&mut self) {
        if let Some(window) = self.window.take() {
            window.close();
        }
        self.widget = None;
        self.state = PanelState::Hidden;
    }

    /// Destroy the panel (non-GTK stub).
    #[cfg(not(feature = "gtk"))]
    pub fn destroy(&mut self) {
        self.state = PanelState::Hidden;
    }
}

impl std::fmt::Debug for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Panel")
            .field("position", &self.position)
            .field("state", &self.state)
            .field("height", &self.height)
            .field("auto_hide", &self.auto_hide)
            .field("transparent", &self.transparent)
            .field("blur", &self.blur)
            .finish_non_exhaustive()
    }
}

/// Builder for creating a `Panel` with custom configuration.
#[derive(Default)]
pub struct PanelBuilder {
    position: Option<PanelPosition>,
    height: Option<i32>,
    auto_hide: Option<bool>,
    transparent: Option<bool>,
    blur: Option<bool>,
}

impl PanelBuilder {
    /// Set panel position.
    pub fn position(mut self, pos: PanelPosition) -> Self {
        self.position = Some(pos);
        self
    }

    /// Set panel height.
    pub fn height(mut self, h: i32) -> Self {
        self.height = Some(h);
        self
    }

    /// Enable or disable auto-hide.
    pub fn auto_hide(mut self, enabled: bool) -> Self {
        self.auto_hide = Some(enabled);
        self
    }

    /// Enable or disable transparency.
    pub fn transparent(mut self, enabled: bool) -> Self {
        self.transparent = Some(enabled);
        self
    }

    /// Enable or disable blur.
    pub fn blur(mut self, enabled: bool) -> Self {
        self.blur = Some(enabled);
        self
    }

    /// Build the panel, injecting required dependencies.
    pub fn build(
        self,
        theme: ThemeManager,
        monitors: MultiMonitorManager,
        localization: LocalizationManager,
        settings: SettingsIntegration,
        accessibility: AccessibilityManager,
    ) -> Panel {
        Panel {
            position: self.position.unwrap_or(PanelPosition::Top),
            state: PanelState::Visible,
            height: self.height.unwrap_or(36),
            auto_hide: self.auto_hide.unwrap_or(false),
            transparent: self.transparent.unwrap_or(false),
            blur: self.blur.unwrap_or(false),
            theme,
            monitors,
            localization,
            settings,
            accessibility,
            #[cfg(feature = "gtk")]
            widget: None,
            #[cfg(feature = "gtk")]
            window: None,
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_panel() -> Panel {
        Panel::new(
            ThemeManager::new(),
            MultiMonitorManager::with_default_monitor(),
            LocalizationManager::new(),
            SettingsIntegration::new(
                edushell_core::config::ConfigManager::new(),
                edushell_core::settings::SettingsBackend::new(),
                edushell_core::event::EventBus::new(),
            ),
            AccessibilityManager::new(),
        )
    }

    #[test]
    fn test_panel_default_construction() {
        let panel = make_panel();
        assert_eq!(panel.position(), PanelPosition::Top);
        assert_eq!(panel.state(), PanelState::Visible);
        assert_eq!(panel.height(), 36);
        assert!(!panel.auto_hide());
        assert!(!panel.transparent());
        assert!(!panel.blur());
    }

    #[test]
    fn test_panel_position_setter() {
        let mut panel = make_panel();
        panel.set_position(PanelPosition::Bottom);
        assert_eq!(panel.position(), PanelPosition::Bottom);
    }

    #[test]
    fn test_panel_height_clamping() {
        let mut panel = make_panel();
        panel.set_height(10);
        assert_eq!(panel.height(), 24);
        panel.set_height(200);
        assert_eq!(panel.height(), 96);
        panel.set_height(48);
        assert_eq!(panel.height(), 48);
    }

    #[test]
    fn test_auto_hide_toggle() {
        let mut panel = make_panel();
        panel.set_auto_hide(true);
        assert!(panel.auto_hide());
        panel.set_auto_hide(false);
        assert!(!panel.auto_hide());
    }

    #[test]
    fn test_transparency_toggle() {
        let mut panel = make_panel();
        panel.set_transparent(true);
        assert!(panel.transparent());
    }

    #[test]
    fn test_blur_toggle() {
        let mut panel = make_panel();
        panel.set_blur(true);
        assert!(panel.blur());
    }

    #[test]
    fn test_panel_state_transitions() {
        let mut panel = make_panel();
        assert_eq!(panel.state(), PanelState::Visible);
        panel.set_state(PanelState::Hidden);
        assert_eq!(panel.state(), PanelState::Hidden);
        panel.set_state(PanelState::AutoHidden);
        assert_eq!(panel.state(), PanelState::AutoHidden);
    }

    #[test]
    fn test_toggle_visibility_from_visible() {
        let mut panel = make_panel();
        panel.toggle_visibility();
        assert_eq!(panel.state(), PanelState::Hidden);
    }

    #[test]
    fn test_toggle_visibility_from_hidden() {
        let mut panel = make_panel();
        panel.set_state(PanelState::Hidden);
        panel.toggle_visibility();
        assert_eq!(panel.state(), PanelState::Visible);
    }

    #[test]
    fn test_toggle_visibility_from_autohidden() {
        let mut panel = make_panel();
        panel.set_state(PanelState::AutoHidden);
        panel.toggle_visibility();
        assert_eq!(panel.state(), PanelState::Visible);
    }

    #[test]
    fn test_show_hide() {
        let mut panel = make_panel();
        panel.hide();
        assert_eq!(panel.state(), PanelState::Hidden);
        panel.show();
        assert_eq!(panel.state(), PanelState::Visible);
    }

    #[test]
    fn test_destroy() {
        let mut panel = make_panel();
        panel.destroy();
        assert_eq!(panel.state(), PanelState::Hidden);
    }

    #[test]
    fn test_builder_default() {
        let panel = Panel::builder()
            .build(
                ThemeManager::new(),
                MultiMonitorManager::with_default_monitor(),
                LocalizationManager::new(),
                SettingsIntegration::new(
                    edushell_core::config::ConfigManager::new(),
                    edushell_core::settings::SettingsBackend::new(),
                    edushell_core::event::EventBus::new(),
                ),
                AccessibilityManager::new(),
            );
        assert_eq!(panel.position(), PanelPosition::Top);
        assert_eq!(panel.height(), 36);
    }

    #[test]
    fn test_builder_custom() {
        let panel = Panel::builder()
            .position(PanelPosition::Bottom)
            .height(48)
            .auto_hide(true)
            .transparent(true)
            .blur(true)
            .build(
                ThemeManager::new(),
                MultiMonitorManager::with_default_monitor(),
                LocalizationManager::new(),
                SettingsIntegration::new(
                    edushell_core::config::ConfigManager::new(),
                    edushell_core::settings::SettingsBackend::new(),
                    edushell_core::event::EventBus::new(),
                ),
                AccessibilityManager::new(),
            );
        assert_eq!(panel.position(), PanelPosition::Bottom);
        assert_eq!(panel.height(), 48);
        assert!(panel.auto_hide());
        assert!(panel.transparent());
        assert!(panel.blur());
    }

    #[test]
    fn test_sub_component_construction() {
        let panel = make_panel();
        let _menu = panel.app_menu();
        let _apps = panel.running_apps();
        let _ws = panel.workspace_indicator();
        let _clock = panel.clock();
        let _tray = panel.system_tray();
        let _qs = panel.quick_settings_button();
    }

    #[test]
    fn test_accessor_methods() {
        let panel = make_panel();
        panel.theme();
        panel.monitors();
        panel.localization();
        panel.settings();
        panel.accessibility();
    }

    #[test]
    fn test_debug_format() {
        let panel = make_panel();
        let debug = format!("{panel:?}");
        assert!(debug.contains("Panel"));
        assert!(debug.contains("Top"));
        assert!(debug.contains("Visible"));
    }

    #[test]
    fn test_build_non_gtk() {
        let mut panel = make_panel();
        panel.build();
        assert_eq!(panel.state(), PanelState::Visible);
    }
}
