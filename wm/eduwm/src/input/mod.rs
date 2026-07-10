#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDeviceType {
    Keyboard,
    Pointer,
    Touchpad,
    Touchscreen,
    Tablet,
    Stylus,
    GameController,
    Accessibility,
    Switch,
    Lid,
}

#[derive(Debug, Clone)]
pub struct InputDevice {
    pub id: usize,
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_type: InputDeviceType,
    pub is_enabled: bool,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Pressed,
    Released,
    Repeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModifierMask(u32);

impl ModifierMask {
    pub const EMPTY: Self = Self(0);
    pub const SHIFT: Self = Self(1 << 0);
    pub const CAPS_LOCK: Self = Self(1 << 1);
    pub const CONTROL: Self = Self(1 << 2);
    pub const ALT: Self = Self(1 << 3);
    pub const META: Self = Self(1 << 4);
    pub const SUPER: Self = Self(1 << 5);
    pub const HYPER: Self = Self(1 << 6);
    pub const ALT_GR: Self = Self(1 << 7);

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

    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    pub fn bits(self) -> u32 {
        self.0
    }

    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
}

impl Default for ModifierMask {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::ops::BitOr for ModifierMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for ModifierMask {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for ModifierMask {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for ModifierMask {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl std::ops::BitOrAssign for ModifierMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for ModifierMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXorAssign for ModifierMask {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[derive(Debug, Clone)]
pub struct PointerEvent {
    pub device_id: usize,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub buttons: Vec<u32>,
    pub state: ButtonState,
    pub modifiers: ModifierMask,
    pub time_us: u64,
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub device_id: usize,
    pub keycode: u32,
    pub keysym: u32,
    pub state: KeyState,
    pub modifiers: ModifierMask,
    pub time_us: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TouchPhase {
    Begin,
    Update,
    End,
    Cancel,
}

#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub device_id: usize,
    pub touch_id: u32,
    pub x: f64,
    pub y: f64,
    pub phase: TouchPhase,
    pub time_us: u64,
}

#[derive(Debug, Clone)]
pub struct TabletEvent {
    pub device_id: usize,
    pub x: f64,
    pub y: f64,
    pub pressure: f64,
    pub tilt_x: f64,
    pub tilt_y: f64,
    pub rotation: f64,
    pub distance: f64,
    pub button: Option<u32>,
    pub proximity: bool,
    pub time_us: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GesturePhase {
    Begin,
    Update,
    End,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum GestureEvent {
    Swipe {
        dx: f64,
        dy: f64,
        fingers: u32,
        phase: GesturePhase,
    },
    Pinch {
        dx: f64,
        dy: f64,
        scale: f64,
        fingers: u32,
        phase: GesturePhase,
    },
    Hold {
        fingers: u32,
        phase: GesturePhase,
    },
    Tap {
        fingers: u32,
    },
    Rotate {
        angle: f64,
        fingers: u32,
        phase: GesturePhase,
    },
}

#[derive(Debug, Clone)]
pub struct ShortcutBinding {
    pub keysym: u32,
    pub modifiers: ModifierMask,
    pub action: String,
    pub description: String,
    pub is_global: bool,
    pub is_reversible: bool,
}

pub struct InputManager {
    devices: Vec<InputDevice>,
    shortcuts: Vec<ShortcutBinding>,
    next_device_id: usize,
    pointer_pos: (f64, f64),
    pointer_accel: f64,
    touchpad_scroll_speed: f64,
    touchpad_tap_to_click: bool,
    touchpad_natural_scroll: bool,
    touchpad_disable_while_typing: bool,
    keyboard_repeat_delay: u32,
    keyboard_repeat_rate: u32,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            shortcuts: Vec::new(),
            next_device_id: 0,
            pointer_pos: (0.0, 0.0),
            pointer_accel: 1.0,
            touchpad_scroll_speed: 1.0,
            touchpad_tap_to_click: true,
            touchpad_natural_scroll: false,
            touchpad_disable_while_typing: true,
            keyboard_repeat_delay: 250,
            keyboard_repeat_rate: 25,
        }
    }

