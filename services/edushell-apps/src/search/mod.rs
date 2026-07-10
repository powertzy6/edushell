use crate::localization::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SearchScope {
    All,
    Applications,
    Settings,
    Learning,
    Projects,
    Commands,
    Bookmarks,
    OfficeTemplates,
    BrowserBookmarks,
    FileManager,
    Welcome,
}

impl SearchScope {
    pub fn name(&self, loc: &LocalizationManager) -> String {
        match self {
            SearchScope::All => loc.get("scope.all"),
            SearchScope::Applications => loc.get("scope.applications"),
            SearchScope::Settings => loc.get("scope.settings"),
            SearchScope::Learning => loc.get("scope.learning"),
            SearchScope::Projects => loc.get("scope.projects"),
            SearchScope::Commands => loc.get("scope.commands"),
            SearchScope::Bookmarks => loc.get("scope.bookmarks"),
            SearchScope::OfficeTemplates => loc.get("scope.office_templates"),
            SearchScope::BrowserBookmarks => loc.get("scope.browser_bookmarks"),
            SearchScope::FileManager => loc.get("scope.file_manager"),
            SearchScope::Welcome => loc.get("scope.welcome"),
        }
        .to_string()
    }

    pub fn icon(&self) -> &'static str {
        match self {
            SearchScope::All => "🔍",
            SearchScope::Applications => "📱",
            SearchScope::Settings => "⚙️",
            SearchScope::Learning => "📚",
            SearchScope::Projects => "📂",
            SearchScope::Commands => "⌨️",
            SearchScope::Bookmarks => "🔖",
            SearchScope::OfficeTemplates => "📝",
            SearchScope::BrowserBookmarks => "🌐",
            SearchScope::FileManager => "📁",
            SearchScope::Welcome => "🏠",
        }
    }

    pub fn all_variants() -> Vec<SearchScope> {
        vec![
            SearchScope::All,
            SearchScope::Applications,
            SearchScope::Settings,
            SearchScope::Learning,
            SearchScope::Projects,
            SearchScope::Commands,
            SearchScope::Bookmarks,
            SearchScope::OfficeTemplates,
            SearchScope::BrowserBookmarks,
            SearchScope::FileManager,
            SearchScope::Welcome,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub icon: String,
    pub scope: SearchScope,
    pub relevance: f64,
    pub source_app: String,
    pub action: String,
    pub action_data: HashMap<String, String>,
    pub keywords: Vec<String>,
    pub match_position: Option<usize>,
    pub is_fuzzy: bool,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub raw_query: String,
    pub normalized_query: String,
    pub scope: SearchScope,
    pub filters: HashMap<String, String>,
    pub max_results: usize,
    pub min_relevance: f64,
    pub fuzzy_threshold: f64,
}

impl SearchQuery {
    pub fn new(raw_query: &str) -> Self {
        let normalized = normalize_query(raw_query);
        SearchQuery {
            raw_query: raw_query.to_string(),
            normalized_query: normalized,
            scope: SearchScope::All,
            filters: HashMap::new(),
            max_results: 20,
            min_relevance: 0.0,
            fuzzy_threshold: 0.4,
        }
    }

    pub fn with_scope(mut self, scope: SearchScope) -> Self {
        self.scope = scope;
        self
    }

    pub fn with_max_results(mut self, max: usize) -> Self {
        self.max_results = max;
        self
    }

    pub fn with_min_relevance(mut self, min: f64) -> Self {
        self.min_relevance = min;
        self
    }

    pub fn with_fuzzy_threshold(mut self, threshold: f64) -> Self {
        self.fuzzy_threshold = threshold;
        self
    }

    pub fn with_filter(mut self, key: &str, value: &str) -> Self {
        self.filters.insert(key.to_string(), value.to_string());
        self
    }
}

fn normalize_query(query: &str) -> String {
    query
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_control())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuzzyMatchType {
    Exact,
    Contains,
    Prefix,
    Suffix,
    Subsequence,
    None,
}

#[derive(Debug, Clone)]
pub struct FuzzyResult {
    pub score: f64,
    pub match_type: FuzzyMatchType,
    pub position: Option<usize>,
}

pub fn fuzzy_score(query: &str, target: &str) -> FuzzyResult {
    let q = query.to_lowercase();
    let t = target.to_lowercase();

    if q.is_empty() {
        return FuzzyResult {
            score: 1.0,
            match_type: FuzzyMatchType::Exact,
            position: Some(0),
        };
    }

    if q == t {
        return FuzzyResult {
            score: 1.0,
            match_type: FuzzyMatchType::Exact,
            position: Some(0),
        };
    }

    if t.starts_with(&q) {
        let len_penalty = 1.0 - (q.len() as f64 / t.len() as f64) * 0.3;
        return FuzzyResult {
            score: 0.7 * len_penalty,
            match_type: FuzzyMatchType::Prefix,
            position: Some(0),
        };
    }

    if t.ends_with(&q) {
        let start = t.len() - q.len();
        return FuzzyResult {
            score: 0.65,
            match_type: FuzzyMatchType::Suffix,
            position: Some(start),
        };
    }

    if t.contains(&q) {
        let pos = t.find(&q).unwrap_or(0);
        let len_penalty = 1.0 - (q.len() as f64 / t.len() as f64) * 0.2;
        return FuzzyResult {
            score: 0.8 * len_penalty,
            match_type: FuzzyMatchType::Contains,
            position: Some(pos),
        };
    }

    if let Some(score) = subsequence_score(&q, &t) {
        if score >= 0.3 {
            return FuzzyResult {
                score,
                match_type: FuzzyMatchType::Subsequence,
                position: subsequence_position(&q, &t),
            };
        }
    }

    FuzzyResult {
        score: 0.0,
        match_type: FuzzyMatchType::None,
        position: None,
    }
}

fn subsequence_score(query: &str, target: &str) -> Option<f64> {
    let q_chars: Vec<char> = query.chars().collect();
    let t_chars: Vec<char> = target.chars().collect();

    let mut q_idx = 0;
    let mut t_idx = 0;
    let mut matches = 0;
    let mut last_match_idx: Option<usize> = None;
    let mut gaps = 0;

    while q_idx < q_chars.len() && t_idx < t_chars.len() {
        if q_chars[q_idx] == t_chars[t_idx] {
            matches += 1;
            if let Some(prev) = last_match_idx {
                gaps += t_idx - prev - 1;
            }
            last_match_idx = Some(t_idx);
            q_idx += 1;
        }
        t_idx += 1;
    }

    if matches < q_chars.len() {
        return None;
    }

    let coverage = matches as f64 / t_chars.len() as f64;
    let match_ratio = matches as f64 / q_chars.len() as f64;
    let gap_penalty = (gaps as f64 / (target.len() as f64 + 1.0)).min(1.0);
    let score = (coverage * 0.4 + match_ratio * 0.4 + 0.2) * (1.0 - gap_penalty * 0.5);
    Some(score.max(0.0).min(1.0))
}

fn subsequence_position(query: &str, target: &str) -> Option<usize> {
    let q_chars: Vec<char> = query.chars().collect();
    let t_chars: Vec<char> = target.chars().collect();
    let mut q_idx = 0;
    let mut t_idx = 0;

    while q_idx < q_chars.len() && t_idx < t_chars.len() {
        if q_chars[q_idx] == t_chars[t_idx] {
            if q_idx == 0 {
                return Some(t_idx);
            }
            q_idx += 1;
        }
        t_idx += 1;
    }
    None
}

pub trait SearchProvider: Send + Sync {
    fn name(&self) -> &str;
    fn scope(&self) -> SearchScope;
    fn search(&self, query: &SearchQuery) -> Vec<SearchResult>;
    fn priority(&self) -> u32;
}

pub struct ApplicationsProvider {
    apps: Vec<SearchResult>,
}

impl ApplicationsProvider {
    pub fn new() -> Self {
        let apps = vec![
            ("calc", "Calculator", "Basic calculator application", "calculator"),
            ("text-editor", "Text Editor", "Advanced text editor with syntax highlighting", "editor"),
            ("browser", "Web Browser", "Fast and secure web browser", "browser"),
            ("email", "Email Client", "Manage your emails efficiently", "email"),
            ("calendar", "Calendar", "Schedule and manage your events", "calendar"),
            ("terminal", "Terminal", "Powerful command-line interface", "terminal"),
            ("file-manager", "File Manager", "Browse and manage files", "files"),
            ("media-player", "Media Player", "Play audio and video files", "media"),
            ("image-viewer", "Image Viewer", "View and edit images", "image"),
            ("settings", "System Settings", "Configure system preferences", "settings"),
            ("notes", "Notes App", "Quick note-taking application", "notes"),
            ("calculator-pro", "Calculator Pro", "Scientific calculator with advanced features", "calculator"),
            ("paint", "Paint", "Simple drawing application", "paint"),
            ("camera", "Camera", "Take photos and record videos", "camera"),
            ("clock", "Clock", "World clock with alarms and timers", "clock"),
        ];

        let apps: Vec<SearchResult> = apps
            .into_iter()
            .map(|(id, title, desc, icon)| SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                description: desc.to_string(),
                category: "Applications".to_string(),
                icon: icon.to_string(),
                scope: SearchScope::Applications,
                relevance: 0.0,
                source_app: "system".to_string(),
                action: "launch".to_string(),
                action_data: {
                    let mut m = HashMap::new();
                    m.insert("app_id".to_string(), id.to_string());
                    m
                },
                keywords: title
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                match_position: None,
                is_fuzzy: false,
            })
            .collect();

        ApplicationsProvider { apps }
    }
}

