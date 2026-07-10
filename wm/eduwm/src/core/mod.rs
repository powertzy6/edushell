use std::collections::HashMap;
use std::str::FromStr;

pub fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(uuid::Uuid);

impl WindowId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for WindowId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(s)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowState {
    Created,
    Mapping,
    Mapped,
    Unmapped,
    Destroyed,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Normal,
    Dialog,
    Modal,
    Utility,
    Dock,
    Splash,
    DropdownMenu,
    PopupMenu,
    Tooltip,
    Notification,
    Desktop,
    OverrideRedirect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowFlags(u32);

impl WindowFlags {
    pub const EMPTY: Self = Self(0);
    pub const DECORATED: Self = Self(1 << 0);
    pub const RESIZABLE: Self = Self(1 << 1);
    pub const MINIMIZABLE: Self = Self(1 << 2);
    pub const MAXIMIZABLE: Self = Self(1 << 3);
    pub const CLOSEABLE: Self = Self(1 << 4);
    pub const SKIP_TASKBAR: Self = Self(1 << 5);
    pub const SKIP_PAGER: Self = Self(1 << 6);
    pub const ABOVE: Self = Self(1 << 7);
    pub const BELOW: Self = Self(1 << 8);
    pub const STICKY: Self = Self(1 << 9);
    pub const MODAL: Self = Self(1 << 10);
    pub const FULLSCREEN: Self = Self(1 << 11);
    pub const ALWAYS_ON_TOP: Self = Self(1 << 12);
    pub const ALWAYS_ON_VISIBLE_WORKSPACE: Self = Self(1 << 13);
    pub const DEMANDS_ATTENTION: Self = Self(1 << 14);
    pub const SHADOW: Self = Self(1 << 15);

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

    pub fn is_decorated(self) -> bool {
        self.contains(Self::DECORATED)
    }

    pub fn is_resizable(self) -> bool {
        self.contains(Self::RESIZABLE)
    }

    pub fn is_minimizable(self) -> bool {
        self.contains(Self::MINIMIZABLE)
    }

    pub fn is_maximizable(self) -> bool {
        self.contains(Self::MAXIMIZABLE)
    }

    pub fn is_closeable(self) -> bool {
        self.contains(Self::CLOSEABLE)
    }

    pub fn is_skip_taskbar(self) -> bool {
        self.contains(Self::SKIP_TASKBAR)
    }

    pub fn is_skip_pager(self) -> bool {
        self.contains(Self::SKIP_PAGER)
    }

    pub fn is_above(self) -> bool {
        self.contains(Self::ABOVE)
    }

    pub fn is_below(self) -> bool {
        self.contains(Self::BELOW)
    }

    pub fn is_sticky(self) -> bool {
        self.contains(Self::STICKY)
    }

    pub fn is_modal(self) -> bool {
        self.contains(Self::MODAL)
    }

    pub fn is_fullscreen(self) -> bool {
        self.contains(Self::FULLSCREEN)
    }

    pub fn is_always_on_top(self) -> bool {
        self.contains(Self::ALWAYS_ON_TOP)
    }

    pub fn is_always_on_visible_workspace(self) -> bool {
        self.contains(Self::ALWAYS_ON_VISIBLE_WORKSPACE)
    }

    pub fn is_demands_attention(self) -> bool {
        self.contains(Self::DEMANDS_ATTENTION)
    }

    pub fn is_shadow(self) -> bool {
        self.contains(Self::SHADOW)
    }

    pub fn bits(self) -> u32 {
        self.0
    }

    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
}

impl Default for WindowFlags {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::ops::BitOr for WindowFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for WindowFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for WindowFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for WindowFlags {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl std::ops::BitOrAssign for WindowFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for WindowFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXorAssign for WindowFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub state: WindowState,
    pub type_: WindowType,
    pub flags: WindowFlags,
    pub geometry: Rect,
    pub workspace: usize,
    pub monitor: usize,
    pub pid: u32,
    pub client_machine: String,
    pub created_at: String,
}

pub struct WindowCore {
    windows: HashMap<WindowId, WindowInfo>,
}

impl WindowCore {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn create_window(&mut self, title: &str, app_id: &str, type_: WindowType) -> WindowId {
        let id = WindowId::new();
        let info = WindowInfo {
            id,
            title: title.to_string(),
            app_id: app_id.to_string(),
            state: WindowState::Created,
            type_,
            flags: WindowFlags::empty(),
            geometry: Rect {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            },
            workspace: 0,
            monitor: 0,
            pid: 0,
            client_machine: String::new(),
            created_at: now(),
        };
        self.windows.insert(id, info);
        id
    }

