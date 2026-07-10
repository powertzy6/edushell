# Plugin, Logging & Crash Recovery — EduShell

---

## 1. Plugin Architecture (v3+ Ready)

### Design Philosophy
Plugin system adalah **infrastruktur v3+** tetapi arsitekturnya harus dipersiapkan sejak v1. Pada v1, cukup sediakan trait dan API boundaries. Implementasi plugin loader ditunda hingga v3.

### Plugin API Traits (v1 Foundation)

```rust
// core/edushell-core/src/plugin/mod.rs

/// Every plugin must implement this trait
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Unique plugin ID (e.g., "org.edushell.weather")
    fn id(&self) -> &'static str;

    /// Human-readable name
    fn name(&self) -> &'static str;

    /// Plugin version (semver)
    fn version(&self) -> &'static str;

    /// Initialize plugin with host context
    async fn init(&mut self, host: &PluginHost) -> Result<(), PluginError>;

    /// Called when plugin is activated
    async fn activate(&self) -> Result<(), PluginError>;

    /// Called when plugin is deactivated
    async fn deactivate(&self) -> Result<(), PluginError>;

    /// Plugin API version this plugin targets
    fn api_version(&self) -> u32;
}

/// Host context provided to plugins
pub struct PluginHost {
    pub config: Arc<RwLock<EduConfig>>,
    pub event_bus: EventBus,
    pub dbus_connection: zbus::Connection,
    pub theme_manager: Arc<ThemeManager>,
}
```

### Plugin Manifest Format (v3)

```toml
# manifest.toml (inside .edushugin package)
[plugin]
id = "org.edushell.weather"
name = "Weather Widget"
version = "1.0.0"
description = "Show current weather in the panel"
author = "Community Developer"
license = "MIT"

[compatibility]
edushell-version = ">=3.0.0"
plugin-api = "1"
gtk-version = ">=4.12"

[permissions]
network = true
config-read = ["shell.panel-position"]
config-write = []
notifications = true
execute = []

[dependencies]
# Other plugins required
requires = []
# Optional plugins for extended features
recommends = ["org.edushell.location"]

[ui]
type = "panel-widget"     # panel-widget | popover | standalone
position = "right"         # left | center | right (in panel)
size = "small"             # small | medium | large
```

### Plugin Isolation Strategy (v3)

```
┌─────────────────────────────────────────────────────────────┐
│                 Plugin Runtime (v3+)                         │
│                                                              │
│  ┌─────────────────────┐  ┌──────────────────────┐          │
│  │  Plugin Manager      │  │  Permission Manager   │         │
│  │                      │  │                       │         │
│  │  - load/unload       │  │  - manifest check     │         │
│  │  - lifecycle mgmt    │  │  - user approval      │         │
│  │  - dependency resolve│  │  - sandbox config     │         │
│  │  - version check     │  └──────────────────────┘          │
│  └─────────────────────┘                                     │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  Plugin Sandbox                                       │    │
│  │                                                       │    │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────┐ │    │
│  │  │ Weather      │  │ Clipboard    │  │ Calculator │ │    │
│  │  │ (Python)     │  │ (Rust)       │  │ (JS)       │ │    │
│  │  └──────────────┘  └──────────────┘  └────────────┘ │    │
│  │                                                       │    │
│  │  Communication: D-Bus (session bus)                   │    │
│  │  Isolation: Separate process per plugin               │    │
│  └──────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Plugin Installation Path

| Path | Type |
|------|------|
| `/usr/share/edushell/plugins/` | System-wide (package manager) |
| `~/.local/share/edushell/plugins/` | User-installed |

### What to Prepare in v1

| Item | Status | Location |
|------|--------|----------|
| `Plugin` trait | ✅ Define in core | `core/edushell-core/src/plugin/` |
| `PluginHost` struct | ✅ Define in core | `core/edushell-core/src/plugin/` |
| `PluginError` enum | ✅ Define in core | `core/edushell-core/src/plugin/` |
| `Permission` enum | ✅ Define in core | `core/edushell-core/src/plugin/` |
| Manifest schema | ✅ Document | `docs/architecture/PLUGIN_LOGGING_CRASH.md` |
| Plugin loader | ❌ Defer to v3 | — |
| Plugin sandbox | ❌ Defer to v3 | — |
| Plugin marketplace | ❌ Defer to v3+ | — |

---

## 2. Logging Architecture

### Design

```
┌─────────────────────────────────────────────────────────────┐
│                    Logging System                            │
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐   │
│  │ Tracing       │    │ Subscriber   │    │ Writer        │   │
│  │ Macros        │───►│              │───►│               │   │
│  │               │    │ Filter by    │    │ File rotation │   │
│  │ info!()       │    │ level/module │    │ Compression   │   │
│  │ warn!()       │    │              │    │ Privacy       │   │
│  │ error!()      │    │ Multi-output │    │               │   │
│  │ debug!()      │    │              │    │ JSON + Text   │   │
│  │ trace!()      │    └──────────────┘    └──────────────┘   │
│  └──────────────┘                                           │
│                                                              │
│  ┌──────────────┐    ┌──────────────┐                        │
│  │ Stdout        │    │ File          │                       │
│  │ (development) │    │ (production)  │                       │
│  │               │    │               │                       │
│  │ Colorized     │    │ JSON lines    │                       │
│  │ Human-readable│    │ Machine-parse │                       │
│  └──────────────┘    └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

