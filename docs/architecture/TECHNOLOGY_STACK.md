# Technology Stack — EduShell v1

## 1. Programming Language: Rust (default) + C (minimal FFI)

### Final Decision
**Rust** adalah bahasa default untuk seluruh komponen EduShell. C digunakan minimal untuk FFI bridging ke GLib/Cinnamon ecosystem saat binding Rust tidak tersedia.

### Rationale

| Criteria | Rust | C | Vala (previous) | C++ |
|----------|------|---|-----------------|-----|
| Memory safety | ✅ Compile-time | ❌ Manual | ⚠️ Ref-counted | ⚠️ Manual |
| Zero-cost abstractions | ✅ | ✅ | ❌ (GLib) | ✅ |
| Ecosystem (2026) | ✅ Large & growing | ✅ Mature | ❌ Declining | ✅ Mature |
| Cinnamon integration | ⚠️ Via FFI | ✅ Native | ✅ Native | ⚠️ Via FFI |
| Async support | ✅ (tokio) | ❌ Manual | ❌ (GLib main loop) | ✅ |
| Tooling (rustfmt, clippy) | ✅ Excellent | ⚠️ Basic | ⚠️ Basic | ⚠️ Partial |
| Cross-compilation | ✅ | ✅ | ❌ | ✅ |
| Package management | ✅ Cargo | ❌ Manual | ❌ (system pkgs) | ⚠️ Conan/vcpkg |
| Learning curve | ⚠️ Steep | ⚠️ Steep | ✅ Moderate | ⚠️ Steep |
| GTK4 support | ✅ gtk4-rs | ✅ native | ✅ native | ⚠️ gtkmm4 |

### Mengapa Rust Menggantikan Vala dari Bagian 1
1. **Keamanan memori**: Rust menjamin tidak ada use-after-free, double-free, atau buffer overflow — kritis untuk desktop shell yang harus uptime 30+ hari.
2. **Kinerja**: Zero-cost abstractions cocok untuk komponen real-time seperti search index, file watcher, dan animasi shell.
3. **Ecosystem**: Rust memiliki library untuk hampir semua kebutuhan: D-Bus (zbus), Wayland (wayland-rs), search (tantivy), async (tokio).
4. **Jangka panjang**: Rust adalah investasi untuk v4/v5 saat kita build window manager sendiri.
5. **Kontributor**: Komunitas Rust di Indonesia aktif dan berkembang.
6. **Tooling**: Cargo, rustfmt, clippy, dan rust-analyzer memberikan developer experience kelas enterprise.

### Komponen yang Tetap Menggunakan Bahasa Lain

| Komponen | Bahasa | Alasan |
|----------|--------|--------|
| **GTK4 widget CSS styling** | CSS | GTK4 native rendering |
| **Build scripts & CI glue** | Python/bash | Ecosystem tooling |
| **GSettings schemas** | XML | GLib requirement |
| **Cinnamon adapter FFI** | C | Minimal FFI untuk GLib struct |
| **Learning Hub content** | HTML/CSS/JS | Static content, ringan |

---

## 2. GUI Toolkit: GTK4 via gtk4-rs

### Final Decision
**GTK4** (gtk4 crate) sebagai GUI toolkit utama. **libadwaita-rs** sebagai opsi untuk komponen settings (dapat di-drop).

### Perbandingan Toolkit

| Toolkit | Bahasa | Desktop Shell | Wayland | Acc | Theme | Cinnamon | Memory | Maturity |
|---------|--------|--------------|---------|-----|-------|----------|--------|----------|
| **GTK4-rs** | Rust | ✅ Sesuai | ✅ Native | ✅ AT-SPI2 | ✅ CSS | ✅ Compat | ~15MB | ✅ Stabil |
| **libadwaita-rs** | Rust | ⚠️ (apps) | ✅ | ✅ | ✅ | ⚠️ | ~20MB | ✅ |
| **Qt6 (qmetaobject)** | Rust/C++ | ✅ | ✅ | ⚠️ | ✅ QSS | ❌ | ~25MB | ⚠️ |
| **egui** | Rust | ❌ (immediate) | ✅ | ❌ | ✅ Custom | ❌ | ~2MB | ⚠️ |
| **iced** | Rust | ⚠️ (promising) | ✅ | ❌ | ✅ | ❌ | ~5MB | ⚠️ Beta |
| **Slint** | Rust | ⚠️ | ✅ | ⚠️ | ✅ | ❌ | ~3MB | ⚠️ |
| **Tauri** | Rust/Web | ❌ (WebView) | ✅ | ⚠️ | ✅ CSS | ❌ | ~100MB+ | ✅ |
| **Druid** | Rust | ❌ | ✅ | ❌ | ✅ | ❌ | ~4MB | ❌ Experimental |

