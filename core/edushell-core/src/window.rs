// SPDX-License-Identifier: GPL-3.0-or-later

//! # Window Tracker
//!
//! Tracks all windows on the desktop: list, focus, workspace,
//! state changes. Integrates with Wayland (wlr-foreign-toplevel)
//! and X11 (via Cinnamon/Muffin) to maintain an up-to-date
//! window registry.

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::EduResult;

/// Unique window identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowId(pub String);

impl From<&str> for WindowId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Current state of a window.
#[derive(Debug, Clone)]
pub struct WindowInfo {
    /// Unique window ID.
    pub id: WindowId,
    /// Application ID (desktop file name).
    pub app_id: String,
    /// Window title.
    pub title: String,
    /// Workspace index (0-based).
    pub workspace: u32,
    /// Whether window is focused.
    pub focused: bool,
    /// Whether window is fullscreen.
    pub fullscreen: bool,
    /// Whether window is minimized.
    pub minimized: bool,
    /// Whether window is maximized.
    pub maximized: bool,
    /// Window PID.
    pub pid: Option<u32>,
    /// Window geometry (x, y, width, height).
    pub geometry: Option<(i32, i32, u32, u32)>,
}

/// Changes in window state (for event emission).
#[derive(Debug, Clone)]
pub enum WindowChange {
    /// Window was created.
    Created(WindowInfo),
    /// Window was closed.
    Closed(WindowId),
    /// Window focus changed.
    FocusChanged(WindowId),
    /// Window title changed.
    TitleChanged { id: WindowId, title: String },
    /// Window workspace changed.
    WorkspaceChanged { id: WindowId, workspace: u32 },
    /// Window geometry changed.
    GeometryChanged { id: WindowId, geometry: (i32, i32, u32, u32) },
    /// Window state flags changed.
    StateChanged { id: WindowId, fullscreen: bool, minimized: bool, maximized: bool },
}

/// Thread-safe window tracker.
#[derive(Clone)]
pub struct WindowTracker {
    windows: Arc<std::sync::RwLock<HashMap<WindowId, WindowInfo>>>,
}

