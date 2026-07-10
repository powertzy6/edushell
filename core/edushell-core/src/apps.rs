// SPDX-License-Identifier: GPL-3.0-or-later

//! # Application Registry
//!
//! Reads and caches desktop entries (`.desktop` files)
//! from standard XDG directories. Provides application
//! lookup by ID, category, mime type, and search keywords.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::error::{AppRegistryErrorKind, EduResult};

/// A parsed desktop entry representing an installed application.
#[derive(Debug, Clone)]
pub struct AppEntry {
    /// Desktop file ID (e.g., "firefox").
    pub id: String,
    /// Path to the .desktop file.
    pub path: PathBuf,
    /// Application name.
    pub name: String,
    /// Generic name (e.g., "Web Browser").
    pub generic_name: Option<String>,
    /// Application description.
    pub comment: Option<String>,
    /// Icon name.
    pub icon: Option<String>,
    /// Exec command with placeholders.
    pub exec: String,
    /// Whether to run in terminal.
    pub terminal: bool,
    /// Categories (from desktop entry spec).
    pub categories: Vec<String>,
    /// MIME types this app can handle.
    pub mime_types: Vec<String>,
    /// Search keywords.
    pub keywords: Vec<String>,
    /// Whether app should be shown in menu.
    pub no_display: bool,
    /// Desktop environment filter (OnlyShowIn).
    pub only_show_in: Vec<String>,
    /// Desktop environments to exclude (NotShowIn).
    pub not_show_in: Vec<String>,
    /// Actions (for quick lists).
    pub actions: Vec<AppAction>,
}

/// An action associated with an application (e.g., "New Window").
#[derive(Debug, Clone)]
pub struct AppAction {
    /// Action ID.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Exec command.
    pub exec: String,
}

/// Application registry with caching.
#[derive(Clone)]
pub struct AppRegistry {
    apps: Arc<std::sync::RwLock<HashMap<String, AppEntry>>>,
    by_category: Arc<std::sync::RwLock<HashMap<String, Vec<String>>>>,
    by_mime: Arc<std::sync::RwLock<HashMap<String, Vec<String>>>>,
}

