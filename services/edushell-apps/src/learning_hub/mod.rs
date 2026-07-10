use std::path::PathBuf;

use crate::learning_engine::{
    CategoryInfo, LearningContentEngine, Lesson, QuizAttempt, SearchIndex, UserProgress,
};
use crate::localization::{Lang, LocalizationManager};
use crate::tr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HubView {
    Dashboard,
    Categories,
    CategoryDetail,
    LessonView,
    Search,
    Bookmarks,
    History,
    Favorites,
    Progress,
}

#[derive(Debug, Clone)]
pub struct LearningHub {
    engine: LearningContentEngine,
    localization: LocalizationManager,
    current_view: HubView,
    selected_category: Option<String>,
    selected_lesson: Option<String>,
    search_query: String,
    search_results: Vec<SearchIndex>,
    #[cfg(feature = "gtk")]
    window: Option<gtk::Window>,
}

#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    pub total_lessons: u32,
    pub completed_lessons: u32,
    pub completion_percentage: f64,
    pub total_xp: u64,
    pub streak_days: u32,
    pub bookmarks_count: u32,
    pub favorites_count: u32,
    pub recent_activity_count: u32,
    pub categories_count: u32,
    pub categories_with_progress: Vec<(String, String, u32, u32, f64)>,
}

impl LearningHub {
    pub fn new(content_dir: Option<PathBuf>) -> Self {
        let mut engine = LearningContentEngine::new(content_dir);
        let localization = LocalizationManager::new();
        let _ = engine.load_progress();
        LearningHub {
            engine,
            localization,
            current_view: HubView::Dashboard,
            selected_category: None,
            selected_lesson: None,
            search_query: String::new(),
            search_results: Vec::new(),
            #[cfg(feature = "gtk")]
            window: None,
        }
    }

    pub fn engine(&self) -> &LearningContentEngine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut LearningContentEngine {
        &mut self.engine
    }

    pub fn view(&self) -> HubView {
        self.current_view
    }

    pub fn set_view(&mut self, view: HubView) {
        self.current_view = view;
    }

    pub fn categories(&self) -> Vec<&CategoryInfo> {
        self.engine.categories()
    }

    pub fn select_category(&mut self, id: &str) {
        self.selected_category = Some(id.to_string());
        self.current_view = HubView::CategoryDetail;
    }

    pub fn selected_category(&self) -> Option<&str> {
        self.selected_category.as_deref()
    }

