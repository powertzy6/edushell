# EduShell Architecture

A detailed technical overview of the EduShell desktop environment architecture.

---

## High-Level Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                        EduShell DE                               │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                     User Applications                     │  │
│  └─────────────────────────┬──────────────────────────────────┘  │
│                            │ Wayland protocol                     │
│  ┌─────────────────────────┴──────────────────────────────────┐  │
│  │                    eduWM (Compositor)                      │  │
│  │  ┌──────────┐ ┌──────────┐ ┌───────────┐ ┌─────────────┐  │  │
│  │  │  Render  │ │  Input   │ │  Window   │ │   Session   │  │  │
│  │  │  Engine  │ │ Handler  │ │  Manager  │ │  Protocol   │  │  │
│  │  └──────────┘ └──────────┘ └───────────┘ └─────────────┘  │  │
│  └─────────────────────────┬──────────────────────────────────┘  │
│                            │ Internal IPC (Unix socket)          │
│  ┌─────────────────────────┴──────────────────────────────────┐  │
│  │              edushell-core (Core Services)                 │  │
│  │  ┌──────────┐ ┌──────────┐ ┌───────────┐ ┌─────────────┐  │  │
│  │  │   IPC    │ │  Config  │ │  Plugin   │ │  Session    │  │  │
│  │  │  Router  │ │  Store   │ │  Loader   │ │  Manager    │  │  │
│  │  └──────────┘ └──────────┘ └───────────┘ └─────────────┘  │  │
│  └───────────┬──────────────────┬─────────────────────────────┘  │
│              │                  │                                 │
│  ┌───────────┴───────────┐  ┌──┴─────────────────────────────┐  │
│  │    edushell-ui        │  │     edushell-daemon             │  │
│  │  ┌─────────────────┐  │  │  ┌───────────┐ ┌────────────┐  │  │
│  │  │  Panel          │  │  │  │  Power    │ │  Network   │  │  │
│  │  │  Launcher       │  │  │  │  Manager  │ │  Manager   │  │  │
│  │  │  Notifications  │  │  │  ├───────────┤ ├────────────┤  │  │
│  │  │  Settings       │  │  │  │  Audio    │ │  Bluetooth │  │  │
│  │  └─────────────────┘  │  │  │  Manager  │ │  Manager   │  │  │
│  └───────────────────────┘  │  └───────────┘ └────────────┘  │  │
│                             └─────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                   edushell-sdk (Plugin API)                │  │
│  │  ┌──────────────┐ ┌──────────────┐ ┌───────────────────┐  │  │
│  │  │  Applet API  │ │  Desklet API │ │  Extension API    │  │  │
│  │  └──────────────┘ └──────────────┘ └───────────────────┘  │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │              Plugins and Extensions                        │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐  │  │
│  │  │  Clock   │ │ Weather  │ │  System  │ │  Cinnamon    │  │  │
│  │  │ Applet   │ │ Applet   │ │ Monitor  │ │  Compat      │  │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────────┘  │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### eduWM — Window Manager and Compositor

**Crate:** `crates/eduwm/`
**Type:** Wayland compositor (pure Rust)

eduWM is the lowest-level component in the stack. It is a Wayland compositor that handles:

- **Surface management** — Allocating, positioning, and rendering application surfaces
- **Input handling** — Processing keyboard, mouse, touchpad, and tablet input events
- **Window decoration** — Server-side decorations (SSD) using GTK4
- **Output management** — Multi-monitor configuration and rendering
- **Session protocol** — Integration with logind/elogind for session management
- **Rendering pipeline** — OpenGL ES / Vulkan rendering via smithay

eduWM communicates with higher-level components through a Unix domain socket using a custom binary protocol defined in `edushell-core`.

### edushell-core — Core Services

**Crate:** `crates/edushell-core/`
**Type:** Shared library (IPC, configuration, plugin system)

edushell-core is the central coordination layer. It provides:

- **IPC Router** — Routes messages between components over Unix sockets
- **Configuration Store** — Manages user and system configuration (TOML-based)
- **Plugin Loader** — Discovers, loads, and manages plugin lifecycles
- **Session Manager** — Tracks desktop session state (windows, workspaces, settings)
- **Event Bus** — Publishes and subscribes to desktop events (window focus, workspace change, etc.)

All UI components, the window manager, and the daemon communicate through edushell-core's IPC layer.

### edushell-ui — Desktop UI Components

**Crate:** `crates/edushell-ui/`
**Type:** Binary (GTK4/libadwaita application)

edushell-ui renders the visible desktop shell elements:

- **Panel** — The taskbar/panel at any screen edge
- **App Launcher** — Full-screen or pop-up application launcher
- **Notification Center** — Notification display and management
- **Settings Application** — Centralized configuration UI
- **Workspace Overview** — Visual workspace switcher
- **Lock Screen** — Session lock interface

edushell-ui uses GTK4 with libadwaita for theming and draws from the edushell-core IPC layer for state and events.

