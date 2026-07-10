// SPDX-License-Identifier: GPL-3.0-or-later

//! # EduShell Daemon
//!
//! Main binary entry point. Initializes logging, loads
//! configuration, creates the event bus, registers and
//! starts all core services, then waits for a shutdown
//! signal to gracefully stop everything.

use clap::Parser;
use tokio::signal;
use tracing::info;

use edushell_core::config::{ConfigManager, EduConfig};
use edushell_core::error::{EduError, EduResult};
use edushell_core::event::EventBus;
use edushell_core::logging::{init_logging, LogConfig, LogLevel};
use edushell_core::service::{Service, ServiceManager, ServicePriority};

/// EduShell daemon CLI arguments.
#[derive(Parser, Debug)]
#[command(name = "edushell-daemon", version, about = "EduShell Desktop Shell Daemon")]
struct Cli {
    /// Path to configuration file.
    #[arg(short, long)]
    config: Option<String>,

    /// Log level (trace, debug, info, warn, error).
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Enable JSON formatted logs.
    #[arg(long)]
    json_logs: bool,

    /// Dry run (initialize but don't start services).
    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> EduResult<()> {
    let cli = Cli::parse();

    // Initialize logging before anything else
    let log_level: LogLevel = cli.log_level.parse().map_err(|e: String| {
        EduError::Unknown(format!("Invalid log level: {e}"))
    })?;

    let log_config = LogConfig {
        stdout_level: log_level,
        json_format: cli.json_logs,
        ..LogConfig::default()
    };

    init_logging(&log_config)?;

    info!(
        target: "edushell::daemon",
        version = env!("CARGO_PKG_VERSION"),
        "EduShell daemon starting"
    );

    // Create shared event bus
    let event_bus = EventBus::new();

    // Load configuration
    let config_manager = ConfigManager::new();
    let config = if let Some(config_path) = cli.config {
        ConfigManager::with_path(std::path::PathBuf::from(config_path)).load()?
    } else {
        config_manager.load()?
    };

    info!(
        target: "edushell::daemon",
        config_path = %config_manager.path().display(),
        "Configuration loaded"
    );

    // Create service manager
    let service_manager = ServiceManager::new(event_bus.clone());

    // Register all core services
    register_core_services(&service_manager, &config, &event_bus).await?;

    // Initialize and start services
    service_manager.init_all().await?;
    info!(target: "edushell::daemon", "All services initialized");

    if cli.dry_run {
        info!(target: "edushell::daemon", "Dry run complete, exiting");
        return Ok(());
    }

    service_manager.start_all().await?;
    info!(target: "edushell::daemon", "All services started");

    // Wait for shutdown signal
    info!(target: "edushell::daemon", "Running, waiting for shutdown signal");
    wait_for_shutdown().await;

    // Graceful shutdown
    info!(target: "edushell::daemon", "Shutting down services");
    service_manager.stop_all().await?;
    info!(target: "edushell::daemon", "Shutdown complete");

    Ok(())
}

/// Register all core EduShell services.
async fn register_core_services(
    mgr: &ServiceManager,
    config: &EduConfig,
    event_bus: &EventBus,
) -> EduResult<()> {
    // Core: config service
    mgr.register(ConfigService {
        config: config.clone(),
        event_bus: event_bus.clone(),
    })
    .await?;

    // System: theme engine
    mgr.register(ThemeService {
        config: config.theme.clone(),
        event_bus: event_bus.clone(),
    })
    .await?;

    // System: window tracker
    mgr.register(WindowService::new(event_bus.clone())).await?;

    // System: session manager
    mgr.register(SessionService::new(event_bus.clone())).await?;

    // System: clock service
    mgr.register(ClockServiceWrapper).await?;

    // Shell: app registry
    mgr.register(AppRegistryService::new(event_bus.clone())).await?;

    // Shell: search engine
    mgr.register(SearchService::new(event_bus.clone())).await?;

    // Application: settings backend
    mgr.register(SettingsService::new(event_bus.clone())).await?;

    Ok(())
}

/// Wait for SIGINT or SIGTERM.
async fn wait_for_shutdown() {
    signal::ctrl_c().await.ok();
}

// ── Default Service Implementations ─────────────────────────────

struct ConfigService {
    config: EduConfig,
    event_bus: EventBus,
}

#[async_trait::async_trait]
impl Service for ConfigService {
    fn name(&self) -> &'static str { "config" }
    fn description(&self) -> &'static str { "Configuration management" }
    fn priority(&self) -> ServicePriority { ServicePriority::Core }

