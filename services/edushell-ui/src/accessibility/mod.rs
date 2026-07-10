// SPDX-License-Identifier: GPL-3.0-or-later

//! # Accessibility Integration
//!
//! Manages keyboard navigation, focus rings, screen reader
//! integration, high contrast mode, large text, and reduced
//! motion preferences for the desktop shell.
//!
//! ## Architecture
//!
//! `AccessibilityManager` serves as the central coordinator,
//! owned by `DesktopShell` and passed to all UI components.

/// Manager for all accessibility features.
#[derive(Clone)]
pub struct AccessibilityManager {
    /// Whether high contrast mode is enabled.
    high_contrast: bool,
    /// Whether large text is enabled.
    large_text: bool,
    /// Whether reduced motion is enabled.
    reduced_motion: bool,
    /// Current text scale factor (1.0 = normal).
    text_scale: f64,
    /// Focus ring width in pixels.
    focus_ring_width: i32,
    /// Whether screen reader is active.
    screen_reader_active: bool,
}

impl AccessibilityManager {
    /// Create a new accessibility manager.
    pub fn new() -> Self {
        Self {
            high_contrast: false,
            large_text: false,
            reduced_motion: false,
            text_scale: 1.0,
            focus_ring_width: 2,
            screen_reader_active: false,
        }
    }

    /// Toggle high contrast mode.
    pub fn set_high_contrast(&mut self, enabled: bool) {
        self.high_contrast = enabled;
    }

    /// Check if high contrast is enabled.
    pub fn high_contrast(&self) -> bool {
        self.high_contrast
    }

    /// Toggle large text.
    pub fn set_large_text(&mut self, enabled: bool) {
        self.large_text = enabled;
        self.text_scale = if enabled { 1.25 } else { 1.0 };
    }

    /// Check if large text is enabled.
    pub fn large_text(&self) -> bool {
        self.large_text
    }

    /// Toggle reduced motion.
    pub fn set_reduced_motion(&mut self, enabled: bool) {
        self.reduced_motion = enabled;
    }

    /// Check if reduced motion is enabled.
    pub fn reduced_motion(&self) -> bool {
        self.reduced_motion
    }

    /// Get text scale factor.
    pub fn text_scale(&self) -> f64 {
        self.text_scale
    }

    /// Set text scale factor directly.
    pub fn set_text_scale(&mut self, scale: f64) {
        self.text_scale = scale.clamp(0.5, 2.0);
    }

    /// Get focus ring width.
    pub fn focus_ring_width(&self) -> i32 {
        self.focus_ring_width
    }

    /// Set focus ring width.
    pub fn set_focus_ring_width(&mut self, width: i32) {
        self.focus_ring_width = width.clamp(1, 8);
    }

    /// Activate/deactivate screen reader.
    pub fn set_screen_reader(&mut self, active: bool) {
        self.screen_reader_active = active;
    }

    /// Check if screen reader is active.
    pub fn screen_reader_active(&self) -> bool {
        self.screen_reader_active
    }

    /// Announce a message to the screen reader.
    pub fn announce(&self, message: &str) {
        if self.screen_reader_active {
            tracing::info!(target: "edushell::a11y", "Screen reader: {message}");
            #[cfg(feature = "gtk")]
            self.announce_gtk(message);
        }
    }

    #[cfg(feature = "gtk")]
    fn announce_gtk(&self, message: &str) {
        use gtk::prelude::*;
        if let Some(display) = gtk::gdk::Display::default() {
            display.beep();
        }
        tracing::debug!(target: "edushell::a11y", "GTK announcement: {message}");
    }
}

impl Default for AccessibilityManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Focus Management ────────────────────────────────────────────

/// Tracks the currently focused widget and navigation history.
#[derive(Clone)]
pub struct FocusManager {
    /// Widget IDs in focus order.
    focus_order: Vec<String>,
    /// Currently focused widget ID.
    current_focus: Option<String>,
    /// Navigation history for back-navigation.
    history: Vec<String>,
}

impl FocusManager {
    /// Create a new focus manager.
    pub fn new() -> Self {
        Self {
            focus_order: Vec::new(),
            current_focus: None,
            history: Vec::new(),
        }
    }

    /// Register a focusable widget.
    pub fn register(&mut self, id: &str) {
        if !self.focus_order.contains(&id.to_string()) {
            self.focus_order.push(id.to_string());
        }
    }

    /// Remove a focusable widget.
    pub fn remove(&mut self, id: &str) {
        self.focus_order.retain(|f| f != id);
        if self.current_focus.as_deref() == Some(id) {
            self.current_focus = None;
        }
    }

    /// Set focus to a specific widget.
    pub fn focus(&mut self, id: &str) {
        if let Some(old) = self.current_focus.take() {
            self.history.push(old);
        }
        self.current_focus = Some(id.to_string());
    }