### Mengapa GTK4-rs Menang

1. **Cinnamon Compatibility** — Syarat mutlak. Cinnamon menggunakan GTK. Tanpa GTK, tidak ada integrasi theming, system tray, aksesibilitas, atau session management.
2. **Accessibility** — AT-SPI2 terintegrasi secara native. WCAG AA compliance tercapai tanpa kerja tambahan.
3. **CSS Theming** — Theme engine GTK memungkinkan light/dark mode, accent color, dan kustomisasi penuh tanpa rekompilasi.
4. **Wayland** — GTK4 memiliki Wayland backend native dengan dukungan wlr-layer-shell.
5. **Maturity** — gtk4-rs telah production-ready, digunakan oleh GNOME dan aplikasi Rust seperti Fractal, NewsFlash.
6. **Memory** — GTK4 lebih ringan dari Qt6, terutama untuk komponen shell sederhana.

### Tradeoffs & Mitigasi

| Tradeoff | Mitigasi |
|----------|----------|
| GObject paradigm tidak idiomatic di Rust | Gunakan pattern wrapper; batasi GObject langsung di layer adapter |
| Binary size lebih besar (~10-15MB GTK) | GTK shared library; strip binary; LTO |
| Build time lebih lama | sccache; Cargo workspace optimization |
| GTK dependency besar | Dynamic linking; minimal GTK features |

### Layout Approach

| Component | Approach |
|-----------|----------|
| Panel (bar) | GTK4 Window with wlr-layer-shell (panel layer) |
| Launcher | GTK4 Popover/Window (overlay) |
| Settings app | GTK4 ApplicationWindow with libadwaita |
| Notifications | GTK4 Popover + Notification widgets |
| Quick Settings | GTK4 Popover |
| OSD | GTK4 Window (overlay layer) |

---

## 3. Display Server Strategy

### Final Decision
**Wayland** primer + **X11** fallback, sama seperti keputusan Bagian 1.

### Wayland Protocols

| Protocol | Crate | Purpose | Required |
|----------|-------|---------|----------|
| wlr-layer-shell | wayland-rs | Panel positioning | ✅ v1 |
| wlr-foreign-toplevel | wayland-rs | Window management | ✅ v1 |
| xdg-shell | gtk4-rs (built-in) | Client windows | ✅ v1 |
| zwlr-screencopy | wayland-rs | Screenshot (future) | 📋 v2 |
| wp-fractional-scale | wayland-rs | Per-monitor scaling | 📋 v2 |

### Detection & Fallback Strategy

```
┌─────────────────────────────────────────┐
│ Display Server Detection                │
│                                         │
│ env::var("WAYLAND_DISPLAY") = Ok(s)  → Wayland Mode
│ env::var("DISPLAY") = Ok(s)          → X11 Fallback
│ neither                              → Error (no display)
└─────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────┐
│ Wayland Mode                            │
│                                         │
│ gdk = GdkDisplay::open("wayland-0")     │
│   if wlr-layer-shell available:         │
│     → Full feature set                  │
│   else:                                 │
│     → Basic feature set (no panel pos)  │
│     → Log warning                       │
└─────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────┐
│ X11 Fallback Mode                       │
│                                         │
│ gdk = GdkDisplay::open(":0")            │
│   → Basic feature set                   │
│   → No per-monitor scaling              │
│   → Log deprecation notice              │
└─────────────────────────────────────────┘
```

### Architecture Decision Record
Display server strategy didokumentasikan di `docs/adr/ADR-003-wayland-x11.md`. Tidak ada perubahan.

---

## 4. Build System: Cargo (workspace) + xtask pattern

### Final Decision
**Cargo workspace** sebagai primary build system. **Meson** digunakan hanya untuk GSettings schemas dan data file installation. **xtask** pattern untuk automation.

### Perbandingan