### Implementation

```rust
// core/edushell-core/src/logging/mod.rs

use tracing_subscriber::{fmt, filter, prelude::*, Registry};
use std::path::PathBuf;

/// Initialize logging system
pub fn init(log_dir: Option<PathBuf>) -> Result<(), LogError> {
    let log_dir = log_dir.unwrap_or_else(|| {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("edushell/logs")
    });

    // Ensure log directory exists
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| LogError::DirCreation(log_dir.clone(), e))?;

    // Log file with rotation
    let file_appender = rolling_file::RollingFileAppender::new(
        log_dir.join("edushell.log"),
        rolling_file::Rotation::Daily,
        50 * 1024 * 1024, // 50 MB max per file
    )
    .map_err(|e| LogError::FileCreation(e))?;

    // Filter: debug in debug builds, info in release
    let filter = if cfg!(debug_assertions) {
        filter::Targets::new()
            .with_target("edushell", tracing::Level::DEBUG)
            .with_default(tracing::Level::INFO)
    } else {
        filter::Targets::new()
            .with_target("edushell", tracing::Level::INFO)
            .with_default(tracing::Level::WARN)
    };

    // JSON file output (machine readable)
    let file_layer = fmt::layer()
        .json()
        .with_writer(file_appender)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    // Human-readable stdout (development)
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .pretty()
        .with_filter(filter::LevelFilter::DEBUG);

    // Combine
    Registry::default()
        .with(filter)
        .with(file_layer)
        .with(stdout_layer)
        .init();

    log::info!("Logging initialized: {}", log_dir.display());
    Ok(())
}
```

### Log Levels

| Level | User Action | Developer Action |
|-------|-------------|------------------|
| `ERROR` | Something broke | Fix immediately |
| `WARN` | Unexpected state | Investigate |
| `INFO` | Normal operation (theme changed, app launched) | Monitor |
| `DEBUG` | Hidden by default | Development debugging |
| `TRACE` | Hidden by default | Intensive debugging (function entry/exit) |

### Log Categories

```rust
// Use target modules for filtering
info!(target: "edushell::panel", "Panel initialized at position {:?}", pos);
info!(target: "edushell::launcher", "Search completed in {}ms", elapsed);
warn!(target: "edushell::daemon::indexer", "Index file skipped: {}", path);
error!(target: "edushell::dbus", "Failed to register service: {}", err);
```

### Log File Format

