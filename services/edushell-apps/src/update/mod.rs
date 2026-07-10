use chrono::Utc;

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateChannel {
    Stable,
    Beta,
    Nightly,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current_version: String,
    pub available_version: Option<String>,
    pub channel: UpdateChannel,
    pub last_check: Option<String>,
    pub update_available: bool,
    pub changelog: Vec<String>,
}

pub struct UpdateManager {
    current_version: String,
    channel: UpdateChannel,
    last_check: Option<String>,
    update_available: bool,
    available_version: Option<String>,
    changelog: Vec<String>,
    auto_check: bool,
}

impl UpdateManager {
    pub fn new(version: &str) -> Self {
        Self {
            current_version: version.to_string(),
            channel: UpdateChannel::Stable,
            last_check: None,
            update_available: false,
            available_version: None,
            changelog: Vec::new(),
            auto_check: true,
        }
    }

    pub fn check_for_updates(&mut self) {
        self.last_check = Some(Utc::now().to_rfc3339());
        self.update_available = false;
        self.available_version = None;
        self.changelog = Vec::new();
    }

    pub fn set_channel(&mut self, channel: UpdateChannel) {
        self.channel = channel;
    }

    pub fn info(&self) -> UpdateInfo {
        UpdateInfo {
            current_version: self.current_version.clone(),
            available_version: self.available_version.clone(),
            channel: self.channel.clone(),
            last_check: self.last_check.clone(),
            update_available: self.update_available,
            changelog: self.changelog.clone(),
        }
    }

    pub fn is_auto_check(&self) -> bool { self.auto_check }
    pub fn set_auto_check(&mut self, enabled: bool) { self.auto_check = enabled; }
}

impl Default for UpdateManager {
    fn default() -> Self { Self::new("1.0.0") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_manager_new() {
        let um = UpdateManager::new("1.0.0");
        assert_eq!(um.info().current_version, "1.0.0");
        assert_eq!(um.info().channel, UpdateChannel::Stable);
    }

    #[test]
    fn test_check_for_updates() {
        let mut um = UpdateManager::new("1.0.0");
        um.check_for_updates();
        assert!(um.info().last_check.is_some());
    }

    #[test]
    fn test_set_channel() {
        let mut um = UpdateManager::new("1.0.0");
        um.set_channel(UpdateChannel::Nightly);
        assert_eq!(um.info().channel, UpdateChannel::Nightly);
    }

    #[test]
    fn test_auto_check() {
        let mut um = UpdateManager::new("1.0.0");
        assert!(um.is_auto_check());
        um.set_auto_check(false);
        assert!(!um.is_auto_check());
    }

    #[test]
    fn test_update_info() {
        let um = UpdateManager::new("1.0.0-rc1");
        let info = um.info();
        assert_eq!(info.current_version, "1.0.0-rc1");
        assert!(!info.update_available);
    }
}
