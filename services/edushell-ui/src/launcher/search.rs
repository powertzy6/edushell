// SPDX-License-Identifier: GPL-3.0-or-later

//! # Launcher Search
//!
//! Search bar component for the application launcher.
//! Supports fuzzy matching, keyword search, and
//! keyboard navigation.

use crate::localization::LocalizationManager;

pub struct LauncherSearch {
    placeholder: String,
    query: String,
    localization: LocalizationManager,
    #[cfg(feature = "gtk")]
    entry: Option<gtk::SearchEntry>,
}

impl LauncherSearch {
    pub fn new(localization: LocalizationManager) -> Self {
        Self {
            placeholder: localization.translate("Search applications..."),
            query: String::new(),
            localization,
            #[cfg(feature = "gtk")]
            entry: None,
        }
    }

    pub fn build(&self) {
        #[cfg(feature = "gtk")]
        if let Some(ref entry) = self.entry {
            entry.set_placeholder_text(Some(&self.placeholder));
        }
    }

    pub fn set_query(&mut self, query: &str) {
        self.query = query.to_string();
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn clear(&mut self) {
        self.query.clear();
        #[cfg(feature = "gtk")]
        if let Some(ref entry) = self.entry {
            entry.set_text("");
        }
    }

    pub fn focus(&self) {
        #[cfg(feature = "gtk")]
        if let Some(ref entry) = self.entry {
            entry.grab_focus();
        }
    }

    pub fn set_placeholder(&mut self, text: &str) {
        self.placeholder = text.to_string();
        #[cfg(feature = "gtk")]
        if let Some(ref entry) = self.entry {
            entry.set_placeholder_text(Some(text));
        }
    }

    /// Simple fuzzy match score between query and text.
    ///
    /// Returns a score where:
    /// - Exact match: 1000
    /// - Prefix match: 500 + length
    /// - Substring match: 300 + length
    /// - Fuzzy match: number of matched characters * 10
    /// - No match: 0
    pub fn fuzzy_score(query: &str, text: &str) -> i32 {
        if query.is_empty() || text.is_empty() {
            return 0;
        }
        let q = query.to_lowercase();
        let t = text.to_lowercase();

        if q == t {
            return 1000;
        }
        if t.starts_with(&q) {
            return 500 + q.len() as i32;
        }
        if t.contains(&q) {
            return 300 + q.len() as i32;
        }

        // Fuzzy character-by-character matching
        let q_chars: Vec<char> = q.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let mut qi = 0;
        let mut matched = 0;
        for &tc in &t_chars {
            if qi < q_chars.len() && tc == q_chars[qi] {
                matched += 1;
                qi += 1;
            }
        }
        if qi == q_chars.len() {
            matched * 10
        } else {
            0
        }
    }

    /// Check if query matches text (fuzzy).
    pub fn fuzzy_match(query: &str, text: &str) -> bool {
        Self::fuzzy_score(query, text) > 0
    }

    /// Rank results by relevance.
    ///
    /// Returns a vector of `(index, score)` pairs sorted descending by score.
    /// Each item is a tuple of `(name, description)`.
    pub fn rank_results(query: &str, items: &[(&str, &str)]) -> Vec<(usize, i32)> {
        let mut scored: Vec<(usize, i32)> = items
            .iter()
            .enumerate()
            .map(|(i, (name, desc))| {
                let name_score = Self::fuzzy_score(query, name);
                let desc_score = Self::fuzzy_score(query, desc);
                (i, name_score.max(desc_score))
            })
            .filter(|(_, score)| *score > 0)
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored
    }
}

impl std::fmt::Debug for LauncherSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LauncherSearch")
            .field("placeholder", &self.placeholder)
            .field("query", &self.query)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::localization::LocalizationManager;

    fn make_locale() -> LocalizationManager {
        LocalizationManager::new("en")
    }

    #[test]
    fn test_new_creates_empty_search() {
        let search = LauncherSearch::new(make_locale());
        assert!(search.query().is_empty());
        assert!(!search.placeholder.is_empty());
    }

    #[test]
    fn test_set_query() {
        let mut search = LauncherSearch::new(make_locale());
        search.set_query("firefox");
        assert_eq!(search.query(), "firefox");
    }

