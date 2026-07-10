# EduWM Window Manager — Wayland Protocol Guide

## 1. Overview of Supported Protocols

EduWM implements a comprehensive set of Wayland protocols to provide full desktop functionality. This document describes each supported protocol, how EduWM implements it, and the message flow between clients and the compositor.

### Protocol Version Matrix

| Protocol | Interface Prefix | Version | Status |
|----------|-----------------|---------|--------|
| core wl_shell | `wl_shell` | 3 | Deprecated (fallback only) |
| xdg_shell | `xdg_wm_base` | 6 | Stable |
| xdg_decoration | `zxdg_decoration_manager_v1` | 1 | Stable |
| layer_shell | `zwlr_layer_shell_v1` | 4 | Stable |
| wp_fractional_scale | `wp_fractional_scale_manager_v1` | 1 | Stable |
| wp_viewporter | `wp_viewporter` | 1 | Stable |
| wp_pointer_constraints | `zwp_pointer_constraints_v1` | 1 | Stable |
| wp_relative_pointer | `zwp_relative_pointer_manager_v1` | 1 | Stable |
| wp_presentation_time | `wp_presentation_time` | 1 | Stable |
| wp_idle_inhibit | `zwp_idle_inhibit_manager_v1` | 1 | Stable |
| ext_session_lock | `ext_session_lock_v1` | 1 | Stable |
| wlr_screencopy | `zwlr_screencopy_manager_v1` | 3 | Stable (wlroots compat) |
| wp_cursor_shape | `wp_cursor_shape_manager_v1` | 1 | Stable |
| zwp_input_method | `zwp_input_method_v2` | 2 | Stable |
| zwp_text_input | `zwp_text_input_manager_v3` | 3 | Stable |
| zwp_virtual_keyboard | `zwp_virtual_keyboard_manager_v1` | 1 | Stable |
| xdg_output | `zxdg_output_manager_v1` | 3 | Stable |
| wl_output | `wl_output` | 4 | Core |
| wl_seat | `wl_seat` | 7 | Core |
| wl_data_device | `wl_data_device_manager` | 3 | Core |
| wl_shm | `wl_shm` | 1 | Core |
| linux_dmabuf | `zwp_linux_dmabuf_v1` | 4 | Stable |

---

## 2. xdg-shell Protocol

The xdg-shell protocol is the primary protocol for window management in EduWM. It provides both toplevel (regular windows) and popup (menus, tooltips) window types.

### 2.1 Toplevel Lifecycle

```
┌──────────────────────────────────────────────────────────────┐
│                    xdg-shell Toplevel Lifecycle                │
│                                                               │
│  Client                        EduWM                          │
│    │                             │                            │
│    │ 1. xdg_wm_base             │                            │
│    │    .get_xdg_surface()       │                            │
│    │────────────────────────────→│  Create XdgSurface          │
│    │                             │  object                     │
│    │                             │                            │
│    │ 2. xdg_surface              │                            │
│    │    .get_toplevel()          │                            │
│    │────────────────────────────→│  Create XdgToplevel         │
│    │                             │  object                     │
│    │                             │                            │
│    │ 3. xdg_toplevel             │                            │
│    │    .set_title("My App")     │                            │
│    │────────────────────────────→│  Store title                │
│    │                             │                            │
│    │ 4. xdg_toplevel             │                            │
│    │    .set_app_id("my.app")    │                            │
│    │────────────────────────────→│  Store app_id               │
│    │                             │                            │
│    │ 5. xdg_toplevel             │                            │
│    │    .set_min_size(w, h)      │                            │
│    │────────────────────────────→│  Store min size             │
│    │                             │                            │
│    │ 6. xdg_toplevel             │                            │
│    │    .set_max_size(w, h)      │                            │
│    │────────────────────────────→│  Store max size             │
│    │                             │                            │
│    │ 7. wl_surface.commit()      │                            │
│    │────────────────────────────→│  Process initial commit     │
│    │                             │                            │
│    │                             │ 8. xdg_toplevel             │
│    │                             │    .configure(w, h, [])     │
│    │                             │───────────────────────────→│
│    │                             │                            │
│    │ 9. xdg_surface              │                            │
│    │    .ack_configure(serial)   │                            │
│    │────────────────────────────→│  Mark serial acknowledged   │
│    │                             │                            │
│    │ 10. [client renders content] │                           │
│    │                             │                            │
│    │ 11. wl_surface.commit()     │                            │
│    │────────────────────────────→│  Window is now mapped!      │
│    │                             │  → Add to WindowCore        │
│    │                             │  → Apply window rules       │
│    │                             │  → Emit "window-created"    │
│    │                             │                            │
│    │     ... window is running ...│                            │
│    │                             │                            │
│    │ 12. [user clicks close]     │                            │
│    │                             │                            │
│    │                             │ 13. xdg_toplevel            │
│    │                             │    .configure([],           │
│    │                             │      "toplevel_closed")     │
│    │                             │───────────────────────────→│
│    │                             │                            │
│    │ 14. wl_surface.destroy()    │                            │
│    │────────────────────────────→│  Window is unmapped         │
│    │                             │  → Remove from WindowCore   │
│    │                             │  → Emit "window-destroyed"  │
│    │                             │                            │
│    │ 15. xdg_surface.destroy()   │                            │
│    │────────────────────────────→│  Destroy XdgSurface         │
│    │                             │                            │
└──────────────────────────────────────────────────────────────┘
```

