# Startup, Session, Memory & Performance — EduShell

---

## 1. Startup Flow

### Complete Boot → Desktop Sequence

```
                    BOOT SEQUENCE TIMELINE
                    ═══════════════════════

  Power On
      │
      ▼  (BIOS/UEFI: ~3-5s)
  Bootloader (GRUB)
      │
      ▼  (Kernel init: ~2-4s)
  Linux Kernel
      │
      ▼  (systemd: ~2-3s)
  Display Manager (LightDM/SDDM)
      │
      ▼  (User login: ~1s)
  ┌─────────────────────────────────────────────────────────┐
  │             EDU SHELL SESSION START                     │
  │                                                         │
  │  T=0ms    Session Init                                  │
  │             ├── cinnamon-session starts                 │
  │             ├── Env: XDG, DBus, Wayland/X11 setup       │
  │             └── muffin (WM) starts                      │
  │                                                         │
  │  T=+500ms EduShell Daemon                               │
  │             ├── edushell-daemon starts                  │
  │             ├── Init logging, config                    │
  │             ├── Connect to D-Bus session bus            │
  │             ├── Start file watcher                      │
  │             └── Start search indexer (lazy)             │
  │                                                         │
  │  T=+800ms Theme & Localization                          │
  │             ├── Detect language                          │
  │             ├── Load translations                        │
  │             ├── Detect color scheme                      │
  │             ├── Load theme manifest                      │
  │             └── Preload core CSS                         │
  │                                                         │
  │  T=+1000ms Panel Process                                │
  │             ├── edushell-panel starts                   │
  │             ├── GTK Application init                    │
  │             ├── Wayland: wlr-layer-shell connect        │
  │             ├── Create panel window                     │
  │             ├── Load panel CSS                          │
  │             ├── Render panel widgets                    │
  │             ├── Start clock update timer                │
  │             └── Connect to D-Bus services               │
  │                                                         │
  │  T=+1500ms Launcher (lazy)                              │
  │             ├── CSS preloaded, not rendered             │
  │             └── Wait for first activation               │
  │                                                         │
  │  T=+1800ms Notification Service                         │
  │             ├── edushell-notifications starts           │
  │             ├── Register on D-Bus as notification daemon│
  │             └── Ready to receive notifications          │
  │                                                         │
  │  T=+2000ms OSD (lazy)                                   │
  │             ├── Preloaded, window hidden                │
  │             └── Wait for volume/brightness events       │
  │                                                         │
  │  T=+2500ms DESKTOP READY                                │
  │             └── User can start working                  │
  │                                                         │
  │  T=+5000ms Background tasks complete                    │
  │             ├── Search index ready (if indexer enabled) │
  │             └── Update check complete                   │
  └─────────────────────────────────────────────────────────┘
```

### Startup Code Architecture

```rust
// shell/edushell-panel/src/main.rs

fn main() {
    // 1. Install panic hook
    crash::install_panic_hook("edushell-panel");

    // 2. Parse CLI arguments
    let args = clap::Command::new("edushell-panel")
        .arg(clap::arg!(--wayland-socket <SOCKET>))
        .get_matches();

    // 3. Initialize async runtime
    let rt = EduRuntime::new();

    // 4. Initialize logging
    logging::init(None).expect("Failed to init logging");
    log::info!("EduShell Panel v{} starting...", env!("CARGO_PKG_VERSION"));

    // 5. Load configuration
    let config_mgr = ConfigManager::load().expect("Failed to load config");
    let config = config_mgr.current();

    // 6. Initialize localization
    localization::init(Some(&config.language.override_lang));

    // 7. Initialize theme engine
    let theme_mgr = Arc::new(ThemeManager::new(&config.theme));
    theme_mgr.apply(/* gtk_app */);

    // 8. Create GTK Application
    let app = gtk4::Application::builder()
        .application_id("org.edushell.panel")
        .build();

    // 9. Connect to Wayland layer shell
    let layer_shell = LayerShell::new(&app);

    // 10. Build panel UI
    app.connect_activate(move |app| {
        let panel = PanelWindow::new(app, &config_mgr, &theme_mgr);
        panel.show();
    });

    // 11. Run GTK main loop
    app.run();
}
```

### Startup Optimization Techniques

| Technique | Impact | Implementation |
|-----------|--------|----------------|
| **Lazy loading** | -400ms startup | Launcher, OSD, quick-settings loaded on demand |
| **Preload CSS** | -100ms first paint | Load panel CSS before GTK main loop |
| **Parallel init** | -500ms startup | Daemon, panel, notifications start in parallel |
| **Deferred indexing** | -300ms startup | Search index builds after desktop is ready |
| **Minimal imports** | -200ms startup | Only import needed modules at startup |
| **Static linking** | -100ms startup | Link core libraries statically (reduces runtime linking) |

