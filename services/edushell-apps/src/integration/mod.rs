use crate::browser_hub::BrowserHub;
use crate::edu_terminal::EduTerminal;
use crate::file_manager::FileManagerIntegration;
use crate::icons::IconManager;
use crate::learning_engine::LearningContentEngine;
use crate::learning_hub::LearningHub;
use crate::localization::LocalizationManager;
use crate::office_hub::OfficeHub;
use crate::project_hub::ProjectHub;
use crate::search::GlobalSearch;
use crate::settings_center::SettingsCenter;
use crate::shortcuts::ShortcutManager;
use crate::software_center::SoftwareCenter;
use crate::theme::AppThemeManager;
use crate::wallpapers::WallpaperManager;
use crate::welcome::WelcomeApp;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntegrationError {
    #[error("module not available: {0}")]
    ModuleNotAvailable(String),
    #[error("communication error: {0}")]
    CommError(String),
    #[error("session error: {0}")]
    SessionError(String),
    #[error("sync error: {0}")]
    SyncError(String),
}

pub struct EduShellAppBus {
    localization: LocalizationManager,
    theme: AppThemeManager,
    icons: IconManager,
    shortcuts: ShortcutManager,
    learning_engine: Arc<Mutex<LearningContentEngine>>,
    learning_hub: Arc<Mutex<LearningHub>>,
    welcome: Arc<Mutex<WelcomeApp>>,
    edu_terminal: Arc<Mutex<EduTerminal>>,
    project_hub: Arc<Mutex<ProjectHub>>,
    office_hub: Arc<Mutex<OfficeHub>>,
    browser_hub: Arc<Mutex<BrowserHub>>,
    settings_center: Arc<Mutex<SettingsCenter>>,
    file_manager: Arc<Mutex<FileManagerIntegration>>,
    software_center: Arc<Mutex<SoftwareCenter>>,
    search: Arc<Mutex<GlobalSearch>>,
    wallpapers: Arc<Mutex<WallpaperManager>>,
    session: SessionManager,
    settings_cache: HashMap<String, String>,
}

impl EduShellAppBus {
    pub fn new() -> Self {
        let localization = LocalizationManager::new();
        let theme = AppThemeManager::default();
        Self {
            session: SessionManager::new(),
            settings_cache: HashMap::new(),
            localization,
            theme,
            icons: IconManager::new(),
            shortcuts: ShortcutManager::new(),
            learning_engine: Arc::new(Mutex::new(LearningContentEngine::new(None))),
            learning_hub: Arc::new(Mutex::new(LearningHub::new(None))),
            welcome: Arc::new(Mutex::new(WelcomeApp::new())),
            edu_terminal: Arc::new(Mutex::new(EduTerminal::new())),
            project_hub: Arc::new(Mutex::new(ProjectHub::new(None))),
            office_hub: Arc::new(Mutex::new(OfficeHub::new(None))),
            browser_hub: Arc::new(Mutex::new(BrowserHub::new())),
            settings_center: Arc::new(Mutex::new(SettingsCenter::new(None))),
            file_manager: Arc::new(Mutex::new(FileManagerIntegration::new())),
            software_center: Arc::new(Mutex::new(SoftwareCenter::new(true))),
            search: Arc::new(Mutex::new(GlobalSearch::new())),
            wallpapers: Arc::new(Mutex::new(WallpaperManager::new(None))),
        }
    }

    pub fn localization(&self) -> &LocalizationManager { &self.localization }
    pub fn theme(&self) -> &AppThemeManager { &self.theme }
    pub fn icons(&self) -> &IconManager { &self.icons }
    pub fn shortcuts(&self) -> &ShortcutManager { &self.shortcuts }
    pub fn learning_engine(&self) -> &Arc<Mutex<LearningContentEngine>> { &self.learning_engine }
    pub fn learning_hub(&self) -> &Arc<Mutex<LearningHub>> { &self.learning_hub }
    pub fn welcome(&self) -> &Arc<Mutex<WelcomeApp>> { &self.welcome }
    pub fn edu_terminal(&self) -> &Arc<Mutex<EduTerminal>> { &self.edu_terminal }
    pub fn project_hub(&self) -> &Arc<Mutex<ProjectHub>> { &self.project_hub }
    pub fn office_hub(&self) -> &Arc<Mutex<OfficeHub>> { &self.office_hub }
    pub fn browser_hub(&self) -> &Arc<Mutex<BrowserHub>> { &self.browser_hub }
    pub fn settings_center(&self) -> &Arc<Mutex<SettingsCenter>> { &self.settings_center }
    pub fn file_manager(&self) -> &Arc<Mutex<FileManagerIntegration>> { &self.file_manager }
    pub fn software_center(&self) -> &Arc<Mutex<SoftwareCenter>> { &self.software_center }
    pub fn search(&self) -> &Arc<Mutex<GlobalSearch>> { &self.search }
    pub fn wallpapers(&self) -> &Arc<Mutex<WallpaperManager>> { &self.wallpapers }
    pub fn session(&self) -> &SessionManager { &self.session }

