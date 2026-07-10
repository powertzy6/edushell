# EduWM Window Manager — API Reference

## 1. WindowCore API

The WindowCore module manages the lifecycle and state of all windows.

### 1.1 Types

```c
typedef uint32_t EduWmWindowId;

typedef struct {
    EduWmWindowId   id;
    const char     *title;
    const char     *app_id;
    pid_t           pid;
    EduWmRect       geometry;
    EduWmWindowState state;
    uint32_t        workspace_id;
    uint32_t        monitor_id;
    bool            has_focus;
    bool            is_floating;
    bool            is_urgent;
    uint32_t        edges_tiled;
    EduWmSurfaceType surface_type;
} EduWmWindowInfo;
```

### 1.2 Functions

#### `edu_wm_window_core_init`

```c
EduWmWindowCore *edu_wm_window_core_init(EduWmCompositor *compositor);
```

**Description:** Initializes the WindowCore module.

**Parameters:**
- `compositor`: Pointer to the compositor instance.

**Returns:** Pointer to the initialized WindowCore, or NULL on failure.

**Example:**
```c
EduWmWindowCore *core = edu_wm_window_core_init(compositor);
if (!core) {
    fprintf(stderr, "Failed to init WindowCore\n");
    return -1;
}
```

---

#### `edu_wm_window_core_create_window`

```c
EduWmWindowId edu_wm_window_core_create_window(
    EduWmWindowCore *core,
    EduWmSurfaceType type,
    struct wl_surface *surface,
    const char *title,
    const char *app_id
);
```

**Description:** Creates a new window entry and associates it with a Wayland surface.

**Parameters:**
- `core`: WindowCore instance.
- `type`: Surface type (WAYLAND_Toplevel, WAYLAND_Popup, WAYLAND_Layer, X11).
- `surface`: The Wayland surface associated with this window.
- `title`: Window title (can be NULL).
- `app_id`: Application identifier (can be NULL).

**Returns:** The new window's unique ID.

**Example:**
```c
EduWmWindowId wid = edu_wm_window_core_create_window(
    core,
    EDU_WM_SURFACE_WAYLAND_TOPLEVEL,
    surface,
    "My Application",
    "com.example.myapp"
);
```

---

#### `edu_wm_window_core_destroy_window`

```c
void edu_wm_window_core_destroy_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id
);
```

**Description:** Destroys a window and removes it from the window list.

**Parameters:**
- `core`: WindowCore instance.
- `window_id`: The window to destroy.

**Example:**
```c
edu_wm_window_core_destroy_window(core, wid);
```

---

#### `edu_wm_window_core_list_windows`

```c
EduWmWindowInfo *edu_wm_window_core_list_windows(
    EduWmWindowCore *core,
    uint32_t *count
);
```

**Description:** Returns an array of all managed windows.

**Parameters:**
- `core`: WindowCore instance.
- `count`: Output parameter set to the number of windows.

**Returns:** Array of `EduWmWindowInfo`. Caller must free with `edu_wm_free_window_list`.

**Example:**
```c
uint32_t count;
EduWmWindowInfo *windows = edu_wm_window_core_list_windows(core, &count);

for (uint32_t i = 0; i < count; i++) {
    printf("Window %u: %s (%s)\n", windows[i].id, windows[i].title, windows[i].app_id);
}

edu_wm_free_window_list(windows, count);
```

---

#### `edu_wm_window_core_query_window`

```c
EduWmWindowInfo edu_wm_window_core_query_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id
);
```

**Description:** Queries information about a specific window.

**Parameters:**
- `core`: WindowCore instance.
- `window_id`: The window to query.

**Returns:** `EduWmWindowInfo` struct with current window state.

**Example:**
```c
EduWmWindowInfo info = edu_wm_window_core_query_window(core, wid);
printf("Window %u: %dx%d at (%d,%d)\n",
    info.id, info.geometry.width, info.geometry.height,
    info.geometry.x, info.geometry.y);
```

---

#### `edu_wm_window_core_get_focused_window`

```c
EduWmWindowId edu_wm_window_core_get_focused_window(EduWmWindowCore *core);
```

**Description:** Returns the ID of the currently focused window.

**Returns:** Window ID, or 0 if no window is focused.

---

#### `edu_wm_window_core_move_window`

```c
void edu_wm_window_core_move_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id,
    int32_t x,
    int32_t y
);
```

**Description:** Moves a window to the specified position.

**Parameters:**
- `core`: WindowCore instance.
- `window_id`: The window to move.
- `x`, `y`: New position in compositor coordinates.

---

#### `edu_wm_window_core_resize_window`

```c
void edu_wm_window_core_resize_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id,
    uint32_t width,
    uint32_t height
);
```

**Description:** Resizes a window to the specified dimensions.

**Parameters:**
- `core`: WindowCore instance.
- `window_id`: The window to resize.
- `width`, `height`: New dimensions in pixels.

---

#### `edu_wm_window_core_minimize_window`

```c
void edu_wm_window_core_minimize_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id
);
```

**Description:** Minimizes a window.

---

#### `edu_wm_window_core_unminimize_window`

```c
void edu_wm_window_core_unminimize_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id
);
```

**Description:** Restores a minimized window.

---

#### `edu_wm_window_core_maximize_window`

```c
void edu_wm_window_core_maximize_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id
);
```

**Description:** Maximizes a window to fill available space (excluding panels).

---

#### `edu_wm_window_core_unmaximize_window`

```c
void edu_wm_window_core_unmaximize_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id
);
```

**Description:** Restores a maximized window to its previous geometry.

---

#### `edu_wm_window_core_fullscreen_window`

```c
void edu_wm_window_core_fullscreen_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id,
    bool fullscreen
);
```

**Description:** Sets or unsets fullscreen mode for a window.

---

#### `edu_wm_window_core_tile_window`

```c
void edu_wm_window_core_tile_window(
    EduWmWindowCore *core,
    EduWmWindowId window_id,
    EduWmTilePosition position
);
```

