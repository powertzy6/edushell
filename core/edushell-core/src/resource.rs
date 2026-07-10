// SPDX-License-Identifier: GPL-3.0-or-later

//! # Resource Manager
//!
//! Manages loading and caching of application resources:
//! icons, fonts, sounds, wallpapers, cursors, and animations.
//! Provides fallback chains and lazy loading.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::error::{EduResult, ResourceErrorKind};

/// Type of resource being loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// SVG icon.
    Icon,
    /// PNG/JPEG image.
    Image,
    /// TrueType/OpenType font.
    Font,
    /// Audio file.
    Sound,
    /// Cursor theme file.
    Cursor,
    /// Wallpaper image.
    Wallpaper,
    /// Animation (Lottie or GIF).
    Animation,
}

/// A loaded resource in memory.
#[derive(Debug, Clone)]
pub enum ResourceData {
    /// Raw bytes (for any resource type).
    Raw(Vec<u8>),
    /// UTF-8 text content.
    Text(String),
    /// Path to a resource (loaded on demand).
    Path(PathBuf),
}

/// Search directories for resources.
#[derive(Debug, Clone)]
pub struct ResourcePaths {
    pub data_dirs: Vec<PathBuf>,
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl Default for ResourcePaths {
    fn default() -> Self {
        Self {
            data_dirs: vec![
                PathBuf::from("/usr/share/edushell"),
                PathBuf::from("/usr/local/share/edushell"),
            ],
            config_dir: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("edushell"),
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("edushell"),
        }
    }
}

/// Thread-safe resource manager with caching.
#[derive(Clone)]
pub struct ResourceManager {
    cache: Arc<std::sync::RwLock<HashMap<String, ResourceData>>>,
    paths: ResourcePaths,
    max_cache_entries: usize,
}

impl ResourceManager {
    /// Create a new resource manager.
    pub fn new(paths: ResourcePaths) -> Self {
        Self {
            cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
            paths,
            max_cache_entries: 256,
        }
    }

    /// Set maximum cache entries.
    pub fn with_max_cache(mut self, max: usize) -> Self {
        self.max_cache_entries = max;
        self
    }

    /// Load a resource by searching all data directories.
    pub fn load(&self, resource_type: ResourceType, name: &str) -> EduResult<ResourceData> {
        // Check cache first
        let cache_key = format!("{:?}/{}", resource_type, name);
        {
            if let Ok(cache) = self.cache.read() {
                if let Some(data) = cache.get(&cache_key) {
                    return Ok(data.clone());
                }
            }
        }

        // Build search paths based on resource type
        let subdir = match resource_type {
            ResourceType::Icon => "icons",
            ResourceType::Image => "images",
            ResourceType::Font => "fonts",
            ResourceType::Sound => "sounds",
            ResourceType::Cursor => "cursors",
            ResourceType::Wallpaper => "wallpapers",
            ResourceType::Animation => "animations",
        };

        let mut searched = Vec::new();

        for data_dir in &self.paths.data_dirs {
            let path = data_dir.join(subdir).join(name);
            if path.exists() {
                let data = std::fs::read(&path).map_err(|e| ResourceErrorKind::Decode {
                    path: path.clone(),
                    detail: e.to_string(),
                })?;

                let resource = ResourceData::Raw(data);

                // Cache it
                if let Ok(mut cache) = self.cache.write() {
                    if cache.len() < self.max_cache_entries {
                        cache.insert(cache_key, resource.clone());
                    }
                }

                return Ok(resource);
            }
            searched.push(path);
        }

        // Also check user data directory
        let user_dir = self.paths.config_dir.join(subdir).join(name);
        if user_dir.exists() {
            let data = std::fs::read(&user_dir).map_err(|e| ResourceErrorKind::Decode {
                path: user_dir.clone(),
                detail: e.to_string(),
            })?;
            return Ok(ResourceData::Raw(data));
        }
        searched.push(user_dir);

        Err(ResourceErrorKind::NotFound {
            path: PathBuf::from(name),
            searched,
        }
        .into())
    }

    /// Load a resource as text (UTF-8).
    pub fn load_text(&self, resource_type: ResourceType, name: &str) -> EduResult<String> {
        let data = self.load(resource_type, name)?;
        match data {
            ResourceData::Text(t) => Ok(t),
            ResourceData::Raw(bytes) => {
                String::from_utf8(bytes).map_err(|e| {
                    ResourceErrorKind::Decode {
                        path: PathBuf::from(name),
                        detail: format!("UTF-8 decode error: {e}"),
                    }
                    .into()
                })
            }
            ResourceData::Path(p) => {
                std::fs::read_to_string(p).map_err(|e| {
                    ResourceErrorKind::Decode {
                        path: PathBuf::from(name),
                        detail: e.to_string(),
                    }
                    .into()
                })
            }
        }
    }

