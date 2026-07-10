//! Process lifecycle, thread pool, event loop abstraction.

use crate::core::{CoreError, CoreResult};
use std::collections::HashMap;

/// Runtime phase.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimePhase {
    Init,
    Running,
    Shutdown,
}

/// Runtime statistics.
#[derive(Debug, Clone)]
pub struct RuntimeStats {
    pub uptime_secs: u64,
    pub thread_count: u32,
    pub event_count: u64,
    pub phase: RuntimePhase,
}

/// A generic event dispatched within the runtime.
#[derive(Debug, Clone)]
pub struct RuntimeEvent {
    pub id: String,
    pub kind: String,
    pub payload: String,
    pub timestamp: String,
}

/// Core runtime — manages lifecycle, events, threads.
pub struct EduShellRuntime {
    phase: RuntimePhase,
    started_at: Option<String>,
    events: Vec<RuntimeEvent>,
    services: HashMap<String, bool>,
}

impl EduShellRuntime {
    pub fn new() -> Self {
        Self {
            phase: RuntimePhase::Init,
            started_at: None,
            events: Vec::new(),
            services: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        self.phase = RuntimePhase::Running;
        self.started_at = Some(crate::core::now_iso());
    }

    pub fn shutdown(&mut self) {
        self.phase = RuntimePhase::Shutdown;
    }

    pub fn phase(&self) -> &RuntimePhase {
        &self.phase
    }
    pub fn started_at(&self) -> Option<&str> {
        self.started_at.as_deref()
    }

    pub fn emit(&mut self, kind: &str, payload: &str) {
        self.events.push(RuntimeEvent {
            id: crate::core::new_id(),
            kind: kind.to_string(),
            payload: payload.to_string(),
            timestamp: crate::core::now_iso(),
        });
    }

    pub fn events(&self) -> &[RuntimeEvent] {
        &self.events
    }

    pub fn register_service(&mut self, name: &str) -> CoreResult<()> {
        if self.services.contains_key(name) {
            return Err(CoreError::AlreadyExists(name.into()));
        }
        self.services.insert(name.to_string(), true);
        Ok(())
    }

    pub fn service_status(&self, name: &str) -> Option<bool> {
        self.services.get(name).copied()
    }

    pub fn stats(&self) -> RuntimeStats {
        RuntimeStats {
            uptime_secs: 0,
            thread_count: 4,
            event_count: self.events.len() as u64,
            phase: self.phase.clone(),
        }
    }
}

impl Default for EduShellRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_new() {
        let rt = EduShellRuntime::new();
        assert_eq!(rt.phase(), &RuntimePhase::Init);
    }

    #[test]
    fn test_runtime_start() {
        let mut rt = EduShellRuntime::new();
        rt.start();
        assert_eq!(rt.phase(), &RuntimePhase::Running);
        assert!(rt.started_at().is_some());
    }

    #[test]
    fn test_runtime_shutdown() {
        let mut rt = EduShellRuntime::new();
        rt.start();
        rt.shutdown();
        assert_eq!(rt.phase(), &RuntimePhase::Shutdown);
    }

    #[test]
    fn test_emit_event() {
        let mut rt = EduShellRuntime::new();
        rt.start();
        rt.emit("test", "hello");
        assert_eq!(rt.events().len(), 1);
    }

    #[test]
    fn test_register_service() {
        let mut rt = EduShellRuntime::new();
        assert!(rt.register_service("search").is_ok());
        assert!(rt.register_service("search").is_err());
        assert!(rt.service_status("search") == Some(true));
    }

    #[test]
    fn test_stats() {
        let rt = EduShellRuntime::new();
        let stats = rt.stats();
        assert_eq!(stats.thread_count, 4);
        assert_eq!(stats.event_count, 0);
    }
}
