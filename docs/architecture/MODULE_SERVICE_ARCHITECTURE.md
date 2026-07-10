# Module, Service & IPC Architecture — EduShell

## 1. Modular Architecture Overview

### Design Principle
Setiap modul adalah crate Rust independen dalam Cargo workspace. Modul berkomunikasi melalui:
- **Internal**: Rust traits + channels (dalam proses yang sama)
- **External**: D-Bus (antar proses berbeda)
- **Shared state**: ConfigManager (broadcast channel)

### Module Catalog

| Module | Crate | Type | Process | Dependencies |
|--------|-------|------|---------|--------------|
| Core | `edushell-core` | Library | In-process | tokio, serde, zbus |
| Cinnamon Adapter | `cinnamon-adapter` | Library | In-process | glib-rs, gio-rs |
| Theme Engine | `theme-engine` | Library | In-process | gtk4, serde |
| Localization | `localization` | Library | In-process | gettext-rs |
| Panel | `edushell-panel` | Binary | Separate | core, gtk4, layer-shell |
| Launcher | `edushell-launcher` | Binary | Same as panel | core, gtk4 |
| Notifications | `edushell-notifications` | Binary | Separate | core, gtk4, zbus |
| OSD | `edushell-osd` | Binary | Same as panel | core, gtk4 |
| Quick Settings | `edushell-quick-settings` | Binary | Same as panel | core, gtk4 |
| Settings | `edushell-settings` | Binary | Separate | core, gtk4, libadwaita |
| Learning Hub | `edushell-learning-hub` | Binary | Separate | core, gtk4 |
| Daemon | `edushell-daemon` | Binary | Separate | core, zbus |

### Module Independence Rules

```
┌────────────────────────────────────────────────────────┐
│                    Module Rules                         │
│                                                        │
│  1. Every module has a single responsibility           │
│  2. Modules communicate only through traits or IPC     │
│  3. No direct coupling between shell modules           │
│  4. Core library is the only shared dependency         │
│  5. Each module can be developed/tested independently  │
│  6. Module replacement = crate replacement             │
│                                                        │
│  ✅ edushell-panel  ─── D-Bus ──── edushell-daemon    │
│  ✅ edushell-panel  ─── traits ──→ edushell-core       │
│  ❌ edushell-panel  ─── import ──→ edushell-launcher   │
│                                                        │
└────────────────────────────────────────────────────────┘
```

### Module API Design

```rust
// Every module exposes a clean public API

/// Core module public API
pub mod edushell_core {
    pub fn init() -> Result<RuntimeHandle, EduError>;
    pub use config::*;
    pub use error::*;
    pub use logging::*;
    pub use ipc::*;
}

/// Theme engine public API
pub mod theme_engine {
    pub struct ThemeManager;
    impl ThemeManager {
        pub fn new() -> Self;
        pub fn load_theme(&self, name: &str) -> Result<(), ThemeError>;
        pub fn set_mode(&self, mode: ThemeMode);        // Light/Dark/Auto
        pub fn set_accent(&self, color: &str) -> Result<(), ThemeError>;
        pub fn apply(&self, gtk_app: &gtk4::Application);
        pub fn current(&self) -> ThemeInfo;
        pub fn watch(&self) -> tokio::sync::broadcast::Receiver<ThemeEvent>;
    }
}
```

---

## 2. Service Architecture

### Process Model