| System | Rust Support | Speed | Flexibility | Cinnamon Compat |
|--------|-------------|-------|-------------|-----------------|
| **Cargo workspace** | ✅ Native | ✅ Fast | ✅ High | ⚠️ (via build.rs) |
| Meson | ⚠️ (limited) | ✅ Fast | ✅ High | ✅ Native |
| CMake | ⚠️ (Corrosion) | ⚠️ | ✅ High | ⚠️ |
| Makefile | ❌ Manual | ✅ | ❌ Low | ⚠️ |
| Just | ⚠️ (runner) | ✅ | ✅ | ❌ |

### Build Architecture

```
Repository Root
│
├── Cargo.toml                    # Workspace definition
├── Cargo.lock                    # Lockfile (committed)
│
├── shell/panel/Cargo.toml        # Panel component
├── shell/launcher/Cargo.toml     # Launcher component
├── shell/workspace/Cargo.toml    # Workspace component
├── apps/settings/Cargo.toml      # Settings app
├── apps/learning-hub/Cargo.toml  # Learning Hub
├── core/edushell-core/Cargo.toml # Core library
├── core/cinnamon-adapter/Cargo.toml
├── core/theme-engine/Cargo.toml
│
├── data/                         # GSettings schemas
│   └── meson.build               # Meson for data files only
│
├── xtask/                        # Build automation
│   ├── Cargo.toml
│   └── src/main.rs               # Custom build tasks
│
└── Makefile                      # Top-level convenience
```

### Cargo.toml (workspace root)

```toml
[workspace]
resolver = "2"
members = [
    "core/*",
    "shell/*",
    "apps/*",
    "services/*",
    "xtask",
]
```

### Build Commands

```bash
# Development
cargo build                        # Build all
cargo build -p edushell-panel      # Build specific component
cargo run -p edushell-settings     # Run specific app

# Testing
cargo test                         # All tests
cargo test -p edushell-core        # Specific package
cargo clippy --all-targets         # Lint
cargo fmt --check                  # Format check

# Release
cargo build --release
cargo xtask dist                   # Build distribution package
cargo xtask deb                    # Build .deb package
```

### Meson (for data files only)

```meson
project('edushell-data', 'c',
  version: '1.0.0',
  meson_version: '>= 1.0.0'
)

# GSettings schemas
install_data(
  'org.edushell.shell.gschema.xml',
  'org.edushell.settings.gschema.xml',
  install_dir: join_paths(get_option('datadir'), 'glib-2.0', 'schemas')
)

# Desktop files
install_data(
  'edushell-session.desktop',
  install_dir: join_paths(get_option('datadir'), 'xsessions')
)
```

### Why Not Pure Meson
Meson's Rust support (since 0.62) is functional but limited:
- No workspace support
- No crate registry integration
- Manual dependency specification
- Slower incremental compilation

Cargo is simply better for Rust development. Using Meson for data files only keeps compatibility with distribution packaging while giving us full Cargo ecosystem benefits.

---

## 5. Async Runtime: Tokio + glib-rs Main Loop

### Final Decision
**Tokio** (multi-threaded) untuk background services dan async I/O. **glib-rs Main Loop** untuk GTK thread.

### Architecture

```
┌──────────────────┐     ┌──────────────────┐
│  GTK Main Loop   │     │  Tokio Runtime   │
│  (glib-rs)       │     │  (multi-thread)  │
│                  │     │                  │
│  - UI rendering  │     │  - D-Bus IPC     │
│  - User input    │     │  - File watching │
│  - Widget update │     │  - Search index  │
│  - Timer         │     │  - Network I/O   │
│                  │     │  - Plugin mgmt   │
└───────┬──────────┘     └───────┬──────────┘
        │                        │
        └────────────┬───────────┘
                     │
        glib::MainContext::channel()
        tokio::sync::mpsc
```

### Channel Bridge

```rust
// src/lib/edushell-core/src/runtime.rs

use glib::MainContext;
use tokio::runtime::Runtime;

pub struct EduRuntime {
    gtk_ctx: MainContext,
    tokio_rt: Runtime,
}

impl EduRuntime {
    pub fn new() -> Self {
        let tokio_rt = Runtime::new().expect("Failed to create tokio runtime");
        let gtk_ctx = MainContext::ref_thread_default();
        Self { gtk_ctx, tokio_rt }
    }

    /// Spawn async task on tokio, result sent to GTK thread
    pub fn spawn_with_callback<F, R>(&self, future: F, callback: impl FnOnce(R) + 'static)
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let (sender, receiver) = MainContext::channel::<R>(glib::PRIORITY_DEFAULT);
        receiver.attach(None, move |result| {
            callback(result);
            glib::ControlFlow::Break
        });
        self.tokio_rt.spawn(async move {
            let result = future.await;
            let _ = sender.send(result);
        });
    }
}
```

