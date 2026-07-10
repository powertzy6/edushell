# EduWM Window Manager — Developer Guide

## 1. Getting Started: Building EduWM from Source

### 1.1 Prerequisites

Install the following development packages:

```bash
# Ubuntu/Debian
sudo apt install \
    build-essential meson ninja-build \
    libwayland-dev wayland-protocols \
    libdrm-dev libgbm-dev libinput-dev \
    libudev-dev libseat-dev \
    libxkbcommon-dev \
    libgl-dev libgles2-mesa-dev \
    libcairo2-dev libpango1.0-dev \
    libglib2.0-dev libdbus-1-dev \
    libpixman-1-dev \
    xwayland \
    hwdata \
    python3-docutils \
    libsystemd-dev \
    libliftoff-dev \
    libdisplay-info-dev

# Fedora
sudo dnf install \
    meson ninja-build gcc \
    wayland-devel wayland-protocols \
    libdrm-devel libgbm-devel libinput-devel \
    systemd-devel libseat-devel \
    libxkbcommon-devel \
    mesa-libEGL-devel mesa-libGLESv2-devel \
    cairo-devel pango-devel \
    glib2-devel dbus-devel \
    pixman-devel \
    xorg-x11-server-Xwayland \
    hwdata \
    python3-docutils \
    libliftoff-devel \
    libdisplay-info-devel

# Arch
sudo pacman -S \
    meson ninja gcc \
    wayland wayland-protocols \
    libdrm libgbm libinput \
    systemd libseat \
    libxkbcommon \
    mesa cairo pango \
    glib2 dbus \
    pixman \
    xorg-server-xwayland \
    hwdata \
    python-sphinx
```

### 1.2 Clone and Build

```bash
# Clone the repository
git clone https://github.com/cinnamon-team/eduwm.git
cd eduwm

# Initialize submodules
git submodule update --init --recursive

# Create build directory
meson setup build

# Build
ninja -C build

# Install (requires root)
sudo ninja -C build install
```

### 1.3 Build Options

```bash
# Custom installation prefix
meson setup build --prefix=/usr/local

# Enable XWayland support (default: enabled)
meson setup build -Dxwayland=enabled

# Enable DRM backend (default: enabled)
meson setup build -Ddrm-backend=enabled

# Enable X11 backend (for testing without DRM)
meson setup build -Dx11-backend=enabled

# Enable Vulkan renderer (default: disabled, experimental)
meson setup build -Dvulkan=enabled

# Enable debugging features
meson setup build -Ddebug=true

# Enable profiling
meson setup build -Dprofiling=true

# Build documentation
meson setup build -Ddocs=true
ninja -C build docs
```

### 1.4 Run Without Installing

```bash
# Run from the build directory
./build/eduwm --wayland-display wayland-1

# Or test with X11 backend
./build/eduwm --x11-display :1
```

---

## 2. Project Structure Overview

