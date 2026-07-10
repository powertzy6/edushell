#[derive(Debug, Clone, PartialEq)]
pub enum DisplayManager {
    LightDM,
    GDM,
    SDDM,
    Unknown(String),
}

pub struct LoginManagerIntegration {
    dm: DisplayManager,
    default_session: String,
    auto_login: bool,
    auto_login_user: Option<String>,
}

impl LoginManagerIntegration {
    pub fn new() -> Self {
        let dm = Self::detect_display_manager();
        Self {
            dm,
            default_session: "edushell".to_string(),
            auto_login: false,
            auto_login_user: None,
        }
    }

    pub fn detect_display_manager() -> DisplayManager {
        let checks = [
            ("lightdm", DisplayManager::LightDM),
            ("gdm", DisplayManager::GDM),
            ("gdm3", DisplayManager::GDM),
            ("sddm", DisplayManager::SDDM),
        ];
        for (name, dm) in &checks {
            if std::process::Command::new("which").arg(name).output().is_ok() {
                let out = std::process::Command::new("which").arg(name).output();
                if let Ok(output) = out {
                    if output.status.success() {
                        return dm.clone();
                    }
                }
            }
        }
        DisplayManager::Unknown("none".into())
    }

    pub fn dm(&self) -> &DisplayManager { &self.dm }
    pub fn default_session(&self) -> &str { &self.default_session }

    pub fn set_auto_login(&mut self, enabled: bool, user: Option<&str>) {
        self.auto_login = enabled;
        self.auto_login_user = user.map(|s| s.to_string());
    }

    pub fn is_auto_login(&self) -> bool { self.auto_login }
    pub fn auto_login_user(&self) -> Option<&str> { self.auto_login_user.as_deref() }

    pub fn session_desktop_file(&self) -> String {
        format!(
            "[Desktop Entry]\nType=Application\nName=EduShell\nComment=Educational Desktop Shell\nExec=edushell-session\nType=XSession\n"
        )
    }
}

impl Default for LoginManagerIntegration {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_new() {
        let li = LoginManagerIntegration::new();
        assert_eq!(li.default_session(), "edushell");
    }

    #[test]
    fn test_auto_login() {
        let mut li = LoginManagerIntegration::new();
        assert!(!li.is_auto_login());
        li.set_auto_login(true, Some("student"));
        assert!(li.is_auto_login());
        assert_eq!(li.auto_login_user(), Some("student"));
    }

    #[test]
    fn test_session_desktop_file() {
        let li = LoginManagerIntegration::new();
        let content = li.session_desktop_file();
        assert!(content.contains("EduShell"));
        assert!(content.contains("edushell-session"));
    }

    #[test]
    fn test_detect_dm() {
        let dm = LoginManagerIntegration::detect_display_manager();
        let _ = dm;
    }

    #[test]
    fn test_display_manager_debug() {
        let dm = DisplayManager::Unknown("custom".into());
        assert_eq!(format!("{:?}", dm), "Unknown(\"custom\")");
    }
}
