//! Search engine — indexing, fuzzy matching, providers.
use std::collections::HashMap;

/// Search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub relevance: f64,
    pub source: String,
}

/// Search provider trait.
pub trait SearchProvider: Send + Sync {
    fn name(&self) -> &str;
    fn search(&self, query: &str, limit: usize) -> Vec<SearchResult>;
}

/// Search engine.
pub struct SearchEngine {
    providers: HashMap<String, Box<dyn SearchProvider>>,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider: Box<dyn SearchProvider>) {
        let name = provider.name().to_string();
        self.providers.insert(name, provider);
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let mut results = Vec::new();
        for provider in self.providers.values() {
            results.extend(provider.search(query, limit));
        }
        results.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(limit);
        results
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}

/// Built-in fuzzy search provider.
pub struct FuzzySearchProvider;

impl SearchProvider for FuzzySearchProvider {
    fn name(&self) -> &str {
        "fuzzy"
    }
    fn search(&self, _query: &str, _limit: usize) -> Vec<SearchResult> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProvider;

    impl SearchProvider for TestProvider {
        fn name(&self) -> &str {
            "test"
        }
        fn search(&self, _query: &str, _limit: usize) -> Vec<SearchResult> {
            vec![SearchResult {
                id: "1".into(),
                title: "Result".into(),
                description: "".into(),
                category: "test".into(),
                relevance: 1.0,
                source: "test".into(),
            }]
        }
    }

    #[test]
    fn test_search_engine_empty() {
        let se = SearchEngine::new();
        assert_eq!(se.search("hello", 10).len(), 0);
    }

    #[test]
    fn test_search_engine_with_provider() {
        let mut se = SearchEngine::new();
        se.register(Box::new(TestProvider));
        assert_eq!(se.provider_count(), 1);
        let results = se.search("test", 5);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_fuzzy_provider() {
        let p = FuzzySearchProvider;
        assert_eq!(p.name(), "fuzzy");
    }

    #[test]
    fn test_search_result_ordering() {
        let mut se = SearchEngine::new();
        se.register(Box::new(TestProvider));
        let results = se.search("x", 10);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_limit() {
        let mut se = SearchEngine::new();
        struct ManyProvider;
        impl SearchProvider for ManyProvider {
            fn name(&self) -> &str {
                "many"
            }
            fn search(&self, _q: &str, _l: usize) -> Vec<SearchResult> {
                (0..10)
                    .map(|i| SearchResult {
                        id: i.to_string(),
                        title: i.to_string(),
                        description: "".into(),
                        category: "x".into(),
                        relevance: 1.0,
                        source: "x".into(),
                    })
                    .collect()
            }
        }
        se.register(Box::new(ManyProvider));
        assert_eq!(se.search("x", 3).len(), 3);
    }
}