### Startup Measurement

```rust
#[instrument]
fn measure_startup() {
    let start = std::time::Instant::now();

    // All startup steps are instrumented via tracing
    // Results logged at INFO level
    log::info!("Startup completed in {:?}", start.elapsed());
}

// To profile:
// EDUSHELL_PROFILE_STARTUP=1 cargo run
// This outputs detailed timing per phase
```

---

## 2. Session Lifecycle

### State Machine

```
                    SESSION LIFECYCLE
                    ═════════════════

                    ┌──────────────┐
                    │   Starting   │
                    │              │
                    │ Init phase   │
                    └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
              ┌────►│   Active     │◄────┐
              │     │              │     │
              │     │ Normal ops   │     │
              │     └──────┬───────┘     │
              │            │             │
              │    ┌───────┴───────┐     │
              │    │               │     │
              │    ▼               ▼     │
              │ ┌────────┐   ┌────────┐  │
              │ │ Locked │   │Suspend │  │
              │ │        │   │ (sleep)│  │
              │ │ Screen │   │        │  │
              │ │ locked │   │  RAM   │  │
              │ └────────┘   │  kept  │  │
              │    │         └────────┘  │
              │    │              │      │
              │    └──────────────┘      │
              │            │             │
              │            ▼             │
              │     ┌──────────────┐     │
              │     │  Unlocking   │─────┘
              │     │  Resuming    │
              │     └──────────────┘
              │
              │     ┌──────────────┐
              ├────►│  Shutting    │
              │     │  Down        │
              │     │              │
              │     │ Save state   │
              │     │ Terminate    │
              │     └──────────────┘
              │
              │     ┌──────────────┐
              └────►│  Switching   │
                    │  User        │
                    │              │
                    │ Fast User    │
                    │ Switch       │
                    └──────────────┘
```

### Session Events

```rust
// services/edushell-session/src/events.rs (placeholder for v3)
// For v1, handled by cinnamon-session via session-bridge

pub enum SessionEvent {
    /// Session starting
    Starting,
    /// Session active (desktop ready)
    Active,
    /// Lock screen requested
    LockRequested,
    /// Screen locked
    Locked,
    /// Screen unlocked
    Unlocked,
    /// Suspend requested (closing lid)
    SuspendRequested,
    /// System resuming from suspend
    Resumed,
    /// Switch to another user
    SwitchUser,
    /// Logout requested
    Logout,
    /// Shutdown requested
    Shutdown,
    /// Reboot requested
    Reboot,
}

/// v1: Bridge to cinnamon-session via D-Bus
impl SessionBridge {
    pub async fn handle_event(&self, event: SessionEvent) {
        match event {
            SessionEvent::Shutdown => {
                // 1. Save panel state
                self.save_session_state().await;
                // 2. Notify all components
                self.event_bus.publish(SystemEvent::ShutdownRequested);
                // 3. Delegate to cinnamon-session
                self.cinnamon_session.shutdown().await;
            }
            SessionEvent::LockRequested => {
                // 1. Blank screen
                // 2. Notify components
                self.event_bus.publish(SystemEvent::SessionLocked);
                // 3. Delegate to cinnamon-screensaver
                self.cinnamon_screensaver.lock().await;
            }
            // ... other events
        }
    }
}
```

### Session Configuration

```toml
# ~/.config/edushell/state.toml (auto-generated)

[last-session]
version = "1.0.0"
last-user = "adi"
last-shutdown = "2026-07-08T21:30:00+07:00"
last-workspace = 2

[restored-apps]
# Apps to auto-start on next login
apps = [
    "firefox.desktop",
    "libreoffice-writer.desktop",
]
```

---

## 3. Memory Strategy

### Target Memory Budget

```
EduShell Memory Budget (Idle)
═══════════════════════════════

Process                       RSS        Notes
─────────────────────────────────────────────────
edushell-panel               ~80MB      Main shell (GTK4, CSS, widgets)
  ├── panel widget            ~15MB
  ├── launcher (preloaded)     ~8MB     CSS cached, no window
  ├── OSD (preloaded)          ~5MB     Hidden window
  ├── quick-settings           ~5MB     CSS cached
  ├── GTK libraries           ~30MB     Shared, but accounted
  └── Core library (shared)   ~17MB     Config, logging, IPC

edushell-daemon               ~15MB     Background services
edushell-notifications        ~10MB     Notification daemon

Cinnamon (external)           ~150MB    Muffin, session, settings
  ├── muffin (WM)             ~60MB
  ├── cinnamon-session        ~5MB
  ├── cinnamon-settings-daemon ~30MB
  └── Other services          ~55MB

System services (shared)      ~200MB    NM, PulseAudio, UPower, etc.
─────────────────────────────────────────────────
Total (EduShell portion)      ~105MB    Our code only
Total (include Cinnamon)      ~255MB    Shell + WM + services
Total (full idle desktop)     ~455MB    + system services

TARGET IDLE: 500-650MB ✅
```

