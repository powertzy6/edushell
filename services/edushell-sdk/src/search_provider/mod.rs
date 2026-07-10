//! Search Provider SDK — add custom search sources to EduShell.
use serde::{Deserialize, Serialize};

/// A single search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub category: String,
    pub relevance: f64,
}

/// Search provider definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProviderDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub priority: u32,
}

/// Trait for implementing a custom search provider.
pub trait SearchProvider: Send + Sync {
    fn def(&self) -> SearchProviderDef;
    fn search(&self, query: &str, limit: usize) -> Vec<SearchResult>;
    fn search_sync(&self, query: &str) -> Vec<SearchResult> {
        self.search(query, 10)
    }
}

/// Built-in search provider: documentation lookup.
pub struct DocumentationSearch;

impl SearchProvider for DocumentationSearch {
    fn def(&self) -> SearchProviderDef {
        SearchProviderDef {
            id: "docs".into(),
            name: "Documentation".into(),
            description: "Search EduShell documentation".into(),
            icon: Some("book".into()),
            priority: 100,
        }
    }

    fn search(&self, _query: &str, _limit: usize) -> Vec<SearchResult> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_search_provider() {
        let p = DocumentationSearch;
        assert_eq!(p.def().id, "docs");
        assert_eq!(p.def().priority, 100);
    }

    #[test]
    fn test_search_sync() {
        let p = DocumentationSearch;
        let results = p.search_sync("rust");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_result_serde() {
        let r = SearchResult {
            id: "1".into(),
            title: "Result".into(),
            subtitle: "Desc".into(),
            url: Some("https://example.com".into()),
            icon: None,
            category: "web".into(),
            relevance: 0.9,
        };
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: SearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.title, "Result");
    }

    #[test]
    fn test_search_provider_def() {
        let def = SearchProviderDef {
            id: "github".into(),
            name: "GitHub".into(),
            description: "Search GitHub".into(),
            icon: Some("github".into()),
            priority: 50,
        };
        assert_eq!(format!("{:?}", def.id), "\"github\"");
    }
}