### 2.2 Toplevel State Changes

When the compositor wants to change the window state (e.g., maximize, fullscreen), it sends a configure event with the new state:

```
Client                        EduWM
  │                             │
  │    ... window is mapped ... │
  │                             │
  │                             │ 1. User triggers maximize
  │                             │    (double-click titlebar or
  │                             │     keyboard shortcut)
  │                             │
  │                             │ 2. xdg_toplevel.configure(
  │                             │      width=output_w,
  │                             │      height=output_h,
  │                             │      states=["maximized"]
  │                             │    )
  │                             │───────────────────────────→│
  │                             │                            │
  │ 3. xdg_surface              │                            │
  │    .ack_configure(serial)   │                            │
  │────────────────────────────→│                            │
  │                             │                            │
  │ 4. [client resizes buffer]  │                            │
  │                             │                            │
  │ 5. wl_surface.commit()      │                            │
  │────────────────────────────→│                            │
  │                             │ 6. Update window geometry   │
  │                             │    Mark damaged             │
  │                             │    Re-composite             │
  │                             │                            │
```

**Supported States:**
- `maximized`: Window fills available space (excluding panels).
- `fullscreen`: Window fills entire output, no decorations.
- `resizing`: Client is currently resizing the window.
- `activated`: Window has keyboard focus.

### 2.3 xdg_toplevel.move and resize

EduWM handles interactive move/resize through the xdg-shell protocol:

```c
// When user starts dragging the titlebar
void edu_wm_toplevel_begin_move(EduWmXdgToplevel *toplevel) {
    // Store initial pointer position and window geometry
    toplevel->move_state = EDU_WM_MOVE_ACTIVE;
    toplevel->move_rect = toplevel->window->geometry;

    // Grab the pointer
    edu_wm_input_manager_pointer_grab(
        input_manager, toplevel->surface, &move_grab_interface
    );
}

// Grab interface callbacks
static void move_grab_button(EduWmPointerGrab *grab,
                              uint32_t time, uint32_t button,
                              uint32_t state) {
    if (button == BTN_LEFT && state == WL_POINTER_BUTTON_STATE_RELEASED) {
        // End move
        grab->interface->cancel(grab);
    }
}

static void move_grab_motion(EduWmPointerGrab *grab,
                              uint32_t time, wl_fixed_t x, wl_fixed_t y) {
    EduWmXdgToplevel *toplevel = grab->data;
    int dx = x - grab->grab_x;
    int dy = y - grab->grab_y;

    // Update window position
    edu_wm_window_move(toplevel->window,
                       toplevel->move_rect.x + dx,
                       toplevel->move_rect.y + dy);
}
```

### 2.4 Popup Lifecycle

Popups (menus, tooltips, dropdowns) follow a simpler lifecycle:

```
Client                        EduWM
  │                             │
  │ 1. xdg_surface              │
  │    .get_popup(parent)       │
  │────────────────────────────→│  Create popup, link to parent
  │                             │
  │ 2. xdg_popup                │
  │    .grab(serial, seat)      │
  │────────────────────────────→│  Grab input (dismiss on
  │                             │  outside click)
  │                             │
  │ 3. wl_surface.commit()      │
  │────────────────────────────→│  Map popup
  │                             │
  │     ... popup displayed ... │
  │                             │
  │ 4. [user clicks outside]    │
  │                             │
  │                             │ 5. xdg_popup.popup_done()
  │                             │───────────────────────────→│
  │                             │
  │ 6. wl_surface.destroy()     │
  │────────────────────────────→│  Unmap popup
  │                             │
  │ 7. xdg_popup.destroy()      │
  │────────────────────────────→│  Destroy popup object
  │                             │
```

**Popup Positioning:**
EduWM positions popups relative to their parent window's geometry. If a popup would extend beyond the output bounds, it is repositioned to stay on-screen. This is done before the configure event, so the client receives the final position.

### 2.5 configure/ack_configure Protocol

The configure/ack_configure handshake ensures synchronization between the compositor and client:

1. **Compositor sends configure**: Contains new dimensions and state.
2. **Client sends ack_configure**: Acknowledges receipt of the configure.
3. **Client renders and commits**: The client renders its content at the new size and commits.
4. **Compositor processes commit**: The compositor updates its internal state.

The serial number in configure/ack_configure ensures that the compositor can correlate which configure a particular commit is responding to. EduWM tracks the most recent configure serial per surface and only applies the commit if the serial matches.

---

## 3. layer-shell Protocol

The layer-shell protocol is used for desktop shell components: panels, wallpapers, notifications, and overlays.

