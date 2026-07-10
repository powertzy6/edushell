# Architecture Overview — EduShell v1

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        EduShell Desktop                         │
│                                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌───────────────┐  │
│  │  Edu     │  │  Edu     │  │  Edu     │  │   Learning    │  │
│  │  Panel   │  │ Launcher │  │ Settings │  │     Hub       │  │
│  ├──────────┤  ├──────────┤  ├──────────┤  ├───────────────┤  │
│  │ Notification Center    │  │ Quick Settings                 │  │
│  │ Workspace Switcher     │  │ System Tray                    │  │
│  └────────────────────────┘  └────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │              EduShell Abstraction Layer                   │   │
│  │  (Cinnamon Adapter, Settings Backend, IPC Bridge)         │   │
│  └───────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                    Cinnamon Desktop (v6+)                        │
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌───────────────┐   │
│  │  Muffin  │  │ Cinnamon │  │ Cinnamon │  │  Cinnamon      │   │
│  │   (WM)   │  │ Session  │  │ Settings │  │  Applets API   │   │
│  ├──────────┤  ├──────────┤  ├──────────┤  ├───────────────┤   │
│  │  Mutter  │  │ DBus     │  │ GSchem   │  │  Extensions   │   │
│  │  (compos)│  │ Services │  │          │  │               │   │
│  └──────────┘  └──────────┘  └──────────┘  └───────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                    System Layer                                   │
│                                                                  │
│  Wayland / X11  │  NetworkManager  │  PulseAudio/WirePlumber    │
│  UPower         │  AccountsService │  elogind/systemd-logind    │
│  D-Bus          │  BlueZ           │  polkit                     │
└─────────────────────────────────────────────────────────────────┘
```

## Architectural Layers

### Layer 1: EduShell Shell Layer (Custom Code)
- **Language**: Vala with GTK4
- **Location**: `src/shell/`
- **Ownership**: 100% EduShell
- **Replacement**: Never replaced, evolves with project

**Components**:
- `src/shell/panel/` — Main panel
- `src/shell/launcher/` — App launcher with search
- `src/shell/workspace/` — Workspace switcher
- `src/shell/notifications/` — Notification center
- `src/shell/tray/` — System tray
- `src/shell/quick-settings/` — Quick settings panel
- `src/shell/user-menu/` — User menu (power, account)

### Layer 2: EduShell Application Layer (Custom Code)
- **Language**: Vala with GTK4
- **Location**: `src/apps/`
- **Ownership**: 100% EduShell

**Components**:
- `src/apps/edu-settings/` — Edu Settings application
- `src/apps/learning-hub/` — Learning Hub portal
- *(v2+: EduApps, Edu Network, Edu Audio)*

### Layer 3: EduShell Abstraction Layer (Custom Code)
- **Language**: Vala / C
- **Location**: `src/lib/`
- **Ownership**: 100% EduShell
- **Replacement**: Evolving, never replaced

**Components**:
- `src/lib/edushell-core/` — Core library: config, logging, IPC
- `src/lib/cinnamon-adapter/` — Adapter to Cinnamon APIs
- `src/lib/settings-backend/` — Settings persistence (GSettings/GSchema)
- `src/lib/translation/` — gettext wrapper
- `src/lib/theme-engine/` — Theme management

### Layer 4: Cinnamon Compatibility Layer (Bridge Code)
- **Language**: Vala / JavaScript
- **Location**: `src/bridge/`
- **Ownership**: EduShell, but depends on Cinnamon
- **Replacement**: v2-v3 — gradually replaced

**Components**:
- `src/bridge/session-bridge/` — Cinnamon session integration
- `src/bridge/applet-bridge/` — Cinnamon applet compatibility
- `src/bridge/background-bridge/` — Desktop background integration

### Layer 5: Cinnamon Desktop (External Dependency)
- **Language**: C / JavaScript / Python
- **Location**: System packages
- **Ownership**: Linux Mint team
- **Replacement**: v2-v5 — gradually forked/replaced

**Components**:
- Muffin (Window Manager) → v4 replacement
- cinnamon-session → v3 replacement
- Cinnamon Settings → v2 replacement (partial)
- Cinnamon Applets API → v3 replacement

## Data Flow

```
User Input (keyboard, mouse, touch)
  │
  ▼
GTK4 Widget (in EduShell component)
  │
  ├──→ Event handled by widget → UI update
  │
  └──→ Signal emitted
         │
         ▼
    Domain Logic (edushell-core)
         │
         ├──→ Settings Backend → GSettings → File
         │
         ├──→ Cinnamon Adapter → DBus → Cinnamon Service
         │
         └──→ System Service (NetworkManager, PulseAudio, UPower)
                  │
                  ▼
            DBus System Bus
```

## Process Model

```
EduShell Process Tree (single user session)
─────────────────────────────────────────────
systemd --user
  │
  ├── cinnamon-session (session manager)
  │     │
  │     ├── muffin (window manager)
  │     │     └── wayland compositor / X11 server
  │     │
  │     ├── edushell-panel (main shell process)
  │     │     ├── UI thread (GTK4 main loop)
  │     │     ├── worker thread (search indexing)
  │     │     └── DBus listener thread
  │     │
  │     ├── edushell-settings-daemon (settings service)
  │     │
  │     ├── cinnamon-killer-daemon (crash recovery)
  │     │
  │     └── [user applications]
```

## Communication Patterns

| Pattern | Technology | Use Case |
|---------|-----------|----------|
| Signal/Slot | GLib (internal) | Intra-process communication |
| D-Bus (session) | DBus | EduShell ↔ Cinnamon services |
| D-Bus (system) | DBus | EduShell ↔ System services (NM, UPower) |
| GSettings | GIO | Settings persistence |
| Files (XDG) | File I/O | Config files, cache, logs |
| UNIX Signals | Signal | Process management |
| GSocket | GLib | Future IPC for Rust components |

## Configuration Storage

| Data | Location | Format | Tool |
|------|----------|--------|------|
| Shell settings | `~/.config/edushell/settings.ini` | INI/keyfile | GLib KeyFile |
| User preferences | GSettings schema `org.edushell.*` | GVariant | GSettings |
| Session state | `~/.local/share/edushell/state/` | JSON | Manual |
| Logs | `~/.local/share/edushell/logs/` | Plain text | GLib logging |
| Cache | `~/.cache/edushell/` | Binary | Various |
| Theme cache | `~/.cache/edushell/theme/` | CSS/images | Theme engine |
