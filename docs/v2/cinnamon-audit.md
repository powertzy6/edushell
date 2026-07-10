# Cinnamon Dependency Audit — EduShell v1.0 → v2.0 Decoupling

## Purpose
Map every Cinnamon component EduShell depends on, classify by coupling severity, and recommend a decoupling strategy for each.

---

## Audit Table

| # | Component | Dependency Type | Coupling | License | Complexity | Strategy | Risk | Target Version |
|---|-----------|----------------|----------|---------|------------|----------|------|----------------|
| 1 | **Muffin** (wm) | Binary (launched) | Runtime call | GPL-2+ | Very High | Fork → Rewrite | Critical | v4.0 |
| 2 | **Nemo** (fm) | DBus / spawn | IPC calls | GPL-2+ | High | Replace (FileManagerIntegration exists) | Medium | v2.0 |
| 3 | **Cinnamon Settings** | DBus | Tight | GPL-2+ | High | Fork | High | v2.0 |
| 4 | **Cinnamon Control Center** | DBus | Tight | GPL-2+ | High | Replace | High | v2.5 |
| 5 | **Cinnamon Session** | Startup script | Runtime | GPL-2+ | Medium | Rewrite (EduShell Session exists) | Low | v2.0 |
| 6 | **Cinnamon Applet API** | JS API | Tight | GPL-2+ | High | Replace with Extension Framework | High | v2.5 |
| 7 | **Cinnamon Extension API** | JS API | Tight | GPL-2+ | High | Replace with Plugin API v2 | High | v2.5 |
| 8 | **Cinnamon Theme Engine** | CSS/JSON | Loose | GPL-2+ | Medium | Rewrite (EduShell Theme Engine exists) | Low | v2.0 |
| 9 | **Cinnamon Panel** | Binary | Runtime | GPL-2+ | Very High | Hybrid → Rewrite | Very High | v2.2 |
| 10 | **Cinnamon Launcher** | DBus | Tight | GPL-2+ | High | Rewrite | High | v2.5 |
| 11 | **Cinnamon Workspace** | IPC | Tight | GPL-2+ | Medium | Rewrite (Workspace Engine) | Medium | v3.0 |
| 12 | **Cinnamon Notification** | DBus | Runtime | GPL-2+ | Medium | Replace (Notification API exists) | Low | v2.0 |
| 13 | **Cinnamon Power** | DBus | Runtime | GPL-2+ | Medium | UPower direct integration | Low | v2.0 |
| 14 | **Cinnamon Network** | DBus | Runtime | GPL-2+ | Medium | NetworkManager direct | Low | v2.0 |
| 15 | **Cinnamon Bluetooth** | DBus | Runtime | GPL-2+ | Low | BlueZ direct | Low | v2.0 |
| 16 | **Cinnamon Background** | DBus | Runtime | GPL-2+ | Low | Replace (WallpaperManager exists) | Low | v2.0 |
| 17 | **Cinnamon Search** | DBus | Tight | GPL-2+ | Medium | Replace (GlobalSearch exists) | Low | v2.0 |
| 18 | **Cinnamon Calendar** | Applet | Tight | GPL-2+ | Low | Replace (Widget SDK) | Low | v2.0 |
| 19 | **Cinnamon Clock** | Applet | Tight | GPL-2+ | Low | Replace (Widget SDK) | Low | v2.0 |
| 20 | **Cinnamon Sound** | DBus | Runtime | GPL-2+ | Low | PulseAudio/WirePlumber direct | Low | v2.0 |
| 21 | **Cinnamon Accessibility** | DBus | Runtime | GPL-2+ | High | ATK/AT-SPI direct | Medium | v3.0 |
| 22 | **Cinnamon Desktop** | IPC | Tight | GPL-2+ | Very High | Window API + Desktop API | Very High | v4.0 |
| 23 | **Cinnamon Context Menu** | JS API | Tight | GPL-2+ | Medium | Extension Framework | Medium | v2.5 |
| 24 | **Cinnamon System Tray** | Applet | Tight | GPL-2+ | Medium | StatusNotifierItem standard | Medium | v2.5 |

## Coupling Categories

- **Runtime call** — launches process, minimal API dependency (easy to replace)
- **IPC calls** — DBus/pipe communication (medium difficulty)
- **Tight** — direct API linkage, deep integration (high difficulty)
- **Binary** — spawns/requires specific binary (medium difficulty)

## Summary

| Severity | Count | Components |
|----------|-------|------------|
| Low (replace in v2.0) | 10 | Nemo, Session, Theme, Notification, Power, Network, BT, Background, Search, Calendar, Clock, Sound |
| Medium (replace in v2.2-2.5) | 6 | Panel, Applet, Extension, Launcher, Context Menu, System Tray |
| High (replace in v3.0-4.0) | 4 | Muffin, Workspace, Desktop, Accessibility |
| Very High (replace in v4.0-5.0) | 2 | Muffin (full replacement), Desktop API |

## Decoupling Roadmap

```
v2.0 ─── v2.2 ─── v2.5 ─── v3.0 ─── v4.0 ─── v5.0
  │        │        │        │        │        │
  ├Session  ├Panel   ├Launcher├Muffin  ├WM API  ├Full DE
  ├Theme    ├Applets ├Extension├Work-   ├Desktop ├Indepen-
  ├Notify   ├CtxMenu ├Search   │space   │API     │dent
  ├Power    ├SysTray │Engine   │        │        │
  ├Network  │        │         │        │        │
  ├BT       │        │         │        │        │
  ├Bg       │        │         │        │        │
  ├Calendar │        │         │        │        │
  ├Clock    │        │         │        │        │
  └Sound    │        │         │        │        │
            │        │         │        │        │
  10 low    6 med   4 medium  2 high   2 high   Complete
  replaced  done    done      done     done     decoupling
```