    pub fn lessons_in_category(&self) -> Vec<Lesson> {
        if let Some(ref cat) = self.selected_category {
            self.engine
                .lessons_by_category(cat)
                .into_iter()
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn select_lesson(&mut self, id: &str) {
        self.selected_lesson = Some(id.to_string());
        self.current_view = HubView::LessonView;
    }

    pub fn selected_lesson(&self) -> Option<&str> {
        self.selected_lesson.as_deref()
    }

    pub fn current_lesson(&self) -> Option<Lesson> {
        self.selected_lesson
            .as_ref()
            .and_then(|id| self.engine.get_lesson(id))
            .cloned()
    }

    pub fn mark_lesson_complete(&mut self, id: &str) {
        self.engine.complete_lesson(id);
    }

    pub fn get_lesson_body(&self) -> String {
        self.selected_lesson
            .as_ref()
            .and_then(|id| self.engine.get_lesson(id))
            .map(|lesson| self.engine.get_lesson_body(lesson).to_string())
            .unwrap_or_default()
    }

    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.search_results = self.engine.search(query);
        self.current_view = HubView::Search;
    }

    pub fn search_results(&self) -> &[SearchIndex] {
        &self.search_results
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_results.clear();
    }

    pub fn bookmarks(&self) -> Vec<&Lesson> {
        self.engine
            .get_progress()
            .bookmarks
            .iter()
            .filter_map(|id| self.engine.get_lesson(id))
            .collect()
    }

    pub fn add_bookmark(&mut self, id: &str) {
        self.engine.add_bookmark(id);
    }

    pub fn remove_bookmark(&mut self, id: &str) {
        self.engine.remove_bookmark(id);
    }

    pub fn is_bookmarked(&self, id: &str) -> bool {
        self.engine.get_progress().bookmarks.contains(&id.to_string())
    }

    pub fn favorites(&self) -> Vec<&Lesson> {
        self.engine
            .get_progress()
            .favorites
            .iter()
            .filter_map(|id| self.engine.get_lesson(id))
            .collect()
    }

    pub fn add_favorite(&mut self, id: &str) {
        self.engine.add_favorite(id);
    }

    pub fn remove_favorite(&mut self, id: &str) {
        self.engine.remove_favorite(id);
    }

    pub fn is_favorite(&self, id: &str) -> bool {
        self.engine.get_progress().favorites.contains(&id.to_string())
    }

    pub fn history(&self) -> Vec<crate::learning_engine::HistoryEntry> {
        self.engine.get_history().to_vec()
    }

    pub fn add_to_history(&mut self, id: &str, progress: u8) {
        self.engine.add_history(id, progress);
    }

    pub fn progress(&self) -> &UserProgress {
        self.engine.get_progress()
    }

    pub fn completion_percentage(&self) -> f64 {
        self.engine.completion_percentage()
    }

    pub fn total_lessons(&self) -> u32 {
        self.engine.total_lessons()
    }

    pub fn completed_count(&self) -> u32 {
        self.engine.completed_lessons_count()
    }

    pub fn total_xp(&self) -> u64 {
        self.engine.get_progress().total_xp
    }

    pub fn streak_days(&self) -> u32 {
        self.engine.get_progress().streak_days
    }

    pub fn streak_info(&self) -> (u32, String) {
        self.engine.get_streak_info()
    }

    pub fn recommendations(&self, count: usize) -> Vec<Lesson> {
        self.engine
            .get_recommendations(count)
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn continue_learning(&self) -> Option<Lesson> {
        if let Some(ref id) = self.engine.get_progress().current_lesson {
            if !self
                .engine
                .get_progress()
                .completed_lessons
                .contains(id)
            {
                if let Some(lesson) = self.engine.get_lesson(id) {
                    return Some(lesson.clone());
                }
            }
        }
        self.engine.get_next_lesson().cloned()
    }

    pub fn dashboard_stats(&self) -> DashboardStats {
        let cats = self.engine.categories();
        let mut categories_with_progress: Vec<(String, String, u32, u32, f64)> = cats
            .iter()
            .map(|cat| {
                let (completed, total, pct) = self.category_progress(&cat.id);
                (cat.id.clone(), cat.name_key.clone(), completed, total, pct)
            })
            .collect();
        categories_with_progress.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));

        DashboardStats {
            total_lessons: self.engine.total_lessons(),
            completed_lessons: self.engine.completed_lessons_count(),
            completion_percentage: self.engine.completion_percentage(),
            total_xp: self.engine.get_progress().total_xp,
            streak_days: self.engine.get_progress().streak_days,
            bookmarks_count: self.engine.get_progress().bookmarks.len() as u32,
            favorites_count: self.engine.get_progress().favorites.len() as u32,
            recent_activity_count: self.engine.get_progress().history.len() as u32,
            categories_count: cats.len() as u32,
            categories_with_progress,
        }
    }

    pub fn recent_activity(&self, count: usize) -> Vec<crate::learning_engine::HistoryEntry> {
        let mut entries = self.engine.get_history().to_vec();
        entries.reverse();
        entries.truncate(count);
        entries
    }

    pub fn category_progress(&self, cat_id: &str) -> (u32, u32, f64) {
        let lessons = self.engine.lessons_by_category(cat_id);
        let total = lessons.len() as u32;
        let completed = lessons
            .iter()
            .filter(|l| {
                self.engine
                    .get_progress()
                    .completed_lessons
                    .contains(&l.metadata.id)
            })
            .count() as u32;
        let pct = if total == 0 {
            0.0
        } else {
            (completed as f64 / total as f64) * 100.0
        };
        (completed, total, pct)
    }

    pub fn quiz_for_current_lesson(&self) -> Option<crate::learning_engine::Quiz> {
        self.selected_lesson
            .as_ref()
            .and_then(|id| self.engine.get_lesson(id))
            .and_then(|lesson| lesson.quiz.clone())
    }

    pub fn submit_quiz_answer(
        &self,
        question_id: &str,
        answer_index: usize,
    ) -> Result<bool, String> {
        let lesson = self
            .selected_lesson
            .as_ref()
            .and_then(|id| self.engine.get_lesson(id))
            .ok_or_else(|| "No lesson selected".to_string())?;

        let quiz = lesson
            .quiz
            .as_ref()
            .ok_or_else(|| "No quiz for this lesson".to_string())?;

        let question = quiz
            .questions
            .iter()
            .find(|q| q.id == question_id)
            .ok_or_else(|| format!("Question not found: {}", question_id))?;

        Ok(answer_index == question.correct_index)
    }