impl SearchProvider for ApplicationsProvider {
    fn name(&self) -> &str {
        "Applications"
    }

    fn scope(&self) -> SearchScope {
        SearchScope::Applications
    }

    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.apps
            .iter()
            .filter_map(|app| {
                let title_result = fuzzy_score(&query.normalized_query, &app.title.to_lowercase());
                let desc_result = fuzzy_score(&query.normalized_query, &app.description.to_lowercase());
                let best_score = title_result.score.max(desc_result.score);

                if best_score >= query.fuzzy_threshold {
                    let mut result = app.clone();
                    result.relevance = best_score;
                    result.is_fuzzy = best_score < 1.0 && title_result.match_type != FuzzyMatchType::Exact;
                    result.match_position = if title_result.score >= desc_result.score {
                        title_result.position
                    } else {
                        desc_result.position
                    };
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }

    fn priority(&self) -> u32 {
        100
    }
}

pub struct SettingsProvider {
    items: Vec<SearchResult>,
}

impl SettingsProvider {
    pub fn new() -> Self {
        let items = vec![
            ("wifi", "Wi-Fi Settings", "Configure wireless network connections", "wifi"),
            ("bluetooth", "Bluetooth", "Manage Bluetooth devices", "bluetooth"),
            ("display", "Display Settings", "Adjust screen resolution and brightness", "display"),
            ("sound", "Sound Settings", "Configure audio output and input", "sound"),
            ("privacy", "Privacy", "Manage privacy and security settings", "privacy"),
            ("network", "Network", "Configure network proxy and connections", "network"),
            ("keyboard", "Keyboard", "Customize keyboard shortcuts and layout", "keyboard"),
            ("mouse", "Mouse", "Adjust mouse speed and button settings", "mouse"),
            ("updates", "Software Updates", "Check and install system updates", "updates"),
            ("users", "User Accounts", "Manage user profiles and permissions", "users"),
            ("language", "Language & Region", "Change language and regional settings", "language"),
            ("backup", "Backup & Restore", "Create and restore system backups", "backup"),
            ("time", "Date & Time", "Set system date, time, and timezone", "time"),
            ("battery", "Battery", "Monitor battery usage and power settings", "battery"),
            ("notifications", "Notifications", "Manage application notifications", "notifications"),
        ];

        let items: Vec<SearchResult> = items
            .into_iter()
            .map(|(id, title, desc, icon)| SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                description: desc.to_string(),
                category: "Settings".to_string(),
                icon: icon.to_string(),
                scope: SearchScope::Settings,
                relevance: 0.0,
                source_app: "system".to_string(),
                action: "open_setting".to_string(),
                action_data: {
                    let mut m = HashMap::new();
                    m.insert("setting_id".to_string(), id.to_string());
                    m
                },
                keywords: title
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                match_position: None,
                is_fuzzy: false,
            })
            .collect();

        SettingsProvider { items }
    }
}

impl SearchProvider for SettingsProvider {
    fn name(&self) -> &str {
        "Settings"
    }

