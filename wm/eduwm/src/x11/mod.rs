use crate::core::{WindowId, WindowType};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum X11State {
    #[default]
    Initialized,
    Running,
    Suspended,
    Stopped,
}

#[derive(Debug, Clone, PartialEq)]
pub struct X11WindowInfo {
    pub x11_window_id: u64,
    pub title: String,
    pub app_id: String,
    pub geometry: (i32, i32, i32, i32),
    pub border_width: u32,
    pub override_redirect: bool,
    pub mapped: bool,
    pub window_type: WindowType,
    pub desktop: i32,
    pub pid: u32,
    pub client_leader: u64,
    pub transient_for: Option<u64>,
}

impl X11WindowInfo {
    pub fn new(x11_window_id: u64) -> Self {
        Self {
            x11_window_id,
            title: String::new(),
            app_id: String::new(),
            geometry: (0, 0, 800, 600),
            border_width: 0,
            override_redirect: false,
            mapped: false,
            window_type: WindowType::Normal,
            desktop: 0,
            pid: 0,
            client_leader: 0,
            transient_for: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct X11Atom(u32);

impl X11Atom {
    pub const WM_PROTOCOLS: Self = Self(1);
    pub const WM_DELETE_WINDOW: Self = Self(2);
    pub const WM_STATE: Self = Self(3);
    pub const WM_NAME: Self = Self(4);
    pub const NET_WM_NAME: Self = Self(5);
    pub const NET_WM_STATE: Self = Self(6);
    pub const NET_WM_WINDOW_TYPE: Self = Self(7);
    pub const NET_WM_PID: Self = Self(8);
    pub const NET_WM_DESKTOP: Self = Self(9);
    pub const NET_ACTIVE_WINDOW: Self = Self(10);
    pub const NET_CLIENT_LIST: Self = Self(11);
    pub const NET_WM_STRUT: Self = Self(12);
    pub const MOTIF_WM_HINTS: Self = Self(13);
    pub const GTK_FRAME_EXTENTS: Self = Self(14);
    pub const NET_WM_ALLOWED_ACTIONS: Self = Self(15);
    pub const NET_WM_STATE_FULLSCREEN: Self = Self(16);
    pub const NET_WM_STATE_MAXIMIZED_VERT: Self = Self(17);
    pub const NET_WM_STATE_MAXIMIZED_HORZ: Self = Self(18);
    pub const NET_WM_STATE_HIDDEN: Self = Self(19);
    pub const NET_WM_STATE_SKIP_TASKBAR: Self = Self(20);
    pub const NET_WM_STATE_ABOVE: Self = Self(21);
    pub const NET_WM_STATE_STICKY: Self = Self(22);
    pub const NET_WM_STATE_DEMANDS_ATTENTION: Self = Self(23);

    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl From<u32> for X11Atom {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<X11Atom> for u32 {
    fn from(atom: X11Atom) -> u32 {
        atom.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum X11CompatMode {
    Native,
    #[default]
    XWayland,
    Disabled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XWaylandIntegration {
    pub enabled: bool,
    pub display: Option<u32>,
    pub xwm_socket: Option<u32>,
    pub xwm_running: bool,
}

impl Default for XWaylandIntegration {
    fn default() -> Self {
        Self {
            enabled: true,
            display: None,
            xwm_socket: None,
            xwm_running: false,
        }
    }
}

pub struct X11Compat {
    state: X11State,
    mode: X11CompatMode,
    xwayland: XWaylandIntegration,
    windows: HashMap<u64, X11WindowInfo>,
    managed_windows: Vec<u64>,
    atom_by_name: HashMap<String, X11Atom>,
    atoms_by_value: HashMap<u32, String>,
    ewmh_support: bool,
    compositing_manager: bool,
    x11_to_wm: HashMap<u64, WindowId>,
}

impl X11Compat {
    pub fn new() -> Self {
        let mut compat = Self {
            state: X11State::Initialized,
            mode: X11CompatMode::XWayland,
            xwayland: XWaylandIntegration::default(),
            windows: HashMap::new(),
            managed_windows: Vec::new(),
            atom_by_name: HashMap::new(),
            atoms_by_value: HashMap::new(),
            ewmh_support: true,
            compositing_manager: false,
            x11_to_wm: HashMap::new(),
        };
        compat.init_default_atoms();
        compat
    }

    fn init_default_atoms(&mut self) {
        let atoms = [
            ("WM_PROTOCOLS", X11Atom::WM_PROTOCOLS),
            ("WM_DELETE_WINDOW", X11Atom::WM_DELETE_WINDOW),
            ("WM_STATE", X11Atom::WM_STATE),
            ("WM_NAME", X11Atom::WM_NAME),
            ("_NET_WM_NAME", X11Atom::NET_WM_NAME),
            ("_NET_WM_STATE", X11Atom::NET_WM_STATE),
            ("_NET_WM_WINDOW_TYPE", X11Atom::NET_WM_WINDOW_TYPE),
            ("_NET_WM_PID", X11Atom::NET_WM_PID),
            ("_NET_WM_DESKTOP", X11Atom::NET_WM_DESKTOP),
            ("_NET_ACTIVE_WINDOW", X11Atom::NET_ACTIVE_WINDOW),
            ("_NET_CLIENT_LIST", X11Atom::NET_CLIENT_LIST),
            ("_NET_WM_STRUT", X11Atom::NET_WM_STRUT),
            ("_MOTIF_WM_HINTS", X11Atom::MOTIF_WM_HINTS),
            ("_GTK_FRAME_EXTENTS", X11Atom::GTK_FRAME_EXTENTS),
            ("_NET_WM_ALLOWED_ACTIONS", X11Atom::NET_WM_ALLOWED_ACTIONS),
            ("_NET_WM_STATE_FULLSCREEN", X11Atom::NET_WM_STATE_FULLSCREEN),
            (
                "_NET_WM_STATE_MAXIMIZED_VERT",
                X11Atom::NET_WM_STATE_MAXIMIZED_VERT,
            ),
            (
                "_NET_WM_STATE_MAXIMIZED_HORZ",
                X11Atom::NET_WM_STATE_MAXIMIZED_HORZ,
            ),
            ("_NET_WM_STATE_HIDDEN", X11Atom::NET_WM_STATE_HIDDEN),
            (
                "_NET_WM_STATE_SKIP_TASKBAR",
                X11Atom::NET_WM_STATE_SKIP_TASKBAR,
            ),
            ("_NET_WM_STATE_ABOVE", X11Atom::NET_WM_STATE_ABOVE),
            ("_NET_WM_STATE_STICKY", X11Atom::NET_WM_STATE_STICKY),
            (
                "_NET_WM_STATE_DEMANDS_ATTENTION",
                X11Atom::NET_WM_STATE_DEMANDS_ATTENTION,
            ),
        ];
        for (name, atom) in &atoms {
            self.atom_by_name.insert(name.to_string(), *atom);
            self.atoms_by_value.insert(atom.value(), name.to_string());
        }
    }

    pub fn start(&mut self) -> bool {
        if self.state == X11State::Stopped || self.state == X11State::Initialized {
            self.state = X11State::Running;
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self) -> bool {
        if self.state == X11State::Running {
            self.state = X11State::Stopped;
            true
        } else {
            false
        }
    }

    pub fn suspend(&mut self) -> bool {
        if self.state == X11State::Running {
            self.state = X11State::Suspended;
            true
        } else {
            false
        }
    }

    pub fn resume(&mut self) -> bool {
        if self.state == X11State::Suspended {
            self.state = X11State::Running;
            true
        } else {
            false
        }
    }

    pub fn set_mode(&mut self, mode: X11CompatMode) -> bool {
        self.mode = mode;
        true
    }

    pub fn get_mode(&self) -> &X11CompatMode {
        &self.mode
    }

    pub fn get_state(&self) -> &X11State {
        &self.state
    }

    pub fn is_running(&self) -> bool {
        self.state == X11State::Running
    }

    pub fn xwayland_start(&mut self) -> bool {
        if self.mode != X11CompatMode::XWayland {
            return false;
        }
        self.xwayland.enabled = true;
        self.xwayland.display = Some(0);
        self.xwayland.xwm_socket = Some(42);
        self.xwayland.xwm_running = true;
        true
    }

    pub fn xwayland_stop(&mut self) -> bool {
        if !self.xwayland.xwm_running {
            return false;
        }
        self.xwayland.enabled = false;
        self.xwayland.display = None;
        self.xwayland.xwm_socket = None;
        self.xwayland.xwm_running = false;
        true
    }

    pub fn manage_window(&mut self, x11_id: u64, info: X11WindowInfo) -> bool {
        if self.windows.contains_key(&x11_id) {
            return false;
        }
        self.windows.insert(x11_id, info);
        if !self.managed_windows.contains(&x11_id) {
            self.managed_windows.push(x11_id);
        }
        self.x11_to_wm.insert(x11_id, WindowId::new());
        true
    }

    pub fn unmanage_window(&mut self, x11_id: u64) -> bool {
        let removed = self.windows.remove(&x11_id).is_some();
        if removed {
            self.managed_windows.retain(|&id| id != x11_id);
            self.x11_to_wm.remove(&x11_id);
        }
        removed
    }

    pub fn get_window(&self, x11_id: u64) -> Option<&X11WindowInfo> {
        self.windows.get(&x11_id)
    }

    pub fn list_managed_windows(&self) -> &[u64] {
        &self.managed_windows
    }

    pub fn managed_count(&self) -> usize {
        self.managed_windows.len()
    }

    pub fn register_atom(&mut self, name: &str) -> X11Atom {
        if let Some(atom) = self.atom_by_name.get(name) {
            return *atom;
        }
        let next_value = self.atoms_by_value.keys().max().copied().unwrap_or(23) + 1;
        let atom = X11Atom::new(next_value);
        self.atom_by_name.insert(name.to_string(), atom);
        self.atoms_by_value.insert(next_value, name.to_string());
        atom
    }

    pub fn get_atom(&self, name: &str) -> Option<X11Atom> {
        self.atom_by_name.get(name).copied()
    }

    pub fn get_atom_name(&self, value: u32) -> Option<&str> {
        self.atoms_by_value.get(&value).map(|s| s.as_str())
    }

    pub fn send_event(&self, _x11_id: u64, _atom: X11Atom, _data: Vec<u8>) -> bool {
        true
    }

    pub fn set_ewmh_support(&mut self, enabled: bool) {
        self.ewmh_support = enabled;
    }

    pub fn set_compositing_manager(&mut self, enabled: bool) {
        self.compositing_manager = enabled;
    }

    pub fn convert_to_window_id(&self, x11_id: u64) -> Option<WindowId> {
        self.x11_to_wm.get(&x11_id).copied()
    }

    pub fn simulate_x11_event(&self, event_type: &str, x11_id: u64) -> String {
        match event_type {
            "map" => format!("Mapped window {}", x11_id),
            "unmap" => format!("Unmapped window {}", x11_id),
            "destroy" => format!("Destroyed window {}", x11_id),
            "configure" => format!("Configured window {}", x11_id),
            "property" => format!("Property changed for window {}", x11_id),
            "focus_in" => format!("Focus in on window {}", x11_id),
            "focus_out" => format!("Focus out on window {}", x11_id),
            "expose" => format!("Expose event for window {}", x11_id),
            "client_message" => format!("Client message for window {}", x11_id),
            _ => format!("Unknown event '{}' for window {}", event_type, x11_id),
        }
    }
}

impl Default for X11Compat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x11_lifecycle() {
        let mut x11 = X11Compat::new();
        assert_eq!(*x11.get_state(), X11State::Initialized);
        assert!(!x11.is_running());

        assert!(x11.start());
        assert_eq!(*x11.get_state(), X11State::Running);
        assert!(x11.is_running());

        assert!(x11.suspend());
        assert_eq!(*x11.get_state(), X11State::Suspended);

        assert!(x11.resume());
        assert_eq!(*x11.get_state(), X11State::Running);

        assert!(x11.stop());
        assert_eq!(*x11.get_state(), X11State::Stopped);

        assert!(!x11.suspend());
        assert!(!x11.resume());
        assert!(x11.start());
        assert_eq!(*x11.get_state(), X11State::Running);
    }

    #[test]
    fn test_xwayland_start_stop() {
        let mut x11 = X11Compat::new();

        assert!(x11.xwayland_start());
        assert!(x11.xwayland.xwm_running);
        assert_eq!(x11.xwayland.display, Some(0));
        assert_eq!(x11.xwayland.xwm_socket, Some(42));

        assert!(x11.xwayland_stop());
        assert!(!x11.xwayland.xwm_running);
        assert!(x11.xwayland.display.is_none());

        assert!(!x11.xwayland_stop());
    }

    #[test]
    fn test_manage_unmanage_window() {
        let mut x11 = X11Compat::new();

        let info = X11WindowInfo::new(100);
        assert!(x11.manage_window(100, info));
        assert_eq!(x11.managed_count(), 1);

        assert!(!x11.manage_window(100, X11WindowInfo::new(100)));

        let stored = x11.get_window(100).unwrap();
        assert_eq!(stored.x11_window_id, 100);

        assert!(x11.unmanage_window(100));
        assert_eq!(x11.managed_count(), 0);
        assert!(x11.get_window(100).is_none());

        assert!(!x11.unmanage_window(999));
    }

    #[test]
    fn test_atom_registry() {
        let mut x11 = X11Compat::new();

        let atom = x11.get_atom("WM_PROTOCOLS");
        assert!(atom.is_some());
        assert_eq!(atom.unwrap(), X11Atom::WM_PROTOCOLS);

        let name = x11.get_atom_name(4);
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "WM_NAME");

        let new_atom = x11.register_atom("_MY_CUSTOM_ATOM");
        assert_eq!(new_atom.value(), 24);

        let retrieved = x11.get_atom("_MY_CUSTOM_ATOM");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), new_atom);

        let atom_name = x11.get_atom_name(24);
        assert!(atom_name.is_some());
        assert_eq!(atom_name.unwrap(), "_MY_CUSTOM_ATOM");
    }

    #[test]
    fn test_mode_switching() {
        let mut x11 = X11Compat::new();
        assert_eq!(*x11.get_mode(), X11CompatMode::XWayland);

        assert!(x11.set_mode(X11CompatMode::Native));
        assert_eq!(*x11.get_mode(), X11CompatMode::Native);

        assert!(x11.set_mode(X11CompatMode::Disabled));
        assert_eq!(*x11.get_mode(), X11CompatMode::Disabled);

        let x11_disabled = X11Compat {
            mode: X11CompatMode::Disabled,
            ..X11Compat::new()
        };
        assert_eq!(*x11_disabled.get_mode(), X11CompatMode::Disabled);
    }

    #[test]
    fn test_ewmh_support() {
        let mut x11 = X11Compat::new();
        assert!(x11.ewmh_support);

        x11.set_ewmh_support(false);
        assert!(!x11.ewmh_support);

        x11.set_ewmh_support(true);
        assert!(x11.ewmh_support);

        x11.set_compositing_manager(true);
        assert!(x11.compositing_manager);

        x11.set_compositing_manager(false);
        assert!(!x11.compositing_manager);
    }

    #[test]
    fn test_send_event() {
        let x11 = X11Compat::new();
        assert!(x11.send_event(100, X11Atom::WM_DELETE_WINDOW, vec![0; 20]));
    }

    #[test]
    fn test_simulate_x11_event() {
        let x11 = X11Compat::new();
        let result = x11.simulate_x11_event("map", 42);
        assert_eq!(result, "Mapped window 42");

        let result = x11.simulate_x11_event("unknown", 10);
        assert_eq!(result, "Unknown event 'unknown' for window 10");
    }

    #[test]
    fn test_convert_window_id() {
        let mut x11 = X11Compat::new();
        x11.manage_window(200, X11WindowInfo::new(200));

        let wm_id = x11.convert_to_window_id(200);
        assert!(wm_id.is_some());

        assert!(x11.convert_to_window_id(999).is_none());

        x11.unmanage_window(200);
        assert!(x11.convert_to_window_id(200).is_none());
    }

    #[test]
    fn test_x11_window_info_defaults() {
        let info = X11WindowInfo::new(500);
        assert_eq!(info.x11_window_id, 500);
        assert_eq!(info.title, "");
        assert_eq!(info.geometry, (0, 0, 800, 600));
        assert!(!info.override_redirect);
        assert!(!info.mapped);
        assert_eq!(info.window_type, WindowType::Normal);
        assert!(info.transient_for.is_none());
    }

    #[test]
    fn test_list_managed_windows() {
        let mut x11 = X11Compat::new();
        assert!(x11.list_managed_windows().is_empty());

        x11.manage_window(10, X11WindowInfo::new(10));
        x11.manage_window(20, X11WindowInfo::new(20));
        x11.manage_window(30, X11WindowInfo::new(30));

        let list = x11.list_managed_windows();
        assert_eq!(list.len(), 3);
        assert!(list.contains(&10));
        assert!(list.contains(&20));
        assert!(list.contains(&30));
    }
}
