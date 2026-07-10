// SPDX-License-Identifier: GPL-3.0-or-later

//! # Multi-Monitor Support
//!
//! Manages multiple display layouts, panel placement
//! per monitor, and independent resolution handling.

use std::collections::HashMap;

/// A single monitor's information.
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    /// Monitor index.
    pub id: u32,
    /// Monitor name (e.g. "eDP-1", "HDMI-A-1").
    pub name: String,
    /// X position (logical coordinate).
    pub x: i32,
    /// Y position (logical coordinate).
    pub y: i32,
    /// Width in pixels.
    pub width: i32,
    /// Height in pixels.
    pub height: i32,
    /// Scale factor (e.g. 1.0, 1.5, 2.0).
    pub scale: f64,
    /// Whether this is the primary monitor.
    pub primary: bool,
    /// Refresh rate in Hz.
    pub refresh_rate: f64,
}

/// Panel placement strategy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelPlacement {
    /// Panel on every monitor.
    All,
    /// Panel only on the primary monitor.
    Primary,
    /// Panel only on a specific monitor.
    Specific(u32),
}

/// Dock placement strategy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DockPlacement {
    /// Dock on every monitor.
    All,
    /// Dock only on the primary monitor.
    Primary,
    /// Dock only on a specific monitor.
    Specific(u32),
}

/// Manager for multi-monitor configuration.
#[derive(Clone)]
pub struct MultiMonitorManager {
    /// Connected monitors.
    monitors: Vec<MonitorInfo>,
    /// Panel placement strategy.
    panel_placement: PanelPlacement,
    /// Dock placement strategy.
    dock_placement: DockPlacement,
    /// Wallpaper per monitor.
    wallpapers: HashMap<u32, String>,
}

