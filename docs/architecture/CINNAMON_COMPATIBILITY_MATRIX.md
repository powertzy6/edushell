# Cinnamon Compatibility Matrix — EduShell

## Purpose
Dokumen ini mencatat komponen Cinnamon mana yang digunakan oleh EduShell, bagaimana cara penggunaannya, dan rencana penggantian di masa depan.

---

## Component Mapping

| EduShell Component | Cinnamon Component Used | Dependency Type | Replacement Version | Notes |
|--------------------|------------------------|-----------------|-------------------|-------|
| **EduPanel** | Cinnamon Panel (inspiration only) | None (custom) | — | Built from scratch. No Cinnamon panel code used. |
| **EduLauncher** | Cinnamon Menu (inspiration only) | None (custom) | — | Built from scratch. Custom search, categories, favorites. |
| **EduWorkspace** | Muffin (winsys/workspace) | Runtime | v4 | Uses Muffin's internal workspace API via DBus. |
| **EduTray** | Cinnamon StatusIconWatcher | Runtime | v2 | Currently reads Cinnamon's status icon watcher for legacy tray icons. |
| **EduNotifications** | Cinnamon Notification Daemon | Runtime | v2 | Replaces `cinnamon-notifications` daemon with custom implementation. |
| **EduQuickSettings** | N/A (new) | None | — | Fully custom. No Cinnamon dependency. |
| **EduUserMenu** | cinnamon-session | Runtime | v3 | Uses `cinnamon-session --logout` etc. Will use custom session manager in v3. |
| **EduOSD** | cinnamon-osd | Build-time | v2 | Currently reads Cinnamon OSD source for protocol reference. |
| **EduSettings** | GSettings/Cinnamon schemas | Build-time | v3 | Uses GSettings with `org.cinnamon` schemas for system settings. Custom `org.edushell` schemas for shell settings. |
| **LearningHub** | N/A (new) | None | — | Fully custom. |
| **libcinnamon-adapter** | Cinnamon DBus API | Runtime | v3+ | Abstraction layer that will be replaced as Cinnamon components are forked. |
| **session-bridge** | cinnamon-session | Runtime | v3 | Currently delegates to cinnamon-session for session lifecycle. |
| **applet-bridge** | Cinnamon Applet API | Runtime | v3 | Provides compatibility layer for third-party Cinnamon applets. |
| **background-bridge** | Cinnamon background API | Runtime | v2 | Uses Cinnamon's DBus interface for wallpaper management. |

---

## API Surface Used from Cinnamon

### DBus Interfaces
| Interface | Purpose | Replacement Strategy |
|-----------|---------|---------------------|
| `org.cinnamon.ScreenSaver` | Lock screen trigger | v2 — Custom EduLock |
| `org.cinnamon.SettingsDaemon` | System settings | v3 — Custom |
| `org.cinnamon.SessionManager` | Session lifecycle | v3 — EduSession |
| `org.cinnamon.Sound` | Sound effects | v2 — Edu Audio |

### GSettings Schemas
| Schema | Purpose | Replacement Strategy |
|--------|---------|---------------------|
| `org.cinnamon.desktop.background` | Wallpaper | v2 — EduWallpaper |
| `org.cinnamon.desktop.keybindings` | Keyboard shortcuts | v2 — Custom |
| `org.cinnamon.desktop.interface` | GTK settings | v3 — Custom |
| `org.cinnamon.desktop.peripherals` | Input devices | v3 — Custom |
| `org.cinnamon.desktop.screensaver` | Screensaver | v2 — EduLock |
| `org.cinnamon.desktop.session` | Session settings | v3 — EduSession |

### File System Interfaces
| Path | Purpose | Replacement Strategy |
|------|---------|---------------------|
| `~/.cinnamon/configs/` | Applet configs | v3 — Custom |
| `~/.local/share/cinnamon/` | Applet data | v3 — Custom |

---

## Dependency Severity

| Severity | Count | Description |
|----------|-------|-------------|
| **Hard** (cannot function without) | 2 | Muffin (WM), cinnamon-session |
| **Soft** (can work with limitations) | 4 | Background, screensaver, keybindings, sound |
| **Optional** (degraded but functional) | 3 | Applet bridge, tray icons, legacy schemas |

---

## Replacement Priority Matrix

| Component | v1 | v2 | v3 | v4 | v5 |
|-----------|----|----|----|----|----|
| Muffin (WM) | 🔴 External | 🔴 External | 🔴 External | 🔴 Custom | ✅ EduWM |
| cinnamon-session | 🔴 External | 🔴 External | 🔴 EduSession | ✅ Custom | ✅ Custom |
| cinnamon-screensaver | 🔴 External | 🔴 EduLock | ✅ Custom | ✅ Custom | ✅ Custom |
| cinnamon-background | 🔴 External | 🔴 EduWallpaper | ✅ Custom | ✅ Custom | ✅ Custom |
| cinnamon-applet API | 🔴 External | 🔴 External | 🔴 Custom | ✅ Custom | ✅ Custom |
| cinnamon-settings | 🔴 External | 🔴 Partial | 🔴 Custom | ✅ Custom | ✅ Custom |
| Nemo (file manager) | 🔴 External | 🔴 External | 🔴 EduFiles | ✅ Custom | ✅ Custom |
| Cinnamon OSD | 🔴 External | 🔴 Custom | ✅ Custom | ✅ Custom | ✅ Custom |
| Cinnamon tray | 🔴 External | 🔴 Custom | ✅ Custom | ✅ Custom | ✅ Custom |
| Mutter (compositor) | 🔴 External | 🔴 External | 🔴 External | 🔴 EduComp | ✅ Custom |
| SDDM/LightDM | 🔴 External | 🔴 Themed | 🔴 Themed | 🔴 Themed | 🔴 EduGrtr |

🔴 = Not EduShell | 🔴 = Partially EduShell | ✅ = Fully EduShell

---

## Risk Assessment for Cinnamon Dependency

| Risk | Impact | Mitigation |
|------|--------|------------|
| Cinnamon v7 breaks DBus API | High | Abstraction layer catches API changes; CI tests against multiple Cinnamon versions |
| Muffin drops X11 support before EduWM ready | Medium | Track Muffin Wayland progress; prepare X11 fallback session |
| cinnamon-session changes startup protocol | High | Monitor upstream; document startup sequence explicitly |
| Cinnamon Wayland session doesn't support layer-shell | Critical | X11 fallback; if layer-shell is missing, work with Cinnamon team to add it |
| Cinnamon applet API deprecated | Low | applet-bridge provides compatibility shim or deprecation warning |
