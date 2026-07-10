# EduWM Window Manager — Architecture Document

## 1. Overview and Design Philosophy

EduWM is a modern, Wayland-native window manager designed as the successor to Muffin for the Cinnamon desktop environment. It was born out of the need to decouple window management from the Cinnamon shell, enabling independent development, better performance, and a cleaner separation of concerns.

### Core Design Principles

1. **Modularity over Monolith**: Each subsystem is an independent module with well-defined interfaces. Modules can be tested, replaced, or extended without affecting others.

2. **Wayland-First, X11-Compatible**: The primary protocol is Wayland. X11 applications are supported through XWayland, not as a first-class citizen.

3. **Damage Tracking by Default**: The compositor uses per-pixel damage tracking to minimize GPU work. Only regions that actually change are redrawn.

4. **Predictable Performance**: The animation engine targets 60fps (or higher refresh rates) with automatic frame pacing. Jank is treated as a bug.

5. **Security as Architecture**: Window isolation is enforced at the compositor level, not bolted on after the fact.

6. **Cinnamon Decoupled**: EduWM does not depend on Cinnamon internals. It exposes a clean IPC surface that Cinnamon (or any shell) can consume.

---

## 2. Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        EduWM Architecture                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                      Input System                             │  │
│  │  ┌──────────┐ ┌──────────┐ ┌─────────┐ ┌──────────────────┐  │  │
│  │  │ Keyboard │ │ Pointer  │ │  Touch  │ │    Gestures /    │  │  │
│  │  │  Events  │ │ Events   │ │ Events  │ │   Tablet Events  │  │  │
│  │  └────┬─────┘ └────┬─────┘ └────┬────┘ └───────┬──────────┘  │  │
│  │       │             │            │              │              │  │
│  │       └─────────────┴────────────┴──────────────┘              │  │
│  │                          │                                     │  │
│  │                   ┌──────┴──────┐                              │  │
│  │                   │ Shortcut    │                              │  │
│  │                   │ Engine      │                              │  │
│  │                   └──────┬──────┘                              │  │
│  └──────────────────────────┼────────────────────────────────────┘  │
│                             │                                       │
│                             ▼                                       │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                    Core Module                                │  │
│  │                                                               │  │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐  │  │
│  │  │ WindowCore   │  │ FocusManager │  │  StackingManager    │  │  │
│  │  │              │  │              │  │                     │  │  │
│  │  │ - create     │  │ - focus      │  │  - z-order          │  │  │
│  │  │ - destroy    │  │ - unfocus    │  │  - raise / lower    │  │  │
│  │  │ - configure  │  │ - refocus    │  │  - fullscreen stack  │  │  │
│  │  │ - map/unmap  │  │ - follows    │  │  - layer ordering   │  │  │
│  │  │ - state      │  │   mouse      │  │                     │  │  │
│  │  └──────┬───────┘  └──────┬───────┘  └──────────┬──────────┘  │  │
│  │         │                 │                      │             │  │
│  │         └────────┬────────┴──────────────────────┘             │  │
│  │                  ▼                                             │  │
│  │         ┌────────────────┐                                    │  │
│  │         │ WindowRules    │                                    │  │
│  │         │ Engine         │                                    │  │
│  │         │                │                                    │  │
│  │         │ - match rules  │                                    │  │
│  │         │ - apply props  │                                    │  │
│  │         │ - eval hooks   │                                    │  │
│  │         └────────────────┘                                    │  │
│  └───────────────────────────┬───────────────────────────────────┘  │
│                              │                                      │
│          ┌───────────────────┼───────────────────┐                  │
│          ▼                   ▼                   ▼                  │
│  ┌──────────────┐  ┌────────────────┐  ┌───────────────────┐       │
│  │  Workspace   │  │   Monitor      │  │   Wayland         │       │
│  │  Engine v2   │  │   Manager      │  │   Compatibility   │       │
│  │              │  │                │  │                   │       │
│  │ - static     │  │ - multi-mon    │  │ - xdg-shell       │       │
│  │ - dynamic    │  │ - scaling      │  │ - layer-shell      │       │
│  │ - overview   │  │ - transforms   │  │ - xdg-decoration   │       │
│  │ - layout     │  │ - hotplug      │  │ - fractional-scale │       │
│  │ - switching  │  │ - DPI          │  │ - pointer-constr.  │       │
│  └──────┬───────┘  └───────┬────────┘  │ - viewporter       │       │
│         │                  │            │ - session-lock      │       │
│         │                  │            │ - screencast        │       │
│         │                  │            │ - cursor-shape      │       │
│         │                  │            └──────────┬─────────┘       │
│         │                  │                       │                 │
│         └──────────────────┼───────────────────────┘                 │
│                            │                                         │
│                            ▼                                         │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │                     Compositor                                │   │
│  │                                                               │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐  │   │
│  │  │ Renderer    │  │ Damage       │  │  Frame Pacing       │  │   │
│  │  │             │  │ Tracker      │  │                     │  │   │
│  │  │ - OpenGL ES │  │ - per-pix    │  │  - vsync            │  │   │
│  │  │ - Vulkan    │  │ - region     │  │  - adaptive sync    │  │   │
│  │  │ - DRM/KMS   │  │   merge      │  │  - triple buffer    │  │   │
│  │  │ - scanout   │  │ - clip       │  │  - latency target   │  │   │
│  │  └─────────────┘  └──────────────┘  └─────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────────┘   │
│                              │                                       │
│                              ▼                                       │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │                   Animation Engine v2                          │   │
│  │                                                               │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐  │   │
│  │  │ Easing      │  │ Parallel     │  │  Reduced Motion     │  │   │
│  │  │ Functions   │  │ Animations   │  │  & Adaptive Perf    │  │   │
│  │  │             │  │              │  │                     │  │   │
│  │  │ - linear    │  │ - timeline   │  │  - respect pref.    │  │   │
│  │  │ - cubic     │  │ - stagger    │  │  - disable anims    │  │   │
│  │  │ - spring    │  │ - sequence   │  │  - frame budget     │  │   │
│  │  │ - bounce    │  │ - group      │  │  - auto degrade     │  │   │
│  │  └─────────────┘  └──────────────┘  └─────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────────┘   │
│                              │                                       │
│                              ▼                                       │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │                    Theme Layer                                 │   │
│  │                                                               │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐  │   │
│  │  │ Borders     │  │ Shadows      │  │  CSS Generator      │  │   │
│  │  │ & Decors    │  │              │  │                     │  │   │
│  │  │             │  │ - drop       │  │  - per-surface      │  │   │
│  │  │ - radius    │  │ - blur       │  │  - pseudo-classes   │  │   │
│  │  │ - color     │  │ - animated   │  │  - media queries    │  │   │
│  │  │ - style     │  │ - per-layer  │  │  - hot-reload       │  │   │
│  │  └─────────────┘  └──────────────┘  └─────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │                  X11 / XWayland Compatibility                 │   │
│  │                                                               │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐  │   │
│  │  │ XWayland     │  │ EWMH Atoms   │  │  Legacy Window     │  │   │
│  │  │ Manager      │  │ Registry     │  │  Handling          │  │   │
│  │  └──────────────┘  └──────────────┘  └────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │               Crash Recovery & Security                       │   │
│  │                                                               │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐  │   │
│  │  │ Session      │  │ Fallback to  │  │  Security          │  │   │
│  │  │ Save/Restore │  │ Muffin       │  │  Manager           │  │   │
│  │  └──────────────┘  └──────────────┘  └────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │              Debug / Profiling Tools                           │   │
│  │                                                               │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐  │   │
│  │  │ Inspector    │  │ Profiler     │  │  Logging           │  │   │
│  │  │ (per-window) │  │ (perf snap.) │  │  (ring buffer)     │  │   │
│  │  └──────────────┘  └──────────────┘  └────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Core Module