### edushell-daemon — Background Services

**Crate:** `crates/edushell-daemon/`
**Type:** Systemd user service

edushell-daemon runs in the background and manages:

- **Power management** — Sleep, hibernate, power button handling
- **Audio management** — Volume control and audio device switching
- **Network management** — Wi-Fi, Ethernet, VPN status and control
- **Bluetooth management** — Device discovery and pairing
- **Idle monitoring** — Screen dimming and auto-lock
- **Session persistence** — Saving and restoring window layouts

The daemon communicates with edushell-core via IPC and exposes D-Bus interfaces for external tool integration.

### edushell-sdk — Plugin SDK

**Crate:** `crates/edushell-sdk/`
**Type:** Library

The SDK provides the public API for third-party plugin development:

- **Applet API** — Build panel applets (clock, weather, system monitors)
- **Desklet API** — Build desktop widgets
- **Extension API** — Build extensions that modify desktop behavior
- **Manifest format** — JSON-based plugin metadata
- **Validation CLI** — `edushell-cli validate` for checking plugin correctness

### edushell-cli — Command-Line Interface

**Crate:** `crates/edushell-cli/`
**Type:** Binary

CLI tooling for developers and administrators:

- `edushell-cli new <name>` — Scaffold a new plugin project
- `edushell-cli build` — Build plugins from a manifest
- `edushell-cli validate` — Validate plugin manifests and APIs
- `edushell-cli list` — List installed plugins
- `edushell-cli info <plugin>` — Show plugin details

---

## IPC and Communication Patterns

### Transport Layer

All inter-component communication uses Unix domain sockets with a custom binary protocol:

```
┌────────────┐     Unix Socket     ┌────────────┐
│  eduWM     │◄───────────────────►│ edushell-  │
│            │                     │   core     │
└────────────┘                     └─────┬──────┘
                                         │
                                  ┌──────┴──────┐
                                  │             │
                           ┌──────┴─────┐ ┌────┴──────┐
                           │ edushell-  │ │ edushell- │
                           │    ui      │ │  daemon   │
                           └────────────┘ └───────────┘
```

### Message Types

Messages are defined as Rust enums with bincode serialization:

```rust
enum IpcMessage {
    // Window management
    WindowFocusChanged(WindowId),
    WindowMoved(WindowId, Position),
    WindowResized(WindowId, Size),
    WindowClosed(WindowId),

    // Workspace
    WorkspaceSwitched(WorkspaceId),
    WorkspaceCreated(WorkspaceId),
    WorkspaceRemoved(WorkspaceId),

    // Settings
    ThemeChanged(ThemeConfig),
    PanelConfigChanged(PanelConfig),

    // Plugin
    PluginLoaded(PluginId),
    PluginUnloaded(PluginId),

    // System
    PowerEvent(PowerAction),
    AudioEvent(AudioState),
    NetworkEvent(NetworkState),
}
```

### Event Bus Pattern

Components publish events to the core, which fans them out to all subscribers:

```
Publisher ──► edushell-core ──► Subscriber A
                   │
                   ├──► Subscriber B
                   └──► Subscriber C
```

This decouples producers from consumers and allows any component to listen for any event without direct dependencies.

### Request-Response Pattern

For synchronous operations (e.g., configuration reads), the IPC layer supports a request-response pattern with timeout handling:

```
Requester ──request──► Core ──request──► Responder
Requester ◄─response── Core ◄─response── Responder
```

---

## Module Dependency Graph

```
edushell-cli ──► edushell-sdk ──► edushell-core
                                      ▲
                                      │
edushell-ui ───────────────────────────┘
edushell-daemon ────────────────────────┘
eduwm ─────────────────────────────────┘
plugins ──► edushell-sdk ──► edushell-core
```

### Dependency Rules

1. `edushell-core` is the leaf dependency — it depends on no other EduShell crate
2. `edushell-sdk` depends only on `edushell-core`
3. `edushell-ui`, `edushell-daemon`, and `eduwm` depend on `edushell-core`
4. `edushell-cli` depends on `edushell-sdk`
5. Plugins depend on `edushell-sdk` (and transitively on `edushell-core`)
6. No circular dependencies are permitted

---

## Data Flow

### Input → Processing → Rendering → Output

```
Hardware Input
      │
      ▼
┌─────────────┐
│   Kernel    │  (evdev, libinput)
│  Input Sub  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    eduWM    │  Input handling, event dispatch
│  (Compositor)│
└──────┬──────┘
       │
       ├──────────────┐
       │              │
       ▼              ▼
┌────────────┐  ┌─────────────┐
│  edushell- │  │  edushell-  │
│    core    │  │     ui      │
│ (Dispatch) │  │ (Panel/Launcher)
└──────┬─────┘  └──────┬──────┘
       │               │
       ▼               ▼
┌─────────────┐  ┌─────────────┐
│  Window     │  │  GTK4       │
│  State      │  │  Rendering  │
│  Update     │  │  Pipeline   │
└──────┬──────┘  └──────┬──────┘
       │               │
       └───────┬───────┘
               │
               ▼
        ┌─────────────┐
        │    eduWM    │  Compositing, frame submission
        │  (Renderer) │
        └──────┬──────┘
               │
               ▼
        ┌─────────────┐
        │   Display   │  Wayland output to display hardware
        └─────────────┘
```