```json
// ~/.local/share/edushell/logs/edushell.log
{"timestamp":"2026-07-09T10:30:00.123Z","level":"INFO","target":"edushell::panel","message":"Panel initialized","position":"Bottom","pid":1234,"thread":"main"}
{"timestamp":"2026-07-09T10:30:01.456Z","level":"ERROR","target":"edushell::dbus","message":"DBus connection failed","error":"Connection refused","pid":1234,"thread":"tokio-worker","file":"src/core/edushell-core/src/ipc/dbus.rs","line":42}
```

### Privacy Protection

```rust
/// Never log sensitive information
impl LogSanitizer {
    pub fn sanitize_path(path: &str) -> String {
        // Replace username in paths
        if let Some(home) = dirs::home_dir() {
            path.replace(&home.to_string_lossy().to_string(), "~")
        } else {
            path.to_string()
        }
    }

    pub fn sanitize_config(config: &str) -> String {
        // Remove any potential secrets from config dumps
        config
            .lines()
            .filter(|l| !l.contains("password") && !l.contains("token") && !l.contains("secret"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

### Log Viewer (Future)

| Feature | Version |
|---------|---------|
| Log file rotation | v1 |
| Log viewer in EduSettings | v2 |
| Crash report submission | v2 |
| Remote log collection (opt-in) | v3 |

---

## 3. Crash Handler & Recovery

### Design Philosophy

```
╔══════════════════════════════════════════════════════════════╗
║                     Crash Recovery Model                     ║
║                                                              ║
║  Every component is designed to fail independently.          ║
║  When one component crashes:                                ║
║    1. Desktop stays running                                  ║
║    2. Component auto-restarts                                ║
║    3. User sees a friendly message                           ║
║    4. Crash log is saved for analysis                        ║
║    5. If repeated crash → safe mode                          ║
╚══════════════════════════════════════════════════════════════╝
```

### Crash Handler Implementation

```rust
// core/edushell-core/src/crash/panic.rs

use std::panic;

/// Install custom panic hook for all EduShell components
pub fn install_panic_hook(component_name: &'static str) {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        // 1. Log the panic
        let location = panic_info.location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown".to_string());

        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        log::error!(
            target: "edushell::crash",
            "PANIC in {}: {} at {}",
            component_name, message, location
        );

        // 2. Save crash dump
        save_crash_dump(component_name, &message, &location, None);

        // 3. Show user-friendly dialog
        show_crash_dialog(component_name, &message);

        // 4. Call default hook (prints to stderr)
        default_hook(panic_info);
    }));
}
```

### Crash Report Structure

```rust
// Generated crash dump
pub struct CrashReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub component: String,
    pub version: String,
    pub message: String,
    pub location: String,
    pub backtrace: String,
    pub system_info: SystemInfo,
    pub config_snapshot: String, // Sanitized config dump
    pub log_tail: String,        // Last 50 log lines
}

// Saved to: ~/.local/share/edushell/crash-reports/
// File: panel_20260709_103000.md
```

### Recovery Strategy

```rust
// core/edushell-core/src/crash/recovery.rs

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Monitors component health and handles recovery
pub struct RecoveryManager {
    /// Track crash counts per component (for crash loop detection)
    crash_counts: HashMap<String, Vec<Instant>>,
    max_restarts: u32,
    window_duration: Duration,
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self {
            crash_counts: HashMap::new(),
            max_restarts: 3,          // Max 3 crashes
            window_duration: Duration::from_secs(60), // In 60 seconds
        }
    }

    /// Called when a component crashes
    pub fn on_crash(&mut self, component: &str) -> RecoveryAction {
        // Record crash time
        let now = Instant::now();
        let crashes = self.crash_counts.entry(component.to_string())
            .or_insert_with(Vec::new);

        // Remove crashes outside the window
        crashes.retain(|t| now.duration_since(*t) < self.window_duration);
        crashes.push(now);

        let crash_count = crashes.len() as u32;

        if crash_count == 1 {
            // First crash: restart immediately
            log::warn!("{} crashed once, restarting...", component);
            RecoveryAction::RestartImmediately
        } else if crash_count <= self.max_restarts {
            // Multiple crashes: delay restart with backoff
            let delay = Duration::from_secs(2u64.pow(crash_count - 1));
            log::warn!(
                "{} crashed {} times in window, restarting in {:?}",
                component, crash_count, delay
            );
            RecoveryAction::RestartWithDelay(delay)
        } else {
            // Crash loop detected: enter safe mode
            log::error!(
                "{} crashed {} times — entering safe mode",
                component, crash_count
            );
            RecoveryAction::EnterSafeMode(component.to_string())
        }
    }
}

