//! Resource Engine — manage themes, fonts, icons, SVGs, wallpapers, cursors.
use std::collections::HashMap;

/// Resource type.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Theme,
    Font,
    Icon,
    Svg,
    Wallpaper,
    Cursor,
    Localization,
    Animation,
}

/// Resource metadata.
#[derive(Debug, Clone)]
pub struct ResourceMeta {
    pub id: String,
    pub resource_type: ResourceType,
    pub name: String,
    pub version: String,
    pub author: String,
    pub path: String,
    pub size_kb: u64,
    pub checksum: String,
}

/// Resource engine.
pub struct ResourceEngine {
    resources: HashMap<String, ResourceMeta>,
    cache_enabled: bool,
    cache_size_kb: u64,
}

impl ResourceEngine {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            cache_enabled: true,
            cache_size_kb: 0,
        }
    }

    pub fn register(&mut self, meta: ResourceMeta) {
        self.cache_size_kb += meta.size_kb;
        self.resources.insert(meta.id.clone(), meta);
    }

    pub fn get(&self, id: &str) -> Option<&ResourceMeta> {
        self.resources.get(id)
    }

    pub fn by_type(&self, resource_type: &ResourceType) -> Vec<&ResourceMeta> {
        self.resources
            .values()
            .filter(|r| r.resource_type == *resource_type)
            .collect()
    }

    pub fn all(&self) -> Vec<&ResourceMeta> {
        self.resources.values().collect()
    }

    pub fn remove(&mut self, id: &str) -> bool {
        if let Some(meta) = self.resources.remove(id) {
            self.cache_size_kb = self.cache_size_kb.saturating_sub(meta.size_kb);
            true
        } else {
            false
        }
    }

    pub fn clear_cache(&mut self) {
        self.resources.clear();
        self.cache_size_kb = 0;
    }

    pub fn total_size_kb(&self) -> u64 {
        self.cache_size_kb
    }
    pub fn count(&self) -> usize {
        self.resources.len()
    }
    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }
}

impl Default for ResourceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_meta(id: &str, rtype: ResourceType) -> ResourceMeta {
        ResourceMeta {
            id: id.into(),
            resource_type: rtype,
            name: id.into(),
            version: "1.0".into(),
            author: "test".into(),
            path: format!("/tmp/{}", id),
            size_kb: 100,
            checksum: "abc".into(),
        }
    }

    #[test]
    fn test_resource_engine_new() {
        let re = ResourceEngine::new();
        assert_eq!(re.count(), 0);
        assert!(re.is_cache_enabled());
    }

    #[test]
    fn test_register_and_get() {
        let mut re = ResourceEngine::new();
        let meta = sample_meta("theme-dark", ResourceType::Theme);
        re.register(meta);
        assert_eq!(re.count(), 1);
        assert!(re.get("theme-dark").is_some());
    }

    #[test]
    fn test_by_type() {
        let mut re = ResourceEngine::new();
        re.register(sample_meta("a", ResourceType::Theme));
        re.register(sample_meta("b", ResourceType::Font));
        re.register(sample_meta("c", ResourceType::Theme));
        assert_eq!(re.by_type(&ResourceType::Theme).len(), 2);
        assert_eq!(re.by_type(&ResourceType::Font).len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut re = ResourceEngine::new();
        re.register(sample_meta("x", ResourceType::Icon));
        assert!(re.remove("x"));
        assert!(!re.remove("x"));
    }

    #[test]
    fn test_clear_cache() {
        let mut re = ResourceEngine::new();
        re.register(sample_meta("a", ResourceType::Theme));
        re.register(sample_meta("b", ResourceType::Font));
        re.clear_cache();
        assert_eq!(re.count(), 0);
        assert_eq!(re.total_size_kb(), 0);
    }

    #[test]
    fn test_size_tracking() {
        let mut re = ResourceEngine::new();
        re.register(sample_meta("big", ResourceType::Wallpaper));
        assert_eq!(re.total_size_kb(), 100);
    }

    #[test]
    fn test_cache_toggle() {
        let mut re = ResourceEngine::new();
        re.set_cache_enabled(false);
        assert!(!re.is_cache_enabled());
    }

    #[test]
    fn test_resource_type_variant() {
        assert_eq!(format!("{:?}", ResourceType::Cursor), "Cursor");
    }
}