### 3.1 WindowCore

WindowCore is the central authority for window state. Every window (Wayland surface, XWayland window, or layer-shell surface) is represented as a `WindowHandle` in this module.

**Responsibilities:**
- Track per-window state: position, size, geometry, title, app_id, pid, roles
- Handle map/unmap lifecycle for both Wayland and X11 surfaces
- Emit signals on state changes (moved, resized, minimized, maximized, fullscreened)
- Maintain a flat list of all managed windows for query operations

**Internal Data Structures:**

```c
typedef struct {
    uint32_t           id;           // unique window ID
    EduWmSurfaceType   type;         // WAYLAND_Toplevel, WAYLAND_Popup,
                                    // WAYLAND_Layer, X11
    EduWmWindowState   state;        // Normal, Minimized, Maximized,
                                    // Fullscreen, Tiled
    EduWmRect          geometry;     // x, y, width, height
    EduWmRect          prev_geometry;// restore geometry before maximize
    char              *title;
    char              *app_id;       // Wayland app_id or X11 WM_CLASS
    pid_t              pid;
    uint32_t           workspace_id;
    uint32_t           monitor_id;
    uint32_t           layer;        // for layer-shell
    bool               has_focus;
    bool               is_floating;
    bool               is_urgent;
    uint32_t           edges_tiled;  // bitmask of tiled edges
    EduWmWindowRules  *matched_rules;
} EduWmWindow;
```

**Lifecycle States:**

```
Created → Configured → Mapped → Focused/Unfocused →
  → Minimized/Maximized/Fullscreened →
  → Unmapped → Destroyed
```

### 3.2 FocusManager

FocusManager determines which window receives input events. It implements a focus stack (LIFO) with support for focus-follows-mouse and click-to-focus modes.

**Focus Modes:**
- `CLICK_TO_FOCUS`: Focus changes only on explicit click. Default mode.
- `FOCUS_FOLLOWS_MOUSE`: Focus changes as the pointer moves over windows.
- `FOCUS_FOLLOWS_MOUSE_DELAYED`: Like follows-mouse, but with a configurable delay.
- `MOUSE_FOLLOWS_FOCUS`: Pointer warps to focused window center on focus change.