### Detailed Data Flow

1. **Hardware input** arrives via the kernel's evdev subsystem and is processed by libinput
2. **eduWM** receives raw input events, determines the target surface, and dispatches them
3. **edushell-core** routes events to the appropriate subscriber (UI, daemon, plugin)
4. **edushell-ui** processes UI-related events (panel clicks, launcher searches) and updates GTK4 widgets
5. **State changes** are published back through the event bus
6. **eduWM** composites all surfaces (application windows + shell UI) into a single frame
7. The composited frame is submitted to the display via the Wayland presentation protocol

---

## Cinnamon Compatibility Architecture

EduShell maintains compatibility with the Cinnamon desktop environment's plugin ecosystem.

### Compatibility Layer

```
┌──────────────────────────────────────┐
│         Cinnamon Plugins             │
│  (Applets, Desklets, Extensions)     │
└───────────────────┬──────────────────┘
                    │
┌───────────────────┴──────────────────┐
│      Cinnamon Compatibility Layer     │
│  ┌────────────────────────────────┐  │
│  │  CJS Runtime (SpiderMonkey)    │  │
│  │  St/Meta/GObject Bindings      │  │
│  │  Cinnamon JS API Shim          │  │
│  └────────────────────────────────┘  │
│  ┌────────────────────────────────┐  │
│  │  edushell-core IPC Bridge      │  │
│  └────────────────────────────────┘  │
└──────────────────────────────────────┘
                    │
                    ▼
           edushell-core (IPC)
```

### Compatibility Scope

EduShell provides API shims for the most commonly used Cinnamon APIs:

- `St.Widget`, `St.BoxLayout`, `St.Label` — UI widgets
- `Meta.Window` — Window information and management
- `Main._` — Desktop shell integration
- `GLib`, `GObject`, `Gio` — GLib/GObject bindings
- `imports.ui.*` — Common UI module imports

### Limitations

- Not all Cinnamon APIs are shimmed; some plugins may require adaptation
- Cinnamon-specific theming (CSS selectors) may not translate 1:1
- Performance characteristics differ due to the Rust-based architecture

---

## EduWM Window Manager Integration

### Protocol

eduWM communicates with edushell-core using a custom binary protocol over Unix sockets. The protocol is defined in `edushell-core/src/ipc/protocol.rs`.

### Session Management

eduWM integrates with systemd-logind for session management:

- **Session activation** — Receives D-Bus signals when the session is activated
- **Idle management** — Inhibits or allows idle based on user settings
- **Sleep/wake** — Saves and restores state across sleep cycles

### Rendering Pipeline

eduWM uses a multi-stage rendering pipeline:

1. **Surface collection** — Gather all visible surfaces for the current frame
2. **Damage tracking** — Only re-render regions that have changed (damage rectangles)
3. **Composition** — Composite surfaces using OpenGL ES or Vulkan
4. **Presentation** — Submit the frame to the Wayland output using the presentation-time protocol

### GPU Requirements

eduWM requires a GPU with:
- OpenGL ES 3.0+ support (for the default renderer)
- Vulkan 1.0+ support (for the optional Vulkan renderer)
- DRM/KMS support for direct scanout when possible

---

## Security Architecture Overview

### Principle of Least Privilege

Each component runs with only the permissions it requires:

- **eduWM** runs as the user session and requires no elevated privileges
- **edushell-daemon** runs as a systemd user service with sandboxing (PrivateTmp, ProtectSystem, etc.)
- **Plugins** run in the same process as edushell-core with no additional capabilities

### Plugin Sandboxing

Plugins are loaded as dynamic libraries into the edushell-core process:

- Plugins cannot access the network directly (no socket APIs exposed in the SDK)
- Plugins cannot execute arbitrary subprocesses (the SDK does not expose `std::process::Command`)
- Plugin file system access is restricted to the plugin's data directory and read-only access to user config
- Plugin manifests declare required permissions; the loader enforces these at load time

### IPC Security

- Unix domain socket permissions are set to user-only (`0600`)
- IPC messages are authenticated by the sender's PID and UID
- No message from a plugin is forwarded to the system without validation

### Wayland Isolation

- Application windows are fully isolated from the shell surface
- The shell UI runs on a privileged Wayland layer surface
- Applications cannot capture shell input or read shell state through the Wayland protocol

### Configuration Integrity

- System configuration is read-only for unprivileged users
- User configuration is validated on load; invalid config is rejected with safe defaults
- Configuration files use restrictive file permissions (`0600`)

### Update Verification

Future versions will support signed plugin packages and verified updates. In v1.0.0, plugin integrity is managed by the system administrator through standard package management.