    /// Move focus to the next widget in order.
    pub fn focus_next(&mut self) -> Option<&str> {
        let current = self.current_focus.as_deref()?;
        let pos = self.focus_order.iter().position(|f| f == current)?;
        let next = (pos + 1) % self.focus_order.len();
        let id = self.focus_order[next].clone();
        self.focus(&id);
        Some(self.current_focus.as_deref().unwrap())
    }

    /// Move focus to the previous widget in order.
    pub fn focus_prev(&mut self) -> Option<&str> {
        let current = self.current_focus.as_deref()?;
        let pos = self.focus_order.iter().position(|f| f == current)?;
        let prev = if pos == 0 { self.focus_order.len() - 1 } else { pos - 1 };
        let id = self.focus_order[prev].clone();
        self.focus(&id);
        Some(self.current_focus.as_deref().unwrap())
    }

    /// Navigate back to the previously focused widget.
    pub fn focus_back(&mut self) -> Option<&str> {
        if let Some(prev) = self.history.pop() {
            self.current_focus = Some(prev.clone());
            return self.current_focus.as_deref();
        }
        None
    }

    /// Get the currently focused widget ID.
    pub fn current(&self) -> Option<&str> {
        self.current_focus.as_deref()
    }

    /// Clear all focus state.
    pub fn clear(&mut self) {
        self.current_focus = None;
        self.history.clear();
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Keyboard Shortcuts ──────────────────────────────────────────

/// A keyboard shortcut binding.
#[derive(Debug, Clone)]
pub struct Shortcut {
    /// Key name (e.g. "F1", "Escape", "t").
    pub key: String,
    /// Whether Ctrl is required.
    pub ctrl: bool,
    /// Whether Alt is required.
    pub alt: bool,
    /// Whether Shift is required.
    pub shift: bool,
    /// Whether Super/Windows key is required.
    pub super_key: bool,
    /// Action identifier.
    pub action: String,
    /// Human-readable description.
    pub description: String,
}

impl Shortcut {
    /// Create a new shortcut.
    pub fn new(key: &str, action: &str, description: &str) -> Self {
        Self {
            key: key.to_string(),
            ctrl: false,
            alt: false,
            shift: false,
            super_key: false,
            action: action.to_string(),
            description: description.to_string(),
        }
    }

    /// Add Ctrl modifier.
    pub fn with_ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    /// Add Alt modifier.
    pub fn with_alt(mut self) -> Self {
        self.alt = true;
        self
    }

    /// Add Shift modifier.
    pub fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    /// Add Super modifier.
    pub fn with_super(mut self) -> Self {
        self.super_key = true;
        self
    }
}

/// Register of all keyboard shortcuts.
#[derive(Clone)]
pub struct ShortcutRegistry {
    shortcuts: Vec<Shortcut>,
}

impl ShortcutRegistry {
    /// Create a new shortcut registry with defaults.
    pub fn new() -> Self {
        let mut reg = Self { shortcuts: Vec::new() };
        reg.register_defaults();
        reg
    }

    fn register_defaults(&mut self) {
        // Navigation
        self.shortcuts.push(Shortcut::new("Escape", "close-overlay", "Close current overlay").with_ctrl());
        self.shortcuts.push(Shortcut::new("F1", "open-help", "Open help"));
        self.shortcuts.push(Shortcut::new("Tab", "focus-next", "Focus next widget"));
        self.shortcuts.push(Shortcut::new("Tab", "focus-prev", "Focus previous widget").with_shift());

        // Launcher
        self.shortcuts.push(Shortcut::new("space", "open-launcher", "Open app launcher").with_alt());
        self.shortcuts.push(Shortcut::new("Escape", "close-launcher", "Close launcher"));

        // Overview
        self.shortcuts.push(Shortcut::new("Tab", "toggle-overview", "Toggle overview").with_super());
        self.shortcuts.push(Shortcut::new("Tab", "toggle-overview", "Toggle overview (alt)").with_alt().with_ctrl());

        // Workspaces
        self.shortcuts.push(Shortcut::new("Left", "workspace-prev", "Previous workspace").with_super().with_ctrl());
        self.shortcuts.push(Shortcut::new("Right", "workspace-next", "Next workspace").with_super().with_ctrl());
        self.shortcuts.push(Shortcut::new("Up", "workspace-up", "Move to workspace above").with_super().with_ctrl());
        self.shortcuts.push(Shortcut::new("Down", "workspace-down", "Move to workspace below").with_super().with_ctrl());

        // Windows
        self.shortcuts.push(Shortcut::new("w", "close-window", "Close current window").with_alt());
        self.shortcuts.push(Shortcut::new("F4", "close-window-alt", "Close current window").with_alt());
        self.shortcuts.push(Shortcut::new("F10", "toggle-maximize", "Toggle maximize"));
        self.shortcuts.push(Shortcut::new("F11", "toggle-fullscreen", "Toggle fullscreen"));

        // Screenshots
        self.shortcuts.push(Shortcut::new("Print", "screenshot", "Take screenshot"));
        self.shortcuts.push(Shortcut::new("Print", "screenshot-area", "Screenshot area").with_shift().with_super());

        // Lock
        self.shortcuts.push(Shortcut::new("l", "lock-screen", "Lock screen").with_super());
    }

    /// Register a custom shortcut.
    pub fn register(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);
    }

    /// Find all shortcuts for a given action.
    pub fn find_by_action(&self, action: &str) -> Vec<&Shortcut> {
        self.shortcuts.iter().filter(|s| s.action == action).collect()
    }

    /// Get all registered shortcuts.
    pub fn all(&self) -> &[Shortcut] {
        &self.shortcuts
    }
}

impl Default for ShortcutRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessibility_manager_defaults() {
        let mgr = AccessibilityManager::new();
        assert!(!mgr.high_contrast());
        assert!(!mgr.large_text());
        assert!(!mgr.reduced_motion());
        assert!((mgr.text_scale() - 1.0).abs() < 1e-6);
        assert_eq!(mgr.focus_ring_width(), 2);
    }

