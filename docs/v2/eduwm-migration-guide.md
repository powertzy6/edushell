# EduWM Window Manager — Migration Guide: Muffin → EduWM

## 1. Overview

This guide covers the complete migration path from Muffin (the current Cinnamon window manager) to EduWM. It is intended for system administrators, distribution maintainers, and advanced users who want to upgrade their desktop environment.

### 1.1 Why EduWM?

Muffin has served the Cinnamon desktop well, but it carries significant technical debt:

| Issue | Muffin Limitation | EduWM Solution |
|-------|-------------------|----------------|
| **X11 Dependency** | Muffin is X11-first; Wayland support is experimental | EduWM is Wayland-native with XWayland compatibility |
| **Tight Coupling** | Muffin is tightly coupled to Cinnamon libraries | EduWM is decoupled; communicates via D-Bus IPC |
| **Performance** | Full-screen damage tracking, no frame pacing | Per-pixel damage tracking, triple buffering, present feedback |
| **Animation System** | Clutter-based (limited, aging) | Custom animation engine with spring physics, adaptive performance |
| **Theming** | GTK/Metacity theme formats | CSS-like native theme language with hot-reload |
| **Security** | No client isolation | Per-surface isolation, permission model |
| **Crash Recovery** | Session restart | Session save/restore with Muffin fallback |
| **Multi-Monitor** | RandR-based | DRM-native with hotplug and fractional scaling |

### 1.2 Key Benefits

- **Performance**: 30-50% reduction in compositor CPU usage during animations.
- **Wayland-Native**: Full support for modern Wayland protocols.
- **Decoupled Architecture**: EduWM can evolve independently of Cinnamon.
- **Better Multi-Monitor**: Native DRM support with hotplug and fractional scaling.
- **Improved Security**: Per-window isolation and permission model.
- **Crash Resilience**: Automatic session save/restore with Muffin fallback.

---

## 2. Prerequisites

### 2.1 System Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| Linux Kernel | 5.4+ | 6.1+ |
| DRM/KMS Support | Yes | Atomic modesetting |
| OpenGL ES | 3.0 | 3.2 |
| Wayland | 1.18+ | 1.22+ |
| libinput | 1.14+ | 1.20+ |
| XWayland | 21.1+ | 23.0+ |
| D-Bus | 1.12+ | 1.14+ |
| GLib | 2.58+ | 2.70+ |
| Cairo | 1.16+ | 1.18+ |
| Pango | 1.42+ | 1.50+ |

### 2.2 Compatibility Matrix

| Component | Muffin Version | EduWM Version | Notes |
|-----------|---------------|---------------|-------|
| Cinnamon Shell | 5.4+ | 5.8+ | Shell must be updated first |
| Cinnamon Settings Daemon | 5.4+ | 5.8+ | |
| Nemo (File Manager) | 5.4+ | 5.8+ | |
| Cinnamon Screensaver | 5.4+ | 5.8+ | Uses session-lock protocol |
| Cinnamon Settings | 5.4+ | 5.8+ | Updated preference panels |

### 2.3 Backup Checklist

Before migrating, create backups of:

```bash
# 1. Cinnamon configuration
cp -r ~/.cinnamon ~/.cinnamon.backup.$(date +%Y%m%d)

# 2. Muffin configuration
dconf dump /org/cinnamon/muffin/ > ~/.muffin-settings-backup.dconf

# 3. Window list applet settings
dconf dump /org/cinnamon/applets/ > ~/.applets-backup.dconf

# 4. Desktop settings
dconf dump /org/cinnamon/ > ~/.cinnamon-all-backup.dconf

# 5. Custom themes
cp -r ~/.themes ~/.themes.backup.$(date +%Y%m%d)

# 6. Window manager keybindings
dconf dump /org/cinnamon/desktop/keybindings/ > ~/.keybindings-backup.dconf

# 7. Monitor configuration
xrandr --query > ~/monitor-config-backup.txt

# 8. Complete dconf dump
dconf dump / > ~/full-dconf-backup-$(date +%Y%m%d).dump
```

