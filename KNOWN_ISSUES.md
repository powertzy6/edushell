# Known Issues and Limitations

This document tracks known issues, limitations, and workarounds for EduShell v1.0.0.

---

## GTK4 System Library Requirement

**Affected crates:** `edushell-ui`, `edushell-core`, `edushell-daemon`

All crates that depend on GTK4 and libadwaita require the corresponding system development libraries to be installed for compilation. These crates cannot be built without them.

### Symptoms

Build errors referencing missing headers or link failures:

```
error: failed to run pkg-config to find native libraries
Could not find specification for -lgtk-4
Could not find specification for -ladwaita-1
```

### Workaround

Install the required system packages before building:

```bash
# Ubuntu 26.04 / Ubuntu Budgie 26.04
sudo apt install libgtk-4-dev libadwaita-1-dev

# Linux Mint 22+
sudo apt install libgtk-4-dev libadwaita-1-dev

# Debian Stable
sudo apt install libgtk-4-dev libadwaita-1-dev
```

### Notes

- The `eduwm` crate (pure Rust compositor) does **not** require GTK4 and can be built without these packages.
- The `edushell-sdk` and `edushell-cli` crates do **not** require GTK4.
- A future release may provide a feature flag to build GTK4-dependent crates conditionally.

---

## eduWM — Advanced Compositor Features

**Affected component:** `eduwm`

eduWM is a pure Rust Wayland compositor. While it provides full window management and compositing capabilities, some advanced compositor features depend on future kernel and Wayland protocol developments that are not yet widely available.

### Missing or Limited Features

- **HDR output** — The Wayland color-management and HDR protocols are still under active development. HDR display support is not available in v1.0.0.
- **Per-surface color management** — Requires the `wp_color_management` protocol, which is not yet stable in all compositors.
- **Direct scanout** — While eduWM supports DRM/KMS, direct scanout for fullscreen surfaces is not yet optimized for all GPU drivers.
- **Vulkan renderer** — Available as an optional feature flag; not yet the default renderer.
- **Explicit sync** — The `wp_linux_drm_syncobj` protocol is supported where available but not required.

### Workaround

EduWM falls back to OpenGL ES compositing where advanced features are unavailable. Performance may be slightly lower on hardware that would benefit from Vulkan or direct scanout.

### Notes

- These features are planned for v3.0 (Wayland Evolution) as Wayland protocol support matures.
- eduWM is actively developed and will adopt new protocols as they stabilize in the Wayland ecosystem.

---

## Headless CI Compilation

**Affected crates:** `edushell-core`, `edushell-daemon`, `edushell-ui`

Crates that depend on Muffin (Cinnamon's window manager library) and GTK4 cannot compile in headless CI environments without GTK4 system packages installed.

### Symptoms

CI pipelines fail with missing system library errors when building the full workspace. The `eduwm`, `edushell-sdk`, and `edushell-cli` crates build successfully in headless environments, but GTK4-dependent crates do not.

### Workaround

For CI environments, build only the crates that do not require GTK4:

```bash
cargo build -p eduwm -p edushell-sdk -p edushell-cli
```

Or use the Docker build environment, which includes all required system packages:

```bash
docker build -t edushell-build .
docker run --rm -v "$(pwd)":/src -w /src edushell-build cargo build --workspace
```

### Notes

- A future release may add a `headless` feature flag to skip GTK4-dependent crates during CI builds.
- Integration tests for GTK4-dependent crates require a display server or a virtual framebuffer (e.g., `Xvfb` with XWayland).

---

## Core v2 API Stability

**Affected crate:** `core2` (`edushell-core2`)

The Core v2 crate (`edushell-core2`) is versioned at 2.0.0 and is not yet part of the EduShell v1.0.0 stable release. It is marked as 2.0.0 to indicate its planned role as the successor to `edushell-core` starting with EduShell v2.0.

### Status

- `edushell-core2` is under active development
- Its API is **not** guaranteed to be stable until EduShell v2.0
- It is included in the workspace for development purposes but is not used by any other v1.0.0 crate

### Implications

- Do not depend on `edushell-core2` in production plugins or extensions
- The API surface may change without notice between v1.x releases
- When EduShell v2.0 is released, `edushell-core2` will be the stable core API

### Notes

- Once EduShell v2.0 ships, `edushell-core2` guarantees API stability from its 1.0.0 release onward
- Plugin authors should target `edushell-sdk` (which wraps `edushell-core`) for v1.x compatibility

---

## wlr-layer-shell Protocol Compatibility

**Affected component:** `eduwm`, `edushell-ui`

The `wlr-layer-shell` Wayland protocol extension is used for panel and overlay placement on Wayland compositors. Not all compositors support this protocol extension.

### Affected Compositors

- eduWM supports `wlr-layer-shell` natively
- GNOME (Mutter) does **not** support `wlr-layer-shell` — EduShell shell UI elements fall back to `xdg-shell` surfaces
- KWin (KDE) supports `wlr-layer-shell` starting with Plasma 6.0
- wlroots-based compositors (Sway, Hyprland, etc.) generally support `wlr-layer-shell`

### Workaround

When running EduShell on a compositor without `wlr-layer-shell` support, shell UI elements (panel, launcher, notifications) are rendered as regular `xdg-shell` surfaces. They function correctly but may not respect layer ordering, exclusive zones, or anchored positioning.

### Notes

- A universal Wayland protocol for layer surfaces is under discussion in the Wayland community
- EduShell will adopt a standardized protocol once it is ratified and available in mainstream compositors
- When running EduShell with its own eduWM compositor, `wlr-layer-shell` is fully supported

---

## Additional Known Issues

### High CPU Usage with Many Notifications

Rapid notification bursts (e.g., from chat applications) may cause temporary high CPU usage in `edushell-ui` as notifications are processed and animated. This is expected behavior and resolves once the notification queue is drained.

**Planned fix:** v1.1 will implement notification batching to reduce rendering overhead.

### Panel Applet Memory Growth

Some panel applets that poll system state (e.g., system monitor applets) may exhibit gradual memory growth over long sessions. This is due to accumulated data points not being pruned.

**Planned fix:** v1.1 will introduce a data retention API in the applet SDK.

### Cinnamon Extension Compatibility

While the Cinnamon compatibility layer supports most common Cinnamon APIs, some extensions that use undocumented or internal Cinnamon APIs may not function correctly. Incompatible extensions will typically fail to load with a descriptive error message in the plugin logs.

**Planned fix:** Ongoing expansion of the compatibility shim in each release. Report specific incompatibilities via the issue tracker.

### Multi-Monitor Panel Configuration

Panel configuration (position, size, applets) currently applies globally across all monitors. Per-monitor panel configuration (e.g., different applets on different screens) is not supported in v1.0.0.

**Planned fix:** Per-monitor panel configuration is planned for v1.1.

### Accessibility Screen Reader

The built-in screen reader integration relies on the AT-SPI2 D-Bus interface. On systems where AT-SPI2 is not installed or is misconfigured, screen reader functionality may not work.

**Workaround:** Ensure `at-spi2-core` and `at-spi2-atk` are installed:

```bash
sudo apt install at-spi2-core at-spi2-atk
```

---

## Reporting New Issues

If you encounter an issue not listed here, please report it via the project's issue tracker:

- Include your distribution and version
- Include the EduShell version (`edushell-cli --version`)
- Include relevant logs (`journalctl --user -u edushell-daemon`)
- Describe steps to reproduce

---

<p align="center"><em>This document is updated with each release. Check for new known issues when upgrading.</em></p>