```
eduwm/
├── src/
│   ├── main.c                     # Entry point
│   ├── compositor/
│   │   ├── compositor.c           # Main compositor loop
│   │   ├── compositor.h
│   │   ├── drm_backend.c          # DRM/KMS backend
│   │   ├── drm_backend.h
│   │   ├── x11_backend.c          # X11 backend (testing)
│   │   ├── renderer.c             # OpenGL ES renderer
│   │   ├── renderer.h
│   │   ├── damage.c               # Damage tracking
│   │   ├── damage.h
│   │   ├── frame_pacing.c         # Frame pacing and vsync
│   │   └── frame_pacing.h
│   ├── core/
│   │   ├── window_core.c          # Window management core
│   │   ├── window_core.h
│   │   ├── focus_manager.c        # Focus management
│   │   ├── focus_manager.h
│   │   ├── stacking_manager.c     # Z-order management
│   │   ├── stacking_manager.h
│   │   └── window_rules_engine.c  # Window rules
│   ├── workspace/
│   │   ├── workspace_engine_v2.c  # Workspace management
│   │   ├── workspace_engine_v2.h
│   │   ├── overview.c             # Overview mode
│   │   └── overview.h
│   ├── monitor/
│   │   ├── monitor_manager.c      # Multi-monitor support
│   │   └── monitor_manager.h
│   ├── input/
│   │   ├── input_manager.c        # Input device management
│   │   ├── input_manager.h
│   │   ├── keyboard.c             # Keyboard handling
│   │   ├── pointer.c              # Pointer handling
│   │   ├── touch.c                # Touch handling
│   │   ├── tablet.c               # Tablet handling
│   │   ├── gesture.c              # Gesture recognition
│   │   └── shortcuts.c            # Shortcut engine
│   ├── animation/
│   │   ├── animation_engine_v2.c  # Animation engine
│   │   ├── animation_engine_v2.h
│   │   ├── easing.c               # Easing functions
│   │   ├── spring.c               # Spring physics
│   │   └── stagger.c              # Stagger animations
│   ├── theme/
│   │   ├── theme_layer.c          # Theme management
│   │   ├── theme_layer.h
│   │   ├── css_parser.c           # CSS parser
│   │   ├── decorations.c          # Window decorations
│   │   ├── shadows.c              # Shadow rendering
│   │   └── border.c               # Border rendering
│   ├── wayland/
│   │   ├── wayland_compat.c       # Wayland protocol support
│   │   ├── wayland_compat.h
│   │   ├── xdg_shell.c            # xdg-shell implementation
│   │   ├── layer_shell.c          # layer-shell implementation
│   │   ├── xdg_decoration.c       # xdg-decoration
│   │   ├── fractional_scale.c     # Fractional scaling
│   │   ├── viewporter.c           # Viewporter
│   │   ├── pointer_constraints.c  # Pointer constraints
│   │   ├── relative_pointer.c     # Relative pointer
│   │   ├── presentation_time.c    # Presentation time
│   │   ├── idle_inhibit.c         # Idle inhibit
│   │   ├── session_lock.c         # Session lock
│   │   ├── screencast.c           # Screencopy
│   │   ├── cursor_shape.c         # Cursor shape
│   │   ├── input_method.c         # Input method
│   │   ├── text_input.c           # Text input
│   │   └── virtual_keyboard.c     # Virtual keyboard
│   ├── x11/
│   │   ├── x11_compat.c           # XWayland support
│   │   ├── x11_compat.h
│   │   ├── ewmh.c                 # EWMH atoms
│   │   └── icccm.c                # ICCCM compliance
│   ├── crash/
│   │   ├── crash_recovery.c       # Crash recovery
│   │   ├── crash_recovery.h
│   │   └── session_save.c         # Session save/restore
│   ├── security/
│   │   ├── security_manager.c     # Security management
│   │   └── security_manager.h
│   ├── debug/
│   │   ├── debug_tool.c           # Debug tools
│   │   ├── debug_tool.h
│   │   ├── profiler.c             # Profiling
│   │   └── inspector.c            # Window inspector
│   ├── ipc/
│   │   ├── dbus_interface.c       # D-Bus IPC
│   │   └── dbus_interface.h
│   └── util/
│       ├── log.c                  # Logging
│       ├── log.h
│       ├── signal.c               # Signal handling
│       ├── signal.h
│       ├── rect.c                 # Rectangle math
│       ├── rect.h
│       └── timer.c                # High-resolution timer
├── protocol/
│   ├── xdg-shell.xml
│   ├── layer-shell.xml
│   ├── xdg-decoration.xml
│   ├── fractional-scale.xml
│   ├── viewporter.xml
│   ├── pointer-constraints.xml
│   ├── relative-pointer.xml
│   ├── presentation-time.xml
│   ├── idle-inhibit.xml
│   ├── session-lock.xml
│   ├── screencopy.xml
│   ├── cursor-shape.xml
│   ├── input-method.xml
│   ├── text-input.xml
│   └── virtual-keyboard.xml
├── data/
│   ├── eduwm.desktop              # Desktop entry
│   ├── eduwm.service              # Systemd service
│   └── eduwm-wayland.desktop      # Wayland session
├── themes/
│   ├── default.css                # Default theme
│   └── minimal.css                # Minimal theme
├── test/
│   ├── test_core.c                # Core module tests
│   ├── test_workspace.c           # Workspace tests
│   ├── test_monitor.c             # Monitor tests
│   ├── test_input.c               # Input tests
│   ├── test_animation.c           # Animation tests
│   ├── test_theme.c               # Theme tests
│   ├── test_wayland.c             # Wayland protocol tests
│   ├── test_x11.c                 # X11 compatibility tests
│   ├── test_crash.c               # Crash recovery tests
│   ├── test_security.c            # Security tests
│   ├── test_stress.c              # Stress tests
│   └── test_perf.c                # Performance tests
├── docs/
│   └── v2/
│       ├── eduwm-architecture.md
│       ├── eduwm-wayland-guide.md
│       ├── eduwm-migration-guide.md
│       ├── eduwm-api-reference.md
│       └── eduwm-developer-guide.md
├── meson.build                    # Build configuration
├── meson_options.txt              # Build options
├── LICENSE                        # MIT License
└── README.md
```

---

## 3. Adding a New Wayland Protocol

This section walks through adding support for a new Wayland protocol.

### 3.1 Example: Adding `wp_content_protection`

We'll add content protection (HDCP) support as an example.

### Step 1: Add Protocol XML

Create `protocol/content-protection.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<protocol name="wp_content_protection">

  <description summary="Content protection for DRM content">
    Protocol for enabling content protection on outputs.
    Used for playing DRM-protected content (Netflix, etc.).
  </description>

  <interface name="wp_content_protection_manager_v1" version="1">
    <description summary="Content protection manager"/>
    <request name="destroy" type="destructor">
      <description summary="Destroy the manager"/>
    </request>
    <request name="create_session">
      <description summary="Create a protection session"/>
      <arg name="id" type="new_id" interface="wp_content_protection_session_v1"/>
    </request>
  </interface>

  <interface name="wp_content_protection_session_v1" version="1">
    <description summary="A content protection session"/>
    <request name="destroy" type="destructor">
      <description summary="Destroy the session"/>
    </request>
    <request name="enable">
      <description summary="Enable protection for an output"/>
      <arg name="output" type="object" interface="wl_output"/>
    </request>
    <request name="disable">
      <description summary="Disable protection for an output"/>
      <arg name="output" type="object" interface="wl_output"/>
    </request>
    <event name="status">
      <description summary="Protection status changed"/>
      <arg name="output" type="object" interface="wl_output"/>
      <arg name="status" type="uint" enum="protection_status"/>
    </event>
  </interface>

  <enum name="protection_status">
    <entry name="disabled" value="0"/>
    <entry name="enabled" value="1"/>
    <entry name="failed" value="2"/>
    <entry name="pending" value="3"/>
  </enum>

</protocol>
```

### Step 2: Generate Bindings

Add to `meson.build`:

```meson
# Generate Wayland protocol bindings
wp_protocols = [
    # ... existing protocols ...
    'content-protection',
]

foreach proto : wp_protocols
    custom_target(
        proto + '_server_h',
        input: proto + '.xml',
        output: proto + '-protocol.h',
        command: [wayland_scanner, 'server-header', '@INPUT@', '@OUTPUT@'],
    )
    custom_target(
        proto + '_code_c',
        input: proto + '.xml',
        output: proto + '-protocol.c',
        command: [wayland_scanner, 'public-code', '@INPUT@', '@OUTPUT@'],
    )
endforeach
```

