// SPDX-License-Identifier: GPL-3.0-or-later

//! # Search Engine
//!
//! Full-text search across applications, settings, files,
//! and learning content. Uses prefix/fuzzy matching with
//! scoring and ranking.

use std::sync::Arc;

/// A single search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Title/name of the result.
    pub title: String,
    /// Description/subtitle.
    pub description: String,
    /// Category (apps, settings, files, learning).
    pub category: String,
    /// Icon name.
    pub icon: Option<String>,
    /// Relevance score (higher = better).
    pub score: f64,
    /// Action to execute when selected.
    pub action: Option<String>,
    /// Unique result ID.
    pub id: String,
}

/// Entry in the search index.
#[derive(Debug, Clone)]
struct IndexEntry {
    /// Terms extracted from the entry.
    terms: Vec<String>,
    /// Associated result data.
    result: SearchResult,
}

/// Search engine with index and query capabilities.
#[derive(Clone)]
pub struct SearchEngine {
    index: Arc<std::sync::RwLock<Vec<IndexEntry>>>,
    max_results: usize,
}

impl SearchEngine {
    /// Create a new search engine.
    pub fn new() -> Self {
        Self {
            index: Arc::new(std::sync::RwLock::new(Vec::new())),
            max_results: 20,
        }
    }

    /// Set maximum number of results per query.
    pub fn with_max_results(mut self, max: usize) -> Self {
        self.max_results = max;
        self
    }

    /// Index a new item for search.
    pub fn index(&self, id: &str, title: &str, description: &str, category: &str,
                 terms: Vec<String>, icon: Option<String>, action: Option<String>) {
        let entry = IndexEntry {
            terms,
            result: SearchResult {
                title: title.to_string(),
                description: description.to_string(),
                category: category.to_string(),
                icon,
                score: 0.0,
                action,
                id: id.to_string(),
            },
        };

        if let Ok(mut index) = self.index.write() {
            // Remove existing entry with same ID
            index.retain(|e| e.result.id != id);
            index.push(entry);
        }
    }

    /// Remove an item from the index.
    pub fn remove(&self, id: &str) {
        if let Ok(mut index) = self.index.write() {
            index.retain(|e| e.result.id != id);
        }
    }

    /// Query the search index.
    pub fn query(&self, query: &str) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

        if query_terms.is_empty() {
            return Vec::new();
        }

        let index = match self.index.read() {
            Ok(idx) => idx,
            Err(_) => return Vec::new(),
        };

        let mut scored: Vec<SearchResult> = index
            .iter()
            .filter_map(|entry| {
                let score = self.score_entry(entry, &query_terms, &query_lower);
                if score > 0.0 {
                    let mut result = entry.result.clone();
                    result.score = score;
                    Some(result)
                } else {
                    None
                }
            })
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        scored.truncate(self.max_results);
        scored
    }

    /// Score an entry against query terms.
    fn score_entry(&self, entry: &IndexEntry, query_terms: &[&str], query_lower: &str) -> f64 {
        let mut score = 0.0;
        let title_lower = entry.result.title.to_lowercase();
        let desc_lower = entry.result.description.to_lowercase();

        // Title exact match (highest weight)
        if title_lower == *query_lower {
            score += 100.0;
        }

        // Title prefix match
        if title_lower.starts_with(query_lower) {
            score += 50.0;
        }

        // Title contains query
        if title_lower.contains(query_lower) {
            score += 25.0;
        }

        // Match individual terms
        for term in query_terms {
            // Exact term match in title
            if title_lower.contains(term) {
                score += 10.0;
            }

            // Term match in description
            if desc_lower.contains(term) {
                score += 5.0;
            }

            // Term match in indexed terms
            for indexed_term in &entry.terms {
                if indexed_term.starts_with(term) {
                    score += 3.0;
                }
            }
        }

        score
    }

    /// Get suggested completions for a partial query.
    pub fn suggest(&self, partial: &str) -> Vec<String> {
        let partial_lower = partial.to_lowercase();
        let index = match self.index.read() {
            Ok(idx) => idx,
            Err(_) => return Vec::new(),
        };

        let mut suggestions: Vec<String> = index
            .iter()
            .filter_map(|entry| {
                let title = entry.result.title.to_lowercase();
                if title.starts_with(&partial_lower) && title != partial_lower {
                    Some(entry.result.title.clone())
                } else {
                    None
                }
            })
            .collect();

        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(10);
        suggestions
    }

    /// Clear the search index.
    pub fn clear(&self) {
        if let Ok(mut index) = self.index.write() {
            index.clear();
        }
    }

    /// Rebuild the entire index from scratch.
    pub fn reindex(&self, entries: Vec<(String, String, String, String, Vec<String>, Option<String>, Option<String>)>) {
        if let Ok(mut index) = self.index.write() {
            index.clear();
            for (id, title, description, category, terms, icon, action) in entries {
                index.push(IndexEntry {
                    terms,
                    result: SearchResult {
                        title,
                        description,
                        category,
                        icon,
                        score: 0.0,
                        action,
                        id,
                    },
                });
            }
        }
    }

    /// Get the number of indexed items.
    pub fn count(&self) -> usize {
        self.index.read().map(|i| i.len()).unwrap_or(0)
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_basic() {
        let engine = SearchEngine::new();
        engine.index("1", "Firefox", "Web Browser", "apps",
                      vec!["browser".into(), "web".into()], None, None);
        engine.index("2", "Terminal", "Command Line", "apps",
                      vec!["console".into(), "shell".into()], None, None);

        let results = engine.query("firefox");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Firefox");
    }

    #[test]
    fn test_search_ranking() {
        let engine = SearchEngine::new();
        engine.index("1", "Settings", "System Settings", "apps",
                      vec!["configuration".into()], None, None);
        engine.index("2", "System Monitor", "System Resources", "apps",
                      vec!["system".into(), "monitor".into()], None, None);

        let results = engine.query("system");
        assert!(results.len() >= 2);
        // "System Monitor" should rank higher for "system" prefix
        assert_eq!(results[0].title, "System Monitor");
    }

    #[test]
    fn test_suggestions() {
        let engine = SearchEngine::new();
        engine.index("1", "Firefox", "Browser", "apps", vec![], None, None);
        engine.index("2", "Files", "File Manager", "apps", vec![], None, None);

        let suggestions = engine.suggest("fi");
        assert!(suggestions.contains(&"Firefox".to_string()));
    }

    #[test]
    fn test_remove_from_index() {
        let engine = SearchEngine::new();
        engine.index("1", "Test", "Desc", "apps", vec![], None, None);
        assert_eq!(engine.count(), 1);

        engine.remove("1");
        assert_eq!(engine.count(), 0);
    }

    #[test]
    fn test_reindex() {
        let engine = SearchEngine::new();
        engine.reindex(vec![
            ("a".into(), "App A".into(), "Desc".into(), "apps".into(),
             vec![], None, None),
        ]);
        assert_eq!(engine.count(), 1);

        let results = engine.query("App A");
        assert_eq!(results.len(), 1);
    }
}