#[derive(Debug)]
pub enum RecoveryAction {
    RestartImmediately,
    RestartWithDelay(Duration),
    EnterSafeMode(String),
}
```

### Crash Flow Diagram

```
                         Component Crash
                              │
                              ▼
                    ┌─────────────────────┐
                    │  Panic Hook Fires    │
                    │                     │
                    │  1. Log error       │
                    │  2. Save crash dump │
                    │  3. Show dialog     │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │  RecoveryManager     │
                    │                     │
                    │  Check crash count  │
                    │  in 60s window      │
                    └──────────┬──────────┘
                               │
          ┌────────────────────┼────────────────────┐
          ▼                    ▼                    ▼
   ┌─────────────┐    ┌─────────────────┐    ┌──────────────┐
   │ Restart     │    │ Restart with    │    │ Safe Mode    │
   │ Immediately │    │ Backoff Delay   │    │              │
   │             │    │                 │    │ Fallback to  │
   │ (1st crash) │    │ (2nd-3rd crash) │    │ Cinnamon     │
   │             │    │                 │    │ Show warning │
   └─────────────┘    └─────────────────┘    └──────────────┘
```

### Safe Mode

```rust
impl SafeMode {
    /// Enter safe mode: minimal shell functionality
    pub fn enter() {
        // 1. Log safe mode entry
        log::error!("Entering SAFE MODE");

        // 2. Show user notification
        show_notification(
            "EduShell Safe Mode",
            "A component has crashed repeatedly. \
             EduShell has entered safe mode. \
             Some features may be disabled. \
             Please check logs and report this issue.",
            NotificationUrgency::Critical,
        );

        // 3. Disable non-essential features
        disable_feature("animations");
        disable_feature("blur_effects");
        disable_feature("auto_hide");

        // 4. Fallback unstable component to Cinnamon
        activate_fallback("panel", "cinnamon-panel");

        // 5. Save crash report for user to submit
        save_crash_report_email();
    }
}
```

### Process-Level Recovery

```rust
// In edushell-daemon (the watchdog)

#[tokio::main]
async fn main() {
    // Start main shell process
    let panel_handle = tokio::spawn(async {
        let status = std::process::Command::new("edushell-panel")
            .spawn()
            .expect("Failed to start panel")
            .wait();
        status
    });

    // Wait for panel to exit
    let exit_status = panel_handle.await.unwrap();

    if !exit_status.success() {
        // Panel did not exit cleanly
        let action = recovery_mgr.on_crash("panel");
        match action {
            RecoveryAction::RestartImmediately => {
                // Restart panel
                main(); // Recursive restart
            }
            RecoveryAction::EnterSafeMode(_) => {
                // Enter safe mode
                SafeMode::enter();
            }
            _ => {}
        }
    }
}
```

### Crash-Proof Design Rules

| Rule | Description |
|------|-------------|
| **No shared state** | Each module owns its state. Shared state goes through ConfigManager |
| **Graceful degradation** | Feature missing → log warning → continue. Never crash on optional feature |
| **Timeout all I/O** | D-Bus calls, file reads, network all have timeouts |
| **Defensive rendering** | GTK widget update wrapped in try/catch |
| **Health endpoint** | Each service exposes Ping() via D-Bus |
| **Startup validation** | Check all prerequisites before starting services |
