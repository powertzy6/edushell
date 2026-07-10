# EduShell Roadmap — v1 to v5

## Version Strategy
- **Semantic Versioning**: `MAJOR.MINOR.PATCH`
- Major version untuk architectural changes (v1 → v2 = major)
- Minor version untuk fitur baru
- Patch untuk bugfix

---

## v1.0 — "Foundation" (Target: Q4 2026)

### Goal
Desktop Shell yang berfungsi penuh di atas Cinnamon Desktop.

### What's Built
| Komponen | Status | Engine |
|----------|--------|--------|
| EduShell Panel | New (Vala/GTK4) | Custom |
| Edu Launcher | New (Vala/GTK4) | Custom |
| Workspace Switcher | New (Vala/GTK4) | Custom |
| System Tray | New (Vala/GTK4) | Custom |
| Notification Center | New (Vala/GTK4) | Custom |
| Quick Settings | New (Vala/GTK4) | Custom |
| Edu Settings | New (Vala/GTK4) | Custom |
| Learning Hub | New (HTML/GTK4 WebView) | Custom |
| Theme (GTK + Shell) | New (CSS/SCSS) | Custom |
| Icon Theme | Adapted | Custom (Papirus-based) |

### What's Borrowed from Cinnamon
| Komponen | Notes | Replacement Plan |
|----------|-------|------------------|
| Window Manager (Muffin) | Stable, mature | v4 — Custom WM |
| Session Manager | cinnamon-session | v3 — Custom session |
| Display Manager integration | LightDM/SDDM | v2 — Edu greeter theme |
| Network Manager applet | Via Cinnamon backend | v2 — Native |
| Audio backend | PulseAudio/WirePlumber | v2 — Native |
| Screensaver/Lock | cinnamon-screensaver | v2 — Custom |
| Background manager | Cinnamon desktop | v2 — Custom |
| File manager (Nemo) | Separate app | v3 — Fork/alternative |

### v1.0 Deliverables
- [ ] Panel (bottom, configurable)
- [ ] Edu Launcher with search
- [ ] Workspace switcher (min 4 ws)
- [ ] System tray (network, volume, battery, clock)
- [ ] Quick Settings toggle panel
- [ ] Notification center
- [ ] User menu (lock, logout, shutdown)
- [ ] Edu Settings (panel, launcher, theme, language, accessibility)
- [ ] Learning Hub (local content)
- [ ] Theme (light + dark, high contrast)
- [ ] Translation system (id, en)
- [ ] Keyboard navigation
- [ ] Screen reader support
- [ ] Installer package (.deb)
- [ ] Documentation (user + developer)
- [ ] CI/CD pipeline

---

## v2.0 — "Ecosystem" (Target: Q2 2027)

### Goal
Desktop Shell + Aplikasi edukasi bawaan.

### New Components
| Komponen | Type | Notes |
|----------|------|-------|
| EduApps (Calculator, Notes, Dictionary) | New (Vala/GTK4) | Aplikasi edukasi ringan |
| Edu Browser | Adapted | Firefox atau GNOME Web kustom |
| Native network manager | New | Ganti NM applet Cinnamon |
| Native audio manager | New | Ganti PA/WP applet |
| Custom screensaver/lock screen | New | |
| Desktop icons manager | New | Ganti Nemo desktop |
| ARM64 support | Infrastructure | Raspberry Pi, Chromebook |

### What's Forked/Replaced from Cinnamon
- Network manager → Edu Network
- Audio manager → Edu Audio
- Screensaver → Edu Lock
- Desktop background → Edu Wallpaper Manager

---

## v3.0 — "Independence" (Target: Q1 2028)

### Goal
Desktop Shell + Fork beberapa komponen inti Cinnamon.

### New Components
| Komponen | Type | Notes |
|----------|------|-------|
| Cinnamon fork (partial) | Fork | Hanya komponen yang diperlukan |
| Rust-based performance modules | New (Rust) | Search index, file watcher |
| Custom session manager | New | Ganti cinnamon-session |
| Nemo fork / alternative | Fork | Edu File Manager |
| Plugin system stabil | API | Untuk ekstensi pihak ketiga |