---

## 6. IPC: D-Bus (zbus) + Rust Channels

### Final Decision
**D-Bus** via `zbus` crate untuk komunikasi antar proses. **tokio::sync** channels untuk komunikasi internal.

### Perbandingan IPC

| Method | Latency | Throughput | Language | Complexity | Use Case |
|--------|---------|------------|----------|------------|----------|
| **D-Bus (zbus)** | ~1ms | Medium | Any | Medium | Service IPC, Cinnamon compat |
| UNIX Socket | ~0.1ms | High | Any | Medium | Plugin communication |
| Shared Memory | ~0.01ms | Very High | Same lang | High | Performance-critical |
| gRPC | ~1-5ms | High | Any | High | Overkill for desktop |
| Named Pipe | ~0.1ms | Medium | Any | Low | Simple IPC |
| Rust channels | ~0.001ms | Very High | Rust only | Low | Internal module comm |

### Mengapa D-Bus

1. **Cinnamon menggunakan D-Bus** — Semua komunikasi dengan Cinnamon services via D-Bus.
2. **Standar Linux Desktop** — NetworkManager, PulseAudio, UPower, logind semuanya via D-Bus.
3. **zbus mature** — zbus adalah D-Bus implementation Rust terbaik, active maintenance.
4. **Type-safe** — zbus menyediakan type-safe interface via derive macros.
5. **Async-ready** — zbus integrated dengan tokio.

### IPC Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Session Bus                        │
│              (DBus, via zbus)                        │
│                                                      │
│  org.edushell.Shell         org.edushell.Settings    │
│  org.edushell.Notifications org.edushell.Search      │
│  org.edushell.Session       org.edushell.Theme       │
│                                                      │
│  (Cinnamon services via existing DBus interfaces)    │
│  org.cinnamon.ScreenSaver   org.cinnamon.SessionMgr  │
│                                                      │
│  (System services)                                   │
│  org.freedesktop.NetworkManager                      │
│  org.freedesktop.UPower                              │
│  org.freedesktop.login1                              │
└─────────────────────────────────────────────────────┘
        ▲                        ▲
        │ D-Bus                  │ Rust channels
        ▼                        ▼
┌───────────────┐     ┌─────────────────┐
│  edushell-    │     │  Internal Module │
│  panel        │◄───►│  Communication   │
│               │     │  (channels)      │
│  Background   │     │                  │
│  Services     │     │  Panel → OSDSignal│
│  Daemon       │     │  Search → Panel  │
└───────────────┘     └─────────────────┘
```

### D-Bus Interface Definition

```rust
// src/lib/edushell-core/src/ipc/shell.rs

use zbus::interface;

pub struct EduShellService {
    panel_visible: bool,
    theme_mode: String,
}

#[interface(name = "org.edushell.Shell")]
impl EduShellService {
    /// Set panel visibility
    fn set_panel_visible(&mut self, visible: bool) {
        self.panel_visible = visible;
    }

    /// Get current theme mode
    fn theme_mode(&self) -> String {
        self.theme_mode.clone()
    }