**Focus Stealing Prevention:**

FocusManager tracks the timestamp of the last user interaction. A window can only receive focus if its focus request timestamp is newer than the last user interaction timestamp. This prevents background windows from stealing focus during typing.

**Stacking Integration:**

When a window gains focus, it is automatically raised to the top of the stacking order (unless `raise_on_focus` is disabled). The focus stack is maintained separately from the stacking order, allowing the user to cycle focus without changing z-order (Alt+Tab behavior).

### 3.3 StackingManager

StackingManager controls the z-order of all windows and layers.

**Layer Model (bottom to top):**

```
Layer 0: Desktop / Wallpaper
Layer 1: Below Normal
Layer 2: Normal (most windows)
Layer 3: Above Normal
Layer 4: Fullscreen
Layer 5: Notification
Layer 6: Urgent / Alert
Layer 7: Overlay (input-method, session-lock)
```

**Operations:**
- `raise(window)`: Move window to top of its layer.
- `lower(window)`: Move window to bottom of its layer.
- `raise_to_layer(window, layer)`: Move window to a specific layer.
- `lower_to_layer(window, layer)`: Lower window to bottom of a specific layer.
- `swap(a, b)`: Swap z-positions of two windows.

**Fullscreen Stacking:**

When a window enters fullscreen mode, it is temporarily moved to Layer 4 and given the full monitor geometry. Its original layer is saved for restoration on unfullscreen.

### 3.4 WindowRulesEngine

WindowRulesEngine evaluates a set of user-defined rules against window properties to automatically apply behaviors.

**Rule Format (JSON-like DSL):**

```json
{
    "match": {
        "app_id": "org.gnome.Terminal",
        "title": ".*",
        "pid": 0,
        "class": "XTerm"
    },
    "actions": {
        "workspace": 2,
        "monitor": 1,
        "floating": false,
        "tile": "right",
        "layer": 2,
        "skip_taskbar": false,
        "skip_pager": false,
        "sticky": false,
        "opacity": 1.0,
        "focus": true
    }
}
```

**Evaluation Order:**
1. Rules are evaluated in declaration order (first match wins by default).
2. Multiple rules can be combined with `"priority"` for override semantics.
3. Wildcards and regex are supported in match patterns.

---

## 4. Workspace Engine v2

### 4.1 Static vs Dynamic Workspaces

**Static Workspaces:**
- A fixed number of workspaces exist at all times (e.g., 4).
- Users navigate between them with keyboard shortcuts or the overview.
- Workspaces are always present, even if empty.

**Dynamic Workspaces:**
- Workspaces are created on demand when a window is placed on a new workspace.
- Empty workspaces at the end are automatically destroyed.
- The workspace count grows and shrinks with usage.

**Configuration:**

```c
typedef enum {
    EDU_WM_WORKSPACE_MODE_STATIC,
    EDU_WM_WORKSPACE_MODE_DYNAMIC
} EduWmWorkspaceMode;

typedef struct {
    EduWmWorkspaceMode mode;
    uint32_t           static_count;  // only for STATIC mode
    bool               wrap_around;   // navigate past last → first
    uint32_t           animation_duration_ms;
} EduWmWorkspaceConfig;
```

### 4.2 Overview Mode

The overview shows all windows across all workspaces in a zoomed-out view, similar to GNOME Shell's overview. It supports:

- **Window Grid**: All windows arranged in a grid per workspace.
- **Workspace Strip**: Horizontal strip of workspace thumbnails at the top.
- **Search**: Filter windows by title or app_id.
- **Hot Corners**: Trigger overview from screen corners.
- **Keyboard Trigger**: Super key toggles overview.

**Overview State Machine:**

```
Normal → [Super press / Hot Corner] → Overview
Overview → [Select Window] → Normal (focused on selected window)
Overview → [Type search] → Overview Search Mode
Overview → [Super press / Escape] → Normal
Overview → [Workspace click] → Switch to workspace (stays in overview)
Overview → [Drag window] → Drag and Drop mode
```

### 4.3 Layout Modes

Each workspace can have an independent layout mode:

- **Floating**: Traditional free-form window placement (default).
- **Tiling (BSP)**: Binary space partitioning. Windows fill space automatically.
- **Tiling (Column)**: Windows arranged in equal-width columns.
- **Tiling (Monocle)**: One window per workspace, full-screened, stacked.

Layout mode is per-workspace and persists across restarts.

---

## 5. Monitor Manager

### 5.1 Multi-Monitor

MonitorManager tracks all connected outputs and their properties:

```c
typedef struct {
    uint32_t        id;
    char           *name;          // e.g., "DP-1", "HDMI-A-1"
    EduWmRect       geometry;      // position and size in logical coords
    uint32_t       physical_width;  // mm
    uint32_t       physical_height; // mm
    float          scale;          // fractional scale factor
    float          refresh_rate;   // Hz
    EduWmTransform transform;     // rotation and reflection
    bool           is_primary;
    bool           is_enabled;
    EduWmMode      *current_mode;
    EduWmMode      **available_modes;
    uint32_t       num_modes;
} EduWmOutput;
```

