use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonitorTransform {
    Normal,
    Rot90,
    Rot180,
    Rot270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

#[derive(Debug, Clone)]
pub struct MonitorMode {
    pub width: i32,
    pub height: i32,
    pub refresh_rate: f64,
    pub preferred: bool,
}

#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub id: usize,
    pub name: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub geometry: (i32, i32, i32, i32),
    pub scale: f64,
    pub refresh_rate: f64,
    pub is_primary: bool,
    pub is_active: bool,
    pub transform: MonitorTransform,
    pub connector: String,
    pub workspaces: Vec<usize>,
    pub physical_width_mm: u32,
    pub physical_height_mm: u32,
    pub edid: Vec<u8>,
    pub modes: Vec<MonitorMode>,
    pub current_mode: usize,
}

#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub monitors: Vec<MonitorInfo>,
    pub primary_id: Option<usize>,
}

pub struct MonitorManager {
    monitors: HashMap<usize, MonitorInfo>,
    primary: Option<usize>,
    next_id: usize,
}

impl MonitorManager {
    pub fn new() -> Self {
        Self {
            monitors: HashMap::new(),
            primary: None,
            next_id: 0,
        }
    }

    pub fn add_monitor(&mut self, mut info: MonitorInfo) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        info.id = id;
        if self.monitors.is_empty() {
            info.is_primary = true;
            self.primary = Some(id);
        }
        self.monitors.insert(id, info);
        id
    }

    pub fn remove_monitor(&mut self, id: usize) -> bool {
        if self.monitors.remove(&id).is_some() {
            if self.primary == Some(id) {
                self.primary = self.monitors.keys().next().copied();
                if let Some(primary_id) = self.primary {
                    if let Some(m) = self.monitors.get_mut(&primary_id) {
                        m.is_primary = true;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_monitor(&self, id: usize) -> Option<&MonitorInfo> {
        self.monitors.get(&id)
    }

    pub fn get_monitor_mut(&mut self, id: usize) -> Option<&mut MonitorInfo> {
        self.monitors.get_mut(&id)
    }

    pub fn list_monitors(&self) -> Vec<&MonitorInfo> {
        self.monitors.values().collect()
    }

    pub fn active_monitors(&self) -> Vec<&MonitorInfo> {
        self.monitors.values().filter(|m| m.is_active).collect()
    }

    pub fn count(&self) -> usize {
        self.monitors.len()
    }

    pub fn set_primary(&mut self, id: usize) -> bool {
        if !self.monitors.contains_key(&id) {
            return false;
        }
        if let Some(old_id) = self.primary {
            if let Some(old) = self.monitors.get_mut(&old_id) {
                old.is_primary = false;
            }
        }
        self.primary = Some(id);
        if let Some(m) = self.monitors.get_mut(&id) {
            m.is_primary = true;
        }
        true
    }

    pub fn get_primary(&self) -> Option<&MonitorInfo> {
        self.primary.and_then(|id| self.monitors.get(&id))
    }

    pub fn find_by_connector(&self, connector: &str) -> Option<&MonitorInfo> {
        self.monitors.values().find(|m| m.connector == connector)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&MonitorInfo> {
        self.monitors.values().find(|m| m.name == name)
    }

    pub fn get_geometry(&self, id: usize) -> Option<(i32, i32, i32, i32)> {
        self.monitors.get(&id).map(|m| m.geometry)
    }

    pub fn set_geometry(&mut self, id: usize, geo: (i32, i32, i32, i32)) -> bool {
        if let Some(m) = self.monitors.get_mut(&id) {
            m.geometry = geo;
            true
        } else {
            false
        }
    }

    pub fn set_scale(&mut self, id: usize, scale: f64) -> bool {
        if let Some(m) = self.monitors.get_mut(&id) {
            m.scale = scale;
            true
        } else {
            false
        }
    }

    pub fn set_transform(&mut self, id: usize, transform: MonitorTransform) -> bool {
        if let Some(m) = self.monitors.get_mut(&id) {
            m.transform = transform;
            true
        } else {
            false
        }
    }

    pub fn set_mode(&mut self, id: usize, mode_idx: usize) -> bool {
        if let Some(m) = self.monitors.get_mut(&id) {
            if mode_idx < m.modes.len() {
                m.current_mode = mode_idx;
                let mode = &m.modes[mode_idx];
                m.geometry.2 = mode.width;
                m.geometry.3 = mode.height;
                m.refresh_rate = mode.refresh_rate;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn add_mode(&mut self, id: usize, mode: MonitorMode) -> bool {
        if let Some(m) = self.monitors.get_mut(&id) {
            m.modes.push(mode);
            true
        } else {
            false
        }
    }

    pub fn assign_workspace(&mut self, monitor_id: usize, ws_id: usize) -> bool {
        if let Some(m) = self.monitors.get_mut(&monitor_id) {
            if !m.workspaces.contains(&ws_id) {
                m.workspaces.push(ws_id);
            }
            true
        } else {
            false
        }
    }

    pub fn remove_workspace(&mut self, monitor_id: usize, ws_id: usize) {
        if let Some(m) = self.monitors.get_mut(&monitor_id) {
            m.workspaces.retain(|&w| w != ws_id);
        }
    }

    pub fn workspaces_for_monitor(&self, id: usize) -> Vec<usize> {
        self.monitors
            .get(&id)
            .map(|m| m.workspaces.clone())
            .unwrap_or_default()
    }

    pub fn handle_hotplug(&mut self, connected: bool, info: MonitorInfo) -> Option<usize> {
        if connected {
            Some(self.add_monitor(info))
        } else {
            let id = info.id;
            self.remove_monitor(id);
            None
        }
    }

    pub fn clone_monitor_config(&self) -> MonitorConfig {
        MonitorConfig {
            monitors: self.monitors.values().cloned().collect(),
            primary_id: self.primary,
        }
    }

    pub fn apply_monitor_config(&mut self, config: MonitorConfig) {
        self.monitors.clear();
        self.primary = None;
        self.next_id = 0;
        for mut info in config.monitors {
            info.id = self.next_id;
            self.next_id += 1;
            if (Some(info.id) == config.primary_id || self.monitors.is_empty())
                && Some(info.id) == config.primary_id
            {
                info.is_primary = true;
                self.primary = Some(info.id);
            }
            self.monitors.insert(info.id, info);
        }
    }
}

impl Default for MonitorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_monitor(id: usize) -> MonitorInfo {
        MonitorInfo {
            id,
            name: format!("Test-{}", id),
            make: "TestMake".to_string(),
            model: "TestModel".to_string(),
            serial: format!("SN{}", id),
            geometry: (0, 0, 1920, 1080),
            scale: 1.0,
            refresh_rate: 60.0,
            is_primary: false,
            is_active: true,
            transform: MonitorTransform::Normal,
            connector: format!("DP-{}", id),
            workspaces: vec![id],
            physical_width_mm: 527,
            physical_height_mm: 296,
            edid: vec![0x00, 0xFF],
            modes: vec![
                MonitorMode {
                    width: 1920,
                    height: 1080,
                    refresh_rate: 60.0,
                    preferred: true,
                },
                MonitorMode {
                    width: 1280,
                    height: 720,
                    refresh_rate: 30.0,
                    preferred: false,
                },
            ],
            current_mode: 0,
        }
    }

    #[test]
    fn test_add_remove_monitor() {
        let mut mgr = MonitorManager::new();
        let id = mgr.add_monitor(make_test_monitor(0));
        assert_eq!(mgr.count(), 1);
        assert!(mgr.get_monitor(id).is_some());
        assert!(mgr.remove_monitor(id));
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_primary() {
        let mut mgr = MonitorManager::new();
        let id1 = mgr.add_monitor(make_test_monitor(0));
        assert!(mgr.get_primary().is_some());
        assert_eq!(mgr.get_primary().unwrap().id, id1);
        let id2 = mgr.add_monitor(make_test_monitor(1));
        assert_eq!(mgr.get_primary().unwrap().id, id1);
        assert!(mgr.set_primary(id2));
        assert_eq!(mgr.get_primary().unwrap().id, id2);
        assert!(!mgr.get_monitor(id1).unwrap().is_primary);
        assert!(mgr.get_monitor(id2).unwrap().is_primary);
    }

    #[test]
    fn test_scaling() {
        let mut mgr = MonitorManager::new();
        let id = mgr.add_monitor(make_test_monitor(0));
        assert!(mgr.set_scale(id, 1.5));
        assert_eq!(mgr.get_monitor(id).unwrap().scale, 1.5);
    }

    #[test]
    fn test_transform() {
        let mut mgr = MonitorManager::new();
        let id = mgr.add_monitor(make_test_monitor(0));
        assert!(mgr.set_transform(id, MonitorTransform::Rot90));
        assert_eq!(
            mgr.get_monitor(id).unwrap().transform,
            MonitorTransform::Rot90
        );
    }

    #[test]
    fn test_hotplug() {
        let mut mgr = MonitorManager::new();
        let info = make_test_monitor(0);
        let result = mgr.handle_hotplug(true, info);
        assert!(result.is_some());
        assert_eq!(mgr.count(), 1);
        let info2 = make_test_monitor(0);
        mgr.handle_hotplug(false, info2);
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_mode_switch() {
        let mut mgr = MonitorManager::new();
        let id = mgr.add_monitor(make_test_monitor(0));
        assert!(mgr.set_mode(id, 1));
        assert_eq!(mgr.get_monitor(id).unwrap().current_mode, 1);
        assert_eq!(mgr.get_monitor(id).unwrap().geometry.2, 1280);
        assert!(!mgr.set_mode(id, 5));
    }

    #[test]
    fn test_workspace_assignment() {
        let mut mgr = MonitorManager::new();
        let id = mgr.add_monitor(make_test_monitor(0));
        assert!(mgr.assign_workspace(id, 10));
        assert!(mgr.assign_workspace(id, 20));
        assert_eq!(mgr.workspaces_for_monitor(id), vec![0, 10, 20]);
        mgr.remove_workspace(id, 10);
        assert_eq!(mgr.workspaces_for_monitor(id), vec![0, 20]);
    }
}