### 3.1 Layer Assignment

```
┌──────────────────────────────────────────────┐
│              Layer Stack Order                 │
│                                               │
│  ┌─────────────────────────────────────────┐  │
│  │  overlay (z=4)                          │  │
│  │  - Notifications, OSD, popups           │  │
│  ├─────────────────────────────────────────┤  │
│  │  top (z=3)                              │  │
│  │  - Panels, docks, taskbars             │  │
│  ├─────────────────────────────────────────┤  │
│  │  [Normal windows - xdg-shell]          │  │
│  ├─────────────────────────────────────────┤  │
│  │  bottom (z=1)                           │  │
│  │  - Desktop widgets, clocks             │  │
│  ├─────────────────────────────────────────┤  │
│  │  background (z=0)                       │  │
│  │  - Wallpapers, desktop background      │  │
│  └─────────────────────────────────────────┘  │
│                                               │
└──────────────────────────────────────────────┘
```

### 3.2 Anchors and Exclusive Zones

Anchors determine how a layer surface is positioned relative to an edge of the output:

```c
typedef enum {
    EDU_WM_ANCHOR_NONE        = 0,
    EDU_WM_ANCHOR_TOP         = 1 << 0,
    EDU_WM_ANCHOR_BOTTOM      = 1 << 1,
    EDU_WM_ANCHOR_LEFT        = 1 << 2,
    EDU_WM_ANCHOR_RIGHT       = 1 << 3,
    EDU_WM_ANCHOR_TOP_LEFT    = ANCHOR_TOP | ANCHOR_LEFT,
    EDU_WM_ANCHOR_TOP_RIGHT   = ANCHOR_TOP | ANCHOR_RIGHT,
    EDU_WM_ANCHOR_BOTTOM_LEFT = ANCHOR_BOTTOM | ANCHOR_LEFT,
    EDU_WM_ANCHOR_BOTTOM_RIGHT= ANCHOR_BOTTOM | ANCHOR_RIGHT,
} EduWmAnchor;
```

**Exclusive Zones:**

When a layer surface specifies an exclusive zone (e.g., a panel), other layer surfaces and xdg-shell windows are adjusted to avoid overlapping it:

```
┌──────────────────────────────────────────┐
│  ┌────────────────────────────────────┐  │  ← Panel (exclusive_zone=32)
│  │  Panel (32px)                      │  │
│  ├────────────────────────────────────┤  │
│  │                                    │  │
│  │  Normal window area                │  │  ← Adjusted for panel
│  │  (excludes panel region)           │  │
│  │                                    │  │
│  └────────────────────────────────────┘  │
└──────────────────────────────────────────┘
```

**Exclusive Zone Calculation:**

```c
void edu_wm_layer_calculate_exclusive(EduWmLayerSurface *surface) {
    int32_t exclusive = surface->exclusive_zone;
    EduWmAnchor anchor = surface->anchor;

    if (anchor & EDU_WM_ANCHOR_TOP) {
        surface->geometry.y = 0;
        surface->geometry.height = exclusive;
    }
    if (anchor & EDU_WM_ANCHOR_BOTTOM) {
        surface->geometry.y = output->height - exclusive;
        surface->geometry.height = exclusive;
    }
    if (anchor & EDU_WM_ANCHOR_LEFT) {
        surface->geometry.x = 0;
        surface->geometry.width = exclusive;
    }
    if (anchor & EDU_WM_ANCHOR_RIGHT) {
        surface->geometry.x = output->width - exclusive;
        surface->geometry.width = exclusive;
    }

    // Update exclusive zones for other surfaces
    edu_wm_layer_update_exclusive_zones(surface->output);
}
```

### 3.3 Keyboard Interactivity

Layer surfaces can request keyboard interactivity:

- `none`: No keyboard input. Surface is purely visual.
- `exclusive`: Surface receives all keyboard input. Other surfaces don't.
- `on_demand`: Surface receives keyboard input only when it has focus.

### 3.4 Layer Surface Message Flow

```
Panel Client                 EduWM
  │                             │
  │ 1. zwlr_layer_shell_v1     │
  │    .get_layer_surface(     │
  │      output,               │
  │      "top",                │
  │      "panel"               │
  │    )                       │
  │────────────────────────────→│  Create layer surface
  │                             │
  │ 2. zwlr_layer_surface_v1   │
  │    .set_anchor("top")      │
  │────────────────────────────→│  Store anchor
  │                             │
  │ 3. zwlr_layer_surface_v1   │
  │    .set_size(0, 32)        │
  │────────────────────────────→│  Store size
  │                             │
  │ 4. zwlr_layer_surface_v1   │
  │    .set_exclusive_zone(32) │
  │────────────────────────────→│  Store exclusive zone
  │                             │
  │ 5. zwlr_layer_surface_v1   │
  │    .set_keyboard_interactivity("exclusive")
  │────────────────────────────→│  Store keyboard mode
  │                             │
  │ 6. wl_surface.commit()     │
  │────────────────────────────→│  Apply configuration
  │                             │  → Recalculate layout
  │                             │  → Emit configure
  │                             │
  │ 7. zwlr_layer_surface_v1   │
  │    .configure(serial, w, h)│
  │                             │───────────────────────────→│
  │                             │                            │
  │ 8. xdg_surface             │
  │    .ack_configure(serial)  │
  │────────────────────────────→│                            │
  │                             │                            │
  │ 9. [panel renders content] │
  │                             │
  │ 10. wl_surface.commit()    │
  │────────────────────────────→│  Map panel                │
  │                             │  → Add to stacking        │
  │                             │  → Apply exclusive zone   │
  │                             │
```