### What's Forked/Replaced
- cinnamon-session → Edu Session
- Nemo → Edu Files (fork/rewrite)
- Cinnamon DBus API → Edu API

---

## v4.0 — "Window Manager" (Target: Q3 2028)

### Goal
Desktop Shell + Window Manager sendiri (fork Muffin atau rewrite).

### New Components
| Komponen | Type | Notes |
|----------|------|-------|
| Edu WM | Fork or New (Rust) | Berbasis wlroots atau fork Muffin |
| Compositor | New (Rust) | Wayland compositor sendiri |
| Window effects | New | Transitions, animations |
| Tiling support | New | Advanced window management |

### What's Replaced
- Muffin → Edu WM
- Meta (Mutter) → Edu Compositor

---

## v5.0 — "Desktop Environment" (Target: Q2 2029)

### Goal
Desktop Environment mandiri penuh.

### New Components
| Komponen | Type | Notes |
|----------|------|-------|
| Edu Display Server | New | Wayland compositor (rewrite) |
| Edu Settings (system-wide) | New | Ganti system settings |
| Edu Greeter | New | Login manager |
| Edu Apps (full suite) | New | Complete application stack |
| Edu Core Libraries | New | Shared foundation |
| Community ecosystem | Infrastructure | Extensions, themes, docs |

### What's Replaced
- Any remaining Cinnamon component
- LightDM/SDDM → Edu Greeter
- GNOME Control Center → Edu System Settings

---

## Summary Diagram

```
v1                    v2                    v3                    v4                    v5
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│ EduShell │    │ EduShell │    │ EduShell │    │ EduShell │    │ EduDE    │
│ Panel    │    │ Panel    │    │ Panel    │    │ Panel    │    │ Panel    │
│ Launcher │    │ Launcher │    │ Launcher │    │ Launcher │    │ Launcher │
│ Settings │    │ Settings │    │ Settings │    │ Settings │    │ Settings │
│ Learning │    │ Learning │    │ Learning │    │ Learning │    │ Learning │
│ Hub      │    │ Hub      │    │ Hub      │    │ Hub      │    │ Hub      │
├──────────┤    ├──────────┤    ├──────────┤    ├──────────┤    ├──────────┤
│ Cinnamon │    │ EduApps  │    │ EduApps  │    │ EduApps  │    │ EduApps  │
│ (base)   │    │ Edu Net  │    │ Edu Net  │    │ Edu Net  │    │ Edu Net  │
│ Muffin   │    │ Edu Audio│    │ Edu Audio│    │ Edu Audio│    │ Edu Audio│
│ (WM)     │    ├──────────┤    ├──────────┤    ├──────────┤    ├──────────┤
│          │    │ Cinnamon │    │ Edu Files│    │ Edu Files│    │ Edu Files│
│          │    │ Muffin   │    │ Edu Sess │    │ Edu Sess │    │ Edu Sess │
│          │    │          │    ├──────────┤    ├──────────┤    ├──────────┤
│          │    │          │    │ Cinnamon │    │ Edu WM   │    │ Edu WM   │
│          │    │          │    │ Muffin   │    ├──────────┤    ├──────────┤
│          │    │          │    │          │    │ Cinnamon │    │ Edu Disp │
│          │    │          │    │          │    │ (min)    │    │ Edu Grtr │
│          │    │          │    │          │    │          │    └──────────┘
└──────────┘    └──────────┘    └──────────┘    └──────────┘
    2026            2027            2028            2028            2029
```

## Between Versions

| Version | Focus |
|---------|-------|
| v1.x | Bug fixes, stability, translation updates, Learning Hub content |
| v2.x | EduApps development, performance optimization |
| v3.x | Rust migration, session stability, API stabilization |
| v4.x | WM stability, Wayland protocol compliance |
| v5.x | Ecosystem growth, community engagement |