---

## 3. Step-by-Step Migration

### Phase 1: Pre-Migration (Week 1)

#### Step 1.1: Install EduWM Packages

```bash
# Add EduWM repository (distribution-specific)
# For Ubuntu/Debian:
sudo add-apt-repository ppa:cinnamon-team/eduwm
sudo apt update

# For Fedora:
sudo dnf install eduwm eduwm-wayland

# For Arch (AUR):
yay -S eduwm eduwm-wayland
```

#### Step 1.2: Verify System Compatibility

```bash
# Check DRM support
ls /sys/class/drm/card*/status

# Check OpenGL version
glxinfo | grep "OpenGL version"

# Check Wayland support
wayland-info

# Check libinput
libinput list-devices

# Run EduWM compatibility checker
eduwm-check --all
```

#### Step 1.3: Install Side-by-Side

EduWM can coexist with Muffin. Install both:

```bash
# Both packages can be installed simultaneously
sudo apt install eduwm muffin
```

### Phase 2: Configuration Migration (Week 2)

#### Step 2.1: Migrate Muffin Settings

EduWM includes a migration tool that converts Muffin settings:

```bash
# Run the migration tool
eduwm-migrate --from-muffin --output ~/.config/eduwm/

# Preview changes without applying
eduwm-migrate --from-muffin --dry-run

# Migrate specific settings
eduwm-migrate --from-muffin --settings window-list,workspace-count
```

**What Gets Migrated:**

| Setting | Muffin Location | EduWM Location | Migration |
|---------|----------------|----------------|-----------|
| Workspace count | `/org/cinnamon/muffin/workspace-count` | `~/.config/eduwm/workspaces.json` | Automatic |
| Window focus mode | `/org/cinnamon/muffin/focus-mode` | `~/.config/eduwm/input.json` | Automatic |
| Edge tiling | `/org/cinnamon/muffin/edge-tiling` | `~/.config/eduwm/window-rules.json` | Automatic |
| Titlebar actions | `/org/cinnamon/muffin/titlebar-*` | `~/.config/eduwm/decorations.json` | Automatic |
| Compositing manager | `/org/cinnamon/muffin/compositing-*` | `~/.config/eduwm/compositor.json` | Automatic |
| Keybindings | `/org/cinnamon/desktop/keybindings/` | `~/.config/eduwm/shortcuts.json` | Automatic |
| Window shortcuts | `/org/cinnamon/muffin/keybindings/` | `~/.config/eduwm/shortcuts.json` | Automatic |
| Theme | `/org/cinnamon/muffin/theme` | `~/.config/eduwm/theme.json` | Automatic |

#### Step 2.2: Migrate Window Rules

```bash
# Export Muffin's window rules
eduwm-migrate --export-window-rules > /tmp/muffin-rules.json

# Convert to EduWM format
eduwm-migrate --convert-rules /tmp/muffin-rules.json \
  --output ~/.config/eduwm/window-rules.json

# Validate converted rules
eduwm-migrate --validate-rules ~/.config/eduwm/window-rules.json
```

**Muffin → EduWM Rule Mapping:**

Muffin Rule:
```json
{
    "app": "org.gnome.Terminal",
    "pin": false,
    "above": false,
    "skip_taskbar": false
}
```

EduWM Rule:
```json
{
    "match": {
        "app_id": "org.gnome.Terminal"
    },
    "actions": {
        "sticky": false,
        "layer": 2,
        "skip_taskbar": false,
        "skip_pager": false
    }
}
```

#### Step 2.3: Migrate Themes

```bash
# Convert Metacity theme to EduWM CSS theme
eduwm-migrate --convert-theme \
  ~/.themes/Mint-Y/metacity-1/metacity-theme-3.xml \
  --output ~/.config/eduwm/themes/Mint-Y.css

# Test the converted theme
eduwm-theme-preview ~/.config/eduwm/themes/Mint-Y.css
```