---

## 4. xdg-decoration Protocol

The xdg-decoration protocol allows negotiation between server-side decorations (SSD) and client-side decorations (CSD).

### 4.1 Negotiation Flow

```
Client                        EduWM
  │                             │
  │ 1. xdg_wm_base             │
  │    .get_xdg_surface()       │
  │────────────────────────────→│
  │                             │
  │ 2. zxdg_decoration_manager │
  │    .get_toplevel_decoration │
  │────────────────────────────→│  Create decoration object
  │                             │
  │ 3. zxdg_toplevel_decoration│
  │    .set_mode("server_side") │
  │────────────────────────────→│  Request SSD
  │                             │
  │                             │ 4. Evaluate decoration rules:
  │                             │    - Client preference
  │                             │    - Window rules
  │                             │    - App ID matching
  │                             │    - User preference
  │                             │
  │                             │ 5. zxdg_toplevel_decoration
  │                             │    .configure("server_side")
  │                             │───────────────────────────→│
  │                             │
  │ 6. [client renders without  │
  │     decorations if SSD]     │
  │                             │
```

### 4.2 Decoration Decision Logic

```c
EduWmDecorationMode edu_wm_decoration_decide(EduWmXdgToplevel *toplevel) {
    // 1. Check window rules (explicit match)
    EduWmWindowRule *rule = edu_wm_rules_match(toplevel->window);
    if (rule && rule->has_decoration_mode) {
        return rule->decoration_mode;
    }

    // 2. Check client preference
    if (toplevel->decoration_request == EDU_WM_DECORATION_SERVER_SIDE) {
        return EDU_WM_DECORATION_SERVER_SIDE;
    }
    if (toplevel->decoration_request == EDU_WM_DECORATION_CLIENT_SIDE) {
        return EDU_WM_DECORATION_CLIENT_SIDE;
    }

    // 3. Check global default
    return global_config.default_decoration_mode;
}
```

### 4.3 SSD Rendering

When server-side decorations are active:
- EduWM renders a title bar and borders around the client's surface.
- The client's surface is placed inside the decoration frame.
- Resize handles are provided by the compositor (invisible, at the edges).
- The client receives `xdg_toplevel.configure` with the inner dimensions (excluding decoration size).

---

## 5. fractional-scale Protocol

The wp-fractional-scale protocol allows per-surface fractional scaling, enabling crisp rendering on HiDPI displays.

### 5.1 Scaling Flow

```
Client                        EduWM
  │                             │
  │ 1. wp_fractional_scale_     │
  │    manager_v1               │
  │    .get_fractional_scale(   │
  │      surface                │
  │    )                       │
  │────────────────────────────→│  Create fractional scale obj
  │                             │
  │ 2. wp_fractional_scale_v1   │
  │    .preferred_scale(1.5)    │
  │                             │───────────────────────────→│
  │                             │
  │ 3. [client renders at 1.5x] │
  │                             │
  │ 4. wl_surface.commit()      │
  │────────────────────────────→│  Surface committed at 1.5x
  │                             │
```

### 5.2 Scale Calculation

The compositor determines the preferred scale based on:
1. The output's physical size and current mode.
2. The user's scaling preference (100%, 125%, 150%, 175%, 200%).
3. The surface's position (if spanning multiple outputs, use the output with the largest intersection).

```c
float edu_wm_calculate_preferred_scale(EduWmOutput *output, EduWmSurface *surface) {
    float base_scale = output->scale;  // user-configured scale factor

    // If surface spans multiple outputs, use the one with largest overlap
    EduWmOutput *primary = edu_wm_surface_get_primary_output(surface);
    if (primary) {
        base_scale = primary->scale;
    }

    return base_scale;
}
```

### 5.3 Fractional vs Integer Scaling

| Scale | Use Case | Rendering |
|-------|----------|-----------|
| 1.0 | 1080p at 1x | Native pixels |
| 1.25 | 1080p at slight zoom | 5:4 pixel ratio |
| 1.5 | 4K at 1.5x | 3:2 pixel ratio (common) |
| 2.0 | 4K at 2x | 2:1 pixel ratio |

Fractional scales produce non-integer pixel ratios, which can cause sub-pixel artifacts. EduWM mitigates this by:
- Using the nearest-integer scale for buffer allocation.
- Applying the fractional scale during composition (not at buffer creation).
- Using high-quality Lanczos downscaling when the fractional scale requires it.

