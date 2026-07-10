use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::localization::*;

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
static HOME_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WelcomeStep {
    Welcome,
    Language,
    Theme,
    Purpose,
    InstallPackages,
    DesktopShortcuts,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserPurpose {
    School,
    Coding,
    Office,
    Multimedia,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: String,
    pub theme_mode: String,
    pub accent_color: String,
    pub purpose: UserPurpose,
    pub install_optional_packages: Vec<String>,
    pub create_desktop_shortcuts: Vec<String>,
    pub completed: bool,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalPackage {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
    pub category: String,
    pub size_mb: u32,
    pub default_selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutOption {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
    pub default_selected: bool,
}

pub struct WelcomeApp {
    current_step: WelcomeStep,
    preferences: UserPreferences,
    localization: LocalizationManager,
    available_packages: Vec<OptionalPackage>,
    available_shortcuts: Vec<ShortcutOption>,
    is_first_run: bool,
    config_dir: PathBuf,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "auto".to_string(),
            theme_mode: "light".to_string(),
            accent_color: "#4A90D9".to_string(),
            purpose: UserPurpose::School,
            install_optional_packages: Vec::new(),
            create_desktop_shortcuts: Vec::new(),
            completed: false,
            completed_at: None,
        }
    }
}

impl WelcomeApp {
    pub fn new() -> Self {
        let is_first_run = Self::check_first_run();
        let config_dir = Self::config_dir();
        let mut app = Self {
            current_step: WelcomeStep::Welcome,
            preferences: UserPreferences::default(),
            localization: LocalizationManager::new(),
            available_packages: Self::default_packages(),
            available_shortcuts: Self::default_shortcuts(),
            is_first_run,
            config_dir,
        };
        let detected = LocalizationManager::detect();
        app.localization.set_language(detected);
        app.apply_default_selections();
        if !is_first_run {
            let _ = app.load_preferences();
        }
        app
    }

    fn config_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join(".config/edushell")
    }

    fn apply_default_selections(&mut self) {
        for pkg in &self.available_packages {
            if pkg.default_selected {
                self.preferences.install_optional_packages.push(pkg.id.clone());
            }
        }
        for sc in &self.available_shortcuts {
            if sc.default_selected {
                self.preferences.create_desktop_shortcuts.push(sc.id.clone());
            }
        }
    }

    pub fn check_first_run() -> bool {
        !Self::config_dir().join("welcome-completed").exists()
    }

    pub fn mark_completed(&mut self) -> std::io::Result<()> {
        self.preferences.completed = true;
        self.preferences.completed_at = Some(chrono::Local::now().to_rfc3339());
        std::fs::create_dir_all(&self.config_dir)?;
        let json = serde_json::to_string_pretty(&self.preferences)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(self.config_dir.join("welcome-preferences.json"), &json)?;
        std::fs::write(self.config_dir.join("welcome-completed"), "")?;
        self.is_first_run = false;
        Ok(())
    }

    pub fn load_preferences(&mut self) -> std::io::Result<()> {
        let path = self.config_dir.join("welcome-preferences.json");
        if path.exists() {
            let data = std::fs::read_to_string(&path)?;
            let prefs: UserPreferences = serde_json::from_str(&data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            self.preferences = prefs;
        }
        Ok(())
    }

    pub fn current_step(&self) -> WelcomeStep {
        self.current_step
    }

    pub fn total_steps() -> u8 {
        7
    }

    pub fn current_step_number(&self) -> u8 {
        match self.current_step {
            WelcomeStep::Welcome => 1,
            WelcomeStep::Language => 2,
            WelcomeStep::Theme => 3,
            WelcomeStep::Purpose => 4,
            WelcomeStep::InstallPackages => 5,
            WelcomeStep::DesktopShortcuts => 6,
            WelcomeStep::Complete => 7,
        }
    }

    pub fn next_step(&mut self) -> bool {
        match self.current_step {
            WelcomeStep::Welcome => self.current_step = WelcomeStep::Language,
            WelcomeStep::Language => self.current_step = WelcomeStep::Theme,
            WelcomeStep::Theme => self.current_step = WelcomeStep::Purpose,
            WelcomeStep::Purpose => self.current_step = WelcomeStep::InstallPackages,
            WelcomeStep::InstallPackages => self.current_step = WelcomeStep::DesktopShortcuts,
            WelcomeStep::DesktopShortcuts => self.current_step = WelcomeStep::Complete,
            WelcomeStep::Complete => return false,
        }
        true
    }

    pub fn prev_step(&mut self) -> bool {
        match self.current_step {
            WelcomeStep::Welcome => return false,
            WelcomeStep::Language => self.current_step = WelcomeStep::Welcome,
            WelcomeStep::Theme => self.current_step = WelcomeStep::Language,
            WelcomeStep::Purpose => self.current_step = WelcomeStep::Theme,
            WelcomeStep::InstallPackages => self.current_step = WelcomeStep::Purpose,
            WelcomeStep::DesktopShortcuts => self.current_step = WelcomeStep::InstallPackages,
            WelcomeStep::Complete => self.current_step = WelcomeStep::DesktopShortcuts,
        }
        true
    }

    pub fn go_to_step(&mut self, step: WelcomeStep) {
        self.current_step = step;
    }

    pub fn is_first_step(&self) -> bool {
        self.current_step == WelcomeStep::Welcome
    }

    pub fn is_last_step(&self) -> bool {
        self.current_step == WelcomeStep::Complete
    }

    pub fn progress_percentage(&self) -> f64 {
        (self.current_step_number() as f64 - 1.0) / (Self::total_steps() as f64 - 1.0)
    }

    pub fn set_language(&mut self, lang: &str) {
        self.preferences.language = lang.to_string();
        match lang {
            "id-ID" => self.localization.set_language(Lang::Indonesian),
            "en-US" => self.localization.set_language(Lang::English),
            _ => {
                let detected = LocalizationManager::detect();
                self.localization.set_language(detected);
            }
        }
    }

    pub fn set_theme_mode(&mut self, mode: &str) {
        self.preferences.theme_mode = mode.to_string();
    }

    pub fn set_accent_color(&mut self, color: &str) {
        self.preferences.accent_color = color.to_string();
    }

    pub fn set_purpose(&mut self, purpose: UserPurpose) {
        self.preferences.purpose = purpose;
    }

    pub fn toggle_package(&mut self, id: &str) {
        let id_str = id.to_string();
        if let Some(pos) = self.preferences.install_optional_packages.iter().position(|p| *p == id_str) {
            self.preferences.install_optional_packages.remove(pos);
        } else {
            self.preferences.install_optional_packages.push(id_str);
        }
    }

    pub fn toggle_shortcut(&mut self, id: &str) {
        let id_str = id.to_string();
        if let Some(pos) = self.preferences.create_desktop_shortcuts.iter().position(|s| *s == id_str) {
            self.preferences.create_desktop_shortcuts.remove(pos);
        } else {
            self.preferences.create_desktop_shortcuts.push(id_str);
        }
    }

    pub fn select_all_packages(&mut self) {
        self.preferences.install_optional_packages = self.available_packages.iter().map(|p| p.id.clone()).collect();
    }

    pub fn deselect_all_packages(&mut self) {
        self.preferences.install_optional_packages.clear();
    }

    pub fn select_all_shortcuts(&mut self) {
        self.preferences.create_desktop_shortcuts = self.available_shortcuts.iter().map(|s| s.id.clone()).collect();
    }

    pub fn deselect_all_shortcuts(&mut self) {
        self.preferences.create_desktop_shortcuts.clear();
    }

    pub fn preferences(&self) -> &UserPreferences {
        &self.preferences
    }

    pub fn available_packages(&self) -> &[OptionalPackage] {
        &self.available_packages
    }

    pub fn available_shortcuts(&self) -> &[ShortcutOption] {
        &self.available_shortcuts
    }

    pub fn selected_packages(&self) -> Vec<&OptionalPackage> {
        self.available_packages.iter()
            .filter(|p| self.preferences.install_optional_packages.contains(&p.id))
            .collect()
    }

    pub fn selected_shortcuts(&self) -> Vec<&ShortcutOption> {
        self.available_shortcuts.iter()
            .filter(|s| self.preferences.create_desktop_shortcuts.contains(&s.id))
            .collect()
    }

    pub fn step_title(&self) -> String {
        match self.current_step {
            WelcomeStep::Welcome => self.localization.get("welcome.step.welcome.title").to_string(),
            WelcomeStep::Language => self.localization.get("welcome.step.language.title").to_string(),
            WelcomeStep::Theme => self.localization.get("welcome.step.theme.title").to_string(),
            WelcomeStep::Purpose => self.localization.get("welcome.step.purpose.title").to_string(),
            WelcomeStep::InstallPackages => self.localization.get("welcome.step.install.title").to_string(),
            WelcomeStep::DesktopShortcuts => self.localization.get("welcome.step.shortcuts.title").to_string(),
            WelcomeStep::Complete => self.localization.get("welcome.step.complete.title").to_string(),
        }
    }

    pub fn step_description(&self) -> String {
        match self.current_step {
            WelcomeStep::Welcome => self.localization.get("welcome.step.welcome.description").to_string(),
            WelcomeStep::Language => self.localization.get("welcome.step.language.description").to_string(),
            WelcomeStep::Theme => self.localization.get("welcome.step.theme.description").to_string(),
            WelcomeStep::Purpose => self.localization.get("welcome.step.purpose.description").to_string(),
            WelcomeStep::InstallPackages => self.localization.get("welcome.step.install.description").to_string(),
            WelcomeStep::DesktopShortcuts => self.localization.get("welcome.step.shortcuts.description").to_string(),
            WelcomeStep::Complete => self.localization.get("welcome.step.complete.description").to_string(),
        }
    }

    pub fn can_proceed(&self) -> bool {
        true
    }

    pub fn get_packages_for_purpose(purpose: UserPurpose) -> Vec<&'static str> {
        match purpose {
            UserPurpose::School => vec!["libreoffice", "gimp", "inkscape", "scratch", "geogebra", "kalzium", "marble", "kstars"],
            UserPurpose::Coding => vec!["code", "git", "nodejs", "python3", "gcc", "rustc", "docker", "postman", "neovim", "vim"],
            UserPurpose::Office => vec!["libreoffice", "thunderbird", "evolution", "pdfarranger", "ocrmypdf", "xournalpp"],
            UserPurpose::Multimedia => vec!["gimp", "inkscape", "blender", "audacity", "kdenlive", "obs-studio", "vlc", "krita"],
            UserPurpose::Custom => vec![],
        }
    }

    pub fn get_shortcuts_for_purpose(purpose: UserPurpose) -> Vec<&'static str> {
        match purpose {
            UserPurpose::School => vec!["edushell-learning", "edushell-terminal", "scratch", "geogebra"],
            UserPurpose::Coding => vec!["code", "edushell-terminal", "edushell-project", "git"],
            UserPurpose::Office => vec!["libreoffice-writer", "libreoffice-calc", "libreoffice-impress", "edushell-office"],
            UserPurpose::Multimedia => vec!["gimp", "blender", "audacity", "vlc"],
            UserPurpose::Custom => vec![],
        }
    }

    pub fn reset(&mut self) {
        self.current_step = WelcomeStep::Welcome;
        self.preferences = UserPreferences::default();
        self.is_first_run = true;
        self.apply_default_selections();
    }

    pub fn is_completed(&self) -> bool {
        self.preferences.completed
    }

    pub fn completion_summary(&self) -> String {
        format!(
            "{}: {}, {}: {}, {}: {:?}, {}: {}, {}: {}",
            self.localization.get("common.language"),
            self.preferences.language,
            self.localization.get("theme.title"),
            self.preferences.theme_mode,
            self.localization.get("welcome.step.purpose.title"),
            self.preferences.purpose,
            self.localization.get("software.installed"),
            self.preferences.install_optional_packages.len(),
            self.localization.get("shortcuts.title"),
            self.preferences.create_desktop_shortcuts.len(),
        )
    }

    #[cfg(feature = "gtk")]
    pub fn build_window(&self) {
        let _display = gtk4::gdk::Display::default();
    }

    #[cfg(not(feature = "gtk"))]
    pub fn build_window(&self) {}

    fn default_packages() -> Vec<OptionalPackage> {
        vec![
            OptionalPackage {
                id: "libreoffice".to_string(),
                name_key: "package.libreoffice.name".to_string(),
                description_key: "package.libreoffice.desc".to_string(),
                category: "office".to_string(),
                size_mb: 800,
                default_selected: true,
            },
            OptionalPackage {
                id: "gimp".to_string(),
                name_key: "package.gimp.name".to_string(),
                description_key: "package.gimp.desc".to_string(),
                category: "graphics".to_string(),
                size_mb: 200,
                default_selected: false,
            },
            OptionalPackage {
                id: "inkscape".to_string(),
                name_key: "package.inkscape.name".to_string(),
                description_key: "package.inkscape.desc".to_string(),
                category: "graphics".to_string(),
                size_mb: 250,
                default_selected: false,
            },
            OptionalPackage {
                id: "scratch".to_string(),
                name_key: "package.scratch.name".to_string(),
                description_key: "package.scratch.desc".to_string(),
                category: "education".to_string(),
                size_mb: 150,
                default_selected: true,
            },
            OptionalPackage {
                id: "geogebra".to_string(),
                name_key: "package.geogebra.name".to_string(),
                description_key: "package.geogebra.desc".to_string(),
                category: "education".to_string(),
                size_mb: 100,
                default_selected: true,
            },
            OptionalPackage {
                id: "kalzium".to_string(),
                name_key: "package.kalzium.name".to_string(),
                description_key: "package.kalzium.desc".to_string(),
                category: "education".to_string(),
                size_mb: 80,
                default_selected: true,
            },
            OptionalPackage {
                id: "marble".to_string(),
                name_key: "package.marble.name".to_string(),
                description_key: "package.marble.desc".to_string(),
                category: "education".to_string(),
                size_mb: 120,
                default_selected: false,
            },
            OptionalPackage {
                id: "kstars".to_string(),
                name_key: "package.kstars.name".to_string(),
                description_key: "package.kstars.desc".to_string(),
                category: "education".to_string(),
                size_mb: 200,
                default_selected: false,
            },
            OptionalPackage {
                id: "code".to_string(),
                name_key: "package.code.name".to_string(),
                description_key: "package.code.desc".to_string(),
                category: "development".to_string(),
                size_mb: 300,
                default_selected: false,
            },
            OptionalPackage {
                id: "git".to_string(),
                name_key: "package.git.name".to_string(),
                description_key: "package.git.desc".to_string(),
                category: "development".to_string(),
                size_mb: 50,
                default_selected: false,
            },
            OptionalPackage {
                id: "nodejs".to_string(),
                name_key: "package.nodejs.name".to_string(),
                description_key: "package.nodejs.desc".to_string(),
                category: "development".to_string(),
                size_mb: 80,
                default_selected: false,
            },
            OptionalPackage {
                id: "python3".to_string(),
                name_key: "package.python3.name".to_string(),
                description_key: "package.python3.desc".to_string(),
                category: "development".to_string(),
                size_mb: 60,
                default_selected: false,
            },
            OptionalPackage {
                id: "gcc".to_string(),
                name_key: "package.gcc.name".to_string(),
                description_key: "package.gcc.desc".to_string(),
                category: "development".to_string(),
                size_mb: 150,
                default_selected: false,
            },
            OptionalPackage {
                id: "rustc".to_string(),
                name_key: "package.rustc.name".to_string(),
                description_key: "package.rustc.desc".to_string(),
                category: "development".to_string(),
                size_mb: 200,
                default_selected: false,
            },
            OptionalPackage {
                id: "docker".to_string(),
                name_key: "package.docker.name".to_string(),
                description_key: "package.docker.desc".to_string(),
                category: "development".to_string(),
                size_mb: 400,
                default_selected: false,
            },
            OptionalPackage {
                id: "postman".to_string(),
                name_key: "package.postman.name".to_string(),
                description_key: "package.postman.desc".to_string(),
                category: "development".to_string(),
                size_mb: 150,
                default_selected: false,
            },
            OptionalPackage {
                id: "neovim".to_string(),
                name_key: "package.neovim.name".to_string(),
                description_key: "package.neovim.desc".to_string(),
                category: "development".to_string(),
                size_mb: 30,
                default_selected: false,
            },
            OptionalPackage {
                id: "vim".to_string(),
                name_key: "package.vim.name".to_string(),
                description_key: "package.vim.desc".to_string(),
                category: "development".to_string(),
                size_mb: 20,
                default_selected: false,
            },
            OptionalPackage {
                id: "thunderbird".to_string(),
                name_key: "package.thunderbird.name".to_string(),
                description_key: "package.thunderbird.desc".to_string(),
                category: "office".to_string(),
                size_mb: 200,
                default_selected: false,
            },
            OptionalPackage {
                id: "evolution".to_string(),
                name_key: "package.evolution.name".to_string(),
                description_key: "package.evolution.desc".to_string(),
                category: "office".to_string(),
                size_mb: 250,
                default_selected: false,
            },
            OptionalPackage {
                id: "pdfarranger".to_string(),
                name_key: "package.pdfarranger.name".to_string(),
                description_key: "package.pdfarranger.desc".to_string(),
                category: "office".to_string(),
                size_mb: 40,
                default_selected: false,
            },
            OptionalPackage {
                id: "ocrmypdf".to_string(),
                name_key: "package.ocrmypdf.name".to_string(),
                description_key: "package.ocrmypdf.desc".to_string(),
                category: "office".to_string(),
                size_mb: 100,
                default_selected: false,
            },
            OptionalPackage {
                id: "xournalpp".to_string(),
                name_key: "package.xournalpp.name".to_string(),
                description_key: "package.xournalpp.desc".to_string(),
                category: "office".to_string(),
                size_mb: 60,
                default_selected: false,
            },
            OptionalPackage {
                id: "blender".to_string(),
                name_key: "package.blender.name".to_string(),
                description_key: "package.blender.desc".to_string(),
                category: "graphics".to_string(),
                size_mb: 500,
                default_selected: false,
            },
            OptionalPackage {
                id: "audacity".to_string(),
                name_key: "package.audacity.name".to_string(),
                description_key: "package.audacity.desc".to_string(),
                category: "multimedia".to_string(),
                size_mb: 80,
                default_selected: false,
            },
            OptionalPackage {
                id: "kdenlive".to_string(),
                name_key: "package.kdenlive.name".to_string(),
                description_key: "package.kdenlive.desc".to_string(),
                category: "multimedia".to_string(),
                size_mb: 300,
                default_selected: false,
            },
            OptionalPackage {
                id: "obs-studio".to_string(),
                name_key: "package.obs-studio.name".to_string(),
                description_key: "package.obs-studio.desc".to_string(),
                category: "multimedia".to_string(),
                size_mb: 200,
                default_selected: false,
            },
            OptionalPackage {
                id: "vlc".to_string(),
                name_key: "package.vlc.name".to_string(),
                description_key: "package.vlc.desc".to_string(),
                category: "multimedia".to_string(),
                size_mb: 100,
                default_selected: false,
            },
            OptionalPackage {
                id: "krita".to_string(),
                name_key: "package.krita.name".to_string(),
                description_key: "package.krita.desc".to_string(),
                category: "graphics".to_string(),
                size_mb: 250,
                default_selected: false,
            },
        ]
    }

    fn default_shortcuts() -> Vec<ShortcutOption> {
        vec![
            ShortcutOption {
                id: "edushell-learning".to_string(),
                name_key: "shortcut.learning.name".to_string(),
                description_key: "shortcut.learning.desc".to_string(),
                default_selected: true,
            },
            ShortcutOption {
                id: "edushell-terminal".to_string(),
                name_key: "shortcut.terminal.name".to_string(),
                description_key: "shortcut.terminal.desc".to_string(),
                default_selected: true,
            },
            ShortcutOption {
                id: "edushell-project".to_string(),
                name_key: "shortcut.project.name".to_string(),
                description_key: "shortcut.project.desc".to_string(),
                default_selected: true,
            },
            ShortcutOption {
                id: "edushell-office".to_string(),
                name_key: "shortcut.office.name".to_string(),
                description_key: "shortcut.office.desc".to_string(),
                default_selected: true,
            },
            ShortcutOption {
                id: "edushell-browser".to_string(),
                name_key: "shortcut.browser.name".to_string(),
                description_key: "shortcut.browser.desc".to_string(),
                default_selected: true,
            },
            ShortcutOption {
                id: "edushell-files".to_string(),
                name_key: "shortcut.files.name".to_string(),
                description_key: "shortcut.files.desc".to_string(),
                default_selected: true,
            },
            ShortcutOption {
                id: "edushell-settings".to_string(),
                name_key: "shortcut.settings.name".to_string(),
                description_key: "shortcut.settings.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "code".to_string(),
                name_key: "shortcut.code.name".to_string(),
                description_key: "shortcut.code.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "libreoffice-writer".to_string(),
                name_key: "shortcut.libreoffice-writer.name".to_string(),
                description_key: "shortcut.libreoffice-writer.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "libreoffice-calc".to_string(),
                name_key: "shortcut.libreoffice-calc.name".to_string(),
                description_key: "shortcut.libreoffice-calc.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "libreoffice-impress".to_string(),
                name_key: "shortcut.libreoffice-impress.name".to_string(),
                description_key: "shortcut.libreoffice-impress.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "scratch".to_string(),
                name_key: "shortcut.scratch.name".to_string(),
                description_key: "shortcut.scratch.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "geogebra".to_string(),
                name_key: "shortcut.geogebra.name".to_string(),
                description_key: "shortcut.geogebra.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "gimp".to_string(),
                name_key: "shortcut.gimp.name".to_string(),
                description_key: "shortcut.gimp.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "blender".to_string(),
                name_key: "shortcut.blender.name".to_string(),
                description_key: "shortcut.blender.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "audacity".to_string(),
                name_key: "shortcut.audacity.name".to_string(),
                description_key: "shortcut.audacity.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "vlc".to_string(),
                name_key: "shortcut.vlc.name".to_string(),
                description_key: "shortcut.vlc.desc".to_string(),
                default_selected: false,
            },
            ShortcutOption {
                id: "git".to_string(),
                name_key: "shortcut.git.name".to_string(),
                description_key: "shortcut.git.desc".to_string(),
                default_selected: false,
            },
        ]
    }
}