### 5.2 Scaling

EduWM supports **fractional scaling** through the `wp_fractional_scale_v1` protocol. Each surface can have an independent scale factor based on the output it resides on.

**Scaling Pipeline:**

```
Surface content (native resolution)
    → Scale factor applied (e.g., 1.5x)
    → Output composited at output native resolution
    → DRM/KMS outputs to display
```

For XWayland surfaces, EduWM handles the scale conversion internally by rendering the X11 buffer at the correct fractional scale and applying damage tracking to minimize redraws.

### 5.3 Transforms

Supported transforms:
- `NORMAL`: No transformation.
- `ROTATED_90`: 90° clockwise rotation.
- `ROTATED_180`: 180° rotation.
- `ROTATED_270`: 270° clockwise rotation.
- `FLIPPED`: Horizontally flipped.
- `FLIPPED_ROTATED_90`: Flipped + 90° rotation.
- `FLIPPED_ROTATED_180`: Flipped + 180° rotation.
- `FLIPPED_ROTATED_270`: Flipped + 270° rotation.

### 5.4 Hotplug

Monitor hotplug is handled via the DRM subsystem:

1. **Connect Event**: MonitorManager receives a hotplug signal from the DRM backend.
2. **Mode Enumeration**: Available modes are queried from the kernel.
3. **Configuration Applied**: The user's monitor configuration (or default fallback) is applied.
4. **Workspace Redistribution**: Workspaces are redistributed across monitors if configured.
5. **Damage Full**: Full-screen damage is emitted to re-composite.

**Fallback Behavior:**
- If a monitor is disconnected and had windows, those windows are moved to the primary monitor.
- If no primary monitor is available, the first connected monitor becomes primary.

---

## 6. Input System

### 6.1 Keyboard

The keyboard subsystem handles:
- Key events (press, release, repeat)
- Modifier state tracking (Ctrl, Alt, Super, Shift)
- Keyboard layout switching (XKB)
- Key repeat rate and delay configuration

**Shortcut Engine:**

Shortcuts are registered as combinations of modifier masks and keycodes/symbols. The shortcut engine matches incoming key events against registered shortcuts and dispatches the associated action.

```c
typedef struct {
    EduWmKeyBinding  binding;      // key + modifiers
    EduWmAction      action;       // action to execute
    void            *user_data;    // context for the action
    bool             enabled;
} EduWmShortcut;
```

**Default Shortcuts:**

| Shortcut | Action |
|----------|--------|
| Super+Arrow | Tile window left/right |
| Super+Up | Maximize window |
| Super+Down | Unmaximize / minimize |
| Super+Left | Switch to left workspace |
| Super+Right | Switch to right workspace |
| Super+Tab | Window switcher (Alt+Tab) |
| Super | Toggle overview |
| Super+L | Lock screen |
| Ctrl+Alt+Del | Logout dialog |
| Print Screen | Screenshot |

### 6.2 Pointer

Pointer events include:
- Motion (absolute and relative)
- Button press/release
- Scroll (discrete and continuous)
- Pointer enter/leave on surfaces
- Cursor shape changes (via `wp_cursor_shape_v1`)

**Pointer Constraints:**

For applications like games and remote desktop, EduWM supports:
- `wp_pointer_constraints_v1`: Lock pointer to a position or confine to a surface.
- `wp_relative_pointer_v1`: Provide relative motion deltas without absolute positioning.

### 6.3 Touch

Touch events are mapped to pointer events for window management actions:
- Single tap → click
- Long press → right-click
- Two-finger pinch → zoom (for surfaces that support it)
- Three-finger drag → move window
- Edge swipe → workspace switch

### 6.4 Tablet

Tablet devices (Wacom, etc.) are supported with:
- Pressure sensitivity passthrough
- Tilt angle tracking
- Eraser detection
- Tablet-specific cursor mapping

### 6.5 Gestures

Gesture recognition is handled by the InputManager's gesture engine:

```c
typedef enum {
    EDU_WM_GESTURE_SWIPE,
    EDU_WM_GESTURE_PINCH,
    EDU_WM_GESTURE_HOLD,
    EDU_WM_GESTURE_TAP
} EduWmGestureType;

typedef struct {
    EduWmGestureType type;
    uint32_t         fingers;
    float            dx;          // accumulated delta
    float            dy;
    float            scale;       // for pinch
    float            rotation;    // for pinch
    bool             cancelled;
} EduWmGesture;
```

---

## 7. Compositor

### 7.1 Rendering Pipeline