---

## 6. viewporter Protocol

The wp-viewporter protocol allows clients to specify a source rectangle within their buffer, enabling cropping and scaling without additional buffer allocations.

### 6.1 Usage

```
Client                        EduWM
  │                             │
  │ 1. wp_viewporter            │
  │    .get_viewport(surface)   │
  │────────────────────────────→│  Create viewport object
  │                             │
  │ 2. wp_viewport.set_source(  │
  │      x, y, w, h            │  Source rect in buffer coords
  │    )                       │
  │────────────────────────────→│  Store source rect
  │                             │
  │ 3. wp_viewport.set_destination│
  │      (w, h)                │  Destination size in surface coords
  │────────────────────────────→│  Store destination size
  │                             │
  │ 4. wl_surface.commit()      │
  │────────────────────────────→│  Apply viewport
  │                             │
```

### 6.2 Compositing with Viewport

When a surface has a viewport:
1. The compositor reads only the source rectangle from the buffer.
2. The source rectangle is scaled to the destination size.
3. The resulting image is composited at the surface's position.

This is useful for:
- **Video players**: Show a specific crop of the decoded video.
- **Image viewers**: Zoom into a region without resizing the buffer.
- **Thumbnail generators**: Extract a portion of a larger surface.

---

## 7. pointer-constraints and relative-pointer

### 7.1 Pointer Constraints

The pointer-constraints protocol allows surfaces to lock or confine the pointer:

**Lock Mode:**
- Pointer position is fixed.
- Relative motion events are still delivered.
- Used for: first-person cameras, orbit controls.

**Confine Mode:**
- Pointer is confined to the surface boundary.
- If the pointer hits the edge, it stops.
- Used for: dragging within a canvas, game menus.

```
Client                        EduWM
  │                             │
  │ 1. zwp_pointer_constraints  │
  │    .lock_pointer(           │
  │      surface,               │
  │      region,                │  NULL = lock at current position
  │      lifetime               │
  │    )                       │
  │────────────────────────────→│  Create pointer lock
  │                             │
  │ 2. zwp_pointer_constraints  │
  │    .lock.confirmed          │
  │                             │───────────────────────────→│
  │                             │
  │ 3. [pointer is now locked]  │
  │                             │
  │ 4. zwp_relative_pointer_v1  │
  │    .relative_motion(        │
  │      utime_hi, utime_lo,    │
  │      dx, dy,                │  Relative motion deltas
  │      dx_unaccel, dy_unaccel │  Unaccelerated deltas
  │    )                       │
  │                             │───────────────────────────→│
  │                             │
```

**Lifetime Options:**
- `on_demand`: Lock persists until the client explicitly releases it.
- `once`: Lock is released after the next button press.

### 7.2 Relative Pointer

The relative-pointer protocol delivers motion events as deltas rather than absolute coordinates:

```c
void edu_wm_relative_pointer_motion(EduWmPointer *pointer,
                                     EduWmSurface *surface,
                                     uint32_t time_hi, uint32_t time_lo,
                                     double dx, double dy) {
    // Only deliver if the surface has a relative pointer listener
    if (!surface->relative_pointer) return;

    double dx_unaccel = dx;  // raw hardware delta
    double dy_unaccel = dy;

    // Apply acceleration for the accelerated deltas
    double acc_dx, acc_dy;
    edu_wm_pointer_accelerate(pointer, dx, dy, &acc_dx, &acc_dy);

    zwp_relative_pointer_v1_send_motion(
        surface->relative_pointer,
        time_hi, time_lo,
        wl_fixed_from_double(acc_dx),
        wl_fixed_from_double(acc_dy),
        wl_fixed_from_double(dx_unaccel),
        wl_fixed_from_double(dy_unaccel)
    );
}
```

---

## 8. presentation-time Protocol

The presentation-time protocol provides frame timing feedback to clients, enabling smooth animations and synchronization.

### 8.1 Feedback Types

```c
typedef enum {
    EDU_WM_PRESENTATION_OK,           // Frame presented on time
    EDU_WM_PRESENTATION_EARLY,        // Frame presented before vsync
    EDU_WM_PRESENTATION_LATE,         // Frame presented after vsync
    EDU_WM_PRESENTATION_FAILED,       // Frame was dropped
    EDU_WM_PRESENTATION_PENDING,      // Frame not yet presented
} EduWmPresentationFeedbackState;
```

### 8.2 Message Flow

```
Client                        EduWM
  │                             │
  │ 1. wp_presentation         │
  │    .feedback(surface)       │
  │────────────────────────────→│  Create feedback object
  │                             │
  │ 2. wl_surface.commit()     │
  │────────────────────────────→│  Surface committed
  │                             │
  │                             │ 3. Feedback attached to
  │                             │    next frame's presentation
  │                             │
  │                             │ 4. After page flip:
  │                             │    wp_presentation_feedback
  │                             │    .presented(
  │                             │      tv_sec_hi, tv_sec_lo,
  │                             │      tv_nsec,
  │                             │      refresh,          // ns per frame
  │                             │      seq_hi, seq_lo,   // sequence
  │                             │      flags             // HW/CLK
  │                             │    )
  │                             │───────────────────────────→│
  │                             │
  │ 5. [client uses timing data │
  │     to schedule next frame] │
  │                             │
```

