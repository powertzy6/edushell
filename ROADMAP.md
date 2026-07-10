# EduShell Roadmap

This document outlines the planned development path for EduShell across major releases.

---

## v1.0 (Current) — Foundation

**Released:** July 10, 2026

The initial stable release establishing the core platform.

### Completed

- [x] Core desktop environment session (panel, launcher, notifications, workspaces, settings)
- [x] Wayland-native window manager (eduWM) built in pure Rust
- [x] Cinnamon compatibility layer for applets, desklets, and extensions
- [x] Plugin SDK with Applet API, Desklet API, and Extension API
- [x] CLI tool for scaffolding, validation, and plugin management
- [x] edushell-daemon for power, audio, network, and Bluetooth management
- [x] Theme engine with light/dark modes and accent colors
- [x] Multi-monitor support with independent panel placement
- [x] Keyboard shortcuts system with full customization
- [x] Session management with lock, suspend, logout, and shutdown
- [x] Install script and Debian packaging
- [x] Docker-based reproducible build environment
- [x] Complete documentation suite

### Maintenance

- [ ] Bug fixes and stability patches
- [ ] Security updates as needed
- [ ] Cinnamon plugin compatibility expansion
- [ ] Additional language translations

---

## v1.1 — Polish and Extension

**Target:** Q4 2026

Focused on stability improvements, user-requested features, and expanding the plugin ecosystem.

### Planned

- [ ] Window tiling assistant (quarter-tiling, smart gaps)
- [ ] Drag-and-drop file support in the launcher
- [ ] Plugin store / marketplace for community plugins
- [ ] Enhanced accessibility: screen reader improvements, keyboard-only navigation
- [ ] Clipboard manager applet
- [ ] Per-application audio volume control in the system tray
- [ ] Custom keyboard shortcut recording UI
- [ ] fractional scaling improvements for HiDPI displays
- [ ] Additional built-in applets: calendar, color picker, device manager
- [ ] Flatpak integration for sandboxed plugin distribution

---

## v2.0 — Platform Maturity

**Target:** Q2 2027

The next major release moves toward independence from Cinnamon's architecture and introduces a native UI toolkit.

### Planned

- [ ] **Core v2 (`edushell-core2`)** — Cinnamon-independent core with a redesigned IPC layer
  - Binary protocol v2 with schema evolution support
  - Asynchronous IPC for improved responsiveness
  - Plugin sandboxing with process isolation
  - Event sourcing for session state persistence
- [ ] **UI Kit v1** — A native EduShell UI toolkit built on GTK4/libadwaita
  - Reusable widget library for applets and settings
  - Theming API with CSS variables and dynamic loading
  - Animation framework for smooth transitions
  - Accessibility-first widget design
- [ ] Enhanced window management features
  - Window groups and tabbing
  - Picture-in-picture mode
  - Per-window audio routing indicators
- [ ] Desktop widgets (desklets) with drag-and-drop positioning
- [ ] Multi-language input method support (IBus, Fcitx5)
- [ ] System monitoring dashboard applet
- [ ] Remote desktop integration (VNC/RDP protocol support)

---

## v3.0 — Wayland Evolution

**Target:** Q4 2027

Deepening Wayland integration and adopting the latest Wayland protocols for a modern, secure desktop.

### Planned

- [ ] **Enhanced Wayland protocol support**
  - `wlr-layer-shell` for improved panel and overlay placement
  - `xdg-decoration` negotiation for client-side or server-side decorations
  - `color-management` protocol for accurate color reproduction
  - `virtual-keyboard` protocol for on-screen keyboard
  - `input-method` v2 for advanced text input
- [ ] **Performance optimization**
  - Vulkan rendering backend as default
  - GPU-accelerated compositing with direct scanout
  - Frame scheduling for consistent 60fps (or higher) rendering
  - Memory-mapped configuration for zero-copy state sharing
- [ ] Screen sharing with portal integration
- [ ] HDR display support (pending Wayland HDR protocol stabilization)
- [ ] Touchscreen and tablet-optimized gestures
- [ ] Wayland-native screenshot and screen recording
- [ ] Dynamic monitor configuration (hot-plug, resolution changes)

---

## v4.0 — Desktop Environment Independence

**Target:** Q2 2028

Full independence from GNOME/Cinnamon infrastructure with a custom compositor and native toolkit.

### Planned

- [ ] **Custom Wayland compositor** — Moving beyond smithay to a purpose-built compositor
  - Embedded GPU scheduling for minimal latency
  - Per-surface rendering optimization
  - Advanced damage tracking with fractional frame rates
- [ ] **Native UI toolkit** — A Rust-native widget library independent of GTK4
  - Retained-mode rendering for shell UI
  - GPU-accelerated text rendering (Swash/Cosmic-text)
  - Built-in accessibility (AT-SPI bridge or native)
  - Theme system with runtime CSS compilation
- [ ] **Protocol extensions**
  - Custom EduShell Wayland protocols for plugin communication
  - Secure IPC between compositor and shell components
  - Session management protocol for login managers
- [ ] Flatpak-native application portal integration
- [ ] Advanced power management (per-component idle states)
- [ ] Predictive app launcher with machine learning ranking
- [ ] Multi-user session support (switching users without logout)

---

## v5.0 — Enterprise Ready

**Target:** Q4 2028

Hardening EduShell for enterprise and institutional deployment at scale.

### Planned

- [ ] **LTS (Long-Term Support) releases**
  - 3-year support cycle for LTS versions
  - Backported security fixes
  - ABI stability guarantees for plugins
- [ ] **Enterprise management**
  - D-Bus interface for centralized configuration (Ansible, Puppet, etc.)
  - Group policy support for mandatory settings
  - Audit logging for compliance requirements
  - Remote desktop administration tools
- [ ] **Scalable deployment**
  - Containerized EduShell sessions (Podman/Docker)
  - Network-based user profiles and configuration sync
  - Automated updates with rollback support
- [ ] **Education-specific features**
  - Classroom management tools (screen broadcasting, remote control)
  - Student progress monitoring integration
  - Content filtering and safe browsing integration
  - Exam mode (lockdown mode for assessments)
- [ ] Enterprise support program and SLA
- [ ] Certification program for hardware vendors
- [ ] Professional services and training materials

---

## Versioning Policy

EduShell follows [Semantic Versioning](https://semver.org/):

- **Major versions** (v1.0, v2.0, ...) may contain breaking API changes
- **Minor versions** (v1.1, v1.2, ...) add features in a backward-compatible manner
- **Patch versions** (v1.0.1, v1.0.2, ...) contain bug fixes and security patches

### API Stability

- `edushell-core` v1.x APIs are stable within the v1.x series
- `edushell-core2` (introduced in v2.0) guarantees API stability from its initial release
- `edushell-sdk` plugin APIs follow a separate stability track; breaking changes require a major SDK version bump
- Internal APIs not exposed in the SDK may change without notice

### Release Cadence

- **Stable releases** are cut quarterly or when a milestone is complete
- **Point releases** are issued for critical bug fixes and security patches as needed
- **Pre-release candidates** (alpha, beta, rc) are published 2–4 weeks before major releases

---

## Contributing

EduShell welcomes contributions. See the project's contributing guidelines for details on:

- Code style and review process
- Plugin development guidelines
- Documentation improvements
- Translation and internationalization
- Testing and QA

---

<p align="center"><em>This roadmap is a living document and will be updated as development progresses.</em></p>