```
┌─────────────────────────────────────────────────────────────┐
│                    User Session (PID 1)                      │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  edushell-daemon (background services)               │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐              │   │
│  │  │ indexer  │ │ watcher  │ │ updater  │              │   │
│  │  └──────────┘ └──────────┘ └──────────┘              │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─────────────────────────┐  ┌─────────────────────────┐   │
│  │  edushell-panel process │  │  edushell-notifications │   │
│  │  (main UI shell)        │  │  (notification daemon)  │   │
│  │  ┌───────┐ ┌─────────┐  │  └─────────────────────────┘   │
│  │  │panel  │ │launcher │  │                                 │
│  │  │widget │ │popover  │  │  ┌─────────────────────────┐   │
│  │  │OSD    │ │quick-   │  │  │  edushell-settings      │   │
│  │  │       │ │settings │  │  │  (separate process)      │   │
│  │  └───────┘ └─────────┘  │  └─────────────────────────┘   │
│  └─────────────────────────┘                                 │
│                                                              │
│  ┌─────────────────────────┐  ┌─────────────────────────┐   │
│  │  edushell-learning-hub  │  │  Cinnamon services      │   │
│  │  (separate process)     │  │  (muffin, session, etc) │   │
│  └─────────────────────────┘  └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Process Rationale

| Decision | Rationale |
|----------|-----------|
| **Panel sebagai proses utama** | Panel adalah komponen yang selalu visible. Menjadi induk untuk launcher, OSD, quick-settings. |
| **Daemon terpisah** | Background services (indexer, watcher) tidak perlu GTK. Proses terpisah menghemat memori dan meningkatkan stabilitas. |
| **Settings terpisah** | Settings app dibuka/ditutup sesuai kebutuhan. Tidak perlu tinggal di memori. |
| **Notifications terpisah** | Notification daemon perlu berjalan terus meskipun panel crash. DBus service terpisah. |
| **Learning Hub terpisah** | Aplikasi berat (WebView) dipisah agar tidak membebani panel. |

### Service Lifecycle

```rust
// services/edushell-daemon/src/service.rs

pub struct DaemonService {
    modules: Vec<Box<dyn ServiceModule>>,
    runtime: RuntimeHandle,
}

#[async_trait]
pub trait ServiceModule: Send + Sync {
    /// Module name (for logging)
    fn name(&self) -> &'static str;

    /// Initialize module (async)
    async fn init(&mut self, runtime: &RuntimeHandle) -> Result<(), EduError>;

    /// Start module
    async fn start(&self) -> Result<(), EduError>;

    /// Handle module event
    async fn handle_event(&self, event: ServiceEvent) -> Result<(), EduError>;

    /// Graceful shutdown
    async fn shutdown(&self) -> Result<(), EduError>;

    /// Health check
    fn health(&self) -> HealthStatus;
}
```

---

## 3. IPC Architecture

### D-Bus Service Map

```xml
<!-- Session Bus Services -->
<node>
  <!-- Main shell service -->
  <interface name="org.edushell.Shell">
    <method name="SetPanelVisible">
      <arg type="b" name="visible" direction="in"/>
    </method>
    <method name="ToggleLauncher"/>
    <method name="ToggleQuickSettings"/>
    <signal name="ThemeChanged">
      <arg type="s" name="mode"/>
    </signal>
  </interface>

  <!-- Settings service -->
  <interface name="org.edushell.Settings">
    <method name="OpenPage">
      <arg type="s" name="page"/>
    </method>
    <method name="GetConfig">
      <arg type="s" name="config" direction="out"/>
    </method>
  </interface>

  <!-- Notification service -->
  <interface name="org.edushell.Notifications">
    <method name="Notify">
      <arg type="s" name="app_name"/>
      <arg type="s" name="summary"/>
      <arg type="s" name="body"/>
      <arg type="u" name="timeout"/>
    </method>
    <method name="GetHistory">
      <arg type="av" name="notifications" direction="out"/>
    </method>
    <signal name="NotificationClosed">
      <arg type="u" name="id"/>
    </signal>
  </interface>

  <!-- Search service -->
  <interface name="org.edushell.Search">
    <method name="Search">
      <arg type="s" name="query"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="IndexReady">
      <arg type="b" name="ready" direction="out"/>
    </method>
  </interface>

  <!-- Daemon service -->
  <interface name="org.edushell.Daemon">
    <method name="Ping"/>
    <method name="GetStatus">
      <arg type="s" name="status" direction="out"/>
    </method>
    <signal name="ServiceEvent">
      <arg type="s" name="event"/>
    </signal>
  </interface>
</node>
```

### Internal Communication (within same process)

```rust
// core/edushell-core/src/ipc/signals.rs

use tokio::sync::broadcast;
use std::sync::Arc;

/// Global event bus for in-process communication
#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(256);
        Self { tx }
    }

    pub fn publish(&self, event: SystemEvent) {
        let _ = self.tx.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.tx.subscribe()
    }
}

