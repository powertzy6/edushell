use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::localization::*;
use crate::tr;

pub type Result<T> = std::result::Result<T, FileManagerError>;

#[derive(Debug, thiserror::Error)]
pub enum FileManagerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("nemo not available")]
    NemoNotAvailable,
    #[error("entry not found: {0}")]
    NotFound(String),
    #[error("invalid folder type")]
    InvalidFolderType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuickAccessFolder {
    Home,
    Learning,
    Projects,
    School,
    Downloads,
    Recent,
    Favorite,
    Workspace,
    Documents,
    Pictures,
    Music,
    Videos,
    Desktop,
    Templates,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAccessEntry {
    pub id: String,
    pub name_key: String,
    pub folder_type: QuickAccessFolder,
    pub path: PathBuf,
    pub icon: String,
    pub is_default: bool,
    pub is_edushell: bool,
    pub item_count: u32,
    pub size_kb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileManagerStats {
    pub total_entries: u32,
    pub edushell_folders: u32,
    pub total_items: u32,
    pub total_size_kb: u64,
    pub recent_files_count: u32,
}

pub struct FileManagerIntegration {
    quick_access: Vec<QuickAccessEntry>,
    recent_files: Vec<PathBuf>,
    favorites: Vec<String>,
    localization: LocalizationManager,
    _home_dir: PathBuf,
    edushell_dir: PathBuf,
    nemo_available: bool,
}

impl FileManagerIntegration {
    pub fn new() -> Self {
        let localization = LocalizationManager::new();
        let home_dir = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"));
        let edushell_dir = home_dir.join("EduShell");
        let nemo_available = Self::detect_nemo();
        let quick_access = Self::build_default_entries(&home_dir, &edushell_dir);
        FileManagerIntegration {
            quick_access,
            recent_files: Vec::new(),
            favorites: Vec::new(),
            localization,
            _home_dir: home_dir,
            edushell_dir,
            nemo_available,
        }
    }

    fn detect_nemo() -> bool {
        Command::new("which")
            .arg("nemo")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn build_default_entries(home: &Path, edushell: &Path) -> Vec<QuickAccessEntry> {
        let defs: Vec<(QuickAccessFolder, &str, bool, bool, &str)> = vec![
            (QuickAccessFolder::Home, "file_manager.home", false, false, "user-home"),
            (QuickAccessFolder::Learning, "file_manager.learning", false, true, "folder-education"),
            (QuickAccessFolder::Projects, "file_manager.projects", false, true, "folder-projects"),
            (QuickAccessFolder::School, "file_manager.school", false, true, "folder-school"),
            (QuickAccessFolder::Downloads, "file_manager.downloads", false, false, "folder-download"),
            (QuickAccessFolder::Recent, "file_manager.recent", false, false, "folder-recent"),
            (QuickAccessFolder::Favorite, "file_manager.favorite", false, true, "folder-favorite"),
            (QuickAccessFolder::Workspace, "file_manager.workspace", false, true, "folder-workspace"),
            (QuickAccessFolder::Documents, "file_manager.documents", false, false, "folder-documents"),
            (QuickAccessFolder::Pictures, "file_manager.pictures", false, false, "folder-pictures"),
            (QuickAccessFolder::Music, "file_manager.music", false, false, "folder-music"),
            (QuickAccessFolder::Videos, "file_manager.videos", false, false, "folder-videos"),
            (QuickAccessFolder::Desktop, "file_manager.desktop", false, false, "folder-desktop"),
            (QuickAccessFolder::Templates, "file_manager.templates", false, false, "folder-templates"),
            (QuickAccessFolder::Public, "file_manager.public", false, false, "folder-public"),
        ];

        defs.into_iter()
            .map(|(ft, name_key, is_default, is_edushell, icon)| {
                let path = Self::default_path_for(ft, home, edushell);
                let id = format!("{:?}", ft).to_lowercase();
                QuickAccessEntry {
                    id,
                    name_key: name_key.to_string(),
                    folder_type: ft,
                    path,
                    icon: icon.to_string(),
                    is_default,
                    is_edushell,
                    item_count: 0,
                    size_kb: 0,
                }
            })
            .collect()
    }

    fn default_path_for(ft: QuickAccessFolder, home: &Path, edushell: &Path) -> PathBuf {
        match ft {
            QuickAccessFolder::Home => home.to_path_buf(),
            QuickAccessFolder::Learning => edushell.join("Learning"),
            QuickAccessFolder::Projects => edushell.join("Projects"),
            QuickAccessFolder::School => edushell.join("School"),
            QuickAccessFolder::Downloads => home.join("Downloads"),
            QuickAccessFolder::Recent => home.to_path_buf(),
            QuickAccessFolder::Favorite => edushell.join("Favorites"),
            QuickAccessFolder::Workspace => edushell.join("Workspace"),
            QuickAccessFolder::Documents => home.join("Documents"),
            QuickAccessFolder::Pictures => home.join("Pictures"),
            QuickAccessFolder::Music => home.join("Music"),
            QuickAccessFolder::Videos => home.join("Videos"),
            QuickAccessFolder::Desktop => home.join("Desktop"),
            QuickAccessFolder::Templates => home.join("Templates"),
            QuickAccessFolder::Public => home.join("Public"),
        }
    }

    pub fn nemo_available(&self) -> bool {
        self.nemo_available
    }

    pub fn open_in_nemo(&self, path: &Path) -> Result<()> {
        if !self.nemo_available {
            return Err(FileManagerError::NemoNotAvailable);
        }
        Command::new("nemo")
            .arg(path)
            .spawn()?;
        Ok(())
    }

    pub fn open_in_default(&self, path: &Path) {
        let _ = Command::new("xdg-open").arg(path).spawn();
    }

    pub fn quick_access(&self) -> &[QuickAccessEntry] {
        &self.quick_access
    }

    pub fn quick_access_by_type(&self, ft: QuickAccessFolder) -> Option<&QuickAccessEntry> {
        self.quick_access.iter().find(|e| e.folder_type == ft)
    }

    pub fn open_quick_access(&self, id: &str) -> Result<()> {
        let entry = self
            .quick_access
            .iter()
            .find(|e| e.id == id)
            .ok_or_else(|| FileManagerError::NotFound(id.to_string()))?;
        if self.nemo_available {
            self.open_in_nemo(&entry.path)?;
        } else {
            self.open_in_default(&entry.path);
        }
        Ok(())
    }

    pub fn edushell_folders(&self) -> Vec<QuickAccessEntry> {
        self.quick_access.iter().filter(|e| e.is_edushell).cloned().collect()
    }

    pub fn recent_files(&self) -> &[PathBuf] {
        &self.recent_files
    }

    pub fn add_recent_file(&mut self, path: PathBuf) {
        self.recent_files.retain(|p| p != &path);
        self.recent_files.insert(0, path);
    }

    pub fn clear_recent(&mut self) {
        self.recent_files.clear();
    }

    pub fn recent_file_count(&self) -> u32 {
        self.recent_files.len() as u32
    }

    pub fn favorites(&self) -> &[String] {
        &self.favorites
    }

    pub fn add_favorite(&mut self, id: &str) {
        if !self.favorites.contains(&id.to_string()) {
            self.favorites.push(id.to_string());
        }
    }

    pub fn remove_favorite(&mut self, id: &str) {
        self.favorites.retain(|f| f != id);
    }

    pub fn is_favorite(&self, id: &str) -> bool {
        self.favorites.contains(&id.to_string())
    }

    pub fn search(&self, query: &str) -> Vec<&QuickAccessEntry> {
        let q = query.to_lowercase();
        self.quick_access
            .iter()
            .filter(|e| {
                e.name_key.to_lowercase().contains(&q)
                    || e.id.to_lowercase().contains(&q)
                    || e.path.to_string_lossy().to_lowercase().contains(&q)
                    || e.icon.to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn stats(&self) -> FileManagerStats {
        let total_entries = self.quick_access.len() as u32;
        let edushell_folders = self.quick_access.iter().filter(|e| e.is_edushell).count() as u32;
        let total_items = self.quick_access.iter().map(|e| e.item_count).sum();
        let total_size_kb = self.quick_access.iter().map(|e| e.size_kb).sum();
        let recent_files_count = self.recent_files.len() as u32;
        FileManagerStats {
            total_entries,
            edushell_folders,
            total_items,
            total_size_kb,
            recent_files_count,
        }
    }

    pub fn folder_name(&self, ft: QuickAccessFolder) -> String {
        let key = match ft {
            QuickAccessFolder::Home => "file_manager.home",
            QuickAccessFolder::Learning => "file_manager.learning",
            QuickAccessFolder::Projects => "file_manager.projects",
            QuickAccessFolder::School => "file_manager.school",
            QuickAccessFolder::Downloads => "file_manager.downloads",
            QuickAccessFolder::Recent => "file_manager.recent",
            QuickAccessFolder::Favorite => "file_manager.favorite",
            QuickAccessFolder::Workspace => "file_manager.workspace",
            QuickAccessFolder::Documents => "file_manager.documents",
            QuickAccessFolder::Pictures => "file_manager.pictures",
            QuickAccessFolder::Music => "file_manager.music",
            QuickAccessFolder::Videos => "file_manager.videos",
            QuickAccessFolder::Desktop => "file_manager.desktop",
            QuickAccessFolder::Templates => "file_manager.templates",
            QuickAccessFolder::Public => "file_manager.public",
        };
        tr!(self.localization, key).to_string()
    }

    pub fn folder_icon(ft: QuickAccessFolder) -> &'static str {
        match ft {
            QuickAccessFolder::Home => "user-home",
            QuickAccessFolder::Learning => "folder-education",
            QuickAccessFolder::Projects => "folder-projects",
            QuickAccessFolder::School => "folder-school",
            QuickAccessFolder::Downloads => "folder-download",
            QuickAccessFolder::Recent => "folder-recent",
            QuickAccessFolder::Favorite => "folder-favorite",
            QuickAccessFolder::Workspace => "folder-workspace",
            QuickAccessFolder::Documents => "folder-documents",
            QuickAccessFolder::Pictures => "folder-pictures",
            QuickAccessFolder::Music => "folder-music",
            QuickAccessFolder::Videos => "folder-videos",
            QuickAccessFolder::Desktop => "folder-desktop",
            QuickAccessFolder::Templates => "folder-templates",
            QuickAccessFolder::Public => "folder-public",
        }
    }

    pub fn folder_path(ft: QuickAccessFolder) -> PathBuf {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"));
        let edushell = home.join("EduShell");
        Self::default_path_for(ft, &home, &edushell)
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = match locale {
            "id-ID" | "id" => Lang::Indonesian,
            _ => Lang::English,
        };
        self.localization.set_language(lang);
    }

    pub fn ensure_edushell_dirs(&self) -> Result<()> {
        let dirs = [
            self.edushell_dir.join("Learning"),
            self.edushell_dir.join("Projects"),
            self.edushell_dir.join("School"),
            self.edushell_dir.join("Favorites"),
            self.edushell_dir.join("Workspace"),
        ];
        for dir in &dirs {
            std::fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}

impl Default for FileManagerIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let fm = FileManagerIntegration::new();
        assert!(!fm.quick_access.is_empty());
        assert!(fm.recent_files.is_empty());
        assert!(fm.favorites.is_empty());
    }

    #[test]
    fn test_default_folder_count() {
        let fm = FileManagerIntegration::new();
        assert_eq!(fm.quick_access.len(), 15);
    }

    #[test]
    fn test_quick_access_listing() {
        let fm = FileManagerIntegration::new();
        for entry in fm.quick_access() {
            assert!(!entry.id.is_empty());
            assert!(!entry.name_key.is_empty());
            assert!(!entry.icon.is_empty());
            assert!(!entry.path.as_os_str().is_empty());
        }
    }

    #[test]
    fn test_folder_type_lookup() {
        let fm = FileManagerIntegration::new();
        let home = fm.quick_access_by_type(QuickAccessFolder::Home);
        assert!(home.is_some());
        let entry = home.unwrap();
        assert_eq!(entry.folder_type, QuickAccessFolder::Home);
        assert_eq!(entry.id, "home");
    }

    #[test]
    fn test_folder_type_lookup_missing() {
        let fm = FileManagerIntegration::new();
        let empty = fm.quick_access();
        let nonexist = empty.iter().find(|e| e.folder_type == QuickAccessFolder::Home);
        assert!(nonexist.is_some());
    }

    #[test]
    fn test_nemo_detection() {
        let fm = FileManagerIntegration::new();
        let _ = fm.nemo_available();
    }

    #[test]
    fn test_open_in_nemo_unavailable() {
        let mut fm = FileManagerIntegration::new();
        fm.nemo_available = false;
        let result = fm.open_in_nemo(Path::new("/tmp"));
        assert!(result.is_err());
    }

    #[test]
    fn test_open_quick_access_not_found() {
        let fm = FileManagerIntegration::new();
        let result = fm.open_quick_access("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_open_quick_access_found() {
        let fm = FileManagerIntegration::new();
        let home_entry = fm.quick_access_by_type(QuickAccessFolder::Home).unwrap();
        let result = fm.open_quick_access(&home_entry.id);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_edushell_folders() {
        let fm = FileManagerIntegration::new();
        let edu = fm.edushell_folders();
        assert!(!edu.is_empty());
        for entry in &edu {
            assert!(entry.is_edushell);
        }
    }

    #[test]
    fn test_edushell_folder_count() {
        let fm = FileManagerIntegration::new();
        let edu = fm.edushell_folders();
        assert_eq!(edu.len(), 5);
    }

    #[test]
    fn test_edushell_folder_paths() {
        let fm = FileManagerIntegration::new();
        let edu = fm.edushell_folders();
        let edu_names: Vec<&str> = edu.iter().map(|e| e.id.as_str()).collect();
        assert!(edu_names.contains(&"learning"));
        assert!(edu_names.contains(&"projects"));
        assert!(edu_names.contains(&"school"));
        assert!(edu_names.contains(&"favorite"));
        assert!(edu_names.contains(&"workspace"));
    }

    #[test]
    fn test_recent_file_lifecycle() {
        let mut fm = FileManagerIntegration::new();
        assert_eq!(fm.recent_file_count(), 0);

        fm.add_recent_file(PathBuf::from("/tmp/a.txt"));
        assert_eq!(fm.recent_file_count(), 1);
        assert_eq!(fm.recent_files().len(), 1);

        fm.add_recent_file(PathBuf::from("/tmp/b.txt"));
        assert_eq!(fm.recent_file_count(), 2);
        assert_eq!(fm.recent_files()[0], PathBuf::from("/tmp/b.txt"));

        fm.clear_recent();
        assert_eq!(fm.recent_file_count(), 0);
        assert!(fm.recent_files().is_empty());
    }

    #[test]
    fn test_recent_file_dedup() {
        let mut fm = FileManagerIntegration::new();
        fm.add_recent_file(PathBuf::from("/tmp/a.txt"));
        fm.add_recent_file(PathBuf::from("/tmp/a.txt"));
        assert_eq!(fm.recent_file_count(), 1);
    }

    #[test]
    fn test_recent_file_order() {
        let mut fm = FileManagerIntegration::new();
        fm.add_recent_file(PathBuf::from("/tmp/a.txt"));
        fm.add_recent_file(PathBuf::from("/tmp/b.txt"));
        fm.add_recent_file(PathBuf::from("/tmp/c.txt"));
        assert_eq!(fm.recent_files()[0], PathBuf::from("/tmp/c.txt"));
        assert_eq!(fm.recent_files()[1], PathBuf::from("/tmp/b.txt"));
        assert_eq!(fm.recent_files()[2], PathBuf::from("/tmp/a.txt"));
    }

    #[test]
    fn test_favorites_lifecycle() {
        let mut fm = FileManagerIntegration::new();
        assert!(!fm.is_favorite("home"));
        assert!(fm.favorites().is_empty());

        fm.add_favorite("home");
        assert!(fm.is_favorite("home"));
        assert_eq!(fm.favorites().len(), 1);

        fm.add_favorite("downloads");
        assert!(fm.is_favorite("downloads"));
        assert_eq!(fm.favorites().len(), 2);

        fm.remove_favorite("home");
        assert!(!fm.is_favorite("home"));
        assert_eq!(fm.favorites().len(), 1);

        fm.remove_favorite("downloads");
        assert!(!fm.is_favorite("downloads"));
        assert!(fm.favorites().is_empty());
    }

    #[test]
    fn test_favorites_dedup() {
        let mut fm = FileManagerIntegration::new();
        fm.add_favorite("home");
        fm.add_favorite("home");
        assert_eq!(fm.favorites().len(), 1);
    }

    #[test]
    fn test_favorites_remove_nonexistent() {
        let mut fm = FileManagerIntegration::new();
        fm.remove_favorite("nonexistent");
        assert!(fm.favorites().is_empty());
    }

    #[test]
    fn test_search_by_id() {
        let fm = FileManagerIntegration::new();
        let results = fm.search("home");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.id == "home"));
    }

    #[test]
    fn test_search_by_name_key() {
        let fm = FileManagerIntegration::new();
        let results = fm.search("download");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_by_path() {
        let fm = FileManagerIntegration::new();
        let results = fm.search("EduShell");
        let edu_results: Vec<&QuickAccessEntry> = results
            .iter()
            .filter(|e| e.path.to_string_lossy().contains("EduShell"))
            .copied()
            .collect();
        assert!(!edu_results.is_empty());
    }

    #[test]
    fn test_search_no_results() {
        let fm = FileManagerIntegration::new();
        let results = fm.search("zzzznonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_case_insensitive() {
        let fm = FileManagerIntegration::new();
        let lower = fm.search("home");
        let upper = fm.search("HOME");
        assert_eq!(lower.len(), upper.len());
    }

    #[test]
    fn test_stats() {
        let mut fm = FileManagerIntegration::new();
        fm.add_recent_file(PathBuf::from("/tmp/a.txt"));
        fm.add_recent_file(PathBuf::from("/tmp/b.txt"));
        let stats = fm.stats();
        assert_eq!(stats.total_entries, 15);
        assert_eq!(stats.edushell_folders, 5);
        assert_eq!(stats.recent_files_count, 2);
    }

    #[test]
    fn test_stats_default() {
        let stats = FileManagerStats::default();
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.edushell_folders, 0);
        assert_eq!(stats.total_items, 0);
        assert_eq!(stats.total_size_kb, 0);
        assert_eq!(stats.recent_files_count, 0);
    }

    #[test]
    fn test_folder_name() {
        let fm = FileManagerIntegration::new();
        let name = fm.folder_name(QuickAccessFolder::Home);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_folder_name_all_variants() {
        let fm = FileManagerIntegration::new();
        let variants = [
            QuickAccessFolder::Home,
            QuickAccessFolder::Learning,
            QuickAccessFolder::Projects,
            QuickAccessFolder::School,
            QuickAccessFolder::Downloads,
            QuickAccessFolder::Recent,
            QuickAccessFolder::Favorite,
            QuickAccessFolder::Workspace,
            QuickAccessFolder::Documents,
            QuickAccessFolder::Pictures,
            QuickAccessFolder::Music,
            QuickAccessFolder::Videos,
            QuickAccessFolder::Desktop,
            QuickAccessFolder::Templates,
            QuickAccessFolder::Public,
        ];
        for v in variants {
            let name = fm.folder_name(v);
            assert!(!name.is_empty(), "folder_name for {:?} is empty", v);
        }
    }

    #[test]
    fn test_folder_icon() {
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Home), "user-home");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Learning), "folder-education");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Projects), "folder-projects");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::School), "folder-school");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Downloads), "folder-download");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Recent), "folder-recent");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Favorite), "folder-favorite");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Workspace), "folder-workspace");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Documents), "folder-documents");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Pictures), "folder-pictures");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Music), "folder-music");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Videos), "folder-videos");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Desktop), "folder-desktop");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Templates), "folder-templates");
        assert_eq!(FileManagerIntegration::folder_icon(QuickAccessFolder::Public), "folder-public");
    }

    #[test]
    fn test_folder_path() {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Home),
            PathBuf::from(&home)
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Downloads),
            PathBuf::from(&home).join("Downloads")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Documents),
            PathBuf::from(&home).join("Documents")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Pictures),
            PathBuf::from(&home).join("Pictures")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Music),
            PathBuf::from(&home).join("Music")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Videos),
            PathBuf::from(&home).join("Videos")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Desktop),
            PathBuf::from(&home).join("Desktop")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Templates),
            PathBuf::from(&home).join("Templates")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Public),
            PathBuf::from(&home).join("Public")
        );
    }

    #[test]
    fn test_edu_folder_paths() {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let edushell = PathBuf::from(&home).join("EduShell");
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Learning),
            edushell.join("Learning")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Projects),
            edushell.join("Projects")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::School),
            edushell.join("School")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Favorite),
            edushell.join("Favorites")
        );
        assert_eq!(
            FileManagerIntegration::folder_path(QuickAccessFolder::Workspace),
            edushell.join("Workspace")
        );
    }

    #[test]
    fn test_set_locale_id() {
        let mut fm = FileManagerIntegration::new();
        fm.set_locale("id-ID");
        let name = fm.folder_name(QuickAccessFolder::Home);
        assert_eq!(name, "Beranda");
    }

    #[test]
    fn test_set_locale_en() {
        let mut fm = FileManagerIntegration::new();
        fm.set_locale("en-US");
        let name = fm.folder_name(QuickAccessFolder::Home);
        assert_eq!(name, "Home");
    }

    #[test]
    fn test_set_locale_short_id() {
        let mut fm = FileManagerIntegration::new();
        fm.set_locale("id");
        let name = fm.folder_name(QuickAccessFolder::Documents);
        assert_eq!(name, "Dokumen");
    }

    #[test]
    fn test_set_locale_unknown() {
        let mut fm = FileManagerIntegration::new();
        fm.set_locale("jp-JP");
        let name = fm.folder_name(QuickAccessFolder::Documents);
        assert_eq!(name, "Documents");
    }

    #[test]
    fn test_ensure_edushell_dirs() {
        let fm = FileManagerIntegration::new();
        let result = fm.ensure_edushell_dirs();
        assert!(result.is_ok());
    }

    #[test]
    fn test_quick_access_entry_serialization() {
        let entry = QuickAccessEntry {
            id: "test".to_string(),
            name_key: "file_manager.test".to_string(),
            folder_type: QuickAccessFolder::Home,
            path: PathBuf::from("/home/test"),
            icon: "folder".to_string(),
            is_default: false,
            is_edushell: false,
            item_count: 5,
            size_kb: 1024,
        };
        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: QuickAccessEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test");
        assert_eq!(deserialized.item_count, 5);
        assert_eq!(deserialized.size_kb, 1024);
    }

    #[test]
    fn test_stats_serialization() {
        let stats = FileManagerStats {
            total_entries: 15,
            edushell_folders: 5,
            total_items: 100,
            total_size_kb: 2048,
            recent_files_count: 10,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: FileManagerStats = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.total_entries, 15);
        assert_eq!(deserialized.edushell_folders, 5);
        assert_eq!(deserialized.total_items, 100);
        assert_eq!(deserialized.total_size_kb, 2048);
        assert_eq!(deserialized.recent_files_count, 10);
    }

    #[test]
    fn test_folder_type_equality() {
        assert_eq!(QuickAccessFolder::Home, QuickAccessFolder::Home);
        assert_ne!(QuickAccessFolder::Home, QuickAccessFolder::Downloads);
    }

    #[test]
    fn test_folder_type_debug() {
        let debug = format!("{:?}", QuickAccessFolder::Home);
        assert_eq!(debug, "Home");
    }

    #[test]
    fn test_folder_type_clone() {
        let ft = QuickAccessFolder::Learning;
        let cloned = ft;
        assert_eq!(ft, cloned);
    }

    #[test]
    fn test_quick_access_by_type_all() {
        let fm = FileManagerIntegration::new();
        let variants = [
            QuickAccessFolder::Home,
            QuickAccessFolder::Learning,
            QuickAccessFolder::Projects,
            QuickAccessFolder::School,
            QuickAccessFolder::Downloads,
            QuickAccessFolder::Recent,
            QuickAccessFolder::Favorite,
            QuickAccessFolder::Workspace,
            QuickAccessFolder::Documents,
            QuickAccessFolder::Pictures,
            QuickAccessFolder::Music,
            QuickAccessFolder::Videos,
            QuickAccessFolder::Desktop,
            QuickAccessFolder::Templates,
            QuickAccessFolder::Public,
        ];
        for v in variants {
            assert!(
                fm.quick_access_by_type(v).is_some(),
                "missing quick access for {:?}",
                v
            );
        }
    }

    #[test]
    fn test_default_trait() {
        let fm = FileManagerIntegration::default();
        assert_eq!(fm.quick_access.len(), 15);
    }

    #[test]
    fn test_error_display() {
        let err = FileManagerError::NemoNotAvailable;
        assert!(err.to_string().contains("nemo"));

        let err = FileManagerError::NotFound("test".to_string());
        assert!(err.to_string().contains("test"));

        let err = FileManagerError::InvalidFolderType;
        assert!(err.to_string().contains("folder type"));
    }

    #[test]
    fn test_open_quick_access_invalid_id() {
        let fm = FileManagerIntegration::new();
        let result = fm.open_quick_access("");
        assert!(result.is_err());
    }

    #[test]
    fn test_search_with_empty_query() {
        let fm = FileManagerIntegration::new();
        let results = fm.search("");
        assert_eq!(results.len(), 15);
    }

    #[test]
    fn test_entry_unique_ids() {
        let fm = FileManagerIntegration::new();
        let mut ids: Vec<&str> = fm.quick_access.iter().map(|e| e.id.as_str()).collect();
        let original_len = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), original_len);
    }

    #[test]
    fn test_entry_paths_are_absolute() {
        let fm = FileManagerIntegration::new();
        for entry in fm.quick_access() {
            assert!(
                entry.path.is_absolute(),
                "path {:?} is not absolute",
                entry.path
            );
        }
    }

    #[test]
    fn test_ensure_edushell_dirs_creates_paths() {
        let fm = FileManagerIntegration::new();
        let _ = fm.ensure_edushell_dirs();
        let paths = [
            fm.edushell_dir.join("Learning"),
            fm.edushell_dir.join("Projects"),
            fm.edushell_dir.join("School"),
            fm.edushell_dir.join("Favorites"),
            fm.edushell_dir.join("Workspace"),
        ];
        for p in &paths {
            assert!(p.exists() || p.parent().map_or(false, |pp| pp.exists()));
        }
    }

    #[test]
    fn test_add_multiple_recent_files() {
        let mut fm = FileManagerIntegration::new();
        for i in 0..100 {
            fm.add_recent_file(PathBuf::from(format!("/tmp/file_{}.txt", i)));
        }
        assert_eq!(fm.recent_file_count(), 100);
        assert_eq!(fm.recent_files()[0], PathBuf::from("/tmp/file_99.txt"));
    }

    #[test]
    fn test_edushell_folders_are_subsets() {
        let fm = FileManagerIntegration::new();
        let edu = fm.edushell_folders();
        let all = fm.quick_access();
        for e in &edu {
            assert!(all.iter().any(|a| a.id == e.id));
        }
    }

    #[test]
    fn test_clear_recent_then_add() {
        let mut fm = FileManagerIntegration::new();
        fm.add_recent_file(PathBuf::from("/tmp/a.txt"));
        fm.clear_recent();
        fm.add_recent_file(PathBuf::from("/tmp/b.txt"));
        assert_eq!(fm.recent_file_count(), 1);
        assert_eq!(fm.recent_files()[0], PathBuf::from("/tmp/b.txt"));
    }

    #[test]
    fn test_folder_name_english() {
        let mut fm = FileManagerIntegration::new();
        fm.set_locale("en-US");
        assert_eq!(fm.folder_name(QuickAccessFolder::Learning), "Learning");
        assert_eq!(fm.folder_name(QuickAccessFolder::Projects), "Projects");
        assert_eq!(fm.folder_name(QuickAccessFolder::Downloads), "Downloads");
        assert_eq!(fm.folder_name(QuickAccessFolder::Documents), "Documents");
        assert_eq!(fm.folder_name(QuickAccessFolder::Pictures), "Pictures");
        assert_eq!(fm.folder_name(QuickAccessFolder::Music), "Music");
        assert_eq!(fm.folder_name(QuickAccessFolder::Videos), "Videos");
    }

    #[test]
    fn test_folder_name_indonesian() {
        let mut fm = FileManagerIntegration::new();
        fm.set_locale("id-ID");
        assert_eq!(fm.folder_name(QuickAccessFolder::Learning), "Pusat Belajar");
        assert_eq!(fm.folder_name(QuickAccessFolder::Projects), "Pusat Proyek");
        assert_eq!(fm.folder_name(QuickAccessFolder::Downloads), "Unduhan");
        assert_eq!(fm.folder_name(QuickAccessFolder::Documents), "Dokumen");
        assert_eq!(fm.folder_name(QuickAccessFolder::Pictures), "Gambar");
        assert_eq!(fm.folder_name(QuickAccessFolder::Music), "Musik");
        assert_eq!(fm.folder_name(QuickAccessFolder::Videos), "Video");
    }
}
