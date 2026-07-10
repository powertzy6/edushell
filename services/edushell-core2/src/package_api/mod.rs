//! Package API — package metadata, versioning, dependency resolution.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Package identifier.
pub type PackageId = String;

/// Package type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PackageType {
    Theme,
    Plugin,
    Extension,
    Widget,
    IconPack,
    WallpaperPack,
    Localization,
    Application,
}

/// Package dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    pub id: PackageId,
    pub version: String,
    pub optional: bool,
}

/// Package metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMeta {
    pub id: PackageId,
    pub name: String,
    pub version: String,
    pub package_type: PackageType,
    pub author: String,
    pub description: String,
    pub min_core_version: String,
    pub dependencies: Vec<PackageDependency>,
    pub checksum: Option<String>,
    pub size_kb: u64,
    pub homepage: Option<String>,
    pub repository: Option<String>,
}

/// Package registry.
pub struct PackageRegistry {
    packages: HashMap<PackageId, PackageMeta>,
}

impl PackageRegistry {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn register(&mut self, meta: PackageMeta) -> bool {
        if self.packages.contains_key(&meta.id) {
            return false;
        }
        self.packages.insert(meta.id.clone(), meta);
        true
    }

    pub fn get(&self, id: &str) -> Option<&PackageMeta> {
        self.packages.get(id)
    }

    pub fn search(&self, query: &str) -> Vec<&PackageMeta> {
        let q = query.to_lowercase();
        self.packages
            .values()
            .filter(|p| p.name.to_lowercase().contains(&q) || p.id.to_lowercase().contains(&q))
            .collect()
    }

    pub fn by_type(&self, package_type: &PackageType) -> Vec<&PackageMeta> {
        self.packages
            .values()
            .filter(|p| p.package_type == *package_type)
            .collect()
    }

    pub fn all(&self) -> Vec<&PackageMeta> {
        self.packages.values().collect()
    }

    pub fn count(&self) -> usize {
        self.packages.len()
    }
}

impl Default for PackageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_meta(id: &str) -> PackageMeta {
        PackageMeta {
            id: id.into(),
            name: id.into(),
            version: "1.0".into(),
            package_type: PackageType::Plugin,
            author: "test".into(),
            description: "".into(),
            min_core_version: "2.0".into(),
            dependencies: vec![],
            checksum: None,
            size_kb: 50,
            homepage: None,
            repository: None,
        }
    }

    #[test]
    fn test_registry_new() {
        let reg = PackageRegistry::new();
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn test_register_and_get() {
        let mut reg = PackageRegistry::new();
        assert!(reg.register(sample_meta("theme-dark")));
        assert!(reg.get("theme-dark").is_some());
    }

    #[test]
    fn test_register_duplicate() {
        let mut reg = PackageRegistry::new();
        reg.register(sample_meta("pkg-1"));
        assert!(!reg.register(sample_meta("pkg-1")));
    }

    #[test]
    fn test_search() {
        let mut reg = PackageRegistry::new();
        reg.register(sample_meta("theme-dark"));
        reg.register(sample_meta("icon-set"));
        assert_eq!(reg.search("dark").len(), 1);
        assert_eq!(reg.search("icon").len(), 1);
    }

    #[test]
    fn test_by_type() {
        let mut reg = PackageRegistry::new();
        let mut m = sample_meta("moon");
        m.package_type = PackageType::Theme;
        reg.register(m);
        reg.register(sample_meta("plug"));
        assert_eq!(reg.by_type(&PackageType::Theme).len(), 1);
        assert_eq!(reg.by_type(&PackageType::Plugin).len(), 1);
    }

    #[test]
    fn test_package_type_variants() {
        assert_eq!(format!("{:?}", PackageType::IconPack), "IconPack");
        assert_eq!(format!("{:?}", PackageType::Localization), "Localization");
    }

    #[test]
    fn test_package_meta_serde() {
        let m = sample_meta("test-pkg");
        let json = serde_json::to_string(&m).unwrap();
        let d: PackageMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(d.id, "test-pkg");
    }
}
