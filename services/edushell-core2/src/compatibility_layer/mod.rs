//! Compatibility Layer — Cinnamon backward compatibility for plugins, themes, configs.
use std::collections::HashMap;

/// Compatibility mode.
#[derive(Debug, Clone, PartialEq)]
pub enum CompatMode {
    Native,
    CinnamonCompat,
    Legacy,
}

/// Compatibility report.
#[derive(Debug, Clone)]
pub struct CompatReport {
    pub component: String,
    pub mode: CompatMode,
    pub compatible: bool,
    pub issues: Vec<String>,
}

/// Compatibility layer.
pub struct CompatibilityLayer {
    enabled: bool,
    reports: Vec<CompatReport>,
    shims: HashMap<String, String>,
}

impl CompatibilityLayer {
    pub fn new() -> Self {
        Self {
            enabled: true,
            reports: Vec::new(),
            shims: HashMap::from([
                ("cinnamon-settings".into(), "edushell-settings".into()),
                ("cinnamon-menu".into(), "edushell-launcher".into()),
                ("cinnamon-workspace".into(), "edushell-workspace".into()),
            ]),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn resolve(&self, cinnamon_name: &str) -> Option<&str> {
        self.shims.get(cinnamon_name).map(|s| s.as_str())
    }

    pub fn check_compatibility(&self, component: &str, mode: &CompatMode) -> CompatReport {
        let compatible = match mode {
            CompatMode::Native => true,
            CompatMode::CinnamonCompat => self.shims.contains_key(component),
            CompatMode::Legacy => false,
        };
        let issues = if compatible {
            vec![]
        } else {
            vec![format!("{} not available in {:?} mode", component, mode)]
        };
        CompatReport {
            component: component.into(),
            mode: mode.clone(),
            compatible,
            issues,
        }
    }

    pub fn register_shim(&mut self, cinnamon_name: &str, edushell_name: &str) {
        self.shims
            .insert(cinnamon_name.to_string(), edushell_name.to_string());
    }

    pub fn report(&self) -> &[CompatReport] {
        &self.reports
    }

    pub fn run_checks(&mut self) {
        self.reports
            .push(self.check_compatibility("cinnamon-settings", &CompatMode::CinnamonCompat));
        self.reports
            .push(self.check_compatibility("cinnamon-menu", &CompatMode::CinnamonCompat));
        self.reports
            .push(self.check_compatibility("muffin", &CompatMode::CinnamonCompat));
        self.reports
            .push(self.check_compatibility("native-module", &CompatMode::Native));
    }

    pub fn shim_count(&self) -> usize {
        self.shims.len()
    }
}

impl Default for CompatibilityLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_new() {
        let cl = CompatibilityLayer::new();
        assert!(cl.is_enabled());
    }

    #[test]
    fn test_resolve() {
        let cl = CompatibilityLayer::new();
        assert_eq!(cl.resolve("cinnamon-settings"), Some("edushell-settings"));
        assert!(cl.resolve("nonexistent").is_none());
    }

    #[test]
    fn test_check_compatibility_native() {
        let cl = CompatibilityLayer::new();
        let report = cl.check_compatibility("any", &CompatMode::Native);
        assert!(report.compatible);
    }

    #[test]
    fn test_check_compatibility_cinnamon() {
        let cl = CompatibilityLayer::new();
        let report = cl.check_compatibility("cinnamon-settings", &CompatMode::CinnamonCompat);
        assert!(report.compatible);
    }

    #[test]
    fn test_check_compatibility_legacy() {
        let cl = CompatibilityLayer::new();
        let report = cl.check_compatibility("old-plugin", &CompatMode::Legacy);
        assert!(!report.compatible);
    }

    #[test]
    fn test_register_shim() {
        let mut cl = CompatibilityLayer::new();
        cl.register_shim("cinnamon-old", "edushell-new");
        assert_eq!(cl.shim_count(), 4);
        assert_eq!(cl.resolve("cinnamon-old"), Some("edushell-new"));
    }

    #[test]
    fn test_run_checks() {
        let mut cl = CompatibilityLayer::new();
        cl.run_checks();
        assert_eq!(cl.report().len(), 4);
    }

    #[test]
    fn test_toggle() {
        let mut cl = CompatibilityLayer::new();
        cl.set_enabled(false);
        assert!(!cl.is_enabled());
    }

    #[test]
    fn test_compat_mode_variants() {
        assert_eq!(
            format!("{:?}", CompatMode::CinnamonCompat),
            "CinnamonCompat"
        );
    }
}