/// All system events
#[derive(Debug, Clone)]
pub enum SystemEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(String),
    ConfigChanged(String),          // Changed key path
    PanelVisible(bool),
    WorkspaceChanged(u32),
    NotificationReceived(Notification),
    ScreenLocked,
    ScreenUnlocked,
    SuspendRequested,
    ShutdownRequested,
    SessionLocked,
    SessionUnlocked,
    DisplayConfigChanged,
    DeviceAdded(String),
    DeviceRemoved(String),
}
```

### IPC Decision Matrix

| Communication | Method | Why |
|---------------|--------|-----|
| Panel ↔ Daemon | D-Bus | Separate processes, async |
| Panel ↔ Notifications | D-Bus | Separate processes |
| Panel ↔ Cinnamon | D-Bus | Cinnamon uses D-Bus |
| Panel ↔ Cinnamon Settings | GSettings via file | Batch reads |
| Panel ↔ Theme Engine | Direct (in-process) | Same process, low latency |
| Panel ↔ Launcher | Direct (same process) | Launcher is panel popover |
| Panel ↔ OSD | Direct | Same process, real-time |
| Settings → Panel | D-Bus | Separate process |
| Daemon → Panel | D-Bus signal | Event push |
| Config change events | Broadcast channel | In-process pub/sub |

### IPC Security

```rust
// D-Bus security: no authentication required on session bus
// But validate all inputs at service boundaries

impl EduShellService {
    fn set_panel_visible(&mut self, visible: bool) {
        // Validate: no injection possible for boolean
        self.panel_visible = visible;
    }

    fn open_url(&self, url: &str) {
        // Validate: sanitize URL before opening
        if !url.starts_with("https://") && !url.starts_with("http://") {
            log::warn!("Blocked non-HTTP URL from D-Bus: {}", url);
            return;
        }
        // ... open URL
    }
}
```

---

## 4. Module Loading Strategy

### Lazy Loading

| Module | Load Time | Why |
|--------|-----------|-----|
| Panel | Session start | Always needed |
| Launcher | On first open (preload CSS) | Save ~200ms startup |
| Quick Settings | On first click | Save ~50ms startup |
| OSD | Session start (invisible) | Must be ready instantly |
| Notifications | Session start | Daemon must run |
| Daemon | Session start | Background services |
| Settings | On user open | Separate process |
| Learning Hub | On user open | Separate process |

### Preloading Strategy

```rust
// Preload common resources at session start
pub struct ResourceCache {
    css_cache: HashMap<String, CssProvider>,
    icon_cache: HashMap<String, Texture>,
    font_cache: Vec<FontFamily>,
}

impl ResourceCache {
    pub fn preload_core() -> Self {
        // Preload common CSS (panel, launcher)
        // Preload commonly used icons
        // Preload fonts
        // Everything else loaded on demand
    }
}
```

---

## 5. Thread Model

```
┌─────────────────────────────────────────────────────────────┐
│                    edushell-panel process                    │
│                                                              │
│  Thread 1 (Main/GTK)                                        │
│  ┌──────────────────────────────┐                            │
│  │  GTK main loop              │  UI events, rendering      │
│  │  Panel widget updates       │  Widget signal handlers    │
│  │  Launcher rendering         │                            │
│  └──────────────────────────────┘                            │
│                                                              │
│  Thread 2 (Tokio worker)                                     │
│  ┌──────────────────────────────┐                            │
│  │  D-Bus message handling      │  zbus event loop           │
│  │  Config file watching        │  notify events             │
│  │  Background I/O              │                            │
│  └──────────────────────────────┘                            │
│                                                              │
│  Thread 3 (Optional: Search)                                 │
│  ┌──────────────────────────────┐                            │
│  │  Tantivy indexer             │  Heavy CPU work            │
│  │  File system scanning        │  Non-blocking to UI       │
│  └──────────────────────────────┘                            │
└─────────────────────────────────────────────────────────────┘
```

### Thread Safety Rules

1. **GTK widgets hanya disentuh dari thread utama**. Gunakan `glib::MainContext::channel()` untuk cross-thread UI updates.
2. **Tokio tasks boleh `Send`** — semua state dibungkus `Arc<Mutex<>>` atau `Arc<RwLock<>>`.
3. **ConfigManager thread-safe** — perubahan config dari D-Bus atau file watcher diamankan dengan `RwLock`.
4. **EventBus thread-safe** — broadcast channel adalah `Send + Sync`.