### Phase 3: Plugin Migration (Week 3)

#### Step 3.1: Plugin Compatibility Check

```bash
# Check which installed plugins are compatible
eduwm-migrate --check-plugins

# Output:
# COMPATIBLE: window-list, workspace-switcher, notifications
# NEEDS UPDATE: calendar, system-monitor, nemo-desktop
# INCOMPATIBLE: (none)
```

#### Step 3.2: Migrate Plugin Settings

```bash
# Export plugin settings
dconf dump /org/cinnamon/applets/ > /tmp/applet-settings.dconf

# Convert to EduWM format
eduwm-migrate --convert-plugins /tmp/applet-settings.dconf \
  --output ~/.config/eduwm/plugins/
```

#### Step 3.3: Custom Applets

Custom applets that use the Cinnamon window management API may need updates:

1. Replace `global.screen` calls with EduWM D-Bus interface calls.
2. Replace `Meta.Window` usage with `EduWm.WindowProxy`.
3. Update workspace management calls.

**Migration Example:**

```javascript
// Before (Muffin/Cinnamon API)
let windows = global.get_window_actors();
let activeWorkspace = global.screen.get_active_workspace_index();

// After (EduWM API)
let windows = await EduWmDBus.WindowCore.ListWindows();
let activeWorkspace = await EduWmDBus.WorkspaceEngine.GetActiveWorkspace();
```

### Phase 4: Testing (Week 4)

#### Step 4.1: Boot into EduWM

```bash
# From login screen (LightDM/GDM):
# Select "Cinnamon (EduWM)" session

# Or manually start from TTY:
cinnamon-session --session=cinnamon-eduwm
```

#### Step 4.2: Run Test Suite

```bash
# Run comprehensive test suite
eduwm-test --suite full

# Run specific test categories
eduwm-test --category window-management
eduwm-test --category workspace
eduwm-test --category multi-monitor
eduwm-test --category input
eduwm-test --category animations
eduwm-test --category security
```

#### Step 4.3: Verify Core Functionality

Checklist:
- [ ] Windows open, close, minimize, maximize, fullscreen
- [ ] Window tiling (left/right half, quarters)
- [ ] Alt+Tab window switcher
- [ ] Workspace switching (keyboard shortcuts)
- [ ] Overview mode (Super key)
- [ ] Multi-monitor display
- [ ] Panel and taskbar function
- [ ] Notifications appear and dismiss
- [ ] Screensaver/lock screen works
- [ ] File manager drag-and-drop
- [ ] Clipboard operations (copy/paste)
- [ ] Window decorations (titlebar, close/min/max buttons)
- [ ] Window shadows
- [ ] Compositing effects (transparency, blur)
- [ ] Theme colors and fonts
- [ ] Screen resolution switching
- [ ] Screen rotation (if applicable)
- [ ] Hotplug monitors
- [ ] Virtual keyboard (if applicable)

---

## 4. Muffin API Equivalents in EduWM

### 4.1 Window Management API

| Muffin (libmuffin) | EduWM | Notes |
|--------------------|-------|-------|
| `meta_window_get_title()` | `edu_wm_window_get_title()` | Same behavior |
| `meta_window_get_app_id()` | `edu_wm_window_get_app_id()` | Wayland app_id or X11 WM_CLASS |
| `meta_window_get_geometry()` | `edu_wm_window_get_geometry()` | Returns `EduWmRect` |
| `meta_window_move()` | `edu_wm_window_move()` | Same behavior |
| `meta_window_resize()` | `edu_wm_window_resize()` | Same behavior |
| `meta_window_maximize()` | `edu_wm_window_maximize()` | Same behavior |
| `meta_window_minimize()` | `edu_wm_window_minimize()` | Same behavior |
| `meta_window_unminimize()` | `edu_wm_window_unminimize()` | Same behavior |
| `meta_window_make_above()` | `edu_wm_stacking_raise()` | Different API |
| `meta_window_make_below()` | `edu_wm_stacking_lower()` | Different API |
| `meta_window_focus()` | `edu_wm_focus_manager_focus()` | Same behavior |
| `meta_window_activate()` | `edu_wm_focus_manager_activate()` | Focus + raise |
| `meta_window_get_frame()` | `edu_wm_window_get_decorations()` | Returns decoration info |
| `meta_window_set_opacity()` | `edu_wm_window_set_opacity()` | Same behavior |
| `meta_window_get_monitor()` | `edu_wm_window_get_monitor()` | Returns monitor ID |
| `meta_window_get_workspace()` | `edu_wm_window_get_workspace()` | Returns workspace ID |
| `meta_window_move_to_workspace()` | `edu_wm_window_move_to_workspace()` | Same behavior |
| `meta_window_tile()` | `edu_wm_window_tile()` | EduWM has more tile positions |

