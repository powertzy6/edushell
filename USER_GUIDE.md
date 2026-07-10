# EduShell User Guide

A complete reference for using the EduShell desktop environment v1.0.0.

---

## Getting Started

### Launching EduShell

After installation, select **EduShell** from your display manager's session selector at the login screen. The session is available under both X11 (via XWayland) and native Wayland modes.

On first launch, EduShell applies the default Adwaita Dark theme, creates four virtual workspaces, and positions the panel at the bottom of the screen.

### Desktop Overview

The EduShell desktop consists of the following elements:

- **Panel** — Bottom bar containing the launcher button, running app indicators, workspace switcher, system tray, clock, and notification area
- **Desktop** — The workspace area where application windows are placed
- **Launcher** — Full-screen or pop-up app launcher for finding and opening applications
- **Notifications** — Pop-up banners and a notification center accessible from the panel
- **Settings** — Centralized configuration for theme, display, keyboard, and accessibility

---

## Panel

The panel is the central control bar of the EduShell desktop.

### Layout

From left to right, the default panel layout is:

1. **App Launcher Button** — Opens the application launcher
2. **Pinned Apps** — Quick-launch icons for frequently used applications
3. **Running Apps** — Indicators for currently open windows
4. **Separator**
5. **Workspace Switcher** — Displays available workspaces with click-to-switch
6. **Separator**
7. **System Tray** — Network, volume, battery, and other status icons
8. **Clock and Date** — Click for calendar popup
9. **Notification Indicator** — Shows unread notification count

### Customizing the Panel

Right-click any empty area of the panel to access panel settings:

- **Position:** Top, Bottom, Left, or Right of screen
- **Size:** Small (24px), Medium (32px), Large (40px)
- **Auto-hide:** Panel hides when not hovered; reveals on mouse approach
- **Transparency:** Adjustable opacity (0–100%)
- **Expansion:** Fill screen width or fit content

### Adding Applets

Right-click the panel > **Add Applet** to browse available applets:

- **Clock** — Customizable date/time display with timezone support
- **Weather** — Weather conditions and forecast
- **System Monitor** — CPU, memory, and network usage graph
- **Workspace Names** — Custom labels for each workspace
- **Custom Script** — Run arbitrary scripts and display output

Applets can be reordered by dragging them along the panel.

### Removing Applets

Right-click an applet > **Remove from Panel**. The applet remains available for re-adding.

---

## Launcher

The application launcher provides fast access to all installed applications.

### Opening the Launcher

- Click the **Launcher Button** on the panel (far left)
- Press `Super` key (default)
- Press `Alt + F2` for a quick-run dialog

### Searching

Type immediately after opening the launcher to search:

- Search matches application names, descriptions, and categories
- Results are ranked by relevance and recent usage
- Arrow keys navigate results; `Enter` launches the selected app

### Keyboard Navigation

| Key              | Action                           |
| ---------------- | -------------------------------- |
| `Up` / `Down`    | Navigate results                 |
| `Enter`          | Launch selected application      |
| `Escape`         | Close launcher                   |
| `Tab`            | Switch between apps and settings |
| `Ctrl + L`       | Focus search field               |

### Pinning Applications

Right-click any application in the launcher > **Pin to Panel** to add it to the panel's quick-launch area.

### Recent Applications

The launcher tracks recently used applications and shows them at the top when opened without a search query.

---

## Workspaces

EduShell provides virtual workspaces for organizing application windows.

### Default Configuration

- 4 workspaces are created by default
- Workspaces are arranged in a horizontal strip
- Each workspace is independent and has its own set of windows

### Switching Workspaces

| Method              | Action                                        |
| ------------------- | --------------------------------------------- |
| Panel click         | Click a workspace indicator in the panel      |
| Keyboard            | `Ctrl + Alt + Left` / `Ctrl + Alt + Right`    |
| Scroll              | `Ctrl + Scroll` over the workspace area       |
| Overview            | `Super + Tab` to enter workspace overview     |

### Workspace Overview

Press `Super + Tab` to enter the workspace overview:

- All workspaces are displayed as thumbnails
- Windows within each workspace are visible
- Click a workspace to switch to it
- Click a window to focus it
- Drag windows between workspaces by dragging their thumbnail

### Adding Workspaces

- Right-click a workspace indicator > **Add Workspace**
- From overview mode, click **+** at the end of the workspace strip

### Removing Workspaces

- Right-click a workspace indicator > **Remove Workspace**
- Workspaces with open windows cannot be removed; move or close windows first

### Naming Workspaces

Right-click a workspace indicator > **Rename** to assign a custom label (e.g., "Browser", "Code", "Terminal").

---

## Notifications

EduShell provides a unified notification system.

### Viewing Notifications

- New notifications appear as banners in the top-right corner
- Click a banner to open the associated application
- Banners auto-dismiss after 5 seconds (configurable)

### Notification Center

- Click the **Notification Indicator** in the panel (right side)
- A dropdown shows all recent notifications
- Notifications are grouped by application
- Click a notification to open it; swipe or click **X** to dismiss
- **Clear All** button removes all notifications

### Notification Settings

Access via Settings > Notifications:

| Setting              | Options                                        |
| ------------------- | ---------------------------------------------- |
| Show banners        | On / Off                                       |
| Banner duration     | 3s / 5s / 10s / Until dismissed                |
| Sound               | On / Off                                       |
| Do Not Disturb      | On / Off (mutes all notifications)             |
| Per-app control     | Enable/disable notifications per application   |

