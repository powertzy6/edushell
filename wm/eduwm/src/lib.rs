pub mod animation;
pub mod compositor;
pub mod core;
pub mod debug;
pub mod input;
pub mod monitor;
pub mod recovery;
pub mod security;
pub mod testing;
pub mod theme;
pub mod wayland;
pub mod workspace;
pub mod x11;

pub use animation::AnimationEngineV2;
pub use compositor::Compositor;
pub use core::{
    AppliedRule, FocusManager, Rect, StackingManager, WindowCore, WindowFlags, WindowId,
    WindowInfo, WindowRule, WindowRulesEngine, WindowState, WindowType,
};
pub use debug::DebugTool;
pub use input::InputManager;
pub use monitor::MonitorManager;
pub use recovery::CrashRecovery;
pub use security::SecurityManager;
pub use testing::EduWmTestFramework;
pub use theme::ThemeLayer;
pub use wayland::WaylandCompat;
pub use workspace::{
    LayoutMode, WorkspaceConfig, WorkspaceEngineV2 as WorkspaceEngine, WorkspaceInfo,
    WorkspaceLayout,
};
pub use x11::X11Compat;

pub const MAJOR: u32 = 1;
pub const MINOR: u32 = 0;
pub const PATCH: u32 = 0;
pub const VERSION_STR: &str = "1.0.0";

pub struct EduWM {
    running: bool,
    initialized: bool,
}

impl EduWM {
    pub fn new() -> Self {
        Self {
            running: false,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    pub fn run(&mut self) {
        self.running = true;
    }

    pub fn shutdown(&mut self) {
        self.running = false;
    }

    pub fn version() -> &'static str {
        VERSION_STR
    }
}

impl Default for EduWM {
    fn default() -> Self {
        Self::new()
    }
}