    fn scope(&self) -> SearchScope {
        SearchScope::Settings
    }

    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.items
            .iter()
            .filter_map(|item| {
                let title_result = fuzzy_score(&query.normalized_query, &item.title.to_lowercase());
                let desc_result = fuzzy_score(&query.normalized_query, &item.description.to_lowercase());
                let best_score = title_result.score.max(desc_result.score);

                if best_score >= query.fuzzy_threshold {
                    let mut result = item.clone();
                    result.relevance = best_score;
                    result.is_fuzzy = best_score < 1.0 && title_result.match_type != FuzzyMatchType::Exact;
                    result.match_position = if title_result.score >= desc_result.score {
                        title_result.position
                    } else {
                        desc_result.position
                    };
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }

    fn priority(&self) -> u32 {
        90
    }
}

pub struct LearningProvider {
    items: Vec<SearchResult>,
}

impl LearningProvider {
    pub fn new() -> Self {
        let items = vec![
            ("rust-basics", "Rust Programming Basics", "Learn the fundamentals of Rust", "rust"),
            ("python-101", "Python 101", "Introduction to Python programming", "python"),
            ("data-structures", "Data Structures", "Arrays, trees, graphs and more", "data"),
            ("algorithms", "Algorithms", "Sorting, searching, and graph algorithms", "algorithms"),
            ("web-dev", "Web Development", "HTML, CSS, and JavaScript basics", "web"),
            ("git-course", "Git Fundamentals", "Version control with Git", "git"),
            ("sql-basics", "Database SQL", "Learn SQL query language", "database"),
            ("machine-learning", "Machine Learning", "Introduction to ML concepts", "ml"),
            ("c-advanced", "Advanced C++", "Template metaprogramming and more", "cpp"),
            ("devops", "DevOps Practices", "CI/CD, Docker, and Kubernetes", "devops"),
        ];

        let items: Vec<SearchResult> = items
            .into_iter()
            .map(|(id, title, desc, icon)| SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                description: desc.to_string(),
                category: "Learning".to_string(),
                icon: icon.to_string(),
                scope: SearchScope::Learning,
                relevance: 0.0,
                source_app: "learning-center".to_string(),
                action: "open_course".to_string(),
                action_data: {
                    let mut m = HashMap::new();
                    m.insert("course_id".to_string(), id.to_string());
                    m
                },
                keywords: title
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                match_position: None,
                is_fuzzy: false,
            })
            .collect();

        LearningProvider { items }
    }
}

impl SearchProvider for LearningProvider {
    fn name(&self) -> &str {
        "Learning"
    }

