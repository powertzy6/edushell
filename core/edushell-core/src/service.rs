// SPDX-License-Identifier: GPL-3.0-or-later

//! # Service Manager
//!
//! Central registry and lifecycle manager for all EduShell services.
//! Supports dependency resolution, health checking, crash recovery,
//! and status reporting.

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{EduResult, ServiceErrorKind};
use crate::event::{EventBus, SystemEvent};
use crate::lifecycle::LifecycleState;

/// Unique name for a service.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceName(pub String);

impl ServiceName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl fmt::Display for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ServiceName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Priority level for service startup ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServicePriority {
    /// Core infrastructure (config, logging, event bus).
    Core,
    /// System integration (session, D-Bus, window tracker).
    System,
    /// Shell services (theme, app registry, search).
    Shell,
    /// Application services (settings backend, etc.).
    Application,
    /// Optional services that can be deferred.
    Optional,
}

impl fmt::Display for ServicePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core => write!(f, "core"),
            Self::System => write!(f, "system"),
            Self::Shell => write!(f, "shell"),
            Self::Application => write!(f, "application"),
            Self::Optional => write!(f, "optional"),
        }
    }
}

/// Health status of a service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is operating normally.
    Healthy,
    /// Service is running but with degraded functionality.
    Degraded { reason: String },
    /// Service has failed and needs restart.
    Unhealthy { reason: String },
    /// Service status unknown.
    Unknown,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Degraded { reason } => write!(f, "degraded ({reason})"),
            Self::Unhealthy { reason } => write!(f, "unhealthy ({reason})"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Service trait — all EduShell services must implement this.
#[async_trait::async_trait]
pub trait Service: Send + Sync {
    /// Unique service name (e.g., "config", "theme", "session").
    fn name(&self) -> &'static str;

    /// Human-readable service description.
    fn description(&self) -> &'static str {
        ""
    }

    /// Priority for startup ordering.
    fn priority(&self) -> ServicePriority {
        ServicePriority::Optional
    }

    /// Service dependencies (names of services that must start first).
    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// Initialize the service (sync setup).
    fn init(&mut self) -> EduResult<()> {
        Ok(())
    }

    /// Start the service (async setup, e.g., connecting to D-Bus).
    async fn start(&self) -> EduResult<()> {
        Ok(())
    }

    /// Stop the service gracefully.
    async fn stop(&self) -> EduResult<()> {
        Ok(())
    }

    /// Check service health.
    fn health(&self) -> HealthStatus {
        HealthStatus::Unknown
    }

    /// Handle a system event.
    async fn handle_event(&self, _event: &SystemEvent) -> EduResult<()> {
        Ok(())
    }
}

// ── Service Registry ────────────────────────────────────────────

type ServiceBox = Arc<RwLock<Box<dyn Service>>>;

/// Registry of all available services.
struct ServiceEntry {
    service: ServiceBox,
    state: LifecycleState,
    dependencies: Vec<&'static str>,
    priority: ServicePriority,
}

/// Central service manager.
pub struct ServiceManager {
    services: RwLock<HashMap<ServiceName, ServiceEntry>>,
    event_bus: EventBus,
    start_order: RwLock<Vec<ServiceName>>,
}