### Step 3: Implement the Handler

Create `src/wayland/content-protection.c`:

```c
#include "content-protection.h"
#include "generated/content-protection-protocol.h"

struct edu_wm_content_protection {
    struct wl_global *manager_global;
    struct wl_list sessions;  // list of EduWmProtectionSession
    EduWmCompositor *compositor;
};

typedef struct {
    struct wl_resource *resource;
    struct wl_list link;
    struct wl_list enabled_outputs;
    bool is_active;
} EduWmProtectionSession;

static void
handle_manager_destroy(struct wl_client *client, struct wl_resource *resource) {
    wl_resource_destroy(resource);
}

static void
handle_manager_create_session(struct wl_client *client,
                               struct wl_resource *manager_resource,
                               uint32_t id) {
    struct edu_wm_content_protection *cp = wl_resource_get_user_data(manager_resource);

    // Create session resource
    struct wl_resource *session_resource = wl_resource_create(
        client, &wp_content_protection_session_v1_interface, 1, id
    );

    // Set implementation
    static const struct wp_content_protection_session_v1_interface impl = {
        .destroy = handle_session_destroy,
        .enable = handle_session_enable,
        .disable = handle_session_disable,
    };
    wl_resource_set_implementation(session_resource, &impl, cp, NULL);

    // Create session object
    EduWmProtectionSession *session = calloc(1, sizeof(*session));
    session->resource = session_resource;
    wl_list_init(&session->enabled_outputs);
    wl_list_insert(&cp->sessions, &session->link);
}

static void
handle_session_enable(struct wl_client *client,
                       struct wl_resource *session_resource,
                       struct wl_resource *output_resource) {
    // Get the output
    EduWmOutput *output = wl_resource_get_user_data(output_resource);
    EduWmProtectionSession *session = wl_resource_get_user_data(session_resource);

    // Enable HDCP on the output
    bool success = edu_wm_drm_enable_hdcp(output);

    // Send status event
    uint32_t status = success ?
        WP_CONTENT_PROTECTION_SESSION_V1_PROTECTION_STATUS_ENABLED :
        WP_CONTENT_PROTECTION_SESSION_V1_PROTECTION_STATUS_FAILED;

    wp_content_protection_session_v1_send_status(
        session->resource, output_resource, status
    );
}

// ... other handlers ...

static void
bind_manager(struct wl_client *client, void *data,
             uint32_t version, uint32_t id) {
    struct edu_wm_content_protection *cp = data;

    struct wl_resource *resource = wl_resource_create(
        client, &wp_content_protection_manager_v1_interface, version, id
    );

    static const struct wp_content_protection_manager_v1_interface impl = {
        .destroy = handle_manager_destroy,
        .create_session = handle_manager_create_session,
    };
    wl_resource_set_implementation(resource, &impl, cp, NULL);
}

EduWmContentProtection *
edu_wm_content_protection_init(EduWmCompositor *compositor) {
    EduWmContentProtection *cp = calloc(1, sizeof(*cp));
    cp->compositor = compositor;
    wl_list_init(&cp->sessions);

    cp->manager_global = wl_global_create(
        compositor->display,
        &wp_content_protection_manager_v1_interface,
        1, cp, bind_manager
    );

    return cp;
}
```

### Step 4: Register in Compositor

In `src/compositor/compositor.c`:

```c
#include "wayland/content-protection.h"

int edu_wm_compositor_init(EduWmCompositor *compositor, int argc, char **argv) {
    // ... existing initialization ...

    // Initialize content protection
    compositor->content_protection = edu_wm_content_protection_init(compositor);

    return 0;
}
```

### Step 5: Write Tests

Create `test/test_content_protection.c`:

```c
#include "test_common.h"
#include "wayland/content-protection.h"

static void test_create_session(EduWmTestFramework *fw) {
    EduWmCompositor *compositor = edu_wm_test_get_compositor(fw);
    EduWmContentProtection *cp = compositor->content_protection;

    // Create a test client
    struct wl_display *client_display = wl_display_connect(NULL);
    TEST_ASSERT(client_display != NULL, "Failed to connect test client");

    // Create session via protocol
    struct wl_registry *registry = wl_display_get_registry(client_display);
    // ... test protocol interaction ...

    wl_display_disconnect(client_display);
}

void test_content_protection_suite(EduWmTestFramework *fw) {
    EduWmTestSuite *suite = edu_wm_test_suite_create(fw, "content-protection");
    edu_wm_test_case_add(suite, "create-session", test_create_session);
    // ... more tests ...
}
```

### Step 6: Documentation

Add documentation to `docs/v2/eduwm-wayland-guide.md` under the appropriate section.

---

## 4. Implementing a New Animation Effect

### 4.1 Creating a Custom Easing Function

Add a new easing function to `src/animation/easing.c`:

```c
// In src/animation/easing.c

float edu_wm_easing_custom_bounce_in_out(float t) {
    // Bounce in-out: bounces at both start and end
    if (t < 0.5) {
        return edu_wm_easing_bounce_in(t * 2) * 0.5f;
    } else {
        return edu_wm_easing_bounce_out(t * 2 - 1) * 0.5f + 0.5f;
    }
}

// Register in the easing function table
static const EduWmEasingEntry easings[] = {
    // ... existing entries ...
    { EDU_WM_ANIMATION_CUSTOM_BOUNCE_IN_OUT, edu_wm_easing_custom_bounce_in_out },
};
```

### 4.2 Creating a Composite Animation

Example: A "window open" animation that combines scale, opacity, and position:

```c
// In src/animation/presets/window_open.c

#include "animation_engine_v2.h"

EduWmAnimationId edu_wm_animation_window_open(
    EduWmAnimationEngine *engine,
    EduWmWindow *window,
    EduWmRect target_rect
) {
    // 1. Create the parallel group
    EduWmAnimationId group = edu_wm_animation_group_create(
        engine, EDU_WM_ANIMATION_PARALLEL
    );

    // 2. Opacity animation: 0 → 1 over 200ms with ease-out
    EduWmAnimationId fade = edu_wm_animation_create(engine,
        (EduWmAnimationConfig){
            .duration_ms = 200,
            .easing = EDU_WM_ANIMATION_EASE_OUT,
            .from = 0.0f,
            .to = 1.0f,
        },
        update_opacity,
        &(struct anim_ctx){ .window = window }
    );

    // 3. Scale animation: 0.8 → 1.0 over 250ms with spring
    EduWmAnimationId scale = edu_wm_animation_spring_create(engine,
        (EduWmSpringConfig){
            .mass = 1.0f,
            .stiffness = 200.0f,
            .damping = 20.0f,
        },
        1.0f,  // target scale
        update_scale,
        &(struct anim_ctx){ .window = window }
    );

    // 4. Position animation: slide up from below
    EduWmAnimationId slide = edu_wm_animation_create(engine,
        (EduWmAnimationConfig){
            .duration_ms = 250,
            .easing = EDU_WM_ANIMATION_CUBIC_OUT,
            .from = target_rect.y + 20,  // 20px below target
            .to = target_rect.y,
        },
        update_position_y,
        &(struct anim_ctx){ .window = window }
    );

    // 5. Add all animations to the group
    edu_wm_animation_group_add(engine, group, fade);
    edu_wm_animation_group_add(engine, group, scale);
    edu_wm_animation_group_add(engine, group, slide);

    // 6. Start the group
    edu_wm_animation_group_start(engine, group);

    return group;
}

// Update callbacks
static void update_opacity(float value, void *user_data) {
    struct anim_ctx *ctx = user_data;
    edu_wm_window_set_opacity(ctx->window, value);
}

static void update_scale(float value, void *user_data) {
    struct anim_ctx *ctx = user_data;
    edu_wm_window_set_scale(ctx->window, value, value);
}

static void update_position_y(float value, void *user_data) {
    struct anim_ctx *ctx = user_data;
    EduWmRect rect = edu_wm_window_get_geometry(ctx->window);
    edu_wm_window_move(ctx->window, rect.x, (int32_t)value);
}
```

### 4.3 Registering Preset Animations

```c
// In src/animation/presets/presets.c

void edu_wm_animation_presets_init(EduWmAnimationEngine *engine) {
    // Register preset animations
    edu_wm_animation_register_preset(engine, "window-open",
        edu_wm_animation_window_open);
    edu_wm_animation_register_preset(engine, "window-close",
        edu_wm_animation_window_close);
    edu_wm_animation_register_preset(engine, "workspace-switch",
        edu_wm_animation_workspace_switch);
    edu_wm_animation_register_preset(engine, "minimize",
        edu_wm_animation_minimize);
    edu_wm_animation_register_preset(engine, "maximize",
        edu_wm_animation_maximize);
}
```

---

## 5. Creating a Custom Window Rule

### 5.1 Define a Rule Type

Add a new rule type to the window rules engine:

```c
// In src/core/window_rules_engine.c

typedef enum {
    EDU_WM_RULE_MATCH_APP_ID,
    EDU_WM_RULE_MATCH_TITLE,
    EDU_WM_RULE_MATCH_CLASS,
    EDU_WM_RULE_MATCH_PID,
    EDU_WM_RULE_MATCH_ROLE,
    EDU_WM_RULE_MATCH_TYPE,
} EduWmRuleMatchType;

typedef struct {
    EduWmRuleMatchType type;
    const char        *pattern;    // regex pattern
    bool               negate;     // match if NOT matching
} EduWmRuleMatch;

typedef struct {
    EduWmRuleMatch  *matches;
    uint32_t         match_count;
    EduWmRuleAction  actions;
    uint32_t         priority;
    const char      *name;         // for debugging
} EduWmWindowRule;
```

### 5.2 Parse Rules from JSON

```c
EduWmWindowRule *edu_wm_window_rule_parse(const char *json) {
    json_object *root = json_tokener_parse(json);
    if (!root) return NULL;

    EduWmWindowRule *rule = calloc(1, sizeof(*rule));

    // Parse match conditions
    json_object *match = json_object_object_get(root, "match");
    if (match) {
        rule->match_count = json_object_object_length(match);
        rule->matches = calloc(rule->match_count, sizeof(EduWmRuleMatch));

        uint32_t i = 0;
        json_object_object_foreach(match, key, val) {
            rule->matches[i].type = match_type_from_string(key);
            rule->matches[i].pattern = strdup(json_object_get_string(val));
            i++;
        }
    }

    // Parse actions
    json_object *actions = json_object_object_get(root, "actions");
    if (actions) {
        rule->actions = parse_actions(actions);
    }

    // Parse priority
    json_object *priority = json_object_object_get(root, "priority");
    if (priority) {
        rule->priority = json_object_get_int(priority);
    }

    json_object_put(root);
    return rule;
}
```

### 5.3 Evaluate Rules Against Windows