impl MultiMonitorManager {
    /// Create a new multi-monitor manager.
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
            panel_placement: PanelPlacement::Primary,
            dock_placement: DockPlacement::Primary,
            wallpapers: HashMap::new(),
        }
    }

    /// Update the list of connected monitors.
    pub fn set_monitors(&mut self, monitors: Vec<MonitorInfo>) {
        self.monitors = monitors;
    }

    /// Get all connected monitors.
    pub fn monitors(&self) -> &[MonitorInfo] {
        &self.monitors
    }

    /// Get the primary monitor.
    pub fn primary(&self) -> Option<&MonitorInfo> {
        self.monitors.iter().find(|m| m.primary)
    }

    /// Get the primary monitor index.
    pub fn primary_index(&self) -> Option<u32> {
        self.monitors.iter().find(|m| m.primary).map(|m| m.id)
    }

    /// Get monitor by ID.
    pub fn get(&self, id: u32) -> Option<&MonitorInfo> {
        self.monitors.iter().find(|m| m.id == id)
    }

    /// Get monitor count.
    pub fn count(&self) -> usize {
        self.monitors.len()
    }

    /// Check if multiple monitors are connected.
    pub fn is_multi_monitor(&self) -> bool {
        self.monitors.len() > 1
    }

    /// Set panel placement strategy.
    pub fn set_panel_placement(&mut self, placement: PanelPlacement) {
        self.panel_placement = placement;
    }

    /// Get panel placement strategy.
    pub fn panel_placement(&self) -> PanelPlacement {
        self.panel_placement
    }

    /// Set dock placement strategy.
    pub fn set_dock_placement(&mut self, placement: DockPlacement) {
        self.dock_placement = placement;
    }

    /// Get dock placement strategy.
    pub fn dock_placement(&self) -> DockPlacement {
        self.dock_placement
    }

    /// Get monitors that should show a panel.
    pub fn panel_monitors(&self) -> Vec<&MonitorInfo> {
        match self.panel_placement {
            PanelPlacement::All => self.monitors.iter().collect(),
            PanelPlacement::Primary => {
                self.monitors.iter().filter(|m| m.primary).collect()
            }
            PanelPlacement::Specific(id) => {
                self.monitors.iter().filter(|m| m.id == id).collect()
            }
        }
    }

    /// Get monitors that should show a dock.
    pub fn dock_monitors(&self) -> Vec<&MonitorInfo> {
        match self.dock_placement {
            DockPlacement::All => self.monitors.iter().collect(),
            DockPlacement::Primary => {
                self.monitors.iter().filter(|m| m.primary).collect()
            }
            DockPlacement::Specific(id) => {
                self.monitors.iter().filter(|m| m.id == id).collect()
            }
        }
    }

    /// Set wallpaper for a specific monitor.
    pub fn set_wallpaper(&mut self, monitor_id: u32, path: &str) {
        self.wallpapers.insert(monitor_id, path.to_string());
    }

    /// Get wallpaper for a monitor.
    pub fn wallpaper(&self, monitor_id: u32) -> Option<&str> {
        self.wallpapers.get(&monitor_id).map(|s| s.as_str())
    }

    /// Get total virtual desktop bounds.
    pub fn total_bounds(&self) -> (i32, i32, i32, i32) {
        if self.monitors.is_empty() {
            return (0, 0, 1920, 1080);
        }

        let min_x = self.monitors.iter().map(|m| m.x).min().unwrap_or(0);
        let min_y = self.monitors.iter().map(|m| m.y).min().unwrap_or(0);
        let max_x = self.monitors.iter().map(|m| m.x + m.width).max().unwrap_or(1920);
        let max_y = self.monitors.iter().map(|m| m.y + m.height).max().unwrap_or(1080);

        (min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Create a default single-monitor layout.
    pub fn with_default_monitor() -> Self {
        let monitor = MonitorInfo {
            id: 0,
            name: "eDP-1".to_string(),
            x: 0, y: 0,
            width: 1920, height: 1080,
            scale: 1.0,
            primary: true,
            refresh_rate: 60.0,
        };
        Self {
            monitors: vec![monitor],
            panel_placement: PanelPlacement::All,
            dock_placement: DockPlacement::All,
            wallpapers: HashMap::new(),
        }
    }
}

impl Default for MultiMonitorManager {
    fn default() -> Self {
        Self::with_default_monitor()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_monitors() -> Vec<MonitorInfo> {
        vec![
            MonitorInfo {
                id: 0, name: "eDP-1".into(),
                x: 0, y: 0, width: 1920, height: 1080,
                scale: 1.0, primary: true, refresh_rate: 60.0,
            },
            MonitorInfo {
                id: 1, name: "HDMI-A-1".into(),
                x: 1920, y: 0, width: 2560, height: 1440,
                scale: 1.0, primary: false, refresh_rate: 144.0,
            },
        ]
    }

    #[test]
    fn test_single_monitor_default() {
        let mgr = MultiMonitorManager::with_default_monitor();
        assert_eq!(mgr.count(), 1);
        assert!(mgr.primary().is_some());
        assert!(!mgr.is_multi_monitor());
    }

    #[test]
    fn test_multi_monitor_detection() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        assert!(mgr.is_multi_monitor());
        assert_eq!(mgr.count(), 2);
    }

    #[test]
    fn test_primary_monitor() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        let primary = mgr.primary().unwrap();
        assert_eq!(primary.id, 0);
        assert!(primary.primary);
    }

    #[test]
    fn test_panel_monitors_all() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        mgr.set_panel_placement(PanelPlacement::All);
        assert_eq!(mgr.panel_monitors().len(), 2);
    }

    #[test]
    fn test_panel_monitors_primary() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        mgr.set_panel_placement(PanelPlacement::Primary);
        assert_eq!(mgr.panel_monitors().len(), 1);
        assert_eq!(mgr.panel_monitors()[0].id, 0);
    }

    #[test]
    fn test_panel_monitors_specific() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        mgr.set_panel_placement(PanelPlacement::Specific(1));
        assert_eq!(mgr.panel_monitors().len(), 1);
        assert_eq!(mgr.panel_monitors()[0].id, 1);
    }

    #[test]
    fn test_dock_placement() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        mgr.set_dock_placement(DockPlacement::Primary);
        assert_eq!(mgr.dock_monitors().len(), 1);
    }

    #[test]
    fn test_wallpaper_per_monitor() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_wallpaper(0, "/path/to/wall1.jpg");
        mgr.set_wallpaper(1, "/path/to/wall2.jpg");
        assert_eq!(mgr.wallpaper(0), Some("/path/to/wall1.jpg"));
        assert_eq!(mgr.wallpaper(1), Some("/path/to/wall2.jpg"));
    }

    #[test]
    fn test_total_bounds() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        let (x, y, w, h) = mgr.total_bounds();
        assert_eq!(x, 0);
        assert_eq!(y, 0);
        assert_eq!(w, 1920 + 2560);
        assert_eq!(h, 1440); // max height
    }

    #[test]
    fn test_total_bounds_empty() {
        let mgr = MultiMonitorManager::new();
        let (x, y, w, h) = mgr.total_bounds();
        assert_eq!(x, 0);
        assert_eq!(y, 0);
        assert_eq!(w, 1920);
        assert_eq!(h, 1080);
    }

    #[test]
    fn test_get_monitor() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        assert!(mgr.get(0).is_some());
        assert!(mgr.get(1).is_some());
        assert!(mgr.get(99).is_none());
    }

    #[test]
    fn test_primary_index() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        assert_eq!(mgr.primary_index(), Some(0));
    }

    #[test]
    fn test_dock_placement_all() {
        let mut mgr = MultiMonitorManager::new();
        mgr.set_monitors(test_monitors());
        mgr.set_dock_placement(DockPlacement::All);
        assert_eq!(mgr.dock_monitors().len(), 2);
    }

    #[test]
    fn test_panel_placement_default() {
        let mgr = MultiMonitorManager::with_default_monitor();
        assert_eq!(mgr.panel_placement(), PanelPlacement::All);
    }
}