### 8.3 Clock Selection

EduWM uses `CLOCK_MONOTONIC` for presentation timestamps, as required by the Wayland protocol. The clock ID is communicated to clients via `wp_presentation.clock_id`.

---

## 9. idle-inhibit Protocol

The idle-inhibit protocol prevents the system from going idle when a surface needs continuous display.

### 9.1 Usage

```
Client (e.g., video player)   EduWM
  │                             │
  │ 1. zwp_idle_inhibit_manager │
  │    .create_inhibitor(       │
  │      surface                │
  │    )                       │
  │────────────────────────────→│  Create inhibitor
  │                             │
  │                             │ 2. Inhibit count > 0
  │                             │    → Suppress idle timer
  │                             │
  │     ... video playing ...   │
  │                             │
  │ 3. zwp_idle_inhibit_        │
  │    .destroy()               │
  │────────────────────────────→│  Destroy inhibitor
  │                             │
  │                             │ 4. Inhibit count == 0
  │                             │    → Resume idle timer
  │                             │
```

### 9.2 Interaction with Session Lock

When the session is locked (via ext-session-lock), idle-inhibit is temporarily disabled. The lock screen takes precedence over idle inhibition.

---

## 10. session-lock Protocol

The ext-session-lock protocol provides a secure way to lock the screen. It replaces the traditional DPMS-based locking.

### 10.1 Lock Flow

```
Lock Client                   EduWM
  │                             │
  │ 1. ext_session_lock_v1      │
  │    .lock()                  │
  │────────────────────────────→│  Create lock object
  │                             │
  │                             │ 2. Lock is pending
  │                             │    → New surfaces are hidden
  │                             │    → Input is grabbed
  │                             │
  │ 3. ext_session_lock_v1      │
  │    .locked()                │
  │                             │───────────────────────────→│
  │                             │
  │                             │ 4. Lock is now active
  │                             │    → Screen is locked
  │                             │    → All surfaces are hidden
  │                             │    → Only the lock surface is visible
  │                             │
  │ 5. [lock screen renders]    │
  │                             │
  │ 6. ext_session_lock_v1      │
  │    .unlock()                │
  │────────────────────────────→│  Unlock request
  │                             │
  │                             │ 7. Unlock is pending
  │                             │    → Prompt for authentication
  │                             │
  │ 8. ext_session_lock_v1      │
  │    .finished()              │
  │────────────────────────────→│  Unlock confirmed
  │                             │
  │                             │ 9. Lock released
  │                             │    → All surfaces restored
  │                             │    → Input released
  │                             │
```

### 10.2 Security Properties

- While locked, no client can capture the screen.
- The lock surface is the only visible surface.
- All other surfaces are frozen and hidden.
- The lock client cannot be replaced (only one lock at a time).
- If the lock client crashes, the screen remains locked.

---

## 11. screencast Protocol

The screencast protocol (via wlr-screencopy or the xdg-desktop-portal) allows screen capture.

### 11.1 Capture Flow

```
Screencast Client             EduWM
  │                             │
  │ 1. zwlr_screencopy_manager  │
  │    .capture_output(         │
  │      cursor_mode,           │
  │      output                 │
  │    )                       │
  │────────────────────────────→│  Create screencopy frame
  │                             │
  │                             │ 2. Check permissions:
  │                             │    → Is client authorized?
  │                             │    → Has user consent?
  │                             │
  │ 3. zwlr_screencopy_frame    │
  │    .buffer(width, height,   │
  │      format, stride)        │
  │                             │───────────────────────────→│
  │                             │
  │ 4. wl_buffer.create(        │
  │      width, height, format, │
  │      stride                │
  │    )                       │
  │────────────────────────────→│  Create buffer
  │                             │
  │ 5. zwlr_screencopy_frame    │
  │    .copy(buffer)            │
  │────────────────────────────→│  Copy frame to buffer
  │                             │
  │ 6. zwlr_screencopy_frame    │
  │    .flags(Flags)            │
  │                             │───────────────────────────→│
  │                             │
  │ 7. zwlr_screencopy_frame    │
  │    .ready(tv_sec, tv_nsec,  │
  │      refresh)              │
  │                             │───────────────────────────→│
  │                             │
  │ 8. [client reads buffer]    │
  │                             │
```

### 11.2 Cursor Modes

- `hidden`: Cursor is not captured.
- `embedded`: Cursor is composited into the captured frame.
- `hidden`: Cursor is hidden in the captured frame.

---

## 12. cursor-shape Protocol

The wp-cursor-shape protocol allows clients to set the cursor shape without providing a custom cursor image.