```c
bool edu_wm_window_rule_matches(
    const EduWmWindowRule *rule,
    const EduWmWindow *window
) {
    for (uint32_t i = 0; i < rule->match_count; i++) {
        bool matched = match_single(&rule->matches[i], window);
        if (rule->matches[i].negate) matched = !matched;
        if (!matched) return false;
    }
    return true;
}

static bool match_single(const EduWmRuleMatch *match, const EduWmWindow *window) {
    const char *value = NULL;

    switch (match->type) {
    case EDU_WM_RULE_MATCH_APP_ID:
        value = window->app_id;
        break;
    case EDU_WM_RULE_MATCH_TITLE:
        value = window->title;
        break;
    case EDU_WM_RULE_MATCH_CLASS:
        value = window->wm_class;
        break;
    case EDU_WM_RULE_MATCH_PID:
        // Compare PID directly
        return window->pid == atoi(match->pattern);
    case EDU_WM_RULE_MATCH_ROLE:
        value = window->role;
        break;
    default:
        return false;
    }

    if (!value) return false;
    return regex_match(match->pattern, value);
}
```

### 5.4 Apply Rule Actions

```c
void edu_wm_window_rule_apply(
    const EduWmWindowRule *rule,
    EduWmWindow *window
) {
    if (rule->actions.has_workspace) {
        edu_wm_window_move_to_workspace(window, rule->actions.workspace);
    }
    if (rule->actions.has_floating) {
        window->is_floating = rule->actions.floating;
    }
    if (rule->actions.has_tile) {
        edu_wm_window_tile(window, rule->actions.tile_position);
    }
    if (rule->actions.has_opacity) {
        edu_wm_window_set_opacity(window, rule->actions.opacity);
    }
    if (rule->actions.has_layer) {
        edu_wm_stacking_set_layer(window, rule->actions.layer);
    }
    // ... more actions ...
}
```

---

## 6. Adding Monitor Configuration Support

### 6.1 Custom Monitor Configurations

Support custom per-monitor configurations:

```c
// In src/monitor/monitor_config.c

typedef struct {
    const char     *output_name;     // e.g., "DP-1"
    uint32_t        x, y;           // position
    uint32_t        width, height;  // resolution
    float           scale;          // scale factor
    float           refresh_rate;   // refresh rate
    EduWmTransform  transform;      // rotation
    bool            is_primary;     // primary monitor
} EduWmMonitorConfigEntry;

typedef struct {
    EduWmMonitorConfigEntry *entries;
    uint32_t                 count;
    bool                     auto_detect;  // auto-detect unknown monitors
} EduWmMonitorConfig;
```

### 6.2 Load Configuration from File

```c
EduWmMonitorConfig *edu_wm_monitor_config_load(const char *path) {
    FILE *f = fopen(path, "r");
    if (!f) return NULL;

    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);

    char *json = malloc(size + 1);
    fread(json, 1, size, f);
    json[size] = '\0';
    fclose(f);

    EduWmMonitorConfig *config = calloc(1, sizeof(*config));
    json_object *root = json_tokener_parse(json);

    // Parse entries
    json_object *monitors = json_object_object_get(root, "monitors");
    config->count = json_object_array_length(monitors);
    config->entries = calloc(config->count, sizeof(EduWmMonitorConfigEntry));

    for (uint32_t i = 0; i < config->count; i++) {
        json_object *entry = json_object_array_get_idx(monitors, i);
        parse_monitor_entry(entry, &config->entries[i]);
    }

    // Parse auto_detect
    json_object *ad = json_object_object_get(root, "auto_detect");
    config->auto_detect = ad ? json_object_get_boolean(ad) : true;

    json_object_put(root);
    free(json);
    return config;
}
```

### 6.3 Apply Configuration on Hotplug

```c
void edu_wm_monitor_config_apply(
    EduWmMonitorConfig *config,
    EduWmMonitorManager *manager,
    EduWmOutput *output
) {
    // Find matching entry
    for (uint32_t i = 0; i < config->count; i++) {
        if (strcmp(config->entries[i].output_name, output->name) == 0) {
            EduWmMonitorConfigEntry *entry = &config->entries[i];

            // Apply position
            edu_wm_output_set_position(output, entry->x, entry->y);

            // Apply mode
            edu_wm_output_set_mode(output, entry->width, entry->height,
                                   entry->refresh_rate);

            // Apply scale
            edu_wm_output_set_scale(output, entry->scale);

            // Apply transform
            edu_wm_output_set_transform(output, entry->transform);

            // Apply primary
            if (entry->is_primary) {
                edu_wm_monitor_set_primary(manager, output->id);
            }

            return;
        }
    }

    // No matching entry found; use defaults
    if (config->auto_detect) {
        edu_wm_monitor_config_apply_defaults(output);
    }
}
```

---

## 7. Extending the Input System with New Device Types

### 7.1 Define the Device Interface

```c
// In src/input/device_interface.h

typedef struct {
    const char     *name;
    EduWmDeviceType  type;

    // Lifecycle
    int  (*open)(struct EduWmInputDevice *dev);
    void (*close)(struct EduWmInputDevice *dev);

    // Event processing
    void (*process_event)(struct EduWmInputDevice *dev,
                          const void *event);

    // Capabilities
    bool (*has_capability)(struct EduWmInputDevice *dev,
                           const char *capability);
} EduWmInputDeviceInterface;
```

### 7.2 Implement the Device Driver