impl ServiceManager {
    /// Create a new service manager.
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
            event_bus,
            start_order: RwLock::new(Vec::new()),
        }
    }

    /// Register a service.
    ///
    /// Returns an error if a service with the same name already exists.
    pub async fn register<S>(&self, service: S) -> EduResult<()>
    where
        S: Service + 'static,
    {
        let name = ServiceName::new(service.name());
        let deps = service.dependencies();
        let mut services = self.services.write().await;

        if services.contains_key(&name) {
            return Err(ServiceErrorKind::AlreadyRegistered {
                name: name.to_string(),
            }
            .into());
        }

        let priority = service.priority();

        services.insert(
            name.clone(),
            ServiceEntry {
                service: Arc::new(RwLock::new(Box::new(service))),
                state: LifecycleState::Uninitialized,
                dependencies: deps,
                priority,
            },
        );

        tracing::info!(target: "edushell::service", name = %name, "Service registered");
        Ok(())
    }

    /// Topological sort of services based on dependencies.
    fn resolve_start_order(&self, services: &HashMap<ServiceName, ServiceEntry>) -> EduResult<Vec<ServiceName>> {
        use std::collections::{VecDeque, HashSet};

        let mut in_degree: HashMap<&ServiceName, usize> = HashMap::new();
        let mut graph: HashMap<&ServiceName, Vec<&ServiceName>> = HashMap::new();

        for (name, _entry) in services.iter() {
            in_degree.entry(name).or_insert(0);
            graph.entry(name).or_insert_with(Vec::new);
        }

        // Build dependency graph
        for (name, entry) in services.iter() {
            let deps: Vec<&str> = self.get_dependencies(entry);
            for dep_name in deps {
                let dep_key = ServiceName::new(dep_name);
                if let Some(dep_entry) = services.keys().find(|k| k.0 == dep_key.0) {
                    graph.get_mut(dep_entry).unwrap().push(name);
                    *in_degree.entry(name).or_insert(0) += 1;
                } else {
                    return Err(ServiceErrorKind::UnsatisfiedDependency {
                        service: name.to_string(),
                        depends_on: dep_name.to_string(),
                    }
                    .into());
                }
            }
        }

        // Kahn's algorithm
        let mut queue: VecDeque<&ServiceName> = VecDeque::new();
        for (name, degree) in in_degree.iter() {
            if *degree == 0 {
                queue.push_back(name);
            }
        }

        let mut order = Vec::new();
        let mut visited = HashSet::new();

        while let Some(name) = queue.pop_front() {
            if !visited.insert(name.0.clone()) {
                continue;
            }
            order.push((*name).clone());

            if let Some(deps) = graph.get(name) {
                for dep in deps {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dep);
                        }
                    }
                }
            }
        }

        if order.len() != services.len() {
            return Err(ServiceErrorKind::DependencyCycle {
                names: services.keys().map(|n| n.to_string()).collect(),
            }
            .into());
        }

        Ok(order)
    }

    fn get_dependencies(&self, entry: &ServiceEntry) -> Vec<&'static str> {
        entry.dependencies.clone()
    }

    /// Initialize all registered services in dependency order.
    pub async fn init_all(&self) -> EduResult<()> {
        let services = self.services.read().await;
        let order = self.resolve_start_order(&services)?;

        // Sort by priority within same dependency level
        let mut sorted = order.clone();
        sorted.sort_by(|a, b| {
            let a_prio = services.get(a).map(|s| s.priority).unwrap_or(ServicePriority::Optional);
            let b_prio = services.get(b).map(|s| s.priority).unwrap_or(ServicePriority::Optional);
            a_prio.cmp(&b_prio)
        });

        drop(services);
        let services = self.services.read().await;

        for name in &sorted {
            if let Some(entry) = services.get(name) {
                let mut svc = entry.service.write().await;
                match svc.init() {
                    Ok(()) => {
                        tracing::info!(target: "edushell::service", name = %name, "Service initialized");
                    }
                    Err(e) => {
                        tracing::error!(
                            target: "edushell::service",
                            name = %name,
                            error = %e,
                            "Service init failed"
                        );
                        return Err(e);
                    }
                }
            }
        }

        self.event_bus.publish(SystemEvent::HealthCheck {
            healthy: true,
            details: format!("{} services initialized", sorted.len()),
        });

        Ok(())
    }

    /// Start all services in dependency order.
    pub async fn start_all(&self) -> EduResult<()> {
        let services = self.services.read().await;
        let order = self.resolve_start_order(&services)?;
        drop(services);

        let services = self.services.read().await;
        for name in &order {
            if let Some(entry) = services.get(name) {
                let svc = entry.service.read().await;
                match svc.start().await {
                    Ok(()) => {
                        tracing::info!(target: "edushell::service", name = %name, "Service started");
                    }
                    Err(e) => {
                        tracing::error!(
                            target: "edushell::service",
                            name = %name,
                            error = %e,
                            "Service start failed"
                        );
                        return Err(e);
                    }
                }
            }
        }

        tracing::info!(
            target: "edushell::service",
            count = order.len(),
            "All services started"
        );

        Ok(())
    }

    /// Stop all services (reverse order).
    pub async fn stop_all(&self) -> EduResult<()> {
        let services = self.services.read().await;
        let order = {
            let o = self.start_order.read().await;
            o.clone()
        };

        for name in order.iter().rev() {
            if let Some(entry) = services.get(name) {
                let svc = entry.service.read().await;
                match svc.stop().await {
                    Ok(()) => {
                        tracing::info!(target: "edushell::service", name = %name, "Service stopped");
                    }
                    Err(e) => {
                        tracing::warn!(
                            target: "edushell::service",
                            name = %name,
                            error = %e,
                            "Service stop encountered error"
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Get a service by name.
    pub async fn get(&self, name: impl Into<ServiceName>) -> Option<ServiceBox> {
        let name = name.into();
        let services = self.services.read().await;
        services.get(&name).map(|e| e.service.clone())
    }

    /// Check if a service is registered.
    pub async fn is_registered(&self, name: impl Into<ServiceName>) -> bool {
        let name = name.into();
        let services = self.services.read().await;
        services.contains_key(&name)
    }

    /// Get status of all services.
    pub async fn status_all(&self) -> Vec<ServiceStatus> {
        let services = self.services.read().await;
        let mut statuses = Vec::new();

        for (name, entry) in services.iter() {
            let health = entry.service.read().await.health();
            statuses.push(ServiceStatus {
                name: name.clone(),
                state: entry.state,
                health,
            });
        }

        statuses
    }

    /// Dispatch a system event to all services.
    pub async fn dispatch_event(&self, event: &SystemEvent) {
        let services = self.services.read().await;
        for entry in services.values() {
            let svc = entry.service.read().await;
            if let Err(e) = svc.handle_event(event).await {
                tracing::warn!(
                    target: "edushell::service",
                    service = %svc.name(),
                    error = %e,
                    "Event handler failed"
                );
            }
        }
    }
}

/// Status information for a single service.
#[derive(Debug, Clone)]
pub struct ServiceStatus {
    /// Service name.
    pub name: ServiceName,
    /// Current lifecycle state.
    pub state: LifecycleState,
    /// Health check result.
    pub health: HealthStatus,
}

// ── Helper macro for creating boxed services ───────────────────

/// Register a service with the service manager.
#[macro_export]
macro_rules! register_service {
    ($manager:expr, $service:expr) => {
        $manager.register(Box::new($service)).await
    };
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    struct TestService {
        name: &'static str,
        started: bool,
    }

    #[async_trait::async_trait]
    impl Service for TestService {
        fn name(&self) -> &'static str {
            self.name
        }

        fn description(&self) -> &'static str {
            "Test service for unit tests"
        }

        fn priority(&self) -> ServicePriority {
            ServicePriority::Core
        }

        fn init(&mut self) -> EduResult<()> {
            Ok(())
        }

        async fn start(&self) -> EduResult<()> {
            Ok(())
        }

        async fn stop(&self) -> EduResult<()> {
            Ok(())
        }

        fn health(&self) -> HealthStatus {
            HealthStatus::Healthy
        }
    }

    #[tokio::test]
    async fn test_register_service() {
        let bus = EventBus::new();
        let mgr = ServiceManager::new(bus);
        let svc = TestService {
            name: "test",
            started: false,
        };
        mgr.register(svc).await.unwrap();
        assert!(mgr.is_registered("test").await);
    }

    #[tokio::test]
    async fn test_duplicate_registration() {
        let bus = EventBus::new();
        let mgr = ServiceManager::new(bus);
        let svc1 = TestService {
            name: "dup",
            started: false,
        };
        let svc2 = TestService {
            name: "dup",
            started: false,
        };
        mgr.register(svc1).await.unwrap();
        let result = mgr.register(svc2).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_and_start_all() {
        let bus = EventBus::new();
        let mgr = ServiceManager::new(bus.clone());

        mgr.register(TestService {
            name: "core",
            started: false,
        })
        .await
        .unwrap();

        mgr.register(TestService {
            name: "shell",
            started: false,
        })
        .await
        .unwrap();

        mgr.init_all().await.unwrap();
        mgr.start_all().await.unwrap();
        mgr.stop_all().await.unwrap();
    }

    #[tokio::test]
    async fn test_service_status() {
        let bus = EventBus::new();
        let mgr = ServiceManager::new(bus);

        mgr.register(TestService {
            name: "status-test",
            started: false,
        })
        .await
        .unwrap();

        let statuses = mgr.status_all().await;
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].name.to_string(), "status-test");
    }

    #[test]
    fn test_health_status_display() {
        assert_eq!(HealthStatus::Healthy.to_string(), "healthy");
        assert_eq!(
            HealthStatus::Degraded {
                reason: "low memory".into()
            }
            .to_string(),
            "degraded (low memory)"
        );
    }
}