    pub fn destroy_window(&mut self, id: WindowId) -> bool {
        self.windows.remove(&id).is_some()
    }

    pub fn update_window(&mut self, id: WindowId, info: WindowInfo) -> bool {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.windows.entry(id) {
            e.insert(info);
            true
        } else {
            false
        }
    }

    pub fn get_window(&self, id: WindowId) -> Option<&WindowInfo> {
        self.windows.get(&id)
    }

    pub fn list_windows(&self) -> &HashMap<WindowId, WindowInfo> {
        &self.windows
    }

    pub fn count(&self) -> usize {
        self.windows.len()
    }

    pub fn windows_by_workspace(&self, workspace: usize) -> Vec<WindowId> {
        self.windows
            .values()
            .filter(|w| w.workspace == workspace)
            .map(|w| w.id)
            .collect()
    }

    pub fn windows_by_state(&self, state: WindowState) -> Vec<WindowId> {
        self.windows
            .values()
            .filter(|w| w.state == state)
            .map(|w| w.id)
            .collect()
    }

    pub fn windows_by_type(&self, type_: WindowType) -> Vec<WindowId> {
        self.windows
            .values()
            .filter(|w| w.type_ == type_)
            .map(|w| w.id)
            .collect()
    }
}

impl Default for WindowCore {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FocusManager {
    active_window: Option<WindowId>,
    focus_stack: Vec<WindowId>,
    recent_windows: Vec<WindowId>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            active_window: None,
            focus_stack: Vec::new(),
            recent_windows: Vec::new(),
        }
    }

    pub fn focus(&mut self, id: WindowId) {
        if let Some(prev) = self.active_window {
            if !self.recent_windows.contains(&prev) {
                self.recent_windows.push(prev);
            }
        }
        self.focus_stack.retain(|&w| w != id);
        self.focus_stack.push(id);
        self.active_window = Some(id);
    }

    pub fn unfocus(&mut self) -> Option<WindowId> {
        let removed = self.focus_stack.pop();
        self.active_window = self.focus_stack.last().copied();
        removed
    }

    pub fn get_active(&self) -> Option<WindowId> {
        self.active_window
    }

    pub fn cycle_focus(&mut self, windows: &[WindowId]) -> Option<WindowId> {
        if windows.is_empty() {
            return None;
        }

        let next = match self.active_window {
            Some(current) => {
                if let Some(pos) = windows.iter().position(|&w| w == current) {
                    windows[(pos + 1) % windows.len()]
                } else {
                    windows[0]
                }
            }
            None => windows[0],
        };

        self.focus(next);
        Some(next)
    }

    pub fn get_focus_stack(&self) -> &Vec<WindowId> {
        &self.focus_stack
    }

    pub fn get_recent(&self) -> &Vec<WindowId> {
        &self.recent_windows
    }

