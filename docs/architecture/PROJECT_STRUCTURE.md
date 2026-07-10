# Project Structure & Source Code Layout — EduShell

## 1. Repository Structure

```
edushell/
│
├── Cargo.toml                          # Workspace root
├── Cargo.lock                          # Dependency lockfile (committed)
├── clippy.toml                         # Clippy lint configuration
├── rustfmt.toml                        # Rust formatting config
├── deny.toml                           # cargo-deny license/config
├── .cargo/config.toml                  # Cargo config (Rustflags, etc.)
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                      # Full CI pipeline
│   │   ├── release.yml                 # Release build & publish
│   │   ├── audit.yml                   # Security/dependency audit
│   │   └── docs.yml                    # Documentation deploy
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   ├── feature_request.md
│   │   └── config.yml
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── CODEOWNERS
│
├── docs/                               # All documentation
│   ├── README.md                       # Documentation index
│   ├── architecture/                   # Architecture documents (Part 1 + 2)
│   │   ├── TECHNOLOGY_STACK.md         # This document set
│   │   ├── PROJECT_STRUCTURE.md
│   │   ├── MODULE_SERVICE_ARCHITECTURE.md
│   │   ├── CONFIGURATION_THEME_LOCALIZATION.md
│   │   ├── PLUGIN_LOGGING_CRASH.md
│   │   ├── STARTUP_SESSION_PERFORMANCE.md
│   │   ├── SECURITY_TESTING_CICD.md
│   │   ├── STANDARDS_RELEASE.md
│   │   └── PREREQUISITES_CHECKLIST.md
│   ├── adr/                            # Architecture Decision Records
│   ├── guides/                         # User & developer guides
│   ├── specs/                          # Technical specifications
│   └── standards/                      # Coding standards
│
├── core/                               # Core libraries (Layer 3: Abstraction)
│   ├── Cargo.toml                      # Workspace member
│   │
│   ├── edushell-core/                  # Core library: config, logging, error, IPC
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config/
│   │       │   ├── mod.rs              # Public API
│   │       │   ├── manager.rs          # ConfigManager
│   │       │   ├── schema.rs           # Typed config structs
│   │       │   ├── migration.rs        # Version migration
│   │       │   └── watch.rs            # File watcher
│   │       ├── error/
│   │       │   ├── mod.rs              # EduError enum
│   │       │   └── macros.rs           # Error convenience macros
│   │       ├── logging/
│   │       │   ├── mod.rs              # Logging setup
│   │       │   └── format.rs           # Log formatting
│   │       ├── ipc/
│   │       │   ├── mod.rs              # IPC types
│   │       │   ├── dbus.rs             # D-Bus connection manager
│   │       │   └── signals.rs           # Internal signals
│   │       ├── runtime/
│   │       │   ├── mod.rs              # Runtime manager (tokio + glib)
│   │       │   └── channel.rs          # Cross-runtime channels
│   │       ├── utils/
│   │       │   ├── mod.rs
│   │       │   ├── xdg.rs              # XDG path resolution
│   │       │   ├── time.rs             # Time utilities
│   │       │   └── fs.rs               # Safe file I/O
│   │       └── crash/
│   │           ├── mod.rs              # Crash handler setup
│   │           ├── panic.rs            # Panic hook
│   │           └── recovery.rs         # Recovery logic
│   │
│   ├── cinnamon-adapter/               # Cinnamon API abstraction
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── session.rs              # cinnamon-session bridge
│   │       ├── background.rs           # Wallpaper/background
│   │       ├── keybindings.rs          # Cinnamon keybinding compat
│   │       ├── settings.rs             # GSettings bridge
│   │       └── dbus.rs                 # Cinnamon DBus proxies
│   │
│   ├── theme-engine/                   # Theme management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── manager.rs              # ThemeManager
│   │       ├── css.rs                  # CSS generation & parsing
│   │       ├── color.rs                # Color scheme processing
│   │       ├── icon.rs                 # Icon theme lookup
│   │       ├── wallpaper.rs            # Wallpaper handling
│   │       ├── font.rs                 # Font configuration
│   │       └── gtk.rs                  # GTK CSS provider integration
│   │
│   └── localization/                   # Translation/i18n
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── manager.rs              # LocaleManager
│           ├── gettext.rs              # gettext binding
│           ├── loader.rs               # .mo file loader
│           └── lang.rs                 # Language enum & detection
│
├── shell/                              # Shell components (Layer 1: UI)
│   ├── Cargo.toml
│   │
│   ├── edushell-panel/                 # Main panel binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs                 # Entry point
│   │       ├── app.rs                  # Application setup
│   │       ├── panel.rs                # PanelWindow (wlr-layer-shell)
│   │       ├── widget/
│   │       │   ├── mod.rs
│   │       │   ├── menu_button.rs      # Edu menu button
│   │       │   ├── task_list.rs        # Running app taskbar
│   │       │   ├── system_tray.rs      # System tray area
│   │       │   ├── clock.rs            # Clock widget
│   │       │   └── workspace_switcher.rs
│   │       └── style.css               # Panel CSS
│   │
│   ├── edushell-launcher/              # Application launcher
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── launcher.rs             # Main launcher window/popover
│   │       ├── search.rs               # Search engine integration
│   │       ├── app_grid.rs             # App grid view
│   │       ├── favorites.rs            # Favorites manager
│   │       ├── recent.rs               # Recent apps
│   │       ├── categories.rs           # Category filtering
│   │       └── style.css
│   │
│   ├── edushell-notifications/         # Notification center
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── service.rs              # Notification daemon (DBus)
│   │       ├── popup.rs                # Popup notification widget
│   │       ├── center.rs               # Notification history panel
│   │       ├── manager.rs              # NotificationManager
│   │       └── style.css
│   │
│   ├── edushell-osd/                   # On-screen display
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── osd.rs                  # OSD window
│   │       ├── volume.rs               # Volume indicator
│   │       ├── brightness.rs           # Brightness indicator
│   │       └── style.css
│   │
│   └── edushell-quick-settings/        # Quick settings popup
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── quick_settings.rs       # Quick settings panel
│           ├── toggles.rs              # WiFi, BT, DND, dark mode
│           ├── sliders.rs              # Volume, brightness
│           └── style.css
│
├── apps/                               # Application binaries (Layer 2: Apps)
│   ├── Cargo.toml
│   │
│   ├── edushell-settings/              # Edu Settings application
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── window.rs               # Main settings window
│   │       ├── sidebar.rs              # Navigation sidebar
│   │       ├── pages/
│   │       │   ├── mod.rs
│   │       │   ├── panel.rs
│   │       │   ├── launcher.rs
│   │       │   ├── theme.rs
│   │       │   ├── language.rs
│   │       │   ├── accessibility.rs
│   │       │   ├── shortcuts.rs
│   │       │   └── about.rs
│   │       └── style.css
│   │
│   └── edushell-learning-hub/          # Learning Hub
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── window.rs
│           ├── content_loader.rs       # Load markdown/HTML content
│           ├── search.rs               # Search within Learning Hub
│           └── content/                # Static content files
│               ├── id/
│               │   ├── index.md
│               │   ├── getting-started.md
│               │   └── tips.md
│               └── en/
│                   ├── index.md
│                   ├── getting-started.md
│                   └── tips.md
│
├── services/                           # Background services (daemon)
│   ├── Cargo.toml
│   │
│   ├── edushell-daemon/                # Main background daemon
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── service.rs              # Service lifecycle
│   │       ├── dbus_service.rs         # D-Bus service host
│   │       └── modules/                # Service sub-modules
│   │           ├── mod.rs
│   │           ├── indexer.rs          # File search indexer
│   │           ├── watcher.rs          # File system watcher
│   │           └── updater.rs          # Update checker
│   │
│   └── edushell-session/               # Session manager (future v3)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs                 # v3+ replacement
│           └── (placeholder for now)
│
├── data/                               # Data files (installed to system)
│   ├── schemas/
│   │   ├── org.edushell.shell.gschema.xml
│   │   ├── org.edushell.settings.gschema.xml
│   │   └── org.edushell.theme.gschema.xml
│   ├── desktop/
│   │   ├── edushell-session.desktop
│   │   └── xdg-desktop-portal-edushell.desktop
│   ├── icons/
│   │   └── hicolor/
│   │       ├── scalable/
│   │       └── symbolic/
│   ├── wallpapers/
│   │   ├── edushell-default-light.png
│   │   └── edushell-default-dark.png
│   └── sounds/
│       ├── startup.ogg
│       ├── notification.ogg
│       └── shutdown.ogg
│
├── po/                                 # Translation files
│   ├── POTFILES.in
│   ├── edushell.pot
│   ├── id.po                          # Indonesian
│   └── en.po                          # English
│
├── tests/                              # Integration & system tests
│   ├── Cargo.toml                      # Test-only workspace member
│   ├── integration/
│   │   ├── panel_lifecycle_test.rs
│   │   ├── launcher_search_test.rs
│   │   ├── config_persistence_test.rs
│   │   └── dbus_interface_test.rs
│   ├── benchmarks/
│   │   ├── launcher_search_bench.rs
│   │   └── config_load_bench.rs
│   └── fixtures/
│       ├── test_config.toml
│       └── test_theme/
│
├── scripts/                            # Build & dev scripts
│   ├── setup-dev.sh                    # Dev environment setup
│   ├── build-full.sh                   # Full build script
│   ├── run-integration-tests.sh
│   └── gen-translation.sh              # Translation helper
│
├── xtask/                              # Cargo xtask automation
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                     # xtask entry
│       ├── dist.rs                     # Distribution package
│       ├── deb.rs                      # .deb packaging
│       └── ci.rs                       # CI helper tasks
│
├── .devcontainer/                      # Dev container (VS Code / Cursor)
│   ├── devcontainer.json
│   └── Dockerfile
│
├── .editorconfig                       # Editor settings
├── .gitignore
├── .pre-commit-config.yaml            # Pre-commit hooks
├── README.md
├── LICENSE                             # GPL-3.0-or-later
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
└── AUTHORS.md
```

