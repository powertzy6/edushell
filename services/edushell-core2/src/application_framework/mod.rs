//! Application Framework — scaffolding, templates, project generation.
use serde::{Deserialize, Serialize};

/// Application template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub framework: String,
    pub files: Vec<String>,
}

/// Generated application metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub template_id: String,
    pub created_at: String,
}

/// Application framework.
pub struct ApplicationFramework {
    templates: Vec<AppTemplate>,
}

impl ApplicationFramework {
    pub fn new() -> Self {
        Self {
            templates: Self::default_templates(),
        }
    }

    pub fn templates(&self) -> &[AppTemplate] {
        &self.templates
    }

    pub fn find(&self, id: &str) -> Option<&AppTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }

    pub fn scaffold(&self, template_id: &str, name: &str) -> AppMetadata {
        AppMetadata {
            id: crate::core::new_id(),
            name: name.to_string(),
            version: "1.0.0".into(),
            author: "EduShell Developer".into(),
            template_id: template_id.to_string(),
            created_at: crate::core::now_iso(),
        }
    }

    fn default_templates() -> Vec<AppTemplate> {
        vec![
            AppTemplate {
                id: "gtk4-rust".into(),
                name: "GTK4 + Rust".into(),
                description: "GTK4 application with Rust".into(),
                language: "rust".into(),
                framework: "gtk4".into(),
                files: vec![
                    "Cargo.toml".into(),
                    "src/main.rs".into(),
                    "src/config.rs".into(),
                ],
            },
            AppTemplate {
                id: "gtk4-python".into(),
                name: "GTK4 + Python".into(),
                description: "GTK4 application with Python".into(),
                language: "python".into(),
                framework: "gtk4".into(),
                files: vec!["main.py".into(), "config.py".into()],
            },
            AppTemplate {
                id: "edushell-plugin".into(),
                name: "EduShell Plugin".into(),
                description: "EduShell plugin in Rust".into(),
                language: "rust".into(),
                framework: "edushell-sdk".into(),
                files: vec![
                    "Cargo.toml".into(),
                    "src/lib.rs".into(),
                    "manifest.json".into(),
                ],
            },
        ]
    }
}

impl Default for ApplicationFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_new() {
        let af = ApplicationFramework::new();
        assert_eq!(af.templates().len(), 3);
    }

    #[test]
    fn test_find_template() {
        let af = ApplicationFramework::new();
        let t = af.find("gtk4-rust");
        assert!(t.is_some());
        assert_eq!(t.unwrap().language, "rust");
    }

    #[test]
    fn test_find_nonexistent() {
        let af = ApplicationFramework::new();
        assert!(af.find("nonexistent").is_none());
    }

    #[test]
    fn test_scaffold() {
        let af = ApplicationFramework::new();
        let meta = af.scaffold("edushell-plugin", "MyPlugin");
        assert_eq!(meta.name, "MyPlugin");
        assert_eq!(meta.template_id, "edushell-plugin");
    }

    #[test]
    fn test_template_files() {
        let af = ApplicationFramework::new();
        let t = af.find("edushell-plugin").unwrap();
        assert!(t.files.contains(&"manifest.json".to_string()));
    }
}
