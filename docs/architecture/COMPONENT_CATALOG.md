# Component Catalog — EduShell Full Project

## Naming Convention
- `[Status]` = ✅ Built | 🔧 In Progress | 📋 Planned | ❌ Not Started
- `[Layer]` = Shell | App | Lib | Bridge | System

---

## v1 Components (All ✅ Required for v1)

### Shell Layer Components

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| C001 | **EduPanel** | ✅ v1 | Main desktop panel (bottom default). Contains menu button, running apps, system tray, clock. Configurable position, autohide, transparency. | Vala/GTK4 | Cinnamon Panel |
| C002 | **EduLauncher** | ✅ v1 | Application launcher with search, categories, favorites, recent apps. Activates via click or Super key. | Vala/GTK4 | Cinnamon Menu |
| C003 | **EduWorkspace** | ✅ v1 | Workspace switcher and manager. Thumbnail preview, keyboard shortcuts, up to 32 workspaces. | Vala/GTK4 | Cinnamon Workspace |
| C004 | **EduTray** | ✅ v1 | System tray area: network, volume, battery, clock, notifications, quick settings toggle. | Vala/GTK4 | Cinnamon Tray |
| C005 | **EduNotifications** | ✅ v1 | Notification center with history, Do Not Disturb mode, per-app settings. | Vala/GTK4 | Cinnamon Notifications |
| C006 | **EduQuickSettings** | ✅ v1 | Quick settings popup: WiFi toggle, Bluetooth, Dark Mode, DND, Volume slider, Brightness. | Vala/GTK4 | New |
| C007 | **EduUserMenu** | ✅ v1 | User menu: account settings, lock screen, switch user, logout, suspend, restart, shutdown. | Vala/GTK4 | Cinnamon User Menu |
| C008 | **EduOSD** | ✅ v1 | On-screen display: volume change, brightness, media playback info. | Vala/GTK4 | Cinnamon OSD |

### Application Layer Components

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| A001 | **EduSettings** | ✅ v1 | Central settings application for EduShell. Panel, launcher, theme, language, accessibility, shortcuts. | Vala/GTK4 | Cinnamon Settings (partial) |
| A002 | **LearningHub** | ✅ v1 | Educational content portal. Built-in guides for Linux basics, desktop tips, keyboard shortcuts, recommended apps. | Vala/GTK4 + HTML | New |

### Library Layer Components

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| L001 | **libedushell-core** | ✅ v1 | Core library: config management, logging, IPC, error handling, utilities. | Vala/C | New |
| L002 | **libcinnamon-adapter** | ✅ v1 | Abstraction layer over Cinnamon APIs. Enables future replacement of Cinnamon dependencies. | Vala/C | New |
| L003 | **libsettings-backend** | ✅ v1 | Settings persistence using GSettings/GSchema with fallback to keyfile. | Vala | New |
| L004 | **libtranslation** | ✅ v1 | gettext wrapper for i18n. Indonesian default, English fallback. | Vala | New |
| L005 | **libtheme-engine** | ✅ v1 | Theme management: load, apply, switch themes. CSS-based with dynamic reload. | Vala | New |

### Bridge Layer Components

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| B001 | **session-bridge** | ✅ v1 | Integration with cinnamon-session lifecycle. Handles startup, shutdown, suspend. | Vala | New |
| B002 | **applet-bridge** | ✅ v1 | Compatibility layer for loading Cinnamon applets within EduShell. | Vala/JS | New |
| B003 | **background-bridge** | ✅ v1 | Desktop background/wallpaper management via Cinnamon. | Vala | New |

### Theme Components

| ID | Component | Status | Description | Format | Replaces |
|----|-----------|--------|-------------|--------|----------|
| T001 | **EduTheme-GTK** | ✅ v1 | GTK4 theme (light + dark variants) | CSS/SCSS | Adwaita |
| T002 | **EduTheme-Shell** | ✅ v1 | Shell theme (panel, launcher, OSD styling) | CSS | Cinnamon theme |
| T003 | **EduTheme-Icons** | ✅ v1 | Icon theme (adapted from Papirus with Edu modifications) | SVG | Mint-X / Papirus |
| T004 | **EduTheme-Cursor** | 📋 v1 | Cursor theme | X11 cursor | DMZ |
| T005 | **EduWallpapers** | ✅ v1 | Default wallpaper set | PNG/JPG | Mint backgrounds |

---

## v2 Components (📋 Planned)

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| A003 | **EduCalculator** | 📋 v2 | Simple calculator app | Vala/GTK4 | New |
| A004 | **EduNotes** | 📋 v2 | Sticky notes / quick notes app | Vala/GTK4 | New |
| A005 | **EduDictionary** | 📋 v2 | Indonesian-English dictionary | Vala/GTK4 | New |
| A006 | **EduNetwork** | 📋 v2 | Native network manager UI | Vala/GTK4 | Cinnamon NM applet |
| A007 | **EduAudio** | 📋 v2 | Native audio manager UI | Vala/GTK4 | Cinnamon audio applet |
| A008 | **EduLock** | 📋 v2 | Custom lock screen / screensaver | Vala/GTK4 | cinnamon-screensaver |
| A009 | **EduWallpaper** | 📋 v2 | Wallpaper manager with dynamic/auto change | Vala | Cinnamon background |
| L006 | **libsearch** | 📋 v2 | Search index for launcher file/app/content search | Vala/C | Tracker/Miner |
| B004 | **greeter-theme** | 📋 v2 | SDDM/LightDM theme matching EduShell | QML/HTML | SDDM default |

---

## v3 Components (📋 Planned)

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| A010 | **EduFiles** | 📋 v3 | File manager (Nemo fork/alternative) | Vala/C | Nemo |
| A011 | **EduSession** | 📋 v3 | Custom session manager | Vala/C | cinnamon-session |
| A012 | **EduApps-Full** | 📋 v3 | Full suite: terminal, text editor, archive manager | Various | Various |
| L007 | **libcore-rust** | 📋 v3 | Rust-based core for performance-critical parts | Rust | libedushell-core (partial) |
| L008 | **libplugin** | 📋 v3 | Plugin system API | Vala/Rust | Cinnamon applet API |
| T006 | **EduTheme-Shell-v2** | 📋 v3 | Shell theme v2 with full customization | CSS/SCSS | EduTheme-Shell |

---

## v4 Components (📋 Planned)

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| C009 | **EduWM** | 📋 v4 | Window Manager (wlroots-based or Muffin fork) | Rust/C | Muffin |
| C010 | **EduCompositor** | 📋 v4 | Wayland compositor | Rust/C | Mutter |
| C011 | **EduEffects** | 📋 v4 | Window animations and effects | Rust/C | Mutter effects |

---

## v5 Components (📋 Planned)

| ID | Component | Status | Description | Language | Replaces |
|----|-----------|--------|-------------|----------|----------|
| A013 | **EduDisplayServer** | 📋 v5 | Complete Wayland compositor rewrite | Rust | EduCompositor |
| A014 | **EduGreeter** | 📋 v5 | Login manager | Rust/GTK | SDDM/LightDM |
| A015 | **EduSystemSettings** | 📋 v5 | System-wide settings (replaces GNOME Control Center) | Vala/Rust | gnome-control-center |
| L009 | **libedushell-platform** | 📋 v5 | Complete platform library | Rust | All previous libs |
