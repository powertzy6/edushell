# Changelog

All notable changes to EduShell will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-07-10

### Added

- Full desktop environment session with panel, launcher, notifications, workspaces, and settings
- Wayland-native window manager (eduWM) built in pure Rust using smithay
- Cinnamon compatibility layer supporting applets, desklets, and extensions via CJS runtime shim
- Plugin SDK (`edushell-sdk`) with Applet API, Desklet API, and Extension API
- CLI tool (`edushell-cli`) for project scaffolding, plugin validation, and build management
- Background daemon (`edushell-daemon`) for power, audio, network, and Bluetooth management
- Panel with applet support, auto-hide, transparency, and multi-position (top/bottom/left/right)
- Full-screen and pop-up application launcher with search, categorization, and recent tracking
- Notification center with grouped notifications, per-app controls, and Do Not Disturb mode
- Workspace management with overview mode, workspace naming, and drag-and-drop window reordering
- Settings application with theme, display, accessibility, keyboard, and power configuration
- Theme engine with light/dark modes, accent colors, and libadwaita integration
- Multi-monitor support with independent panel placement and per-display settings
- Keyboard shortcuts system with customizable bindings for all desktop actions
- Session management with graceful lock, suspend, logout, and shutdown flows
- Wayland session file for display manager integration (GDM, LightDM, SDDM)
- XWayland support for running legacy X11 applications
- Server-side window decorations (SSD) via GTK4
- Damage tracking for efficient rendering performance
- Internationalization support with RTL language capability
- Install script supporting prefix-based and unattended installations
- Debian package for Debian-based distributions
- Docker-based reproducible build environment
- Comprehensive documentation: README, BUILD, INSTALL, USER_GUIDE, ARCHITECTURE, ROADMAP

### Changed

- Initial stable release — no prior versions to compare against

### Deprecated

- Nothing deprecated in this release

### Removed

- Nothing removed in this release

### Fixed

- Nothing fixed in this release (first stable release)

### Security

- Plugins run in-process with restricted API surface (no network access, no subprocess execution)
- IPC sockets use user-only permissions (`0600`)
- Configuration files validated on load with safe defaults for invalid input
- Wayland surface isolation prevents applications from reading shell state
- edushell-daemon runs under systemd sandboxing (PrivateTmp, ProtectSystem, NoNewPrivileges)
- Plugin permissions declared in manifests and enforced at load time