**Description:** Tiles a window to the specified position.

**Tile Positions:**
- `EDU_WM_TILE_LEFT`: Left half of the monitor.
- `EDU_WM_TILE_RIGHT`: Right half of the monitor.
- `EDU_WM_TILE_TOP_LEFT`: Top-left quarter.
- `EDU_WM_TILE_TOP_RIGHT`: Top-right quarter.
- `EDU_WM_TILE_BOTTOM_LEFT`: Bottom-left quarter.
- `EDU_WM_TILE_BOTTOM_RIGHT`: Bottom-right quarter.
- `EDU_WM_TILE_CENTER`: Center (50% width, 50% height).
- `EDU_WM_TILE_MAXIMIZE`: Fill entire monitor.

---

#### `edu_wm_window_core_set_opacity`

```c
void edu_wm_window_core_set_opacity(
    EduWmWindowCore *core,
    EduWmWindowId window_id,
    float opacity
);
```

**Description:** Sets the opacity of a window.

**Parameters:**
- `opacity`: Value between 0.0 (fully transparent) and 1.0 (fully opaque).

---

#### `edu_wm_window_core_connect_signal`

```c
EduWmSignalId edu_wm_window_core_connect_signal(
    EduWmWindowCore *core,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Description:** Connects a callback to a WindowCore signal.

**Signal Names:**
- `"window-created"`: Emitted when a new window is created.
- `"window-destroyed"`: Emitted when a window is destroyed.
- `"window-focused"`: Emitted when a window gains focus.
- `"window-unfocused"`: Emitted when a window loses focus.
- `"window-moved"`: Emitted when a window is moved.
- `"window-resized"`: Emitted when a window is resized.
- `"window-minimized"`: Emitted when a window is minimized.
- `"window-maximized"`: Emitted when a window is maximized.
- `"window-fullscreened"`: Emitted when fullscreen state changes.

---

## 2. WorkspaceEngineV2 API

### 2.1 Types

```c
typedef uint32_t EduWmWorkspaceId;