## 2. Directory Justification

| Directory | Purpose | Layer | Language |
|-----------|---------|-------|----------|
| `core/` | Shared libraries: config, error, IPC, logging, theme, i18n | Abstraction | Rust |
| `shell/` | UI shell components: panel, launcher, notifications, OSD | Shell | Rust + GTK4 |
| `apps/` | Application binaries: settings, learning hub | Apps | Rust + GTK4 |
| `services/` | Background daemons: indexer, watcher, session | Services | Rust |
| `data/` | System data files: schemas, icons, desktop files | — | Various |
| `po/` | Translation files: .pot, .po | — | gettext |
| `tests/` | Integration tests, benchmarks, fixtures | — | Rust |
| `scripts/` | Developer shell scripts | — | Bash/Python |
| `xtask/` | Cargo task automation | — | Rust |
| `.github/` | CI/CD workflows, templates | — | YAML |

## 3. Dependency Graph (High Level)

```
edushell-panel (binary)
    │
    ├── edushell-core (lib)
    │   ├── serde + toml (config)
    │   ├── tracing (logging)
    │   ├── zbus (D-Bus IPC)
    │   ├── tokio (async runtime)
    │   ├── notify (file watcher)
    │   └── human-panic (crash)
    │
    ├── cinnamon-adapter (lib)
    │   ├── edushell-core
    │   └── glib-rs + gio-rs
    │
    ├── theme-engine (lib)
    │   ├── edushell-core
    │   └── gtk4 + css-color
    │
    ├── localization (lib)
    │   ├── edushell-core
    │   └── gettext-rs
    │
    └── gtk4 + gtk4-layer-shell (GUI)
        └── glib-rs + cairo-rs + pango-rs

edushell-settings (binary)
    │
    ├── edushell-core
    ├── cinnamon-adapter
    ├── theme-engine
    ├── localization
    └── gtk4 + libadwaita-rs
```

## 4. Crate Naming Convention

| Prefix | Pattern | Example |
|--------|---------|---------|
| Library | `edushell-<name>` | `edushell-core`, `edushell-theme-engine` |
| Binary | `edushell-<name>` | `edushell-panel`, `edushell-settings` |
| Service | `edushell-<name>` | `edushell-daemon`, `edushell-session` |

No `lib` or `bin` prefix — use Cargo.toml `[lib]` and `[[bin]]` sections.

## 5. Crate Export Convention

```
edushell-core (lib)
  └── pub use config::*;           # Re-export main types
      pub use error::*;
      pub use logging::*;
      pub use ipc::*;
      pub use runtime::*;
      // Internal: config::manager, config::schema NOT re-exported
```

Internal modules are `pub(crate)` by default. Public API surface is explicitly managed via re-exports in `lib.rs`.