```
┌──────────────────────────────────────────────────────┐
│                  Rendering Pipeline                    │
│                                                       │
│  1. Input Event Processing                            │
│     └─→ Pointer hit test → surface under cursor       │
│     └─→ Keyboard dispatch → focused surface           │
│                                                       │
│  2. State Update                                      │
│     └─→ Process pending surface commits               │
│     └─→ Apply window rules                            │
│     └─→ Update damage regions                         │
│                                                       │
│  3. Animation Update                                  │
│     └─→ Compute animation values for current frame    │
│     └─→ Mark damaged regions from animations          │
│                                                       │
│  4. Composite Pass                                    │
│     └─→ For each output:                              │
│         └─→ Determine visible surfaces (occlusion)    │
│         └─→ For each visible surface:                 │
│             └─→ Apply transform, scale, opacity       │
│             └─→ Submit to GPU as a textured quad      │
│         └─→ Render decorations (borders, shadows)     │
│         └─→ Render cursor                             │
│                                                       │
│  5. Output                                            │
│     └─→ DRM/KMS page flip (atomic)                    │
│     └─→ Wait for vblank / present fence               │
│     └─→ Frame pacing feedback to clients              │
│                                                       │
└──────────────────────────────────────────────────────┘
```

### 7.2 Damage Tracking

EduWM uses a two-level damage tracking system:

**Per-Surface Damage:**
- Each surface maintains its own damage region (a set of rectangles).
- When a surface commits a new buffer, its damage is updated.
- The compositor merges surface damage into a global damage region.

**Global Damage:**
- The global damage region represents the output areas that need recompositing.
- Only the damaged regions are re-rendered. Undamaged regions use the previous frame's content (via buffer age / buffer recycling).

**Damage Merging:**

```c
void edu_wm_damage_merge(EduWmDamage *global, EduWmDamage *surface) {
    // Transform surface damage by its position, scale, and transform
    EduWmRect transformed = transform_rect(
        surface->damage, surface->position, surface->scale, surface->transform
    );
    // Union with global damage
    rect_union(&global->region, &transformed);
}
```

### 7.3 Frame Pacing

Frame pacing ensures smooth animation by synchronizing frame submission with display refresh.

**Triple Buffering:**
- Three buffers are allocated per output.
- While one buffer is being displayed, one is being rendered, and one is available as a fallback.
- This prevents stalls when rendering takes slightly longer than one frame period.

**Present Feedback:**
- The compositor sends `wp_presentation_time_v1` feedback to clients, including:
  - Exact presentation time (CLOCK_MONOTONIC)
  - Refresh interval
  - State (ready, early, on_time, late, failed, pending)
  - Sequence number for frame correlation

### 7.4 Backend

The compositor supports two DRM backends:
- **Atomic DRM**: Uses atomic modesetting for batched output configuration.
- **Legacy DRM**: Fallback for older kernels.

EduWM does not use libdrm's legacy page_flip API by default. Atomic modesetting is preferred for its ability to test configurations before applying them.

---

## 8. Animation Engine v2

### 8.1 Easing Functions

The animation engine provides a library of easing functions:

| Function | Signature | Use Case |
|----------|-----------|----------|
| `linear` | `t` | No acceleration |
| `ease_in` | `t^2` | Slow start |
| `ease_out` | `1 - (1-t)^2` | Slow end |
| `ease_in_out` | `t < 0.5 ? 2t^2 : 1 - (-2t+2)^2/2` | Smooth both ends |
| `cubic_in` | `t^3` | Smooth acceleration |
| `cubic_out` | `1 - (1-t)^3` | Smooth deceleration |
| `cubic_in_out` | `t < 0.5 ? 4t^3 : 1 - (-2t+2)^3/2` | Smooth both ends |
| `spring` | Damped harmonic oscillator | Bouncy / organic feel |
| `bounce` | Piecewise polynomial | Bounce at end |
| `elastic` | Exponential decay * sin | Overshoot + settle |
| `back_in` | `c3 * t^3 - c1 * t^2` | Anticipation |
| `back_out` | `1 + c3 * (t-1)^3 + c1 * (t-1)^2` | Overshoot |

**Spring Parameters:**

```c
typedef struct {
    float mass;          // default: 1.0
    float stiffness;     // default: 170.0
    float damping;       // default: 26.0
    float velocity;      // initial velocity, default: 0.0
} EduWmSpringConfig;
```

### 8.2 Parallel Animations

Animations can be composed in parallel or sequence:

```c
// Parallel: both animations run simultaneously
EduWmAnimationGroup *group = edu_wm_animation_group_new(
    EDU_WM_ANIMATION_PARALLEL
);
edu_wm_animation_group_add(group, fade_animation);
edu_wm_animation_group_add(group, move_animation);
edu_wm_animation_group_start(group);

// Sequence: animations run one after another
EduWmAnimationGroup *seq = edu_wm_animation_group_new(
    EDU_WM_ANIMATION_SEQUENCE
);
edu_wm_animation_group_add(seq, step1);
edu_wm_animation_group_add(seq, step2);
edu_wm_animation_group_start(seq);
```

### 8.3 Stagger Animations

Stagger animations apply the same animation to multiple targets with incremental delays:

```c
EduWmStaggerConfig stagger = {
    .delay_ms = 50,    // delay between each element
    .from = 0,         // start index
    .to = windows->len // end index
};
edu_wm_animation_stagger(group, windows, stagger, property, target);
```

### 8.4 Reduced Motion

When the user has `prefers-reduced-motion` enabled in their accessibility settings:
- All animations are replaced with instant transitions (duration = 0).
- Alternatively, a "reduced" animation can be defined per-animation.
- The `EduWmAnimationEngine` queries the system preference at startup and on change.

### 8.5 Adaptive Performance

The animation engine monitors frame timing:
- If frames consistently miss the deadline (e.g., < 30fps on a 60Hz display), animations are automatically simplified:
  - Reduce particle count
  - Disable blur/shadow effects during animation
  - Reduce animation resolution (fewer intermediate steps)
- This is transparent to the user and does not affect the final state.

---

## 9. Theme Layer

### 9.1 Window Decorations

EduWM provides server-side decorations (SSD) for windows that don't use client-side decorations:

**Decoration Elements:**
- **Title Bar**: Title text, close/minimize/maximize buttons, app icon.
- **Borders**: Configurable width, color, and radius.
- **Shadows**: Drop shadows with configurable offset, blur radius, and color.
- **Rounded Corners**: Configurable corner radius.

**Decoration Rendering:**
Decorations are rendered as part of the compositor's damage tracking, not as separate surfaces. This means they don't add extra buffer allocations or commit latency.

### 9.2 CSS-Like Styling

The theme layer supports a CSS-like syntax for styling decorations:

```css
/* Example theme file */
window {
    border-width: 2px;
    border-color: #3c3c3c;
    border-radius: 12px;
    shadow-color: rgba(0, 0, 0, 0.4);
    shadow-offset: 0 4px;
    shadow-blur: 8px;
}

window.focused {
    border-color: #4a9eff;
    shadow-color: rgba(74, 158, 255, 0.3);
}

window.titlebar {
    background: linear-gradient(to bottom, #2d2d2d, #1a1a1a);
    padding: 4px 8px;
    font-size: 13px;
    font-weight: bold;
    color: #ffffff;
}

window.titlebar .close-button {
    icon: "window-close";
    color: #ff6b6b;
    hover-color: #ff4444;
}

window.titlebar .minimize-button {
    icon: "window-minimize";
    color: #ffd93d;
    hover-color: #ffcc00;
}

window.titlebar .maximize-button {
    icon: "window-maximize";
    color: #6bcb77;
    hover-color: #4caf50;
}
```

### 9.3 Hot Reload

Themes support hot-reload: when a theme file is modified on disk, the ThemeLayer detects the change via inotify, re-parses the theme, and re-renders all decorations on the next frame. No restart is required.

---

## 10. Wayland Compatibility

### 10.1 Supported Protocols

| Protocol | Version | Purpose |
|----------|---------|---------|
| `xdg-shell` | v6 | Toplevel and popup window management |
| `xdg-decoration` | v1 | Server-side decoration negotiation |
| `layer-shell` | v4 | Panel, wallpaper, notification surfaces |
| `wp-fractional-scale` | v1 | Per-surface fractional scaling |
| `wp-viewporter` | v1 | Surface cropping and scaling |
| `wp-pointer-constraints` | v1 | Pointer lock and confinement |
| `wp-relative-pointer` | v1 | Relative pointer motion |
| `wp-presentation-time` | v1 | Frame timing feedback |
| `wp-idle-inhibit` | v1 | Inhibit idle/suspend |
| `ext-session-lock` | v1 | Secure session locking |
| `wlr-screencopy` | v1 | Screen capture |
| `wp-cursor-shape` | v1 | Cursor shape management |
| `zwp-input-method` | v2 | Input method (IME) |
| `zwp-text-input` | v3 | Text input (IME) |
| `zwp-virtual-keyboard` | v1 | Virtual keyboard |
| `xdg-output` | v1 | Output information |
| `wl-output` | v4 | Output configuration |
| `wl-data-device` | v3 | Clipboard / DnD |
| `wl-seat` | v7 | Input device aggregation |
| `wl_shm` | v1 | Shared memory buffers |
| `linux-dmabuf` | v4 | DMA-BUF buffer sharing |

### 10.2 xdg-shell Lifecycle

```
Client                          EduWM
  │                               │
  │─── xdg_wm_base.get_xdg_surface ──→  (create XdgSurface)
  │                               │
  │─── xdg_toplevel.get_toplevel ────→  (create XdgToplevel)
  │                               │
  │                               │─── configure event ──→  (size, state)
  │                               │
  │─── ack_configure ─────────────→  (client acknowledges)
  │                               │
  │─── wl_surface.commit ─────────→  (first content commit)
  │                               │
  │                               │─── map window ──→  (add to window list)
  │                               │
  │    ... (window is mapped) ...  │
  │                               │
  │─── xdg_toplevel.close ────────→  (close request)
  │                               │
  │                               │─── unmap window ──→
  │                               │
  │─── wl_surface.destroy ────────→  (destroy surface)
  │                               │
```

