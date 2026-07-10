# EduShell v1.0.0 Release Notes

**Release Date:** July 10, 2026
**Codename:** Foundation

---

## Overview

EduShell v1.0.0 is the first stable release of a modern, Rust-based desktop environment designed for educational institutions and performance-conscious users. This release establishes the foundation of the EduShell platform, including a Wayland-native window manager (eduWM), a full desktop shell with panel and launcher, a Cinnamon-compatible plugin ecosystem, and a complete SDK for third-party development.

This is a production-ready release suitable for deployment in educational labs, development workstations, and general desktop use on supported Linux distributions.

---

## New Features

### Desktop Shell

- **Panel** — Configurable taskbar with applets, app indicators, workspace switcher, system tray, and notification center. Supports top, bottom, left, and right positions with auto-hide and transparency.
- **Application Launcher** — Fast full-screen or pop-up launcher with instant search, application categorization, recent apps, and keyboard navigation.
- **Notification Center** — Notification banners with configurable duration, grouped notifications, per-app notification controls, and Do Not Disturb mode.
- **Workspace Management** — Virtual workspaces with overview mode, workspace naming, and keyboard-driven switching.
- **Settings Application** — Centralized configuration for theme, display, accessibility, keyboard shortcuts, and power management.

### Window Management

- **eduWM Compositor** — A custom Wayland compositor written entirely in Rust, providing hardware-accelerated rendering via OpenGL ES or Vulkan.
- **Server-Side Decorations** — Window title bars and borders rendered by the compositor using GTK4, ensuring consistent appearance.
- **Multi-Monitor Support** — Independent panel placement and per-display configuration for multi-screen setups.
- **Damage Tracking** — Only changed screen regions are recomposited, ensuring efficient rendering.
- **XWayland** — Full compatibility with legacy X11 applications through XWayland support.

### Plugin Ecosystem

- **Plugin SDK** — A public API (`edushell-sdk`) for building panel applets, desktop widgets (desklets), and behavior-modifying extensions.
- **Cinnamon Compatibility** — A CJS-based compatibility layer allows many existing Cinnamon applets, desklets, and extensions to run unmodified on EduShell.
- **CLI Tooling** — `edushell-cli` provides commands for scaffolding new plugins, validating manifests, building plugins, and listing installed extensions.

### System Services

- **edushell-daemon** — A background service managing power, audio, network, and Bluetooth. Runs as a systemd user service with sandboxing.
- **Session Persistence** — Window layouts are saved on logout and restored on login.
- **Lock Screen** — Secure session lock with configurable timeout and idle detection.

### Developer Experience

- **Cargo Workspace** — Clean multi-crate workspace with well-defined dependency boundaries.
- **Docker Builds** — Reproducible build environment based on Ubuntu 26.04.
- **Comprehensive Documentation** — Full documentation suite covering building, installation, user guide, architecture, and roadmap.

---

## Compatibility Notes

### Supported Distributions

| Distribution            | Version   | Status       |
| ----------------------- | --------- | ------------ |
| Ubuntu Budgie           | 26.04     | Fully tested |
| Ubuntu                  | 26.04     | Fully tested |
| Linux Mint              | 22+       | Fully tested |
| Debian                  | Stable    | Supported    |

### Required System Libraries

EduShell requires GTK4 and libadwaita for compilation and runtime. On Ubuntu 26.04:

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

### Display Server

- **Native Wayland** — Fully supported and recommended
- **XWayland** — Supported for legacy X11 applications
- **X11 session** — Not supported; EduShell requires a Wayland session

### GPU Requirements

- OpenGL ES 3.0+ (required)
- Vulkan 1.0+ (optional, for Vulkan rendering backend)
- DRM/KMS kernel driver support

### Cinnamon Plugin Compatibility

Most Cinnamon applets and desklets using the following APIs are compatible:

- `St.Widget`, `St.BoxLayout`, `St.Label`
- `Meta.Window` (limited)
- `GLib`, `GObject`, `Gio`
- `Main._` (partial)

Plugins using advanced Cinnamon internals or direct DBus interfaces may require adaptation.

---

## Known Issues

See [KNOWN_ISSUES.md](KNOWN_ISSUES.md) for a complete list of known issues and limitations. Key items include:

- GTK4 crates require system libraries (`libgtk-4-dev`, `libadwaita-1-dev`) for compilation
- eduWM is a pure Rust implementation — some advanced compositor features require future kernel/Wayland protocol support
- Muffin-based crates cannot compile in headless CI environments without GTK4 system packages
- No Wayland protocol extension for `wlr-layer-shell` on all compositors yet

---

## System Requirements

### Minimum

- **CPU:** 64-bit x86_64 or aarch64 processor
- **RAM:** 1 GB available
- **Storage:** 500 MB for EduShell installation
- **GPU:** OpenGL ES 3.0 capable GPU
- **Display:** Any resolution; 1920×1080 recommended
- **OS:** Linux kernel 5.15+ with Wayland support

### Recommended

- **CPU:** Modern quad-core processor
- **RAM:** 4 GB or more
- **Storage:** 2 GB for installation plus user data
- **GPU:** Vulkan-capable GPU for best performance
- **Display:** 2560×1440 or higher

---

## Upgrade Instructions

This is the first stable release. There are no previous stable versions to upgrade from.

### Fresh Installation

Follow the instructions in [INSTALL.md](INSTALL.md):

```bash
sudo ./install.sh
```

Or install the Debian package:

```bash
sudo dpkg -i target/debian/edushell_1.0.0_amd64.deb
```

### From Pre-release / Development Builds

If you have a development build installed, uninstall it first:

```bash
sudo ./install.sh --uninstall
```

Then perform a fresh install:

```bash
sudo ./install.sh
```

User configuration in `~/.config/edushell/` is preserved across reinstalls. If you encounter issues after upgrading, delete the configuration directory to reset to defaults:

```bash
rm -rf ~/.config/edushell/
```

---

## Acknowledgments

EduShell v1.0.0 is the result of extensive development and testing. Special thanks to:

- The Rust community for an exceptional systems programming language
- The smithay project for Wayland compositor infrastructure
- The GNOME and GTK4 teams for the UI toolkit
- The Cinnamon team for the desktop compatibility reference
- The System76 COSMIC team for architectural inspiration
- All testers, contributors, and early adopters

---

<p align="center"><em>EduShell v1.0.0 — Foundation. Built for learning. Built for speed.</em></p>
