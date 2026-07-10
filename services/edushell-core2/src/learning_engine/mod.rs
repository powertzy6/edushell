//! Learning Engine v2 — educational content, progress tracking, adaptive learning.
use std::collections::HashMap;

/// Lesson difficulty.
#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// Lesson metadata.
#[derive(Debug, Clone)]
pub struct LessonMeta {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub difficulty: Difficulty,
    pub duration_minutes: u32,
    pub prerequisites: Vec<String>,
}

/// User progress for a lesson.
#[derive(Debug, Clone)]
pub struct LessonProgress {
    pub lesson_id: String,
    pub completed: bool,
    pub score: f64,
    pub attempts: u32,
    pub completed_at: Option<String>,
}

/// Learning engine v2.
pub struct LearningEngineV2 {
    lessons: HashMap<String, LessonMeta>,
    progress: HashMap<String, LessonProgress>,
}

impl LearningEngineV2 {
    pub fn new() -> Self {
        Self {
            lessons: HashMap::new(),
            progress: HashMap::new(),
        }
    }

    pub fn register_lesson(&mut self, meta: LessonMeta) {
        self.lessons.insert(meta.id.clone(), meta);
    }

    pub fn get_lesson(&self, id: &str) -> Option<&LessonMeta> {
        self.lessons.get(id)
    }

    pub fn all_lessons(&self) -> Vec<&LessonMeta> {
        self.lessons.values().collect()
    }

    pub fn by_category(&self, category: &str) -> Vec<&LessonMeta> {
        self.lessons
            .values()
            .filter(|l| l.category == category)
            .collect()
    }

    pub fn by_difficulty(&self, difficulty: &Difficulty) -> Vec<&LessonMeta> {
        self.lessons
            .values()
            .filter(|l| l.difficulty == *difficulty)
            .collect()
    }

    pub fn record_progress(&mut self, lesson_id: &str, score: f64) {
        let entry = self
            .progress
            .entry(lesson_id.to_string())
            .or_insert(LessonProgress {
                lesson_id: lesson_id.to_string(),
                completed: false,
                score: 0.0,
                attempts: 0,
                completed_at: None,
            });
        entry.attempts += 1;
        entry.score = entry.score.max(score);
        if score >= 70.0 {
            entry.completed = true;
            entry.completed_at = Some(crate::core::now_iso());
        }
    }

    pub fn get_progress(&self, lesson_id: &str) -> Option<&LessonProgress> {
        self.progress.get(lesson_id)
    }

    pub fn overall_progress(&self) -> f64 {
        if self.lessons.is_empty() {
            return 0.0;
        }
        let completed = self.progress.values().filter(|p| p.completed).count();
        completed as f64 / self.lessons.len() as f64 * 100.0
    }

    pub fn completed_lessons(&self) -> Vec<&LessonProgress> {
        self.progress.values().filter(|p| p.completed).collect()
    }

    pub fn suggested_next(&self) -> Vec<&LessonMeta> {
        let completed_ids: Vec<&String> = self
            .progress
            .iter()
            .filter(|(_, p)| p.completed)
            .map(|(id, _)| id)
            .collect();
        self.lessons
            .values()
            .filter(|l| {
                !completed_ids.contains(&&l.id)
                    && l.prerequisites.iter().all(|p| completed_ids.contains(&p))
            })
            .collect()
    }
}

impl Default for LearningEngineV2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_lesson(id: &str, diff: Difficulty, prereqs: Vec<String>) -> LessonMeta {
        LessonMeta {
            id: id.into(),
            title: id.into(),
            description: "".into(),
            category: "rust".into(),
            difficulty: diff,
            duration_minutes: 10,
            prerequisites: prereqs,
        }
    }

    #[test]
    fn test_engine_new() {
        let le = LearningEngineV2::new();
        assert!(le.all_lessons().is_empty());
    }

    #[test]
    fn test_register_and_get() {
        let mut le = LearningEngineV2::new();
        le.register_lesson(sample_lesson("l1", Difficulty::Beginner, vec![]));
        assert!(le.get_lesson("l1").is_some());
    }

    #[test]
    fn test_by_category() {
        let mut le = LearningEngineV2::new();
        le.register_lesson(sample_lesson("a", Difficulty::Beginner, vec![]));
        le.register_lesson(sample_lesson("b", Difficulty::Intermediate, vec![]));
        assert_eq!(le.by_category("rust").len(), 2);
    }

    #[test]
    fn test_by_difficulty() {
        let mut le = LearningEngineV2::new();
        le.register_lesson(sample_lesson("a", Difficulty::Beginner, vec![]));
        le.register_lesson(sample_lesson("b", Difficulty::Advanced, vec![]));
        assert_eq!(le.by_difficulty(&Difficulty::Beginner).len(), 1);
    }

    #[test]
    fn test_record_progress() {
        let mut le = LearningEngineV2::new();
        le.register_lesson(sample_lesson("l1", Difficulty::Beginner, vec![]));
        le.record_progress("l1", 85.0);
        let p = le.get_progress("l1").unwrap();
        assert!(p.completed);
        assert_eq!(p.attempts, 1);
    }

    #[test]
    fn test_overall_progress() {
        let mut le = LearningEngineV2::new();
        le.register_lesson(sample_lesson("a", Difficulty::Beginner, vec![]));
        le.register_lesson(sample_lesson("b", Difficulty::Intermediate, vec![]));
        le.record_progress("a", 90.0);
        assert!((le.overall_progress() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_suggested_next() {
        let mut le = LearningEngineV2::new();
        le.register_lesson(sample_lesson("basics", Difficulty::Beginner, vec![]));
        le.register_lesson(sample_lesson(
            "advanced",
            Difficulty::Advanced,
            vec!["basics".into()],
        ));
        le.record_progress("basics", 90.0);
        let next = le.suggested_next();
        assert_eq!(next.len(), 1);
        assert_eq!(next[0].id, "advanced");
    }

    #[test]
    fn test_difficulty_variant() {
        assert_eq!(format!("{:?}", Difficulty::Intermediate), "Intermediate");
    }
}