### 10.3 layer-shell Layers

Layer-shell surfaces are placed on one of four layers:

| Layer | Z-Order | Use Case |
|-------|---------|----------|
| `background` | Bottom | Wallpapers, desktop icons |
| `bottom` | Lower middle | Desktop widgets |
| `top` | Upper middle | Panels, docks |
| `overlay` | Top | Notifications, OSD |

Each layer has its own stacking within its level, and exclusive zones prevent windows from overlapping panels.

---

## 11. X11/XWayland Compatibility

### 11.1 XWayland Integration

EduWM spawns an XWayland instance for legacy X11 applications:

```c
typedef struct {
    pid_t           xwayland_pid;
    int             xwayland_display_fd;
    struct wl_client *xwayland_client;
    struct wl_display *xwayland_display;
    bool            ready;
} EduWmXWayland;
```

XWayland windows are exposed to the rest of EduWM through the same `WindowCore` API as Wayland windows. The differences (buffer format, atom handling) are abstracted behind the `EduWmSurfaceType::X11` type.

### 11.2 EWMH Atoms

EduWM supports the following EWMH atoms for XWayland compatibility:

- `_NET_WM_NAME`, `_NET_WM_VISIBLE_NAME`
- `_NET_WM_WINDOW_TYPE` (all standard types)
- `_NET_WM_STATE` (maximized, fullscreen, sticky, above, below, skip_taskbar, skip_pager)
- `_NET_WM_DESKTOP` (workspace mapping)
- `_NET_WM_PID`
- `_NET_WM_USER_TIME` (focus stealing prevention)
- `_NET_WM_ICON` (taskbar icon)
- `_NET_WM_WINDOW_OPACITY`
- `_NET_WM_BYPASS_COMPOSITOR`
- `_MOTIF_WM_HINTS` (decorations)

---

## 12. Crash Recovery

### 12.1 Session Save/Restore

EduWM periodically saves window state to a JSON file:

```json
{
    "version": 1,
    "timestamp": "2026-07-10T14:30:00Z",
    "workspaces": [
        {
            "id": 1,
            "name": "Workspace 1",
            "windows": [
                {
                    "app_id": "org.gnome.Terminal",
                    "geometry": {"x": 100, "y": 100, "w": 800, "h": 600},
                    "state": "normal",
                    "workspace": 1,
                    "monitor": 0,
                    "focused": true
                }
            ]
        }
    ]
}
```

**Save Triggers:**
- Every 30 seconds (configurable)
- On window close
- On workspace change
- On monitor hotplug
- On SIGUSR1

### 12.2 Fallback to Muffin

If EduWM crashes:
1. A crash handler writes a minidump to `~/.local/share/eduwm/crashes/`.
2. The `eduwm-watchdog` process detects the crash and attempts to restart EduWM.
3. If EduWM fails to start 3 times in 60 seconds, the watchdog falls back to Muffin.
4. Session state is restored from the last save.

```bash
# Manual fallback
eduwm --fallback-to-muffin

# Check crash logs
ls ~/.local/share/eduwm/crashes/
```

---

## 13. Debug/Profiling Tools

### 13.1 Inspector

The inspector allows runtime inspection of any window:

```bash
# Activate inspector mode
eduwm-inspector --activate

# Query window properties
eduwm-inspector --get-window 42

# Highlight a window
eduwm-inspector --highlight 42 --color red --duration 2s
```

### 13.2 Profiling

```bash
# Start profiling session
eduwm-profiler --start --duration 10s --output /tmp/profile.json

# View profiling data
eduwm-profiler --view /tmp/profile.json

# Real-time FPS counter overlay
eduwm-profiler --fps-overlay
```

### 13.3 Debug Logging

```bash
# Enable verbose logging
EUDUWM_LOG_LEVEL=debug eduwm

# Filter by module
EUDUWM_LOG_FILTER=Compositor,AnimationEngine eduwm

# Ring buffer export
eduwm-debug --export-log /tmp/eduwm.log
```

---

## 14. Security Model

### 14.1 Window Isolation