typedef struct {
    EduWmWorkspaceId id;
    char            *name;
    uint32_t         window_count;
    bool             is_active;
    EduWmLayoutMode  layout;
} EduWmWorkspaceInfo;
```

### 2.2 Functions

#### `edu_wm_workspace_engine_init`

```c
EduWmWorkspaceEngine *edu_wm_workspace_engine_init(
    EduWmCompositor *compositor,
    EduWmWorkspaceConfig *config
);
```

**Description:** Initializes the workspace engine.

---

#### `edu_wm_workspace_add`

```c
EduWmWorkspaceId edu_wm_workspace_add(
    EduWmWorkspaceEngine *engine,
    const char *name
);
```

**Description:** Adds a new workspace.

**Parameters:**
- `name`: Workspace name (can be NULL for auto-generated name).

**Returns:** The new workspace's ID.

---

#### `edu_wm_workspace_remove`

```c
void edu_wm_workspace_remove(
    EduWmWorkspaceEngine *engine,
    EduWmWorkspaceId workspace_id
);
```

**Description:** Removes a workspace. Windows on this workspace are moved to the adjacent workspace.

---

#### `edu_wm_workspace_switch`

```c
void edu_wm_workspace_switch(
    EduWmWorkspaceEngine *engine,
    EduWmWorkspaceId workspace_id
);
```

**Description:** Switches to the specified workspace.

---

#### `edu_wm_workspace_switch_relative`

```c
void edu_wm_workspace_switch_relative(
    EduWmWorkspaceEngine *engine,
    int32_t direction
);
```

**Description:** Switches to the next (direction=1) or previous (direction=-1) workspace.

---

#### `edu_wm_workspace_get_active`

```c
EduWmWorkspaceId edu_wm_workspace_get_active(EduWmWorkspaceEngine *engine);
```

**Description:** Returns the currently active workspace ID.

---

#### `edu_wm_workspace_get_count`

```c
uint32_t edu_wm_workspace_get_count(EduWmWorkspaceEngine *engine);
```

**Description:** Returns the total number of workspaces.

---

#### `edu_wm_workspace_list`

```c
EduWmWorkspaceInfo *edu_wm_workspace_list(
    EduWmWorkspaceEngine *engine,
    uint32_t *count
);
```

**Description:** Returns an array of all workspaces.

---

#### `edu_wm_workspace_get_windows`

```c
EduWmWindowId *edu_wm_workspace_get_windows(
    EduWmWorkspaceEngine *engine,
    EduWmWorkspaceId workspace_id,
    uint32_t *count
);
```

**Description:** Returns an array of window IDs on the specified workspace.

---

#### `edu_wm_workspace_set_layout`

```c
void edu_wm_workspace_set_layout(
    EduWmWorkspaceEngine *engine,
    EduWmWorkspaceId workspace_id,
    EduWmLayoutMode layout
);
```

**Description:** Sets the layout mode for a workspace.

---

#### `edu_wm_workspace_overview_enter`

```c
void edu_wm_workspace_overview_enter(EduWmWorkspaceEngine *engine);
```

**Description:** Enters overview mode showing all windows across all workspaces.

---

#### `edu_wm_workspace_overview_exit`

```c
void edu_wm_workspace_overview_exit(EduWmWorkspaceEngine *engine);
```

**Description:** Exits overview mode.

---

#### `edu_wm_workspace_connect_signal`

```c
EduWmSignalId edu_wm_workspace_connect_signal(
    EduWmWorkspaceEngine *engine,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Signal Names:**
- `"workspace-created"`: New workspace added.
- `"workspace-destroyed"`: Workspace removed.
- `"workspace-switched"`: Active workspace changed.
- `"workspace-renamed"`: Workspace name changed.
- `"overview-entered"`: Overview mode activated.
- `"overview-exited"`: Overview mode deactivated.

---

## 3. MonitorManager API

### 3.1 Types

```c
typedef uint32_t EduWmMonitorId;

typedef struct {
    EduWmMonitorId  id;
    const char     *name;
    EduWmRect       geometry;
    float           scale;
    float           refresh_rate;
    EduWmTransform  transform;
    bool            is_primary;
    bool            is_enabled;
} EduWmMonitorInfo;
```

### 3.2 Functions

#### `edu_wm_monitor_manager_init`

```c
EduWmMonitorManager *edu_wm_monitor_manager_init(EduWmCompositor *compositor);
```

**Description:** Initializes the monitor manager and enumerates connected outputs.

---

#### `edu_wm_monitor_add`

```c
EduWmMonitorId edu_wm_monitor_add(
    EduWmMonitorManager *manager,
    const char *name,
    EduWmRect geometry,
    float scale,
    float refresh_rate
);
```

**Description:** Registers a new monitor (typically called by the DRM backend on hotplug).

---

#### `edu_wm_monitor_remove`

```c
void edu_wm_monitor_remove(
    EduWmMonitorManager *manager,
    EduWmMonitorId monitor_id
);
```

**Description:** Removes a monitor (typically called by the DRM backend on unplug).

---

#### `edu_wm_monitor_set_primary`

```c
void edu_wm_monitor_set_primary(
    EduWmMonitorManager *manager,
    EduWmMonitorId monitor_id
);
```

**Description:** Sets the specified monitor as primary.

---

#### `edu_wm_monitor_set_scale`

```c
void edu_wm_monitor_set_scale(
    EduWmMonitorManager *manager,
    EduWmMonitorId monitor_id,
    float scale
);
```

**Description:** Sets the scale factor for a monitor.

---

#### `edu_wm_monitor_set_transform`

```c
void edu_wm_monitor_set_transform(
    EduWmMonitorManager *manager,
    EduWmMonitorId monitor_id,
    EduWmTransform transform
);
```

**Description:** Sets the transform (rotation/reflection) for a monitor.

---

#### `edu_wm_monitor_get_geometry`

```c
EduWmRect edu_wm_monitor_get_geometry(
    EduWmMonitorManager *manager,
    EduWmMonitorId monitor_id
);
```

**Description:** Returns the geometry of a monitor.

---

#### `edu_wm_monitor_get_at_point`

```c
EduWmMonitorId edu_wm_monitor_get_at_point(
    EduWmMonitorManager *manager,
    int32_t x,
    int32_t y
);
```

**Description:** Returns the monitor ID that contains the given point.

---

#### `edu_wm_monitor_get_primary`

```c
EduWmMonitorId edu_wm_monitor_get_primary(EduWmMonitorManager *manager);
```

**Description:** Returns the primary monitor ID.

---

#### `edu_wm_monitor_get_count`

```c
uint32_t edu_wm_monitor_get_count(EduWmMonitorManager *manager);
```

**Description:** Returns the number of active monitors.

---

#### `edu_wm_monitor_list`

```c
EduWmMonitorInfo *edu_wm_monitor_list(
    EduWmMonitorManager *manager,
    uint32_t *count
);
```

**Description:** Returns an array of all active monitors.

---

#### `edu_wm_monitor_connect_signal`

```c
EduWmSignalId edu_wm_monitor_connect_signal(
    EduWmMonitorManager *manager,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Signal Names:**
- `"monitor-added"`: New monitor connected.
- `"monitor-removed"`: Monitor disconnected.
- `"monitor-geometry-changed"`: Monitor geometry or mode changed.
- `"monitor-scale-changed"`: Monitor scale changed.

---

## 4. InputManager API

### 4.1 Types

```c
typedef uint32_t EduWmDeviceId;

typedef struct {
    EduWmDeviceId   id;
    const char     *name;
    EduWmDeviceType type;   // KEYBOARD, POINTER, TOUCH, TABLET
    const char     *output_name;
    bool            is_active;
} EduWmDeviceInfo;

typedef struct {
    uint32_t        key;
    uint32_t        modifiers;
    EduWmAction     action;
    void           *action_data;
    bool            enabled;
} EduWmShortcutEntry;
```

### 4.2 Functions

#### `edu_wm_input_manager_init`

```c
EduWmInputManager *edu_wm_input_manager_init(
    EduWmCompositor *compositor,
    struct libinput *input
);
```

**Description:** Initializes the input manager with libinput integration.

---

#### `edu_wm_input_device_register`

```c
EduWmDeviceId edu_wm_input_device_register(
    EduWmInputManager *manager,
    const char *name,
    EduWmDeviceType type,
    const char *output_name
);
```

**Description:** Registers a new input device.

---

#### `edu_wm_input_device_unregister`

```c
void edu_wm_input_device_unregister(
    EduWmInputManager *manager,
    EduWmDeviceId device_id
);
```

**Description:** Unregisters an input device.

---

#### `edu_wm_input_list_devices`

```c
EduWmDeviceInfo *edu_wm_input_list_devices(
    EduWmInputManager *manager,
    uint32_t *count
);
```

**Description:** Returns an array of all registered input devices.

---

#### `edu_wm_input_shortcut_register`

```c
EduWmSignalId edu_wm_input_shortcut_register(
    EduWmInputManager *manager,
    uint32_t key,
    uint32_t modifiers,
    EduWmShortcutCallback callback,
    void *user_data
);
```

**Description:** Registers a keyboard shortcut.

**Parameters:**
- `key`: Keycode (e.g., `KEY_TAB`, `KEY_LEFT`).
- `modifiers`: Modifier mask (e.g., `EDU_WM_MOD_SUPER | EDU_WM_MOD_SHIFT`).
- `callback`: Function to call when the shortcut is triggered.
- `user_data`: User data passed to the callback.

**Returns:** Signal ID for unregistering.

**Modifier Constants:**
- `EDU_WM_MOD_SHIFT`: Shift key.
- `EDU_WM_MOD_CTRL`: Ctrl key.
- `EDU_WM_MOD_ALT`: Alt key.
- `EDU_WM_MOD_SUPER`: Super/Meta key.
- `EDU_WM_MOD_CAPS`: Caps Lock.

---

#### `edu_wm_input_shortcut_unregister`

```c
void edu_wm_input_shortcut_unregister(
    EduWmInputManager *manager,
    EduWmSignalId signal_id
);
```

**Description:** Unregisters a keyboard shortcut.

---

#### `edu_wm_input_pointer_grab`

```c
void edu_wm_input_pointer_grab(
    EduWmInputManager *manager,
    struct wl_surface *surface,
    const EduWmPointerGrabInterface *iface
);
```

**Description:** Grabs the pointer for interactive operations (move, resize).

---

#### `edu_wm_input_pointer_ungrab`

```c
void edu_wm_input_pointer_ungrab(EduWmInputManager *manager);
```

**Description:** Releases the pointer grab.

---

#### `edu_wm_input_keyboard_grab`

```c
void edu_wm_input_keyboard_grab(
    EduWmInputManager *manager,
    struct wl_surface *surface,
    const EduWmKeyboardGrabInterface *iface
);
```

**Description:** Grabs the keyboard for modal dialogs.

---

#### `edu_wm_input_keyboard_ungrab`

```c
void edu_wm_input_keyboard_ungrab(EduWmInputManager *manager);
```

**Description:** Releases the keyboard grab.

---

#### `edu_wm_input_connect_signal`

```c
EduWmSignalId edu_wm_input_connect_signal(
    EduWmInputManager *manager,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Signal Names:**
- `"device-added"`: New input device detected.
- `"device-removed"`: Input device removed.
- `"shortcut-triggered"`: A registered shortcut was triggered.
- `"gesture-detected"`: A touchpad gesture was recognized.

---

## 5. Compositor API

### 5.1 Types

```c
typedef struct {
    uint32_t        fps;
    uint32_t        frame_time_ms;
    uint32_t        damaged_pixels;
    uint32_t        total_pixels;
    bool            vsync_enabled;
    uint32_t        buffer_count;
} EduWmCompositorStats;

typedef struct {
    const char     *name;
    uint32_t        width;
    uint32_t        height;
    float           refresh_rate;
    bool            is_connected;
} EduWmOutputInfo;
```

### 5.2 Functions

#### `edu_wm_compositor_init`

```c
EduWmCompositor *edu_wm_compositor_init(int argc, char **argv);
```

**Description:** Initializes the compositor. Must be called before any other EduWM API.

**Returns:** Pointer to the compositor, or NULL on failure.

---

#### `edu_wm_compositor_start`

```c
int edu_wm_compositor_start(EduWmCompositor *compositor);
```

**Description:** Starts the compositor event loop. This function blocks until the compositor is shut down.

**Returns:** 0 on clean shutdown, non-zero on error.

---

#### `edu_wm_compositor_stop`

```c
void edu_wm_compositor_stop(EduWmCompositor *compositor);
```

**Description:** Signals the compositor to stop its event loop.

---

#### `edu_wm_compositor_get_backend`

```c
const char *edu_wm_compositor_get_backend(EduWmCompositor *compositor);
```

**Description:** Returns the backend type ("drm" or "x11").

---

#### `edu_wm_compositor_schedule_repaint`

```c
void edu_wm_compositor_schedule_repaint(EduWmCompositor *compositor);
```

**Description:** Schedules a repaint on the next vblank.

---

#### `edu_wm_compositor_get_stats`

```c
EduWmCompositorStats edu_wm_compositor_get_stats(EduWmCompositor *compositor);
```

**Description:** Returns current compositor statistics.

---

#### `edu_wm_compositor_get_outputs`

```c
EduWmOutputInfo *edu_wm_compositor_get_outputs(
    EduWmCompositor *compositor,
    uint32_t *count
);
```

**Description:** Returns information about all outputs.

---

#### `edu_wm_compositor_damage_full`

```c
void edu_wm_compositor_damage_full(EduWmCompositor *compositor);
```

**Description:** Marks the entire screen as damaged (forces full re-composite).

---

#### `edu_wm_compositor_damage_region`

```c
void edu_wm_compositor_damage_region(
    EduWmCompositor *compositor,
    EduWmRect region
);
```

**Description:** Marks a specific region as damaged.

---

#### `edu_wm_compositor_connect_signal`

```c
EduWmSignalId edu_wm_compositor_connect_signal(
    EduWmCompositor *compositor,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Signal Names:**
- `"started"`: Compositor started.
- `"stopping"`: Compositor is shutting down.
- `"output-added"`: New output detected.
- `"output-removed"`: Output disconnected.
- `"frame-pending"`: Next frame is pending.
- `"frame-presented"`: Frame was presented.

---

## 6. AnimationEngineV2 API

### 6.1 Types

```c
typedef uint32_t EduWmAnimationId;

typedef enum {
    EDU_WM_ANIMATION_LINEAR,
    EDU_WM_ANIMATION_EASE_IN,
    EDU_WM_ANIMATION_EASE_OUT,
    EDU_WM_ANIMATION_EASE_IN_OUT,
    EDU_WM_ANIMATION_CUBIC_IN,
    EDU_WM_ANIMATION_CUBIC_OUT,
    EDU_WM_ANIMATION_CUBIC_IN_OUT,
    EDU_WM_ANIMATION_SPRING,
    EDU_WM_ANIMATION_BOUNCE,
    EDU_WM_ANIMATION_ELASTIC,
    EDU_WM_ANIMATION_BACK_IN,
    EDU_WM_ANIMATION_BACK_OUT,
} EduWmEasingFunction;

typedef enum {
    EDU_WM_ANIMATION_RUNNING,
    EDU_WM_ANIMATION_PAUSED,
    EDU_WM_ANIMATION_COMPLETED,
    EDU_WM_ANIMATION_CANCELLED,
} EduWmAnimationState;

typedef enum {
    EDU_WM_ANIMATION_PARALLEL,
    EDU_WM_ANIMATION_SEQUENCE,
} EduWmAnimationGroupMode;

typedef struct {
    uint32_t    duration_ms;
    EduWmEasingFunction easing;
    float       from;
    float       to;
} EduWmAnimationConfig;

typedef struct {
    float       value;
    bool        completed;
} EduWmAnimationFrame;
```

### 6.2 Functions

#### `edu_wm_animation_engine_init`

```c
EduWmAnimationEngine *edu_wm_animation_engine_init(EduWmCompositor *compositor);
```

**Description:** Initializes the animation engine.

---

#### `edu_wm_animation_create`

```c
EduWmAnimationId edu_wm_animation_create(
    EduWmAnimationEngine *engine,
    EduWmAnimationConfig config,
    EduWmAnimationUpdateCallback callback,
    void *user_data
);
```

**Description:** Creates a new animation.

**Parameters:**
- `engine`: Animation engine instance.
- `config`: Animation configuration (duration, easing, from, to).
- `callback`: Function called on each frame with the current value.
- `user_data`: User data passed to the callback.

**Returns:** Animation ID.

**Example:**
```c
void on_opacity_update(float value, void *user_data) {
    EduWmWindowId *wid = user_data;
    edu_wm_window_core_set_opacity(core, *wid, value);
}

EduWmAnimationId anim = edu_wm_animation_create(engine,
    (EduWmAnimationConfig){
        .duration_ms = 300,
        .easing = EDU_WM_ANIMATION_EASE_OUT,
        .from = 0.0,
        .to = 1.0
    },
    on_opacity_update,
    &wid
);
edu_wm_animation_start(engine, anim);
```

---

#### `edu_wm_animation_start`

```c
void edu_wm_animation_start(
    EduWmAnimationEngine *engine,
    EduWmAnimationId animation_id
);
```

**Description:** Starts a paused or newly created animation.

---

#### `edu_wm_animation_pause`

```c
void edu_wm_animation_pause(
    EduWmAnimationEngine *engine,
    EduWmAnimationId animation_id
);
```

**Description:** Pauses a running animation.

---

#### `edu_wm_animation_resume`

```c
void edu_wm_animation_resume(
    EduWmAnimationEngine *engine,
    EduWmAnimationId animation_id
);
```

**Description:** Resumes a paused animation.

---

#### `edu_wm_animation_cancel`

```c
void edu_wm_animation_cancel(
    EduWmAnimationEngine *engine,
    EduWmAnimationId animation_id
);
```

**Description:** Cancels a running or paused animation. The completion callback is NOT called.

---

#### `edu_wm_animation_get_state`

```c
EduWmAnimationState edu_wm_animation_get_state(
    EduWmAnimationEngine *engine,
    EduWmAnimationId animation_id
);
```

**Description:** Returns the current state of an animation.

---

#### `edu_wm_animation_group_create`

```c
EduWmAnimationId edu_wm_animation_group_create(
    EduWmAnimationEngine *engine,
    EduWmAnimationGroupMode mode
);
```

**Description:** Creates a new animation group (parallel or sequence).

---

#### `edu_wm_animation_group_add`

```c
void edu_wm_animation_group_add(
    EduWmAnimationEngine *engine,
    EduWmAnimationId group_id,
    EduWmAnimationId animation_id
);
```

**Description:** Adds an animation to a group.

---

#### `edu_wm_animation_group_start`

```c
void edu_wm_animation_group_start(
    EduWmAnimationEngine *engine,
    EduWmAnimationId group_id
);
```

**Description:** Starts all animations in a group.

---

#### `edu_wm_animation_spring_create`

```c
EduWmAnimationId edu_wm_animation_spring_create(
    EduWmAnimationEngine *engine,
    EduWmSpringConfig spring_config,
    float target,
    EduWmAnimationUpdateCallback callback,
    void *user_data
);
```

**Description:** Creates a spring-based animation.

---

#### `edu_wm_animation_stagger`

```c
void edu_wm_animation_stagger(
    EduWmAnimationEngine *engine,
    EduWmAnimationId group_id,
    EduWmStaggerConfig stagger_config
);
```

**Description:** Applies stagger timing to animations in a group.

---

#### `edu_wm_animation_engine_is_reduced_motion`

```c
bool edu_wm_animation_engine_is_reduced_motion(EduWmAnimationEngine *engine);
```

**Description:** Returns true if reduced motion is enabled (accessibility preference).

---

#### `edu_wm_animation_engine_connect_signal`

```c
EduWmSignalId edu_wm_animation_engine_connect_signal(
    EduWmAnimationEngine *engine,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Signal Names:**
- `"animation-started"`: Animation began.
- `"animation-completed"`: Animation finished.
- `"animation-cancelled"`: Animation was cancelled.

---

## 7. ThemeLayer API

### 7.1 Types

```c
typedef uint32_t EduWmThemeId;

typedef struct {
    EduWmThemeId    id;
    const char     *name;
    const char     *path;
    bool            is_loaded;
} EduWmThemeInfo;

typedef struct {
    uint32_t        border_width;
    uint32_t        border_radius;
    uint32_t        titlebar_height;
    EduWmColor      border_color;
    EduWmColor      shadow_color;
    uint32_t        shadow_offset_x;
    uint32_t        shadow_offset_y;
    uint32_t        shadow_blur_radius;
    EduWmButtonLayout button_layout;
} EduWmThemeConfig;
```

### 7.2 Functions

#### `edu_wm_theme_manager_init`

```c
EduWmThemeManager *edu_wm_theme_manager_init(EduWmCompositor *compositor);
```

**Description:** Initializes the theme manager.

---

#### `edu_wm_theme_load`

```c
EduWmThemeId edu_wm_theme_load(
    EduWmThemeManager *manager,
    const char *path
);
```

**Description:** Loads a theme from a CSS file.

**Returns:** Theme ID, or 0 on failure.

---

#### `edu_wm_theme_apply`

```c
void edu_wm_theme_apply(
    EduWmThemeManager *manager,
    EduWmThemeId theme_id
);
```

**Description:** Applies a theme. All window decorations are re-rendered.

---

#### `edu_wm_theme_get_config`

```c
EduWmThemeConfig edu_wm_theme_get_config(
    EduWmThemeManager *manager,
    EduWmThemeId theme_id
);
```

**Description:** Returns the configuration of a loaded theme.

---

#### `edu_wm_theme_set_config`

```c
void edu_wm_theme_set_config(
    EduWmThemeManager *manager,
    EduWmThemeId theme_id,
    EduWmThemeConfig config
);
```

**Description:** Updates theme configuration (overrides CSS values).

---

#### `edu_wm_theme_list`

```c
EduWmThemeInfo *edu_wm_theme_list(
    EduWmThemeManager *manager,
    uint32_t *count
);
```

**Description:** Returns a list of available themes.

---

#### `edu_wm_theme_generate_css`

```c
char *edu_wm_theme_generate_css(
    EduWmThemeManager *manager,
    EduWmThemeConfig config
);
```

**Description:** Generates a CSS string from a theme configuration.

**Returns:** Caller must free the returned string.

---

#### `edu_wm_theme_hot_reload_enable`

```c
void edu_wm_theme_hot_reload_enable(
    EduWmThemeManager *manager,
    bool enable
);
```

**Description:** Enables or disables hot-reload via inotify.

---

#### `edu_wm_theme_connect_signal`

```c
EduWmSignalId edu_wm_theme_connect_signal(
    EduWmThemeManager *manager,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Signal Names:**
- `"theme-loaded"`: A theme was loaded.
- `"theme-changed"`: Active theme changed.
- `"theme-reloaded"`: Theme was hot-reloaded.

---

## 8. WaylandCompat API

### 8.1 Types

```c
typedef struct {
    struct wl_surface      *surface;
    EduWmSurfaceType        type;
    struct xdg_surface     *xdg_surface;
    struct xdg_toplevel    *xdg_toplevel;
    struct xdg_popup       *xdg_popup;
    struct zwlr_layer_surface_v1 *layer_surface;
    bool                    is_mapped;
    uint32_t                configure_serial;
} EduWmWaylandSurface;
```

### 8.2 Functions

#### `edu_wm_wayland_compat_init`

```c
EduWmWaylandCompat *edu_wm_wayland_compat_init(EduWmCompositor *compositor);
```

**Description:** Initializes Wayland protocol support.

---

#### `edu_wm_wayland_surface_create_xdg_toplevel`

```c
EduWmWaylandSurface *edu_wm_wayland_surface_create_xdg_toplevel(
    EduWmWaylandCompat *compat,
    struct wl_surface *surface
);
```

**Description:** Creates an xdg-toplevel surface.

---

#### `edu_wm_wayland_surface_create_xdg_popup`

```c
EduWmWaylandSurface *edu_wm_wayland_surface_create_xdg_popup(
    EduWmWaylandCompat *compat,
    struct wl_surface *surface,
    struct wl_surface *parent,
    struct xdg_positioner *positioner
);
```

**Description:** Creates an xdg-popup surface.

---

#### `edu_wm_wayland_surface_create_layer_surface`

```c
EduWmWaylandSurface *edu_wm_wayland_surface_create_layer_surface(
    EduWmWaylandCompat *compat,
    struct wl_surface *surface,
    struct wl_output *output,
    uint32_t layer,
    const char *name_space
);
```

**Description:** Creates a layer-shell surface.

---

#### `edu_wm_wayland_surface_destroy`

```c
void edu_wm_wayland_surface_destroy(EduWmWaylandSurface *wsurface);
```

**Description:** Destroys a wayland surface and its associated objects.

---

#### `edu_wm_wayland_surface_configure`

```c
void edu_wm_wayland_surface_configure(
    EduWmWaylandSurface *wsurface,
    uint32_t width,
    uint32_t height,
    uint32_t states
);
```

**Description:** Sends a configure event to the surface.

---

#### `edu_wm_wayland_surface_set_decoration_mode`

```c
void edu_wm_wayland_surface_set_decoration_mode(
    EduWmWaylandSurface *wsurface,
    EduWmDecorationMode mode
);
```

**Description:** Sets the decoration mode for an xdg-toplevel.

---

#### `edu_wm_wayland_surface_set_fractional_scale`

```c
void edu_wm_wayland_surface_set_fractional_scale(
    EduWmWaylandSurface *wsurface,
    float scale
);
```

**Description:** Sends the preferred fractional scale to a surface.

---

## 9. X11Compat API

### 9.1 Functions

#### `edu_wm_x11_compat_init`

```c
EduWmX11Compat *edu_wm_x11_compat_init(EduWmCompositor *compositor);
```

**Description:** Initializes XWayland support.

---

#### `edu_wm_x11_compat_start`

```c
int edu_wm_x11_compat_start(EduWmX11Compat *x11);
```

**Description:** Starts the XWayland server.

**Returns:** 0 on success, -1 on failure.

---

#### `edu_wm_x11_compat_stop`

```c
void edu_wm_x11_compat_stop(EduWmX11Compat *x11);
```

**Description:** Stops the XWayland server.

---

#### `edu_wm_x11_compat_is_ready`

```c
bool edu_wm_x11_compat_is_ready(EduWmX11Compat *x11);
```

**Description:** Returns true if XWayland is ready to accept connections.

---

#### `edu_wm_x11_compat_get_display`

```c
int edu_wm_x11_compat_get_display(EduWmX11Compat *x11);
```

**Description:** Returns the X11 display number.

---

#### `edu_wm_x11_compat_set_atom`

```c
void edu_wm_x11_compat_set_atom(
    EduWmX11Compat *x11,
    Window xwindow,
    Atom atom,
    const void *data,
    uint32_t length
);
```

**Description:** Sets an EWMH atom on an X11 window.

---

#### `edu_wm_x11_compat_get_atom`

```c
void *edu_wm_x11_compat_get_atom(
    EduWmX11Compat *x11,
    Window xwindow,
    Atom atom,
    uint32_t *length
);
```

**Description:** Gets an EWMH atom value from an X11 window.

---

#### `edu_wm_x11_compat_connect_signal`

```c
EduWmSignalId edu_wm_x11_compat_connect_signal(
    EduWmX11Compat *x11,
    const char *signal_name,
    EduWmSignalCallback callback,
    void *user_data
);
```

**Signal Names:**
- `"xwayland-ready"`: XWayland server is ready.
- `"x11-window-created"`: New X11 window detected.
- `"x11-window-destroyed"`: X11 window destroyed.
- `"x11-window-mapped"`: X11 window mapped.
- `"x11-window-unmapped"`: X11 window unmapped.

---

## 10. CrashRecovery API

### 10.1 Types

```c
typedef struct {
    uint32_t    crash_id;
    char       *timestamp;
    char       *backtrace;
    char       *signal;
    uint32_t    pid;
} EduWmCrashLog;

typedef struct {
    uint32_t    session_id;
    char       *timestamp;
    uint32_t    workspace_count;
    uint32_t    active_workspace;
    // Window data follows in the file
} EduWmSessionSave;
```

### 10.2 Functions

#### `edu_wm_crash_recovery_init`

```c
EduWmCrashRecovery *edu_wm_crash_recovery_init(EduWmCompositor *compositor);
```

**Description:** Initializes the crash recovery system.

---

#### `edu_wm_crash_recovery_save_session`

```c
int edu_wm_crash_recovery_save_session(EduWmCrashRecovery *cr);
```

**Description:** Saves the current session state to disk.

**Returns:** 0 on success, -1 on failure.

---

#### `edu_wm_crash_recovery_restore_session`

```c
int edu_wm_crash_recovery_restore_session(EduWmCrashRecovery *cr);
```

**Description:** Restores the last saved session state.

**Returns:** 0 on success, -1 on failure (no save found or data corrupt).

---

#### `edu_wm_crash_recovery_log_crash`

```c
int edu_wm_crash_recovery_log_crash(
    EduWmCrashRecovery *cr,
    int signal,
    void *context
);
```

**Description:** Logs a crash with backtrace to disk.

---

#### `edu_wm_crash_recovery_get_crash_logs`

```c
EduWmCrashLog *edu_wm_crash_recovery_get_crash_logs(
    EduWmCrashRecovery *cr,
    uint32_t *count
);
```

**Description:** Returns a list of recent crash logs.

---

#### `edu_wm_crash_recovery_clear_crash_logs`

```c
void edu_wm_crash_recovery_clear_crash_logs(EduWmCrashRecovery *cr);
```

**Description:** Clears all crash logs.

---

#### `edu_wm_crash_recovery_set_save_interval`

```c
void edu_wm_crash_recovery_set_save_interval(
    EduWmCrashRecovery *cr,
    uint32_t interval_seconds
);
```

**Description:** Sets the periodic session save interval.

---

#### `edu_wm_crash_recovery_enable_auto_save`

```c
void edu_wm_crash_recovery_enable_auto_save(
    EduWmCrashRecovery *cr,
    bool enable
);
```

**Description:** Enables or disables automatic session saving.

---

## 11. DebugTool / ProfilingTool API

### 11.1 Types

```c
typedef struct {
    uint32_t    window_id;
    const char *title;
    const char *app_id;
    EduWmRect   geometry;
    uint32_t    surface_id;
    uint32_t    buffer_age;
    uint32_t    damage_count;
    float       opacity;
    bool        has_focus;
    const char *layer;
} EduWmWindowInspectorData;

typedef struct {
    uint32_t    frame_number;
    uint64_t    timestamp_ns;
    uint32_t    render_time_us;
    uint32_t    composite_time_us;
    uint32_t    present_time_us;
    uint32_t    damaged_regions;
    uint32_t    total_damage_pixels;
    bool        vsync_missed;
} EduWmFrameProfile;
```

### 11.2 Functions

#### `edu_wm_debug_tool_init`

```c
EduWmDebugTool *edu_wm_debug_tool_init(EduWmCompositor *compositor);
```

**Description:** Initializes the debug tool.

---

#### `edu_wm_debug_inspector_activate`

```c
void edu_wm_debug_inspector_activate(EduWmDebugTool *debug);
```

**Description:** Activates the inspector mode. The next window click will display its properties.

---

#### `edu_wm_debug_inspector_deactivate`

```c
void edu_wm_debug_inspector_deactivate(EduWmDebugTool *debug);
```

**Description:** Deactivates the inspector mode.

---

#### `edu_wm_debug_inspector_get_window`

```c
EduWmWindowInspectorData edu_wm_debug_inspector_get_window(
    EduWmDebugTool *debug,
    EduWmWindowId window_id
);
```

**Description:** Returns detailed information about a window.

---

#### `edu_wm_debug_inspector_highlight`

```c
void edu_wm_debug_inspector_highlight(
    EduWmDebugTool *debug,
    EduWmWindowId window_id,
    EduWmColor color,
    uint32_t duration_ms
);
```

**Description:** Highlights a window with a colored border for a specified duration.

---

#### `edu_wm_profiling_tool_init`

```c
EduWmProfilingTool *edu_wm_profiling_tool_init(EduWmCompositor *compositor);
```

**Description:** Initializes the profiling tool.

---

#### `edu_wm_profiling_start`

```c
void edu_wm_profiling_start(
    EduWmProfilingTool *profiler,
    uint32_t duration_seconds
);
```

**Description:** Starts a profiling session for the specified duration.

---

#### `edu_wm_profiling_stop`

```c
void edu_wm_profiling_stop(EduWmProfilingTool *profiler);
```

**Description:** Stops the current profiling session.

---

#### `edu_wm_profiling_get_frames`

```c
EduWmFrameProfile *edu_wm_profiling_get_frames(
    EduWmProfilingTool *profiler,
    uint32_t *count
);
```

**Description:** Returns frame profile data from the last profiling session.

---

#### `edu_wm_profiling_export`

```c
int edu_wm_profiling_export(
    EduWmProfilingTool *profiler,
    const char *output_path,
    const char *format  // "json" or "chrome-trace"
);
```

**Description:** Exports profiling data to a file.

---

#### `edu_wm_profiling_fps_overlay`

```c
void edu_wm_profiling_fps_overlay(
    EduWmProfilingTool *profiler,
    bool enable
);
```

**Description:** Enables or disables the real-time FPS counter overlay.

---

## 12. SecurityManager API

### 12.1 Types

```c
typedef uint32_t EduWmPermissionId;

typedef struct {
    EduWmPermissionId   id;
    const char         *client_name;
    EduWmPermission      permissions;
    bool                is_allowed;
} EduWmClientPermission;
```

### 12.2 Functions

#### `edu_wm_security_init`

```c
EduWmSecurityManager *edu_wm_security_init(EduWmCompositor *compositor);
```

**Description:** Initializes the security manager.

---

#### `edu_wm_security_check_permission`

```c
bool edu_wm_security_check_permission(
    EduWmSecurityManager *security,
    struct wl_client *client,
    EduWmPermission permission
);
```

**Description:** Checks if a client has a specific permission.

---

#### `edu_wm_security_grant_permission`

```c
void edu_wm_security_grant_permission(
    EduWmSecurityManager *security,
    struct wl_client *client,
    EduWmPermission permission
);
```

**Description:** Grants a permission to a client.

---

#### `edu_wm_security_revoke_permission`

```c
void edu_wm_security_revoke_permission(
    EduWmSecurityManager *security,
    struct wl_client *client,
    EduWmPermission permission
);
```

**Description:** Revokes a permission from a client.

---

#### `edu_wm_security_list_permissions`

```c
EduWmClientPermission *edu_wm_security_list_permissions(
    EduWmSecurityManager *security,
    uint32_t *count
);
```

**Description:** Returns a list of all client permissions.

---

#### `edu_wm_security_audit_log`

```c
void edu_wm_security_audit_log(
    EduWmSecurityManager *security,
    const char *event,
    struct wl_client *client,
    const char *details
);
```

**Description:** Logs a security audit event.

---

## 13. EduWmTestFramework API

### 13.1 Types

```c
typedef struct EduWmTestSuite EduWmTestSuite;
typedef struct EduWmTestCase EduWmTestCase;

typedef enum {
    EDU_WM_TEST_PASS,
    EDU_WM_TEST_FAIL,
    EDU_WM_TEST_SKIP,
    EDU_WM_TEST_ERROR,
} EduWmTestResult;

typedef void (*EduWmTestFunc)(EduWmTestCase *test);
```

### 13.2 Functions

#### `edu_wm_test_init`

```c
EduWmTestFramework *edu_wm_test_init(int argc, char **argv);
```

**Description:** Initializes the test framework.

---

#### `edu_wm_test_suite_create`

```c
EduWmTestSuite *edu_wm_test_suite_create(
    EduWmTestFramework *fw,
    const char *name
);
```

**Description:** Creates a new test suite.

---

#### `edu_wm_test_case_add`

```c
void edu_wm_test_case_add(
    EduWmTestSuite *suite,
    const char *name,
    EduWmTestFunc func
);
```

**Description:** Adds a test case to a suite.

---

#### `edu_wm_test_assert_true`

```c
void edu_wm_test_assert_true(EduWmTestCase *test, bool condition, const char *msg);
```

**Description:** Asserts that a condition is true.

---

#### `edu_wm_test_assert_false`

```c
void edu_wm_test_assert_false(EduWmTestCase *test, bool condition, const char *msg);
```

**Description:** Asserts that a condition is false.

---

#### `edu_wm_test_assert_equal_int`

```c
void edu_wm_test_assert_equal_int(
    EduWmTestCase *test,
    int64_t actual,
    int64_t expected,
    const char *msg
);
```

**Description:** Asserts that two integers are equal.

---

#### `edu_wm_test_assert_equal_str`

```c
void edu_wm_test_assert_equal_str(
    EduWmTestCase *test,
    const char *actual,
    const char *expected,
    const char *msg
);
```

**Description:** Asserts that two strings are equal.

---

#### `edu_wm_test_run_suite`

```c
EduWmTestResult edu_wm_test_run_suite(
    EduWmTestFramework *fw,
    EduWmTestSuite *suite
);
```

**Description:** Runs all test cases in a suite.

---

#### `edu_wm_test_run_all`

```c
EduWmTestResult edu_wm_test_run_all(EduWmTestFramework *fw);
```

**Description:** Runs all test suites.

---

#### `edu_wm_test_stress`

```c
EduWmTestResult edu_wm_test_stress(
    EduWmTestFramework *fw,
    EduWmStressConfig config
);
```

**Description:** Runs a stress test with the specified configuration.

**Stress Config:**
```c
typedef struct {
    uint32_t window_count;
    uint32_t operations_per_second;
    uint32_t duration_seconds;
    uint32_t workspace_count;
    uint32_t monitor_count;
} EduWmStressConfig;
```

---

#### `edu_wm_test_performance`

```c
EduWmTestResult edu_wm_test_performance(
    EduWmTestFramework *fw,
    EduWmPerfConfig config
);
```

**Description:** Runs a performance test and outputs metrics.

**Perf Config:**
```c
typedef struct {
    uint32_t duration_seconds;
    uint32_t target_fps;
    bool     measure_compositor_cpu;
    bool     measure_gpu_usage;
    bool     measure_memory;
} EduWmPerfConfig;
```

---

*This document is part of the EduWM v2 documentation suite.*
*Last updated: 2026-07-10*