    fn init(&mut self) -> EduResult<()> {
        info!(target: "edushell::config", "Config service initialized");
        Ok(())
    }
}

struct ThemeService {
    config: edushell_core::config::ThemeConfig,
    event_bus: EventBus,
}

#[async_trait::async_trait]
impl Service for ThemeService {
    fn name(&self) -> &'static str { "theme" }
    fn description(&self) -> &'static str { "Theme engine" }
    fn priority(&self) -> ServicePriority { ServicePriority::System }
    fn dependencies(&self) -> Vec<&'static str> { vec!["config"] }

    fn init(&mut self) -> EduResult<()> {
        info!(target: "edushell::theme", "Theme service initialized");
        Ok(())
    }
}

struct WindowService {
    event_bus: EventBus,
}

impl WindowService {
    fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
}

#[async_trait::async_trait]
impl Service for WindowService {
    fn name(&self) -> &'static str { "window" }
    fn description(&self) -> &'static str { "Window tracker" }
    fn priority(&self) -> ServicePriority { ServicePriority::System }
    fn dependencies(&self) -> Vec<&'static str> { vec!["config"] }

    fn init(&mut self) -> EduResult<()> {
        info!(target: "edushell::window", "Window service initialized");
        Ok(())
    }
}

struct SessionService {
    event_bus: EventBus,
}

impl SessionService {
    fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
}

#[async_trait::async_trait]
impl Service for SessionService {
    fn name(&self) -> &'static str { "session" }
    fn description(&self) -> &'static str { "Session management" }
    fn priority(&self) -> ServicePriority { ServicePriority::System }
    fn dependencies(&self) -> Vec<&'static str> { vec!["config"] }
}

struct ClockServiceWrapper;

#[async_trait::async_trait]
impl Service for ClockServiceWrapper {
    fn name(&self) -> &'static str { "clock" }
    fn description(&self) -> &'static str { "Time and date service" }
    fn priority(&self) -> ServicePriority { ServicePriority::System }
}

struct AppRegistryService {
    event_bus: EventBus,
}

impl AppRegistryService {
    fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
}

#[async_trait::async_trait]
impl Service for AppRegistryService {
    fn name(&self) -> &'static str { "app-registry" }
    fn description(&self) -> &'static str { "Application desktop entry registry" }
    fn priority(&self) -> ServicePriority { ServicePriority::Shell }
    fn dependencies(&self) -> Vec<&'static str> { vec!["config"] }
}

struct SearchService {
    event_bus: EventBus,
}

impl SearchService {
    fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
}

#[async_trait::async_trait]
impl Service for SearchService {
    fn name(&self) -> &'static str { "search" }
    fn description(&self) -> &'static str { "Search engine" }
    fn priority(&self) -> ServicePriority { ServicePriority::Shell }
    fn dependencies(&self) -> Vec<&'static str> { vec!["app-registry", "settings"] }
}

struct SettingsService {
    event_bus: EventBus,
}

impl SettingsService {
    fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
}

#[async_trait::async_trait]
impl Service for SettingsService {
    fn name(&self) -> &'static str { "settings" }
    fn description(&self) -> &'static str { "Settings backend" }
    fn priority(&self) -> ServicePriority { ServicePriority::Application }
    fn dependencies(&self) -> Vec<&'static str> { vec!["config"] }
}