### 12.1 Supported Shapes

```c
typedef enum {
    EDU_WM_CURSOR_DEFAULT,
    EDU_WM_CURSOR_CONTEXT_MENU,
    EDU_WM_CURSOR_HELP,
    EDU_WM_CURSOR_POINTER,
    EDU_WM_CURSOR_PROGRESS,
    EDU_WM_CURSOR_WAIT,
    EDU_WM_CURSOR_CELL,
    EDU_WM_CURSOR_CROSSHAIR,
    EDU_WM_CURSOR_TEXT,
    EDU_WM_CURSOR_VERTICAL_TEXT,
    EDU_WM_CURSOR_ALIAS,
    EDU_WM_CURSOR_COPY,
    EDU_WM_CURSOR_NO_DROP,
    EDU_WM_CURSOR_GRAB,
    EDU_WM_CURSOR_GRABBING,
    EDU_WM_CURSOR_E_RESIZE,
    EDU_WM_CURSOR_N_RESIZE,
    EDU_WM_CURSOR_NE_RESIZE,
    EDU_WM_CURSOR_NW_RESIZE,
    EDU_WM_CURSOR_S_RESIZE,
    EDU_WM_CURSOR_SE_RESIZE,
    EDU_WM_CURSOR_SW_RESIZE,
    EDU_WM_CURSOR_W_RESIZE,
    EDU_WM_CURSOR_EW_RESIZE,
    EDU_WM_CURSOR_NS_RESIZE,
    EDU_WM_CURSOR_NESW_RESIZE,
    EDU_WM_CURSOR_NWSE_RESIZE,
    EDU_WM_CURSOR_COL_RESIZE,
    EDU_WM_CURSOR_ROW_RESIZE,
} EduWmCursorShape;
```

### 12.2 Usage

```
Client                        EduWM
  │                             │
  │ 1. wp_cursor_shape_manager │
  │    .get_cursor_shape(       │
  │      pointer,               │
  │      shape                  │
  │    )                       │
  │────────────────────────────→│  Set cursor shape
  │                             │
  │                             │ 2. Load cursor from theme
  │                             │    Set cursor on output
  │                             │
```

---

## 13. input-method and text-input Protocols

These protocols enable Input Method Editor (IME) support for CJK input and other complex text input.

### 13.1 text-input Protocol

The text-input protocol allows the compositor to communicate with the input method about the text being edited:

```
Text Input Client             EduWM                  Input Method
  │                             │                        │
  │ 1. zwp_text_input_v3        │                        │
  │    .enable()                │                        │
  │────────────────────────────→│                        │
  │                             │                        │
  │ 2. zwp_text_input_v3        │                        │
  │    .set_cursor_area(x, y,   │                        │
  │      w, h)                  │                        │
  │────────────────────────────→│                        │
  │                             │                        │
  │ 3. zwp_text_input_v3        │                        │
  │    .commit()                │                        │
  │────────────────────────────→│                        │
  │                             │                        │
  │                             │ 4. zwp_input_method_v2│
  │                             │    .activate(surface)  │
  │                             │──────────────────────→│
  │                             │                        │
  │                             │ 5. zwp_input_popup_    │
  │                             │    surface_v2          │
  │                             │    .configure(x, y,    │
  │                             │      w, h)             │
  │                             │──────────────────────→│
  │                             │                        │
  │                             │ 6. [IME renders        │
  │                             │     candidate window]  │
  │                             │                        │
  │                             │ 7. zwp_input_method_v2│
  │                             │    .commit_string(     │
  │                             │      "候補")           │
  │                             │──────────────────────→│
  │                             │                        │
  │                             │ 8. zwp_text_input_v3   │
  │                             │    .commit_string(     │
  │                             │      "候補")           │
  │                             │──────────────────────→│
  │                             │                        │
  │ 9. zwp_text_input_v3        │                        │
  │    .commit_string("候補")    │                        │
  │                             │──────────────────────→│
  │                             │                        │
```

### 13.2 Virtual Keyboard Protocol

The virtual keyboard protocol allows software keyboards to inject key events:

```
Virtual Keyboard Client       EduWM
  │                             │
  │ 1. zwp_virtual_keyboard_    │
  │    manager_v1               │
  │    .create_virtual_keyboard │
  │    (seat)                  │
  │────────────────────────────→│  Create virtual keyboard
  │                             │
  │ 2. zwp_virtual_keyboard_v1  │
  │    .key(time, key, state)   │
  │────────────────────────────→│  Inject key event
  │                             │
  │ 3. zwp_virtual_keyboard_v1  │
  │    .modifiers(              │
  │      mods_depressed,        │
  │      mods_latched,          │
  │      mods_locked,           │
  │      group                  │
  │    )                       │
  │────────────────────────────→│  Set modifier state
  │                             │
```

---

## 14. How EduWM Implements Each Protocol

### 14.1 Internal State Machines

Each protocol handler maintains a state machine:

```c
typedef struct {
    EduWmProtocolState  state;
    uint32_t            last_configure_serial;
    uint32_t            pending_configure_serial;
    bool                is_configured;
    bool                is_mapped;
    // Protocol-specific fields
    union {
        struct {
            EduWmToplevelState  toplevel_state;
            bool                has_decorations;
        } xdg_shell;
        struct {
            EduWmLayer          layer;
            EduWmAnchor         anchor;
            int32_t             exclusive_zone;
        } layer_shell;
        // ...
    };
} EduWmProtocolHandler;
```

### 14.2 Message Dispatch

Protocol messages are dispatched through a vtable:

```c
typedef struct {
    void (* destroy)(void *handler);
    void (* commit)(void *handler);
    void (* configure)(void *handler, uint32_t width, uint32_t height);
} EduWmProtocolVtable;
```

Each protocol implementation registers its vtable:

```c
static const EduWmProtocolVtable xdg_toplevel_vtable = {
    .destroy = edu_wm_xdg_toplevel_destroy,
    .commit = edu_wm_xdg_toplevel_commit,
    .configure = edu_wm_xdg_toplevel_configure,
};
```

### 14.3 Surface Lifecycle Integration

All protocol handlers integrate with the core surface lifecycle:

```
Protocol Created → Protocol Configured → Surface Committed →
  → Surface Mapped → Surface Active → Surface Unmapped →
  → Surface Destroyed → Protocol Destroyed
```

---

## 15. Protocol Extension Guide for Developers

### 15.1 Adding a New Protocol

To add support for a new Wayland protocol in EduWM:

1. **Add the XML definition** to `protocol/`:
   ```xml
   <protocol name="wp_example">
     <interface name="wp_example_manager_v1" version="1">
       <description summary="Example protocol">
         An example protocol for demonstration.
       </description>
       <request name="get_example">
         <description summary="Create an example object"/>
         <arg name="id" type="new_id" interface="wp_example_v1"/>
       </request>
     </interface>
   </protocol>
   ```

2. **Generate the bindings**:
   ```bash
   wayland-scanner server-header protocol/wp-example.xml \
     src/generated/wp-example-protocol.h
   wayland-scanner public-code protocol/wp-example.xml \
     src/generated/wp-example-protocol.c
   ```

3. **Implement the handler**:
   ```c
   // src/protocols/wp_example.c
   struct wp_example_manager_v1_interface manager_impl = {
       .destroy = handle_manager_destroy,
       .get_example = handle_get_example,
   };

   static void handle_get_example(struct wl_client *client,
                                    struct wl_resource *manager_resource,
                                    uint32_t id,
                                    struct wl_resource *surface) {
       // Create the example object
       struct wp_example_v1 *example = calloc(1, sizeof(*example));
       // ... initialize ...
   }
   ```

4. **Register the global**:
   ```c
   void edu_wm_protocols_init(EduWmCompositor *compositor) {
       wl_global_create(compositor->display,
                        &wp_example_manager_v1_interface, 1,
                        compositor, bind_example_manager);
   }
   ```

### 15.2 Testing New Protocols

1. Write a test client that exercises all protocol requests.
2. Verify the compositor handles all edge cases (null surfaces, invalid arguments, etc.).
3. Run the protocol under Wayland's protocol validator (`wayland-info`).
4. Test with `WAYLAND_DEBUG=1` to verify message sequences.

---

## 16. Testing Wayland Protocol Compliance

### 16.1 Compliance Testing Tools

| Tool | Purpose |
|------|---------|
| `wayland-info` | Display advertised protocols and their versions |
| `wlr-randr` | Test output management |
| `weston-terminal` | Test basic xdg-shell |
| `kwin_wayland` | Cross-reference behavior |
| `WAYLAND_DEBUG=1` | Verbose protocol logging |
| `wayland-test-runner` | Automated protocol testing |

### 16.2 Compliance Checklist

For each protocol:
- [ ] All interfaces are advertised at the correct version.
- [ ] All requests are handled correctly.
- [ ] All events are sent at the correct time.
- [ ] Error conditions are handled (invalid arguments, bad state).
- [ ] Protocol objects are destroyed when the client disconnects.
- [ ] Multiple clients can use the protocol simultaneously.
- [ ] The protocol works correctly with XWayland surfaces.

### 16.3 Automated Testing

EduWM includes an automated protocol test suite:

```bash
# Run all protocol tests
eduwm-test --protocols

# Run tests for a specific protocol
eduwm-test --protocol xdg-shell

# Run with WAYLAND_DEBUG for verbose output
WAYLAND_DEBUG=1 eduwm-test --protocol xdg-shell
```

### 16.4 Stress Testing

```bash
# Create 1000 windows rapidly
eduwm-stress --create-windows 1000 --delay 1ms

# Rapid workspace switching
eduwm-stress --switch-workspaces 10000 --delay 10ms

# Monitor hotplug simulation
eduwm-stress --hotplug 100 --delay 100ms
```

---

*This document is part of the EduWM v2 documentation suite.*
*Last updated: 2026-07-10*