```c
// In src/input/touchpad_driver.c

#include "device_interface.h"

static int touchpad_open(EduWmInputDevice *dev) {
    // Open the device file
    dev->fd = open(dev->path, O_RDONLY | O_NONBLOCK);
    if (dev->fd < 0) return -1;

    // Enable necessary events
    ioctl(dev->fd, EVIOCSGIDMAP, &keymap);

    return 0;
}

static void touchpad_close(EduWmInputDevice *dev) {
    close(dev->fd);
}

static void touchpad_process_event(EduWmInputDevice *dev, const void *event) {
    const struct input_event *ev = event;

    switch (ev->type) {
    case EV_ABS:
        touchpad_handle_abs(dev, ev);
        break;
    case EV_REL:
        touchpad_handle_rel(dev, ev);
        break;
    case EV_KEY:
        touchpad_handle_key(dev, ev);
        break;
    case EV_SYN:
        touchpad_handle_syn(dev, ev);
        break;
    }
}

static void touchpad_handle_syn(EduWmInputDevice *dev,
                                 const struct input_event *ev) {
    // Process accumulated events
    if (dev->touch_count > 0) {
        // Multi-touch gesture
        edu_wm_gesture_process_touch(dev->gesture_engine, dev->touches,
                                      dev->touch_count);
    } else {
        // Single pointer movement
        edu_wm_pointer_update(dev->pointer, dev->abs_x, dev->abs_y);
    }

    // Reset for next frame
    dev->pending_dx = 0;
    dev->pending_dy = 0;
}

const EduWmInputDeviceInterface edu_wm_touchpad_driver = {
    .name = "touchpad",
    .type = EDU_WM_DEVICE_POINTER,
    .open = touchpad_open,
    .close = touchpad_close,
    .process_event = touchpad_process_event,
    .has_capability = touchpad_has_capability,
};
```

### 7.3 Register the Driver

```c
// In src/input/input_manager.c

static const EduWmInputDeviceInterface *drivers[] = {
    &edu_wm_keyboard_driver,
    &edu_wm_pointer_driver,
    &edu_wm_touchpad_driver,
    &edu_wm_touchscreen_driver,
    &edu_wm_tablet_driver,
    &edu_wm_gamepad_driver,      // new!
    NULL,
};

void edu_wm_input_manager_register_drivers(EduWmInputManager *manager) {
    for (int i = 0; drivers[i]; i++) {
        wl_list_insert(&manager->driver_list, &drivers[i]->link);
    }
}
```

---

## 8. Writing Security Policies

### 8.1 Define Permission Rules

Create a security policy file (`~/.config/eduwm/security-policy.json`):

```json
{
    "version": 1,
    "default_permissions": "screen_capture deny, pointer_lock deny, keyboard_grab deny",
    "rules": [
        {
            "match": {
                "app_id": "org.gnome.Screenshot"
            },
            "permissions": "screen_capture allow"
        },
        {
            "match": {
                "app_id": "org.remmina.Remmina"
            },
            "permissions": "screen_capture allow, pointer_lock allow"
        },
        {
            "match": {
                "app_id": "com.example.game"
            },
            "permissions": "pointer_lock allow, keyboard_grab allow"
        },
        {
            "match": {
                "app_id": "org.freedesktop.portal.*"
            },
            "permissions": "screen_capture allow",
            "require_consent": true
        }
    ]
}
```

### 8.2 Implement the Policy Engine

```c
// In src/security/policy_engine.c

typedef struct {
    const char *app_id_pattern;
    EduWmPermission permissions;
    bool require_consent;
} EduWmSecurityRule;

EduWmPermission edu_wm_security_evaluate(
    EduWmPolicyEngine *engine,
    struct wl_client *client,
    EduWmPermission requested
) {
    // Get client's app_id
    const char *app_id = edu_wm_security_get_client_app_id(client);

    // Check rules
    for (uint32_t i = 0; i < engine->rule_count; i++) {
        EduWmSecurityRule *rule = &engine->rules[i];

        if (regex_match(rule->app_id_pattern, app_id)) {
            // Check if requested permission is in the rule
            if (requested & rule->permissions) {
                // Check if consent is required
                if (rule->require_consent) {
                    if (!edu_wm_security_has_consent(client, requested)) {
                        // Prompt the user
                        bool granted = edu_wm_security_prompt_user(
                            client, requested
                        );
                        if (granted) {
                            edu_wm_security_record_consent(client, requested);
                            return requested;
                        }
                        return EDU_WM_PERM_NONE;
                    }
                }
                return requested;
            }
        }
    }

    // No rule matched; apply default policy
    return engine->default_permissions & requested;
}
```

### 8.3 Audit Logging

```c
void edu_wm_security_audit(
    EduWmSecurityManager *security,
    const char *event_type,
    struct wl_client *client,
    EduWmPermission permission,
    bool granted
) {
    char timestamp[64];
    time_t now = time(NULL);
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%dT%H:%M:%S", localtime(&now));

    const char *app_id = edu_wm_security_get_client_app_id(client);
    pid_t pid = edu_wm_security_get_client_pid(client);

    fprintf(security->audit_log,
        "[%s] %s: app=%s pid=%d permission=%s result=%s\n",
        timestamp,
        event_type,
        app_id ? app_id : "unknown",
        pid,
        edu_wm_permission_name(permission),
        granted ? "GRANTED" : "DENIED"
    );
}
```

---

## 9. Creating Custom Inspectors/Debug Tools

### 9.1 Define an Inspector

```c
// In src/debug/inspector_custom.c

typedef struct {
    const char *name;
    const char *description;
    void (*inspect)(EduWmDebugInspector *inspector,
                    EduWmWindow *window,
                    EduWmInspectorOutput *output);
} EduWmCustomInspector;

// Example: Memory usage inspector
static void inspect_memory(EduWmDebugInspector *inspector,
                            EduWmWindow *window,
                            EduWmInspectorOutput *output) {
    EduWmBufferInfo buf = edu_wm_window_get_buffer_info(window);

    edu_wm_inspector_output_add_field(output, "Buffer Size",
        "%zu bytes", buf.size);
    edu_wm_inspector_output_add_field(output, "Buffer Format",
        "%s", buf.format_name);
    edu_wm_inspector_output_add_field(output, "Buffer Age",
        "%d frames", buf.age);
    edu_wm_inspector_output_add_field(output, "Damage Regions",
        "%d", buf.damage_count);
}

// Register the inspector
void edu_wm_debug_register_custom_inspectors(EduWmDebugTool *debug) {
    edu_wm_debug_register_inspector(debug,
        &(EduWmCustomInspector){
            .name = "memory",
            .description = "Display memory usage information",
            .inspect = inspect_memory,
        });

    edu_wm_debug_register_inspector(debug,
        &(EduWmCustomInspector){
            .name = "compositing",
            .description = "Display compositing information",
            .inspect = inspect_compositing,
        });
}
```