### 4.2 Workspace API

| Muffin | EduWM | Notes |
|--------|-------|-------|
| `meta_screen_get_n_workspaces()` | `edu_wm_workspace_get_count()` | Same behavior |
| `meta_screen_get_active_workspace()` | `edu_wm_workspace_get_active()` | Same behavior |
| `meta_screen_get_workspace_by_index()` | `edu_wm_workspace_get_by_id()` | ID-based instead of index |
| `meta_screen_append_workspace()` | `edu_wm_workspace_add()` | Same behavior |
| `meta_screen_remove_workspace()` | `edu_wm_workspace_remove()` | Same behavior |
| `meta_workspace_activate()` | `edu_wm_workspace_switch()` | Same behavior |
| `meta_workspace_get_windows()` | `edu_wm_workspace_list_windows()` | Returns list of window IDs |

### 4.3 Monitor API

| Muffin | EduWM | Notes |
|--------|-------|-------|
| `meta_screen_get_monitor_geometry()` | `edu_wm_monitor_get_geometry()` | Same behavior |
| `meta_screen_get_monitor_at_point()` | `edu_wm_monitor_get_at_point()` | Same behavior |
| `meta_screen_get_primary_monitor()` | `edu_wm_monitor_get_primary()` | Same behavior |
| `meta_screen_get_n_monitors()` | `edu_wm_monitor_get_count()` | Same behavior |
| `meta_screen_get_display()` | N/A | Not exposed in EduWM |
| `meta_screen_get_display_name()` | `edu_wm_monitor_get_name()` | Same behavior |

### 4.4 Display/Compositor API

| Muffin | EduWM | Notes |
|--------|-------|-------|
| `meta_get_backend()` | `edu_wm_compositor_get_backend()` | Returns "DRM" or "X11" |
| `meta_get_display()` | N/A | Replaced by compositor object |
| `meta_compositor_get_later()` | `edu_wm_compositor_schedule_repaint()` | Same behavior |
| `meta_compositor_grab_window()` | `edu_wm_input_grab_window()` | Input grab |
| `meta_compositor_ungrab_window()` | `edu_wm_input_ungrab_window()` | Release input grab |

---

## 5. Configuration Differences

### 5.1 Configuration File Locations

| Component | Muffin | EduWM |
|-----------|--------|-------|
| Main config | dconf `/org/cinnamon/muffin/` | `~/.config/eduwm/config.json` |
| Window rules | dconf `/org/cinnamon/muffin/window-rules/` | `~/.config/eduwm/window-rules.json` |
| Keybindings | dconf `/org/cinnamon/desktop/keybindings/` | `~/.config/eduwm/shortcuts.json` |
| Theme | dconf + files | `~/.config/eduwm/theme.css` |
| Monitors | dconf `/org/cinnamon/muffin/` | `~/.config/eduwm/monitors.json` |
| Workspace | dconf `/org/cinnamon/muffin/` | `~/.config/eduwm/workspaces.json` |
| Compositor | dconf `/org/cinnamon/muffin/compositing-*` | `~/.config/eduwm/compositor.json` |
| Logging | syslog | `~/.local/share/eduwm/logs/` |