    pub fn record_quiz_attempt(&mut self, attempt: QuizAttempt) {
        self.engine.record_quiz_attempt(attempt);
    }

    pub fn get_quiz_score(&self, quiz_id: &str) -> Option<QuizAttempt> {
        self.engine.get_quiz_score(quiz_id).cloned()
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = match locale {
            "en" => Lang::English,
            _ => Lang::Indonesian,
        };
        self.localization.set_language(lang);
        self.engine.set_locale(locale);
    }

    pub fn generate_dashboard(&self) -> String {
        let stats = self.dashboard_stats();
        let mut out = String::new();

        out.push_str("==============================\n");
        out.push_str(tr!(self.localization, "learning.title"));
        out.push_str(" - ");
        out.push_str(tr!(self.localization, "welcome.subtitle"));
        out.push_str("\n==============================\n\n");

        if let Some(lesson) = self.continue_learning() {
            out.push_str(">>> ");
            out.push_str(tr!(self.localization, "learning.in_progress"));
            out.push_str(": ");
            out.push_str(&lesson.metadata.title);
            out.push_str("\n\n");
        }

        let recs = self.recommendations(4);
        if !recs.is_empty() {
            out.push_str("--- ");
            out.push_str(tr!(self.localization, "learning.recommended"));
            out.push_str(" ---\n");
            for rec in &recs {
                out.push_str("  * ");
                out.push_str(&rec.metadata.title);
                out.push_str(" [");
                out.push_str(&format!("{:?}", rec.metadata.difficulty));
                out.push_str("]\n");
            }
            out.push('\n');
        }

        out.push_str("--- ");
        out.push_str(tr!(self.localization, "learning.progress"));
        out.push_str(" ---\n");
        for (_, name_key, completed, total, pct) in &stats.categories_with_progress {
            if *total == 0 {
                continue;
            }
            let label = tr!(self.localization, name_key);
            out.push_str(&format!("  {}: {}/{} ({:.0}%)\n", label, completed, total, pct));
        }
        out.push('\n');

        let recent = self.recent_activity(5);
        if !recent.is_empty() {
            out.push_str("--- ");
            out.push_str(tr!(self.localization, "welcome.recent"));
            out.push_str(" ---\n");
            for entry in &recent {
                if let Some(lesson) = self.engine.get_lesson(&entry.content_id) {
                    out.push_str(&format!(
                        "  * {} ({}%)\n",
                        lesson.metadata.title, entry.progress_percent
                    ));
                }
            }
            out.push('\n');
        }

        out.push_str("--- ");
        out.push_str(tr!(self.localization, "common.quick_info"));
        out.push_str(" ---\n");
        out.push_str(&format!(
            "XP: {} | {}: {} | {}: {:.0}% | {}: {}\n",
            stats.total_xp,
            tr!(self.localization, "common.bookmarks"),
            stats.bookmarks_count,
            tr!(self.localization, "learning.progress"),
            stats.completion_percentage,
            tr!(self.localization, "common.streak"),
            stats.streak_days,
        ));

        out
    }

    pub fn generate_lesson_view(&self) -> String {
        let lesson = match self
            .selected_lesson
            .as_ref()
            .and_then(|id| self.engine.get_lesson(id))
        {
            Some(l) => l,
            None => return String::new(),
        };

        let mut out = String::new();

        out.push_str("==============================\n");
        out.push_str(&lesson.metadata.title);
        out.push('\n');
        out.push_str("==============================\n\n");

        out.push_str(&format!(
            "{}: {} | {}: {:?} | {}: {} {}\n\n",
            tr!(self.localization, "common.category"),
            lesson.metadata.category,
            tr!(self.localization, "learning.difficulty"),
            lesson.metadata.difficulty,
            tr!(self.localization, "common.duration"),
            lesson.metadata.duration_minutes,
            tr!(self.localization, "common.minutes"),
        ));

        let body = self.engine.get_lesson_body(lesson);
        out.push_str("--- ");
        out.push_str(tr!(self.localization, "common.content"));
        out.push_str(" ---\n");
        let stripped = strip_markdown(body);
        out.push_str(&stripped);
        out.push('\n');

        if !lesson.vocabulary.is_empty() {
            out.push_str("\n--- ");
            out.push_str(tr!(self.localization, "common.vocabulary"));
            out.push_str(" ---\n");
            for v in &lesson.vocabulary {
                out.push_str(&format!("  * {}: {}\n", v.term, v.definition));
            }
        }

        if !lesson.key_points.is_empty() {
            out.push_str("\n--- ");
            out.push_str(tr!(self.localization, "common.key_points"));
            out.push_str(" ---\n");
            for kp in &lesson.key_points {
                out.push_str(&format!("  * {}\n", kp));
            }
        }

        if lesson.quiz.is_some() {
            out.push_str(&format!(
                "\n--- {} ---\n  {}\n",
                tr!(self.localization, "learning.quizzes"),
                tr!(self.localization, "common.available"),
            ));
        }

        if !lesson.related_content.is_empty() {
            out.push_str("\n--- ");
            out.push_str(tr!(self.localization, "common.related"));
            out.push_str(" ---\n");
            for rel_id in &lesson.related_content {
                if let Some(rel) = self.engine.get_lesson(rel_id) {
                    out.push_str(&format!("  * {}\n", rel.metadata.title));
                }
            }
        }

        out.push_str(&format!(
            "\n[{} | {} | {}]\n",
            tr!(self.localization, "common.bookmark"),
            tr!(self.localization, "common.favorite"),
            tr!(self.localization, "common.complete"),
        ));

        out
    }