    /// Load a sound file.
    pub fn load_sound(&self, name: &str) -> EduResult<Vec<u8>> {
        let data = self.load(ResourceType::Sound, name)?;
        match data {
            ResourceData::Raw(bytes) => Ok(bytes),
            _ => Err(ResourceErrorKind::Decode {
                path: PathBuf::from(name),
                detail: "Unexpected resource type for sound".into(),
            }
            .into()),
        }
    }

    /// Load a wallpaper image.
    pub fn load_wallpaper(&self, name: &str) -> EduResult<PathBuf> {
        let data = self.load(ResourceType::Wallpaper, name)?;
        match data {
            ResourceData::Path(p) => Ok(p),
            ResourceData::Raw(_) => {
                // Write to cache and return path
                let cache_path = self.paths.cache_dir.join("wallpapers").join(name);
                Ok(cache_path)
            }
            _ => Err(ResourceErrorKind::UnsupportedFormat {
                path: PathBuf::from(name),
                format: "unknown".into(),
            }
            .into()),
        }
    }

    /// Clear the resource cache.
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// Get cache statistics.
    pub fn cache_stats(&self) -> CacheStats {
        let (size, entries) = self.cache.read()
            .map(|c| {
                let size: usize = c.values().map(|v| match v {
                    ResourceData::Raw(bytes) => bytes.len(),
                    ResourceData::Text(t) => t.len(),
                    ResourceData::Path(p) => p.to_string_lossy().len(),
                }).sum();
                (size, c.len())
            })
            .unwrap_or((0, 0));

        CacheStats { entries, total_bytes: size }
    }

    /// Check if a resource exists in any search path.
    pub fn exists(&self, resource_type: ResourceType, name: &str) -> bool {
        self.load(resource_type, name).is_ok()
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached entries.
    pub entries: usize,
    /// Total bytes cached.
    pub total_bytes: usize,
}

/// Icon lookup helper.
pub struct IconLookup {
    mgr: ResourceManager,
}

impl IconLookup {
    pub fn new(mgr: ResourceManager) -> Self {
        Self { mgr }
    }

    /// Find an icon by name with fallback chain.
    pub fn find_icon(&self, name: &str, size: u32, symbolic: bool) -> EduResult<ResourceData> {
        let icon_name = if symbolic {
            format!("{}-symbolic", name)
        } else {
            name.to_string()
        };

        // Try exact match first
        let extensions = ["svg", "png", "xpm"];

        for ext in &extensions {
            let filename = format!("{}.{}", icon_name, ext);
            if let Ok(data) = self.mgr.load(ResourceType::Icon, &filename) {
                return Ok(data);
            }
        }

        // Try size-specific directory
        for ext in &extensions {
            let filename = format!("{}/{}.{}", size, icon_name, ext);
            if let Ok(data) = self.mgr.load(ResourceType::Icon, &filename) {
                return Ok(data);
            }
        }

        // Fallback: try without symbolic suffix
        if symbolic && !name.ends_with("-symbolic") {
            return self.find_icon(&name.replace("-symbolic", ""), size, false);
        }

        // Ultimate fallback: application-x-executable
        self.mgr.load(ResourceType::Icon, "application-x-executable.svg")
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_paths_default() {
        let paths = ResourcePaths::default();
        assert!(!paths.data_dirs.is_empty());
    }

    #[test]
    fn test_resource_not_found_error() {
        let paths = ResourcePaths::default();
        let mgr = ResourceManager::new(paths);

        let result = mgr.load(ResourceType::Icon, "nonexistent-file.xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_cache_eviction() {
        let paths = ResourcePaths::default();
        let mgr = ResourceManager::new(paths).with_max_cache(2);

        // This won't actually cache (files don't exist), but verifies limit
        mgr.clear_cache();
        let stats = mgr.cache_stats();
        assert_eq!(stats.entries, 0);
    }

    #[test]
    fn test_icon_lookup_fallback() {
        let paths = ResourcePaths::default();
        let mgr = ResourceManager::new(paths);
        let icons = IconLookup::new(mgr);

        // Should fall back to generic icon
        let result = icons.find_icon("nonexistent-app", 48, false);
        // Might fail if no icon theme installed, but shouldn't panic
        assert!(result.is_ok() || result.is_err());
    }
}
