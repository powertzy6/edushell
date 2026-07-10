# Component Replacement Matrix

## Legend
- **Use** — keep dependency as-is
- **Fork** — copy source, maintain separately
- **Rewrite** — write from scratch
- **Replace** — use existing EduShell module
- **Hybrid** — wrap then incrementally replace

| # | Component | Strategy | Reason | Existing Asset | Effort | Priority |
|---|-----------|----------|--------|----------------|--------|----------|
| 1 | Muffin | Fork→Replace | Very high complexity, needs gradual takeover | — | XXL | P0 |
| 2 | Nemo | Replace | FileManagerIntegration already handles quick-access | `file_manager` | S | P1 |
| 3 | Cinnamon Settings | Fork | High complexity, many backends; fork first, rewrite later | — | XL | P0 |
| 4 | Control Center | Replace | Can build atop EduShell UI Kit + SettingsCenter | `settings_center` | M | P2 |
| 5 | Cinnamon Session | Rewrite | Simple — `EduShellSession` already exists | `session` | S | P0 |
| 6 | Cinnamon Panel | Hybrid | Wrap current panel, replace incrementally | — | XL | P1 |
| 7 | Cinnamon Applets | Replace | Extension Framework v2 + Widget SDK | `plugin`, `widget` | L | P2 |
| 8 | Cinnamon Extensions | Replace | Extension Framework v2 + Plugin API | `plugin` | L | P2 |
| 9 | Theme Engine | Replace | EduShell Theme Engine + Theme SDK | `theme`, `Theme SDK` | S | P0 |
| 10 | Launcher | Rewrite | GlobalSearch + Launcher API | `search` | M | P2 |
| 11 | Workspace | Rewrite | Workspace Engine (new) | — | L | P3 |
| 12 | Notification | Replace | Notification API in SDK | `api::Notification` | S | P0 |
| 13 | Power | Replace | UPower DBus direct | — | S | P1 |
| 14 | Network | Replace | NetworkManager DBus direct | — | S | P1 |
| 15 | Bluetooth | Replace | BlueZ DBus direct | — | S | P1 |
| 16 | Background | Replace | WallpaperManager exists | `wallpapers` | S | P0 |
| 17 | Search | Replace | GlobalSearch exists | `search` | S | P0 |
| 18 | Calendar | Replace | Widget SDK CalendarWidget | `widget::WidgetCategory::Calendar` | S | P0 |
| 19 | Clock | Replace | Widget SDK ClockWidget | `widget::ClockWidget` | S | P0 |
| 20 | Sound | Replace | PulseAudio DBus direct | — | S | P1 |
| 21 | Accessibility | Use | Keep AT-SPI for now; strengthen later | — | — | P4 |
| 22 | Desktop | Rewrite | Window API + Desktop API (new) | — | XXL | P3 |
| 23 | Context Menu | Replace | Extension Framework | — | M | P2 |
| 24 | System Tray | Replace | StatusNotifierItem implementation | — | M | P2 |

## Priority Groups

### P0 — v2.0 Critical (must ship)
Session, Theme Engine, Notification, Background, Search, Calendar, Clock, Settings (fork)

### P1 — v2.2 Important
Panel (hybrid), Nemo replacement, Power, Network, Bluetooth, Sound

### P2 — v2.5 Strategic
Control Center, Applets, Extensions, Launcher, Context Menu, System Tray

### P3 — v3.0 Foundation
Workspace Engine, Desktop API, Muffin fork begins

### P4 — v4.0+ Future
Full Muffin replacement, full Desktop Environment independence, Accessibility rewrite