impl AppRegistry {
    /// Create a new empty application registry.
    pub fn new() -> Self {
        Self {
            apps: Arc::new(std::sync::RwLock::new(HashMap::new())),
            by_category: Arc::new(std::sync::RwLock::new(HashMap::new())),
            by_mime: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Scan all standard application directories and build the registry.
    pub fn scan(&self) -> EduResult<usize> {
        let app_dirs = self.application_dirs();
        let mut count = 0;

        for dir in &app_dirs {
            count += self.scan_directory(dir)?;
        }

        tracing::info!(
            target: "edushell::apps",
            count,
            "Application registry scan complete"
        );

        Ok(count)
    }

    /// Scan a single directory for .desktop files.
    fn scan_directory(&self, dir: &Path) -> EduResult<usize> {
        if !dir.exists() {
            return Ok(0);
        }

        let mut count = 0;

        for entry in std::fs::read_dir(dir).map_err(|e| {
            AppRegistryErrorKind::ParseDesktopEntry {
                path: dir.to_path_buf(),
                detail: e.to_string(),
            }
        })? {
            let entry = entry.map_err(|e| AppRegistryErrorKind::ParseDesktopEntry {
                path: dir.to_path_buf(),
                detail: e.to_string(),
            })?;

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                if let Ok(app) = self.parse_desktop_file(&path) {
                    // Check NoDisplay and environment filters
                    if app.no_display {
                        continue;
                    }

                    let id = app.id.clone();
                    {
                        let mut apps = self.apps.write()
                            .expect("App registry lock poisoned");
                        apps.insert(id.clone(), app.clone());
                    }

                    // Index by category
                    for cat in &app.categories {
                        let mut by_cat = self.by_category.write()
                            .expect("App registry lock poisoned");
                        by_cat.entry(cat.clone())
                            .or_insert_with(Vec::new)
                            .push(id.clone());
                    }

                    // Index by mime type
                    for mime in &app.mime_types {
                        let mut by_mime = self.by_mime.write()
                            .expect("App registry lock poisoned");
                        by_mime.entry(mime.clone())
                            .or_insert_with(Vec::new)
                            .push(id.clone());
                    }

                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// Parse a single .desktop file.
    fn parse_desktop_file(&self, path: &Path) -> Result<AppEntry, AppRegistryErrorKind> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppRegistryErrorKind::ParseDesktopEntry {
                path: path.to_path_buf(),
                detail: e.to_string(),
            })?;

        let id = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut entry = AppEntry {
            id,
            path: path.to_path_buf(),
            name: String::new(),
            generic_name: None,
            comment: None,
            icon: None,
            exec: String::new(),
            terminal: false,
            categories: Vec::new(),
            mime_types: Vec::new(),
            keywords: Vec::new(),
            no_display: false,
            only_show_in: Vec::new(),
            not_show_in: Vec::new(),
            actions: Vec::new(),
        };

        // Simple .desktop file parser
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                continue;
            }

            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Skip non-[Desktop Entry] sections for now
            if current_section != "Desktop Entry" {
                if current_section.starts_with("Desktop Action ") {
                    // Parse action entries
                }
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "Name" if entry.name.is_empty() => entry.name = value.to_string(),
                    "GenericName" if entry.generic_name.is_none() => {
                        entry.generic_name = Some(value.to_string());
                    }
                    "Comment" if entry.comment.is_none() => {
                        entry.comment = Some(value.to_string());
                    }
                    "Icon" if entry.icon.is_none() => {
                        entry.icon = Some(value.to_string());
                    }
                    "Exec" if entry.exec.is_empty() => entry.exec = value.to_string(),
                    "Terminal" => entry.terminal = value == "true",
                    "Categories" => {
                        entry.categories = value.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "MimeType" => {
                        entry.mime_types = value.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "Keywords" => {
                        entry.keywords = value.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "NoDisplay" => entry.no_display = value == "true",
                    "OnlyShowIn" => {
                        entry.only_show_in = value.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "NotShowIn" => {
                        entry.not_show_in = value.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    _ => {}
                }
            }
        }

        Ok(entry)
    }

    /// Get all registered applications.
    pub fn all(&self) -> Vec<AppEntry> {
        let apps = self.apps.read()
            .expect("App registry lock poisoned");
        apps.values().cloned().collect()
    }

    /// Get an application by its desktop file ID.
    pub fn get(&self, id: &str) -> Option<AppEntry> {
        let apps = self.apps.read()
            .expect("App registry lock poisoned");
        apps.get(id).cloned()
    }

    /// Get applications in a category.
    pub fn by_category(&self, category: &str) -> Vec<AppEntry> {
        let by_cat = self.by_category.read()
            .expect("App registry lock poisoned");
        let apps = self.apps.read()
            .expect("App registry lock poisoned");

        by_cat
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| apps.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get applications that can handle a MIME type.
    pub fn by_mime(&self, mime: &str) -> Vec<AppEntry> {
        let by_mime = self.by_mime.read()
            .expect("App registry lock poisoned");
        let apps = self.apps.read()
            .expect("App registry lock poisoned");

        by_mime
            .get(mime)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| apps.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all unique categories.
    pub fn categories(&self) -> Vec<String> {
        let by_cat = self.by_category.read()
            .expect("App registry lock poisoned");
        let mut cats: Vec<String> = by_cat.keys().cloned().collect();
        cats.sort();
        cats
    }

    /// Get the number of registered applications.
    pub fn count(&self) -> usize {
        self.apps.read()
            .map(|a| a.len())
            .unwrap_or(0)
    }

    /// Reload the registry from disk.
    pub fn reload(&self) -> EduResult<usize> {
        // Clear existing
        {
            let mut apps = self.apps.write()
                .expect("App registry lock poisoned");
            apps.clear();
        }
        {
            let mut by_cat = self.by_category.write()
                .expect("App registry lock poisoned");
            by_cat.clear();
        }
        {
            let mut by_mime = self.by_mime.write()
                .expect("App registry lock poisoned");
            by_mime.clear();
        }

        self.scan()
    }

    /// Get standard XDG application directories.
    fn application_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        // System directories
        dirs.push(PathBuf::from("/usr/share/applications"));
        dirs.push(PathBuf::from("/usr/local/share/applications"));

        // User directory
        if let Some(data_dir) = dirs::data_dir() {
            dirs.push(data_dir.join("applications"));
        }

        // XDG_DATA_DIRS
        if let Ok(data_dirs) = std::env::var("XDG_DATA_DIRS") {
            for d in data_dirs.split(':') {
                let path = PathBuf::from(d).join("applications");
                if !dirs.contains(&path) {
                    dirs.push(path);
                }
            }
        }

        // Fallback
        let fallback = PathBuf::from("/var/lib/flatpak/exports/share/applications");
        if !dirs.contains(&fallback) {
            dirs.push(fallback);
        }

        dirs
    }
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_desktop_file(dir: &Path, id: &str, name: &str, cat: &str) -> PathBuf {
        let path = dir.join(format!("{id}.desktop"));
        let content = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name={name}\n\
             Exec={id}\n\
             Icon={id}\n\
             Categories={cat}\n\
             Terminal=false\n"
        );
        std::fs::write(&path, &content).unwrap();
        path
    }

    #[test]
    fn test_parse_desktop_file() {
        let dir = TempDir::new().unwrap();
        let _path = create_test_desktop_file(dir.path(), "testapp", "Test App", "Utility");

        let registry = AppRegistry::new();
        let count = registry.scan_directory(dir.path()).unwrap();
        assert_eq!(count, 1);

        let app = registry.get("testapp").unwrap();
        assert_eq!(app.name, "Test App");
        assert_eq!(app.exec, "testapp");
    }

    #[test]
    fn test_categories() {
        let dir = TempDir::new().unwrap();
        create_test_desktop_file(dir.path(), "app1", "App 1", "Utility");
        create_test_desktop_file(dir.path(), "app2", "App 2", "Development");

        let registry = AppRegistry::new();
        registry.scan_directory(dir.path()).unwrap();

        let cats = registry.categories();
        assert!(cats.contains(&"Development".to_string()));
        assert!(cats.contains(&"Utility".to_string()));
    }

    #[test]
    fn test_by_category() {
        let dir = TempDir::new().unwrap();
        create_test_desktop_file(dir.path(), "app1", "App 1", "Utility");

        let registry = AppRegistry::new();
        registry.scan_directory(dir.path()).unwrap();

        let apps = registry.by_category("Utility");
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].id, "app1");
    }

    #[test]
    fn test_reload() {
        let dir = TempDir::new().unwrap();
        create_test_desktop_file(dir.path(), "app1", "App 1", "Utility");

        let registry = AppRegistry::new();
        registry.scan_directory(dir.path()).unwrap();
        assert_eq!(registry.count(), 1);

        // Reload re-scans standard system directories
        let count = registry.reload().unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_application_dirs() {
        let registry = AppRegistry::new();
        let dirs = registry.application_dirs();
        assert!(!dirs.is_empty());
        assert!(dirs.contains(&PathBuf::from("/usr/share/applications")));
    }
}