    /// Signal: theme changed
    #[zbus(signal)]
    async fn theme_changed(signal_ctxt: &SignalContext<'_>, mode: &str) -> zbus::Result<()>;
}
```

---

## 7. Configuration System: TOML + serde + Watch

### Final Decision
**TOML** format via **serde** dengan file watcher. Tidak menggunakan GSettings untuk independence jangka panjang. GSettings bridge untuk Cinnamon compatibility.

### Perbandingan Format

| Format | serde | Type Safety | Comments | Human Readable | Standard |
|--------|-------|-------------|----------|----------------|----------|
| **TOML** | ✅ | ✅ Strict | ✅ | ✅ | ✅ (RFC) |
| JSON | ✅ | ⚠️ No comments | ❌ | ⚠️ | ✅ (RFC) |
| YAML | ✅ | ⚠️ No types | ✅ | ✅ | ⚠️ (complex) |
| RON | ✅ | ✅ | ✅ | ✅ | ❌ (Rust-only) |
| INI | ⚠️ | ❌ | ✅ | ✅ | ❌ |
| GSettings | ❌ | ⚠️ | ❌ | ❌ | GNOME-only |

### File Locations (XDG)

| Path | Purpose |
|------|---------|
| `~/.config/edushell/config.toml` | Main user configuration |
| `~/.config/edushell/keybindings.toml` | Keyboard shortcuts |
| `~/.config/edushell/state.toml` | Session state (restored on login) |
| `~/.local/share/edushell/` | Application data |
| `~/.cache/edushell/` | Cache data |
| `~/.local/share/edushell/logs/` | Log files |
| `/usr/share/edushell/defaults.toml` | System defaults (read-only) |

### Configuration Schema Design

```rust
// src/lib/edushell-core/src/config/mod.rs

use serde::{Deserialize, Serialize};

/// Root configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EduConfig {
    pub version: String,           // Schema version for migration
    pub shell: ShellConfig,
    pub launcher: LauncherConfig,
    pub workspace: WorkspaceConfig,
    pub theme: ThemeConfig,
    pub accessibility: AccessibilityConfig,
    pub language: LanguageConfig,
    pub shortcuts: ShortcutsConfig,
    pub notifications: NotificationsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub panel_position: PanelPosition,  // Bottom, Top, Left, Right
    pub panel_autohide: bool,
    pub panel_autohide_delay_ms: u32,   // default 500
    pub panel_opacity: f64,             // 0.0 - 1.0
    pub workspace_count: u32,           // 1-32
    pub show_favorites: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanelPosition {
    Bottom,
    Top,
    Left,
    Right,
}

/// ... other config structs ...
```

### Configuration Manager

```rust
// src/lib/edushell-core/src/config/manager.rs

use notify::{Event, RecursiveMode, Watcher};
use std::path::PathBuf;

pub struct ConfigManager {
    path: PathBuf,
    current: EduConfig,
    watcher: notify::RecommendedWatcher,
    change_tx: tokio::sync::broadcast::Sender<ConfigEvent>,
}

impl ConfigManager {
    /// Load config with automatic migration
    pub fn load() -> Result<Self, ConfigError> { ... }

    /// Save current config to file
    pub fn save(&self) -> Result<(), ConfigError> { ... }

    /// Watch for external changes and broadcast
    pub fn watch(&mut self) -> Result<(), ConfigError> { ... }

    /// Migrate from older schema version
    fn migrate(config: &mut EduConfig) -> Result<(), ConfigError> { ... }

    /// Export config for backup
    pub fn export(&self) -> Result<String, ConfigError> { ... }

    /// Import config from backup
    pub fn import(&mut self, data: &str) -> Result<(), ConfigError> { ... }

    /// Reset to factory defaults
    pub fn reset(&mut self) -> Result<(), ConfigError> { ... }

    /// Create backup before mutation
    fn backup(&self) -> Result<(), ConfigError> { ... }
}
```

### Migration Strategy

```
Schema Version 1 (v1.0) → Schema Version 2 (v1.1)
     │                          │
     └── ConfigManager::load()
              │
              ├── Read file
              ├── Parse config
              ├── Check version
              ├── If version < current:
              │     ├── Backup old file
              │     ├── Apply migration v1→v2
              │     ├── Update version field
              │     └── Save migrated config
              └── Return config
```

### GSettings Bridge (for Cinnamon compatibility)

```rust
// src/lib/cinnamon-adapter/src/settings_bridge.rs

use glib::Variant;

/// Bridge EduShell TOML config ↔ Cinnamon GSettings
pub struct SettingsBridge {
    gsettings: Map<String, glib::Settings>,
}

impl SettingsBridge {
    /// Sync EduShell config to Cinnamon GSettings for compatibility
    pub fn sync_to_cinnamon(&self, config: &EduConfig) { ... }