    pub fn len(&self) -> usize {
        self.focus_stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.focus_stack.is_empty()
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StackingManager {
    stacking_order: Vec<WindowId>,
    lower: Vec<WindowId>,
    above_layer: Vec<WindowId>,
}

impl StackingManager {
    pub fn new() -> Self {
        Self {
            stacking_order: Vec::new(),
            lower: Vec::new(),
            above_layer: Vec::new(),
        }
    }

    pub fn raise(&mut self, id: WindowId) {
        self.stacking_order.retain(|&w| w != id);
        self.stacking_order.push(id);
    }

    pub fn lower(&mut self, id: WindowId) {
        self.stacking_order.retain(|&w| w != id);
        self.stacking_order.insert(0, id);
    }

    pub fn stack_above(&mut self, id: WindowId, target: WindowId) {
        self.stacking_order.retain(|&w| w != id);
        if let Some(pos) = self.stacking_order.iter().position(|&w| w == target) {
            self.stacking_order.insert(pos + 1, id);
        } else {
            self.stacking_order.push(id);
        }
    }

    pub fn stack_below(&mut self, id: WindowId, target: WindowId) {
        self.stacking_order.retain(|&w| w != id);
        if let Some(pos) = self.stacking_order.iter().position(|&w| w == target) {
            self.stacking_order.insert(pos, id);
        } else {
            self.stacking_order.insert(0, id);
        }
    }

    pub fn restack(&mut self, order: Vec<WindowId>) {
        self.stacking_order = order;
    }

    pub fn get_stacking_order(&self) -> &Vec<WindowId> {
        &self.stacking_order
    }

    pub fn get_lower(&self) -> &Vec<WindowId> {
        &self.lower
    }

    pub fn get_above(&self) -> &Vec<WindowId> {
        &self.above_layer
    }

    pub fn ensure_lower(&mut self) {
        self.stacking_order.retain(|id| !self.lower.contains(id));
        let mut new_order = self.lower.clone();
        new_order.append(&mut self.stacking_order);
        self.stacking_order = new_order;
    }
}

impl Default for StackingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct WindowRule {
    pub match_app_id: Option<String>,
    pub match_title: Option<String>,
    pub match_type: Option<WindowType>,
    pub rule_workspace: Option<usize>,
    pub rule_monitor: Option<usize>,
    pub rule_state: Option<WindowState>,
    pub rule_flags: Option<WindowFlags>,
    pub rule_position: Option<(i32, i32)>,
    pub rule_size: Option<(u32, u32)>,
    pub rule_always_on_top: Option<bool>,
    pub rule_always_on_visible_workspace: Option<bool>,
    pub rule_skip_taskbar: Option<bool>,
    pub rule_sticky: Option<bool>,
    pub rule_animation: Option<bool>,
}

impl WindowRule {
    fn matches(&self, info: &WindowInfo) -> bool {
        if let Some(ref app_id) = self.match_app_id {
            if &info.app_id != app_id {
                return false;
            }
        }
        if let Some(ref title) = self.match_title {
            if &info.title != title {
                return false;
            }
        }
        if let Some(type_) = self.match_type {
            if info.type_ != type_ {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct AppliedRule {
    pub window_id: WindowId,
    pub rule_index: usize,
    pub changes: Vec<String>,
}

pub struct WindowRulesEngine {
    rules: Vec<WindowRule>,
}

impl WindowRulesEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: WindowRule) -> usize {
        let idx = self.rules.len();
        self.rules.push(rule);
        idx
    }

    pub fn remove_rule(&mut self, index: usize) -> bool {
        if index < self.rules.len() {
            self.rules.remove(index);
            true
        } else {
            false
        }
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    pub fn list_rules(&self) -> &Vec<WindowRule> {
        &self.rules
    }

    pub fn match_window(&self, info: &WindowInfo) -> Option<&WindowRule> {
        self.rules.iter().find(|rule| rule.matches(info))
    }

    pub fn apply_rules(&self, info: &WindowInfo) -> Vec<AppliedRule> {
        let mut applied = Vec::new();
        for (idx, rule) in self.rules.iter().enumerate() {
            if !rule.matches(info) {
                continue;
            }
            let mut changes = Vec::new();
            if let Some(ws) = rule.rule_workspace {
                changes.push(format!("set workspace to {}", ws));
            }
            if let Some(m) = rule.rule_monitor {
                changes.push(format!("set monitor to {}", m));
            }
            if let Some(state) = rule.rule_state {
                changes.push(format!("set state to {:?}", state));
            }
            if let Some(flags) = rule.rule_flags {
                if !flags.is_empty() {
                    changes.push(format!("set flags to {:?}", flags));
                }
            }
            if let Some((x, y)) = rule.rule_position {
                changes.push(format!("set position to ({}, {})", x, y));
            }
            if let Some((w, h)) = rule.rule_size {
                changes.push(format!("set size to {}x{}", w, h));
            }
            if let Some(v) = rule.rule_always_on_top {
                changes.push(format!("set always_on_top to {}", v));
            }
            if let Some(v) = rule.rule_always_on_visible_workspace {
                changes.push(format!("set always_on_visible_workspace to {}", v));
            }
            if let Some(v) = rule.rule_skip_taskbar {
                changes.push(format!("set skip_taskbar to {}", v));
            }
            if let Some(v) = rule.rule_sticky {
                changes.push(format!("set sticky to {}", v));
            }
            if !changes.is_empty() {
                applied.push(AppliedRule {
                    window_id: info.id,
                    rule_index: idx,
                    changes,
                });
            }
        }
        applied
    }
}

impl Default for WindowRulesEngine {
    fn default() -> Self {
        Self::new()
    }
}
