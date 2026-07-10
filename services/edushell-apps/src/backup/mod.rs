use chrono::Utc;

#[derive(Debug, Clone)]
pub struct BackupEntry {
    pub path: String,
    pub size_kb: u64,
    pub checksum: String,
}

pub struct BackupManager {
    backups: Vec<BackupSnapshot>,
    max_backups: usize,
}

#[derive(Debug, Clone)]
pub struct BackupSnapshot {
    pub id: String,
    pub timestamp: String,
    pub label: String,
    pub entries: Vec<BackupEntry>,
    pub total_size_kb: u64,
}

impl BackupManager {
    pub fn new() -> Self {
        Self {
            backups: Vec::new(),
            max_backups: 10,
        }
    }

    pub fn create_snapshot(&mut self, label: &str) -> String {
        let id = format!("backup-{}", Utc::now().timestamp());
        let snapshot = BackupSnapshot {
            id: id.clone(),
            timestamp: Utc::now().to_rfc3339(),
            label: label.to_string(),
            entries: Vec::new(),
            total_size_kb: 0,
        };
        if self.backups.len() >= self.max_backups {
            self.backups.remove(0);
        }
        self.backups.push(snapshot);
        id
    }

    pub fn add_entry(&mut self, backup_id: &str, path: &str, size_kb: u64, checksum: &str) -> bool {
        if let Some(snap) = self.backups.iter_mut().find(|s| s.id == backup_id) {
            snap.entries.push(BackupEntry {
                path: path.to_string(),
                size_kb,
                checksum: checksum.to_string(),
            });
            snap.total_size_kb += size_kb;
            true
        } else {
            false
        }
    }

    pub fn snapshots(&self) -> &[BackupSnapshot] { &self.backups }

    pub fn get_snapshot(&self, id: &str) -> Option<&BackupSnapshot> {
        self.backups.iter().find(|s| s.id == id)
    }

    pub fn delete_snapshot(&mut self, id: &str) -> bool {
        let len = self.backups.len();
        self.backups.retain(|s| s.id != id);
        self.backups.len() < len
    }

    pub fn latest_snapshot(&self) -> Option<&BackupSnapshot> {
        self.backups.last()
    }

    pub fn total_backup_size(&self) -> u64 {
        self.backups.iter().map(|s| s.total_size_kb).sum()
    }
}

impl Default for BackupManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_new() {
        let bm = BackupManager::new();
        assert!(bm.snapshots().is_empty());
    }

    #[test]
    fn test_create_snapshot() {
        let mut bm = BackupManager::new();
        let id = bm.create_snapshot("before-update");
        assert!(id.starts_with("backup-"));
        assert_eq!(bm.snapshots().len(), 1);
    }

    #[test]
    fn test_add_entry() {
        let mut bm = BackupManager::new();
        let id = bm.create_snapshot("test");
        assert!(bm.add_entry(&id, "/home/user/docs", 1024, "abc123"));
        let snap = bm.get_snapshot(&id).unwrap();
        assert_eq!(snap.entries.len(), 1);
        assert_eq!(snap.total_size_kb, 1024);
    }

    #[test]
    fn test_add_entry_invalid_id() {
        let mut bm = BackupManager::new();
        assert!(!bm.add_entry("nonexistent", "/tmp", 0, ""));
    }

    #[test]
    fn test_delete_snapshot() {
        let mut bm = BackupManager::new();
        let id = bm.create_snapshot("test");
        assert!(bm.delete_snapshot(&id));
        assert!(bm.snapshots().is_empty());
    }

    #[test]
    fn test_latest_snapshot() {
        let mut bm = BackupManager::new();
        assert!(bm.latest_snapshot().is_none());
        bm.create_snapshot("first");
        assert!(bm.latest_snapshot().is_some());
    }

    #[test]
    fn test_max_backups() {
        let mut bm = BackupManager::new();
        bm.max_backups = 3;
        for i in 0..5 {
            bm.create_snapshot(&format!("backup {}", i));
        }
        assert_eq!(bm.snapshots().len(), 3);
    }

    #[test]
    fn test_total_backup_size() {
        let mut bm = BackupManager::new();
        let id = bm.create_snapshot("test");
        bm.add_entry(&id, "/a", 500, "x");
        bm.add_entry(&id, "/b", 300, "y");
        assert_eq!(bm.total_backup_size(), 800);
    }
}
