# ![EduShell Logo](docs/assets/logo.svg) EduShell

<!-- Badges -->
![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-GPL--3.0-blue)
![Version](https://img.shields.io/badge/version-1.0.0-stable)
![Rust Edition](https://img.shields.io/badge/rust--edition-2021-orange)

## Modern Educational Desktop Environment

EduShell is a modern, modular, and extensible desktop environment built in Rust, designed for educational institutions and power users who demand performance, flexibility, and a cohesive user experience. Built from the ground up on Wayland, EduShell combines a custom window manager (EduWM), a Cinnamon-compatible desktop layer, and a full SDK for third-party plugin development.

EduShell aims to deliver a polished desktop experience that rivals established environments while providing a foundation for educational workflows — from classroom management to developer tooling. Every component is designed to be fast, secure, and customizable, with accessibility and internationalization built in from day one.

Whether you are a system administrator deploying across a computer lab, a developer looking to extend the desktop, or an end user who simply wants a fast and beautiful desktop, EduShell provides the tools and documentation to make the experience seamless.

---

## Features

- **Wayland-native window management** via EduWM, a custom compositor built in pure Rust
- **Cinnamon compatibility layer** — run existing Cinnamon applets, desklets, and extensions
- **Full desktop session** — panel, launcher, notifications, workspaces, and settings
- **Plugin SDK** — build and distribute third-party plugins with a well-documented API
- **CLI tooling** — scaffold new plugins, validate manifests, and manage development workflows
- **Workspace management** — fluid workspace switching with overview mode
- **Keyboard-driven navigation** — comprehensive shortcuts for every action
- **Theme engine** — system-wide dark/light themes with GTK4 and libadwaita support
- **Notification center** — dismissible, grouped, and configurable notifications
- **App launcher** — fast search, categorization, and recent-apps tracking
- **Display and accessibility settings** — scaling, color, screen reader, and high-contrast support
- **Multi-monitor support** — independent wallpapers and panel placement per display
- **Session management** — graceful lock, suspend, logout, and shutdown flows
- **Internationalization** — fully translatable UI with RTL language support
- **Security hardened** — sandboxed plugins, Wayland isolation, and minimal privilege surfaces

---

## Architecture

EduShell is organized as a Cargo workspace with multiple crates that communicate over a lightweight IPC layer. The high-level architecture is as follows:

```
┌─────────────────────────────────────────────────┐
│                   EduShell DE                   │
│  ┌───────────┐  ┌───────────┐  ┌────────────┐  │
│  │   Panel   │  │ Launcher  │  │   Settings  │  │
│  └─────┬─────┘  └─────┬─────┘  └──────┬─────┘  │
│        │              │               │          │
│  ┌─────┴──────────────┴───────────────┴──────┐  │
│  │            edushell-core (IPC)            │  │
│  └─────────────────┬─────────────────────────┘  │
│                    │                             │
│  ┌─────────────────┴─────────────────────────┐  │
│  │           eduWM (Compositor)              │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for a detailed breakdown.

---

## Quick Start

### Prerequisites

- Rust 1.80+ (via [rustup](https://rustup.rs/))
- System dependencies (see [BUILD.md](BUILD.md))

### Build

```bash
./scripts/dev.sh build
```

### Install

```bash
./scripts/dev.sh install
```

### Run

Select **EduShell** from your display manager's session selector at login, or run:

```bash
eduwm
```

For detailed instructions, see [INSTALL.md](INSTALL.md).

---

## Project Structure

| Directory              | Description                                             |
| ---------------------- | ------------------------------------------------------- |
| `crates/eduwm/`       | Wayland compositor and window manager                   |
| `crates/edushell-core/`| Core desktop services and IPC layer                     |
| `crates/edushell-ui/`  | UI toolkit and desktop shell components                 |
| `crates/edushell-sdk/` | Plugin SDK for third-party development                  |
| `crates/edushell-cli/` | CLI tool for scaffolding and validation                 |
| `crates/edushell-daemon/` | Background services and session management           |
| `crates/core2/`        | Core v2 — Cinnamon-independent next-generation core     |
| `plugins/`             | Built-in plugins and applets                            |
| `themes/`              | Default themes and assets                               |
| `scripts/`             | Development and build scripts                           |
| `docs/`                | Extended documentation and guides                       |
| `data/`                | Desktop entries, session files, and system integration  |

---

## Documentation

| Document                     | Description                              |
| ---------------------------- | ---------------------------------------- |
| [BUILD.md](BUILD.md)         | Build instructions and prerequisites     |
| [INSTALL.md](INSTALL.md)     | Installation and configuration guide     |
| [USER_GUIDE.md](USER_GUIDE.md) | End user reference manual             |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Detailed architecture overview   |
| [CHANGELOG.md](CHANGELOG.md) | Release history and changes              |
| [RELEASE_NOTES.md](RELEASE_NOTES.md) | v1.0.0 release notes             |
| [ROADMAP.md](ROADMAP.md)     | Future development roadmap               |
| [KNOWN_ISSUES.md](KNOWN_ISSUES.md) | Known issues and limitations       |
| [SECURITY.md](SECURITY.md)   | Security policy and reporting            |

---

## License

EduShell is licensed under the **GNU General Public License v3.0** — see the [LICENSE](LICENSE) file for details.

## Credits

- The **Rust Project** — foundation language
- The **GNOME Project** — GTK4 and libadwaita libraries
- The **Cinnamon Project** — desktop compatibility reference
- The **System76 COSMIC Project** — architectural inspiration
- All contributors and the open-source community

---

<p align="center"><em>EduShell — Built for learning. Built for speed. Built for the future.</em></p>