    #[test]
    fn test_clear() {
        let mut search = LauncherSearch::new(make_locale());
        search.set_query("test");
        assert!(!search.query().is_empty());
        search.clear();
        assert!(search.query().is_empty());
    }

    #[test]
    fn test_set_placeholder() {
        let mut search = LauncherSearch::new(make_locale());
        search.set_placeholder("Type to search...");
        assert_eq!(search.placeholder, "Type to search...");
    }

    #[test]
    fn test_fuzzy_score_exact_match() {
        assert_eq!(LauncherSearch::fuzzy_score("firefox", "firefox"), 1000);
    }

    #[test]
    fn test_fuzzy_score_prefix_match() {
        let score = LauncherSearch::fuzzy_score("fire", "firefox");
        assert!(score >= 500 && score < 1000);
    }

    #[test]
    fn test_fuzzy_score_substring_match() {
        let score = LauncherSearch::fuzzy_score("fox", "firefox");
        assert!(score >= 300 && score < 500);
    }

    #[test]
    fn test_fuzzy_score_fuzzy_match() {
        let score = LauncherSearch::fuzzy_score("ff", "firefox");
        assert!(score > 0 && score < 300);
    }

    #[test]
    fn test_fuzzy_score_no_match() {
        assert_eq!(LauncherSearch::fuzzy_score("xyz", "firefox"), 0);
    }

    #[test]
    fn test_fuzzy_score_empty() {
        assert_eq!(LauncherSearch::fuzzy_score("", "firefox"), 0);
        assert_eq!(LauncherSearch::fuzzy_score("firefox", ""), 0);
    }

    #[test]
    fn test_fuzzy_match() {
        assert!(LauncherSearch::fuzzy_match("ff", "firefox"));
        assert!(LauncherSearch::fuzzy_match("fx", "firefox"));
        assert!(LauncherSearch::fuzzy_match("firefox", "firefox"));
        assert!(!LauncherSearch::fuzzy_match("xyz", "firefox"));
    }

    #[test]
    fn test_rank_results_orders_by_relevance() {
        let items = [
            ("Firefox", "Web Browser"),
            ("Thunderbird", "Email Client"),
            ("Fireworks", "Animation Tool"),
        ];
        let results = LauncherSearch::rank_results("fire", &items);
        assert_eq!(results.len(), 3);
        // First result should be the best match
        assert!(results[0].1 >= results[1].1);
        assert!(results[1].1 >= results[2].1);
    }

    #[test]
    fn test_rank_results_filters_non_matching() {
        let items = [
            ("Firefox", "Web Browser"),
            ("GIMP", "Image Editor"),
            ("VLC", "Media Player"),
        ];
        let results = LauncherSearch::rank_results("fire", &items);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 0);
    }

    #[test]
    fn test_rank_results_name_over_description() {
        let items = [
            ("Blender", "3D Creation Suite"),
            ("Calculator", "Scientific Calculator"),
        ];
        let results = LauncherSearch::rank_results("calc", &items);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 1); // Calculator matches
    }

    #[test]
    fn test_debug_format() {
        let search = LauncherSearch::new(make_locale());
        let debug = format!("{:?}", search);
        assert!(debug.contains("placeholder"));
        assert!(debug.contains("query"));
    }

    #[test]
    fn test_build_does_not_panic() {
        let search = LauncherSearch::new(make_locale());
        search.build();
    }

    #[test]
    fn test_focus_does_not_panic() {
        let search = LauncherSearch::new(make_locale());
        search.focus();
    }

    #[test]
    fn test_query_after_set() {
        let mut search = LauncherSearch::new(make_locale());
        search.set_query("Visual Studio Code");
        assert_eq!(search.query(), "Visual Studio Code");
    }

    #[test]
    fn test_rank_results_empty_input() {
        let results = LauncherSearch::rank_results("test", &[]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_fuzzy_score_case_insensitive() {
        assert_eq!(LauncherSearch::fuzzy_score("FIREFOX", "firefox"), 1000);
        assert_eq!(LauncherSearch::fuzzy_score("firefox", "FIREFOX"), 1000);
    }
}
