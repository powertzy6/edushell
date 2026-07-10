use crate::monitor::MonitorTransform;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WlSurfaceRole {
    #[default]
    None,
    ShellSurface,
    XdgSurface,
    LayerSurface,
    Subsurface,
    Popup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WlSurfaceState {
    #[default]
    Initial,
    Configured,
    Mapped,
    Unmapped,
    Destroyed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WlSurface {
    pub id: usize,
    pub role: WlSurfaceRole,
    pub state: WlSurfaceState,
    pub width: i32,
    pub height: i32,
    pub scale: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub opaque_region: Vec<(i32, i32, i32, i32)>,
    pub input_region: Vec<(i32, i32, i32, i32)>,
    pub buffer_scale: i32,
    pub buffer_transform: MonitorTransform,
}

impl Default for WlSurface {
    fn default() -> Self {
        Self {
            id: 0,
            role: WlSurfaceRole::None,
            state: WlSurfaceState::Initial,
            width: 0,
            height: 0,
            scale: 1,
            offset_x: 0,
            offset_y: 0,
            opaque_region: Vec::new(),
            input_region: Vec::new(),
            buffer_scale: 1,
            buffer_transform: MonitorTransform::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XdgSurfaceRole {
    Toplevel,
    Popup,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct XdgToplevelState {
    pub title: String,
    pub app_id: String,
    pub min_size: (i32, i32),
    pub max_size: (i32, i32),
    pub maximized: bool,
    pub fullscreen: bool,
    pub resizing: bool,
    pub activated: bool,
    pub tiled_edges: u8,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct XdgPopupState {
    pub parent: Option<usize>,
    pub x: i32,
    pub y: i32,
    pub grab: bool,
    pub anchor: u8,
    pub gravity: u8,
    pub constraint_adjustment: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LayerShellLayer {
    Background,
    #[default]
    Bottom,
    Top,
    Overlay,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayerSurfaceState {
    pub anchor: u8,
    pub exclusive_zone: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub keyboard_interactivity: bool,
    pub layer: LayerShellLayer,
    pub namespace: String,
}

impl Default for LayerSurfaceState {
    fn default() -> Self {
        Self {
            anchor: 0,
            exclusive_zone: 0,
            margin_top: 0,
            margin_bottom: 0,
            margin_left: 0,
            margin_right: 0,
            keyboard_interactivity: false,
            layer: LayerShellLayer::Bottom,
            namespace: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WlSeatCapability(u32);

impl WlSeatCapability {
    pub const EMPTY: Self = Self(0);
    pub const POINTER: Self = Self(1);
    pub const KEYBOARD: Self = Self(2);
    pub const TOUCH: Self = Self(4);

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    pub fn bits(self) -> u32 {
        self.0
    }

    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
}

impl Default for WlSeatCapability {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::ops::BitOr for WlSeatCapability {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for WlSeatCapability {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for WlSeatCapability {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for WlSeatCapability {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WlPointerState {
    pub focused_surface: Option<usize>,
    pub cursor_surface: Option<usize>,
    pub cursor_serial: u32,
    pub cursor_hotspot_x: i32,
    pub cursor_hotspot_y: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WlKeyboardState {
    pub focused_surface: Option<usize>,
    pub pressed_keys: Vec<u32>,
    pub modifiers: u32,
    pub mods_latched: u32,
    pub mods_locked: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WlTouchState {
    pub focus_surface: Option<usize>,
    pub touch_points: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractionalScaleInfo {
    pub scale: f64,
    pub preferred_scale: f64,
}

impl Default for FractionalScaleInfo {
    fn default() -> Self {
        Self {
            scale: 1.0,
            preferred_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum XdgDecorationMode {
    #[default]
    None,
    ClientSide,
    ServerSide,
}

pub struct WaylandCompat {
    surfaces: HashMap<usize, WlSurface>,
    next_surface_id: usize,
    xdg_toplevels: HashMap<usize, XdgToplevelState>,
    xdg_popups: HashMap<usize, XdgPopupState>,
    layer_surfaces: HashMap<usize, LayerSurfaceState>,
    seat_state: (WlPointerState, WlKeyboardState, WlTouchState),
    decoration_mode: XdgDecorationMode,
    supported_protocols: Vec<String>,
    fractional_scale: FractionalScaleInfo,
    #[allow(dead_code)]
    compositor_version: u32,
    configure_serial: u32,
}

impl WaylandCompat {
    pub fn new() -> Self {
        Self {
            surfaces: HashMap::new(),
            next_surface_id: 1,
            xdg_toplevels: HashMap::new(),
            xdg_popups: HashMap::new(),
            layer_surfaces: HashMap::new(),
            seat_state: (
                WlPointerState::default(),
                WlKeyboardState::default(),
                WlTouchState::default(),
            ),
            decoration_mode: XdgDecorationMode::None,
            supported_protocols: vec![
                "xdg_shell".to_string(),
                "layer_shell".to_string(),
                "xdg_decoration".to_string(),
                "fractional_scale".to_string(),
                "viewporter".to_string(),
                "pointer_constraints".to_string(),
                "relative_pointer".to_string(),
                "presentation_time".to_string(),
                "idle_inhibit".to_string(),
                "session_lock".to_string(),
                "screencast".to_string(),
                "cursor_shape".to_string(),
            ],
            fractional_scale: FractionalScaleInfo::default(),
            compositor_version: 6,
            configure_serial: 0,
        }
    }

    pub fn create_surface(&mut self, role: WlSurfaceRole) -> usize {
        let id = self.next_surface_id;
        self.next_surface_id += 1;
        self.surfaces.insert(
            id,
            WlSurface {
                id,
                role,
                state: WlSurfaceState::Initial,
                ..Default::default()
            },
        );
        id
    }

    pub fn destroy_surface(&mut self, id: usize) -> bool {
        let removed = self.surfaces.remove(&id).is_some();
        self.xdg_toplevels.remove(&id);
        self.xdg_popups.remove(&id);
        self.layer_surfaces.remove(&id);
        let pointer = &mut self.seat_state.0;
        if pointer.focused_surface == Some(id) {
            pointer.focused_surface = None;
        }
        if pointer.cursor_surface == Some(id) {
            pointer.cursor_surface = None;
        }
        let keyboard = &mut self.seat_state.1;
        if keyboard.focused_surface == Some(id) {
            keyboard.focused_surface = None;
        }
        let touch = &mut self.seat_state.2;
        if touch.focus_surface == Some(id) {
            touch.focus_surface = None;
        }
        removed
    }

    pub fn get_surface(&self, id: usize) -> Option<&WlSurface> {
        self.surfaces.get(&id)
    }

    pub fn get_surface_mut(&mut self, id: usize) -> Option<&mut WlSurface> {
        self.surfaces.get_mut(&id)
    }

    pub fn surface_count(&self) -> usize {
        self.surfaces.len()
    }

    pub fn set_xdg_toplevel(&mut self, surface_id: usize, toplevel: XdgToplevelState) -> bool {
        if self.surfaces.contains_key(&surface_id) {
            self.xdg_toplevels.insert(surface_id, toplevel);
            true
        } else {
            false
        }
    }

    pub fn get_xdg_toplevel(&self, surface_id: usize) -> Option<&XdgToplevelState> {
        self.xdg_toplevels.get(&surface_id)
    }

    pub fn set_xdg_popup(&mut self, surface_id: usize, popup: XdgPopupState) -> bool {
        if self.surfaces.contains_key(&surface_id) {
            self.xdg_popups.insert(surface_id, popup);
            true
        } else {
            false
        }
    }

    pub fn get_xdg_popup(&self, surface_id: usize) -> Option<&XdgPopupState> {
        self.xdg_popups.get(&surface_id)
    }

    pub fn set_layer_surface(&mut self, surface_id: usize, layer: LayerSurfaceState) -> bool {
        if self.surfaces.contains_key(&surface_id) {
            self.layer_surfaces.insert(surface_id, layer);
            true
        } else {
            false
        }
    }

    pub fn get_layer_surface(&self, surface_id: usize) -> Option<&LayerSurfaceState> {
        self.layer_surfaces.get(&surface_id)
    }

    pub fn configure_toplevel(&mut self, surface_id: usize, width: i32, height: i32) -> Vec<u8> {
        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            surface.width = width;
            surface.height = height;
        }
        if let Some(toplevel) = self.xdg_toplevels.get_mut(&surface_id) {
            if width > 0 {
                if toplevel.max_size.0 > 0 {
                    toplevel.min_size.0 = toplevel.min_size.0.min(width);
                }
                if toplevel.max_size.0 > 0 {
                    toplevel.max_size.0 = toplevel.max_size.0.max(width);
                }
            }
            if height > 0 {
                if toplevel.max_size.1 > 0 {
                    toplevel.min_size.1 = toplevel.min_size.1.min(height);
                }
                if toplevel.max_size.1 > 0 {
                    toplevel.max_size.1 = toplevel.max_size.1.max(height);
                }
            }
        }
        self.configure_serial += 1;
        self.configure_serial.to_ne_bytes().to_vec()
    }

    pub fn ack_configure(&self, _serial: Vec<u8>) {}

    pub fn set_fractional_scale(&mut self, surface_id: usize, scale: f64) -> bool {
        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            surface.scale = scale as i32;
            true
        } else {
            false
        }
    }

    pub fn set_decoration_mode(&mut self, mode: XdgDecorationMode) {
        self.decoration_mode = mode;
    }

    pub fn get_decoration_mode(&self) -> &XdgDecorationMode {
        &self.decoration_mode
    }

    pub fn get_fractional_scale_info(&self) -> &FractionalScaleInfo {
        &self.fractional_scale
    }

    pub fn supports_protocol(&self, name: &str) -> bool {
        self.supported_protocols.iter().any(|p| p == name)
    }

    pub fn list_protocols(&self) -> &[String] {
        &self.supported_protocols
    }

    pub fn simulate_configure(&mut self, surface_id: usize) {
        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            surface.state = WlSurfaceState::Configured;
        }
    }
}

impl Default for WaylandCompat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_destroy_surface() {
        let mut wl = WaylandCompat::new();
        assert_eq!(wl.surface_count(), 0);

        let id = wl.create_surface(WlSurfaceRole::XdgSurface);
        assert_eq!(wl.surface_count(), 1);
        assert!(wl.get_surface(id).is_some());

        let surface = wl.get_surface(id).unwrap();
        assert_eq!(surface.role, WlSurfaceRole::XdgSurface);
        assert_eq!(surface.state, WlSurfaceState::Initial);

        assert!(wl.destroy_surface(id));
        assert_eq!(wl.surface_count(), 0);
        assert!(wl.get_surface(id).is_none());

        assert!(!wl.destroy_surface(999));
    }

    #[test]
    fn test_xdg_toplevel() {
        let mut wl = WaylandCompat::new();
        let id = wl.create_surface(WlSurfaceRole::XdgSurface);

        let toplevel = XdgToplevelState {
            title: "Test Window".to_string(),
            app_id: "test.app".to_string(),
            min_size: (400, 300),
            max_size: (1920, 1080),
            ..Default::default()
        };

        assert!(wl.set_xdg_toplevel(id, toplevel));
        let stored = wl.get_xdg_toplevel(id).unwrap();
        assert_eq!(stored.title, "Test Window");
        assert_eq!(stored.app_id, "test.app");
        assert_eq!(stored.min_size, (400, 300));

        assert!(!wl.set_xdg_toplevel(999, XdgToplevelState::default()));
        assert!(wl.get_xdg_toplevel(999).is_none());
    }

    #[test]
    fn test_xdg_popup() {
        let mut wl = WaylandCompat::new();
        let parent = wl.create_surface(WlSurfaceRole::XdgSurface);
        let popup_id = wl.create_surface(WlSurfaceRole::Popup);

        let popup = XdgPopupState {
            parent: Some(parent),
            x: 100,
            y: 200,
            grab: true,
            anchor: 0x04,
            gravity: 0x08,
            constraint_adjustment: 0x03,
        };

        assert!(wl.set_xdg_popup(popup_id, popup));
        let stored = wl.get_xdg_popup(popup_id).unwrap();
        assert_eq!(stored.parent, Some(parent));
        assert_eq!(stored.x, 100);
        assert_eq!(stored.y, 200);
        assert!(stored.grab);
    }

    #[test]
    fn test_layer_surface() {
        let mut wl = WaylandCompat::new();
        let id = wl.create_surface(WlSurfaceRole::LayerSurface);

        let layer = LayerSurfaceState {
            anchor: 0x0F,
            exclusive_zone: 32,
            margin_top: 10,
            margin_bottom: 10,
            margin_left: 10,
            margin_right: 10,
            keyboard_interactivity: true,
            layer: LayerShellLayer::Top,
            namespace: "dock".to_string(),
        };

        assert!(wl.set_layer_surface(id, layer));
        let stored = wl.get_layer_surface(id).unwrap();
        assert_eq!(stored.anchor, 0x0F);
        assert_eq!(stored.exclusive_zone, 32);
        assert_eq!(stored.layer, LayerShellLayer::Top);
        assert_eq!(stored.namespace, "dock");
        assert!(stored.keyboard_interactivity);
    }

    #[test]
    fn test_fractional_scale() {
        let mut wl = WaylandCompat::new();
        let id = wl.create_surface(WlSurfaceRole::XdgSurface);

        assert!(wl.set_fractional_scale(id, 1.5));
        let surface = wl.get_surface(id).unwrap();
        assert_eq!(surface.scale, 1);

        let info = wl.get_fractional_scale_info();
        assert_eq!(info.scale, 1.0);
        assert_eq!(info.preferred_scale, 1.0);
    }

    #[test]
    fn test_decoration_mode() {
        let mut wl = WaylandCompat::new();
        assert_eq!(*wl.get_decoration_mode(), XdgDecorationMode::None);

        wl.set_decoration_mode(XdgDecorationMode::ServerSide);
        assert_eq!(*wl.get_decoration_mode(), XdgDecorationMode::ServerSide);

        wl.set_decoration_mode(XdgDecorationMode::ClientSide);
        assert_eq!(*wl.get_decoration_mode(), XdgDecorationMode::ClientSide);
    }

    #[test]
    fn test_protocol_support() {
        let wl = WaylandCompat::new();
        assert!(wl.supports_protocol("xdg_shell"));
        assert!(wl.supports_protocol("layer_shell"));
        assert!(wl.supports_protocol("xdg_decoration"));
        assert!(wl.supports_protocol("fractional_scale"));
        assert!(wl.supports_protocol("viewporter"));
        assert!(!wl.supports_protocol("nonexistent_protocol"));

        let protocols = wl.list_protocols();
        assert!(protocols.len() >= 12);
    }

    #[test]
    fn test_configure_toplevel() {
        let mut wl = WaylandCompat::new();
        let id = wl.create_surface(WlSurfaceRole::XdgSurface);

        wl.set_xdg_toplevel(id, XdgToplevelState::default());
        let serial = wl.configure_toplevel(id, 1024, 768);

        assert!(!serial.is_empty());
        let surface = wl.get_surface(id).unwrap();
        assert_eq!(surface.width, 1024);
        assert_eq!(surface.height, 768);

        let serial2 = wl.configure_toplevel(id, 800, 600);
        assert_ne!(serial, serial2);

        wl.ack_configure(serial2);
    }

    #[test]
    fn test_simulate_configure() {
        let mut wl = WaylandCompat::new();
        let id = wl.create_surface(WlSurfaceRole::XdgSurface);
        assert_eq!(wl.get_surface(id).unwrap().state, WlSurfaceState::Initial);

        wl.simulate_configure(id);
        assert_eq!(
            wl.get_surface(id).unwrap().state,
            WlSurfaceState::Configured
        );
    }

    #[test]
    fn test_surface_cleanup_on_destroy() {
        let mut wl = WaylandCompat::new();
        let id = wl.create_surface(WlSurfaceRole::XdgSurface);

        wl.set_xdg_toplevel(id, XdgToplevelState::default());
        assert!(wl.get_xdg_toplevel(id).is_some());

        wl.destroy_surface(id);
        assert!(wl.get_xdg_toplevel(id).is_none());
    }

    #[test]
    fn test_seat_state_defaults() {
        let wl = WaylandCompat::new();
        let (ref pointer, ref keyboard, ref touch) = wl.seat_state;

        assert!(pointer.focused_surface.is_none());
        assert!(pointer.cursor_surface.is_none());
        assert_eq!(pointer.cursor_serial, 0);

        assert!(keyboard.focused_surface.is_none());
        assert!(keyboard.pressed_keys.is_empty());

        assert!(touch.focus_surface.is_none());
        assert!(touch.touch_points.is_empty());
    }
}