impl WindowTracker {
    /// Create a new window tracker.
    pub fn new() -> Self {
        Self {
            windows: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register or update a window.
    pub fn upsert(&self, info: WindowInfo) -> WindowChange {
        let mut windows = self.windows.write()
            .expect("Window tracker lock poisoned");

        let exists = windows.contains_key(&info.id);
        let change = if !exists {
            WindowChange::Created(info.clone())
        } else {
            let old = windows.get(&info.id).unwrap();
            if old.focused != info.focused {
                WindowChange::FocusChanged(info.id.clone())
            } else if old.title != info.title {
                WindowChange::TitleChanged {
                    id: info.id.clone(),
                    title: info.title.clone(),
                }
            } else if old.workspace != info.workspace {
                WindowChange::WorkspaceChanged {
                    id: info.id.clone(),
                    workspace: info.workspace,
                }
            } else {
                WindowChange::StateChanged {
                    id: info.id.clone(),
                    fullscreen: info.fullscreen,
                    minimized: info.minimized,
                    maximized: info.maximized,
                }
            }
        };

        windows.insert(info.id.clone(), info);
        change
    }

    /// Remove a closed window.
    pub fn remove(&self, id: &WindowId) -> Option<WindowInfo> {
        let mut windows = self.windows.write()
            .expect("Window tracker lock poisoned");
        windows.remove(id)
    }

    /// Get info for a specific window.
    pub fn get(&self, id: &WindowId) -> Option<WindowInfo> {
        let windows = self.windows.read()
            .expect("Window tracker lock poisoned");
        windows.get(id).cloned()
    }

    /// Get all tracked windows.
    pub fn all(&self) -> Vec<WindowInfo> {
        let windows = self.windows.read()
            .expect("Window tracker lock poisoned");
        windows.values().cloned().collect()
    }

    /// Get the currently focused window.
    pub fn focused(&self) -> Option<WindowInfo> {
        let windows = self.windows.read()
            .expect("Window tracker lock poisoned");
        windows.values().find(|w| w.focused).cloned()
    }

    /// Get windows on a specific workspace.
    pub fn on_workspace(&self, workspace: u32) -> Vec<WindowInfo> {
        let windows = self.windows.read()
            .expect("Window tracker lock poisoned");
        windows
            .values()
            .filter(|w| w.workspace == workspace && !w.minimized)
            .cloned()
            .collect()
    }

    /// Get window count.
    pub fn count(&self) -> usize {
        self.windows.read()
            .map(|w| w.len())
            .unwrap_or(0)
    }

    /// Set focused window.
    pub fn set_focused(&self, id: &WindowId) -> EduResult<()> {
        let mut windows = self.windows.write()
            .map_err(|e| {
                crate::error::EduError::Unknown(format!("Window tracker lock: {e}"))
            })?;

        // Unfocus all
        for window in windows.values_mut() {
            window.focused = false;
        }

        // Focus target
        if let Some(window) = windows.get_mut(id) {
            window.focused = true;
        }

        Ok(())
    }

    /// Move window to workspace.
    pub fn move_to_workspace(&self, id: &WindowId, workspace: u32) -> EduResult<()> {
        let mut windows = self.windows.write()
            .map_err(|e| {
                crate::error::EduError::Unknown(format!("Window tracker lock: {e}"))
            })?;

        if let Some(window) = windows.get_mut(id) {
            window.workspace = workspace;
        }

        Ok(())
    }

    /// Update window state flags.
    pub fn update_state(
        &self,
        id: &WindowId,
        fullscreen: Option<bool>,
        minimized: Option<bool>,
        maximized: Option<bool>,
    ) -> EduResult<()> {
        let mut windows = self.windows.write()
            .map_err(|e| {
                crate::error::EduError::Unknown(format!("Window tracker lock: {e}"))
            })?;

        if let Some(window) = windows.get_mut(id) {
            if let Some(v) = fullscreen { window.fullscreen = v; }
            if let Some(v) = minimized { window.minimized = v; }
            if let Some(v) = maximized { window.maximized = v; }
        }

        Ok(())
    }
}

impl Default for WindowTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_window(id: &str, title: &str, workspace: u32) -> WindowInfo {
        WindowInfo {
            id: WindowId(id.to_string()),
            app_id: id.to_string(),
            title: title.to_string(),
            workspace,
            focused: false,
            fullscreen: false,
            minimized: false,
            maximized: false,
            pid: None,
            geometry: None,
        }
    }

    #[test]
    fn test_add_window() {
        let tracker = WindowTracker::new();
        let win = make_window("test", "Test Window", 0);
        tracker.upsert(win);
        assert_eq!(tracker.count(), 1);
    }

    #[test]
    fn test_remove_window() {
        let tracker = WindowTracker::new();
        let id = WindowId("test".into());
        tracker.upsert(make_window("test", "Test", 0));
        tracker.remove(&id);
        assert_eq!(tracker.count(), 0);
    }

    #[test]
    fn test_focused_window() {
        let tracker = WindowTracker::new();
        let id1 = WindowId("win1".into());
        let id2 = WindowId("win2".into());

        tracker.upsert(make_window("win1", "Window 1", 0));
        tracker.upsert(make_window("win2", "Window 2", 1));

        tracker.set_focused(&id1).unwrap();
        assert_eq!(tracker.focused().unwrap().id, id1);

        tracker.set_focused(&id2).unwrap();
        assert_eq!(tracker.focused().unwrap().id, id2);
    }

    #[test]
    fn test_windows_on_workspace() {
        let tracker = WindowTracker::new();
        tracker.upsert(make_window("a", "A", 0));
        tracker.upsert(make_window("b", "B", 0));
        tracker.upsert(make_window("c", "C", 1));

        let ws0 = tracker.on_workspace(0);
        assert_eq!(ws0.len(), 2);

        let ws1 = tracker.on_workspace(1);
        assert_eq!(ws1.len(), 1);
    }

    #[test]
    fn test_move_to_workspace() {
        let tracker = WindowTracker::new();
        let id = WindowId("a".into());
        tracker.upsert(make_window("a", "A", 0));

        tracker.move_to_workspace(&id, 2).unwrap();
        let win = tracker.get(&id).unwrap();
        assert_eq!(win.workspace, 2);
    }

    #[test]
    fn test_update_state() {
        let tracker = WindowTracker::new();
        let id = WindowId("a".into());
        tracker.upsert(make_window("a", "A", 0));

        tracker.update_state(&id, Some(true), None, None).unwrap();
        let win = tracker.get(&id).unwrap();
        assert!(win.fullscreen);
    }
}
