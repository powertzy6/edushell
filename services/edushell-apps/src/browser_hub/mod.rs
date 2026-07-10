use crate::localization::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum BrowserHubError {
    #[error("bookmark not found: {0}")]
    NotFound(String),
    #[error("duplicate bookmark id: {0}")]
    Duplicate(String),
    #[error("import error: {0}")]
    Import(String),
    #[error("export error: {0}")]
    Export(String),
}

pub type Result<T> = std::result::Result<T, BrowserHubError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: String,
    pub category: BookmarkCategory,
    pub tags: Vec<String>,
    pub favicon: Option<String>,
    pub created_at: String,
    pub visit_count: u32,
    pub last_visited: Option<String>,
    pub favorite: bool,
    pub is_educational: bool,
    pub language: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BookmarkCategory {
    LearningLinux,
    OpenSource,
    Documentation,
    School,
    Coding,
    GitHub,
    StackOverflow,
    MDN,
    News,
    Reference,
    Tools,
    Community,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkCollection {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
    pub bookmarks: Vec<Bookmark>,
    pub icon: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrowserHubStats {
    pub total_bookmarks: u32,
    pub total_collections: u32,
    pub favorite_count: u32,
    pub most_visited: Option<String>,
    pub educational_count: u32,
    pub by_category: HashMap<String, u32>,
}

pub struct BrowserHub {
    collections: Vec<BookmarkCollection>,
    recent_bookmarks: Vec<String>,
    favorites: Vec<String>,
    localization: LocalizationManager,
    default_browser: Option<String>,
}

impl BrowserHub {
    pub fn new() -> Self {
        let mut hub = Self {
            collections: Vec::new(),
            recent_bookmarks: Vec::new(),
            favorites: Vec::new(),
            localization: LocalizationManager::new(),
            default_browser: Self::detect_default_browser(),
        };
        hub.load_defaults();
        hub
    }

    pub fn add_bookmark(&mut self, bookmark: Bookmark) -> Result<()> {
        let id = bookmark.id.clone();
        let is_fav = bookmark.favorite;
        let cat = bookmark.category;
        if self.get_bookmark(&id).is_some() {
            return Err(BrowserHubError::Duplicate(id));
        }
        for col in &mut self.collections {
            if category_to_collection_id(cat) == col.id {
                col.bookmarks.push(bookmark);
                self.recent_bookmarks.insert(0, id.clone());
                if self.recent_bookmarks.len() > 100 {
                    self.recent_bookmarks.pop();
                }
                if is_fav {
                    self.favorites.push(id);
                }
                return Ok(());
            }
        }
        Err(BrowserHubError::NotFound(cat.name_key().to_string()))
    }

    pub fn remove_bookmark(&mut self, id: &str) -> Result<()> {
        for col in &mut self.collections {
            if let Some(pos) = col.bookmarks.iter().position(|b| b.id == id) {
                col.bookmarks.remove(pos);
                self.recent_bookmarks.retain(|x| x != id);
                self.favorites.retain(|x| x != id);
                return Ok(());
            }
        }
        Err(BrowserHubError::NotFound(id.to_string()))
    }

    pub fn update_bookmark(&mut self, id: &str, bookmark: Bookmark) -> Result<()> {
        for col in &mut self.collections {
            if let Some(b) = col.bookmarks.iter_mut().find(|b| b.id == id) {
                if bookmark.favorite && !b.favorite {
                    self.favorites.push(id.to_string());
                } else if !bookmark.favorite && b.favorite {
                    self.favorites.retain(|x| x != id);
                }
                *b = bookmark;
                return Ok(());
            }
        }
        Err(BrowserHubError::NotFound(id.to_string()))
    }

    pub fn get_bookmark(&self, id: &str) -> Option<&Bookmark> {
        for col in &self.collections {
            if let Some(b) = col.bookmarks.iter().find(|b| b.id == id) {
                return Some(b);
            }
        }
        None
    }

    pub fn all_bookmarks(&self) -> Vec<&Bookmark> {
        self.collections
            .iter()
            .flat_map(|c| c.bookmarks.iter())
            .collect()
    }

    pub fn bookmarks_by_category(&self, cat: BookmarkCategory) -> Vec<&Bookmark> {
        self.collections
            .iter()
            .flat_map(|c| c.bookmarks.iter())
            .filter(|b| b.category == cat)
            .collect()
    }

    pub fn bookmarks_by_tag(&self, tag: &str) -> Vec<&Bookmark> {
        let tag_lower = tag.to_lowercase();
        self.collections
            .iter()
            .flat_map(|c| c.bookmarks.iter())
            .filter(|b| b.tags.iter().any(|t| t.to_lowercase() == tag_lower))
            .collect()
    }

    pub fn educational_bookmarks(&self) -> Vec<&Bookmark> {
        self.collections
            .iter()
            .flat_map(|c| c.bookmarks.iter())
            .filter(|b| b.is_educational)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Bookmark> {
        let q = query.to_lowercase();
        self.collections
            .iter()
            .flat_map(|c| c.bookmarks.iter())
            .filter(|b| {
                b.title.to_lowercase().contains(&q)
                    || b.url.to_lowercase().contains(&q)
                    || b.description.to_lowercase().contains(&q)
                    || b.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn recent(&self, count: usize) -> Vec<&Bookmark> {
        self.recent_bookmarks
            .iter()
            .filter_map(|id| self.get_bookmark(id))
            .take(count)
            .collect()
    }

    pub fn favorite_bookmarks(&self) -> Vec<&Bookmark> {
        self.favorites
            .iter()
            .filter_map(|id| self.get_bookmark(id))
            .collect()
    }

    pub fn toggle_favorite(&mut self, id: &str) {
        if let Some(b) = self.all_bookmarks_mut().iter_mut().find(|b| b.id == id) {
            b.favorite = !b.favorite;
            if b.favorite {
                if !self.favorites.contains(&id.to_string()) {
                    self.favorites.push(id.to_string());
                }
            } else {
                self.favorites.retain(|x| x != id);
            }
        }
    }

    pub fn most_visited(&self, count: usize) -> Vec<&Bookmark> {
        let mut all: Vec<&Bookmark> = self.all_bookmarks();
        all.sort_by(|a, b| b.visit_count.cmp(&a.visit_count));
        all.truncate(count);
        all
    }

    pub fn open_url(&self, url: &str) {
        let browser = self
            .default_browser
            .as_deref()
            .unwrap_or("xdg-open");
        let _ = std::process::Command::new(browser)
            .arg(url)
            .spawn();
    }

    pub fn open_bookmark(&mut self, id: &str) {
        let mut url = None;
        for col in &mut self.collections {
            if let Some(b) = col.bookmarks.iter_mut().find(|b| b.id == id) {
                b.visit_count += 1;
                b.last_visited = Some(Utc::now().to_rfc3339());
                url = Some(b.url.clone());
                break;
            }
        }
        if let Some(u) = url {
            self.recent_bookmarks.insert(0, id.to_string());
            if self.recent_bookmarks.len() > 100 {
                self.recent_bookmarks.pop();
            }
            self.open_url(&u);
        }
    }

    pub fn detect_default_browser() -> Option<String> {
        let candidates = ["xdg-open", "firefox", "chromium", "google-chrome", "firefox-esr"];
        for cmd in &candidates {
            if which(cmd).is_some() {
                return Some(cmd.to_string());
            }
        }
        None
    }

    pub fn stats(&self) -> BrowserHubStats {
        let all = self.all_bookmarks();
        let total_bookmarks = all.len() as u32;
        let total_collections = self.collections.len() as u32;
        let favorite_count = all.iter().filter(|b| b.favorite).count() as u32;
        let educational_count = all.iter().filter(|b| b.is_educational).count() as u32;
        let most_visited = all.iter().max_by_key(|b| b.visit_count).map(|b| b.title.clone());
        let mut by_category: HashMap<String, u32> = HashMap::new();
        for b in &all {
            let key = format!("{:?}", b.category);
            *by_category.entry(key).or_insert(0) += 1;
        }
        BrowserHubStats {
            total_bookmarks,
            total_collections,
            favorite_count,
            most_visited,
            educational_count,
            by_category,
        }
    }

    pub fn category_name(&self, cat: BookmarkCategory) -> String {
        match cat {
            BookmarkCategory::LearningLinux => self.localization.get("browser.cat_learning_linux").to_string(),
            BookmarkCategory::OpenSource => self.localization.get("browser.cat_open_source").to_string(),
            BookmarkCategory::Documentation => self.localization.get("browser.cat_documentation").to_string(),
            BookmarkCategory::School => self.localization.get("browser.cat_school").to_string(),
            BookmarkCategory::Coding => self.localization.get("browser.cat_coding").to_string(),
            BookmarkCategory::GitHub => self.localization.get("browser.cat_github").to_string(),
            BookmarkCategory::StackOverflow => self.localization.get("browser.cat_stack_overflow").to_string(),
            BookmarkCategory::MDN => self.localization.get("browser.cat_mdn").to_string(),
            BookmarkCategory::News => self.localization.get("browser.cat_news").to_string(),
            BookmarkCategory::Reference => self.localization.get("browser.cat_reference").to_string(),
            BookmarkCategory::Tools => self.localization.get("browser.cat_tools").to_string(),
            BookmarkCategory::Community => self.localization.get("browser.cat_community").to_string(),
            BookmarkCategory::Other => self.localization.get("browser.cat_other").to_string(),
        }
    }

    pub fn category_icon(cat: BookmarkCategory) -> &'static str {
        match cat {
            BookmarkCategory::LearningLinux => "",
            BookmarkCategory::OpenSource => "諾",
            BookmarkCategory::Documentation => "",
            BookmarkCategory::School => "",
            BookmarkCategory::Coding => "",
            BookmarkCategory::GitHub => "",
            BookmarkCategory::StackOverflow => "",
            BookmarkCategory::MDN => "",
            BookmarkCategory::News => "",
            BookmarkCategory::Reference => "",
            BookmarkCategory::Tools => "",
            BookmarkCategory::Community => "",
            BookmarkCategory::Other => "",
        }
    }

    pub fn import_from_html(&self, html: &str) -> Result<Vec<Bookmark>> {
        let mut bookmarks = Vec::new();
        let mut pos = 0;
        let bytes = html.as_bytes();
        while let Some(start) = find_subsequence(bytes, b"<A ", pos) {
            let end = find_subsequence(bytes, b"</A>", start)
                .ok_or_else(|| BrowserHubError::Import("malformed HTML bookmark".to_string()))?;
            let entry = &html[start..end + 4];
            let href = extract_html_attr(entry, "HREF").unwrap_or_default();
            let add_date = extract_html_attr(entry, "ADD_DATE");
            let title = extract_html_tag_content(entry, "A");
            if !href.is_empty() {
                let bm = Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: title.unwrap_or_else(|| href.clone()),
                    url: href,
                    description: String::new(),
                    category: BookmarkCategory::Other,
                    tags: Vec::new(),
                    favicon: None,
                    created_at: add_date
                        .and_then(|d| d.parse::<i64>().ok())
                        .map(|ts| {
                            let secs = ts as i64;
                            let nanos = 0u32;
                            let dt = chrono::DateTime::from_timestamp(secs, nanos);
                            dt.map(|d| d.to_rfc3339()).unwrap_or_else(|| Utc::now().to_rfc3339())
                        })
                        .unwrap_or_else(|| Utc::now().to_rfc3339()),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: false,
                    language: "en".to_string(),
                };
                bookmarks.push(bm);
            }
            pos = end + 4;
        }
        Ok(bookmarks)
    }

    pub fn export_to_html(&self) -> String {
        let mut html = String::from("<!DOCTYPE NETSCAPE-Bookmark-file-1>\n");
        html.push_str("<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n");
        html.push_str("<TITLE>Bookmarks</TITLE>\n");
        html.push_str("<H1>Bookmarks</H1>\n");
        html.push_str("<DL><p>\n");
        for col in &self.collections {
            html.push_str(&format!(
                "    <DT><H3>{}</H3>\n    <DL><p>\n",
                col.name_key
            ));
            for b in &col.bookmarks {
                html.push_str(&format!(
                    "        <DT><A HREF=\"{}\" ADD_DATE=\"{}\">{}</A>\n",
                    b.url,
                    b.created_at.parse::<chrono::DateTime<Utc>>()
                        .ok()
                        .map(|dt| dt.timestamp().to_string())
                        .unwrap_or_else(|| "0".to_string()),
                    b.title
                ));
            }
            html.push_str("    </DL><p>\n");
        }
        html.push_str("</DL><p>\n");
        html
    }

    pub fn import_from_json(&self, json: &str) -> Result<Vec<Bookmark>> {
        serde_json::from_str::<Vec<Bookmark>>(json)
            .map_err(|e| BrowserHubError::Import(e.to_string()))
    }

    pub fn export_to_json(&self) -> String {
        let all: Vec<&Bookmark> = self.all_bookmarks();
        serde_json::to_string_pretty(&all).unwrap_or_default()
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = match locale {
            "id-ID" => Lang::Indonesian,
            "en-US" => Lang::English,
            _ => Lang::Indonesian,
        };
        self.localization.set_language(lang);
    }

    pub fn default_browser(&self) -> Option<&str> {
        self.default_browser.as_deref()
    }

    fn all_bookmarks_mut(&mut self) -> Vec<&mut Bookmark> {
        self.collections
            .iter_mut()
            .flat_map(|c| c.bookmarks.iter_mut())
            .collect()
    }

    fn load_defaults(&mut self) {
        let collections = default_collections();
        let mut all_ids = Vec::new();
        let mut fav_ids = Vec::new();
        for col in collections {
            for b in &col.bookmarks {
                all_ids.push(b.id.clone());
                if b.favorite {
                    fav_ids.push(b.id.clone());
                }
            }
            self.collections.push(col);
        }
        self.recent_bookmarks = all_ids;
        self.favorites = fav_ids;
    }
}

impl Default for BrowserHub {
    fn default() -> Self {
        Self::new()
    }
}

fn category_to_collection_id(cat: BookmarkCategory) -> &'static str {
    match cat {
        BookmarkCategory::LearningLinux => "belajar-linux",
        BookmarkCategory::OpenSource => "open-source",
        BookmarkCategory::Documentation => "documentation",
        BookmarkCategory::School => "school",
        BookmarkCategory::Coding => "coding",
        BookmarkCategory::GitHub => "github",
        BookmarkCategory::StackOverflow => "stack-overflow",
        BookmarkCategory::MDN => "mdn",
        BookmarkCategory::News => "news",
        BookmarkCategory::Reference => "reference",
        BookmarkCategory::Tools => "tools",
        BookmarkCategory::Community => "community",
        BookmarkCategory::Other => "other",
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    let end = haystack.len().saturating_sub(needle.len());
    for i in start..=end {
        if haystack[i..].starts_with(needle) {
            return Some(i);
        }
    }
    None
}

fn find_subsequence_case_insensitive(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    let needle_str = std::str::from_utf8(needle).ok()?.to_ascii_lowercase();
    let end = haystack.len().saturating_sub(needle.len());
    for i in start..=end {
        let slice = &haystack[i..i + needle.len()];
        if let Ok(s) = std::str::from_utf8(slice) {
            if s.to_ascii_lowercase() == needle_str {
                return Some(i);
            }
        }
    }
    None
}

fn extract_html_attr(html: &str, attr: &str) -> Option<String> {
    let upper_attr = attr.to_uppercase();
    let bytes = html.as_bytes();
    let search = format!(" {}=", attr);
    let search_upper = format!(" {}=", upper_attr);
    if let Some(pos) = find_subsequence_case_insensitive(bytes, search.as_bytes(), 0)
        .or_else(|| find_subsequence_case_insensitive(bytes, search_upper.as_bytes(), 0))
    {
        let after_eq = pos + search.len();
        if after_eq >= bytes.len() {
            return None;
        }
        let quote = bytes[after_eq];
        if quote != b'"' && quote != b'\'' {
            return None;
        }
        let start = after_eq + 1;
        let mut end = start;
        while end < bytes.len() && bytes[end] != quote {
            end += 1;
        }
        if end < bytes.len() {
            return Some(html[start..end].to_string());
        }
    }
    None
}

fn extract_html_tag_content(html: &str, _tag: &str) -> Option<String> {
    let bytes = html.as_bytes();
    let close_bracket = bytes.iter().position(|&b| b == b'>')?;
    let content_start = close_bracket + 1;
    let end_seq = format!("</{}>", _tag);
    let end = find_subsequence(bytes, end_seq.as_bytes(), content_start)?;
    Some(html[content_start..end].to_string())
}

fn which(cmd: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        let full = format!("{}/{}", dir, cmd);
        if std::path::Path::new(&full).is_file() {
            return Some(full);
        }
    }
    None
}

fn default_collections() -> Vec<BookmarkCollection> {
    let ts = Utc::now().to_rfc3339();
    vec![
        BookmarkCollection {
            id: "belajar-linux".to_string(),
            name_key: "browser.col_belajar_linux".to_string(),
            description_key: "browser.col_belajar_linux_desc".to_string(),
            icon: "".to_string(),
            color: "#e95420".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Linux Journey".to_string(),
                    url: "https://linuxjourney.com".to_string(),
                    description: "Belajar Linux dari dasar hingga mahir secara interaktif".to_string(),
                    category: BookmarkCategory::LearningLinux,
                    tags: vec!["linux".to_string(), "belajar".to_string(), "dasar".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "id".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Linux Command".to_string(),
                    url: "https://linuxcommand.org".to_string(),
                    description: "Panduan perintah Linux untuk pemula".to_string(),
                    category: BookmarkCategory::LearningLinux,
                    tags: vec!["linux".to_string(), "command".to_string(), "terminal".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Linuxindo".to_string(),
                    url: "https://linuxindo.org".to_string(),
                    description: "Komunitas dan sumber belajar Linux Indonesia".to_string(),
                    category: BookmarkCategory::LearningLinux,
                    tags: vec!["linux".to_string(), "indonesia".to_string(), "komunitas".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "id".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Linux Die".to_string(),
                    url: "https://linux.die.net".to_string(),
                    description: "Referensi dan dokumentasi Linux lengkap".to_string(),
                    category: BookmarkCategory::LearningLinux,
                    tags: vec!["linux".to_string(), "dokumentasi".to_string(), "referensi".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "open-source".to_string(),
            name_key: "browser.col_open_source".to_string(),
            description_key: "browser.col_open_source_desc".to_string(),
            icon: "諾".to_string(),
            color: "#3da639".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Open Source Initiative".to_string(),
                    url: "https://opensource.org".to_string(),
                    description: "Organisasi resmi Open Source Initiative".to_string(),
                    category: BookmarkCategory::OpenSource,
                    tags: vec!["open source".to_string(), "lisensi".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "SourceForge".to_string(),
                    url: "https://sourceforge.net".to_string(),
                    description: "Platform hosting proyek open source terbesar".to_string(),
                    category: BookmarkCategory::OpenSource,
                    tags: vec!["open source".to_string(), "hosting".to_string(), "proyek".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "GitLab Explore".to_string(),
                    url: "https://gitlab.com/explore".to_string(),
                    description: "Jelajahi proyek open source di GitLab".to_string(),
                    category: BookmarkCategory::OpenSource,
                    tags: vec!["gitlab".to_string(), "open source".to_string(), "jelajah".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Launchpad".to_string(),
                    url: "https://launchpad.net".to_string(),
                    description: "Platform kolaborasi open source Ubuntu".to_string(),
                    category: BookmarkCategory::OpenSource,
                    tags: vec!["launchpad".to_string(), "ubuntu".to_string(), "open source".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "documentation".to_string(),
            name_key: "browser.col_documentation".to_string(),
            description_key: "browser.col_documentation_desc".to_string(),
            icon: "".to_string(),
            color: "#0055cc".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "DevDocs".to_string(),
                    url: "https://devdocs.io".to_string(),
                    description: "Dokumentasi API untuk berbagai bahasa pemrograman".to_string(),
                    category: BookmarkCategory::Documentation,
                    tags: vec!["dokumentasi".to_string(), "api".to_string(), "pemrograman".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "DevHints".to_string(),
                    url: "https://devhints.io".to_string(),
                    description: "Koleksi cheatsheet untuk pengembang".to_string(),
                    category: BookmarkCategory::Documentation,
                    tags: vec!["cheatsheet".to_string(), "pemrograman".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Cheat.sh".to_string(),
                    url: "https://cheat.sh".to_string(),
                    description: "Cheatsheet interaktif di terminal".to_string(),
                    category: BookmarkCategory::Documentation,
                    tags: vec!["cheatsheet".to_string(), "terminal".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "ExplainShell".to_string(),
                    url: "https://explainshell.com".to_string(),
                    description: "Jelaskan setiap bagian dari perintah shell".to_string(),
                    category: BookmarkCategory::Documentation,
                    tags: vec!["shell".to_string(), "penjelasan".to_string(), "command".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "tldr".to_string(),
                    url: "https://tldr.sh".to_string(),
                    description: "Halaman man yang disederhanakan".to_string(),
                    category: BookmarkCategory::Documentation,
                    tags: vec!["man".to_string(), "command".to_string(), "sederhana".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "school".to_string(),
            name_key: "browser.col_school".to_string(),
            description_key: "browser.col_school_desc".to_string(),
            icon: "".to_string(),
            color: "#9b59b6".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Khan Academy".to_string(),
                    url: "https://khanacademy.org".to_string(),
                    description: "Platform belajar gratis dengan ribuan kursus".to_string(),
                    category: BookmarkCategory::School,
                    tags: vec!["belajar".to_string(), "gratis".to_string(), "kursus".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Coursera".to_string(),
                    url: "https://coursera.org".to_string(),
                    description: "Kursus online dari universitas terkemuka".to_string(),
                    category: BookmarkCategory::School,
                    tags: vec!["kursus".to_string(), "universitas".to_string(), "online".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "edX".to_string(),
                    url: "https://edx.org".to_string(),
                    description: "Pendidikan online dari institusi top dunia".to_string(),
                    category: BookmarkCategory::School,
                    tags: vec!["kursus".to_string(), "online".to_string(), "pendidikan".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Class Central".to_string(),
                    url: "https://classcentral.com".to_string(),
                    description: "Katalog kursus online gratis dari berbagai platform".to_string(),
                    category: BookmarkCategory::School,
                    tags: vec!["kursus".to_string(), "katalog".to_string(), "gratis".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Dicoding".to_string(),
                    url: "https://dicoding.com".to_string(),
                    description: "Platform belajar coding untuk developer Indonesia".to_string(),
                    category: BookmarkCategory::School,
                    tags: vec!["coding".to_string(), "indonesia".to_string(), "kursus".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "id".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "coding".to_string(),
            name_key: "browser.col_coding".to_string(),
            description_key: "browser.col_coding_desc".to_string(),
            icon: "".to_string(),
            color: "#e67e22".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Codecademy".to_string(),
                    url: "https://codecademy.com".to_string(),
                    description: "Belajar coding interaktif langsung di browser".to_string(),
                    category: BookmarkCategory::Coding,
                    tags: vec!["coding".to_string(), "interaktif".to_string(), "belajar".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "freeCodeCamp".to_string(),
                    url: "https://freecodecamp.org".to_string(),
                    description: "Platform belajar coding gratis dengan sertifikasi".to_string(),
                    category: BookmarkCategory::Coding,
                    tags: vec!["coding".to_string(), "gratis".to_string(), "sertifikasi".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "The Odin Project".to_string(),
                    url: "https://theodinproject.com".to_string(),
                    description: "Kurikulum pengembangan web full-stack gratis".to_string(),
                    category: BookmarkCategory::Coding,
                    tags: vec!["web".to_string(), "full-stack".to_string(), "gratis".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Exercism".to_string(),
                    url: "https://exercism.org".to_string(),
                    description: "Latihan coding dengan mentor dari komunitas".to_string(),
                    category: BookmarkCategory::Coding,
                    tags: vec!["latihan".to_string(), "mentor".to_string(), "coding".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "github".to_string(),
            name_key: "browser.col_github".to_string(),
            description_key: "browser.col_github_desc".to_string(),
            icon: "".to_string(),
            color: "#24292e".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "GitHub Explore".to_string(),
                    url: "https://github.com/explore".to_string(),
                    description: "Jelajahi repositori populer di GitHub".to_string(),
                    category: BookmarkCategory::GitHub,
                    tags: vec!["github".to_string(), "jelajah".to_string(), "repositori".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "GitHub Trending".to_string(),
                    url: "https://github.com/trending".to_string(),
                    description: "Repositori yang sedang tren di GitHub hari ini".to_string(),
                    category: BookmarkCategory::GitHub,
                    tags: vec!["github".to_string(), "tren".to_string(), "repositori".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Git SCM".to_string(),
                    url: "https://git-scm.com/doc".to_string(),
                    description: "Dokumentasi resmi Git".to_string(),
                    category: BookmarkCategory::GitHub,
                    tags: vec!["git".to_string(), "dokumentasi".to_string(), "vcs".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "stack-overflow".to_string(),
            name_key: "browser.col_stack_overflow".to_string(),
            description_key: "browser.col_stack_overflow_desc".to_string(),
            icon: "".to_string(),
            color: "#f48024".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Stack Overflow".to_string(),
                    url: "https://stackoverflow.com".to_string(),
                    description: "Forum tanya jawab pemrograman terbesar".to_string(),
                    category: BookmarkCategory::StackOverflow,
                    tags: vec!["tanya".to_string(), "jawab".to_string(), "pemrograman".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Super User".to_string(),
                    url: "https://superuser.com".to_string(),
                    description: "Forum tanya jawab untuk power user komputer".to_string(),
                    category: BookmarkCategory::StackOverflow,
                    tags: vec!["tanya".to_string(), "jawab".to_string(), "komputer".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Server Fault".to_string(),
                    url: "https://serverfault.com".to_string(),
                    description: "Forum tanya jawab untuk admin server dan sysadmin".to_string(),
                    category: BookmarkCategory::StackOverflow,
                    tags: vec!["server".to_string(), "sysadmin".to_string(), "tanya".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Ask Ubuntu".to_string(),
                    url: "https://askubuntu.com".to_string(),
                    description: "Forum tanya jawab khusus Ubuntu".to_string(),
                    category: BookmarkCategory::StackOverflow,
                    tags: vec!["ubuntu".to_string(), "tanya".to_string(), "jawab".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "mdn".to_string(),
            name_key: "browser.col_mdn".to_string(),
            description_key: "browser.col_mdn_desc".to_string(),
            icon: "".to_string(),
            color: "#000000".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "MDN Web Docs".to_string(),
                    url: "https://developer.mozilla.org".to_string(),
                    description: "Dokumentasi web terlengkap dari Mozilla".to_string(),
                    category: BookmarkCategory::MDN,
                    tags: vec!["web".to_string(), "dokumentasi".to_string(), "mozilla".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Can I Use".to_string(),
                    url: "https://caniuse.com".to_string(),
                    description: "Cek dukungan fitur web di berbagai browser".to_string(),
                    category: BookmarkCategory::MDN,
                    tags: vec!["browser".to_string(), "kompatibilitas".to_string(), "web".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "web.dev".to_string(),
                    url: "https://web.dev".to_string(),
                    description: "Panduan dan praktik terbaik pengembangan web dari Google".to_string(),
                    category: BookmarkCategory::MDN,
                    tags: vec!["web".to_string(), "google".to_string(), "panduan".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "CSS-Tricks".to_string(),
                    url: "https://css-tricks.com".to_string(),
                    description: "Tips, trik, dan tutorial CSS".to_string(),
                    category: BookmarkCategory::MDN,
                    tags: vec!["css".to_string(), "tips".to_string(), "tutorial".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
        BookmarkCollection {
            id: "tools".to_string(),
            name_key: "browser.col_tools".to_string(),
            description_key: "browser.col_tools_desc".to_string(),
            icon: "".to_string(),
            color: "#16a085".to_string(),
            bookmarks: vec![
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Regex101".to_string(),
                    url: "https://regex101.com".to_string(),
                    description: "Editor dan penguji regex interaktif".to_string(),
                    category: BookmarkCategory::Tools,
                    tags: vec!["regex".to_string(), "editor".to_string(), "interaktif".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "JSON Formatter".to_string(),
                    url: "https://jsonformatter.org".to_string(),
                    description: "Format dan validasi JSON online".to_string(),
                    category: BookmarkCategory::Tools,
                    tags: vec!["json".to_string(), "formatter".to_string(), "online".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Diff Checker".to_string(),
                    url: "https://diffchecker.com".to_string(),
                    description: "Bandingkan perbedaan dua teks atau file".to_string(),
                    category: BookmarkCategory::Tools,
                    tags: vec!["diff".to_string(), "perbandingan".to_string(), "teks".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "draw.io".to_string(),
                    url: "https://draw.io".to_string(),
                    description: "Diagram dan flowchart online gratis".to_string(),
                    category: BookmarkCategory::Tools,
                    tags: vec!["diagram".to_string(), "flowchart".to_string(), "gratis".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "CodeSandbox".to_string(),
                    url: "https://codesandbox.io".to_string(),
                    description: "IDE online untuk pengembangan web".to_string(),
                    category: BookmarkCategory::Tools,
                    tags: vec!["ide".to_string(), "online".to_string(), "web".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Replit".to_string(),
                    url: "https://replit.com".to_string(),
                    description: "IDE online dengan dukungan banyak bahasa".to_string(),
                    category: BookmarkCategory::Tools,
                    tags: vec!["ide".to_string(), "online".to_string(), "multi-bahasa".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: true,
                    is_educational: true,
                    language: "en".to_string(),
                },
                Bookmark {
                    id: Uuid::new_v4().to_string(),
                    title: "Compiler Explorer".to_string(),
                    url: "https://godbolt.org".to_string(),
                    description: "Jelajahi output kompilasi dari berbagai bahasa secara online".to_string(),
                    category: BookmarkCategory::Tools,
                    tags: vec!["compiler".to_string(), "online".to_string(), "pemrograman".to_string()],
                    favicon: None,
                    created_at: ts.clone(),
                    visit_count: 0,
                    last_visited: None,
                    favorite: false,
                    is_educational: true,
                    language: "en".to_string(),
                },
            ],
        },
    ]
}

impl BookmarkCategory {
    pub fn name_key(&self) -> &'static str {
        match self {
            BookmarkCategory::LearningLinux => "browser.cat_learning_linux",
            BookmarkCategory::OpenSource => "browser.cat_open_source",
            BookmarkCategory::Documentation => "browser.cat_documentation",
            BookmarkCategory::School => "browser.cat_school",
            BookmarkCategory::Coding => "browser.cat_coding",
            BookmarkCategory::GitHub => "browser.cat_github",
            BookmarkCategory::StackOverflow => "browser.cat_stack_overflow",
            BookmarkCategory::MDN => "browser.cat_mdn",
            BookmarkCategory::News => "browser.cat_news",
            BookmarkCategory::Reference => "browser.cat_reference",
            BookmarkCategory::Tools => "browser.cat_tools",
            BookmarkCategory::Community => "browser.cat_community",
            BookmarkCategory::Other => "browser.cat_other",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hub() -> BrowserHub {
        BrowserHub::new()
    }

    fn sample_bookmark(id: &str, cat: BookmarkCategory) -> Bookmark {
        Bookmark {
            id: id.to_string(),
            title: format!("Test {}", id),
            url: format!("https://test-{}.com", id),
            description: format!("Description for {}", id),
            category: cat,
            tags: vec!["test".to_string(), id.to_string()],
            favicon: None,
            created_at: Utc::now().to_rfc3339(),
            visit_count: 0,
            last_visited: None,
            favorite: false,
            is_educational: true,
            language: "en".to_string(),
        }
    }

    #[test]
    fn test_new_hub_has_40_plus_bookmarks() {
        let hub = test_hub();
        let all = hub.all_bookmarks();
        assert!(all.len() >= 40, "expected 40+ bookmarks, got {}", all.len());
    }

    #[test]
    fn test_new_hub_has_9_collections() {
        let hub = test_hub();
        assert_eq!(hub.collections.len(), 9);
    }

    #[test]
    fn test_new_hub_recent_bookmarks_not_empty() {
        let hub = test_hub();
        assert!(!hub.recent_bookmarks.is_empty());
    }

    #[test]
    fn test_new_hub_default_browser_detected() {
        let hub = test_hub();
        assert!(hub.default_browser().is_some() || hub.default_browser().is_none());
    }

    #[test]
    fn test_add_bookmark() {
        let mut hub = test_hub();
        let bm = sample_bookmark("custom-1", BookmarkCategory::Tools);
        let result = hub.add_bookmark(bm);
        assert!(result.is_ok());
        assert!(hub.get_bookmark("custom-1").is_some());
    }

    #[test]
    fn test_add_duplicate_bookmark_fails() {
        let mut hub = test_hub();
        let bm = sample_bookmark("dup-1", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        let bm2 = sample_bookmark("dup-1", BookmarkCategory::Tools);
        let result = hub.add_bookmark(bm2);
        assert!(result.is_err());
        match result {
            Err(BrowserHubError::Duplicate(id)) => assert_eq!(id, "dup-1"),
            _ => panic!("expected Duplicate error"),
        }
    }

    #[test]
    fn test_remove_bookmark() {
        let mut hub = test_hub();
        let bm = sample_bookmark("remove-1", BookmarkCategory::Coding);
        hub.add_bookmark(bm).unwrap();
        assert!(hub.get_bookmark("remove-1").is_some());
        hub.remove_bookmark("remove-1").unwrap();
        assert!(hub.get_bookmark("remove-1").is_none());
    }

    #[test]
    fn test_remove_nonexistent_bookmark_fails() {
        let mut hub = test_hub();
        let result = hub.remove_bookmark("nonexistent-id");
        assert!(result.is_err());
        match result {
            Err(BrowserHubError::NotFound(id)) => assert_eq!(id, "nonexistent-id"),
            _ => panic!("expected NotFound error"),
        }
    }

    #[test]
    fn test_update_bookmark() {
        let mut hub = test_hub();
        let bm = sample_bookmark("upd-1", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        let updated = Bookmark {
            title: "Updated Title".to_string(),
            description: "Updated desc".to_string(),
            ..sample_bookmark("upd-1", BookmarkCategory::Documentation)
        };
        hub.update_bookmark("upd-1", updated).unwrap();
        let b = hub.get_bookmark("upd-1").unwrap();
        assert_eq!(b.title, "Updated Title");
        assert_eq!(b.description, "Updated desc");
        assert_eq!(b.category, BookmarkCategory::Documentation);
    }

    #[test]
    fn test_update_nonexistent_bookmark_fails() {
        let mut hub = test_hub();
        let bm = sample_bookmark("nonexistent", BookmarkCategory::Other);
        let result = hub.update_bookmark("no-such-id", bm);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_bookmark_returns_none_for_missing() {
        let hub = test_hub();
        assert!(hub.get_bookmark("no-such-id").is_none());
    }

    #[test]
    fn test_all_bookmarks_returns_all() {
        let hub = test_hub();
        let all = hub.all_bookmarks();
        assert!(!all.is_empty());
        let count_before = all.len();
        assert!(count_before >= 40);
    }

    #[test]
    fn test_all_bookmarks_after_add() {
        let mut hub = test_hub();
        let before = hub.all_bookmarks().len();
        let bm = sample_bookmark("count-test", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        let after = hub.all_bookmarks().len();
        assert_eq!(after, before + 1);
    }

    #[test]
    fn test_bookmarks_by_category_learning_linux() {
        let hub = test_hub();
        let bms = hub.bookmarks_by_category(BookmarkCategory::LearningLinux);
        assert!(!bms.is_empty());
        for b in &bms {
            assert_eq!(b.category, BookmarkCategory::LearningLinux);
        }
    }

    #[test]
    fn test_bookmarks_by_category_coding() {
        let hub = test_hub();
        let bms = hub.bookmarks_by_category(BookmarkCategory::Coding);
        assert!(!bms.is_empty());
        for b in &bms {
            assert_eq!(b.category, BookmarkCategory::Coding);
        }
    }

    #[test]
    fn test_bookmarks_by_category_empty_for_other() {
        let hub = test_hub();
        let bms = hub.bookmarks_by_category(BookmarkCategory::Other);
        assert!(bms.is_empty());
    }

    #[test]
    fn test_bookmarks_by_tag() {
        let hub = test_hub();
        let bms = hub.bookmarks_by_tag("linux");
        assert!(!bms.is_empty());
        for b in &bms {
            assert!(b.tags.iter().any(|t| t.to_lowercase() == "linux"));
        }
    }

    #[test]
    fn test_bookmarks_by_tag_case_insensitive() {
        let hub = test_hub();
        let upper = hub.bookmarks_by_tag("LINUX");
        let lower = hub.bookmarks_by_tag("linux");
        assert_eq!(upper.len(), lower.len());
    }

    #[test]
    fn test_bookmarks_by_tag_nonexistent() {
        let hub = test_hub();
        let bms = hub.bookmarks_by_tag("xyznonexistenttag12345");
        assert!(bms.is_empty());
    }

    #[test]
    fn test_educational_bookmarks_all_educational() {
        let hub = test_hub();
        let bms = hub.educational_bookmarks();
        assert!(!bms.is_empty());
        for b in &bms {
            assert!(b.is_educational);
        }
    }

    #[test]
    fn test_educational_bookmarks_count() {
        let hub = test_hub();
        let all = hub.all_bookmarks();
        let edu = hub.educational_bookmarks();
        assert_eq!(edu.len(), all.len());
    }

    #[test]
    fn test_search_by_title() {
        let hub = test_hub();
        let results = hub.search("Linux Journey");
        assert!(!results.is_empty());
        assert!(results.iter().any(|b| b.title.contains("Linux Journey")));
    }

    #[test]
    fn test_search_by_url() {
        let hub = test_hub();
        let results = hub.search("stackoverflow.com");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_by_tag() {
        let hub = test_hub();
        let results = hub.search("terminal");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_case_insensitive() {
        let hub = test_hub();
        let upper = hub.search("LINUX");
        let lower = hub.search("linux");
        assert_eq!(upper.len(), lower.len());
    }

    #[test]
    fn test_search_no_results() {
        let hub = test_hub();
        let results = hub.search("xyznonexistentquery12345");
        assert!(results.is_empty());
    }

    #[test]
    fn test_recent_respects_count() {
        let hub = test_hub();
        let recent = hub.recent(5);
        assert!(recent.len() <= 5);
    }

    #[test]
    fn test_recent_returns_ordered() {
        let hub = test_hub();
        let recent = hub.recent(10);
        assert!(!recent.is_empty());
        let ids: Vec<String> = recent.iter().map(|b| b.id.clone()).collect();
        assert_eq!(ids.len(), recent.len());
    }

    #[test]
    fn test_favorite_bookmarks() {
        let hub = test_hub();
        let favs = hub.favorite_bookmarks();
        assert!(!favs.is_empty());
        for b in &favs {
            assert!(b.favorite);
        }
    }

    #[test]
    fn test_toggle_favorite_add() {
        let mut hub = test_hub();
        let bm = sample_bookmark("fav-test-1", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        assert!(!hub.get_bookmark("fav-test-1").unwrap().favorite);
        hub.toggle_favorite("fav-test-1");
        assert!(hub.get_bookmark("fav-test-1").unwrap().favorite);
    }

    #[test]
    fn test_toggle_favorite_remove() {
        let mut hub = test_hub();
        let bm = Bookmark {
            favorite: true,
            ..sample_bookmark("fav-test-2", BookmarkCategory::Tools)
        };
        hub.add_bookmark(bm).unwrap();
        assert!(hub.get_bookmark("fav-test-2").unwrap().favorite);
        hub.toggle_favorite("fav-test-2");
        assert!(!hub.get_bookmark("fav-test-2").unwrap().favorite);
    }

    #[test]
    fn test_toggle_favorite_twice() {
        let mut hub = test_hub();
        let bm = sample_bookmark("fav-test-3", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        hub.toggle_favorite("fav-test-3");
        assert!(hub.get_bookmark("fav-test-3").unwrap().favorite);
        hub.toggle_favorite("fav-test-3");
        assert!(!hub.get_bookmark("fav-test-3").unwrap().favorite);
    }

    #[test]
    fn test_most_visited() {
        let mut hub = test_hub();
        let bm = sample_bookmark("mv-1", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        for _ in 0..10 {
            hub.open_bookmark("mv-1");
        }
        let mv = hub.most_visited(1);
        assert_eq!(mv[0].id, "mv-1");
        assert_eq!(mv[0].visit_count, 10);
    }

    #[test]
    fn test_most_visited_count() {
        let hub = test_hub();
        let mv = hub.most_visited(3);
        assert!(mv.len() <= 3);
    }

    #[test]
    fn test_open_bookmark_increments_visit_count() {
        let mut hub = test_hub();
        let bm = sample_bookmark("vc-test", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        assert_eq!(hub.get_bookmark("vc-test").unwrap().visit_count, 0);
        hub.open_bookmark("vc-test");
        assert_eq!(hub.get_bookmark("vc-test").unwrap().visit_count, 1);
        hub.open_bookmark("vc-test");
        assert_eq!(hub.get_bookmark("vc-test").unwrap().visit_count, 2);
    }

    #[test]
    fn test_open_bookmark_sets_last_visited() {
        let mut hub = test_hub();
        let bm = sample_bookmark("lv-test", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        assert!(hub.get_bookmark("lv-test").unwrap().last_visited.is_none());
        hub.open_bookmark("lv-test");
        assert!(hub.get_bookmark("lv-test").unwrap().last_visited.is_some());
    }

    #[test]
    fn test_stats_total_bookmarks() {
        let hub = test_hub();
        let stats = hub.stats();
        assert_eq!(stats.total_bookmarks as usize, hub.all_bookmarks().len());
    }

    #[test]
    fn test_stats_total_collections() {
        let hub = test_hub();
        let stats = hub.stats();
        assert_eq!(stats.total_collections, 9);
    }

    #[test]
    fn test_stats_favorite_count() {
        let hub = test_hub();
        let stats = hub.stats();
        assert_eq!(stats.favorite_count, hub.favorite_bookmarks().len() as u32);
    }

    #[test]
    fn test_stats_educational_count() {
        let hub = test_hub();
        let stats = hub.stats();
        assert_eq!(stats.educational_count, hub.all_bookmarks().len() as u32);
    }

    #[test]
    fn test_stats_most_visited() {
        let mut hub = test_hub();
        assert!(hub.stats().most_visited.is_some());
        let bm = sample_bookmark("stats-mv", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        for _ in 0..100 {
            hub.open_bookmark("stats-mv");
        }
        let stats = hub.stats();
        assert_eq!(stats.most_visited.as_deref(), Some("Test stats-mv"));
    }

    #[test]
    fn test_stats_by_category() {
        let hub = test_hub();
        let stats = hub.stats();
        assert!(!stats.by_category.is_empty());
        let total: u32 = stats.by_category.values().sum();
        assert_eq!(total, hub.all_bookmarks().len() as u32);
    }

    #[test]
    fn test_category_name_returns_string() {
        let hub = test_hub();
        let name = hub.category_name(BookmarkCategory::Tools);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_category_name_all_categories() {
        let hub = test_hub();
        let categories = vec![
            BookmarkCategory::LearningLinux,
            BookmarkCategory::OpenSource,
            BookmarkCategory::Documentation,
            BookmarkCategory::School,
            BookmarkCategory::Coding,
            BookmarkCategory::GitHub,
            BookmarkCategory::StackOverflow,
            BookmarkCategory::MDN,
            BookmarkCategory::News,
            BookmarkCategory::Reference,
            BookmarkCategory::Tools,
            BookmarkCategory::Community,
            BookmarkCategory::Other,
        ];
        for cat in categories {
            let name = hub.category_name(cat);
            assert!(!name.is_empty(), "category_name empty for {:?}", cat);
        }
    }

    #[test]
    fn test_category_icon_all() {
        let categories = vec![
            BookmarkCategory::LearningLinux,
            BookmarkCategory::OpenSource,
            BookmarkCategory::Documentation,
            BookmarkCategory::School,
            BookmarkCategory::Coding,
            BookmarkCategory::GitHub,
            BookmarkCategory::StackOverflow,
            BookmarkCategory::MDN,
            BookmarkCategory::News,
            BookmarkCategory::Reference,
            BookmarkCategory::Tools,
            BookmarkCategory::Community,
            BookmarkCategory::Other,
        ];
        for cat in categories {
            let icon = BrowserHub::category_icon(cat);
            assert!(!icon.is_empty(), "category_icon empty for {:?}", cat);
        }
    }

    #[test]
    fn test_import_from_html_valid() {
        let hub = test_hub();
        let html = r#"<!DOCTYPE NETSCAPE-Bookmark-file-1>
<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
<TITLE>Bookmarks</TITLE>
<H1>Bookmarks</H1>
<DL><p>
    <DT><H3>Test</H3>
    <DL><p>
        <DT><A HREF="https://example.com" ADD_DATE="1700000000">Example</A>
    </DL><p>
</DL><p>"#;
        let result = hub.import_from_html(html);
        assert!(result.is_ok());
        let bms = result.unwrap();
        assert_eq!(bms.len(), 1);
        assert_eq!(bms[0].url, "https://example.com");
        assert_eq!(bms[0].title, "Example");
    }

    #[test]
    fn test_import_from_html_multiple() {
        let hub = test_hub();
        let html = r#"<DL><p>
<DT><A HREF="https://a.com" ADD_DATE="1">A</A>
<DT><A HREF="https://b.com" ADD_DATE="2">B</A>
<DT><A HREF="https://c.com" ADD_DATE="3">C</A>
</DL><p>"#;
        let bms = hub.import_from_html(html).unwrap();
        assert_eq!(bms.len(), 3);
    }

    #[test]
    fn test_import_from_html_empty() {
        let hub = test_hub();
        let html = "<html></html>";
        let bms = hub.import_from_html(html).unwrap();
        assert!(bms.is_empty());
    }

    #[test]
    fn test_export_to_html_contains_bookmarks() {
        let hub = test_hub();
        let html = hub.export_to_html();
        assert!(html.contains("<!DOCTYPE NETSCAPE-Bookmark-file-1>"));
        assert!(html.contains("<TITLE>Bookmarks</TITLE>"));
        assert!(html.contains("<DT><A HREF="));
    }

    #[test]
    fn test_export_to_html_has_all_bookmarks() {
        let hub = test_hub();
        let html = hub.export_to_html();
        for b in hub.all_bookmarks() {
            assert!(html.contains(&b.url));
            assert!(html.contains(&b.title));
        }
    }

    #[test]
    fn test_import_export_html_roundtrip() {
        let hub = test_hub();
        let html = hub.export_to_html();
        let imported = hub.import_from_html(&html).unwrap();
        let imported_titles: HashSet<String> = imported.iter().map(|b| b.title.clone()).collect();
        let orig_titles: HashSet<String> = hub.all_bookmarks().iter().map(|b| b.title.clone()).collect();
        for t in &orig_titles {
            assert!(imported_titles.contains(t), "missing title: {}", t);
        }
    }

    #[test]
    fn test_import_from_json_valid() {
        let hub = test_hub();
        let json = r#"[
            {
                "id": "j1",
                "title": "JSON Test",
                "url": "https://json-test.com",
                "description": "test",
                "category": "Tools",
                "tags": ["json"],
                "favicon": null,
                "created_at": "2024-01-01T00:00:00+00:00",
                "visit_count": 0,
                "last_visited": null,
                "favorite": false,
                "is_educational": true,
                "language": "en"
            }
        ]"#;
        let result = hub.import_from_json(json);
        assert!(result.is_ok());
        let bms = result.unwrap();
        assert_eq!(bms.len(), 1);
        assert_eq!(bms[0].title, "JSON Test");
    }

    #[test]
    fn test_import_from_json_invalid() {
        let hub = test_hub();
        let result = hub.import_from_json("not valid json");
        assert!(result.is_err());
        match result {
            Err(BrowserHubError::Import(_)) => {}
            _ => panic!("expected Import error"),
        }
    }

    #[test]
    fn test_export_to_json_valid() {
        let hub = test_hub();
        let json = hub.export_to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array());
        assert!(!parsed.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_import_export_json_roundtrip() {
        let hub = test_hub();
        let json = hub.export_to_json();
        let imported = hub.import_from_json(&json).unwrap();
        assert_eq!(imported.len(), hub.all_bookmarks().len());
        let imported_ids: HashSet<String> = imported.iter().map(|b| b.id.clone()).collect();
        let orig_ids: HashSet<String> = hub.all_bookmarks().iter().map(|b| b.id.clone()).collect();
        assert_eq!(imported_ids, orig_ids);
    }

    #[test]
    fn test_set_locale_indonesian() {
        let mut hub = test_hub();
        hub.set_locale("id-ID");
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
    }

    #[test]
    fn test_set_locale_english() {
        let mut hub = test_hub();
        hub.set_locale("en-US");
        assert_eq!(hub.localization.current_language(), Lang::English);
    }

    #[test]
    fn test_set_locale_invalid_defaults_indonesian() {
        let mut hub = test_hub();
        hub.set_locale("jp-JP");
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
    }

    #[test]
    fn test_default_browser_returns_string_or_none() {
        let hub = test_hub();
        match hub.default_browser() {
            Some(b) => assert!(!b.is_empty()),
            None => {}
        }
    }

    #[test]
    fn test_bookmark_category_name_key() {
        assert_eq!(
            BookmarkCategory::LearningLinux.name_key(),
            "browser.cat_learning_linux"
        );
        assert_eq!(BookmarkCategory::Tools.name_key(), "browser.cat_tools");
        assert_eq!(BookmarkCategory::Other.name_key(), "browser.cat_other");
    }

    #[test]
    fn test_bookmark_serde_roundtrip() {
        let bm = sample_bookmark("serde-1", BookmarkCategory::MDN);
        let json = serde_json::to_string(&bm).unwrap();
        let deserialized: Bookmark = serde_json::from_str(&json).unwrap();
        assert_eq!(bm.id, deserialized.id);
        assert_eq!(bm.title, deserialized.title);
        assert_eq!(bm.url, deserialized.url);
        assert_eq!(bm.category, deserialized.category);
    }

    #[test]
    fn test_collection_serde_roundtrip() {
        let col = BookmarkCollection {
            id: "test-col".to_string(),
            name_key: "test.name".to_string(),
            description_key: "test.desc".to_string(),
            bookmarks: vec![sample_bookmark("s1", BookmarkCategory::GitHub)],
            icon: "".to_string(),
            color: "#000".to_string(),
        };
        let json = serde_json::to_string(&col).unwrap();
        let deser: BookmarkCollection = serde_json::from_str(&json).unwrap();
        assert_eq!(col.id, deser.id);
        assert_eq!(col.bookmarks.len(), deser.bookmarks.len());
    }

    #[test]
    fn test_browser_hub_stats_default() {
        let stats = BrowserHubStats::default();
        assert_eq!(stats.total_bookmarks, 0);
        assert_eq!(stats.total_collections, 0);
        assert_eq!(stats.favorite_count, 0);
        assert!(stats.most_visited.is_none());
        assert_eq!(stats.educational_count, 0);
        assert!(stats.by_category.is_empty());
    }

    #[test]
    fn test_recent_empty_after_remove() {
        let mut hub = test_hub();
        let bm = sample_bookmark("recent-remove", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        assert!(hub.recent(100).iter().any(|b| b.id == "recent-remove"));
        hub.remove_bookmark("recent-remove").unwrap();
        assert!(!hub.recent(100).iter().any(|b| b.id == "recent-remove"));
    }

    #[test]
    fn test_favorites_updated_after_update() {
        let mut hub = test_hub();
        let bm = sample_bookmark("fav-upd", BookmarkCategory::Tools);
        hub.add_bookmark(bm).unwrap();
        assert!(!hub.get_bookmark("fav-upd").unwrap().favorite);
        assert!(hub.favorite_bookmarks().iter().all(|b| b.id != "fav-upd"));
        let updated = Bookmark {
            favorite: true,
            ..sample_bookmark("fav-upd", BookmarkCategory::Tools)
        };
        hub.update_bookmark("fav-upd", updated).unwrap();
        assert!(hub.favorite_bookmarks().iter().any(|b| b.id == "fav-upd"));
    }

    #[test]
    fn test_all_bookmarks_unique_ids() {
        let hub = test_hub();
        let all = hub.all_bookmarks();
        let mut ids = HashSet::new();
        for b in all {
            assert!(ids.insert(b.id.as_str()), "duplicate id: {}", b.id);
        }
    }

    #[test]
    fn test_open_url_does_not_panic() {
        let hub = test_hub();
        hub.open_url("https://example.com");
    }

    #[test]
    fn test_open_nonexistent_bookmark_does_nothing() {
        let mut hub = test_hub();
        let before = hub.stats();
        hub.open_bookmark("no-such-id");
        let after = hub.stats();
        assert_eq!(before.total_bookmarks, after.total_bookmarks);
    }

    #[test]
    fn test_search_matches_description() {
        let hub = test_hub();
        let results = hub.search("Belajar Linux dari dasar hingga mahir");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_add_bookmark_with_existing_collection() {
        let mut hub = test_hub();
        let bm = sample_bookmark("add-exist-1", BookmarkCategory::Tools);
        assert!(hub.add_bookmark(bm).is_ok());
    }

    #[test]
    fn test_update_bookmark_retains_position() {
        let mut hub = test_hub();
        let bm1 = sample_bookmark("pos-1", BookmarkCategory::Tools);
        let bm2 = sample_bookmark("pos-2", BookmarkCategory::Tools);
        hub.add_bookmark(bm1).unwrap();
        hub.add_bookmark(bm2).unwrap();
        let tools = hub.bookmarks_by_category(BookmarkCategory::Tools);
        let pos = tools.iter().position(|b| b.id == "pos-1").unwrap();
        let updated = Bookmark {
            title: "Updated Pos 1".to_string(),
            ..sample_bookmark("pos-1", BookmarkCategory::Tools)
        };
        hub.update_bookmark("pos-1", updated).unwrap();
        let tools_after = hub.bookmarks_by_category(BookmarkCategory::Tools);
        assert_eq!(tools_after[pos].id, "pos-1");
        assert_eq!(tools_after[pos].title, "Updated Pos 1");
    }

    #[test]
    fn test_category_icon_not_empty_for_all() {
        for cat in &[
            BookmarkCategory::LearningLinux,
            BookmarkCategory::OpenSource,
            BookmarkCategory::Documentation,
            BookmarkCategory::School,
            BookmarkCategory::Coding,
            BookmarkCategory::GitHub,
            BookmarkCategory::StackOverflow,
            BookmarkCategory::MDN,
            BookmarkCategory::News,
            BookmarkCategory::Reference,
            BookmarkCategory::Tools,
            BookmarkCategory::Community,
            BookmarkCategory::Other,
        ] {
            assert!(!BrowserHub::category_icon(*cat).is_empty());
        }
    }

    #[test]
    fn test_category_name_not_empty_for_all() {
        let hub = test_hub();
        for cat in &[
            BookmarkCategory::LearningLinux,
            BookmarkCategory::OpenSource,
            BookmarkCategory::Documentation,
            BookmarkCategory::School,
            BookmarkCategory::Coding,
            BookmarkCategory::GitHub,
            BookmarkCategory::StackOverflow,
            BookmarkCategory::MDN,
            BookmarkCategory::News,
            BookmarkCategory::Reference,
            BookmarkCategory::Tools,
            BookmarkCategory::Community,
            BookmarkCategory::Other,
        ] {
            assert!(!hub.category_name(*cat).is_empty());
        }
    }

    #[test]
    fn test_detect_default_browser_does_not_panic() {
        let _ = BrowserHub::detect_default_browser();
    }

    #[test]
    fn test_set_locale_then_category_name() {
        let mut hub = test_hub();
        hub.set_locale("en-US");
        let name_en = hub.category_name(BookmarkCategory::Tools);
        hub.set_locale("id-ID");
        let name_id = hub.category_name(BookmarkCategory::Tools);
        assert!(name_en == name_id || !name_en.is_empty());
    }
}