### Memory Optimization Techniques

| Technique | Saving | Implementation |
|-----------|--------|----------------|
| **Lazy widget creation** | ~10MB | Don't create widgets until needed |
| **CSS sharing** | ~5MB | Share CSS provider across widgets |
| **Icon cache limits** | ~10MB | Cap icon cache at 100 entries |
| **String interning** | ~3MB | Intern repeated strings |
| **Texture sharing** | ~8MB | Share textures between panel/launcher |
| **Drop caches on suspend** | ~20MB | Clear non-essential caches on suspend |
| **jemalloc allocator** | ~10MB | Better memory fragmentation than glibc |
| **Thin LTO** | ~5MB | Smaller binary, better inlining |

### Memory Profiling

```rust
/// Memory usage reporter (for diagnostics)
pub fn report_memory_usage() {
    use procfs::process::Process;

    if let Ok(proc) = Process::myself() {
        if let Ok(status) = proc.status() {
            log::info!("Memory: RSS={}kB, VM={}kB",
                status.rss_bytes() / 1024,
                status.vm_size / 1024);
        }
    }
}
```

### Heap Allocator

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

# Use jemalloc for better memory fragmentation characteristics
[dependencies]
tikv-jemallocator = "0.5"

# In src/main.rs:
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
```

---

## 4. Performance Strategy

### CPU Budget

```
CPU Budget (Idle on Celeron N3060)
═══════════════════════════════════

edushell-panel              < 0.5%     Clock update, D-Bus listeners
edushell-daemon             < 0.3%     File watcher, idle indexer
edushell-notifications      < 0.1%     D-Bus listener

Total EduShell idle CPU     < 1.0%     ✅ Target met
```

### Performance Optimization Rules

| Rule | Description |
|------|-------------|
| **No polling** | All I/O is event-driven (epoll, D-Bus signals, notify events) |
| **Debounce rapid events** | File changes, volume changes debounced at 100ms |
| **Batch D-Bus calls** | Group related settings changes into single D-Bus call |
| **Animation budget** | Max 16ms per frame (60fps). Use GTK's built-in animation framework |
| **CSS performance** | Avoid expensive selectors. Use class selectors over descendant selectors |
| **Async I/O** | All disk and network I/O uses tokio async |
| **Profile before optimize** | Measure with `perf` before making changes |

### Frame Budget (60fps)

```
Frame Budget @ 60fps
════════════════════

Total frame time:     16.67ms
├── GTK layout           2ms
├── CSS styling          1ms
├── Widget rendering     3ms
├── Animation update     1ms
├── D-Bus processing     2ms
├── Event handling       1ms
└── Buffer (safety)     ~6ms

Target: Keep frame time under 10ms on Celeron N3060
```

### Animation Performance

```rust
// Use GTK's timeline-based animation for smooth 60fps
impl PanelWindow {
    fn animate_panel_hide(&self) {
        let animation = gtk4::Timeline::new(200); // 200ms
        animation.add_callback(move |_, progress| {
            // progress: 0.0 → 1.0
            let opacity = 1.0 - progress;
            panel.set_opacity(opacity);
        });
        animation.start();
    }
}
```

### Resource Monitoring

```rust
/// Periodic health check (every 60 seconds when idle)
pub struct HealthMonitor {
    memory_threshold: u64,  // KB
    cpu_threshold: f64,     // percent
}

impl HealthMonitor {
    pub async fn check_health(&self) -> HealthReport {
        let memory = self.get_memory_usage().await;
        let cpu = self.get_cpu_usage().await;

        if memory > self.memory_threshold {
            log::warn!("Memory usage exceeds threshold: {}KB", memory);
            Self::suggest_cleanup();
        }

        if cpu > self.cpu_threshold {
            log::warn!("CPU usage exceeds threshold: {}%", cpu);
        }

        HealthReport { memory, cpu, healthy: memory <= self.memory_threshold }
    }
}
```

---

## 5. Startup Optimization Summary

| Phase | Target (ms) | Actual (ms) | Technique |
|-------|-------------|-------------|-----------|
| Kernel → DM | 3000 | — | systemd optimization |
| DM → Session | 1000 | — | LightDM fast user switching |
| Session init | 500 | — | cinnamon-session |
| Daemon init | 500 | — | Lazy indexer |
| Theme/Config | 300 | — | Load in parallel |
| Panel render | 500 | — | Minimal widgets first |
| Total to desktop | **~2500** | — | **Target: < 3s** |
