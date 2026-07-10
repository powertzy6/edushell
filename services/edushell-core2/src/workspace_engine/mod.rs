//! Workspace (virtual desktop) engine.
use serde::{Deserialize, Serialize};

/// Workspace identifier.
pub type WorkspaceId = String;

/// Workspace metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub index: u32,
    pub active: bool,
    pub window_count: u32,
}

/// Workspace engine.
pub struct WorkspaceEngine {
    workspaces: Vec<Workspace>,
    active_index: u32,
}

impl WorkspaceEngine {
    pub fn new(count: u32) -> Self {
        let workspaces: Vec<Workspace> = (0..count)
            .map(|i| Workspace {
                id: format!("ws-{}", i + 1),
                name: format!("Workspace {}", i + 1),
                index: i,
                active: i == 0,
                window_count: 0,
            })
            .collect();
        Self {
            workspaces,
            active_index: 0,
        }
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }

    pub fn active(&self) -> Option<&Workspace> {
        self.workspaces.get(self.active_index as usize)
    }

    pub fn switch_to(&mut self, index: u32) -> bool {
        if index >= self.workspaces.len() as u32 {
            return false;
        }
        if let Some(ws) = self.workspaces.get_mut(self.active_index as usize) {
            ws.active = false;
        }
        self.active_index = index;
        if let Some(ws) = self.workspaces.get_mut(index as usize) {
            ws.active = true;
        }
        true
    }

    pub fn add(&mut self, name: &str) {
        let idx = self.workspaces.len() as u32;
        self.workspaces.push(Workspace {
            id: format!("ws-{}", idx + 1),
            name: name.to_string(),
            index: idx,
            active: false,
            window_count: 0,
        });
    }

    pub fn remove(&mut self, index: u32) -> bool {
        if index >= self.workspaces.len() as u32 || self.workspaces.len() <= 1 {
            return false;
        }
        self.workspaces.remove(index as usize);
        if self.active_index >= index && self.active_index > 0 {
            self.active_index -= 1;
        }
        true
    }

    pub fn rename(&mut self, index: u32, name: &str) -> bool {
        if let Some(ws) = self.workspaces.get_mut(index as usize) {
            ws.name = name.to_string();
            true
        } else {
            false
        }
    }

    pub fn count(&self) -> u32 {
        self.workspaces.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_new() {
        let we = WorkspaceEngine::new(4);
        assert_eq!(we.count(), 4);
        assert!(we.active().is_some());
        assert_eq!(we.active().unwrap().name, "Workspace 1");
    }

    #[test]
    fn test_workspace_switch() {
        let mut we = WorkspaceEngine::new(3);
        assert!(we.switch_to(1));
        assert_eq!(we.active().unwrap().index, 1);
    }

    #[test]
    fn test_workspace_switch_invalid() {
        let mut we = WorkspaceEngine::new(2);
        assert!(!we.switch_to(5));
    }

    #[test]
    fn test_workspace_add() {
        let mut we = WorkspaceEngine::new(1);
        we.add("Custom");
        assert_eq!(we.count(), 2);
    }

    #[test]
    fn test_workspace_remove() {
        let mut we = WorkspaceEngine::new(3);
        assert!(we.remove(1));
        assert_eq!(we.count(), 2);
    }

    #[test]
    fn test_workspace_cannot_remove_last() {
        let mut we = WorkspaceEngine::new(1);
        assert!(!we.remove(0));
    }

    #[test]
    fn test_workspace_rename() {
        let mut we = WorkspaceEngine::new(2);
        assert!(we.rename(0, "Main"));
        assert_eq!(we.workspaces()[0].name, "Main");
    }
}