### 5.2 Configuration File Format

EduWM uses JSON configuration files (not dconf):

```json
{
    "version": 1,
    "workspaces": {
        "mode": "static",
        "count": 4,
        "wrap_around": true
    },
    "input": {
        "focus_mode": "click",
        "focus_follows_mouse_delay": 250,
        "mouse_follows_focus": false,
        "edge_tiling": true,
        "edge_flip": true
    },
    "compositor": {
        "backend": "drm",
        "vsync": true,
        "triple_buffer": true,
        "gl_backend": "gles2",
        "shadow_radius": 12,
        "shadow_offset_x": 0,
        "shadow_offset_y": 4,
        "frame_rate_limit": 0
    },
    "decorations": {
        "default_mode": "server_side",
        "border_width": 2,
        "border_radius": 12,
        "titlebar_height": 32,
        "button_layout": ["close", "minimize", "maximize"]
    },
    "shortcuts": {
        "super": "toggle-overview",
        "super+left": "tile-left",
        "super+right": "tile-right",
        "super+up": "maximize",
        "super+down": "unmaximize",
        "super+tab": "window-switcher",
        "super+l": "lock-screen",
        "ctrl+alt+delete": "logout",
        "ctrl+alt+left": "workspace-left",
        "ctrl+alt+right": "workspace-right"
    }
}
```

### 5.3 Keybinding Differences

| Action | Muffin Default | EduWM Default |
|--------|---------------|---------------|
| Switch workspace left | `Ctrl+Alt+Left` | `Ctrl+Alt+Left` |
| Switch workspace right | `Ctrl+Alt+Right` | `Ctrl+Alt+Right` |
| Move window left workspace | `Ctrl+Shift+Alt+Left` | `Super+Shift+Left` |
| Move window right workspace | `Ctrl+Shift+Alt+Right` | `Super+Shift+Right` |
| Tile left | `Super+Left` | `Super+Left` |
| Tile right | `Super+Right` | `Super+Right` |
| Maximize | `Super+Up` | `Super+Up` |
| Minimize | `Super+Down` | `Super+Down` |
| Close window | `Alt+F4` | `Alt+F4` |
| Window switcher | `Alt+Tab` | `Super+Tab` |
| Overview | N/A | `Super` (hold) |
| Lock screen | `Ctrl+Alt+L` | `Super+L` |

---

## 6. Plugin/Extension Migration Path

### 6.1 Cinnamon Applets

Cinnamon applets that interact with the window manager need to use EduWM's D-Bus interface:

**Before (Muffin):**
```javascript
// Access window list
let screen = global.screen;
let windows = screen.get_display().get_tab_list(
    Meta.TabList.NORMAL_ALL,
    screen.get_active_workspace()
);
```

**After (EduWM):**
```javascript
// Access window list via D-Bus
const EduWmDBus = imports.ui.eduwmDBus;

async function getWindowList() {
    const windows = await EduWmDBus.call(
        'org.eduwM.Core',
        '/org/eduwm/core',
        'org.eduwM.Core',
        'ListWindows',
        [{}]  // filter: all windows
    );
    return windows;
}
```

### 6.2 Cinnamon Desklets

Desklets that display window information:

**Before:**
```javascript
let window = global.display.focus_window;
let title = window.get_title();
let app = window.get_app();
```

**After:**
```javascript
const EduWmDBus = imports.ui.eduwmDBus;

async function getFocusedWindow() {
    const info = await EduWmDBus.call(
        'org.eduwM.Core',
        '/org/eduwm/core',
        'org.eduwM.Core',
        'GetFocusedWindow',
        []
    );
    return {
        title: info.title,
        app_id: info.app_id,
        geometry: info.geometry
    };
}
```

### 6.3 Cinnamon Extensions

Extensions that modify window behavior:

**Before:**
```javascript
// Override window placement
global.display.connect('window-created', (display, window) => {
    // Custom placement logic
    window.move_frame(true, x, y);
});
```

**After:**
```javascript
const EduWmDBus = imports.ui.eduwmDBus;

// Register a window rule via D-Bus
async function addWindowRule(match, actions) {
    await EduWmDBus.call(
        'org.eduwM.Core',
        '/org/eduwm/core',
        'org.eduwM.Core',
        'AddWindowRule',
        [match, actions]
    );
}
```

### 6.4 Supported Cinnamon Features

| Feature | EduWM Support | Notes |
|---------|--------------|-------|
| Window list applet | Full | Updated to use EduWM API |
| Workspace switcher | Full | Updated to use EduWM API |
| Notifications | Full | Updated to use EduWM API |
| Calendar applet | Full | No WM interaction needed |
| System monitor | Full | No WM interaction needed |
| Nemo desktop | Full | Updated to use EduWM API |
| Screensaver | Full | Uses ext-session-lock |
| Hot corners | Full | Native support |
| Window tiling | Full | Extended tile positions |
| Expo (overview) | Full | Built-in overview mode |

---

## 7. Known Differences and Behavior Changes

### 7.1 Behavioral Changes

| Behavior | Muffin | EduWM | Impact |
|----------|--------|-------|--------|
| Window focus on workspace switch | Focuses last focused window | Focuses last focused window | None |
| Edge tiling delay | Instant | 150ms delay (configurable) | Slight UX change |
| Window raise on focus | Always raises | Configurable (default: raise) | None |
| Minimize animation | Fade out | Slide down (configurable) | Visual change |
| Workspace switch animation | Slide | Crossfade (configurable) | Visual change |
| Popup positioning | Fixed offset | Smart repositioning (stays on-screen) | Improvement |
| Monitor DPI | Per-monitor RandR | Per-surface fractional scale | Better HiDPI |
| Window snapping | 4 positions (quarters) | 8 positions (halves + quarters) | More options |
| Alt+Tab behavior | Linear list | Grid + linear (configurable) | More options |

### 7.2 Visual Changes

- **Window Shadows**: EduWM uses a different shadow algorithm. Shadows may appear slightly different.
- **Title Bar Height**: Default is 32px (Muffin default is 26px). Configurable.
- **Border Radius**: Default is 12px (Muffin default is 0px for some themes). Configurable.
- **Button Layout**: Default order is Close-Minimize-Maximize (Muffin is Minimize-Maximize-Close). Configurable.
- **Animations**: EduWM's animations are smoother but may feel different. All can be customized.

### 7.3 Removed Features

| Feature | Reason | Alternative |
|---------|--------|-------------|
| Compiz-style effects | Not compatible with Wayland | Use CSS animations or EduWM's animation engine |
| Custom window borders via GTK | Not compatible with Wayland | Use EduWM's CSS theme system |
| Muffin plugin API | Proprietary to Muffin | Use EduWM's D-Bus interface |
| GSettings-based config | Replaced by JSON | Use EduWM config files |

### 7.4 New Features in EduWM

- **Overview Mode**: GNOME Shell-style window overview.
- **Dynamic Workspaces**: Auto-create/destroy workspaces.
- **Better Tiling**: 8-position tiling with keyboard shortcuts.
- **CSS Themes**: Hot-reloadable CSS-like themes.
- **Session Lock**: Secure session locking via ext-session-lock.
- **Per-Pixel Damage**: Improved performance.
- **Adaptive Animations**: Animations that adapt to system performance.
- **Inspector Tool**: Runtime window inspection.
- **Profiler**: Performance profiling overlay.

---

## 8. Rollback Procedure

If EduWM doesn't work or you need to revert to Muffin:

### 8.1 Quick Rollback

```bash
# Switch session at login screen
# Select "Cinnamon" (not "Cinnamon (EduWM)")

# Or from TTY:
cinnamon-session --session=cinnamon
```

### 8.2 Full Rollback