    pub fn register_device(&mut self, mut device: InputDevice) -> usize {
        let id = self.next_device_id;
        self.next_device_id += 1;
        device.id = id;
        self.devices.push(device);
        id
    }

    pub fn remove_device(&mut self, id: usize) -> bool {
        let pos = self.devices.iter().position(|d| d.id == id);
        if let Some(pos) = pos {
            self.devices.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_device(&self, id: usize) -> Option<&InputDevice> {
        self.devices.iter().find(|d| d.id == id)
    }

    pub fn list_devices(&self) -> &[InputDevice] {
        &self.devices
    }

    pub fn enable_device(&mut self, id: usize) -> bool {
        if let Some(d) = self.devices.iter_mut().find(|d| d.id == id) {
            d.is_enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_device(&mut self, id: usize) -> bool {
        if let Some(d) = self.devices.iter_mut().find(|d| d.id == id) {
            d.is_enabled = false;
            true
        } else {
            false
        }
    }

    pub fn set_pointer_position(&mut self, x: f64, y: f64) {
        self.pointer_pos = (x, y);
    }

    pub fn get_pointer_position(&self) -> (f64, f64) {
        self.pointer_pos
    }

    pub fn set_pointer_accel(&mut self, accel: f64) {
        self.pointer_accel = accel;
    }

    pub fn get_pointer_accel(&self) -> f64 {
        self.pointer_accel
    }

    pub fn set_touchpad_settings(
        &mut self,
        tap_to_click: bool,
        natural_scroll: bool,
        disable_while_typing: bool,
    ) {
        self.touchpad_tap_to_click = tap_to_click;
        self.touchpad_natural_scroll = natural_scroll;
        self.touchpad_disable_while_typing = disable_while_typing;
    }

    pub fn get_touchpad_scroll_speed(&self) -> f64 {
        self.touchpad_scroll_speed
    }

    pub fn set_touchpad_scroll_speed(&mut self, speed: f64) {
        self.touchpad_scroll_speed = speed;
    }

    pub fn register_shortcut(&mut self, binding: ShortcutBinding) -> usize {
        let id = self.shortcuts.len();
        self.shortcuts.push(binding);
        id
    }

    pub fn unregister_shortcut(&mut self, id: usize) -> bool {
        if id < self.shortcuts.len() {
            self.shortcuts.remove(id);
            true
        } else {
            false
        }
    }

    pub fn find_shortcut(&self, keysym: u32, modifiers: ModifierMask) -> Option<&ShortcutBinding> {
        self.shortcuts
            .iter()
            .find(|s| s.keysym == keysym && s.modifiers == modifiers)
    }

    pub fn list_shortcuts(&self) -> &[ShortcutBinding] {
        &self.shortcuts
    }

    pub fn handle_keyboard_event(&self, event: &KeyboardEvent) -> Option<String> {
        if event.state != KeyState::Pressed {
            return None;
        }
        self.shortcuts
            .iter()
            .find(|s| s.keysym == event.keysym && s.modifiers == event.modifiers)
            .map(|s| s.action.clone())
    }

    pub fn set_keyboard_repeat(&mut self, delay: u32, rate: u32) {
        self.keyboard_repeat_delay = delay;
        self.keyboard_repeat_rate = rate;
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_keyboard_device() -> InputDevice {
        InputDevice {
            id: 0,
            name: "Test Keyboard".to_string(),
            vendor_id: 0x1234,
            product_id: 0x5678,
            device_type: InputDeviceType::Keyboard,
            is_enabled: true,
            capabilities: vec!["keyboard".to_string()],
        }
    }

    fn make_pointer_device() -> InputDevice {
        InputDevice {
            id: 0,
            name: "Test Mouse".to_string(),
            vendor_id: 0x1234,
            product_id: 0x5679,
            device_type: InputDeviceType::Pointer,
            is_enabled: true,
            capabilities: vec!["pointer".to_string()],
        }
    }

    #[test]
    fn test_device_register_remove() {
        let mut mgr = InputManager::new();
        let id1 = mgr.register_device(make_keyboard_device());
        let id2 = mgr.register_device(make_pointer_device());
        assert!(mgr.get_device(id1).is_some());
        assert!(mgr.get_device(id2).is_some());
        assert_eq!(mgr.list_devices().len(), 2);
        assert!(mgr.remove_device(id1));
        assert_eq!(mgr.list_devices().len(), 1);
    }

    #[test]
    fn test_pointer_position() {
        let mut mgr = InputManager::new();
        mgr.set_pointer_position(100.0, 200.0);
        assert_eq!(mgr.get_pointer_position(), (100.0, 200.0));
    }

    #[test]
    fn test_pointer_accel() {
        let mut mgr = InputManager::new();
        assert_eq!(mgr.get_pointer_accel(), 1.0);
        mgr.set_pointer_accel(2.5);
        assert_eq!(mgr.get_pointer_accel(), 2.5);
    }

    #[test]
    fn test_touchpad_settings() {
        let mut mgr = InputManager::new();
        mgr.set_touchpad_settings(false, true, false);
        let devices = mgr.list_devices();
        assert_eq!(devices.len(), 0);
        assert!(!mgr.touchpad_tap_to_click);
        assert!(mgr.touchpad_natural_scroll);
        assert!(!mgr.touchpad_disable_while_typing);
    }

    #[test]
    fn test_shortcut_register_find() {
        let mut mgr = InputManager::new();
        let binding = ShortcutBinding {
            keysym: 113,
            modifiers: ModifierMask::CONTROL | ModifierMask::ALT,
            action: "toggle_overview".to_string(),
            description: "Toggle overview".to_string(),
            is_global: true,
            is_reversible: true,
        };
        let id = mgr.register_shortcut(binding);
        let found = mgr.find_shortcut(113, ModifierMask::CONTROL | ModifierMask::ALT);
        assert!(found.is_some());
        assert_eq!(found.unwrap().action, "toggle_overview");
        assert!(mgr.unregister_shortcut(id));
        assert!(mgr
            .find_shortcut(113, ModifierMask::CONTROL | ModifierMask::ALT)
            .is_none());
    }

    #[test]
    fn test_shortcut_match() {
        let mut mgr = InputManager::new();
        mgr.register_shortcut(ShortcutBinding {
            keysym: 113,
            modifiers: ModifierMask::CONTROL | ModifierMask::ALT,
            action: "toggle_overview".to_string(),
            description: "Toggle overview".to_string(),
            is_global: true,
            is_reversible: true,
        });
        let found = mgr.find_shortcut(113, ModifierMask::CONTROL | ModifierMask::ALT);
        assert!(found.is_some());
        let not_found = mgr.find_shortcut(113, ModifierMask::empty());
        assert!(not_found.is_none());
    }

    #[test]
    fn test_keyboard_event_shortcut() {
        let mut mgr = InputManager::new();
        mgr.register_shortcut(ShortcutBinding {
            keysym: 32,
            modifiers: ModifierMask::SUPER,
            action: "launch_terminal".to_string(),
            description: "Launch terminal".to_string(),
            is_global: true,
            is_reversible: false,
        });
        let event = KeyboardEvent {
            device_id: 0,
            keycode: 65,
            keysym: 32,
            state: KeyState::Pressed,
            modifiers: ModifierMask::SUPER,
            time_us: 1000,
        };
        let action = mgr.handle_keyboard_event(&event);
        assert_eq!(action, Some("launch_terminal".to_string()));
        let event_no_match = KeyboardEvent {
            device_id: 0,
            keycode: 65,
            keysym: 32,
            state: KeyState::Pressed,
            modifiers: ModifierMask::empty(),
            time_us: 1000,
        };
        assert!(mgr.handle_keyboard_event(&event_no_match).is_none());
    }
}
