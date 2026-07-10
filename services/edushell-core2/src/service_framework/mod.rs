//! Service Framework — background service lifecycle.
use crate::core::CoreResult;
use std::collections::HashMap;

/// Service state.
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    Stopped,
    Running,
    Failed(String),
}

/// Service definition.
#[derive(Debug, Clone)]
pub struct ServiceDef {
    pub id: String,
    pub name: String,
    pub version: String,
    pub auto_start: bool,
    pub critical: bool,
}

/// Service instance.
#[derive(Debug, Clone)]
pub struct ServiceInstance {
    pub def: ServiceDef,
    pub state: ServiceState,
}

/// Service framework.
pub struct ServiceFramework {
    services: HashMap<String, ServiceInstance>,
}

impl ServiceFramework {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, def: ServiceDef) -> CoreResult<()> {
        if self.services.contains_key(&def.id) {
            return Err(crate::core::CoreError::AlreadyExists(def.id));
        }
        self.services.insert(
            def.id.clone(),
            ServiceInstance {
                def,
                state: ServiceState::Stopped,
            },
        );
        Ok(())
    }

    pub fn start(&mut self, id: &str) -> bool {
        if let Some(svc) = self.services.get_mut(id) {
            svc.state = ServiceState::Running;
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self, id: &str) -> bool {
        if let Some(svc) = self.services.get_mut(id) {
            svc.state = ServiceState::Stopped;
            true
        } else {
            false
        }
    }

    pub fn fail(&mut self, id: &str, error: &str) {
        if let Some(svc) = self.services.get_mut(id) {
            svc.state = ServiceState::Failed(error.to_string());
        }
    }

    pub fn get(&self, id: &str) -> Option<&ServiceInstance> {
        self.services.get(id)
    }

    pub fn list(&self) -> Vec<&ServiceInstance> {
        self.services.values().collect()
    }

    pub fn list_running(&self) -> Vec<&ServiceInstance> {
        self.services
            .values()
            .filter(|s| s.state == ServiceState::Running)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.services.len()
    }
}

impl Default for ServiceFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_def(id: &str) -> ServiceDef {
        ServiceDef {
            id: id.into(),
            name: id.into(),
            version: "1.0".into(),
            auto_start: true,
            critical: false,
        }
    }

    #[test]
    fn test_framework_new() {
        let sf = ServiceFramework::new();
        assert_eq!(sf.count(), 0);
    }

    #[test]
    fn test_register() {
        let mut sf = ServiceFramework::new();
        assert!(sf.register(sample_def("search")).is_ok());
        assert_eq!(sf.count(), 1);
    }

    #[test]
    fn test_register_duplicate_fails() {
        let mut sf = ServiceFramework::new();
        sf.register(sample_def("search")).unwrap();
        assert!(sf.register(sample_def("search")).is_err());
    }

    #[test]
    fn test_start_stop() {
        let mut sf = ServiceFramework::new();
        sf.register(sample_def("notify")).unwrap();
        assert!(sf.start("notify"));
        assert_eq!(sf.get("notify").unwrap().state, ServiceState::Running);
        assert!(sf.stop("notify"));
        assert_eq!(sf.get("notify").unwrap().state, ServiceState::Stopped);
    }

    #[test]
    fn test_fail() {
        let mut sf = ServiceFramework::new();
        sf.register(sample_def("sync")).unwrap();
        sf.fail("sync", "timeout");
        let s = sf.get("sync").unwrap();
        assert_eq!(format!("{:?}", s.state), "Failed(\"timeout\")");
    }

    #[test]
    fn test_list_running() {
        let mut sf = ServiceFramework::new();
        sf.register(sample_def("a")).unwrap();
        sf.register(sample_def("b")).unwrap();
        sf.start("a");
        assert_eq!(sf.list_running().len(), 1);
    }
}
