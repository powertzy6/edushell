use crate::core::{now, WindowId};

#[derive(Debug, Clone)]
pub enum WorkspaceConfig {
    Static { count: usize },
    Dynamic { min: usize, max: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LayoutMode {
    Floating,
    Tiling,
    Monocle,
    Grid,
    SplitHorizontal,
    SplitVertical,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct WorkspaceLayout {
    pub columns: u32,
    pub rows: u32,
    pub gaps: i32,
    pub layout_mode: LayoutMode,
}

impl Default for WorkspaceLayout {
    fn default() -> Self {
        Self {
            columns: 1,
            rows: 1,
            gaps: 5,
            layout_mode: LayoutMode::Floating,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub id: usize,
    pub name: String,
    pub windows: Vec<WindowId>,
    pub monitor: usize,
    pub layout: WorkspaceLayout,
    pub is_active: bool,
    pub is_visible: bool,
    pub created_at: String,
}

pub struct WorkspaceEngineV2 {
    #[allow(dead_code)]
    pub config: WorkspaceConfig,
    pub workspaces: Vec<WorkspaceInfo>,
    pub active_workspace: usize,
    #[allow(dead_code)]
    pub overview_active: bool,
    #[allow(dead_code)]
    pub animation_in_progress: bool,
}

impl WorkspaceEngineV2 {
    pub fn new(config: WorkspaceConfig) -> Self {
        Self {
            config,
            workspaces: Vec::new(),
            active_workspace: 0,
            overview_active: false,
            animation_in_progress: false,
        }
    }

    pub fn init(&mut self, count: usize) {
        self.workspaces.clear();
        for i in 0..count {
            self.workspaces.push(WorkspaceInfo {
                id: i,
                name: format!("{}", i + 1),
                windows: Vec::new(),
                monitor: 0,
                layout: WorkspaceLayout::default(),
                is_active: i == 0,
                is_visible: true,
                created_at: now(),
            });
        }
        self.active_workspace = 0;
    }

    pub fn add_workspace(&mut self) -> usize {
        let id = self.workspaces.len();
        self.workspaces.push(WorkspaceInfo {
            id,
            name: format!("{}", id + 1),
            windows: Vec::new(),
            monitor: 0,
            layout: WorkspaceLayout::default(),
            is_active: false,
            is_visible: false,
            created_at: now(),
        });
        id
    }

    pub fn remove_workspace(&mut self, id: usize) -> bool {
        if id >= self.workspaces.len() || self.workspaces.len() <= 1 {
            return false;
        }
        self.workspaces.remove(id);
        for (i, ws) in self.workspaces.iter_mut().enumerate() {
            ws.id = i;
            ws.name = format!("{}", i + 1);
        }
        if self.active_workspace >= self.workspaces.len() {
            self.active_workspace = self.workspaces.len() - 1;
        }
        self.workspaces[self.active_workspace].is_active = true;
        true
    }

    pub fn switch_to(&mut self, id: usize) -> bool {
        if id >= self.workspaces.len() {
            return false;
        }
        self.workspaces[self.active_workspace].is_active = false;
        self.active_workspace = id;
        self.workspaces[id].is_active = true;
        true
    }

    pub fn get_workspace(&self, id: usize) -> Option<&WorkspaceInfo> {
        self.workspaces.get(id)
    }

    pub fn get_workspace_mut(&mut self, id: usize) -> Option<&mut WorkspaceInfo> {
        self.workspaces.get_mut(id)
    }

    pub fn list_workspaces(&self) -> &Vec<WorkspaceInfo> {
        &self.workspaces
    }

    pub fn active(&self) -> &WorkspaceInfo {
        &self.workspaces[self.active_workspace]
    }

    pub fn active_mut(&mut self) -> &mut WorkspaceInfo {
        &mut self.workspaces[self.active_workspace]
    }

    pub fn move_window_to(&mut self, window_id: WindowId, from_ws: usize, to_ws: usize) -> bool {
        if from_ws >= self.workspaces.len() || to_ws >= self.workspaces.len() || from_ws == to_ws {
            return false;
        }
        let pos = self.workspaces[from_ws]
            .windows
            .iter()
            .position(|&w| w == window_id);
        if let Some(pos) = pos {
            self.workspaces[from_ws].windows.remove(pos);
            self.workspaces[to_ws].windows.push(window_id);
            true
        } else {
            false
        }
    }

    pub fn move_window_to_monitor(
        &mut self,
        window_id: WindowId,
        ws_id: usize,
        monitor_id: usize,
    ) -> bool {
        if ws_id >= self.workspaces.len() {
            return false;
        }
        let has_window = self.workspaces[ws_id].windows.contains(&window_id);
        if has_window {
            self.workspaces[ws_id].monitor = monitor_id;
            true
        } else {
            false
        }
    }

    pub fn set_overview(&mut self, active: bool) -> bool {
        let changed = self.overview_active != active;
        self.overview_active = active;
        changed
    }

    pub fn is_overview(&self) -> bool {
        self.overview_active
    }

    pub fn set_layout(&mut self, ws_id: usize, layout: WorkspaceLayout) -> bool {
        if ws_id < self.workspaces.len() {
            self.workspaces[ws_id].layout = layout;
            true
        } else {
            false
        }
    }

    pub fn get_layout(&self, ws_id: usize) -> Option<&WorkspaceLayout> {
        self.workspaces.get(ws_id).map(|ws| &ws.layout)
    }

    pub fn windows_on_workspace(&self, ws_id: usize) -> Vec<WindowId> {
        if ws_id < self.workspaces.len() {
            self.workspaces[ws_id].windows.clone()
        } else {
            Vec::new()
        }
    }

    pub fn window_count(&self, ws_id: usize) -> usize {
        if ws_id < self.workspaces.len() {
            self.workspaces[ws_id].windows.len()
        } else {
            0
        }
    }

    pub fn total_windows(&self) -> usize {
        self.workspaces.iter().map(|ws| ws.windows.len()).sum()
    }

    pub fn rename_workspace(&mut self, id: usize, name: &str) -> bool {
        if id < self.workspaces.len() {
            self.workspaces[id].name = name.to_string();
            true
        } else {
            false
        }
    }
}
