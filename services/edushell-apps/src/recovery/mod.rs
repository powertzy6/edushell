use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CrashReport {
    pub module: String,
    pub error: String,
    pub recovered: bool,
    pub timestamp: String,
}

pub struct CrashRecovery {
    reports: Vec<CrashReport>,
    safe_mode: bool,
    disabled_modules: Vec<String>,
    fallback_config: HashMap<String, String>,
}

impl CrashRecovery {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
            safe_mode: false,
            disabled_modules: Vec::new(),
            fallback_config: HashMap::new(),
        }
    }

    pub fn report_crash(&mut self, module: &str, error: &str, recovered: bool) {
        self.reports.push(CrashReport {
            module: module.to_string(),
            error: error.to_string(),
            recovered,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
        self.disabled_modules.push(module.to_string());
        self.safe_mode = true;
    }

    pub fn is_safe_mode(&self) -> bool { self.safe_mode }
    pub fn reports(&self) -> &[CrashReport] { &self.reports }
    pub fn disabled_modules(&self) -> &[String] { &self.disabled_modules }

    pub fn enable_module(&mut self, module: &str) {
        self.disabled_modules.retain(|m| m != module);
        if self.disabled_modules.is_empty() {
            self.safe_mode = false;
        }
    }

    pub fn disable_module(&mut self, module: &str) {
        if !self.disabled_modules.contains(&module.to_string()) {
            self.disabled_modules.push(module.to_string());
        }
        self.safe_mode = true;
    }

    pub fn set_fallback(&mut self, key: &str, value: &str) {
        self.fallback_config.insert(key.to_string(), value.to_string());
    }

    pub fn get_fallback(&self, key: &str) -> Option<&str> {
        self.fallback_config.get(key).map(|s| s.as_str())
    }

    pub fn clear_reports(&mut self) {
        self.reports.clear();
    }
}

impl Default for CrashRecovery {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_new() {
        let r = CrashRecovery::new();
        assert!(!r.is_safe_mode());
        assert!(r.reports().is_empty());
    }

    #[test]
    fn test_report_crash_activates_safe_mode() {
        let mut r = CrashRecovery::new();
        r.report_crash("theme", "panic: index out of bounds", true);
        assert!(r.is_safe_mode());
        assert_eq!(r.reports().len(), 1);
        assert!(r.reports()[0].recovered);
    }

    #[test]
    fn test_enable_module_exits_safe_mode() {
        let mut r = CrashRecovery::new();
        r.report_crash("search", "thread panic", true);
        r.enable_module("search");
        assert!(!r.is_safe_mode());
    }

    #[test]
    fn test_disable_module() {
        let mut r = CrashRecovery::new();
        r.disable_module("office_hub");
        assert!(r.is_safe_mode());
        assert!(r.disabled_modules().contains(&"office_hub".to_string()));
    }

    #[test]
    fn test_fallback_config() {
        let mut r = CrashRecovery::new();
        r.set_fallback("theme.mode", "light");
        assert_eq!(r.get_fallback("theme.mode"), Some("light"));
    }

    #[test]
    fn test_clear_reports() {
        let mut r = CrashRecovery::new();
        r.report_crash("icons", "svg parse error", false);
        r.clear_reports();
        assert!(r.reports().is_empty());
        assert!(r.is_safe_mode());
    }
}