    fn scope(&self) -> SearchScope {
        SearchScope::Learning
    }

    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.items
            .iter()
            .filter_map(|item| {
                let title_result = fuzzy_score(&query.normalized_query, &item.title.to_lowercase());
                let desc_result = fuzzy_score(&query.normalized_query, &item.description.to_lowercase());
                let best_score = title_result.score.max(desc_result.score);

                if best_score >= query.fuzzy_threshold {
                    let mut result = item.clone();
                    result.relevance = best_score;
                    result.is_fuzzy = best_score < 1.0 && title_result.match_type != FuzzyMatchType::Exact;
                    result.match_position = if title_result.score >= desc_result.score {
                        title_result.position
                    } else {
                        desc_result.position
                    };
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }

    fn priority(&self) -> u32 {
        80
    }
}

pub struct ProjectsProvider {
    items: Vec<SearchResult>,
}

impl ProjectsProvider {
    pub fn new() -> Self {
        let items = vec![
            ("web-app", "Web Application", "React frontend with Node.js backend", "web"),
            ("mobile-app", "Mobile App", "Cross-platform mobile application", "mobile"),
            ("api-server", "API Server", "RESTful API service", "api"),
            ("data-pipeline", "Data Pipeline", "ETL data processing pipeline", "data"),
            ("cli-tool", "CLI Tool", "Command-line utility application", "cli"),
            ("game-project", "Game Project", "2D game built with Rust and SDL", "game"),
            ("documentation", "Documentation Site", "Static site for project docs", "docs"),
            ("ml-model", "ML Model", "Machine learning model training project", "ml"),
            ("microservice", "Microservice", "Cloud-native microservice", "micro"),
            ("library", "Shared Library", "Reusable Rust library crate", "library"),
        ];

        let items: Vec<SearchResult> = items
            .into_iter()
            .map(|(id, title, desc, icon)| SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                description: desc.to_string(),
                category: "Projects".to_string(),
                icon: icon.to_string(),
                scope: SearchScope::Projects,
                relevance: 0.0,
                source_app: "project-manager".to_string(),
                action: "open_project".to_string(),
                action_data: {
                    let mut m = HashMap::new();
                    m.insert("project_id".to_string(), id.to_string());
                    m
                },
                keywords: title
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                match_position: None,
                is_fuzzy: false,
            })
            .collect();

        ProjectsProvider { items }
    }
}

impl SearchProvider for ProjectsProvider {
    fn name(&self) -> &str {
        "Projects"
    }

    fn scope(&self) -> SearchScope {
        SearchScope::Projects
    }

    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.items
            .iter()
            .filter_map(|item| {
                let title_result = fuzzy_score(&query.normalized_query, &item.title.to_lowercase());
                let desc_result = fuzzy_score(&query.normalized_query, &item.description.to_lowercase());
                let best_score = title_result.score.max(desc_result.score);

                if best_score >= query.fuzzy_threshold {
                    let mut result = item.clone();
                    result.relevance = best_score;
                    result.is_fuzzy = best_score < 1.0 && title_result.match_type != FuzzyMatchType::Exact;
                    result.match_position = if title_result.score >= desc_result.score {
                        title_result.position
                    } else {
                        desc_result.position
                    };
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }

    fn priority(&self) -> u32 {
        70
    }
}

pub struct CommandsProvider {
    items: Vec<SearchResult>,
}

impl CommandsProvider {
    pub fn new() -> Self {
        let items = vec![
            ("ls", "List files", "List directory contents", "list"),
            ("cd", "Change directory", "Change the current working directory", "directory"),
            ("cp", "Copy files", "Copy files or directories", "copy"),
            ("mv", "Move files", "Move or rename files or directories", "move"),
            ("rm", "Remove files", "Remove files or directories", "remove"),
            ("mkdir", "Make directory", "Create a new directory", "create"),
            ("find", "Find files", "Search for files in a directory tree", "search"),
            ("grep", "Search text", "Search for patterns in files", "pattern"),
            ("chmod", "Change permissions", "Change file access permissions", "permissions"),
            ("git", "Git version control", "Git version control operations", "git"),
            ("docker", "Docker containers", "Manage Docker containers", "docker"),
            ("npm", "NPM package manager", "Node package manager operations", "npm"),
            ("cargo", "Cargo Rust build", "Rust package manager and build tool", "rust"),
            ("ssh", "SSH remote access", "Secure shell remote connection", "remote"),
            ("ping", "Network test", "Test network connectivity", "network"),
        ];

        let items: Vec<SearchResult> = items
            .into_iter()
            .map(|(id, title, desc, icon)| SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                description: desc.to_string(),
                category: "Commands".to_string(),
                icon: icon.to_string(),
                scope: SearchScope::Commands,
                relevance: 0.0,
                source_app: "terminal".to_string(),
                action: "run_command".to_string(),
                action_data: {
                    let mut m = HashMap::new();
                    m.insert("command".to_string(), id.to_string());
                    m
                },
                keywords: title
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                match_position: None,
                is_fuzzy: false,
            })
            .collect();

        CommandsProvider { items }
    }
}

impl SearchProvider for CommandsProvider {
    fn name(&self) -> &str {
        "Commands"
    }

    fn scope(&self) -> SearchScope {
        SearchScope::Commands
    }

    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.items
            .iter()
            .filter_map(|item| {
                let title_result = fuzzy_score(&query.normalized_query, &item.title.to_lowercase());
                let desc_result = fuzzy_score(&query.normalized_query, &item.description.to_lowercase());
                let best_score = title_result.score.max(desc_result.score);

                if best_score >= query.fuzzy_threshold {
                    let mut result = item.clone();
                    result.relevance = best_score;
                    result.is_fuzzy = best_score < 1.0 && title_result.match_type != FuzzyMatchType::Exact;
                    result.match_position = if title_result.score >= desc_result.score {
                        title_result.position
                    } else {
                        desc_result.position
                    };
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }

    fn priority(&self) -> u32 {
        60
    }
}

pub struct BookmarksProvider {
    items: Vec<SearchResult>,
}

impl BookmarksProvider {
    pub fn new() -> Self {
        let items = vec![
            ("github", "GitHub", "Code hosting platform", "github"),
            ("stackoverflow", "Stack Overflow", "Q&A for developers", "stackoverflow"),
            ("rust-docs", "Rust Documentation", "Official Rust language docs", "rust"),
            ("mdn", "MDN Web Docs", "Web development reference", "web"),
            ("hacker-news", "Hacker News", "Tech news aggregator", "news"),
            ("reddit", "Reddit", "Social media platform", "reddit"),
            ("wikipedia", "Wikipedia", "Free online encyclopedia", "wikipedia"),
            ("youtube", "YouTube", "Video sharing platform", "video"),
            ("notion", "Notion", "All-in-one workspace", "notion"),
            ("figma", "Figma", "Design tool", "design"),
        ];

        let items: Vec<SearchResult> = items
            .into_iter()
            .map(|(id, title, desc, icon)| SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                description: desc.to_string(),
                category: "Bookmarks".to_string(),
                icon: icon.to_string(),
                scope: SearchScope::Bookmarks,
                relevance: 0.0,
                source_app: "bookmarks".to_string(),
                action: "open_bookmark".to_string(),
                action_data: {
                    let mut m = HashMap::new();
                    m.insert("bookmark_id".to_string(), id.to_string());
                    m
                },
                keywords: title
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                match_position: None,
                is_fuzzy: false,
            })
            .collect();

        BookmarksProvider { items }
    }
}

impl SearchProvider for BookmarksProvider {
    fn name(&self) -> &str {
        "Bookmarks"
    }

