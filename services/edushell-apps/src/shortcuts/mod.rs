use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutAction {
    OpenLauncher,
    OpenSearch,
    OpenTerminal,
    OpenLearningHub,
    OpenProjectHub,
    OpenOfficeHub,
    OpenBrowserHub,
    OpenSettings,
    OpenFileManager,
    NextWorkspace,
    PrevWorkspace,
    SwitchWorkspace(u32),
    Screenshot,
    ScreenshotArea,
    ScreenshotWindow,
    ToggleNotificationCenter,
    ToggleQuickSettings,
    LockScreen,
    LogOut,
    Shutdown,
    Restart,
    OpenWelcome,
    OpenSoftwareCenter,
    FocusSearch,
    GlobalSearch,
    ToggleDarkMode,
    IncreaseVolume,
    DecreaseVolume,
    Mute,
    IncreaseBrightness,
    DecreaseBrightness,
    ToggleAccessibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutCategory {
    Launcher,
    Desktop,
    Workspace,
    Screenshot,
    System,
    Accessibility,
    Media,
    Applications,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutBinding {
    pub action: ShortcutAction,
    pub keys: Vec<String>,
    pub description_key: &'static str,
    pub category: ShortcutCategory,
}

pub struct ShortcutManager {
    bindings: Vec<ShortcutBinding>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            bindings: Self::default_shortcuts(),
        }
    }

    pub fn default_shortcuts() -> Vec<ShortcutBinding> {
        vec![
            ShortcutBinding {
                action: ShortcutAction::OpenLauncher,
                keys: vec!["<Super>".into(), "Space".into()],
                description_key: "shortcuts.open_launcher",
                category: ShortcutCategory::Launcher,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenSearch,
                keys: vec!["<Super>".into(), "Slash".into()],
                description_key: "shortcuts.open_search",
                category: ShortcutCategory::Launcher,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenTerminal,
                keys: vec!["<Super>".into(), "T".into()],
                description_key: "shortcuts.open_terminal",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenLearningHub,
                keys: vec!["<Super>".into(), "L".into()],
                description_key: "shortcuts.open_learning",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenProjectHub,
                keys: vec!["<Super>".into(), "P".into()],
                description_key: "shortcuts.open_project",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenOfficeHub,
                keys: vec!["<Super>".into(), "O".into()],
                description_key: "shortcuts.open_office",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenBrowserHub,
                keys: vec!["<Super>".into(), "B".into()],
                description_key: "shortcuts.open_browser",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenSettings,
                keys: vec!["<Super>".into(), "S".into()],
                description_key: "shortcuts.open_settings",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenFileManager,
                keys: vec!["<Super>".into(), "E".into()],
                description_key: "shortcuts.open_files",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenWelcome,
                keys: vec!["<Super>".into(), "W".into()],
                description_key: "shortcuts.open_welcome",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::NextWorkspace,
                keys: vec!["<Super>".into(), "Tab".into()],
                description_key: "shortcuts.next_workspace",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::PrevWorkspace,
                keys: vec!["<Super>".into(), "<Shift>".into(), "Tab".into()],
                description_key: "shortcuts.prev_workspace",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(1),
                keys: vec!["<Super>".into(), "1".into()],
                description_key: "shortcuts.switch_workspace_1",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(2),
                keys: vec!["<Super>".into(), "2".into()],
                description_key: "shortcuts.switch_workspace_2",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(3),
                keys: vec!["<Super>".into(), "3".into()],
                description_key: "shortcuts.switch_workspace_3",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(4),
                keys: vec!["<Super>".into(), "4".into()],
                description_key: "shortcuts.switch_workspace_4",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(5),
                keys: vec!["<Super>".into(), "5".into()],
                description_key: "shortcuts.switch_workspace_5",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(6),
                keys: vec!["<Super>".into(), "6".into()],
                description_key: "shortcuts.switch_workspace_6",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(7),
                keys: vec!["<Super>".into(), "7".into()],
                description_key: "shortcuts.switch_workspace_7",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(8),
                keys: vec!["<Super>".into(), "8".into()],
                description_key: "shortcuts.switch_workspace_8",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::SwitchWorkspace(9),
                keys: vec!["<Super>".into(), "9".into()],
                description_key: "shortcuts.switch_workspace_9",
                category: ShortcutCategory::Workspace,
            },
            ShortcutBinding {
                action: ShortcutAction::Screenshot,
                keys: vec!["Print".into()],
                description_key: "shortcuts.screenshot_full",
                category: ShortcutCategory::Screenshot,
            },
            ShortcutBinding {
                action: ShortcutAction::ScreenshotArea,
                keys: vec!["<Shift>".into(), "Print".into()],
                description_key: "shortcuts.screenshot_area",
                category: ShortcutCategory::Screenshot,
            },
            ShortcutBinding {
                action: ShortcutAction::ScreenshotWindow,
                keys: vec!["<Alt>".into(), "Print".into()],
                description_key: "shortcuts.screenshot_window",
                category: ShortcutCategory::Screenshot,
            },
            ShortcutBinding {
                action: ShortcutAction::ToggleNotificationCenter,
                keys: vec!["<Super>".into(), "N".into()],
                description_key: "shortcuts.toggle_notifications",
                category: ShortcutCategory::Desktop,
            },
            ShortcutBinding {
                action: ShortcutAction::ToggleQuickSettings,
                keys: vec!["<Super>".into(), "A".into()],
                description_key: "shortcuts.toggle_quick_settings",
                category: ShortcutCategory::Desktop,
            },
            ShortcutBinding {
                action: ShortcutAction::LockScreen,
                keys: vec!["<Super>".into(), "<Escape>".into()],
                description_key: "shortcuts.lock_screen",
                category: ShortcutCategory::System,
            },
            ShortcutBinding {
                action: ShortcutAction::OpenTerminal,
                keys: vec!["<Primary>".into(), "<Alt>".into(), "T".into()],
                description_key: "shortcuts.open_terminal",
                category: ShortcutCategory::Applications,
            },
            ShortcutBinding {
                action: ShortcutAction::FocusSearch,
                keys: vec!["<Super>".into(), "F".into()],
                description_key: "shortcuts.focus_search",
                category: ShortcutCategory::Launcher,
            },
            ShortcutBinding {
                action: ShortcutAction::GlobalSearch,
                keys: vec!["<Primary>".into(), "<Shift>".into(), "F".into()],
                description_key: "shortcuts.global_search",
                category: ShortcutCategory::Launcher,
            },
            ShortcutBinding {
                action: ShortcutAction::ToggleDarkMode,
                keys: vec!["<Super>".into(), "D".into()],
                description_key: "shortcuts.toggle_dark_mode",
                category: ShortcutCategory::Desktop,
            },
            ShortcutBinding {
                action: ShortcutAction::IncreaseVolume,
                keys: vec!["AudioRaiseVolume".into()],
                description_key: "shortcuts.increase_volume",
                category: ShortcutCategory::Media,
            },
            ShortcutBinding {
                action: ShortcutAction::DecreaseVolume,
                keys: vec!["AudioLowerVolume".into()],
                description_key: "shortcuts.decrease_volume",
                category: ShortcutCategory::Media,
            },
            ShortcutBinding {
                action: ShortcutAction::Mute,
                keys: vec!["AudioMute".into()],
                description_key: "shortcuts.mute",
                category: ShortcutCategory::Media,
            },
            ShortcutBinding {
                action: ShortcutAction::IncreaseBrightness,
                keys: vec!["MonBrightnessUp".into()],
                description_key: "shortcuts.increase_brightness",
                category: ShortcutCategory::Media,
            },
            ShortcutBinding {
                action: ShortcutAction::DecreaseBrightness,
                keys: vec!["MonBrightnessDown".into()],
                description_key: "shortcuts.decrease_brightness",
                category: ShortcutCategory::Media,
            },
            ShortcutBinding {
                action: ShortcutAction::ToggleAccessibility,
                keys: vec!["<Super>".into(), "<Alt>".into(), "S".into()],
                description_key: "shortcuts.accessibility_toggle",
                category: ShortcutCategory::Accessibility,
            },
        ]
    }

    pub fn bindings(&self) -> &[ShortcutBinding] {
        &self.bindings
    }

    pub fn bindings_mut(&mut self) -> &mut Vec<ShortcutBinding> {
        &mut self.bindings
    }

    pub fn get(&self, action: &ShortcutAction) -> Option<&ShortcutBinding> {
        self.bindings.iter().find(|b| &b.action == action)
    }

    pub fn set_keys(&mut self, action: &ShortcutAction, keys: Vec<String>) -> bool {
        if let Some(binding) = self.bindings.iter_mut().find(|b| &b.action == action) {
            binding.keys = keys;
            true
        } else {
            false
        }
    }

    pub fn reset_all(&mut self) {
        self.bindings = Self::default_shortcuts();
    }

    pub fn reset_action(&mut self, action: &ShortcutAction) {
        let defaults = Self::default_shortcuts();
        if let Some(default) = defaults.into_iter().find(|b| &b.action == action) {
            if let Some(binding) = self.bindings.iter_mut().find(|b| &b.action == action) {
                binding.keys = default.keys;
                binding.description_key = default.description_key;
                binding.category = default.category;
            }
        }
    }

    pub fn find_by_keys(&self, keys: &[String]) -> Option<&ShortcutBinding> {
        self.bindings.iter().find(|b| b.keys == keys)
    }

    pub fn export_to_string(&self) -> String {
        let mut result = String::new();
        for binding in &self.bindings {
            let action = format!("{:?}", binding.action);
            let keys: Vec<String> = binding.keys.iter().map(|k| k.to_string()).collect();
            let keys_str = keys.join("+");
            result.push_str(&format!(
                "{}={};{}\n",
                action, keys_str, binding.description_key
            ));
        }
        result
    }

    pub fn import_from_string(&mut self, data: &str) -> Result<(), String> {
        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(format!("invalid line: {}", line));
            }
            let rest: Vec<&str> = parts[1].splitn(2, ';').collect();
            if rest.len() != 2 {
                return Err(format!("invalid line: {}", line));
            }
            let keys_str = rest[0];
            let keys: Vec<String> = keys_str.split('+').map(|s| s.to_string()).collect();
            let found = self
                .bindings
                .iter_mut()
                .find(|b| format!("{:?}", b.action) == parts[0]);
            if let Some(binding) = found {
                binding.keys = keys;
            }
        }
        Ok(())
    }

    pub fn categories(&self) -> Vec<ShortcutCategory> {
        vec![
            ShortcutCategory::Launcher,
            ShortcutCategory::Desktop,
            ShortcutCategory::Workspace,
            ShortcutCategory::Screenshot,
            ShortcutCategory::System,
            ShortcutCategory::Accessibility,
            ShortcutCategory::Media,
            ShortcutCategory::Applications,
        ]
    }

    pub fn bindings_by_category(&self, cat: ShortcutCategory) -> Vec<&ShortcutBinding> {
        self.bindings.iter().filter(|b| b.category == cat).collect()
    }

    pub fn conflicts_with(&self, keys: &[String]) -> Vec<&ShortcutBinding> {
        self.bindings.iter().filter(|b| b.keys == keys).collect()
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_shortcuts_created() {
        let mgr = ShortcutManager::new();
        assert!(!mgr.bindings().is_empty());
        assert!(mgr.bindings().len() >= 30);
    }

    #[test]
    fn test_lookup_by_action() {
        let mgr = ShortcutManager::new();
        let binding = mgr.get(&ShortcutAction::OpenLauncher);
        assert!(binding.is_some());
        assert_eq!(binding.unwrap().keys, vec!["<Super>", "Space"]);
    }

    #[test]
    fn test_lookup_nonexistent() {
        let mgr = ShortcutManager::new();
        let not_found = mgr.get(&ShortcutAction::LogOut);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_reset_action() {
        let mut mgr = ShortcutManager::new();
        mgr.set_keys(
            &ShortcutAction::OpenLauncher,
            vec!["<Super>".into(), "L".into()],
        );
        assert_eq!(
            mgr.get(&ShortcutAction::OpenLauncher).unwrap().keys,
            vec!["<Super>", "L"]
        );
        mgr.reset_action(&ShortcutAction::OpenLauncher);
        assert_eq!(
            mgr.get(&ShortcutAction::OpenLauncher).unwrap().keys,
            vec!["<Super>", "Space"]
        );
    }

    #[test]
    fn test_reset_all() {
        let mut mgr = ShortcutManager::new();
        mgr.set_keys(&ShortcutAction::OpenLauncher, vec!["X".into()]);
        mgr.set_keys(&ShortcutAction::OpenTerminal, vec!["Y".into()]);
        mgr.reset_all();
        assert_eq!(
            mgr.get(&ShortcutAction::OpenLauncher).unwrap().keys,
            vec!["<Super>", "Space"]
        );
        assert_eq!(
            mgr.get(&ShortcutAction::OpenTerminal).unwrap().keys,
            vec!["<Super>", "T"]
        );
    }

    #[test]
    fn test_conflicts() {
        let mgr = ShortcutManager::new();
        let conflicts = mgr.conflicts_with(&[">Super<".to_string(), "Space".to_string()]);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_export_import_roundtrip() {
        let mgr = ShortcutManager::new();
        let exported = mgr.export_to_string();
        assert!(!exported.is_empty());
        let mut other = ShortcutManager::new();
        other.set_keys(&ShortcutAction::OpenLauncher, vec!["X".into(), "Y".into()]);
        let result = other.import_from_string(&exported);
        assert!(result.is_ok());
        assert_eq!(
            other.get(&ShortcutAction::OpenLauncher).unwrap().keys,
            vec!["<Super>", "Space"]
        );
    }

    #[test]
    fn test_import_invalid_line() {
        let mut mgr = ShortcutManager::new();
        let result = mgr.import_from_string("invalid_line_no_equals");
        assert!(result.is_err());
    }

    #[test]
    fn test_all_categories_non_empty() {
        let mgr = ShortcutManager::new();
        for cat in mgr.categories() {
            let bindings = mgr.bindings_by_category(cat);
            assert!(!bindings.is_empty(), "category {:?} has no bindings", cat);
        }
    }

    #[test]
    fn test_no_duplicate_default_keybindings() {
        let mgr = ShortcutManager::new();
        let mut seen = std::collections::HashSet::new();
        for binding in mgr.bindings() {
            let key_str = binding.keys.join("+");
            assert!(
                seen.insert(key_str.clone()),
                "duplicate keybinding found: {}",
                key_str
            );
        }
    }

    #[test]
    fn test_set_keys() {
        let mut mgr = ShortcutManager::new();
        let result = mgr.set_keys(
            &ShortcutAction::OpenTerminal,
            vec!["<Super>".into(), "X".into()],
        );
        assert!(result);
        assert_eq!(
            mgr.get(&ShortcutAction::OpenTerminal).unwrap().keys,
            vec!["<Super>", "X"]
        );
    }

    #[test]
    fn test_set_keys_nonexistent_action() {
        let mut mgr = ShortcutManager::new();
        let result = mgr.set_keys(&ShortcutAction::LogOut, vec!["X".into()]);
        assert!(!result);
    }

    #[test]
    fn test_find_by_keys() {
        let mgr = ShortcutManager::new();
        let binding = mgr.find_by_keys(&["<Super>".to_string(), "Space".to_string()]);
        assert!(binding.is_some());
        assert_eq!(binding.unwrap().action, ShortcutAction::OpenLauncher);
    }

    #[test]
    fn test_find_by_keys_not_found() {
        let mgr = ShortcutManager::new();
        let binding = mgr.find_by_keys(&["Nonexistent".to_string()]);
        assert!(binding.is_none());
    }

    #[test]
    fn test_bindings_by_category() {
        let mgr = ShortcutManager::new();
        let workspace_bindings = mgr.bindings_by_category(ShortcutCategory::Workspace);
        assert!(!workspace_bindings.is_empty());
        for b in &workspace_bindings {
            assert_eq!(b.category, ShortcutCategory::Workspace);
        }
    }

    #[test]
    fn test_bindings_mut() {
        let mut mgr = ShortcutManager::new();
        mgr.bindings_mut().push(ShortcutBinding {
            action: ShortcutAction::LogOut,
            keys: vec!["<Super>".into(), "Q".into()],
            description_key: "shortcuts.logout",
            category: ShortcutCategory::System,
        });
        assert!(mgr.get(&ShortcutAction::LogOut).is_some());
    }

    #[test]
    fn test_reset_action_nonexistent() {
        let mut mgr = ShortcutManager::new();
        mgr.reset_action(&ShortcutAction::LogOut);
    }

    #[test]
    fn test_switch_workspace_bindings() {
        let mgr = ShortcutManager::new();
        for i in 1..=9 {
            let binding = mgr.get(&ShortcutAction::SwitchWorkspace(i));
            assert!(binding.is_some(), "SwitchWorkspace({}) not found", i);
            assert_eq!(binding.unwrap().keys, vec!["<Super>", &i.to_string()]);
        }
    }

    #[test]
    fn test_screenshot_shortcuts() {
        let mgr = ShortcutManager::new();
        assert!(mgr.get(&ShortcutAction::Screenshot).is_some());
        assert!(mgr.get(&ShortcutAction::ScreenshotArea).is_some());
        assert!(mgr.get(&ShortcutAction::ScreenshotWindow).is_some());
    }

    #[test]
    fn test_media_shortcuts() {
        let mgr = ShortcutManager::new();
        assert!(mgr.get(&ShortcutAction::IncreaseVolume).is_some());
        assert!(mgr.get(&ShortcutAction::DecreaseVolume).is_some());
        assert!(mgr.get(&ShortcutAction::Mute).is_some());
        assert!(mgr.get(&ShortcutAction::IncreaseBrightness).is_some());
        assert!(mgr.get(&ShortcutAction::DecreaseBrightness).is_some());
    }
}
