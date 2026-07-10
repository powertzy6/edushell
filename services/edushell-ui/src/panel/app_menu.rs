// SPDX-License-Identifier: GPL-3.0-or-later

//! # Application Menu
//!
//! Panel button that opens the application launcher.
//! Displays the EduShell logo and "Applications" label.

use crate::localization::LocalizationManager;

/// A panel button that opens the application launcher menu.
pub struct AppMenuButton {
    label: String,
    localization: LocalizationManager,
    #[cfg(feature = "gtk")]
    button: Option<gtk::MenuButton>,
}

impl AppMenuButton {
    /// Create a new application menu button.
    pub fn new(localization: LocalizationManager) -> Self {
        let label = localization.translate("app_menu");
        Self {
            label,
            localization,
            #[cfg(feature = "gtk")]
            button: None,
        }
    }

    /// Build the GTK widget for this button.
    #[cfg(feature = "gtk")]
    pub fn build(&self) {
        let menu_button = gtk::MenuButton::new();
        menu_button.set_label(&self.label);
        menu_button.add_css_class("app-menu-button");
        // In a full implementation, a PopoverMenu would be attached.
        menu_button.set_has_frame(false);
    }

    /// Build stub for non-GTK mode.
    #[cfg(not(feature = "gtk"))]
    pub fn build(&self) {
        tracing::info!(target: "edushell::ui::panel::app_menu", "App menu button stub: label={}", self.label);
    }

    /// Get the button label text.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Set the button label text.
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
        #[cfg(feature = "gtk")]
        if let Some(button) = &self.button {
            button.set_label(&self.label);
        }
    }
}

impl std::fmt::Debug for AppMenuButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppMenuButton")
            .field("label", &self.label)
            .finish_non_exhaustive()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_button() -> AppMenuButton {
        AppMenuButton::new(LocalizationManager::new())
    }

    #[test]
    fn test_new_button_default_label() {
        let btn = make_button();
        assert_eq!(btn.label(), "Applications");
    }

    #[test]
    fn test_localized_label() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("id-ID");
        let btn = AppMenuButton::new(lm);
        assert_eq!(btn.label(), "Aplikasi");
    }

    #[test]
    fn test_set_label() {
        let mut btn = make_button();
        btn.set_label("Custom");
        assert_eq!(btn.label(), "Custom");
    }

    #[test]
    fn test_set_label_updates() {
        let mut btn = make_button();
        btn.set_label("Menu");
        assert_eq!(btn.label(), "Menu");
        btn.set_label("Apps");
        assert_eq!(btn.label(), "Apps");
    }

    #[test]
    fn test_build_does_not_panic() {
        let btn = make_button();
        btn.build();
    }

    #[test]
    fn test_debug() {
        let btn = make_button();
        let d = format!("{btn:?}");
        assert!(d.contains("AppMenuButton"));
        assert!(d.contains("Applications"));
    }

    #[test]
    fn test_new_with_french_locale() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("fr-FR"); // French uses same keys as en-US fallback
        let btn = AppMenuButton::new(lm);
        assert_eq!(btn.label(), "Applications");
    }

    #[test]
    fn test_clone_localization() {
        let lm = LocalizationManager::new();
        let btn = AppMenuButton::new(lm.clone());
        assert_eq!(btn.label(), lm.translate("app_menu"));
    }

    #[test]
    fn test_empty_label() {
        let mut btn = make_button();
        btn.set_label("");
        assert_eq!(btn.label(), "");
    }

    #[test]
    fn test_label_after_multiple_sets() {
        let mut btn = make_button();
        btn.set_label("First");
        btn.set_label("Second");
        btn.set_label("Third");
        assert_eq!(btn.label(), "Third");
    }

    #[test]
    fn test_es_locale_fallback() {
        let mut lm = LocalizationManager::new();
        lm.set_locale("es-ES");
        let btn = AppMenuButton::new(lm);
        assert_eq!(btn.label(), "Applications");
    }

    #[test]
    fn test_build_called_twice() {
        let btn = make_button();
        btn.build();
        btn.build();
    }
}