```bash
# 1. Stop EduWM
systemctl --user stop eduwm

# 2. Restore Muffin configuration
dconf load /org/cinnamon/muffin/ < ~/.muffin-settings-backup.dconf
dconf load /org/cinnamon/desktop/keybindings/ < ~/.keybindings-backup.dconf

# 3. Restore Cinnamon configuration
dconf load /org/cinnamon/ < ~/.cinnamon-all-backup.dconf

# 4. Remove EduWM configuration (optional)
rm -rf ~/.config/eduwm/

# 5. Restart Cinnamon
cinnamon --replace &
```

### 8.3 Automatic Fallback

If EduWM crashes 3 times within 60 seconds, the watchdog automatically falls back to Muffin:

```bash
# Check fallback status
systemctl status eduwm-watchdog

# View fallback logs
journalctl --user -u eduwm-watchdog
```

### 8.4 Selective Rollback

If only specific EduWM features are problematic:

```bash
# Disable EduWM animations (use Muffin-style)
eduwm-config set animations.enabled false

# Disable EduWM tiling (use Muffin-style)
eduwm-config set tiling.enabled false

# Disable EduWM overview (use Alt+Tab only)
eduwm-config set overview.enabled false

# Disable EduWM compositor features
eduwm-config set compositor.damage-tracking false
eduwm-config set compositor.triple-buffer false
```

---

## 9. Common Migration Issues and Troubleshooting

### 9.1 Issue: Windows Don't Appear

**Symptoms**: Windows open but are not visible.

**Solution**:
```bash
# Check window rules
eduwm-debug --list-windows

# Check if windows are being mapped
EUDUWM_LOG_LEVEL=debug eduwm 2>&1 | grep "window-created"

# Check XWayland compatibility
eduwm-debug --list-x11-windows
```

### 9.2 Issue: Panel Not Displaying

**Symptoms**: Cinnamon panel is missing.

**Solution**:
```bash
# Check if layer-shell is working
eduwm-debug --list-layer-surfaces

# Restart Cinnamon
cinnamon --replace &

# Check panel logs
journalctl --user -u cinnamon-panel
```

### 9.3 Issue: Animations Laggy

**Symptoms**: Window animations are not smooth.

**Solution**:
```bash
# Check frame rate
eduwm-profiler --fps-overlay

# Reduce animation complexity
eduwm-config set animations.reduced-motion true

# Disable blur effects
eduwm-config set compositor.blur false

# Check GPU driver
glxinfo | grep "OpenGL renderer"
```

### 9.4 Issue: Multi-Monitor Problems

**Symptoms**: Monitors not detected or wrong configuration.

**Solution**:
```bash
# Check DRM output
eduwm-debug --list-outputs

# Reset monitor configuration
eduwm-config reset monitors

# Force specific configuration
eduwm-config set monitors.config /path/to/monitors.json
```

### 9.5 Issue: Keyboard Shortcuts Not Working

**Symptoms**: EduWM shortcuts don't respond.

**Solution**:
```bash
# Check registered shortcuts
eduwm-debug --list-shortcuts

# Reset shortcuts to defaults
eduwm-config reset shortcuts

# Check for conflicts with Cinnamon shortcuts
dconf read /org/cinnamon/desktop/keybindings/custom-keybindings
```

### 9.6 Issue: X11 Applications Broken

**Symptoms**: X11 apps crash or display incorrectly.

**Solution**:
```bash
# Check XWayland status
eduwm-debug --xwayland-status

# Set XWayland compatibility mode
eduwm-config set xwayland.compatibility-mode full

# Restart XWayland
eduwm-debug --restart-xwayland
```

### 9.7 Issue: Theme Not Applied

**Symptoms**: Window decorations look wrong.

**Solution**:
```bash
# Check theme status
eduwm-debug --theme-status

# Reload theme
eduwm-config reload-theme

# Reset to default theme
eduwm-config reset theme
```

---

## 10. Testing Checklist After Migration