- Each window runs in its own process (enforced by Wayland's client-server model).
- A malicious client cannot read another client's buffers.
- The compositor mediates all shared state.

### 14.2 Permissions

```c
typedef enum {
    EDU_WM_PERM_SCREEN_CAPTURE  = 1 << 0,
    EDU_WM_PERM_POINTER_LOCK    = 1 << 1,
    EDU_WM_PERM_KEYBOARD_GRAB   = 1 << 2,
    EDU_WM_PERM_GLOBAL_SHORTCUT = 1 << 3,
    EDU_WM_PERM_INPUT_METHOD    = 1 << 4,
    EDU_WM_PERM_ACCESSIBILITY   = 1 << 5,
} EduWmPermission;
```

Permissions are checked at protocol request time. Denials are logged for auditing.

### 14.3 Sandboxing

EduWM enforces the following restrictions:
- Clients cannot access DRM/KMS directly.
- Clients cannot read other clients' shared memory.
- Pointer constraints are only granted for surfaces with keyboard focus.
- Screencopy requires explicit user consent (portal integration).

---

## 15. Comparison with Muffin Architecture

| Aspect | Muffin | EduWM |
|--------|--------|-------|
| Protocol | X11 (primary), Wayland (experimental) | Wayland (primary), X11 (via XWayland) |
| Compositor | Clutter/Muffin compositor | Custom DRM/KMS compositor |
| Shell Integration | Tightly coupled to Cinnamon | Decoupled, IPC surface |
| Animation System | Clutter timeline + easing | Custom animation engine v2 |
| Theming | GTK/Metacity themes | CSS-like native themes |
| Multi-Monitor | RandR-based | DRM-native, hotplug |
| Crash Recovery | Restart session | Save/restore + Muffin fallback |
| Input | X11 input + libinput | libinput + Wayland protocols |
| Security | No isolation | Per-client isolation |
| Damage Tracking | Per-screen full redraw | Per-pixel damage tracking |
| Frame Pacing | Basic vsync | Triple buffering + present feedback |

---

## 16. Complete Data Flow Diagram

```
User Input Event → libinput → InputManager
    │
    ├─→ Keyboard Event → Shortcut Engine
    │   └─→ Action: tile, focus, switch workspace, etc.
    │       └─→ Core Module: WindowCore / FocusManager / WorkspaceEngine
    │           └─→ Compositor: state change → damage regions
    │               └─→ Animation Engine: trigger animation
    │                   └─→ Compositor: re-composite damaged regions
    │                       └─→ DRM/KMS: page flip → display
    │
    ├─→ Pointer Event → Hit Test
    │   └─→ Surface under cursor: enter/leave/button/motion
    │       └─→ Client receives events via Wayland protocol
    │           └─→ Client commits new buffer → Surface damage
    │               └─→ Compositor: merge damage → re-composite
    │                   └─→ DRM/KMS: page flip → display
    │
    ├─→ Touch Event → Gesture Recognition
    │   └─→ Matched gesture → Workspace switch / Window move / Zoom
    │       └─→ (same flow as keyboard action above)
    │
    └─→ Tablet Event → Passthrough to focused surface
        └─→ Client processes tablet data → commits buffer
            └─→ (same flow as pointer event above)

Shell IPC (D-Bus) → EduWM D-Bus Interface
    │
    ├─→ Window operations: minimize, maximize, close, tile
    ├─→ Workspace operations: create, destroy, switch
    ├─→ Monitor operations: configure, set primary
    ├─→ System operations: lock, logout, suspend
    └─→ Query operations: list windows, get properties

XWayland Event → XWayland Server → X11 Compat Layer
    │
    ├─→ EWMH atom processing
    ├─→ Window property mapping
    └─→ Convert to WindowCore internal format → (same flow as Wayland)
```

---

## 17. Key Architectural Decisions and Rationale

### Decision 1: Custom Compositor over wlroots
**Rationale:** While wlroots provides a solid foundation, EduWM needs deep integration with Cinnamon's D-Bus interface, custom window rules, and Muffin fallback logic. A custom compositor allows tighter control over the rendering pipeline and avoids the abstraction overhead of wlroots for EduWM-specific features.

### Decision 2: CSS-Like Theme Language
**Rationale:** CSS is a well-understood styling language. Using a CSS-like syntax for themes lowers the barrier for theme developers and allows hot-reload without restarting the compositor.

### Decision 3: Periodic Session Save
**Rationale:** A periodic save (rather than event-driven save) ensures that the session state is always recoverable, even in the event of a crash during a rapid sequence of window operations.

### Decision 4: Muffin Fallback
**Rationale:** EduWM is designed to be a drop-in replacement for Muffin, but stability is paramount. The fallback mechanism ensures users are never left without a working window manager, even if EduWM has a regression.

### Decision 5: Per-Pixel Damage Tracking
**Rationale:** Full-screen damage is wasteful on high-resolution displays. Per-pixel damage tracking (using the DRM damage protocol) ensures that only changed regions are re-rendered, significantly reducing GPU workload during animations and partial screen updates.

### Decision 6: Decoupled Shell Integration
**Rationale:** By exposing a clean D-Bus IPC surface instead of requiring Cinnamon headers at compile time, EduWM can be used with other shells (or standalone) and can evolve independently of Cinnamon's release cycle.

### Decision 7: Triple Buffering over Double
**Rationale:** Triple buffering adds minimal memory overhead but prevents frame stalls when rendering takes slightly longer than expected. On a 144Hz display with a 6.9ms frame budget, this is critical for smooth animations.

### Decision 8: Animation Engine v2 over Clutter
**Rationale:** Clutter's animation system is tightly coupled to the Clutter scene graph. EduWM's custom animation engine allows direct control over per-frame timing, parallel/sequence composition, and adaptive performance without Clutter's overhead.

---

*This document is part of the EduWM v2 documentation suite.*
*Last updated: 2026-07-10*