    fn scope(&self) -> SearchScope {
        SearchScope::Bookmarks
    }

    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.items
            .iter()
            .filter_map(|item| {
                let title_result = fuzzy_score(&query.normalized_query, &item.title.to_lowercase());
                let desc_result = fuzzy_score(&query.normalized_query, &item.description.to_lowercase());
                let best_score = title_result.score.max(desc_result.score);

                if best_score >= query.fuzzy_threshold {
                    let mut result = item.clone();
                    result.relevance = best_score;
                    result.is_fuzzy = best_score < 1.0 && title_result.match_type != FuzzyMatchType::Exact;
                    result.match_position = if title_result.score >= desc_result.score {
                        title_result.position
                    } else {
                        desc_result.position
                    };
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }

    fn priority(&self) -> u32 {
        50
    }
}

pub struct FileManagerProvider {
    items: Vec<SearchResult>,
}

impl FileManagerProvider {
    pub fn new() -> Self {
        let items = vec![
            ("home", "Home Directory", "User home folder", "folder"),
            ("documents", "Documents", "Documents folder", "documents"),
            ("downloads", "Downloads", "Downloads folder", "downloads"),
            ("desktop", "Desktop", "Desktop folder", "desktop"),
            ("pictures", "Pictures", "Pictures folder", "pictures"),
            ("music", "Music", "Music folder", "music"),
            ("videos", "Videos", "Videos folder", "videos"),
            ("projects", "Projects Folder", "Developer projects directory", "projects"),
            ("config", "Configuration Files", "Hidden config directory", "config"),
            ("temp", "Temporary Files", "System temporary directory", "temp"),
        ];

        let items: Vec<SearchResult> = items
            .into_iter()
            .map(|(id, title, desc, icon)| SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                description: desc.to_string(),
                category: "FileManager".to_string(),
                icon: icon.to_string(),
                scope: SearchScope::FileManager,
                relevance: 0.0,
                source_app: "file-manager".to_string(),
                action: "open_folder".to_string(),
                action_data: {
                    let mut m = HashMap::new();
                    m.insert("folder_id".to_string(), id.to_string());
                    m
                },
                keywords: title
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                match_position: None,
                is_fuzzy: false,
            })
            .collect();

        FileManagerProvider { items }
    }
}

impl SearchProvider for FileManagerProvider {
    fn name(&self) -> &str {
        "FileManager"
    }

    fn scope(&self) -> SearchScope {
        SearchScope::FileManager
    }

    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.items
            .iter()
            .filter_map(|item| {
                let title_result = fuzzy_score(&query.normalized_query, &item.title.to_lowercase());
                let desc_result = fuzzy_score(&query.normalized_query, &item.description.to_lowercase());
                let best_score = title_result.score.max(desc_result.score);

                if best_score >= query.fuzzy_threshold {
                    let mut result = item.clone();
                    result.relevance = best_score;
                    result.is_fuzzy = best_score < 1.0 && title_result.match_type != FuzzyMatchType::Exact;
                    result.match_position = if title_result.score >= desc_result.score {
                        title_result.position
                    } else {
                        desc_result.position
                    };
                    Some(result)
                } else {
                    None
                }
            })
            .collect()
    }

    fn priority(&self) -> u32 {
        40
    }
}

#[derive(Debug, Clone)]
pub struct SearchStats {
    pub total_results: usize,
    pub providers_searched: usize,
    pub search_time_ms: u64,
    pub scope: SearchScope,
    pub query: String,
}

pub struct GlobalSearch {
    providers: Vec<Arc<dyn SearchProvider>>,
    pub last_results: Vec<SearchResult>,
    pub search_history: Vec<SearchQuery>,
    pub favorites: Vec<String>,
    localization: LocalizationManager,
}

impl GlobalSearch {
    pub fn new() -> Self {
        let localization = LocalizationManager::new();
        GlobalSearch {
            providers: Vec::new(),
            last_results: Vec::new(),
            search_history: Vec::new(),
            favorites: Vec::new(),
            localization,
        }
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = if locale.starts_with("id") { crate::localization::Lang::Indonesian } else { crate::localization::Lang::English };
        self.localization.set_language(lang);
    }

