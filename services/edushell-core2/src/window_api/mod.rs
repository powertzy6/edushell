//! Window management abstraction — API for window operations.

use serde::{Deserialize, Serialize};

/// Window identifier.
pub type WindowId = String;

/// Window state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

/// Window geometry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Window metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub state: WindowState,
    pub geometry: WindowGeometry,
    pub workspace: u32,
    pub focused: bool,
}

/// Window manager API trait.
pub trait WindowManagerApi: Send + Sync {
    fn list_windows(&self) -> Vec<WindowInfo>;
    fn focus_window(&self, id: &str) -> bool;
    fn close_window(&self, id: &str) -> bool;
    fn minimize_window(&self, id: &str) -> bool;
    fn maximize_window(&self, id: &str) -> bool;
    fn move_window(&self, id: &str, x: i32, y: i32) -> bool;
    fn resize_window(&self, id: &str, width: u32, height: u32) -> bool;
    fn active_window(&self) -> Option<WindowInfo>;
}

/// Stub implementation for testing.
pub struct StubWindowManager;

impl WindowManagerApi for StubWindowManager {
    fn list_windows(&self) -> Vec<WindowInfo> {
        Vec::new()
    }
    fn focus_window(&self, _id: &str) -> bool {
        true
    }
    fn close_window(&self, _id: &str) -> bool {
        true
    }
    fn minimize_window(&self, _id: &str) -> bool {
        true
    }
    fn maximize_window(&self, _id: &str) -> bool {
        true
    }
    fn move_window(&self, _id: &str, _x: i32, _y: i32) -> bool {
        true
    }
    fn resize_window(&self, _id: &str, _width: u32, _height: u32) -> bool {
        true
    }
    fn active_window(&self) -> Option<WindowInfo> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_window_manager() {
        let wm = StubWindowManager;
        assert!(wm.list_windows().is_empty());
        assert!(wm.focus_window("test"));
        assert!(wm.active_window().is_none());
    }

    #[test]
    fn test_window_geometry_serde() {
        let g = WindowGeometry {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };
        let json = serde_json::to_string(&g).unwrap();
        let d: WindowGeometry = serde_json::from_str(&json).unwrap();
        assert_eq!(d.width, 800);
    }

    #[test]
    fn test_window_state_variants() {
        assert_eq!(format!("{:?}", WindowState::Fullscreen), "Fullscreen");
        assert_eq!(format!("{:?}", WindowState::Hidden), "Hidden");
    }

    #[test]
    fn test_window_info() {
        let info = WindowInfo {
            id: "w1".into(),
            title: "Terminal".into(),
            app_id: "org.gnome.Terminal".into(),
            state: WindowState::Normal,
            geometry: WindowGeometry {
                x: 100,
                y: 100,
                width: 800,
                height: 600,
            },
            workspace: 1,
            focused: true,
        };
        assert_eq!(info.title, "Terminal");
    }
}