### 10.1 Core Window Management

- [ ] Open windows from different applications
- [ ] Close windows with close button, Alt+F4, and window menu
- [ ] Minimize and restore windows
- [ ] Maximize and unmaximize windows
- [ ] Fullscreen windows (F11 or menu)
- [ ] Move windows by dragging title bar
- [ ] Resize windows by dragging edges/corners
- [ ] Tile windows left, right, and quarters
- [ ] Pin windows to all workspaces (sticky)
- [ ] Set window always on top/bottom

### 10.2 Workspace Management

- [ ] Switch workspaces with keyboard shortcuts
- [ ] Switch workspaces with mouse scroll
- [ ] Move windows between workspaces
- [ ] Create new workspace (if dynamic)
- [ ] Remove empty workspace (if dynamic)
- [ ] Overview mode displays all windows
- [ ] Overview mode search works
- [ ] Overview mode window selection works

### 10.3 Multi-Monitor

- [ ] All monitors detected and configured
- [ ] Windows can be moved between monitors
- [ ] Panel displays on correct monitor(s)
- [ ] Notifications appear on primary monitor
- [ ] Screensaver covers all monitors
- [ ] Hotplug monitor connect/disconnect
- [ ] Resolution change works
- [ ] Rotation works (if applicable)
- [ ] Fractional scaling works

### 10.4 Input

- [ ] Mouse/trackpad works on all monitors
- [ ] Keyboard input works
- [ ] Touch input works (if applicable)
- [ ] Stylus/tablet input works (if applicable)
- [ ] Custom keyboard shortcuts work
- [ ] Mouse shortcuts work
- [ ] Touchpad gestures work
- [ ] Edge gestures work

### 10.5 Visual

- [ ] Window decorations render correctly
- [ ] Window shadows appear
- [ ] Transparency works
- [ ] Blur effects work
- [ ] Animations are smooth
- [ ] Theme colors are correct
- [ ] Fonts render correctly
- [ ] Cursor changes correctly
- [ ] Screenshots work

### 10.6 System Integration

- [ ] Notification system works
- [ ] Clipboard works (copy/paste)
- [ ] Drag and drop works
- [ ] File manager desktop icons work
- [ ] Screensaver/lock screen works
- [ ] Power management works (suspend, hibernate)
- [ ] Audio volume OSD works
- [ ] Brightness OSD works

---

## 11. Recommended Phased Rollout Timeline

### Phase 1: Preparation (Weeks 1-2)

- [ ] Install EduWM alongside Muffin
- [ ] Run compatibility checker
- [ ] Backup all configurations
- [ ] Migrate settings with migration tool
- [ ] Test EduWM in a separate user account
- [ ] Document any customizations

### Phase 2: Testing (Weeks 3-4)

- [ ] Boot into EduWM as primary session
- [ ] Run full test suite
- [ ] Test all core functionality
- [ ] Test all plugins and applets
- [ ] Test multi-monitor (if applicable)
- [ ] Test accessibility features
- [ ] Test performance under load
- [ ] Document any issues

### Phase 3: Gradual Rollout (Weeks 5-6)

- [ ] Deploy to tech-savvy users first
- [ ] Collect feedback
- [ ] Address reported issues
- [ ] Update documentation
- [ ] Deploy to wider user base
- [ ] Monitor system logs for errors

### Phase 4: Full Deployment (Week 7+)

- [ ] All users migrated to EduWM
- [ ] Muffin removed (or kept as fallback)
- [ ] Monitor performance metrics
- [ ] Collect user feedback
- [ ] Plan next iteration improvements

### Rollback Triggers

Immediately rollback to Muffin if:
- EduWM crashes more than 3 times in a session
- Core functionality is broken (can't open/close windows)
- Performance is significantly worse than Muffin
- Critical accessibility features don't work
- Data loss occurs (unsaved work)

---

*This document is part of the EduWM v2 documentation suite.*
*Last updated: 2026-07-10*