    pub fn generate_progress_view(&self) -> String {
        let stats = self.dashboard_stats();
        let mut out = String::new();

        out.push_str("==============================\n");
        out.push_str(tr!(self.localization, "learning.progress"));
        out.push_str("\n==============================\n\n");

        let bar_width: usize = 30;
        let filled = (stats.completion_percentage / 100.0 * bar_width as f64).round() as usize;
        let empty = bar_width.saturating_sub(filled);
        out.push_str(&format!(
            "[{}{}] {:.0}%\n\n",
            "#".repeat(filled),
            "-".repeat(empty),
            stats.completion_percentage
        ));

        out.push_str(&format!(
            "{}: {}/{}\n\n",
            tr!(self.localization, "common.completed"),
            stats.completed_lessons,
            stats.total_lessons
        ));

        out.push_str("--- ");
        out.push_str(tr!(self.localization, "common.category"));
        out.push_str(" ---\n");
        for (_, name_key, completed, total, pct) in &stats.categories_with_progress {
            if *total == 0 {
                continue;
            }
            let label = tr!(self.localization, name_key);
            let cfilled = ((*pct / 100.0) * bar_width as f64).round() as usize;
            let cempty: usize = bar_width.saturating_sub(cfilled);
            out.push_str(&format!(
                "{} [{}{}] {}/{} ({:.0}%)\n",
                label,
                "#".repeat(cfilled),
                "-".repeat(cempty),
                completed,
                total,
                pct
            ));
        }

        out.push_str(&format!(
            "\nXP: {} | {}: {} {}\n",
            stats.total_xp,
            tr!(self.localization, "common.streak"),
            stats.streak_days,
            tr!(self.localization, "common.days"),
        ));

        out.push_str(&format!(
            "\n{}: {}\n",
            tr!(self.localization, "common.bookmarks"),
            stats.bookmarks_count
        ));
        out.push_str(&format!(
            "{}: {}\n",
            tr!(self.localization, "common.favorites"),
            stats.favorites_count
        ));

        let quiz_count = self.engine.get_progress().quiz_scores.len();
        if quiz_count > 0 {
            out.push_str(&format!("\n--- {} ---\n", tr!(self.localization, "learning.quizzes")));
            for (qid, attempt) in &self.engine.get_progress().quiz_scores {
                out.push_str(&format!("  * {}: {}%\n", qid, attempt.score));
            }
        }

        out
    }
}