    pub fn broadcast_event(&mut self, event: SystemEvent) {
        match event {
            SystemEvent::LocaleChanged(code) => self.sync_locale(&code),
            SystemEvent::ThemeChanged(mode) => self.sync_theme(&mode),
            SystemEvent::UserSessionStarted(user) => self.on_session_start(&user),
            SystemEvent::UserSessionEnded(user) => self.on_session_end(&user),
            SystemEvent::SettingsUpdated(key, value) => self.cache_setting(&key, &value),
        }
    }

    fn sync_locale(&self, _code: &str) {}
    fn sync_theme(&self, _mode: &str) {}
    fn on_session_start(&self, _user: &str) {}
    fn on_session_end(&self, _user: &str) {}

    fn cache_setting(&mut self, key: &str, value: &str) {
        self.settings_cache.insert(key.to_string(), value.to_string());
    }

    pub fn get_cached_setting(&self, key: &str) -> Option<&str> {
        self.settings_cache.get(key).map(|s| s.as_str())
    }

    pub fn runtime_info(&self) -> RuntimeInfo {
        RuntimeInfo {
            modules: 16,
            active_session: self.session.is_active(),
            locale: self.localization.current_language().code().to_string(),
            theme_mode: format!("{:?}", self.theme.config().mode),
        }
    }
}

impl Default for EduShellAppBus {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone)]
pub enum SystemEvent {
    LocaleChanged(String),
    ThemeChanged(String),
    UserSessionStarted(String),
    UserSessionEnded(String),
    SettingsUpdated(String, String),
}

#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub modules: usize,
    pub active_session: bool,
    pub locale: String,
    pub theme_mode: String,
}

pub struct SessionManager {
    active: bool,
    current_user: Option<String>,
    session_start: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self { active: false, current_user: None, session_start: None }
    }

    pub fn start_session(&mut self, username: &str) {
        self.active = true;
        self.current_user = Some(username.to_string());
        self.session_start = Some(Utc::now().to_rfc3339());
    }

    pub fn end_session(&mut self) {
        self.active = false;
        self.current_user = None;
        self.session_start = None;
    }

    pub fn is_active(&self) -> bool { self.active }
    pub fn current_user(&self) -> Option<&str> { self.current_user.as_deref() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_creation() {
        let bus = EduShellAppBus::new();
        let info = bus.runtime_info();
        assert_eq!(info.modules, 16);
        assert!(!info.active_session);
    }

    #[test]
    fn test_session_lifecycle() {
        let mut bus = EduShellAppBus::new();
        assert!(!bus.session().is_active());
        bus.session.start_session("student1");
        assert!(bus.session.is_active());
        assert_eq!(bus.session.current_user(), Some("student1"));
        bus.session.end_session();
        assert!(!bus.session.is_active());
    }

    #[test]
    fn test_broadcast_locale_changed() {
        let mut bus = EduShellAppBus::new();
        bus.broadcast_event(SystemEvent::LocaleChanged("en-US".into()));
        assert!(bus.get_cached_setting("locale").is_none());
    }

    #[test]
    fn test_broadcast_settings_updated() {
        let mut bus = EduShellAppBus::new();
        bus.broadcast_event(SystemEvent::SettingsUpdated("theme.mode".into(), "dark".into()));
        assert_eq!(bus.get_cached_setting("theme.mode"), Some("dark"));
    }

    #[test]
    fn test_locale_code() {
        let bus = EduShellAppBus::new();
        assert_eq!(bus.runtime_info().locale, "id-ID");
    }

    #[test]
    fn test_broadcast_session_start() {
        let mut bus = EduShellAppBus::new();
        bus.broadcast_event(SystemEvent::UserSessionStarted("teacher".into()));
        assert!(!bus.session.is_active());
    }

    #[test]
    fn test_broadcast_session_end() {
        let mut bus = EduShellAppBus::new();
        bus.session.start_session("teacher");
        bus.broadcast_event(SystemEvent::UserSessionEnded("teacher".into()));
        assert!(bus.session.is_active());
    }
}