impl Default for WelcomeApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_app() -> WelcomeApp {
        with_temp_home("create-app", WelcomeApp::new)
    }

    fn with_temp_home<F: FnOnce() -> T, T>(name: &str, f: F) -> T {
        let _lock = HOME_LOCK.lock().unwrap();
        let temp = std::env::temp_dir().join(format!("welcome-test-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&temp);
        std::fs::create_dir_all(&temp).unwrap();
        let old_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", &temp);
        let result = f();
        if let Some(h) = old_home {
            std::env::set_var("HOME", h);
        } else {
            std::env::remove_var("HOME");
        }
        let _ = std::fs::remove_dir_all(&temp);
        result
    }

    #[test]
    fn test_creation_with_defaults() {
        let app = create_app();
        assert_eq!(app.current_step(), WelcomeStep::Welcome);
        assert_eq!(app.preferences().theme_mode, "light");
        assert_eq!(app.preferences().accent_color, "#4A90D9");
        assert_eq!(app.preferences().purpose, UserPurpose::School);
        assert!(app.is_first_run);
        assert!(!app.is_completed());
        assert!(app.is_first_step());
        assert!(!app.is_last_step());
    }

    #[test]
    fn test_step_navigation_next() {
        let mut app = create_app();
        assert_eq!(app.current_step(), WelcomeStep::Welcome);
        assert!(app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::Language);
        assert!(app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::Theme);
        assert!(app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::Purpose);
        assert!(app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::InstallPackages);
        assert!(app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::DesktopShortcuts);
        assert!(app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::Complete);
        assert!(!app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::Complete);
    }

    #[test]
    fn test_step_navigation_prev() {
        let mut app = create_app();
        app.go_to_step(WelcomeStep::DesktopShortcuts);
        assert!(app.prev_step());
        assert_eq!(app.current_step(), WelcomeStep::InstallPackages);
        assert!(app.prev_step());
        assert_eq!(app.current_step(), WelcomeStep::Purpose);
        assert!(app.prev_step());
        assert_eq!(app.current_step(), WelcomeStep::Theme);
        assert!(app.prev_step());
        assert_eq!(app.current_step(), WelcomeStep::Language);
        assert!(app.prev_step());
        assert_eq!(app.current_step(), WelcomeStep::Welcome);
        assert!(!app.prev_step());
        assert_eq!(app.current_step(), WelcomeStep::Welcome);
    }

    #[test]
    fn test_step_navigation_boundaries() {
        let mut app = create_app();
        assert!(!app.prev_step());
        assert_eq!(app.current_step(), WelcomeStep::Welcome);
        app.go_to_step(WelcomeStep::Complete);
        assert!(!app.next_step());
        assert_eq!(app.current_step(), WelcomeStep::Complete);
    }

    #[test]
    fn test_go_to_step() {
        let mut app = create_app();
        app.go_to_step(WelcomeStep::Purpose);
        assert_eq!(app.current_step(), WelcomeStep::Purpose);
        app.go_to_step(WelcomeStep::Complete);
        assert_eq!(app.current_step(), WelcomeStep::Complete);
        app.go_to_step(WelcomeStep::Welcome);
        assert_eq!(app.current_step(), WelcomeStep::Welcome);
    }

    #[test]
    fn test_first_step_detection() {
        let mut app = create_app();
        assert!(app.is_first_step());
        app.next_step();
        assert!(!app.is_first_step());
        app.go_to_step(WelcomeStep::Complete);
        assert!(!app.is_first_step());
    }

    #[test]
    fn test_last_step_detection() {
        let mut app = create_app();
        assert!(!app.is_last_step());
        app.go_to_step(WelcomeStep::Complete);
        assert!(app.is_last_step());
        app.go_to_step(WelcomeStep::Language);
        assert!(!app.is_last_step());
    }

    #[test]
    fn test_progress_percentage() {
        let mut app = create_app();
        assert!((app.progress_percentage() - 0.0).abs() < f64::EPSILON);
        app.go_to_step(WelcomeStep::Complete);
        assert!((app.progress_percentage() - 1.0).abs() < f64::EPSILON);
        app.go_to_step(WelcomeStep::Theme);
        assert!((app.progress_percentage() - 2.0 / 6.0).abs() < f64::EPSILON);
        app.go_to_step(WelcomeStep::InstallPackages);
        assert!((app.progress_percentage() - 4.0 / 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_set_language() {
        let mut app = create_app();
        assert_eq!(app.preferences().language, "auto");
        app.set_language("en-US");
        assert_eq!(app.preferences().language, "en-US");
        app.set_language("id-ID");
        assert_eq!(app.preferences().language, "id-ID");
    }

    #[test]
    fn test_set_theme_mode() {
        let mut app = create_app();
        assert_eq!(app.preferences().theme_mode, "light");
        app.set_theme_mode("dark");
        assert_eq!(app.preferences().theme_mode, "dark");
        app.set_theme_mode("auto");
        assert_eq!(app.preferences().theme_mode, "auto");
    }

    #[test]
    fn test_set_accent_color() {
        let mut app = create_app();
        assert_eq!(app.preferences().accent_color, "#4A90D9");
        app.set_accent_color("#ff0000");
        assert_eq!(app.preferences().accent_color, "#ff0000");
    }

    #[test]
    fn test_set_purpose() {
        let mut app = create_app();
        assert_eq!(app.preferences().purpose, UserPurpose::School);
        app.set_purpose(UserPurpose::Coding);
        assert_eq!(app.preferences().purpose, UserPurpose::Coding);
        app.set_purpose(UserPurpose::Custom);
        assert_eq!(app.preferences().purpose, UserPurpose::Custom);
    }

    #[test]
    fn test_toggle_package() {
        let mut app = create_app();
        app.deselect_all_packages();
        assert!(app.selected_packages().is_empty());
        app.toggle_package("libreoffice");
        assert_eq!(app.selected_packages().len(), 1);
        assert_eq!(app.selected_packages()[0].id, "libreoffice");
        app.toggle_package("libreoffice");
        assert!(app.selected_packages().is_empty());
    }

    #[test]
    fn test_select_deselect_all_packages() {
        let mut app = create_app();
        app.select_all_packages();
        assert_eq!(app.selected_packages().len(), app.available_packages().len());
        app.deselect_all_packages();
        assert!(app.selected_packages().is_empty());
    }

    #[test]
    fn test_toggle_shortcut() {
        let mut app = create_app();
        app.deselect_all_shortcuts();
        assert!(app.selected_shortcuts().is_empty());
        app.toggle_shortcut("code");
        assert_eq!(app.selected_shortcuts().len(), 1);
        assert_eq!(app.selected_shortcuts()[0].id, "code");
        app.toggle_shortcut("code");
        assert!(app.selected_shortcuts().is_empty());
    }

    #[test]
    fn test_select_deselect_all_shortcuts() {
        let mut app = create_app();
        app.select_all_shortcuts();
        assert_eq!(app.selected_shortcuts().len(), app.available_shortcuts().len());
        app.deselect_all_shortcuts();
        assert!(app.selected_shortcuts().is_empty());
    }

    #[test]
    fn test_purpose_based_package_recommendations() {
        let school = WelcomeApp::get_packages_for_purpose(UserPurpose::School);
        assert!(school.contains(&"libreoffice"));
        assert!(school.contains(&"scratch"));
        assert!(school.contains(&"kalzium"));

        let coding = WelcomeApp::get_packages_for_purpose(UserPurpose::Coding);
        assert!(coding.contains(&"code"));
        assert!(coding.contains(&"rustc"));
        assert!(coding.contains(&"docker"));

        let office = WelcomeApp::get_packages_for_purpose(UserPurpose::Office);
        assert!(office.contains(&"thunderbird"));
        assert!(office.contains(&"xournalpp"));

        let multimedia = WelcomeApp::get_packages_for_purpose(UserPurpose::Multimedia);
        assert!(multimedia.contains(&"blender"));
        assert!(multimedia.contains(&"kdenlive"));

        let custom = WelcomeApp::get_packages_for_purpose(UserPurpose::Custom);
        assert!(custom.is_empty());
    }

    #[test]
    fn test_purpose_based_shortcut_recommendations() {
        let school = WelcomeApp::get_shortcuts_for_purpose(UserPurpose::School);
        assert!(school.contains(&"scratch"));
        assert!(school.contains(&"geogebra"));

        let coding = WelcomeApp::get_shortcuts_for_purpose(UserPurpose::Coding);
        assert!(coding.contains(&"code"));
        assert!(coding.contains(&"git"));

        let office = WelcomeApp::get_shortcuts_for_purpose(UserPurpose::Office);
        assert!(office.contains(&"libreoffice-writer"));

        let multimedia = WelcomeApp::get_shortcuts_for_purpose(UserPurpose::Multimedia);
        assert!(multimedia.contains(&"gimp"));

        let custom = WelcomeApp::get_shortcuts_for_purpose(UserPurpose::Custom);
        assert!(custom.is_empty());
    }

    #[test]
    fn test_step_titles_not_empty() {
        let mut app = create_app();
        for step in &[
            WelcomeStep::Welcome,
            WelcomeStep::Language,
            WelcomeStep::Theme,
            WelcomeStep::Purpose,
            WelcomeStep::InstallPackages,
            WelcomeStep::DesktopShortcuts,
            WelcomeStep::Complete,
        ] {
            app.go_to_step(*step);
            assert!(!app.step_title().is_empty(), "title for {:?} should not be empty", step);
        }
    }

    #[test]
    fn test_step_descriptions_not_empty() {
        let mut app = create_app();
        for step in &[
            WelcomeStep::Welcome,
            WelcomeStep::Language,
            WelcomeStep::Theme,
            WelcomeStep::Purpose,
            WelcomeStep::InstallPackages,
            WelcomeStep::DesktopShortcuts,
            WelcomeStep::Complete,
        ] {
            app.go_to_step(*step);
            assert!(!app.step_description().is_empty(), "description for {:?} should not be empty", step);
        }
    }

    #[test]
    fn test_can_proceed() {
        let app = create_app();
        assert!(app.can_proceed());
    }

    #[test]
    fn test_reset() {
        let mut app = create_app();
        app.go_to_step(WelcomeStep::Complete);
        app.set_language("en-US");
        app.set_theme_mode("dark");
        app.set_purpose(UserPurpose::Coding);
        app.deselect_all_packages();
        app.reset();
        assert_eq!(app.current_step(), WelcomeStep::Welcome);
        assert_eq!(app.preferences().language, "auto");
        assert_eq!(app.preferences().theme_mode, "light");
        assert_eq!(app.preferences().purpose, UserPurpose::School);
        assert!(app.is_first_run);
        assert!(!app.preferences.install_optional_packages.is_empty());
        assert!(!app.preferences.create_desktop_shortcuts.is_empty());
    }

    #[test]
    fn test_completion_summary_not_empty() {
        let app = create_app();
        assert!(!app.completion_summary().is_empty());
    }

    #[test]
    fn test_total_steps() {
        assert_eq!(WelcomeApp::total_steps(), 7);
    }

    #[test]
    fn test_step_number_calculation() {
        let mut app = create_app();
        assert_eq!(app.current_step_number(), 1);
        app.go_to_step(WelcomeStep::Language);
        assert_eq!(app.current_step_number(), 2);
        app.go_to_step(WelcomeStep::Theme);
        assert_eq!(app.current_step_number(), 3);
        app.go_to_step(WelcomeStep::Purpose);
        assert_eq!(app.current_step_number(), 4);
        app.go_to_step(WelcomeStep::InstallPackages);
        assert_eq!(app.current_step_number(), 5);
        app.go_to_step(WelcomeStep::DesktopShortcuts);
        assert_eq!(app.current_step_number(), 6);
        app.go_to_step(WelcomeStep::Complete);
        assert_eq!(app.current_step_number(), 7);
    }

    #[test]
    fn test_completion_marking_roundtrip() {
        with_temp_home("mark-roundtrip", || {
            let mut app = WelcomeApp::new();
            assert!(app.is_first_run);
            assert!(!app.is_completed());
            app.set_language("en-US");
            app.set_accent_color("#00ff00");
            app.set_purpose(UserPurpose::Coding);
            app.mark_completed().unwrap();
            assert!(app.is_completed());
            assert!(!app.is_first_run);
            assert!(app.preferences.completed_at.is_some());

            let app2 = WelcomeApp::new();
            assert!(!app2.is_first_run);
            assert!(app2.is_completed());
            assert_eq!(app2.preferences().language, "en-US");
            assert_eq!(app2.preferences().accent_color, "#00ff00");
            assert_eq!(app2.preferences().purpose, UserPurpose::Coding);
        });
    }

    #[test]
    fn test_first_run_detection() {
        with_temp_home("firstrun", || {
            assert!(WelcomeApp::check_first_run());

            let app = WelcomeApp::new();
            assert!(app.is_first_run);

            let config_dir = WelcomeApp::config_dir();
            let marker_path = config_dir.join("welcome-completed");
            std::fs::create_dir_all(marker_path.parent().unwrap()).unwrap();
            std::fs::write(&marker_path, "").unwrap();

            assert!(!WelcomeApp::check_first_run());

            let app2 = WelcomeApp::new();
            assert!(!app2.is_first_run);
            drop(app);
        });
    }

    #[test]
    fn test_config_persistence() {
        with_temp_home("config-persist", || {
            let mut app = WelcomeApp::new();
            app.set_language("id-ID");
            app.set_theme_mode("dark");
            app.set_accent_color("#ff0000");
            app.set_purpose(UserPurpose::Multimedia);
            app.toggle_package("blender");
            app.toggle_shortcut("vlc");
            app.mark_completed().unwrap();

            let config_dir = WelcomeApp::config_dir();
            let prefs_path = config_dir.join("welcome-preferences.json");
            assert!(prefs_path.exists());
            let content = std::fs::read_to_string(&prefs_path).unwrap();
            assert!(content.contains("id-ID"));
            assert!(content.contains("dark"));
            assert!(content.contains("#ff0000"));
            assert!(content.contains("Multimedia"));
            assert!(content.contains("blender"));
            assert!(content.contains("vlc"));

            let marker_path = config_dir.join("welcome-completed");
            assert!(marker_path.exists());

            let app2 = WelcomeApp::new();
            assert!(!app2.is_first_run);
            assert_eq!(app2.preferences().language, "id-ID");
            assert_eq!(app2.preferences().theme_mode, "dark");
            assert_eq!(app2.preferences().accent_color, "#ff0000");
            assert_eq!(app2.preferences().purpose, UserPurpose::Multimedia);
            assert!(app2.preferences().install_optional_packages.contains(&"blender".to_string()));
            assert!(app2.preferences().create_desktop_shortcuts.contains(&"vlc".to_string()));
        });
    }

    #[test]
    fn test_selected_packages_filters_correctly() {
        let mut app = create_app();
        app.deselect_all_packages();
        assert!(app.selected_packages().is_empty());
        app.toggle_package("gimp");
        let selected = app.selected_packages();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, "gimp");
        assert_eq!(selected[0].name_key, "package.gimp.name");
    }

    #[test]
    fn test_selected_shortcuts_filters_correctly() {
        let mut app = create_app();
        app.deselect_all_shortcuts();
        assert!(app.selected_shortcuts().is_empty());
        app.toggle_shortcut("edushell-files");
        let selected = app.selected_shortcuts();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, "edushell-files");
    }

    #[test]
    fn test_default_selections_applied() {
        let app = create_app();
        let default_pkg_count = app.available_packages().iter().filter(|p| p.default_selected).count();
        assert_eq!(app.preferences().install_optional_packages.len(), default_pkg_count);
        let default_sc_count = app.available_shortcuts().iter().filter(|s| s.default_selected).count();
        assert_eq!(app.preferences().create_desktop_shortcuts.len(), default_sc_count);
    }

    #[test]
    fn test_build_window_noop_without_gtk() {
        let app = create_app();
        app.build_window();
    }

    #[test]
    fn test_available_packages_all_present() {
        let app = create_app();
        let ids: Vec<&str> = app.available_packages().iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&"libreoffice"));
        assert!(ids.contains(&"gimp"));
        assert!(ids.contains(&"blender"));
        assert!(ids.contains(&"code"));
        assert!(ids.contains(&"vlc"));
        assert_eq!(app.available_packages().len(), 29);
    }

    #[test]
    fn test_available_shortcuts_all_present() {
        let app = create_app();
        let ids: Vec<&str> = app.available_shortcuts().iter().map(|s| s.id.as_str()).collect();
        assert!(ids.contains(&"edushell-learning"));
        assert!(ids.contains(&"code"));
        assert!(ids.contains(&"vlc"));
        assert!(ids.contains(&"git"));
        assert_eq!(app.available_shortcuts().len(), 18);
    }
}