fn strip_markdown(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_code_block = false;
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            if in_code_block {
                result.push_str("  CODE:\n");
            }
            i += 1;
            continue;
        }
        if in_code_block {
            result.push_str("  ");
            result.push_str(line);
            result.push('\n');
            i += 1;
            continue;
        }
        let clean = trimmed
            .trim_start_matches("# ")
            .trim_start_matches("## ")
            .trim_start_matches("### ")
            .trim_start_matches("* ")
            .trim_start_matches("- ")
            .trim_start_matches("|");
        if line.trim().starts_with('|') && line.trim().contains("---") {
            i += 1;
            continue;
        }
        if !clean.is_empty() {
            result.push_str(clean);
            result.push('\n');
        } else {
            result.push('\n');
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_hub() -> LearningHub {
        LearningHub::new(Some(PathBuf::from("/tmp/edushell-test-hub-content")))
    }

    #[test]
    fn test_hub_creation_default_state() {
        let hub = test_hub();
        assert_eq!(hub.view(), HubView::Dashboard);
        assert!(hub.selected_category().is_none());
        assert!(hub.selected_lesson().is_none());
        assert!(hub.search_results().is_empty());
        assert!(hub.total_lessons() > 0);
    }

    #[test]
    fn test_view_switching() {
        let mut hub = test_hub();
        assert_eq!(hub.view(), HubView::Dashboard);
        hub.set_view(HubView::Categories);
        assert_eq!(hub.view(), HubView::Categories);
        hub.set_view(HubView::Bookmarks);
        assert_eq!(hub.view(), HubView::Bookmarks);
        hub.set_view(HubView::Progress);
        assert_eq!(hub.view(), HubView::Progress);
    }

    #[test]
    fn test_all_views_accessible() {
        let views = vec![
            HubView::Dashboard,
            HubView::Categories,
            HubView::CategoryDetail,
            HubView::LessonView,
            HubView::Search,
            HubView::Bookmarks,
            HubView::History,
            HubView::Favorites,
            HubView::Progress,
        ];
        let mut hub = test_hub();
        for view in views {
            hub.set_view(view);
            assert_eq!(hub.view(), view);
        }
    }

    #[test]
    fn test_category_selection_and_listing() {
        let mut hub = test_hub();
        let cats = hub.categories();
        assert!(!cats.is_empty());

        let first_cat = cats[0].id.clone();
        hub.select_category(&first_cat);
        assert_eq!(hub.selected_category(), Some(first_cat.as_str()));
        assert_eq!(hub.view(), HubView::CategoryDetail);
    }

    #[test]
    fn test_lessons_in_category() {
        let mut hub = test_hub();
        hub.select_category("belajar-terminal");
        let lessons = hub.lessons_in_category();
        assert_eq!(lessons.len(), 2);
        assert!(lessons.iter().any(|l| l.metadata.id == "pengenalan-terminal"));
        assert!(lessons.iter().any(|l| l.metadata.id == "navigasi-file-system"));
    }

    #[test]
    fn test_empty_category_lessons() {
        let mut hub = test_hub();
        hub.select_category("belajar-rust");
        let lessons = hub.lessons_in_category();
        assert!(lessons.is_empty());
    }

    #[test]
    fn test_lesson_selection_and_retrieval() {
        let mut hub = test_hub();
        hub.select_lesson("pengenalan-terminal");
        assert_eq!(hub.selected_lesson(), Some("pengenalan-terminal"));
        assert_eq!(hub.view(), HubView::LessonView);

        let lesson = hub.current_lesson();
        assert!(lesson.is_some());
        assert_eq!(lesson.unwrap().metadata.id, "pengenalan-terminal");
    }

    #[test]
    fn test_get_lesson_body_localized_id() {
        let mut hub = test_hub();
        hub.select_lesson("pengenalan-terminal");
        let body = hub.get_lesson_body();
        assert!(body.contains("Pengenalan Terminal"));
    }

    #[test]
    fn test_get_lesson_body_localized_en() {
        let mut hub = test_hub();
        hub.set_locale("en");
        hub.select_lesson("pengenalan-terminal");
        let body = hub.get_lesson_body();
        assert!(body.contains("Introduction to Terminal"));
    }

    #[test]
    fn test_search_matches() {
        let mut hub = test_hub();
        hub.search("terminal");
        assert_eq!(hub.view(), HubView::Search);
        assert!(!hub.search_results().is_empty());
        assert!(hub.search_results().iter().any(|r| r.content_id == "pengenalan-terminal"));
    }

    #[test]
    fn test_search_no_matches() {
        let mut hub = test_hub();
        hub.search("xyznonexistent12345");
        assert!(hub.search_results().is_empty());
    }

    #[test]
    fn test_search_results_cleared_on_new_search() {
        let mut hub = test_hub();
        hub.search("terminal");
        assert!(!hub.search_results().is_empty());
        hub.search("xyznonexistent12345");
        assert!(hub.search_results().is_empty());
    }

    #[test]
    fn test_clear_search() {
        let mut hub = test_hub();
        hub.search("terminal");
        assert!(!hub.search_results().is_empty());
        hub.clear_search();
        assert!(hub.search_results().is_empty());
    }

    #[test]
    fn test_bookmark_lifecycle() {
        let mut hub = test_hub();
        assert!(!hub.is_bookmarked("pengenalan-terminal"));
        hub.add_bookmark("pengenalan-terminal");
        assert!(hub.is_bookmarked("pengenalan-terminal"));
        let bm = hub.bookmarks();
        assert_eq!(bm.len(), 1);
        assert_eq!(bm[0].metadata.id, "pengenalan-terminal");
        hub.remove_bookmark("pengenalan-terminal");
        assert!(!hub.is_bookmarked("pengenalan-terminal"));
        assert!(hub.bookmarks().is_empty());
    }

    #[test]
    fn test_favorite_lifecycle() {
        let mut hub = test_hub();
        assert!(!hub.is_favorite("pengenalan-terminal"));
        hub.add_favorite("pengenalan-terminal");
        assert!(hub.is_favorite("pengenalan-terminal"));
        let fav = hub.favorites();
        assert_eq!(fav.len(), 1);
        assert_eq!(fav[0].metadata.id, "pengenalan-terminal");
        hub.remove_favorite("pengenalan-terminal");
        assert!(!hub.is_favorite("pengenalan-terminal"));
        assert!(hub.favorites().is_empty());
    }

    #[test]
    fn test_history_recording() {
        let mut hub = test_hub();
        assert!(hub.history().is_empty());
        hub.add_to_history("pengenalan-terminal", 50);
        assert_eq!(hub.history().len(), 1);
        assert_eq!(hub.history()[0].content_id, "pengenalan-terminal");
        assert_eq!(hub.history()[0].progress_percent, 50);
    }

    #[test]
    fn test_history_overwrites_same_id() {
        let mut hub = test_hub();
        hub.add_to_history("pengenalan-terminal", 50);
        hub.add_to_history("pengenalan-terminal", 100);
        assert_eq!(hub.history().len(), 1);
        assert_eq!(hub.history()[0].progress_percent, 100);
    }

    #[test]
    fn test_progress_tracking() {
        let mut hub = test_hub();
        let initial_completed = hub.completed_count();
        let initial_pct = hub.completion_percentage();
        assert_eq!(initial_completed, 0);
        assert_eq!(initial_pct, 0.0);

        hub.mark_lesson_complete("pengenalan-terminal");
        assert_eq!(hub.completed_count(), 1);
        assert!(hub.completion_percentage() > 0.0);
    }

    #[test]
    fn test_multiple_lessons_completion() {
        let mut hub = test_hub();
        hub.mark_lesson_complete("pengenalan-terminal");
        hub.mark_lesson_complete("navigasi-file-system");
        assert_eq!(hub.completed_count(), 2);
    }

    #[test]
    fn test_dashboard_stats_calculation() {
        let mut hub = test_hub();
        let stats = hub.dashboard_stats();
        assert_eq!(stats.total_lessons, hub.total_lessons());
        assert_eq!(stats.completed_lessons, 0);
        assert_eq!(stats.bookmarks_count, 0);
        assert_eq!(stats.favorites_count, 0);

        hub.add_bookmark("pengenalan-terminal");
        hub.add_favorite("navigasi-file-system");
        let stats = hub.dashboard_stats();
        assert_eq!(stats.bookmarks_count, 1);
        assert_eq!(stats.favorites_count, 1);
    }

    #[test]
    fn test_category_progress() {
        let hub = test_hub();
        let (completed, total, pct) = hub.category_progress("belajar-terminal");
        assert_eq!(total, 2);
        assert_eq!(completed, 0);
        assert_eq!(pct, 0.0);
    }

    #[test]
    fn test_category_progress_after_completion() {
        let mut hub = test_hub();
        hub.mark_lesson_complete("pengenalan-terminal");
        let (completed, total, pct) = hub.category_progress("belajar-terminal");
        assert_eq!(total, 2);
        assert_eq!(completed, 1);
        assert!((pct - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_continue_learning_empty() {
        let hub = test_hub();
        let cont = hub.continue_learning();
        assert!(cont.is_some());
        assert_eq!(cont.unwrap().metadata.id, "pengenalan-terminal");
    }

    #[test]
    fn test_continue_learning_after_completing_some() {
        let mut hub = test_hub();
        hub.mark_lesson_complete("pengenalan-terminal");
        let cont = hub.continue_learning();
        assert!(cont.is_some());
        assert_eq!(cont.unwrap().metadata.id, "navigasi-file-system");
    }

    #[test]
    fn test_recommendation_generation() {
        let hub = test_hub();
        let recs = hub.recommendations(5);
        assert!(!recs.is_empty());
        assert!(recs.len() <= 5);
    }

    #[test]
    fn test_recommendations_exclude_completed() {
        let mut hub = test_hub();
        hub.mark_lesson_complete("pengenalan-terminal");
        let recs = hub.recommendations(10);
        assert!(!recs.iter().any(|r| r.metadata.id == "pengenalan-terminal"));
    }

    #[test]
    fn test_quiz_retrieval() {
        let mut hub = test_hub();
        hub.select_lesson("pengenalan-terminal");
        let quiz = hub.quiz_for_current_lesson();
        assert!(quiz.is_some());
        assert_eq!(quiz.unwrap().id, "quiz-pengenalan-terminal");
    }

    #[test]
    fn test_quiz_for_lesson_without_quiz() {
        let mut hub = test_hub();
        hub.select_lesson("nonexistent");
        let quiz = hub.quiz_for_current_lesson();
        assert!(quiz.is_none());
    }

    #[test]
    fn test_submit_quiz_answer_correct() {
        let mut hub = test_hub();
        hub.select_lesson("pengenalan-terminal");
        let result = hub.submit_quiz_answer("q-terminal-1", 0);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_submit_quiz_answer_wrong() {
        let mut hub = test_hub();
        hub.select_lesson("pengenalan-terminal");
        let result = hub.submit_quiz_answer("q-terminal-1", 1);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_submit_quiz_answer_no_lesson() {
        let hub = test_hub();
        let result = hub.submit_quiz_answer("q-terminal-1", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_submit_quiz_answer_question_not_found() {
        let mut hub = test_hub();
        hub.select_lesson("pengenalan-terminal");
        let result = hub.submit_quiz_answer("nonexistent-q", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_record_and_get_quiz_attempt() {
        let mut hub = test_hub();
        let attempt = QuizAttempt {
            quiz_id: "quiz-pengenalan-terminal".to_string(),
            score: 85,
            answers: vec![0, 2, 1, 3],
            completed_at: "2025-01-15T10:00:00Z".to_string(),
            time_spent_seconds: 420,
        };
        hub.record_quiz_attempt(attempt);
        let score = hub.get_quiz_score("quiz-pengenalan-terminal");
        assert!(score.is_some());
        assert_eq!(score.unwrap().score, 85);
    }

    #[test]
    fn test_get_quiz_score_not_found() {
        let hub = test_hub();
        assert!(hub.get_quiz_score("nonexistent").is_none());
    }

    #[test]
    fn test_locale_switching() {
        let mut hub = test_hub();
        hub.set_locale("id");
        let body_id = {
            hub.select_lesson("pengenalan-terminal");
            hub.get_lesson_body()
        };
        assert!(body_id.contains("Pengenalan Terminal"));

        hub.set_locale("en");
        let body_en = {
            hub.get_lesson_body()
        };
        assert!(body_en.contains("Introduction to Terminal"));
    }

    #[test]
    fn test_streak_info() {
        let mut hub = test_hub();
        let (days, _) = hub.streak_info();
        assert_eq!(days, 0);
        hub.mark_lesson_complete("pengenalan-terminal");
        let (days2, _) = hub.streak_info();
        assert_eq!(days2, 1);
    }

    #[test]
    fn test_total_xp_after_completing_lessons() {
        let mut hub = test_hub();
        let xp_before = hub.total_xp();
        assert_eq!(xp_before, 0);
        hub.mark_lesson_complete("pengenalan-terminal");
        let xp_after = hub.total_xp();
        assert_eq!(xp_after, 100);
        hub.mark_lesson_complete("navigasi-file-system");
        let xp_after_two = hub.total_xp();
        assert_eq!(xp_after_two, 200);
    }

    #[test]
    fn test_recent_activity() {
        let mut hub = test_hub();
        assert!(hub.recent_activity(5).is_empty());
        hub.add_to_history("pengenalan-terminal", 100);
        hub.add_to_history("navigasi-file-system", 50);
        let recent = hub.recent_activity(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].content_id, "navigasi-file-system");
    }

    #[test]
    fn test_generate_dashboard() {
        let hub = test_hub();
        let dashboard = hub.generate_dashboard();
        assert!(!dashboard.is_empty());
        assert!(dashboard.contains("Pusat Belajar"));
    }

    #[test]
    fn test_generate_lesson_view() {
        let mut hub = test_hub();
        hub.select_lesson("pengenalan-terminal");
        let view = hub.generate_lesson_view();
        assert!(!view.is_empty());
        assert!(view.contains("content.pengenalan_terminal.title"));
    }

    #[test]
    fn test_generate_lesson_view_no_selection() {
        let hub = test_hub();
        let view = hub.generate_lesson_view();
        assert!(view.is_empty());
    }

    #[test]
    fn test_generate_progress_view() {
        let hub = test_hub();
        let view = hub.generate_progress_view();
        assert!(!view.is_empty());
        assert!(view.contains("Kemajuan"));
    }

    #[test]
    fn test_dashboard_stats_categories_with_progress() {
        let hub = test_hub();
        let stats = hub.dashboard_stats();
        assert!(!stats.categories_with_progress.is_empty());
        for (_, _, _, total, _) in &stats.categories_with_progress {
            assert!(*total > 0 || *total == 0);
        }
    }

    #[test]
    fn test_dashboard_stats_categories_sorted() {
        let mut hub = test_hub();
        hub.mark_lesson_complete("pengenalan-terminal");
        let stats = hub.dashboard_stats();
        let pcts: Vec<f64> = stats.categories_with_progress.iter().map(|(_, _, _, _, p)| *p).collect();
        for i in 1..pcts.len() {
            assert!(pcts[i - 1] >= pcts[i], "categories not sorted by progress descending");
        }
    }

    #[test]
    fn test_engine_access() {
        let hub = test_hub();
        assert_eq!(hub.engine().total_lessons(), 2);
    }

    #[test]
    fn test_engine_mut_access() {
        let mut hub = test_hub();
        hub.engine_mut().set_locale("en");
        assert_eq!(hub.engine().get_locale(), "en");
    }

    #[test]
    fn test_progress_reference() {
        let hub = test_hub();
        let p = hub.progress();
        assert_eq!(p.completed_lessons.len(), 0);
    }

    #[test]
    fn test_set_locale_maps_to_lang() {
        let mut hub = test_hub();
        hub.set_locale("en");
        assert_eq!(hub.engine().get_locale(), "en");
        hub.set_locale("id");
        assert_eq!(hub.engine().get_locale(), "id");
    }

    #[test]
    fn test_empty_lessons_in_category_no_selection() {
        let hub = test_hub();
        let lessons = hub.lessons_in_category();
        assert!(lessons.is_empty());
    }

    #[test]
    fn test_continue_learning_follows_current_lesson() {
        let mut hub = test_hub();
        hub.engine_mut().get_progress_mut().current_lesson = Some("navigasi-file-system".to_string());
        let cont = hub.continue_learning();
        assert!(cont.is_some());
        assert_eq!(cont.unwrap().metadata.id, "navigasi-file-system");
    }

    #[test]
    fn test_continue_learning_skips_completed_current() {
        let mut hub = test_hub();
        hub.mark_lesson_complete("pengenalan-terminal");
        hub.engine_mut().get_progress_mut().current_lesson = Some("pengenalan-terminal".to_string());
        let cont = hub.continue_learning();
        assert!(cont.is_some());
        assert_eq!(cont.unwrap().metadata.id, "navigasi-file-system");
    }

    #[test]
    fn test_completion_percentage_double_complete_same() {
        let mut hub = test_hub();
        hub.mark_lesson_complete("pengenalan-terminal");
        let pct1 = hub.completion_percentage();
        hub.mark_lesson_complete("pengenalan-terminal");
        let pct2 = hub.completion_percentage();
        assert!((pct1 - pct2).abs() < 0.001);
    }

    #[test]
    fn test_dashboard_stats_empty_hub() {
        let hub = LearningHub::new(Some(PathBuf::from("/tmp/edushell-test-empty-stats")));
        let stats = hub.dashboard_stats();
        assert!(stats.total_lessons > 0);
        assert_eq!(stats.completed_lessons, 0);
        assert_eq!(stats.bookmarks_count, 0);
        assert!(stats.categories_count > 0);
    }

    #[test]
    fn test_multiple_bookmarks() {
        let mut hub = test_hub();
        hub.add_bookmark("pengenalan-terminal");
        hub.add_bookmark("navigasi-file-system");
        assert_eq!(hub.bookmarks().len(), 2);
        assert!(hub.is_bookmarked("pengenalan-terminal"));
        assert!(hub.is_bookmarked("navigasi-file-system"));
    }

    #[test]
    fn test_bookmark_remove_nonexistent() {
        let mut hub = test_hub();
        hub.remove_bookmark("nonexistent");
        assert!(hub.bookmarks().is_empty());
    }

    #[test]
    fn test_favorite_remove_nonexistent() {
        let mut hub = test_hub();
        hub.remove_favorite("nonexistent");
        assert!(hub.favorites().is_empty());
    }
}