    pub fn register_provider(&mut self, provider: Arc<dyn SearchProvider>) {
        self.providers.push(provider);
        self.providers.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    pub fn search(&mut self, query: SearchQuery) -> Vec<SearchResult> {
        self.search_history.push(query.clone());
        if self.search_history.len() > 100 {
            self.search_history.remove(0);
        }

        let mut all_results: Vec<SearchResult> = Vec::new();

        for provider in &self.providers {
            if query.scope == SearchScope::All || provider.scope() == query.scope {
                let mut results = provider.search(&query);
                all_results.append(&mut results);
            }
        }

        all_results.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        all_results.retain(|r| r.relevance >= query.min_relevance);
        all_results.truncate(query.max_results);

        self.last_results = all_results.clone();
        all_results
    }

    pub fn search_in_scope(&mut self, query: &str, scope: SearchScope) -> Vec<SearchResult> {
        let q = SearchQuery::new(query).with_scope(scope);
        self.search(q)
    }

    pub fn fuzzy_search(&mut self, query: &str) -> Vec<SearchResult> {
        let q = SearchQuery::new(query).with_fuzzy_threshold(0.3);
        self.search(q)
    }

    pub fn last_results(&self) -> &[SearchResult] {
        &self.last_results
    }

    pub fn best_result(&self) -> Option<&SearchResult> {
        self.last_results.iter().max_by(|a, b| {
            a.relevance
                .partial_cmp(&b.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn top_results(&self, count: usize) -> Vec<&SearchResult> {
        self.last_results.iter().take(count).collect()
    }

    pub fn stats(&self) -> SearchStats {
        SearchStats {
            total_results: self.last_results.len(),
            providers_searched: self.providers.len(),
            search_time_ms: 0,
            scope: SearchScope::All,
            query: String::new(),
        }
    }

    pub fn history(&self) -> &[SearchQuery] {
        &self.search_history
    }

    pub fn clear_history(&mut self) {
        self.search_history.clear();
    }

    pub fn add_favorite(&mut self, result_id: &str) -> bool {
        if !self.favorites.contains(&result_id.to_string()) {
            self.favorites.push(result_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn remove_favorite(&mut self, result_id: &str) -> bool {
        let len_before = self.favorites.len();
        self.favorites.retain(|f| f != result_id);
        self.favorites.len() < len_before
    }

    pub fn is_favorite(&self, result_id: &str) -> bool {
        self.favorites.contains(&result_id.to_string())
    }

    pub fn favorites(&self) -> &[String] {
        &self.favorites
    }

    pub fn scope_name(&self, scope: &SearchScope) -> String {
        scope.name(&self.localization)
    }

    pub fn scope_icon(&self, scope: &SearchScope) -> &'static str {
        scope.icon()
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_search() -> GlobalSearch {
        let mut gs = GlobalSearch::new();
        gs.register_provider(Arc::new(ApplicationsProvider::new()));
        gs.register_provider(Arc::new(SettingsProvider::new()));
        gs.register_provider(Arc::new(LearningProvider::new()));
        gs.register_provider(Arc::new(ProjectsProvider::new()));
        gs.register_provider(Arc::new(CommandsProvider::new()));
        gs.register_provider(Arc::new(BookmarksProvider::new()));
        gs.register_provider(Arc::new(FileManagerProvider::new()));
        gs
    }

    #[test]
    fn test_scope_name_all() {
        let gs = GlobalSearch::new();
        let name = gs.scope_name(&SearchScope::All);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_scope_icon_applications() {
        assert_eq!(SearchScope::Applications.icon(), "📱");
    }

    #[test]
    fn test_scope_icon_settings() {
        assert_eq!(SearchScope::Settings.icon(), "⚙️");
    }

    #[test]
    fn test_scope_icon_learning() {
        assert_eq!(SearchScope::Learning.icon(), "📚");
    }

    #[test]
    fn test_scope_icon_projects() {
        assert_eq!(SearchScope::Projects.icon(), "📂");
    }

    #[test]
    fn test_scope_icon_commands() {
        assert_eq!(SearchScope::Commands.icon(), "⌨️");
    }

    #[test]
    fn test_scope_icon_bookmarks() {
        assert_eq!(SearchScope::Bookmarks.icon(), "🔖");
    }

    #[test]
    fn test_scope_icon_file_manager() {
        assert_eq!(SearchScope::FileManager.icon(), "📁");
    }

    #[test]
    fn test_scope_icon_welcome() {
        assert_eq!(SearchScope::Welcome.icon(), "🏠");
    }

    #[test]
    fn test_scope_all_variants_count() {
        assert_eq!(SearchScope::all_variants().len(), 11);
    }

    #[test]
    fn test_search_query_new() {
        let q = SearchQuery::new("hello");
        assert_eq!(q.raw_query, "hello");
        assert_eq!(q.normalized_query, "hello");
        assert_eq!(q.scope, SearchScope::All);
    }

    #[test]
    fn test_search_query_normalize() {
        let q = SearchQuery::new("  Hello   World  ");
        assert_eq!(q.normalized_query, "hello world");
    }

    #[test]
    fn test_search_query_with_scope() {
        let q = SearchQuery::new("test").with_scope(SearchScope::Applications);
        assert_eq!(q.scope, SearchScope::Applications);
    }

    #[test]
    fn test_search_query_with_max_results() {
        let q = SearchQuery::new("test").with_max_results(5);
        assert_eq!(q.max_results, 5);
    }

    #[test]
    fn test_search_query_with_min_relevance() {
        let q = SearchQuery::new("test").with_min_relevance(0.5);
        assert_eq!(q.min_relevance, 0.5);
    }

    #[test]
    fn test_search_query_with_fuzzy_threshold() {
        let q = SearchQuery::new("test").with_fuzzy_threshold(0.6);
        assert_eq!(q.fuzzy_threshold, 0.6);
    }

    #[test]
    fn test_search_query_with_filter() {
        let q = SearchQuery::new("test").with_filter("category", "apps");
        assert_eq!(q.filters.get("category").unwrap(), "apps");
    }

    #[test]
    fn test_fuzzy_score_exact() {
        let result = fuzzy_score("hello", "hello");
        assert_eq!(result.score, 1.0);
        assert!(matches!(result.match_type, FuzzyMatchType::Exact));
    }

    #[test]
    fn test_fuzzy_score_exact_case_insensitive() {
        let result = fuzzy_score("Hello", "hello");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn test_fuzzy_score_contains() {
        let result = fuzzy_score("ell", "hello");
        assert!(result.score > 0.7 && result.score <= 0.8);
        assert!(matches!(result.match_type, FuzzyMatchType::Contains));
    }

    #[test]
    fn test_fuzzy_score_prefix() {
        let result = fuzzy_score("hel", "hello");
        assert!(result.score > 0.0);
        assert_eq!(result.match_type, FuzzyMatchType::Prefix);
    }

    #[test]
    fn test_fuzzy_score_suffix() {
        let result = fuzzy_score("llo", "hello");
        assert!(result.score > 0.0);
        assert_eq!(result.match_type, FuzzyMatchType::Suffix);
    }

    #[test]
    fn test_fuzzy_score_subsequence() {
        let result = fuzzy_score("hlo", "hello");
        assert!(result.score > 0.0);
        assert!(matches!(result.match_type, FuzzyMatchType::Subsequence));
    }

    #[test]
    fn test_fuzzy_score_no_match() {
        let result = fuzzy_score("xyz", "hello");
        assert_eq!(result.score, 0.0);
        assert!(matches!(result.match_type, FuzzyMatchType::None));
    }

    #[test]
    fn test_fuzzy_score_empty_query() {
        let result = fuzzy_score("", "hello");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn test_fuzzy_score_empty_target() {
        let result = fuzzy_score("hello", "");
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn test_fuzzy_score_longer_contains_shorter() {
        let result = fuzzy_score("hello world", "say hello world to everyone");
        assert!(result.score > 0.5);
    }

    #[test]
    fn test_normalize_query_whitespace() {
        assert_eq!(normalize_query("  a  b  c  "), "a b c");
    }

    #[test]
    fn test_normalize_query_special_chars() {
        let result = normalize_query("hello\x00world");
        assert_eq!(result, "helloworld");
    }

    #[test]
    fn test_register_provider() {
        let gs = make_search();
        assert_eq!(gs.provider_count(), 7);
    }

    #[test]
    fn test_search_calculator() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("calculator"));
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.title == "Calculator"));
    }

    #[test]
    fn test_search_exact_title() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("Terminal"));
        assert!(!results.is_empty());
        assert!(results[0].relevance >= 1.0);
    }

    #[test]
    fn test_search_scope_applications() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("editor").with_scope(SearchScope::Applications));
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.scope == SearchScope::Applications));
    }

    #[test]
    fn test_search_scope_settings() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("wifi").with_scope(SearchScope::Settings));
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.scope == SearchScope::Settings));
    }