    #[test]
    fn test_toggle_features() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_high_contrast(true);
        assert!(mgr.high_contrast());
        mgr.set_large_text(true);
        assert!(mgr.large_text());
        assert!((mgr.text_scale() - 1.25).abs() < 1e-6);
        mgr.set_reduced_motion(true);
        assert!(mgr.reduced_motion());
    }

    #[test]
    fn test_focus_manager() {
        let mut fm = FocusManager::new();
        fm.register("button1");
        fm.register("button2");
        fm.register("button3");
        assert!(fm.current().is_none());

        fm.focus("button1");
        assert_eq!(fm.current(), Some("button1"));
        assert_eq!(fm.focus_next(), Some("button2"));
        assert_eq!(fm.focus_next(), Some("button3"));
        assert_eq!(fm.focus_next(), Some("button1")); // wraps
        assert_eq!(fm.focus_prev(), Some("button3")); // wraps back
    }

    #[test]
    fn test_focus_back() {
        let mut fm = FocusManager::new();
        fm.register("a");
        fm.register("b");
        fm.focus("a");
        fm.focus("b");
        assert_eq!(fm.focus_back(), Some("a"));
    }

    #[test]
    fn test_focus_remove() {
        let mut fm = FocusManager::new();
        fm.register("widget");
        fm.focus("widget");
        fm.remove("widget");
        assert!(fm.current().is_none());
    }

    #[test]
    fn test_shortcut_registry() {
        let reg = ShortcutRegistry::new();
        let close = reg.find_by_action("close-window");
        assert!(!close.is_empty());
        assert_eq!(close[0].key, "w");
        assert!(close[0].alt);
    }

    #[test]
    fn test_shortcut_builder() {
        let s = Shortcut::new("q", "quit", "Quit")
            .with_ctrl()
            .with_alt();
        assert_eq!(s.key, "q");
        assert!(s.ctrl);
        assert!(s.alt);
        assert!(!s.shift);
    }

    #[test]
    fn test_register_custom() {
        let mut reg = ShortcutRegistry::new();
        reg.register(Shortcut::new("p", "toggle-presentation", "Toggle presentation mode").with_ctrl().with_shift());
        let found = reg.find_by_action("toggle-presentation");
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_text_scale_limits() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_text_scale(10.0);
        assert!((mgr.text_scale() - 2.0).abs() < 1e-6);
        mgr.set_text_scale(0.1);
        assert!((mgr.text_scale() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_focus_ring_width_limits() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_focus_ring_width(-1);
        assert_eq!(mgr.focus_ring_width(), 1);
        mgr.set_focus_ring_width(100);
        assert_eq!(mgr.focus_ring_width(), 8);
    }

    #[test]
    fn test_focus_clear() {
        let mut fm = FocusManager::new();
        fm.register("a");
        fm.focus("a");
        fm.clear();
        assert!(fm.current().is_none());
        assert!(fm.focus_back().is_none());
    }

    #[test]
    fn test_shortcut_find_multiple() {
        let reg = ShortcutRegistry::new();
        let overview = reg.find_by_action("toggle-overview");
        assert!(overview.len() >= 2); // both Super+Tab and Ctrl+Alt+Tab
    }

    #[test]
    fn test_screen_reader_announce() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_screen_reader(true);
        assert!(mgr.screen_reader_active());
        mgr.announce("Test announcement");
        // Should not panic
    }
}