### 9.2 Custom Profiling Metrics

```c
// In src/debug/profiler_custom.c

typedef struct {
    const char *name;
    const char *unit;
    double (*collect)(void *data);
    void *collect_data;
} EduWmCustomMetric;

// Example: Queue depth metric
static double collect_queue_depth(void *data) {
    EduWmCompositor *compositor = data;
    return (double)edu_wm_compositor_get_pending_frame_count(compositor);
}

void edu_wm_profiler_register_custom_metrics(EduWmProfilingTool *profiler) {
    EduWmCustomMetric metrics[] = {
        {
            .name = "queue_depth",
            .unit = "frames",
            .collect = collect_queue_depth,
            .collect_data = compositor,
        },
        // ... more metrics ...
    };

    for (int i = 0; i < ARRAY_SIZE(metrics); i++) {
        edu_wm_profiler_register_metric(profiler, &metrics[i]);
    }
}
```

---

## 10. Integration with EduShell

### 10.1 D-Bus Interface

EduWM exposes its functionality via D-Bus for EduShell (and other shells) to consume:

```
Bus Name:       org.eduwm.Core
Object Path:    /org/eduwm/core
Interface:      org.eduwm.Core

Methods:
    ListWindows() → a(usss)
    GetWindow(u: id) → (ssuuuuub)
    CloseWindow(u: id) → ()
    MinimizeWindow(u: id) → ()
    MaximizeWindow(u: id) → ()
    TileWindow(u: id, s: position) → ()
    MoveWindow(u: id, i: x, i: y) → ()
    ResizeWindow(u: id, u: width, u: height) → ()
    FocusWindow(u: id) → ()

    ListWorkspaces() → a(usu)
    SwitchWorkspace(u: id) → ()
    AddWorkspace(s: name) → u
    RemoveWorkspace(u: id) → ()

    ListMonitors() → a(siiuub)
    SetPrimary(u: id) → ()
    SetScale(u: id, d: scale) → ()

    LockScreen() → ()
    Logout() → ()

Signals:
    WindowCreated(u: id, s: title, s: app_id)
    WindowDestroyed(u: id)
    WindowFocusChanged(u: id)
    WorkspaceSwitched(u: from, u: to)
    MonitorAdded(s: name)
    MonitorRemoved(s: name)
```

### 10.2 EduShell Integration

EduShell uses the D-Bus interface to:

1. **Display the taskbar**: Call `ListWindows` and listen for `WindowCreated`/`WindowDestroyed` signals.
2. **Show workspace switcher**: Call `ListWorkspaces` and `SwitchWorkspace`.
3. **Implement the overview**: Call `ListWindows` and render a grid of window thumbnails.
4. **Show monitor settings**: Call `ListMonitors` and `SetScale`.
5. **Lock the screen**: Call `LockScreen`.

---

## 11. Building and Testing Guide

### 11.1 Build Commands

```bash
# Clean build
rm -rf build && meson setup build && ninja -C build

# Incremental build
ninja -C build

# Build with specific optimization
ninja -C build -Dbuildtype=release

# Build tests only
ninja -C build eduwm-tests
```

### 11.2 Running Tests

```bash
# Run all tests
./build/test/eduwm-tests

# Run specific test suite
./build/test/eduwm-tests --suite core
./build/test/eduwm-tests --suite workspace
./build/test/eduwm-tests --suite wayland
./build/test/eduwm-tests --suite animation
./build/test/eduwm-tests --suite theme

# Run with verbose output
./build/test/eduwm-tests --verbose

# Run with address sanitizer
meson setup build -Db_sanitize=address
ninja -C build
./build/test/eduwm-tests

# Run with thread sanitizer
meson setup build -Db_sanitize=thread
ninja -C build
./build/test/eduwm-tests

# Run with memory leak detection
meson setup build -Db_sanitize=address,undefined
ninja -C build
ASAN_OPTIONS=detect_leaks=1 ./build/test/eduwm-tests
```

### 11.3 Stress Testing

```bash
# Run stress tests
./build/test/eduwm-tests --stress

# Custom stress test
./build/test/eduwm-stress \
    --windows 1000 \
    --operations 10000 \
    --duration 60s \
    --workers 4
```

### 11.4 Performance Testing

```bash
# Run performance tests
./build/test/eduwm-tests --perf

# Detailed profiling
./build/test/eduwm-tests --perf --detail

# Export profiling data
./build/test/eduwm-tests --perf --export /tmp/profile.json
```

---

## 12. Contribution Guidelines

### 12.1 Code Style

- Follow the Linux kernel coding style (with some modifications).
- Use tabs for indentation, not spaces.
- Maximum line length: 100 characters.
- Use `snake_case` for functions and variables.
- Use `UPPER_SNAKE_CASE` for constants and macros.
- Prefix all public functions with `edu_wm_`.
- Prefix all public types with `EduWm`.
- Prefix all public macros with `EDU_WM_`.

### 12.2 Commit Messages