    #[test]
    fn test_search_scope_learning() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("rust").with_scope(SearchScope::Learning));
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.scope == SearchScope::Learning));
    }

    #[test]
    fn test_search_scope_commands() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("git").with_scope(SearchScope::Commands));
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.scope == SearchScope::Commands));
    }

    #[test]
    fn test_search_scope_projects() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("web").with_scope(SearchScope::Projects));
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.scope == SearchScope::Projects));
    }

    #[test]
    fn test_search_scope_bookmarks() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("github").with_scope(SearchScope::Bookmarks));
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.scope == SearchScope::Bookmarks));
    }

    #[test]
    fn test_search_scope_file_manager() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("home").with_scope(SearchScope::FileManager));
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.scope == SearchScope::FileManager));
    }

    #[test]
    fn test_search_max_results() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("a").with_max_results(3));
        assert!(results.len() <= 3);
    }

    #[test]
    fn test_search_min_relevance() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("xyz").with_min_relevance(0.5));
        assert!(results.iter().all(|r| r.relevance >= 0.5));
    }

    #[test]
    fn test_search_in_scope() {
        let mut gs = make_search();
        let results = gs.search_in_scope("terminal", SearchScope::Applications);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_fuzzy_search() {
        let mut gs = make_search();
        let results = gs.fuzzy_search("calc");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_last_results() {
        let mut gs = make_search();
        gs.search(SearchQuery::new("editor"));
        assert!(!gs.last_results().is_empty());
    }

    #[test]
    fn test_best_result() {
        let mut gs = make_search();
        gs.search(SearchQuery::new("terminal"));
        let best = gs.best_result();
        assert!(best.is_some());
        assert_eq!(best.unwrap().title, "Terminal");
    }

    #[test]
    fn test_best_result_none() {
        let gs = GlobalSearch::new();
        assert!(gs.best_result().is_none());
    }

    #[test]
    fn test_top_results() {
        let mut gs = make_search();
        gs.search(SearchQuery::new("a"));
        let top = gs.top_results(5);
        assert!(top.len() <= 5);
    }

    #[test]
    fn test_search_history() {
        let mut gs = make_search();
        gs.search(SearchQuery::new("hello"));
        gs.search(SearchQuery::new("world"));
        assert_eq!(gs.history().len(), 2);
    }

    #[test]
    fn test_clear_history() {
        let mut gs = make_search();
        gs.search(SearchQuery::new("hello"));
        gs.clear_history();
        assert!(gs.history().is_empty());
    }

    #[test]
    fn test_add_favorite() {
        let mut gs = make_search();
        assert!(gs.add_favorite("calc"));
        assert!(gs.is_favorite("calc"));
    }

    #[test]
    fn test_add_favorite_duplicate() {
        let mut gs = make_search();
        gs.add_favorite("calc");
        assert!(!gs.add_favorite("calc"));
    }

    #[test]
    fn test_remove_favorite() {
        let mut gs = make_search();
        gs.add_favorite("calc");
        assert!(gs.remove_favorite("calc"));
        assert!(!gs.is_favorite("calc"));
    }

    #[test]
    fn test_remove_favorite_not_found() {
        let mut gs = make_search();
        assert!(!gs.remove_favorite("nonexistent"));
    }

    #[test]
    fn test_favorites_list() {
        let mut gs = make_search();
        gs.add_favorite("calc");
        gs.add_favorite("editor");
        assert_eq!(gs.favorites().len(), 2);
    }

    #[test]
    fn test_search_results_sorted_by_relevance() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("a"));
        for i in 1..results.len() {
            assert!(results[i - 1].relevance >= results[i].relevance);
        }
    }

    #[test]
    fn test_provider_priority_ordering() {
        let gs = make_search();
        let priorities: Vec<u32> = gs.providers.iter().map(|p| p.priority()).collect();
        for i in 1..priorities.len() {
            assert!(priorities[i - 1] >= priorities[i]);
        }
    }

    #[test]
    fn test_search_result_has_action_data() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("calculator"));
        assert!(!results.is_empty());
        assert!(!results[0].action_data.is_empty());
    }

    #[test]
    fn test_search_result_has_keywords() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("calculator"));
        assert!(!results.is_empty());
        assert!(!results[0].keywords.is_empty());
    }

    #[test]
    fn test_search_empty_query() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new(""));
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_case_insensitive() {
        let mut gs = make_search();
        let results_lower = gs.search(SearchQuery::new("terminal"));
        let results_upper = gs.search(SearchQuery::new("TERMINAL"));
        assert_eq!(results_lower.len(), results_upper.len());
    }

    #[test]
    fn test_stats() {
        let gs = make_search();
        let stats = gs.stats();
        assert_eq!(stats.providers_searched, 7);
    }

    #[test]
    fn test_scope_name_applications() {
        let gs = GlobalSearch::new();
        let name = gs.scope_name(&SearchScope::Applications);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_scope_name_commands() {
        let gs = GlobalSearch::new();
        let name = gs.scope_name(&SearchScope::Commands);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_scope_name_welcome() {
        let gs = GlobalSearch::new();
        let name = gs.scope_name(&SearchScope::Welcome);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_scope_icon_all() {
        assert_eq!(SearchScope::All.icon(), "🔍");
    }

    #[test]
    fn test_scope_icon_office_templates() {
        assert_eq!(SearchScope::OfficeTemplates.icon(), "📝");
    }

    #[test]
    fn test_scope_icon_browser_bookmarks() {
        assert_eq!(SearchScope::BrowserBookmarks.icon(), "🌐");
    }

    #[test]
    fn test_fuzzy_search_lower_threshold() {
        let mut gs = make_search();
        let results = gs.fuzzy_search("calc");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_result_id_uniqueness() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("a"));
        let ids: Vec<&str> = results.iter().map(|r| r.id.as_str()).collect();
        let unique: Vec<&str> = ids.iter().copied().collect::<std::collections::HashSet<_>>().into_iter().collect();
        assert_eq!(ids.len(), unique.len());
    }

    #[test]
    fn test_search_result_has_category() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("hello"));
        for r in &results {
            assert!(!r.category.is_empty());
        }
    }

    #[test]
    fn test_search_result_has_icon() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("terminal"));
        for r in &results {
            assert!(!r.icon.is_empty());
        }
    }

    #[test]
    fn test_subsequence_score_basic() {
        let score = subsequence_score("abc", "a1b2c3");
        assert!(score.is_some());
        assert!(score.unwrap() > 0.0);
    }

    #[test]
    fn test_subsequence_score_no_match() {
        let score = subsequence_score("xyz", "hello");
        assert!(score.is_none());
    }

    #[test]
    fn test_subsequence_position() {
        let pos = subsequence_position("abc", "a_b_c");
        assert_eq!(pos, Some(0));
    }

    #[test]
    fn test_search_result_has_source_app() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("terminal"));
        assert!(!results.is_empty());
        assert!(!results[0].source_app.is_empty());
    }

    #[test]
    fn test_search_result_has_action() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("terminal"));
        assert!(!results.is_empty());
        assert!(!results[0].action.is_empty());
    }

    #[test]
    fn test_search_result_has_scope() {
        let mut gs = make_search();
        let results = gs.search(SearchQuery::new("terminal"));
        assert!(!results.is_empty());
        assert_eq!(results[0].scope, SearchScope::Applications);
    }

    #[test]
    fn test_search_all_scopes_independently() {
        let mut gs = make_search();
        for scope in SearchScope::all_variants() {
            if scope == SearchScope::All {
                continue;
            }
            let results = gs.search(SearchQuery::new("a").with_scope(scope));
            for r in &results {
                assert_eq!(r.scope, scope);
            }
        }
    }
}