    /// Read from Cinnamon GSettings (fallback if EduShell key missing)
    pub fn read_cinnamon(&self, schema: &str, key: &str) -> Option<Variant> { ... }
}
```

---

## 8. Dependency Management

### Strategy
1. **Minimal dependencies**: Every crate must justify its existence.
2. **Pin versions**: `Cargo.lock` committed for reproducible builds.
3. **Audit regularly**: `cargo audit` in CI.
4. **Prefer pure Rust**: Avoid C FFI dependencies where possible.
5. **License check**: `cargo-deny` ensures GPL-3.0 compatible licenses.

### Core Dependencies (v1)

| Crate | Purpose | License | Justification |
|-------|---------|---------|---------------|
| gtk4 | GUI toolkit | LGPL-2.1+ | Desktop shell requirement |
| gtk4-layer-shell | Panel positioning on Wayland | LGPL-2.1+ | Essential for Wayland panel |
| glib-rs | GLib bindings | MIT | GTK dependency |
| zbus | D-Bus IPC | MIT | Inter-process communication |
| tokio | Async runtime | MIT | Background services |
| serde + toml | Configuration | MIT/Apache-2.0 | Config serialization |
| tracing + tracing-subscriber | Logging | MIT | Structured logging |
| notify | File watcher | CC0-1.0 | Config hot-reload |
| tantivy | Search engine | MIT | File/app search (future) |
| image | Image loading | MIT/Apache-2.0 | Wallpaper handling |
| css-color | Color parsing | MIT | Theme engine |
| chrono | Date/time | MIT/Apache-2.0 | Clock widget |
| clap | CLI argument parsing | MIT/Apache-2.0 | Daemon CLI |
| human-panic | Panic handler | MIT | Crash reporting |
| cargo-deny | License audit | Apache-2.0 | CI only |

### Dev Dependencies

| Crate | Purpose |
|-------|---------|
| criterion | Benchmarking |
| mockall | Mocking for tests |
| tempfile | Temp directories in tests |

### Dependency Audit Playbook

```yaml
# .github/workflows/deps.yml
name: Dependency Audit
on: [push, pull_request]
jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/audit@v1
      - uses: EmbarkStudios/cargo-deny-action@v1
        with:
          log-level: error
          command: check
```

---

## 9. Error Handling Strategy

### Pattern
```rust
// src/lib/edushell-core/src/error.rs

use std::fmt;
use std::path::PathBuf;

/// Unified error type for all EduShell components
#[derive(Debug)]
pub enum EduError {
    Config(ConfigError),
    Io(std::io::Error),
    Dbus(zbus::Error),
    Gtk(gtk4::glib::BoolError),
    Theme(ThemeError),
    NotFound { what: String, context: String },
    External { source: Box<dyn std::error::Error + Send> },
}

/// Component-specific errors
#[derive(Debug)]
pub enum ConfigError {
    Parse { path: PathBuf, detail: String },
    Migrate { from: String, to: String, detail: String },
    Write { path: PathBuf, source: std::io::Error },
    Validate { field: String, reason: String },
}

/// Result alias
pub type EduResult<T> = Result<T, EduError>;
```

### Error Handling Principles
1. **All errors typed**: No `String` errors. Every error has context.
2. **User-facing errors**: Friendly, non-technical, actionable.
3. **Developer-facing errors**: Include file, line, backtrace.
4. **No unwrap in production**: `.expect()` only in tests.
5. **Defensive boundaries**: Validate at module boundaries.

---

## 10. Summary: Technology Stack Table

| Category | Choice | Version | Justification |
|----------|--------|---------|---------------|
| Language | Rust | 1.80+ (2024 edition) | Safety, performance, ecosystem |
| GUI | GTK4 (gtk4-rs) | 4.12+ | Cinnamon compat, acc, theming |
| Shell Layer | gtk4-layer-shell | 0.8+ | Wayland panel positioning |
| IPC | D-Bus (zbus) | 4.0+ | Standard, Cinnamon compatible |
| Async | Tokio | 1.x | Mature, performant |
| Config | TOML + serde | — | Independent, typed, human-readable |
| Logging | tracing | 0.1 | Structured, async-aware |
| Build | Cargo workspace | — | Native Rust, fast |
| Packaging helper | Meson | 1.0+ | GSettings schemas, data files |
| Crash handler | human-panic | 1.x | User-friendly crash reports |
| File watch | notify | 7.x | Config hot-reload |
| Search (future v2) | tantivy | 0.22 | Pure Rust search engine |
| Async GTK bridge | glib-rs | 0.20 | GTK main loop integration |
| CLI | clap | 4.x | Argument parsing |
| Automation | xtask | — | Custom build tasks |
