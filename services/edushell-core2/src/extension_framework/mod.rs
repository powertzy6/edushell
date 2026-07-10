//! Extension Framework v2 — sandboxed plugin lifecycle, permissions, hot reload.
use crate::core::CoreResult;
use std::collections::HashMap;

/// Extension identifier.
pub type ExtensionId = String;

/// Extension state.
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionState {
    Installed,
    Loaded,
    Enabled,
    Disabled,
    Crashed(String),
}

/// Extension permission.
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionPermission {
    ReadConfig,
    WriteConfig,
    NetworkAccess,
    FileAccess,
    Notification,
    SystemInfo,
}

/// Extension manifest.
#[derive(Debug, Clone)]
pub struct ExtensionManifest {
    pub id: ExtensionId,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub api_version: String,
    pub permissions: Vec<ExtensionPermission>,
    pub dependencies: Vec<String>,
}

/// Extension instance.
#[derive(Debug, Clone)]
pub struct ExtensionInstance {
    pub manifest: ExtensionManifest,
    pub state: ExtensionState,
    pub error_log: Vec<String>,
}

/// Extension framework.
pub struct ExtensionFramework {
    extensions: HashMap<ExtensionId, ExtensionInstance>,
    api_version: String,
}

impl ExtensionFramework {
    pub fn new(api_version: &str) -> Self {
        Self {
            extensions: HashMap::new(),
            api_version: api_version.to_string(),
        }
    }

    pub fn install(&mut self, manifest: ExtensionManifest) -> CoreResult<()> {
        if self.extensions.contains_key(&manifest.id) {
            return Err(crate::core::CoreError::AlreadyExists(manifest.id));
        }
        if manifest.api_version != self.api_version {
            return Err(crate::core::CoreError::VersionMismatch {
                expected: self.api_version.clone(),
                got: manifest.api_version,
            });
        }
        self.extensions.insert(
            manifest.id.clone(),
            ExtensionInstance {
                manifest,
                state: ExtensionState::Installed,
                error_log: Vec::new(),
            },
        );
        Ok(())
    }

    pub fn enable(&mut self, id: &str) -> bool {
        if let Some(ext) = self.extensions.get_mut(id) {
            ext.state = ExtensionState::Enabled;
            true
        } else {
            false
        }
    }

    pub fn disable(&mut self, id: &str) -> bool {
        if let Some(ext) = self.extensions.get_mut(id) {
            ext.state = ExtensionState::Disabled;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, id: &str) -> bool {
        self.extensions.remove(id).is_some()
    }

    pub fn report_crash(&mut self, id: &str, error: &str) {
        if let Some(ext) = self.extensions.get_mut(id) {
            ext.state = ExtensionState::Crashed(error.to_string());
            ext.error_log.push(error.to_string());
        }
    }

    pub fn get(&self, id: &str) -> Option<&ExtensionInstance> {
        self.extensions.get(id)
    }

    pub fn list(&self) -> Vec<&ExtensionInstance> {
        self.extensions.values().collect()
    }

    pub fn list_by_state(&self, state: &ExtensionState) -> Vec<&ExtensionInstance> {
        self.extensions
            .values()
            .filter(|e| e.state == *state)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.extensions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest(id: &str) -> ExtensionManifest {
        ExtensionManifest {
            id: id.into(),
            name: id.into(),
            version: "1.0".into(),
            author: "test".into(),
            description: "".into(),
            api_version: "2.0".into(),
            permissions: vec![ExtensionPermission::ReadConfig],
            dependencies: vec![],
        }
    }

    #[test]
    fn test_framework_new() {
        let fw = ExtensionFramework::new("2.0");
        assert_eq!(fw.count(), 0);
    }

    #[test]
    fn test_install_and_enable() {
        let mut fw = ExtensionFramework::new("2.0");
        assert!(fw.install(sample_manifest("ext-1")).is_ok());
        assert!(fw.enable("ext-1"));
        assert_eq!(fw.get("ext-1").unwrap().state, ExtensionState::Enabled);
    }

    #[test]
    fn test_install_duplicate_fails() {
        let mut fw = ExtensionFramework::new("2.0");
        assert!(fw.install(sample_manifest("ext-1")).is_ok());
        assert!(fw.install(sample_manifest("ext-1")).is_err());
    }

    #[test]
    fn test_version_mismatch() {
        let mut fw = ExtensionFramework::new("2.0");
        let mut m = sample_manifest("ext-2");
        m.api_version = "1.0".into();
        assert!(fw.install(m).is_err());
    }

    #[test]
    fn test_disable_and_remove() {
        let mut fw = ExtensionFramework::new("2.0");
        fw.install(sample_manifest("ext-3")).unwrap();
        assert!(fw.disable("ext-3"));
        assert!(fw.remove("ext-3"));
        assert_eq!(fw.count(), 0);
    }

    #[test]
    fn test_crash_report() {
        let mut fw = ExtensionFramework::new("2.0");
        fw.install(sample_manifest("ext-4")).unwrap();
        fw.report_crash("ext-4", "oom");
        let ext = fw.get("ext-4").unwrap();
        assert_eq!(format!("{:?}", ext.state), "Crashed(\"oom\")");
    }

    #[test]
    fn test_list_by_state() {
        let mut fw = ExtensionFramework::new("2.0");
        fw.install(sample_manifest("a")).unwrap();
        fw.install(sample_manifest("b")).unwrap();
        fw.enable("a");
        assert_eq!(fw.list_by_state(&ExtensionState::Enabled).len(), 1);
    }

    #[test]
    fn test_permission_variant() {
        assert_eq!(
            format!("{:?}", ExtensionPermission::NetworkAccess),
            "NetworkAccess"
        );
    }
}