Follow the conventional commits format:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code refactoring
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `perf`: Performance improvements
- `chore`: Maintenance tasks

Examples:
```
feat(compositor): add triple buffering support

fix(animation): prevent spring animation from overshooting

refactor(input): extract gesture recognition into separate module

docs(api): add examples to WindowCore documentation

test(wayland): add xdg-shell popup positioning tests
```

### 12.3 Pull Request Process

1. Create a feature branch from `main`.
2. Make changes and ensure all tests pass.
3. Write tests for new functionality.
4. Update documentation if needed.
5. Submit a pull request with a clear description.
6. Address review feedback.
7. Squash commits before merging.

### 12.4 Review Checklist

- [ ] Code follows the coding style guide.
- [ ] All new functions have doc comments.
- [ ] All new features have tests.
- [ ] All tests pass.
- [ ] No memory leaks (check with AddressSanitizer).
- [ ] No thread safety issues (check with ThreadSanitizer).
- [ ] Documentation is updated.
- [ ] Performance impact is acceptable.

---

## 13. Performance Guidelines

### 13.1 What to Avoid

- **Allocation in hot paths**: Never call `malloc`/`calloc` in the render loop or event processing.
- **String comparison in loops**: Pre-hash strings for O(1) lookup.
- **Excessive damage**: Only damage regions that actually changed.
- **Blocking operations**: Never block the compositor thread with I/O or synchronization.
- **Unnecessary re-compositing**: Check damage before compositing.

### 13.2 Best Practices

- **Use fixed-size arrays** where possible instead of dynamic allocations.
- **Batch operations**: Group related state changes and apply them atomically.
- **Use the animation engine**: Never implement animations manually with timers.
- **Profile first**: Use the profiling tools to identify bottlenecks before optimizing.
- **Cache computed values**: Cache window geometry calculations that don't change frequently.

### 13.3 Memory Management

```c
// GOOD: Pre-allocate and reuse
static EduWmWindow windows[MAX_WINDOWS];
static uint32_t window_count = 0;

EduWmWindowId create_window() {
    if (window_count >= MAX_WINDOWS) return 0;
    return &windows[window_count++];
}

// BAD: Allocate per window
EduWmWindowId create_window() {
    return calloc(1, sizeof(EduWmWindow));
}
```

### 13.4 Rendering Optimization

```c
// GOOD: Damage-based rendering
void render_frame(EduWmCompositor *compositor) {
    EduWmDamage damage = edu_wm_compositor_get_damage(compositor);
    if (edu_wm_damage_is_empty(&damage)) return;  // Skip if no damage

    // Only render damaged regions
    render_damaged_regions(compositor, &damage);
    edu_wm_damage_clear(&damage);
}

// BAD: Full-screen rendering every frame
void render_frame_bad(EduWmCompositor *compositor) {
    render_everything(compositor);  // Always renders everything
}
```

---

## 14. Debugging Tips

### 14.1 Using GDB

```bash
# Build with debug symbols
meson setup build -Dbuildtype=debug
ninja -C build

# Run under GDB
gdb ./build/eduwm

# Common GDB commands for EduWM
(gdb) break edu_wm_compositor_start
(gdb) break edu_wm_window_core_create_window
(gdb) thread apply all bt          # backtrace all threads
(gdb) info threads                  # list all threads
(gdb) print compositor->window_count
```

### 14.2 Using Valgrind

```bash
# Memory leak check
valgrind --leak-check=full --show-leak-kinds=all \
    ./build/eduwm --x11-display :1

# Call graph
valgrind --tool=callgrind ./build/eduwm --x11-display :1

# Cache profiling
valgrind --tool=cachegrind ./build/eduwm --x11-display :1
```

### 14.3 Using rr (Record and Replay)

```bash
# Record a session
rr record ./build/eduwm --x11-display :1

# Replay the session
rr replay

# In GDB during replay
(rr) reverse-continue    # Go backwards in time
(rr) reverse-step        # Step backwards
```

### 14.4 Using ftrace

```bash
# Trace compositor events
echo 1 > /sys/kernel/debug/tracing/events/eduwm/enable
cat /sys/kernel/debug/tracing/trace_pipe

# Or use trace-cmd
trace-cmd record -e eduwm ./build/eduwm
trace-cmd report
```

### 14.5 Common Debug Scenarios

**Scenario: Window not appearing**
```bash
# Check if window is being created
EUDUWM_LOG_LEVEL=debug ./build/eduwm 2>&1 | grep "window-created"

# Check if surface is being mapped
EUDUWM_LOG_LEVEL=debug ./build/eduwm 2>&1 | grep "surface-mapped"

# Use the inspector
./build/eduwm-inspector --list-windows
```

**Scenario: Animation lag**
```bash
# Enable FPS overlay
./build/eduwm-profiler --fps-overlay

# Check frame timing
EUDUWM_LOG_LEVEL=debug ./build/eduwm 2>&1 | grep "frame-time"

# Profile the animation
./build/eduwm-profiler --start --duration 5s --output /tmp/profile.json
```

**Scenario: Input not working**
```bash
# Check input devices
./build/eduwm-debug --list-input-devices

# Check libinput
libinput list-devices

# Trace input events
EUDUWM_LOG_LEVEL=debug ./build/eduwm 2>&1 | grep "input-event"
```

**Scenario: Multi-monitor issues**
```bash
# Check DRM outputs
./build/eduwm-debug --list-outputs

# Check monitor configuration
./build/eduwm-debug --show-monitor-config

# Force specific configuration
./build/eduwm-debug --set-output DP-1 --mode 1920x1080@60 --position 0,0
```

---

*This document is part of the EduWM v2 documentation suite.*
*Last updated: 2026-07-10*
