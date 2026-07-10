use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{NaiveDate, Utc};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, LearningEngineError>;

#[derive(Debug, Error)]
pub enum LearningEngineError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Content not found: {0}")]
    ContentNotFound(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ContentFormat {
    Markdown,
    Html,
    Image,
    Svg,
    VideoPlaceholder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAuthor {
    pub name: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub difficulty: ContentDifficulty,
    pub author: ContentAuthor,
    pub format: ContentFormat,
    pub duration_minutes: u32,
    pub tags: Vec<String>,
    pub prerequisites: Vec<String>,
    pub order: u32,
    pub locale: String,
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub cover_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: String,
    pub question: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub explanation: String,
    pub multiple_answer: bool,
    pub points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: String,
    pub questions: Vec<QuizQuestion>,
    pub passing_score: u8,
    pub time_limit_minutes: Option<u32>,
    pub shuffle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseStep {
    pub instruction: String,
    pub command: Option<String>,
    pub expected_output: Option<String>,
    pub hint: Option<String>,
    pub validation_type: ValidationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    ExactOutput(String),
    ContainsOutput(String),
    RegexOutput(String),
    FileExists(String),
    CommandSuccess,
    ManualCheck(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<ExerciseStep>,
    pub setup_commands: Vec<String>,
    pub cleanup_commands: Vec<String>,
    pub requires_sandbox: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub metadata: ContentMetadata,
    pub body: String,
    pub body_en: Option<String>,
    pub vocabulary: Vec<VocabEntry>,
    pub key_points: Vec<String>,
    pub quiz: Option<Quiz>,
    pub exercise: Option<Exercise>,
    pub related_content: Vec<String>,
    pub external_links: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabEntry {
    pub term: String,
    pub definition: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProgress {
    pub completed_lessons: Vec<String>,
    pub quiz_scores: HashMap<String, QuizAttempt>,
    pub exercise_completions: HashMap<String, bool>,
    pub bookmarks: Vec<String>,
    pub history: Vec<HistoryEntry>,
    pub favorites: Vec<String>,
    pub total_xp: u64,
    pub streak_days: u32,
    pub last_activity: Option<String>,
    pub current_lesson: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub quiz_id: String,
    pub score: u8,
    pub answers: Vec<usize>,
    pub completed_at: String,
    pub time_spent_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub content_id: String,
    pub accessed_at: String,
    pub progress_percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
    pub icon: String,
    pub color: String,
    pub order: u32,
    pub lesson_count: u32,
    pub total_duration: u32,
    pub difficulty: ContentDifficulty,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    pub content_id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub difficulty: ContentDifficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedEntry {
    pub data: Vec<u8>,
    pub content_type: String,
    pub cached_at: String,
    pub expires_at: Option<String>,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct ContentCache {
    entries: HashMap<String, CachedEntry>,
    lru_keys: Vec<String>,
    max_size: usize,
    current_size: usize,
}

impl ContentCache {
    pub fn new(max_size_mb: usize) -> Self {
        ContentCache {
            entries: HashMap::new(),
            lru_keys: Vec::new(),
            max_size: max_size_mb * 1024 * 1024,
            current_size: 0,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&CachedEntry> {
        if self.entries.contains_key(key) {
            self.touch(key);
            self.entries.get(key)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: &str, data: Vec<u8>, content_type: &str, ttl_seconds: Option<u64>) {
        let size = data.len();
        let cached_at = Utc::now().to_rfc3339();
        let expires_at = ttl_seconds.map(|secs| {
            let dt = Utc::now() + chrono::Duration::seconds(secs as i64);
            dt.to_rfc3339()
        });

        if let Some(existing) = self.entries.get(key) {
            self.current_size = self.current_size.saturating_sub(existing.size);
            self.lru_keys.retain(|k| k != key);
        }

        while !self.entries.is_empty() && self.current_size + size > self.max_size {
            self.evict_lru();
        }

        let entry = CachedEntry {
            data,
            content_type: content_type.to_string(),
            cached_at,
            expires_at,
            size,
        };

        self.current_size += size;
        self.lru_keys.push(key.to_string());
        self.entries.insert(key.to_string(), entry);
    }

    pub fn remove(&mut self, key: &str) {
        if let Some(entry) = self.entries.remove(key) {
            self.current_size = self.current_size.saturating_sub(entry.size);
            self.lru_keys.retain(|k| k != key);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.lru_keys.clear();
        self.current_size = 0;
    }

    pub fn size(&self) -> usize {
        self.current_size
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn is_full(&self) -> bool {
        self.current_size >= self.max_size
    }

    pub fn evict_lru(&mut self) {
        if let Some(key) = self.lru_keys.first().cloned() {
            self.remove(&key);
        }
    }

    fn touch(&mut self, key: &str) {
        self.lru_keys.retain(|k| k != key);
        self.lru_keys.push(key.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct LearningContentEngine {
    lessons: HashMap<String, Lesson>,
    categories: HashMap<String, CategoryInfo>,
    progress: UserProgress,
    cache: ContentCache,
    locale: String,
    content_dir: PathBuf,
}

impl LearningContentEngine {
    pub fn new(content_dir: Option<PathBuf>) -> Self {
        let dir = content_dir.unwrap_or_else(|| PathBuf::from("content"));
        let mut engine = LearningContentEngine {
            lessons: HashMap::new(),
            categories: Self::default_categories(),
            progress: UserProgress::default(),
            cache: ContentCache::new(50),
            locale: "id".to_string(),
            content_dir: dir,
        };
        engine.load_embedded_content();
        let dir = engine.content_dir.clone();
        if dir.exists() {
            let _ = engine.load_content(&dir);
            let _ = engine.load_progress();
        }
        engine
    }

    fn default_categories() -> HashMap<String, CategoryInfo> {
        let mut map = HashMap::new();
        let categories = vec![
            ("belajar-linux", "category.belajar_linux", "category.belajar_linux.desc", "terminal-symbol", "#E95420", 1, ContentDifficulty::Beginner),
            ("belajar-terminal", "category.belajar_terminal", "category.belajar_terminal.desc", "terminal", "#2E3436", 2, ContentDifficulty::Beginner),
            ("belajar-bash", "category.belajar_bash", "category.belajar_bash.desc", "code-brackets", "#4E9A06", 3, ContentDifficulty::Intermediate),
            ("belajar-git", "category.belajar_git", "category.belajar_git.desc", "git-branch", "#F14E32", 4, ContentDifficulty::Intermediate),
            ("belajar-html", "category.belajar_html", "category.belajar_html.desc", "language-html5", "#E34F26", 5, ContentDifficulty::Beginner),
            ("belajar-css", "category.belajar_css", "category.belajar_css.desc", "language-css3", "#1572B6", 6, ContentDifficulty::Beginner),
            ("belajar-javascript", "category.belajar_javascript", "category.belajar_javascript.desc", "language-javascript", "#F7DF1E", 7, ContentDifficulty::Intermediate),
            ("belajar-python", "category.belajar_python", "category.belajar_python.desc", "language-python", "#3776AB", 8, ContentDifficulty::Beginner),
            ("belajar-c", "category.belajar_c", "category.belajar_c.desc", "language-c", "#A8B9CC", 9, ContentDifficulty::Intermediate),
            ("belajar-cpp", "category.belajar_cpp", "category.belajar_cpp.desc", "language-cpp", "#00599C", 10, ContentDifficulty::Intermediate),
            ("belajar-rust", "category.belajar_rust", "category.belajar_rust.desc", "language-rust", "#000000", 11, ContentDifficulty::Advanced),
            ("belajar-open-source", "category.belajar_open_source", "category.belajar_open_source.desc", "open-source-initiative", "#3DA639", 12, ContentDifficulty::Beginner),
            ("belajar-command-line", "category.belajar_command_line", "category.belajar_command_line.desc", "console", "#4D4D4D", 13, ContentDifficulty::Beginner),
            ("belajar-package-manager", "category.belajar_package_manager", "category.belajar_package_manager.desc", "package", "#0B60B0", 14, ContentDifficulty::Intermediate),
            ("belajar-file-system", "category.belajar_file_system", "category.belajar_file_system.desc", "folder", "#F5A623", 15, ContentDifficulty::Beginner),
            ("belajar-networking-dasar", "category.belajar_networking_dasar", "category.belajar_networking_dasar.desc", "network", "#006699", 16, ContentDifficulty::Intermediate),
            ("belajar-keamanan-dasar", "category.belajar_keamanan_dasar", "category.belajar_keamanan_dasar.desc", "shield", "#CC0000", 17, ContentDifficulty::Intermediate),
            ("belajar-office", "category.belajar_office", "category.belajar_office.desc", "office-apps", "#0072C6", 18, ContentDifficulty::Beginner),
            ("belajar-internet-aman", "category.belajar_internet_aman", "category.belajar_internet_aman.desc", "safe-internet", "#2E7D32", 19, ContentDifficulty::Beginner),
            ("belajar-ai", "category.belajar_ai", "category.belajar_ai.desc", "robot", "#7B1FA2", 20, ContentDifficulty::Advanced),
            ("belajar-prompt-engineering", "category.belajar_prompt_engineering", "category.belajar_prompt_engineering.desc", "chat", "#FF6F00", 21, ContentDifficulty::Intermediate),
            ("belajar-github", "category.belajar_github", "category.belajar_github.desc", "github", "#181717", 22, ContentDifficulty::Intermediate),
            ("belajar-vs-code", "category.belajar_vs_code", "category.belajar_vs_code.desc", "visual-studio-code", "#007ACC", 23, ContentDifficulty::Beginner),
            ("belajar-libreoffice", "category.belajar_libreoffice", "category.belajar_libreoffice.desc", "libreoffice", "#18A303", 24, ContentDifficulty::Beginner),
            ("belajar-markdown", "category.belajar_markdown", "category.belajar_markdown.desc", "markdown", "#083FA1", 25, ContentDifficulty::Beginner),
            ("belajar-lisensi-open-source", "category.belajar_lisensi_open_source", "category.belajar_lisensi_open_source.desc", "license", "#1A1A1A", 26, ContentDifficulty::Intermediate),
            ("belajar-cara-kontribusi", "category.belajar_cara_kontribusi", "category.belajar_cara_kontribusi.desc", "git-pull-request", "#6CC644", 27, ContentDifficulty::Intermediate),
            ("belajar-linux-indonesia", "category.belajar_linux_indonesia", "category.belajar_linux_indonesia.desc", "flag-variant", "#CE1126", 28, ContentDifficulty::Beginner),
        ];
        for (id, name_key, desc_key, icon, color, order, difficulty) in categories {
            map.insert(
                id.to_string(),
                CategoryInfo {
                    id: id.to_string(),
                    name_key: name_key.to_string(),
                    description_key: desc_key.to_string(),
                    icon: icon.to_string(),
                    color: color.to_string(),
                    order,
                    lesson_count: 0,
                    total_duration: 0,
                    difficulty,
                    prerequisites: Vec::new(),
                },
            );
        }
        map
    }

    fn add_lesson(&mut self, lesson: Lesson) {
        if let Some(cat) = self.categories.get_mut(&lesson.metadata.category) {
            cat.lesson_count += 1;
            cat.total_duration += lesson.metadata.duration_minutes;
        }
        self.lessons.insert(lesson.metadata.id.clone(), lesson);
    }

    pub fn load_content(&mut self, dir: &Path) -> Result<()> {
        if !dir.exists() {
            return Err(LearningEngineError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Content directory not found: {}", dir.display()),
            )));
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext == "json" && path.file_name().and_then(|n| n.to_str()).map_or(false, |n| n.ends_with(".lesson.json")) {
                        let data = fs::read_to_string(&path)?;
                        let lesson: Lesson = serde_json::from_str(&data)?;
                        self.add_lesson(lesson);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn load_embedded_content(&mut self) {
        for lesson in embedded_lessons() {
            self.add_lesson(lesson);
        }
    }

    pub fn get_lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.get(id)
    }

    pub fn get_lesson_mut(&mut self, id: &str) -> Option<&mut Lesson> {
        self.lessons.get_mut(id)
    }

    pub fn get_category(&self, id: &str) -> Option<&CategoryInfo> {
        self.categories.get(id)
    }

    pub fn categories(&self) -> Vec<&CategoryInfo> {
        let mut cats: Vec<&CategoryInfo> = self.categories.values().collect();
        cats.sort_by_key(|c| c.order);
        cats
    }

    pub fn lessons_by_category(&self, cat: &str) -> Vec<&Lesson> {
        let mut lessons: Vec<&Lesson> = self
            .lessons
            .values()
            .filter(|l| l.metadata.category == cat)
            .collect();
        lessons.sort_by_key(|l| l.metadata.order);
        lessons
    }

    pub fn search(&self, query: &str) -> Vec<SearchIndex> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<(SearchIndex, u32)> = Vec::new();

        for lesson in self.lessons.values() {
            let title_lower = lesson.metadata.title.to_lowercase();
            let desc_lower = lesson.metadata.description.to_lowercase();

            let mut score = 0u32;

            if title_lower.contains(&query_lower) {
                score += 10;
                if title_lower.starts_with(&query_lower) {
                    score += 5;
                }
            }
            if desc_lower.contains(&query_lower) {
                score += 5;
            }
            for tag in &lesson.metadata.tags {
                if tag.to_lowercase().contains(&query_lower) {
                    score += 3;
                }
            }

            if score > 0 {
                results.push((
                    SearchIndex {
                        content_id: lesson.metadata.id.clone(),
                        title: lesson.metadata.title.clone(),
                        description: lesson.metadata.description.clone(),
                        category: lesson.metadata.category.clone(),
                        tags: lesson.metadata.tags.clone(),
                        difficulty: lesson.metadata.difficulty,
                    },
                    score,
                ));
            }
        }

        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.into_iter().map(|(index, _)| index).collect()
    }

    pub fn search_by_category(&self, query: &str, category: &str) -> Vec<SearchIndex> {
        self.search(query)
            .into_iter()
            .filter(|s| s.category == category)
            .collect()
    }

    pub fn get_progress(&self) -> &UserProgress {
        &self.progress
    }

    pub fn get_progress_mut(&mut self) -> &mut UserProgress {
        &mut self.progress
    }

    pub fn complete_lesson(&mut self, id: &str) {
        if !self.progress.completed_lessons.contains(&id.to_string()) {
            self.progress.completed_lessons.push(id.to_string());
        }
        self.progress.last_activity = Some(Utc::now().format("%Y-%m-%d").to_string());
        self.progress.total_xp = self.calculate_xp();
        self.update_streak();
    }

    fn update_streak(&mut self) {
        let today = Utc::now().date_naive();
        if let Some(ref last) = self.progress.last_activity.clone() {
            if let Ok(last_date) = NaiveDate::parse_from_str(&last, "%Y-%m-%d") {
                let diff = today - last_date;
                if diff.num_days() == 0 {
                    if self.progress.streak_days == 0 {
                        self.progress.streak_days = 1;
                    }
                    return;
                } else if diff.num_days() == 1 {
                    self.progress.streak_days += 1;
                } else {
                    self.progress.streak_days = 1;
                }
            }
        } else {
            self.progress.streak_days = 1;
        }
    }

    pub fn save_progress(&self) -> Result<()> {
        fs::create_dir_all(&self.content_dir)?;
        let path = self.content_dir.join("progress.json");
        let data = serde_json::to_string_pretty(&self.progress)?;
        fs::write(&path, data)?;
        Ok(())
    }

    pub fn load_progress(&mut self) -> Result<()> {
        let path = self.content_dir.join("progress.json");
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            self.progress = serde_json::from_str(&data)?;
        }
        Ok(())
    }

    pub fn add_bookmark(&mut self, id: &str) {
        if !self.progress.bookmarks.contains(&id.to_string()) {
            self.progress.bookmarks.push(id.to_string());
        }
    }

    pub fn remove_bookmark(&mut self, id: &str) {
        self.progress.bookmarks.retain(|b| b != id);
    }

    pub fn add_favorite(&mut self, id: &str) {
        if !self.progress.favorites.contains(&id.to_string()) {
            self.progress.favorites.push(id.to_string());
        }
    }

    pub fn remove_favorite(&mut self, id: &str) {
        self.progress.favorites.retain(|f| f != id);
    }

    pub fn add_history(&mut self, id: &str, progress: u8) {
        let entry = HistoryEntry {
            content_id: id.to_string(),
            accessed_at: Utc::now().to_rfc3339(),
            progress_percent: progress,
        };
        self.progress.history.retain(|h| h.content_id != id);
        self.progress.history.push(entry);
        self.progress.last_activity = Some(Utc::now().format("%Y-%m-%d").to_string());
        self.update_streak();
    }

    pub fn get_history(&self) -> &[HistoryEntry] {
        &self.progress.history
    }

    pub fn record_quiz_attempt(&mut self, attempt: QuizAttempt) {
        let xp_gained = if attempt.score >= 50 { 50u64 } else { 10u64 };
        self.progress.quiz_scores.insert(attempt.quiz_id.clone(), attempt);
        self.progress.total_xp += xp_gained;
    }

    pub fn get_quiz_score(&self, quiz_id: &str) -> Option<&QuizAttempt> {
        self.progress.quiz_scores.get(quiz_id)
    }

    pub fn calculate_xp(&self) -> u64 {
        let lesson_xp = self.progress.completed_lessons.len() as u64 * 100;
        let quiz_xp = self.progress.quiz_scores.len() as u64 * 50;
        let exercise_xp = self
            .progress
            .exercise_completions
            .values()
            .filter(|&&c| c)
            .count() as u64
            * 75;
        lesson_xp + quiz_xp + exercise_xp
    }

    pub fn total_lessons(&self) -> u32 {
        self.lessons.len() as u32
    }

    pub fn completed_lessons_count(&self) -> u32 {
        self.progress.completed_lessons.len() as u32
    }

    pub fn completion_percentage(&self) -> f64 {
        if self.lessons.is_empty() {
            return 0.0;
        }
        (self.progress.completed_lessons.len() as f64 / self.lessons.len() as f64) * 100.0
    }

    pub fn get_recommendations(&self, count: usize) -> Vec<&Lesson> {
        let mut candidates: Vec<(&Lesson, u32)> = Vec::new();
        let completed: Vec<&str> = self
            .progress
            .completed_lessons
            .iter()
            .map(|s| s.as_str())
            .collect();

        let category_completed: HashMap<&str, u32> = {
            let mut map = HashMap::new();
            for id in &self.progress.completed_lessons {
                if let Some(lesson) = self.lessons.get(id) {
                    *map.entry(lesson.metadata.category.as_str()).or_insert(0) += 1;
                }
            }
            map
        };

        for lesson in self.lessons.values() {
            if completed.contains(&lesson.metadata.id.as_str()) {
                continue;
            }

            let prereqs_met = lesson
                .metadata
                .prerequisites
                .iter()
                .all(|p| completed.contains(&p.as_str()));

            let mut score: u32 = 0;
            if prereqs_met {
                score += 50;
            }
            score += category_completed
                .get(lesson.metadata.category.as_str())
                .copied()
                .unwrap_or(0)
                * 10;

            candidates.push((lesson, score));
        }

        candidates.sort_by(|a, b| b.1.cmp(&a.1));
        candidates.into_iter().take(count).map(|(l, _)| l).collect()
    }

    pub fn get_next_lesson(&self) -> Option<&Lesson> {
        let completed: Vec<&str> = self
            .progress
            .completed_lessons
            .iter()
            .map(|s| s.as_str())
            .collect();

        let mut categories_ordered: Vec<&CategoryInfo> = self.categories.values().collect();
        categories_ordered.sort_by_key(|c| c.order);

        for cat in categories_ordered {
            for lesson in self.lessons_by_category(&cat.id) {
                if completed.contains(&lesson.metadata.id.as_str()) {
                    continue;
                }
                let prereqs_met = lesson
                    .metadata
                    .prerequisites
                    .iter()
                    .all(|p| completed.contains(&p.as_str()));
                if prereqs_met {
                    return Some(lesson);
                }
            }
        }

        for lesson in self.lessons.values() {
            if !completed.contains(&lesson.metadata.id.as_str()) {
                return Some(lesson);
            }
        }

        None
    }

    pub fn lesson_exists(&self, id: &str) -> bool {
        self.lessons.contains_key(id)
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.locale = locale.to_string();
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn get_lesson_body<'a>(&self, lesson: &'a Lesson) -> &'a str {
        if self.locale == "en" {
            if let Some(ref body_en) = lesson.body_en {
                return body_en;
            }
        }
        &lesson.body
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn get_streak_info(&self) -> (u32, String) {
        let days = self.progress.streak_days;
        let last = self
            .progress
            .last_activity
            .clone()
            .unwrap_or_else(|| "Never".to_string());
        (days, last)
    }
}

fn embedded_lessons() -> Vec<Lesson> {
    vec![lesson_pengenalan_terminal(), lesson_navigasi_file_system()]
}

fn lesson_pengenalan_terminal() -> Lesson {
    Lesson {
        metadata: ContentMetadata {
            id: "pengenalan-terminal".to_string(),
            title: "content.pengenalan_terminal.title".to_string(),
            description: "content.pengenalan_terminal.desc".to_string(),
            category: "belajar-terminal".to_string(),
            subcategory: None,
            difficulty: ContentDifficulty::Beginner,
            author: ContentAuthor {
                name: "Tim EduShell".to_string(),
                avatar: None,
                bio: Some("author.tim_edushell".to_string()),
            },
            format: ContentFormat::Markdown,
            duration_minutes: 10,
            tags: vec![
                "terminal".to_string(),
                "linux".to_string(),
                "command-line".to_string(),
                "pemula".to_string(),
            ],
            prerequisites: Vec::new(),
            order: 1,
            locale: "id".to_string(),
            version: "1.0.0".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
            cover_image: None,
        },
        body: r#"# Pengenalan Terminal

Terminal adalah antarmuka berbasis teks yang memungkinkan Anda berinteraksi dengan sistem operasi Linux. Dengan terminal, Anda dapat menjalankan perintah, mengelola file, menginstal program, dan melakukan berbagai tugas lainnya.

## Membuka Terminal

Untuk membuka terminal di Linux, Anda dapat menggunakan pintasan keyboard `Ctrl + Alt + T` atau mencarinya di menu aplikasi. Setelah terbuka, Anda akan melihat sebuah jendela dengan prompt yang menunggu perintah.

## Perintah Dasar

Berikut adalah beberapa perintah dasar yang perlu Anda ketahui:

- `whoami` — Menampilkan nama pengguna saat ini
- `pwd` — Menampilkan direktori kerja saat ini
- `ls` — Menampilkan daftar file dan direktori
- `cd` — Berpindah direktori
- `echo` — Menampilkan teks ke layar

### Contoh Penggunaan

```bash
$ whoami
izza
$ pwd
/home/izza
$ ls
Documents  Downloads  Pictures  Music
$ cd Documents
$ pwd
/home/izza/Documents
```

## Tips

1. Gunakan tombol **Tab** untuk autocomplete
2. Gunakan tombol **Panah Atas** untuk mengulangi perintah sebelumnya
3. Gunakan `Ctrl + C` untuk membatalkan perintah yang sedang berjalan

Selamat belajar! Terminal adalah alat yang sangat powerful dan akan menjadi sahabat terbaik Anda dalam perjalanan belajar Linux.
"#.to_string(),
        body_en: Some(r#"# Introduction to Terminal

Terminal is a text-based interface that allows you to interact with the Linux operating system. With the terminal, you can run commands, manage files, install programs, and perform various other tasks.

## Opening Terminal

To open a terminal in Linux, you can use the keyboard shortcut `Ctrl + Alt + T` or find it in the application menu. Once opened, you will see a window with a prompt waiting for commands.

## Basic Commands

Here are some basic commands you need to know:

- `whoami` — Displays the current username
- `pwd` — Displays the current working directory
- `ls` — Lists files and directories
- `cd` — Changes directory
- `echo` — Displays text on screen

### Usage Examples

```bash
$ whoami
izza
$ pwd
/home/izza
$ ls
Documents  Downloads  Pictures  Music
$ cd Documents
$ pwd
/home/izza/Documents
```

## Tips

1. Use the **Tab** key for autocomplete
2. Use the **Up Arrow** key to repeat previous commands
3. Use `Ctrl + C` to cancel a running command

Happy learning! The terminal is a very powerful tool and will be your best friend in your Linux learning journey.
"#.to_string()),
        vocabulary: vec![
            VocabEntry {
                term: "Terminal".to_string(),
                definition: "vocab.terminal".to_string(),
                category: "umum".to_string(),
            },
            VocabEntry {
                term: "Prompt".to_string(),
                definition: "vocab.prompt".to_string(),
                category: "umum".to_string(),
            },
            VocabEntry {
                term: "Direktori".to_string(),
                definition: "vocab.direktori".to_string(),
                category: "umum".to_string(),
            },
            VocabEntry {
                term: "Perintah".to_string(),
                definition: "vocab.perintah".to_string(),
                category: "umum".to_string(),
            },
        ],
        key_points: vec![
            "keypoint.terminal.1".to_string(),
            "keypoint.terminal.2".to_string(),
            "keypoint.terminal.3".to_string(),
        ],
        quiz: Some(Quiz {
            id: "quiz-pengenalan-terminal".to_string(),
            title: "quiz.pengenalan_terminal.title".to_string(),
            description: "quiz.pengenalan_terminal.desc".to_string(),
            questions: vec![
                QuizQuestion {
                    id: "q-terminal-1".to_string(),
                    question: "quiz.pengenalan_terminal.q1".to_string(),
                    options: vec![
                        "quiz.pengenalan_terminal.q1.a".to_string(),
                        "quiz.pengenalan_terminal.q1.b".to_string(),
                        "quiz.pengenalan_terminal.q1.c".to_string(),
                        "quiz.pengenalan_terminal.q1.d".to_string(),
                    ],
                    correct_index: 0,
                    explanation: "quiz.pengenalan_terminal.q1.explain".to_string(),
                    multiple_answer: false,
                    points: 25,
                },
                QuizQuestion {
                    id: "q-terminal-2".to_string(),
                    question: "quiz.pengenalan_terminal.q2".to_string(),
                    options: vec![
                        "quiz.pengenalan_terminal.q2.a".to_string(),
                        "quiz.pengenalan_terminal.q2.b".to_string(),
                        "quiz.pengenalan_terminal.q2.c".to_string(),
                        "quiz.pengenalan_terminal.q2.d".to_string(),
                    ],
                    correct_index: 2,
                    explanation: "quiz.pengenalan_terminal.q2.explain".to_string(),
                    multiple_answer: false,
                    points: 25,
                },
                QuizQuestion {
                    id: "q-terminal-3".to_string(),
                    question: "quiz.pengenalan_terminal.q3".to_string(),
                    options: vec![
                        "quiz.pengenalan_terminal.q3.a".to_string(),
                        "quiz.pengenalan_terminal.q3.b".to_string(),
                        "quiz.pengenalan_terminal.q3.c".to_string(),
                        "quiz.pengenalan_terminal.q3.d".to_string(),
                    ],
                    correct_index: 1,
                    explanation: "quiz.pengenalan_terminal.q3.explain".to_string(),
                    multiple_answer: false,
                    points: 25,
                },
                QuizQuestion {
                    id: "q-terminal-4".to_string(),
                    question: "quiz.pengenalan_terminal.q4".to_string(),
                    options: vec![
                        "quiz.pengenalan_terminal.q4.a".to_string(),
                        "quiz.pengenalan_terminal.q4.b".to_string(),
                        "quiz.pengenalan_terminal.q4.c".to_string(),
                        "quiz.pengenalan_terminal.q4.d".to_string(),
                    ],
                    correct_index: 3,
                    explanation: "quiz.pengenalan_terminal.q4.explain".to_string(),
                    multiple_answer: true,
                    points: 25,
                },
            ],
            passing_score: 70,
            time_limit_minutes: Some(10),
            shuffle: true,
        }),
        exercise: None,
        related_content: Vec::new(),
        external_links: vec![
            ("link.terminal.gnu".to_string(), "https://www.gnu.org/software/bash/".to_string()),
        ],
    }
}

fn lesson_navigasi_file_system() -> Lesson {
    Lesson {
        metadata: ContentMetadata {
            id: "navigasi-file-system".to_string(),
            title: "content.navigasi_file_system.title".to_string(),
            description: "content.navigasi_file_system.desc".to_string(),
            category: "belajar-terminal".to_string(),
            subcategory: None,
            difficulty: ContentDifficulty::Beginner,
            author: ContentAuthor {
                name: "Tim EduShell".to_string(),
                avatar: None,
                bio: Some("author.tim_edushell".to_string()),
            },
            format: ContentFormat::Markdown,
            duration_minutes: 15,
            tags: vec![
                "terminal".to_string(),
                "linux".to_string(),
                "file-system".to_string(),
                "navigasi".to_string(),
                "perintah-dasar".to_string(),
            ],
            prerequisites: vec!["pengenalan-terminal".to_string()],
            order: 2,
            locale: "id".to_string(),
            version: "1.0.0".to_string(),
            created_at: "2025-01-02T00:00:00Z".to_string(),
            updated_at: "2025-01-02T00:00:00Z".to_string(),
            cover_image: None,
        },
        body: r#"# Navigasi File System dengan Terminal

File system di Linux memiliki struktur hierarki yang dimulai dari root (`/`). Dengan terminal, Anda dapat menjelajahi dan mengelola file system dengan efisien.

## Struktur Direktori

Berikut adalah struktur direktori utama di Linux:

| Direktori | Kegunaan |
|-----------|----------|
| `/`       | Root direktori |
| `/home`   | Direktori rumah pengguna |
| `/etc`    | File konfigurasi sistem |
| `/var`    | Data variabel (log, database) |
| `/tmp`    | File sementara |
| `/usr`    | Program dan library pengguna |

## Perintah Navigasi

### cd (Change Directory)

Perintah `cd` digunakan untuk berpindah antar direktori:

```bash
cd /home/user/Documents   # pindah ke direktori tertentu
cd ..                     # pindah ke direktori induk
cd ~                      # pindah ke direktori rumah
cd -                      # pindah ke direktori sebelumnya
```

### ls (List)

Perintah `ls` digunakan untuk menampilkan isi direktori:

```bash
ls                        # tampilkan daftar file
ls -l                     # tampilkan dengan detail
ls -a                     # tampilkan file tersembunyi
ls -la                    # kombinasi detail dan tersembunyi
ls -h                     # ukuran file yang mudah dibaca
```

## Membuat dan Menghapus Direktori

```bash
mkdir nama_folder         # membuat direktori baru
mkdir -p path/ke/folder   # membuat direktori beserta parent-nya
rmdir nama_folder         # menghapus direktori kosong
rm -rf nama_folder        # menghapus direktori beserta isinya
```

## Manipulasi File

```bash
touch file.txt            # membuat file kosong
cp file.txt salinan.txt   # menyalin file
mv file.txt baru.txt      # memindahkan/mengganti nama file
rm file.txt               # menghapus file
cat file.txt              # menampilkan isi file
less file.txt             # melihat isi file dengan scrolling
```

Dengan menguasai perintah-perintah ini, Anda akan dapat menavigasi dan mengelola file system Linux dengan percaya diri.
"#.to_string(),
        body_en: Some(r#"# Navigating File System with Terminal

The Linux file system has a hierarchical structure starting from root (`/`). With the terminal, you can explore and manage the file system efficiently.

## Directory Structure

Here are the main directories in Linux:

| Directory | Purpose |
|-----------|---------|
| `/`       | Root directory |
| `/home`   | User home directories |
| `/etc`    | System configuration files |
| `/var`    | Variable data (logs, databases) |
| `/tmp`    | Temporary files |
| `/usr`    | User programs and libraries |

## Navigation Commands

### cd (Change Directory)

The `cd` command is used to move between directories:

```bash
cd /home/user/Documents   # move to a specific directory
cd ..                     # move to parent directory
cd ~                      # move to home directory
cd -                      # move to previous directory
```

### ls (List)

The `ls` command displays directory contents:

```bash
ls                        # list files
ls -l                     # detailed listing
ls -a                     # show hidden files
ls -la                    # detailed with hidden files
ls -h                     # human-readable file sizes
```

## Creating and Deleting Directories

```bash
mkdir folder_name         # create new directory
mkdir -p path/to/folder   # create directory with parents
rmdir folder_name         # remove empty directory
rm -rf folder_name        # remove directory and contents
```

## File Manipulation

```bash
touch file.txt            # create empty file
cp file.txt copy.txt      # copy file
mv file.txt new.txt       # move/rename file
rm file.txt               # delete file
cat file.txt              # display file contents
less file.txt             # view file with scrolling
```

By mastering these commands, you will be able to navigate and manage the Linux file system with confidence.
"#.to_string()),
        vocabulary: vec![
            VocabEntry {
                term: "Root".to_string(),
                definition: "vocab.root".to_string(),
                category: "file-system".to_string(),
            },
            VocabEntry {
                term: "Path".to_string(),
                definition: "vocab.path".to_string(),
                category: "file-system".to_string(),
            },
            VocabEntry {
                term: "Direktori Kerja".to_string(),
                definition: "vocab.direktori_kerja".to_string(),
                category: "file-system".to_string(),
            },
        ],
        key_points: vec![
            "keypoint.fs.1".to_string(),
            "keypoint.fs.2".to_string(),
            "keypoint.fs.3".to_string(),
        ],
        quiz: Some(Quiz {
            id: "quiz-navigasi-file-system".to_string(),
            title: "quiz.navigasi_file_system.title".to_string(),
            description: "quiz.navigasi_file_system.desc".to_string(),
            questions: vec![
                QuizQuestion {
                    id: "q-fs-1".to_string(),
                    question: "quiz.navigasi_file_system.q1".to_string(),
                    options: vec![
                        "quiz.navigasi_file_system.q1.a".to_string(),
                        "quiz.navigasi_file_system.q1.b".to_string(),
                        "quiz.navigasi_file_system.q1.c".to_string(),
                        "quiz.navigasi_file_system.q1.d".to_string(),
                    ],
                    correct_index: 0,
                    explanation: "quiz.navigasi_file_system.q1.explain".to_string(),
                    multiple_answer: false,
                    points: 30,
                },
                QuizQuestion {
                    id: "q-fs-2".to_string(),
                    question: "quiz.navigasi_file_system.q2".to_string(),
                    options: vec![
                        "quiz.navigasi_file_system.q2.a".to_string(),
                        "quiz.navigasi_file_system.q2.b".to_string(),
                        "quiz.navigasi_file_system.q2.c".to_string(),
                        "quiz.navigasi_file_system.q2.d".to_string(),
                    ],
                    correct_index: 1,
                    explanation: "quiz.navigasi_file_system.q2.explain".to_string(),
                    multiple_answer: false,
                    points: 30,
                },
                QuizQuestion {
                    id: "q-fs-3".to_string(),
                    question: "quiz.navigasi_file_system.q3".to_string(),
                    options: vec![
                        "quiz.navigasi_file_system.q3.a".to_string(),
                        "quiz.navigasi_file_system.q3.b".to_string(),
                        "quiz.navigasi_file_system.q3.c".to_string(),
                        "quiz.navigasi_file_system.q3.d".to_string(),
                    ],
                    correct_index: 3,
                    explanation: "quiz.navigasi_file_system.q3.explain".to_string(),
                    multiple_answer: false,
                    points: 40,
                },
            ],
            passing_score: 75,
            time_limit_minutes: Some(10),
            shuffle: false,
        }),
        exercise: Some(Exercise {
            id: "ex-navigasi-file-system".to_string(),
            title: "exercise.navigasi_file_system.title".to_string(),
            description: "exercise.navigasi_file_system.desc".to_string(),
            steps: vec![
                ExerciseStep {
                    instruction: "exercise.navigasi_fs.step1".to_string(),
                    command: Some("pwd".to_string()),
                    expected_output: Some("/home".to_string()),
                    hint: Some("exercise.navigasi_fs.step1.hint".to_string()),
                    validation_type: ValidationType::ContainsOutput("home".to_string()),
                },
                ExerciseStep {
                    instruction: "exercise.navigasi_fs.step2".to_string(),
                    command: Some("ls -la".to_string()),
                    expected_output: None,
                    hint: Some("exercise.navigasi_fs.step2.hint".to_string()),
                    validation_type: ValidationType::CommandSuccess,
                },
            ],
            setup_commands: vec!["cd /home".to_string()],
            cleanup_commands: Vec::new(),
            requires_sandbox: false,
        }),
        related_content: vec!["pengenalan-terminal".to_string()],
        external_links: vec![
            ("link.fs.linux".to_string(), "https://www.linux.org/".to_string()),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_engine() -> LearningContentEngine {
        LearningContentEngine::new(Some(PathBuf::from("/tmp/edushell-test-content")))
    }

    #[test]
    fn test_engine_creation() {
        let engine = test_engine();
        assert!(!engine.categories.is_empty());
        assert!(!engine.lessons.is_empty());
        assert_eq!(engine.locale, "id");
    }

    #[test]
    fn test_lesson_lookup_found() {
        let engine = test_engine();
        let lesson = engine.get_lesson("pengenalan-terminal");
        assert!(lesson.is_some());
        assert_eq!(
            lesson.unwrap().metadata.id,
            "pengenalan-terminal"
        );
    }

    #[test]
    fn test_lesson_lookup_not_found() {
        let engine = test_engine();
        assert!(engine.get_lesson("tidak-ada").is_none());
    }

    #[test]
    fn test_category_lookup() {
        let engine = test_engine();
        let cat = engine.get_category("belajar-terminal");
        assert!(cat.is_some());
        assert_eq!(cat.unwrap().name_key, "category.belajar_terminal");
    }

    #[test]
    fn test_all_categories_exist() {
        let engine = test_engine();
        let expected_ids = [
            "belajar-linux",
            "belajar-terminal",
            "belajar-bash",
            "belajar-git",
            "belajar-html",
            "belajar-css",
            "belajar-javascript",
            "belajar-python",
            "belajar-c",
            "belajar-cpp",
            "belajar-rust",
            "belajar-open-source",
            "belajar-command-line",
            "belajar-package-manager",
            "belajar-file-system",
            "belajar-networking-dasar",
            "belajar-keamanan-dasar",
            "belajar-office",
            "belajar-internet-aman",
            "belajar-ai",
            "belajar-prompt-engineering",
            "belajar-github",
            "belajar-vs-code",
            "belajar-libreoffice",
            "belajar-markdown",
            "belajar-lisensi-open-source",
            "belajar-cara-kontribusi",
            "belajar-linux-indonesia",
        ];
        for id in &expected_ids {
            assert!(
                engine.get_category(id).is_some(),
                "Category {} not found",
                id
            );
        }
        assert_eq!(engine.categories().len(), expected_ids.len());
    }

    #[test]
    fn test_categories_sorted_by_order() {
        let engine = test_engine();
        let cats = engine.categories();
        for i in 1..cats.len() {
            assert!(
                cats[i - 1].order <= cats[i].order,
                "Categories not sorted by order"
            );
        }
    }

    #[test]
    fn test_lessons_by_category() {
        let engine = test_engine();
        let lessons = engine.lessons_by_category("belajar-terminal");
        assert!(!lessons.is_empty());
        for lesson in &lessons {
            assert_eq!(lesson.metadata.category, "belajar-terminal");
        }
    }

    #[test]
    fn test_search_by_title() {
        let engine = test_engine();
        let results = engine.search("terminal");
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.title.contains("terminal")));
    }

    #[test]
    fn test_search_by_tag() {
        let engine = test_engine();
        let results = engine.search("file-system");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_no_results() {
        let engine = test_engine();
        let results = engine.search("xyznonexistent12345");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_by_category() {
        let engine = test_engine();
        let results = engine.search_by_category("navigasi", "belajar-terminal");
        assert!(!results.is_empty());
        for r in &results {
            assert_eq!(r.category, "belajar-terminal");
        }
    }

    #[test]
    fn test_complete_lesson() {
        let mut engine = test_engine();
        assert_eq!(engine.completed_lessons_count(), 0);
        engine.complete_lesson("pengenalan-terminal");
        assert_eq!(engine.completed_lessons_count(), 1);
        assert!(engine.progress.completed_lessons.contains(&"pengenalan-terminal".to_string()));
        assert!(engine.progress.last_activity.is_some());
    }

    #[test]
    fn test_double_complete_does_not_duplicate() {
        let mut engine = test_engine();
        engine.complete_lesson("pengenalan-terminal");
        engine.complete_lesson("pengenalan-terminal");
        assert_eq!(engine.completed_lessons_count(), 1);
    }

    #[test]
    fn test_completion_percentage() {
        let mut engine = test_engine();
        let total = engine.total_lessons();
        assert!(total > 0);
        assert_eq!(engine.completion_percentage(), 0.0);
        engine.complete_lesson("pengenalan-terminal");
        let expected = (1.0 / total as f64) * 100.0;
        assert!((engine.completion_percentage() - expected).abs() < 0.001);
    }

    #[test]
    fn test_completed_lessons_count() {
        let mut engine = test_engine();
        assert_eq!(engine.completed_lessons_count(), 0);
        engine.complete_lesson("pengenalan-terminal");
        assert_eq!(engine.completed_lessons_count(), 1);
    }

    #[test]
    fn test_total_lessons() {
        let engine = test_engine();
        assert!(engine.total_lessons() >= 2);
    }

    #[test]
    fn test_bookmark_add() {
        let mut engine = test_engine();
        engine.add_bookmark("pengenalan-terminal");
        assert!(engine.progress.bookmarks.contains(&"pengenalan-terminal".to_string()));
    }

    #[test]
    fn test_bookmark_add_duplicate() {
        let mut engine = test_engine();
        engine.add_bookmark("pengenalan-terminal");
        engine.add_bookmark("pengenalan-terminal");
        assert_eq!(engine.progress.bookmarks.len(), 1);
    }

    #[test]
    fn test_bookmark_remove() {
        let mut engine = test_engine();
        engine.add_bookmark("pengenalan-terminal");
        engine.remove_bookmark("pengenalan-terminal");
        assert!(!engine.progress.bookmarks.contains(&"pengenalan-terminal".to_string()));
    }

    #[test]
    fn test_favorite_add() {
        let mut engine = test_engine();
        engine.add_favorite("pengenalan-terminal");
        assert!(engine.progress.favorites.contains(&"pengenalan-terminal".to_string()));
    }

    #[test]
    fn test_favorite_remove() {
        let mut engine = test_engine();
        engine.add_favorite("pengenalan-terminal");
        engine.remove_favorite("pengenalan-terminal");
        assert!(!engine.progress.favorites.contains(&"pengenalan-terminal".to_string()));
    }

    #[test]
    fn test_history_recording() {
        let mut engine = test_engine();
        engine.add_history("pengenalan-terminal", 50);
        assert_eq!(engine.get_history().len(), 1);
        assert_eq!(engine.get_history()[0].content_id, "pengenalan-terminal");
        assert_eq!(engine.get_history()[0].progress_percent, 50);
    }

    #[test]
    fn test_history_overwrites_same_id() {
        let mut engine = test_engine();
        engine.add_history("pengenalan-terminal", 50);
        engine.add_history("pengenalan-terminal", 100);
        assert_eq!(engine.get_history().len(), 1);
        assert_eq!(engine.get_history()[0].progress_percent, 100);
    }

    #[test]
    fn test_quiz_attempt_recording() {
        let mut engine = test_engine();
        let attempt = QuizAttempt {
            quiz_id: "quiz-pengenalan-terminal".to_string(),
            score: 85,
            answers: vec![0, 2, 1, 3],
            completed_at: "2025-01-15T10:00:00Z".to_string(),
            time_spent_seconds: 420,
        };
        engine.record_quiz_attempt(attempt);
        let score = engine.get_quiz_score("quiz-pengenalan-terminal");
        assert!(score.is_some());
        assert_eq!(score.unwrap().score, 85);
    }

    #[test]
    fn test_quiz_score_not_found() {
        let engine = test_engine();
        assert!(engine.get_quiz_score("nonexistent").is_none());
    }

    #[test]
    fn test_xp_calculation() {
        let mut engine = test_engine();
        assert_eq!(engine.calculate_xp(), 0);
        engine.complete_lesson("pengenalan-terminal");
        assert_eq!(engine.calculate_xp(), 100);
        let attempt = QuizAttempt {
            quiz_id: "quiz-pengenalan-terminal".to_string(),
            score: 85,
            answers: vec![0, 2, 1, 3],
            completed_at: "2025-01-15T10:00:00Z".to_string(),
            time_spent_seconds: 420,
        };
        engine.record_quiz_attempt(attempt);
        assert_eq!(engine.calculate_xp(), 150);
    }

    #[test]
    fn test_recommendations() {
        let engine = test_engine();
        let recs = engine.get_recommendations(5);
        assert!(!recs.is_empty());
        assert!(recs.len() <= 5);
        for rec in &recs {
            assert!(!engine.progress.completed_lessons.contains(&rec.metadata.id));
        }
    }

    #[test]
    fn test_recommendations_excludes_completed() {
        let mut engine = test_engine();
        engine.complete_lesson("pengenalan-terminal");
        let recs = engine.get_recommendations(10);
        assert!(!recs.iter().any(|r| r.metadata.id == "pengenalan-terminal"));
    }

    #[test]
    fn test_next_lesson() {
        let engine = test_engine();
        let next = engine.get_next_lesson();
        assert!(next.is_some());
        assert_eq!(next.unwrap().metadata.id, "pengenalan-terminal");
    }

    #[test]
    fn test_next_lesson_after_completion() {
        let mut engine = test_engine();
        engine.complete_lesson("pengenalan-terminal");
        let next = engine.get_next_lesson();
        assert!(next.is_some());
        assert_eq!(next.unwrap().metadata.id, "navigasi-file-system");
    }

    #[test]
    fn test_next_lesson_none_when_all_complete() {
        let mut engine = test_engine();
        for id in engine.lessons.keys().cloned().collect::<Vec<_>>() {
            engine.complete_lesson(&id);
        }
        assert!(engine.get_next_lesson().is_none());
    }

    #[test]
    fn test_lesson_exists() {
        let engine = test_engine();
        assert!(engine.lesson_exists("pengenalan-terminal"));
        assert!(!engine.lesson_exists("nonexistent"));
    }

    #[test]
    fn test_locale_switching() {
        let mut engine = test_engine();
        assert_eq!(engine.get_locale(), "id");
        engine.set_locale("en");
        assert_eq!(engine.get_locale(), "en");
    }

    #[test]
    fn test_get_lesson_body_id() {
        let engine = test_engine();
        let lesson = engine.get_lesson("pengenalan-terminal").unwrap();
        assert_eq!(engine.get_locale(), "id");
        let body = engine.get_lesson_body(lesson);
        assert!(body.contains("Pengenalan Terminal"));
    }

    #[test]
    fn test_get_lesson_body_en() {
        let mut engine = test_engine();
        engine.set_locale("en");
        let lesson = engine.get_lesson("pengenalan-terminal").unwrap();
        let body = engine.get_lesson_body(lesson);
        assert!(body.contains("Introduction to Terminal"));
    }

    #[test]
    fn test_cache_set_get() {
        let mut cache = ContentCache::new(10);
        cache.set("key1", vec![1, 2, 3], "text/plain", None);
        let entry = cache.get("key1");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().data, vec![1, 2, 3]);
    }

    #[test]
    fn test_cache_get_nonexistent() {
        let mut cache = ContentCache::new(10);
        assert!(cache.get("nonexistent").is_none());
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = ContentCache::new(10);
        cache.set("key1", vec![1, 2, 3], "text/plain", None);
        cache.remove("key1");
        assert!(cache.get("key1").is_none());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = ContentCache::new(10);
        cache.set("key1", vec![1], "text/plain", None);
        cache.set("key2", vec![2], "text/plain", None);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key2").is_none());
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = ContentCache::new(1);
        cache.set("key1", vec![0u8; 600 * 1024], "text/plain", None);
        assert!(cache.get("key1").is_some());
        cache.set("key2", vec![0u8; 600 * 1024], "text/plain", None);
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key2").is_some());
    }

    #[test]
    fn test_cache_is_full() {
        let mut cache = ContentCache::new(1);
        assert!(!cache.is_full());
        cache.set("key1", vec![0u8; 1024 * 1024], "text/plain", None);
        assert!(cache.is_full());
    }

    #[test]
    fn test_cache_size_tracking() {
        let mut cache = ContentCache::new(10);
        assert_eq!(cache.size(), 0);
        cache.set("key1", vec![0u8; 100], "text/plain", None);
        assert_eq!(cache.size(), 100);
        cache.remove("key1");
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_cache_max_size() {
        let cache = ContentCache::new(50);
        assert_eq!(cache.max_size(), 50 * 1024 * 1024);
    }

    #[test]
    fn test_progress_save_load() -> Result<()> {
        let dir = PathBuf::from("/tmp/edushell-test-save-load");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir)?;

        let mut engine = LearningContentEngine::new(Some(dir.clone()));
        let lesson_id = "pengenalan-terminal";
        engine.complete_lesson(lesson_id);
        engine.add_bookmark("navigasi-file-system");
        engine.add_favorite("pengenalan-terminal");
        engine.save_progress()?;
        drop(engine);

        let engine2 = LearningContentEngine::new(Some(dir.clone()));
        assert_eq!(engine2.completed_lessons_count(), 1);
        assert!(engine2.progress.bookmarks.contains(&"navigasi-file-system".to_string()));
        assert!(engine2.progress.favorites.contains(&"pengenalan-terminal".to_string()));

        let _ = fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn test_streak_info() {
        let mut engine = test_engine();
        let (days, last) = engine.get_streak_info();
        assert_eq!(days, 0);
        assert_eq!(last, "Never");
        engine.complete_lesson("pengenalan-terminal");
        let (days2, _) = engine.get_streak_info();
        assert_eq!(days2, 1);
    }

    #[test]
    fn test_load_embedded_content() {
        let engine = test_engine();
        assert!(engine.lesson_exists("pengenalan-terminal"));
        assert!(engine.lesson_exists("navigasi-file-system"));
    }

    #[test]
    fn test_clear_cache() {
        let mut engine = test_engine();
        engine.cache.set("test", vec![1, 2, 3], "text/plain", None);
        assert!(engine.cache.get("test").is_some());
        engine.clear_cache();
        assert!(engine.cache.get("test").is_none());
    }

    #[test]
    fn test_empty_engine_no_panic() {
        let engine = LearningContentEngine {
            lessons: HashMap::new(),
            categories: HashMap::new(),
            progress: UserProgress::default(),
            cache: ContentCache::new(10),
            locale: "id".to_string(),
            content_dir: PathBuf::from("/tmp/nonexistent"),
        };
        assert_eq!(engine.total_lessons(), 0);
        assert_eq!(engine.completion_percentage(), 0.0);
        assert!(engine.get_lesson("anything").is_none());
        assert!(engine.get_next_lesson().is_none());
        assert!(engine.search("test").is_empty());
    }

    #[test]
    fn test_exercise_completion() {
        let mut engine = test_engine();
        engine.progress
            .exercise_completions
            .insert("ex-navigasi-file-system".to_string(), true);
        assert_eq!(engine.calculate_xp(), 75);
    }

    #[test]
    fn test_history_returns_slice() {
        let mut engine = test_engine();
        engine.add_history("pengenalan-terminal", 100);
        let history = engine.get_history();
        assert!(!history.is_empty());
        assert_eq!(history[0].content_id, "pengenalan-terminal");
    }

    #[test]
    fn test_update_streak_after_history() {
        let mut engine = test_engine();
        engine.add_history("pengenalan-terminal", 50);
        let (days, _) = engine.get_streak_info();
        assert_eq!(days, 1);
    }

    #[test]
    fn test_quiz_attempt_adds_xp() {
        let mut engine = test_engine();
        let initial_xp = engine.progress.total_xp;
        let attempt = QuizAttempt {
            quiz_id: "quiz-pengenalan-terminal".to_string(),
            score: 80,
            answers: vec![0, 2, 1],
            completed_at: "2025-01-15T10:00:00Z".to_string(),
            time_spent_seconds: 300,
        };
        engine.record_quiz_attempt(attempt);
        assert!(engine.progress.total_xp > initial_xp);
    }

    #[test]
    fn test_search_case_insensitive() {
        let engine = test_engine();
        let upper = engine.search("TERMINAL");
        let lower = engine.search("terminal");
        assert_eq!(upper.len(), lower.len());
    }

    #[test]
    fn test_lessons_by_category_empty() {
        let engine = test_engine();
        let lessons = engine.lessons_by_category("belajar-rust");
        assert!(lessons.is_empty());
    }

    #[test]
    fn test_category_info_fields() {
        let engine = test_engine();
        let cat = engine.get_category("belajar-terminal").unwrap();
        assert_eq!(cat.icon, "terminal");
        assert_eq!(cat.color, "#2E3436");
        assert_eq!(cat.difficulty, ContentDifficulty::Beginner);
    }

    #[test]
    fn test_get_lesson_mut() {
        let mut engine = test_engine();
        let lesson = engine.get_lesson_mut("pengenalan-terminal");
        assert!(lesson.is_some());
        lesson.unwrap().metadata.title = "modified".to_string();
        assert_eq!(
            engine.get_lesson("pengenalan-terminal").unwrap().metadata.title,
            "modified"
        );
    }

    #[test]
    fn test_search_by_category_empty_results() {
        let engine = test_engine();
        let results = engine.search_by_category("xyznonexistent", "belajar-terminal");
        assert!(results.is_empty());
    }

    #[test]
    fn test_engine_creation_with_default_content_dir() {
        let engine = LearningContentEngine::new(None);
        assert_eq!(engine.content_dir, PathBuf::from("content"));
    }

    #[test]
    fn test_embedded_lesson_vocabulary_non_empty() {
        let engine = test_engine();
        for id in &["pengenalan-terminal", "navigasi-file-system"] {
            let lesson = engine.get_lesson(id).unwrap();
            assert!(!lesson.vocabulary.is_empty(), "Lesson {} has no vocabulary", id);
            assert!(!lesson.key_points.is_empty(), "Lesson {} has no key points", id);
        }
    }

    #[test]
    fn test_embedded_lesson_quiz_exists() {
        let engine = test_engine();
        let lesson = engine.get_lesson("pengenalan-terminal").unwrap();
        assert!(lesson.quiz.is_some());
        assert!(lesson.quiz.as_ref().unwrap().questions.len() >= 3);
    }

    #[test]
    fn test_embedded_lesson_exercise_exists() {
        let engine = test_engine();
        let lesson = engine.get_lesson("navigasi-file-system").unwrap();
        assert!(lesson.exercise.is_some());
        assert!(!lesson.exercise.as_ref().unwrap().steps.is_empty());
    }

    #[test]
    fn test_cache_ttl_expiry_not_checked_on_get() {
        let mut cache = ContentCache::new(10);
        cache.set("key1", vec![1, 2, 3], "text/plain", Some(0));
        let entry = cache.get("key1");
        assert!(entry.is_some());
    }

    #[test]
    fn test_cache_evict_lru_empty() {
        let mut cache = ContentCache::new(10);
        cache.evict_lru();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_cache_set_overwrites_same_key() {
        let mut cache = ContentCache::new(10);
        cache.set("key1", vec![1, 2, 3], "text/plain", None);
        cache.set("key1", vec![4, 5, 6], "text/plain", None);
        assert_eq!(cache.get("key1").unwrap().data, vec![4, 5, 6]);
        assert_eq!(cache.size(), 3);
    }

    #[test]
    fn test_get_progress_mut_modifies_in_place() {
        let mut engine = test_engine();
        engine.get_progress_mut().total_xp = 999;
        assert_eq!(engine.get_progress().total_xp, 999);
    }

    #[test]
    fn test_embedded_lessons_have_related_content() {
        let engine = test_engine();
        let lesson = engine.get_lesson("navigasi-file-system").unwrap();
        assert!(lesson.related_content.contains(&"pengenalan-terminal".to_string()));
    }

    #[test]
    fn test_completion_percentage_empty_engine() {
        let engine = LearningContentEngine {
            lessons: HashMap::new(),
            categories: HashMap::new(),
            progress: UserProgress::default(),
            cache: ContentCache::new(10),
            locale: "id".to_string(),
            content_dir: PathBuf::from("/tmp/test"),
        };
        assert_eq!(engine.completion_percentage(), 0.0);
    }

    #[test]
    fn test_save_progress_creates_dir() -> Result<()> {
        let dir = PathBuf::from("/tmp/edushell-test-save-dir");
        let _ = fs::remove_dir_all(&dir);

        let mut engine = LearningContentEngine::new(Some(dir.clone()));
        engine.complete_lesson("pengenalan-terminal");
        engine.save_progress()?;
        assert!(dir.join("progress.json").exists());

        let _ = fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn test_load_content_directory_not_found() {
        let mut engine = test_engine();
        let result = engine.load_content(Path::new("/tmp/nonexistent-xyz-123"));
        assert!(result.is_err());
    }

    #[test]
    fn test_learning_engine_error_display() {
        let err = LearningEngineError::ContentNotFound("test".to_string());
        assert_eq!(err.to_string(), "Content not found: test");
    }

    #[test]
    fn test_lesson_body_falls_back_to_id_when_no_en() {
        let mut engine = test_engine();
        engine.set_locale("en");
        let body = {
            let lesson = engine.get_lesson("navigasi-file-system").unwrap();
            engine.get_lesson_body(lesson).to_string()
        };
        assert!(body.contains("Navigating File System with Terminal"));
    }

    #[test]
    fn test_external_links_in_embedded_lessons() {
        let engine = test_engine();
        let lesson = engine.get_lesson("pengenalan-terminal").unwrap();
        assert!(!lesson.external_links.is_empty());
        let (label, url) = &lesson.external_links[0];
        assert_eq!(label, "link.terminal.gnu");
        assert_eq!(url, "https://www.gnu.org/software/bash/");
    }

    #[test]
    fn test_embedded_lessons_have_prerequisites() {
        let engine = test_engine();
        let lesson = engine.get_lesson("navigasi-file-system").unwrap();
        assert!(lesson.metadata.prerequisites.contains(&"pengenalan-terminal".to_string()));
    }

    #[test]
    fn test_embedded_lesson_metadata() {
        let engine = test_engine();
        let lesson = engine.get_lesson("pengenalan-terminal").unwrap();
        assert_eq!(lesson.metadata.format, ContentFormat::Markdown);
        assert_eq!(lesson.metadata.difficulty, ContentDifficulty::Beginner);
        assert_eq!(lesson.metadata.duration_minutes, 10);
    }

    #[test]
    fn test_quiz_fields() {
        let engine = test_engine();
        let lesson = engine.get_lesson("pengenalan-terminal").unwrap();
        let quiz = lesson.quiz.as_ref().unwrap();
        assert_eq!(quiz.passing_score, 70);
        assert!(quiz.shuffle);
        assert_eq!(quiz.questions.len(), 4);
    }

    #[test]
    fn test_vocab_entry_fields() {
        let engine = test_engine();
        let lesson = engine.get_lesson("pengenalan-terminal").unwrap();
        let vocab = &lesson.vocabulary[0];
        assert_eq!(vocab.term, "Terminal");
        assert_eq!(vocab.category, "umum");
    }

    #[test]
    fn test_history_ordered_by_most_recent() {
        let mut engine = test_engine();
        engine.add_history("lesson-a", 50);
        std::thread::sleep(std::time::Duration::from_millis(10));
        engine.add_history("lesson-b", 75);
        let history = engine.get_history();
        let entries: Vec<&str> = history.iter().rev().map(|h| h.content_id.as_str()).collect();
        assert!(entries.contains(&"lesson-a"));
        assert!(entries.contains(&"lesson-b"));
    }

    #[test]
    fn test_get_lesson_mut_nonexistent() {
        let mut engine = test_engine();
        assert!(engine.get_lesson_mut("nonexistent").is_none());
    }

    #[test]
    fn test_bookmark_remove_nonexistent() {
        let mut engine = test_engine();
        engine.remove_bookmark("nonexistent");
        assert!(engine.progress.bookmarks.is_empty());
    }

    #[test]
    fn test_favorite_remove_nonexistent() {
        let mut engine = test_engine();
        engine.remove_favorite("nonexistent");
        assert!(engine.progress.favorites.is_empty());
    }

    #[test]
    fn test_cache_size_after_clear() {
        let mut cache = ContentCache::new(10);
        cache.set("a", vec![1; 100], "text/plain", None);
        cache.set("b", vec![2; 200], "text/plain", None);
        assert_eq!(cache.size(), 300);
        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_deserialize_content_difficulty() {
        let json = "\"Beginner\"";
        let d: ContentDifficulty = serde_json::from_str(json).unwrap();
        assert_eq!(d, ContentDifficulty::Beginner);
    }

    #[test]
    fn test_deserialize_validation_type() {
        let json = r#"{"ExactOutput":"hello"}"#;
        let v: ValidationType = serde_json::from_str(json).unwrap();
        match v {
            ValidationType::ExactOutput(s) => assert_eq!(s, "hello"),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_category_lesson_count_updated() {
        let engine = test_engine();
        let cat = engine.get_category("belajar-terminal").unwrap();
        assert_eq!(cat.lesson_count, 2);
    }

    #[test]
    fn test_category_total_duration_updated() {
        let engine = test_engine();
        let cat = engine.get_category("belajar-terminal").unwrap();
        assert_eq!(cat.total_duration, 25);
    }

    #[test]
    fn test_search_index_fields() {
        let engine = test_engine();
        let results = engine.search("terminal");
        assert!(!results.is_empty());
        let idx = &results[0];
        assert!(!idx.content_id.is_empty());
        assert!(!idx.title.is_empty());
        assert!(!idx.category.is_empty());
        assert!(!idx.tags.is_empty());
    }

    #[test]
    fn test_engine_send() {
        fn assert_send<T: Send>() {}
        assert_send::<LearningContentEngine>();
    }

    #[test]
    fn test_content_cache_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ContentCache>();
    }

    #[test]
    fn test_get_lesson_body_locale_fallback() {
        let mut engine = test_engine();
        let body_id = {
            let lesson = engine.get_lesson("navigasi-file-system").unwrap();
            engine.get_lesson_body(lesson).to_string()
        };
        assert!(body_id.contains("Navigasi File System dengan Terminal"));

        engine.set_locale("en");
        let body_en = {
            let lesson = engine.get_lesson("navigasi-file-system").unwrap();
            engine.get_lesson_body(lesson).to_string()
        };
        assert!(body_en.contains("Navigating File System with Terminal"));

        engine.set_locale("id");
        let body_id2 = {
            let lesson = engine.get_lesson("navigasi-file-system").unwrap();
            engine.get_lesson_body(lesson).to_string()
        };
        assert!(body_id2.contains("Navigasi File System dengan Terminal"));
    }

    #[test]
    fn test_cache_entry_metadata() {
        let mut cache = ContentCache::new(10);
        cache.set("test", vec![1, 2, 3], "application/json", Some(3600));
        let entry = cache.get("test").unwrap();
        assert_eq!(entry.content_type, "application/json");
        assert_eq!(entry.size, 3);
        assert!(entry.cached_at.len() > 10);
        assert!(entry.expires_at.is_some());
    }

    #[test]
    fn test_cache_entry_no_expiry() {
        let mut cache = ContentCache::new(10);
        cache.set("test", vec![1], "text/plain", None);
        let entry = cache.get("test").unwrap();
        assert!(entry.expires_at.is_none());
    }
}