### Do Not Disturb

Toggle Do Not Disturb from the notification center or Settings. When active, all notification banners are suppressed and the indicator shows a "DND" badge. Notifications are still collected in the center.

---

## Settings

The EduShell Settings application provides centralized configuration.

### Launching Settings

- Click **Settings** in the launcher
- Right-click the desktop > **Settings**
- Press `Super + ,`

### Theme

| Setting         | Description                                      |
| --------------- | ------------------------------------------------ |
| Appearance      | Light, Dark, or System (follows GTK theme)       |
| Accent Color    | Choose from predefined accent colors              |
| Cursor Theme    | Select cursor theme and size                      |
| Icon Theme      | Select icon set                                   |
| Font            | Interface font, size, and monospace font          |
| Window Controls | Button layout (minimize, maximize, close)        |

### Display

| Setting         | Description                                      |
| --------------- | ------------------------------------------------ |
| Resolution      | Screen resolution per display                     |
| Scaling         | Fractional scaling (100%–200%)                    |
| Refresh Rate    | Monitor refresh rate                              |
| Multi-Monitor   | Arrangement (left/right/above/below), primary     |
| Night Light     | Blue light filter with schedule                   |

### Accessibility

| Setting         | Description                                      |
| --------------- | ------------------------------------------------ |
| High Contrast   | Increase color contrast for visibility            |
| Large Text      | Increase base font size                          |
| Cursor Size     | Increase pointer size                             |
| Screen Reader   | Enable Orca screen reader integration             |
| Keyboard        | Sticky keys, slow keys, bounce keys               |
| Visual Alerts   | Flash screen for system alerts                    |

### Keyboard

| Setting         | Description                                      |
| --------------- | ------------------------------------------------ |
| Shortcuts       | View and customize all keyboard shortcuts         |
| Layout          | Add/remove keyboard layouts                       |
| Repeat Keys     | Key repeat delay and speed                        |

### Power

| Setting         | Description                                      |
| --------------- | ------------------------------------------------ |
| Power Button    | Suspend, shutdown, hibernate, or ask              |
| Lid Close       | Suspend or do nothing                             |
| Automatic Suspend| Idle timeout before suspend                      |
| Battery         | Low battery notification and action               |

---

## Keyboard Shortcuts Reference

### Window Management

| Shortcut                  | Action                         |
| ------------------------- | ------------------------------ |
| `Super + Up`              | Maximize window                |
| `Super + Down`            | Restore/minimize window        |
| `Super + Left`            | Snap window to left half       |
| `Super + Right`           | Snap window to right half      |
| `Super + M`               | Minimize window                |
| `Super + Q`               | Close window                   |
| `Alt + F4`                | Close window                   |
| `Alt + F5`                | Restore window                 |
| `Super + F`               | Toggle fullscreen              |

### Workspace Navigation

| Shortcut                  | Action                         |
| ------------------------- | ------------------------------ |
| `Ctrl + Alt + Left`       | Switch to workspace left       |
| `Ctrl + Alt + Right`      | Switch to workspace right      |
| `Ctrl + Alt + 1–4`        | Switch to workspace 1–4        |
| `Super + Tab`             | Workspace overview             |
| `Shift + Ctrl + Alt + Left` | Move window to workspace left |
| `Shift + Ctrl + Alt + Right` | Move window to workspace right|

### Launching and Navigation

| Shortcut                  | Action                         |
| ------------------------- | ------------------------------ |
| `Super`                   | Open launcher                  |
| `Alt + F2`                | Quick-run dialog               |
| `Super + ,`               | Open Settings                  |
| `Super + L`               | Lock screen                    |
| `Print Screen`            | Screenshot (full screen)       |
| `Shift + Print Screen`    | Screenshot (selection)         |

### Panel and UI

| Shortcut                  | Action                         |
| ------------------------- | ------------------------------ |
| `Super + B`               | Toggle panel visibility        |
| `Super + N`               | Toggle notification center     |
| `Super + D`               | Show desktop (minimize all)    |
| `Ctrl + Alt + Delete`     | Session menu (logout/shutdown) |

---

## Tips and Tricks

### Quick Application Switching

Press `Alt + Tab` to cycle through open windows. Hold `Shift` to cycle in reverse direction.

### Window Tiling

Hold `Super` and press an arrow key to snap windows to halves or quarters of the screen. EduShell supports quarter-tiling with diagonal arrow combinations.

### Middle-Click Actions

- **Middle-click on panel app:** Open a new instance
- **Middle-click on notification:** Dismiss it
- **Middle-click on workspace:** Close the workspace (if empty)

### Custom Shortcuts

Go to Settings > Keyboard > Shortcuts and click **Add Custom Shortcut** to bind any command to a keyboard combination.

### Recent Files

The launcher tracks recently opened files and documents. Open the launcher and switch to the **Recent** tab.

### Accessibility Quick Toggle

Press `Super + A` to toggle high-contrast mode instantly, regardless of current settings.

### Command Palette

Press `Ctrl + Shift + P` within any EduShell application to open a command palette for quick access to functions.

### Session Recovery

EduShell saves your window layout on logout. When you log back in, windows are restored to their previous positions and workspaces.

### Performance Mode

For battery-powered devices, enable **Performance Mode** in Settings > Power to reduce background activity and extend battery life.

### Multiple Panels

You can create additional panels on any edge of the screen. Right-click the desktop > **Add Panel** and configure its contents independently.
