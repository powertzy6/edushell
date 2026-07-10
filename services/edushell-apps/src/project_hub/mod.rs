use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::localization::*;
use crate::tr;

pub type Result<T> = std::result::Result<T, ProjectHubError>;

#[derive(Debug, thiserror::Error)]
pub enum ProjectHubError {
    #[error("project not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("template not found: {0}")]
    TemplateNotFound(String),
    #[error("invalid format: {0}")]
    InvalidFormat(String),
    #[error("invalid grade: {0}")]
    InvalidGrade(u8),
    #[error("export error: {0}")]
    ExportError(String),
    #[error("import error: {0}")]
    ImportError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectLanguage {
    Html,
    Css,
    JavaScript,
    Python,
    Rust,
    C,
    Cpp,
    Java,
    Scratch,
    Arduino,
    Markdown,
    Bash,
    Other(String),
}

impl ProjectLanguage {
    pub fn extension(&self) -> &'static str {
        match self {
            ProjectLanguage::Html => "html",
            ProjectLanguage::Css => "css",
            ProjectLanguage::JavaScript => "js",
            ProjectLanguage::Python => "py",
            ProjectLanguage::Rust => "rs",
            ProjectLanguage::C => "c",
            ProjectLanguage::Cpp => "cpp",
            ProjectLanguage::Java => "java",
            ProjectLanguage::Scratch => "sb3",
            ProjectLanguage::Arduino => "ino",
            ProjectLanguage::Markdown => "md",
            ProjectLanguage::Bash => "sh",
            ProjectLanguage::Other(_) => "txt",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ProjectLanguage::Html => "text-html",
            ProjectLanguage::Css => "text-css",
            ProjectLanguage::JavaScript => "application-javascript",
            ProjectLanguage::Python => "application-python",
            ProjectLanguage::Rust => "application-rust",
            ProjectLanguage::C => "text-x-c",
            ProjectLanguage::Cpp => "text-x-cpp",
            ProjectLanguage::Java => "application-java",
            ProjectLanguage::Scratch => "application-scratch",
            ProjectLanguage::Arduino => "application-arduino",
            ProjectLanguage::Markdown => "text-markdown",
            ProjectLanguage::Bash => "application-bash",
            ProjectLanguage::Other(_) => "application-octet-stream",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectStatus {
    Draft,
    InProgress,
    Completed,
    Archived,
    Submitted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub language: ProjectLanguage,
    pub status: ProjectStatus,
    pub created_at: String,
    pub updated_at: String,
    pub path: PathBuf,
    pub template_id: Option<String>,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub category: String,
    pub grade: Option<u8>,
    pub feedback: Option<String>,
    pub version: u32,
    pub file_count: u32,
    pub size_kb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
    pub language: ProjectLanguage,
    pub category: String,
    pub difficulty: String,
    pub files: Vec<TemplateFile>,
    pub estimated_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    pub path: String,
    pub content: String,
    pub is_main: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectStats {
    pub total_projects: u32,
    pub completed_projects: u32,
    pub in_progress: u32,
    pub archived: u32,
    pub drafts: u32,
    pub by_language: HashMap<String, u32>,
    pub by_category: HashMap<String, u32>,
    pub favorite_count: u32,
    pub total_files: u32,
    pub total_size_kb: u64,
}

fn now_iso() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string()
}

fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub struct ProjectHub {
    projects: Vec<Project>,
    templates: Vec<ProjectTemplate>,
    recent_projects: Vec<String>,
    favorites: Vec<String>,
    localization: LocalizationManager,
    projects_dir: PathBuf,
}

impl ProjectHub {
    pub fn new(projects_dir: Option<PathBuf>) -> Self {
        let localization = LocalizationManager::new();
        let projects_dir = projects_dir.unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".local/share/edushell/projects")
        });
        let templates = default_templates();
        let projects = Self::scan_projects(&projects_dir);
        let favorites: Vec<String> = projects.iter().filter(|p| p.favorite).map(|p| p.id.clone()).collect();
        let mut recent: Vec<&Project> = projects.iter().collect();
        recent.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        let recent_projects: Vec<String> = recent.iter().take(20).map(|p| p.id.clone()).collect();
        ProjectHub {
            projects,
            templates,
            recent_projects,
            favorites,
            localization,
            projects_dir,
        }
    }

    fn scan_projects(dir: &Path) -> Vec<Project> {
        if !dir.exists() {
            return Vec::new();
        }
        let mut projects = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let meta_path = path.join("project.json");
                    if let Ok(content) = std::fs::read_to_string(&meta_path) {
                        if let Ok(project) = serde_json::from_str::<Project>(&content) {
                            projects.push(project);
                        }
                    }
                }
            }
        }
        projects
    }

    fn save_project_meta(project: &Project, projects_dir: &Path) -> Result<()> {
        let project_dir = projects_dir.join(&project.id);
        std::fs::create_dir_all(&project_dir)?;
        let meta_path = project_dir.join("project.json");
        let content = serde_json::to_string_pretty(project)?;
        std::fs::write(meta_path, content)?;
        Ok(())
    }

    pub fn create_project(
        &mut self,
        title: &str,
        language: ProjectLanguage,
        template_id: Option<&str>,
    ) -> Result<Project> {
        let id = generate_id();
        let now = now_iso();
        let template_files = template_id.and_then(|tid| {
            self.templates.iter().find(|t| t.id == tid).map(|t| t.files.clone())
        });
        let file_count = template_files.as_ref().map(|f| f.len() as u32).unwrap_or(1);
        let category = template_id
            .and_then(|tid| self.templates.iter().find(|t| t.id == tid).map(|t| t.category.clone()))
            .unwrap_or_else(|| "personal".to_string());
        let project_dir = self.projects_dir.join(&id);
        std::fs::create_dir_all(&project_dir)?;
        if let Some(files) = &template_files {
            for tf in files {
                let file_path = project_dir.join(&tf.path);
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&file_path, &tf.content)?;
            }
        } else {
            let ext = language.extension();
            let main_file = project_dir.join(format!("main.{}", ext));
            std::fs::write(&main_file, "")?;
        }
        let size_kb = calculate_dir_size(&project_dir);
        let project = Project {
            id: id.clone(),
            title: title.to_string(),
            description: String::new(),
            language,
            status: ProjectStatus::Draft,
            created_at: now.clone(),
            updated_at: now,
            path: project_dir,
            template_id: template_id.map(String::from),
            tags: Vec::new(),
            favorite: false,
            category,
            grade: None,
            feedback: None,
            version: 1,
            file_count,
            size_kb,
        };
        Self::save_project_meta(&project, &self.projects_dir)?;
        self.recent_projects.insert(0, id.clone());
        self.projects.push(project.clone());
        Ok(project)
    }

    pub fn open_project(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }

    pub fn open_project_mut(&mut self, id: &str) -> Option<&mut Project> {
        self.projects.iter_mut().find(|p| p.id == id)
    }

    pub fn delete_project(&mut self, id: &str) -> Result<()> {
        let pos = self.projects.iter().position(|p| p.id == id).ok_or_else(|| ProjectHubError::NotFound(id.to_string()))?;
        let project = self.projects.remove(pos);
        let _ = std::fs::remove_dir_all(&project.path);
        self.recent_projects.retain(|x| x != id);
        self.favorites.retain(|x| x != id);
        Ok(())
    }

    pub fn duplicate_project(&mut self, id: &str) -> Result<Project> {
        let original = self.projects.iter().find(|p| p.id == id).ok_or_else(|| ProjectHubError::NotFound(id.to_string()))?;
        let new_id = generate_id();
        let now = now_iso();
        let new_path = self.projects_dir.join(&new_id);
        copy_dir(&original.path, &new_path)?;
        let dup = Project {
            id: new_id.clone(),
            title: format!("{} (copy)", original.title),
            description: original.description.clone(),
            language: original.language.clone(),
            status: ProjectStatus::Draft,
            created_at: now.clone(),
            updated_at: now,
            path: new_path,
            template_id: original.template_id.clone(),
            tags: original.tags.clone(),
            favorite: false,
            category: original.category.clone(),
            grade: None,
            feedback: None,
            version: 1,
            file_count: original.file_count,
            size_kb: calculate_dir_size(&self.projects_dir.join(&new_id)),
        };
        Self::save_project_meta(&dup, &self.projects_dir)?;
        self.recent_projects.insert(0, dup.id.clone());
        self.projects.push(dup.clone());
        Ok(dup)
    }

    pub fn rename_project(&mut self, id: &str, new_title: &str) -> Result<()> {
        let project = self.projects.iter_mut().find(|p| p.id == id).ok_or_else(|| ProjectHubError::NotFound(id.to_string()))?;
        project.title = new_title.to_string();
        project.updated_at = now_iso();
        Self::save_project_meta(project, &self.projects_dir)?;
        Ok(())
    }

    pub fn update_status(&mut self, id: &str, status: ProjectStatus) {
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == id) {
            project.status = status;
            project.updated_at = now_iso();
            let _ = Self::save_project_meta(project, &self.projects_dir);
        }
    }

    pub fn add_tag(&mut self, id: &str, tag: &str) {
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == id) {
            if !project.tags.contains(&tag.to_string()) {
                project.tags.push(tag.to_string());
                project.updated_at = now_iso();
                let _ = Self::save_project_meta(project, &self.projects_dir);
            }
        }
    }

    pub fn remove_tag(&mut self, id: &str, tag: &str) {
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == id) {
            project.tags.retain(|t| t != tag);
            project.updated_at = now_iso();
            let _ = Self::save_project_meta(project, &self.projects_dir);
        }
    }

    pub fn set_grade(&mut self, id: &str, grade: u8) {
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == id) {
            project.grade = Some(grade.min(100));
            project.updated_at = now_iso();
            let _ = Self::save_project_meta(project, &self.projects_dir);
        }
    }

    pub fn set_feedback(&mut self, id: &str, feedback: &str) {
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == id) {
            project.feedback = Some(feedback.to_string());
            project.updated_at = now_iso();
            let _ = Self::save_project_meta(project, &self.projects_dir);
        }
    }

    pub fn all_projects(&self) -> &[Project] {
        &self.projects
    }

    pub fn all_projects_mut(&mut self) -> &mut Vec<Project> {
        &mut self.projects
    }

    pub fn projects_by_language(&self, lang: ProjectLanguage) -> Vec<&Project> {
        self.projects.iter().filter(|p| p.language == lang).collect()
    }

    pub fn projects_by_status(&self, status: ProjectStatus) -> Vec<&Project> {
        self.projects.iter().filter(|p| p.status == status).collect()
    }

    pub fn projects_by_category(&self, cat: &str) -> Vec<&Project> {
        self.projects.iter().filter(|p| p.category == cat).collect()
    }

    pub fn projects_by_tag(&self, tag: &str) -> Vec<&Project> {
        self.projects.iter().filter(|p| p.tags.iter().any(|t| t == tag)).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Project> {
        let q = query.to_lowercase();
        self.projects
            .iter()
            .filter(|p| {
                p.title.to_lowercase().contains(&q)
                    || p.description.to_lowercase().contains(&q)
                    || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn recent(&self, count: usize) -> Vec<&Project> {
        let mut sorted: Vec<&Project> = self.projects.iter().collect();
        sorted.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        sorted.truncate(count);
        sorted
    }

    pub fn favorite_projects(&self) -> Vec<&Project> {
        self.projects.iter().filter(|p| p.favorite).collect()
    }

    pub fn toggle_favorite(&mut self, id: &str) {
        if let Some(project) = self.projects.iter_mut().find(|p| p.id == id) {
            project.favorite = !project.favorite;
            if project.favorite {
                self.favorites.push(project.id.clone());
            } else {
                self.favorites.retain(|x| x != id);
            }
            let _ = Self::save_project_meta(project, &self.projects_dir);
        }
    }

    pub fn is_favorite(&self, id: &str) -> bool {
        self.projects.iter().any(|p| p.id == id && p.favorite)
    }

    pub fn templates(&self) -> &[ProjectTemplate] {
        &self.templates
    }

    pub fn templates_for_language(&self, lang: ProjectLanguage) -> Vec<&ProjectTemplate> {
        self.templates.iter().filter(|t| t.language == lang).collect()
    }

    pub fn template(&self, id: &str) -> Option<&ProjectTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }

    pub fn get_template_files(&self, template_id: &str) -> Vec<TemplateFile> {
        self.templates
            .iter()
            .find(|t| t.id == template_id)
            .map(|t| t.files.clone())
            .unwrap_or_default()
    }

    pub fn stats(&self) -> ProjectStats {
        let total_projects = self.projects.len() as u32;
        let completed_projects = self.projects.iter().filter(|p| p.status == ProjectStatus::Completed).count() as u32;
        let in_progress = self.projects.iter().filter(|p| p.status == ProjectStatus::InProgress).count() as u32;
        let archived = self.projects.iter().filter(|p| p.status == ProjectStatus::Archived).count() as u32;
        let drafts = self.projects.iter().filter(|p| p.status == ProjectStatus::Draft).count() as u32;
        let mut by_language: HashMap<String, u32> = HashMap::new();
        let mut by_category: HashMap<String, u32> = HashMap::new();
        let mut total_files = 0u32;
        let mut total_size_kb = 0u64;
        let mut favorite_count = 0u32;
        for p in &self.projects {
            let lang_key = format!("{:?}", p.language);
            *by_language.entry(lang_key).or_insert(0) += 1;
            *by_category.entry(p.category.clone()).or_insert(0) += 1;
            total_files += p.file_count;
            total_size_kb += p.size_kb;
            if p.favorite {
                favorite_count += 1;
            }
        }
        ProjectStats {
            total_projects,
            completed_projects,
            in_progress,
            archived,
            drafts,
            by_language,
            by_category,
            favorite_count,
            total_files,
            total_size_kb,
        }
    }

    pub fn language_name(&self, lang: ProjectLanguage) -> String {
        match lang {
            ProjectLanguage::Html => tr!(self.localization, "project.lang_html").to_string(),
            ProjectLanguage::Css => tr!(self.localization, "project.lang_css").to_string(),
            ProjectLanguage::JavaScript => tr!(self.localization, "project.lang_javascript").to_string(),
            ProjectLanguage::Python => tr!(self.localization, "project.lang_python").to_string(),
            ProjectLanguage::Rust => tr!(self.localization, "project.lang_rust").to_string(),
            ProjectLanguage::C => tr!(self.localization, "project.lang_c").to_string(),
            ProjectLanguage::Cpp => tr!(self.localization, "project.lang_cpp").to_string(),
            ProjectLanguage::Java => tr!(self.localization, "project.lang_java").to_string(),
            ProjectLanguage::Scratch => tr!(self.localization, "project.lang_scratch").to_string(),
            ProjectLanguage::Arduino => tr!(self.localization, "project.lang_arduino").to_string(),
            ProjectLanguage::Markdown => tr!(self.localization, "project.lang_markdown").to_string(),
            ProjectLanguage::Bash => tr!(self.localization, "project.lang_bash").to_string(),
            ProjectLanguage::Other(s) => s,
        }
    }

    pub fn language_extension(lang: ProjectLanguage) -> &'static str {
        lang.extension()
    }

    pub fn language_icon(lang: ProjectLanguage) -> &'static str {
        lang.icon()
    }

    pub fn supported_languages() -> Vec<ProjectLanguage> {
        vec![
            ProjectLanguage::Html,
            ProjectLanguage::Css,
            ProjectLanguage::JavaScript,
            ProjectLanguage::Python,
            ProjectLanguage::Rust,
            ProjectLanguage::C,
            ProjectLanguage::Cpp,
            ProjectLanguage::Java,
            ProjectLanguage::Scratch,
            ProjectLanguage::Arduino,
            ProjectLanguage::Markdown,
            ProjectLanguage::Bash,
        ]
    }

    pub fn export_project(&self, id: &str, format: &str) -> Result<PathBuf> {
        let project = self.projects.iter().find(|p| p.id == id).ok_or_else(|| ProjectHubError::NotFound(id.to_string()))?;
        let export_dir = std::env::temp_dir().join(format!("edushell-export-{}", id));
        if export_dir.exists() {
            std::fs::remove_dir_all(&export_dir)?;
        }
        copy_dir(&project.path, &export_dir)?;
        let meta_path = export_dir.join("project.json");
        let meta_content = serde_json::to_string_pretty(project)?;
        std::fs::write(meta_path, meta_content)?;
        match format {
            "dir" => Ok(export_dir),
            "zip" => {
                let zip_path = std::env::temp_dir().join(format!("{}.zip", id));
                let output = std::process::Command::new("zip")
                    .arg("-r")
                    .arg(&zip_path)
                    .arg(".")
                    .current_dir(&export_dir)
                    .output()
                    .map_err(|e| ProjectHubError::ExportError(format!("zip command failed: {}", e)))?;
                if !output.status.success() {
                    return Err(ProjectHubError::ExportError("zip command failed".to_string()));
                }
                let _ = std::fs::remove_dir_all(&export_dir);
                Ok(zip_path)
            }
            "tar" => {
                let tar_path = std::env::temp_dir().join(format!("{}.tar.gz", id));
                let output = std::process::Command::new("tar")
                    .arg("-czf")
                    .arg(&tar_path)
                    .arg(".")
                    .current_dir(&export_dir)
                    .output()
                    .map_err(|e| ProjectHubError::ExportError(format!("tar command failed: {}", e)))?;
                if !output.status.success() {
                    return Err(ProjectHubError::ExportError("tar command failed".to_string()));
                }
                let _ = std::fs::remove_dir_all(&export_dir);
                Ok(tar_path)
            }
            other => Err(ProjectHubError::InvalidFormat(other.to_string())),
        }
    }

    pub fn import_project(&mut self, path: &Path) -> Result<Project> {
        let import_dir = if path.is_dir() {
            path.to_path_buf()
        } else {
            let extract_dir = std::env::temp_dir().join(format!("edushell-import-{}", generate_id()));
            std::fs::create_dir_all(&extract_dir)?;
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            match ext {
                "zip" => {
                    let output = std::process::Command::new("unzip")
                        .arg(&path)
                        .arg("-d")
                        .arg(&extract_dir)
                        .output()
                        .map_err(|e| ProjectHubError::ImportError(format!("unzip command failed: {}", e)))?;
                    if !output.status.success() {
                        return Err(ProjectHubError::ImportError("unzip command failed".to_string()));
                    }
                }
                "gz" | "tgz" | "tar" => {
                    let flag = if ext == "gz" || ext == "tgz" { "-xzf" } else { "-xf" };
                    let output = std::process::Command::new("tar")
                        .arg(flag)
                        .arg(&path)
                        .arg("-C")
                        .arg(&extract_dir)
                        .output()
                        .map_err(|e| ProjectHubError::ImportError(format!("tar command failed: {}", e)))?;
                    if !output.status.success() {
                        return Err(ProjectHubError::ImportError("tar command failed".to_string()));
                    }
                }
                _ => return Err(ProjectHubError::ImportError(format!("unsupported format: {}", ext))),
            }
            extract_dir
        };
        let meta_path = import_dir.join("project.json");
        if !meta_path.exists() {
            return Err(ProjectHubError::ImportError("project.json not found".to_string()));
        }
        let content = std::fs::read_to_string(&meta_path)?;
        let mut project: Project = serde_json::from_str(&content)?;
        let new_id = generate_id();
        let new_path = self.projects_dir.join(&new_id);
        copy_dir(&import_dir, &new_path)?;
        project.id = new_id;
        project.path = new_path;
        project.updated_at = now_iso();
        Self::save_project_meta(&project, &self.projects_dir)?;
        self.recent_projects.insert(0, project.id.clone());
        self.projects.push(project.clone());
        if path.is_dir() {
        } else {
            let _ = std::fs::remove_dir_all(&import_dir);
        }
        Ok(project)
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = match locale {
            "id-ID" | "id" => Lang::Indonesian,
            _ => Lang::English,
        };
        self.localization.set_language(lang);
    }
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    if let Ok(entries) = std::fs::read_dir(src) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dst.join(&file_name);
            if entry_path.is_dir() {
                copy_dir(&entry_path, &dest_path)?;
            } else {
                std::fs::copy(&entry_path, &dest_path)?;
            }
        }
    }
    Ok(())
}

fn calculate_dir_size(dir: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                total += calculate_dir_size(&path);
            } else if let Ok(meta) = std::fs::metadata(&path) {
                total += meta.len();
            }
        }
    }
    total / 1024
}

fn default_templates_inner() -> Vec<ProjectTemplate> {
    vec![
        ProjectTemplate {
            id: "html-basic".into(),
            name_key: "project.tpl_html_basic".into(),
            description_key: "project.tpl_html_basic_desc".into(),
            language: ProjectLanguage::Html,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 10,
            files: vec![
                TemplateFile {
                    path: "index.html".into(),
                    content: "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>My Page</title>\n</head>\n<body>\n  <h1>Hello, World!</h1>\n  <p>Welcome to my first HTML page.</p>\n</body>\n</html>\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "html-form".into(),
            name_key: "project.tpl_html_form".into(),
            description_key: "project.tpl_html_form_desc".into(),
            language: ProjectLanguage::Html,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 15,
            files: vec![
                TemplateFile {
                    path: "form.html".into(),
                    content: "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <title>Contact Form</title>\n</head>\n<body>\n  <h1>Contact Form</h1>\n  <form action=\"#\" method=\"post\">\n    <label for=\"name\">Name:</label>\n    <input type=\"text\" id=\"name\" name=\"name\" required><br>\n    <label for=\"email\">Email:</label>\n    <input type=\"email\" id=\"email\" name=\"email\" required><br>\n    <label for=\"message\">Message:</label><br>\n    <textarea id=\"message\" name=\"message\" rows=\"4\" cols=\"50\"></textarea><br>\n    <button type=\"submit\">Send</button>\n  </form>\n</body>\n</html>\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "html-portfolio".into(),
            name_key: "project.tpl_html_portfolio".into(),
            description_key: "project.tpl_html_portfolio_desc".into(),
            language: ProjectLanguage::Html,
            category: "personal".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 30,
            files: vec![
                TemplateFile {
                    path: "index.html".into(),
                    content: "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>My Portfolio</title>\n  <link rel=\"stylesheet\" href=\"style.css\">\n</head>\n<body>\n  <header>\n    <h1>My Portfolio</h1>\n    <nav>\n      <a href=\"#about\">About</a>\n      <a href=\"#projects\">Projects</a>\n      <a href=\"#contact\">Contact</a>\n    </nav>\n  </header>\n  <section id=\"about\"><h2>About Me</h2><p>Write about yourself here.</p></section>\n  <section id=\"projects\"><h2>Projects</h2><p>Showcase your work.</p></section>\n  <section id=\"contact\"><h2>Contact</h2><p>Email: your@email.com</p></section>\n</body>\n</html>\n".into(),
                    is_main: true,
                },
                TemplateFile {
                    path: "style.css".into(),
                    content: "body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }\nheader { background: #333; color: white; padding: 1em; }\nnav a { color: white; margin: 0 10px; }\nsection { margin: 20px 0; padding: 20px; background: white; border-radius: 8px; }\n".into(),
                    is_main: false,
                },
            ],
        },
        ProjectTemplate {
            id: "css-basics".into(),
            name_key: "project.tpl_css_basics".into(),
            description_key: "project.tpl_css_basics_desc".into(),
            language: ProjectLanguage::Css,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 10,
            files: vec![
                TemplateFile {
                    path: "style.css".into(),
                    content: "/* CSS Styling Basics */\n* { margin: 0; padding: 0; box-sizing: border-box; }\nbody { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; line-height: 1.6; color: #333; background: #fff; }\nh1 { color: #2c3e50; font-size: 2.5em; margin-bottom: 0.5em; }\np { margin-bottom: 1em; }\n.container { max-width: 1200px; margin: 0 auto; padding: 20px; }\n.btn { display: inline-block; padding: 10px 20px; background: #3498db; color: white; text-decoration: none; border-radius: 5px; }\n.btn:hover { background: #2980b9; }\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "css-flexbox".into(),
            name_key: "project.tpl_css_flexbox".into(),
            description_key: "project.tpl_css_flexbox_desc".into(),
            language: ProjectLanguage::Css,
            category: "tutorial".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 15,
            files: vec![
                TemplateFile {
                    path: "flexbox.css".into(),
                    content: "/* CSS Flexbox Layout */\n.flex-container {\n  display: flex;\n  flex-direction: row;\n  justify-content: space-between;\n  align-items: center;\n  flex-wrap: wrap;\n  gap: 20px;\n  padding: 20px;\n  background: #ecf0f1;\n}\n.flex-item {\n  flex: 1 1 200px;\n  padding: 20px;\n  background: #3498db;\n  color: white;\n  text-align: center;\n  border-radius: 8px;\n}\n.flex-item:nth-child(2) { flex: 2 1 300px; }\n@media (max-width: 768px) {\n  .flex-container { flex-direction: column; }\n}\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "css-grid".into(),
            name_key: "project.tpl_css_grid".into(),
            description_key: "project.tpl_css_grid_desc".into(),
            language: ProjectLanguage::Css,
            category: "tutorial".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 15,
            files: vec![
                TemplateFile {
                    path: "grid.css".into(),
                    content: "/* CSS Grid Layout */\n.grid-container {\n  display: grid;\n  grid-template-columns: repeat(3, 1fr);\n  grid-template-rows: auto;\n  gap: 15px;\n  padding: 20px;\n  background: #ecf0f1;\n}\n.grid-item {\n  padding: 30px;\n  background: #e74c3c;\n  color: white;\n  text-align: center;\n  border-radius: 8px;\n  font-size: 1.2em;\n}\n.grid-item:first-child {\n  grid-column: 1 / -1;\n  background: #c0392b;\n}\n@media (max-width: 768px) {\n  .grid-container { grid-template-columns: 1fr; }\n}\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "js-hello".into(),
            name_key: "project.tpl_js_hello".into(),
            description_key: "project.tpl_js_hello_desc".into(),
            language: ProjectLanguage::JavaScript,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 5,
            files: vec![
                TemplateFile {
                    path: "index.js".into(),
                    content: "// Hello, World! in JavaScript\nfunction greet(name) {\n  return `Hello, ${name}!`;\n}\n\nconst message = greet('World');\nconsole.log(message);\n\n// Try it in the browser:\n// document.getElementById('output').textContent = message;\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "js-calculator".into(),
            name_key: "project.tpl_js_calculator".into(),
            description_key: "project.tpl_js_calculator_desc".into(),
            language: ProjectLanguage::JavaScript,
            category: "school".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 20,
            files: vec![
                TemplateFile {
                    path: "calculator.js".into(),
                    content: "// Simple Calculator\nclass Calculator {\n  constructor() { this.result = 0; }\n  add(a, b) { this.result = a + b; return this.result; }\n  subtract(a, b) { this.result = a - b; return this.result; }\n  multiply(a, b) { this.result = a * b; return this.result; }\n  divide(a, b) {\n    if (b === 0) throw new Error('Division by zero');\n    this.result = a / b;\n    return this.result;\n  }\n  clear() { this.result = 0; }\n}\nmodule.exports = Calculator;\n".into(),
                    is_main: true,
                },
                TemplateFile {
                    path: "test.js".into(),
                    content: "const Calculator = require('./calculator');\nconst calc = new Calculator();\nconsole.log('2 + 3 =', calc.add(2, 3));\nconsole.log('10 - 4 =', calc.subtract(10, 4));\nconsole.log('6 * 7 =', calc.multiply(6, 7));\nconsole.log('20 / 4 =', calc.divide(20, 4));\n".into(),
                    is_main: false,
                },
            ],
        },
        ProjectTemplate {
            id: "js-todo".into(),
            name_key: "project.tpl_js_todo".into(),
            description_key: "project.tpl_js_todo_desc".into(),
            language: ProjectLanguage::JavaScript,
            category: "school".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 25,
            files: vec![
                TemplateFile {
                    path: "todo.js".into(),
                    content: "// Todo App\nclass TodoApp {\n  constructor() { this.tasks = []; }\n  add(title) { this.tasks.push({ id: Date.now(), title, done: false }); }\n  remove(id) { this.tasks = this.tasks.filter(t => t.id !== id); }\n  toggle(id) { const t = this.tasks.find(t => t.id === id); if (t) t.done = !t.done; }\n  list() { return this.tasks; }\n  pending() { return this.tasks.filter(t => !t.done); }\n  completed() { return this.tasks.filter(t => t.done); }\n  clear() { this.tasks = []; }\n}\nmodule.exports = TodoApp;\n".into(),
                    is_main: true,
                },
                TemplateFile {
                    path: "test.js".into(),
                    content: "const TodoApp = require('./todo');\nconst app = new TodoApp();\napp.add('Learn JavaScript');\napp.add('Build a project');\napp.add('Write tests');\nconsole.log('All tasks:', app.list());\napp.toggle(app.list()[0].id);\nconsole.log('Pending:', app.pending());\nconsole.log('Completed:', app.completed());\n".into(),
                    is_main: false,
                },
            ],
        },
        ProjectTemplate {
            id: "js-game".into(),
            name_key: "project.tpl_js_game".into(),
            description_key: "project.tpl_js_game_desc".into(),
            language: ProjectLanguage::JavaScript,
            category: "personal".into(),
            difficulty: "advanced".into(),
            estimated_minutes: 30,
            files: vec![
                TemplateFile {
                    path: "game.js".into(),
                    content: "// Simple Number Guessing Game\nconst readline = require('readline');\n\nclass GuessingGame {\n  constructor(max = 100) {\n    this.secret = Math.floor(Math.random() * max) + 1;\n    this.attempts = 0;\n    this.max = max;\n  }\n  guess(n) {\n    this.attempts++;\n    if (n < this.secret) return 'too low';\n    if (n > this.secret) return 'too high';\n    return `correct in ${this.attempts} attempts`;\n  }\n}\n\nmodule.exports = GuessingGame;\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "py-hello".into(),
            name_key: "project.tpl_py_hello".into(),
            description_key: "project.tpl_py_hello_desc".into(),
            language: ProjectLanguage::Python,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 5,
            files: vec![
                TemplateFile {
                    path: "main.py".into(),
                    content: "# Hello, World! in Python\ndef greet(name):\n    return f\"Hello, {name}!\"\n\ndef main():\n    message = greet(\"World\")\n    print(message)\n\nif __name__ == \"__main__\":\n    main()\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "py-data-analysis".into(),
            name_key: "project.tpl_py_data".into(),
            description_key: "project.tpl_py_data_desc".into(),
            language: ProjectLanguage::Python,
            category: "school".into(),
            difficulty: "advanced".into(),
            estimated_minutes: 30,
            files: vec![
                TemplateFile {
                    path: "analysis.py".into(),
                    content: "# Data Analysis Template\nimport csv\nimport json\nfrom collections import Counter, defaultdict\n\ndef load_csv(path):\n    with open(path, 'r') as f:\n        return list(csv.DictReader(f))\n\ndef load_json(path):\n    with open(path, 'r') as f:\n        return json.load(f)\n\ndef summary(data):\n    return {\n        'count': len(data),\n        'columns': list(data[0].keys()) if data else [],\n    }\n\ndef frequency(data, field):\n    return dict(Counter(row[field] for row in data if field in row))\n\ndef main():\n    print(\"Data Analysis Template\")\n    print(\"Load your CSV or JSON data and analyze!\")\n\nif __name__ == \"__main__\":\n    main()\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "py-scraper".into(),
            name_key: "project.tpl_py_scraper".into(),
            description_key: "project.tpl_py_scraper_desc".into(),
            language: ProjectLanguage::Python,
            category: "school".into(),
            difficulty: "advanced".into(),
            estimated_minutes: 25,
            files: vec![
                TemplateFile {
                    path: "scraper.py".into(),
                    content: "# Web Scraper Template\nimport urllib.request\nfrom html.parser import HTMLParser\n\nclass LinkParser(HTMLParser):\n    def __init__(self):\n        super().__init__()\n        self.links = []\n    def handle_starttag(self, tag, attrs):\n        if tag == 'a':\n            for name, value in attrs:\n                if name == 'href':\n                    self.links.append(value)\n\ndef fetch_page(url):\n    with urllib.request.urlopen(url) as response:\n        return response.read().decode('utf-8')\n\ndef extract_links(html):\n    parser = LinkParser()\n    parser.feed(html)\n    return parser.links\n\ndef main():\n    url = \"https://example.com\"\n    html = fetch_page(url)\n    links = extract_links(html)\n    print(f\"Found {len(links)} links on {url}\")\n    for link in links[:10]:\n        print(f\"  - {link}\")\n\nif __name__ == \"__main__\":\n    main()\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "py-cli".into(),
            name_key: "project.tpl_py_cli".into(),
            description_key: "project.tpl_py_cli_desc".into(),
            language: ProjectLanguage::Python,
            category: "personal".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 15,
            files: vec![
                TemplateFile {
                    path: "cli.py".into(),
                    content: "# CLI Tool Template\nimport sys\nimport argparse\n\ndef main():\n    parser = argparse.ArgumentParser(description=\"CLI Tool\")\n    parser.add_argument(\"input\", help=\"Input file\")\n    parser.add_argument(\"-o\", \"--output\", help=\"Output file\")\n    parser.add_argument(\"-v\", \"--verbose\", action=\"store_true\", help=\"Verbose output\")\n    args = parser.parse_args()\n    \n    if args.verbose:\n        print(f\"Input: {args.input}\")\n        print(f\"Output: {args.output}\")\n    \n    print(f\"Processing {args.input}...\")\n\nif __name__ == \"__main__\":\n    main()\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "rs-hello".into(),
            name_key: "project.tpl_rs_hello".into(),
            description_key: "project.tpl_rs_hello_desc".into(),
            language: ProjectLanguage::Rust,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 5,
            files: vec![
                TemplateFile {
                    path: "src/main.rs".into(),
                    content: "fn main() {\n    println!(\"Hello, World!\");\n}\n".into(),
                    is_main: true,
                },
                TemplateFile {
                    path: "Cargo.toml".into(),
                    content: "[package]\nname = \"hello_world\"\nversion = \"0.1.0\"\nedition = \"2021\"\n".into(),
                    is_main: false,
                },
            ],
        },
        ProjectTemplate {
            id: "rs-cli-calc".into(),
            name_key: "project.tpl_rs_cli_calc".into(),
            description_key: "project.tpl_rs_cli_calc_desc".into(),
            language: ProjectLanguage::Rust,
            category: "school".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 20,
            files: vec![
                TemplateFile {
                    path: "src/main.rs".into(),
                    content: "use std::io;\n\nfn main() {\n    println!(\"Rust CLI Calculator\");\n    loop {\n        let mut input = String::new();\n        io::stdin().read_line(&mut input).expect(\"Failed to read\");\n        let input = input.trim();\n        if input == \"quit\" || input == \"exit\" { break; }\n        let parts: Vec<&str> = input.split_whitespace().collect();\n        if parts.len() != 3 {\n            println!(\"Usage: <num> <op> <num>\");\n            continue;\n        }\n        let a: f64 = match parts[0].parse() { Ok(n) => n, Err(_) => { println!(\"Invalid number\"); continue; } };\n        let b: f64 = match parts[2].parse() { Ok(n) => n, Err(_) => { println!(\"Invalid number\"); continue; } };\n        let result = match parts[1] {\n            \"+\" => a + b,\n            \"-\" => a - b,\n            \"*\" => a * b,\n            \"/\" => if b != 0.0 { a / b } else { println!(\"Division by zero\"); continue; },\n            op => { println!(\"Unknown operator: {}\", op); continue; }\n        };\n        println!(\"= {}\", result);\n    }\n}\n".into(),
                    is_main: true,
                },
                TemplateFile {
                    path: "Cargo.toml".into(),
                    content: "[package]\nname = \"cli_calculator\"\nversion = \"0.1.0\"\nedition = \"2021\"\n".into(),
                    is_main: false,
                },
            ],
        },
        ProjectTemplate {
            id: "c-hello".into(),
            name_key: "project.tpl_c_hello".into(),
            description_key: "project.tpl_c_hello_desc".into(),
            language: ProjectLanguage::C,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 5,
            files: vec![
                TemplateFile {
                    path: "hello.c".into(),
                    content: "#include <stdio.h>\n\nint main(void) {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "c-calc".into(),
            name_key: "project.tpl_c_calc".into(),
            description_key: "project.tpl_c_calc_desc".into(),
            language: ProjectLanguage::C,
            category: "school".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 15,
            files: vec![
                TemplateFile {
                    path: "calc.c".into(),
                    content: "#include <stdio.h>\n\nint main(void) {\n    double a, b;\n    char op;\n    printf(\"Enter expression (e.g. 2 + 3): \");\n    scanf(\"%lf %c %lf\", &a, &op, &b);\n    double result;\n    switch (op) {\n        case '+': result = a + b; break;\n        case '-': result = a - b; break;\n        case '*': result = a * b; break;\n        case '/':\n            if (b == 0) { printf(\"Error: division by zero\\n\"); return 1; }\n            result = a / b; break;\n        default: printf(\"Error: unknown operator\\n\"); return 1;\n    }\n    printf(\"%.2f %c %.2f = %.2f\\n\", a, op, b, result);\n    return 0;\n}\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "cpp-hello".into(),
            name_key: "project.tpl_cpp_hello".into(),
            description_key: "project.tpl_cpp_hello_desc".into(),
            language: ProjectLanguage::Cpp,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 5,
            files: vec![
                TemplateFile {
                    path: "hello.cpp".into(),
                    content: "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "cpp-student-db".into(),
            name_key: "project.tpl_cpp_student".into(),
            description_key: "project.tpl_cpp_student_desc".into(),
            language: ProjectLanguage::Cpp,
            category: "school".into(),
            difficulty: "advanced".into(),
            estimated_minutes: 30,
            files: vec![
                TemplateFile {
                    path: "student.cpp".into(),
                    content: "#include <iostream>\n#include <vector>\n#include <string>\n\nstruct Student {\n    std::string name;\n    int id;\n    double grade;\n};\n\nclass StudentDatabase {\n    std::vector<Student> students;\npublic:\n    void add(const std::string& name, int id, double grade) {\n        students.push_back({name, id, grade});\n    }\n    void display() const {\n        for (const auto& s : students) {\n            std::cout << \"ID: \" << s.id << \", Name: \" << s.name << \", Grade: \" << s.grade << std::endl;\n        }\n    }\n    double average() const {\n        if (students.empty()) return 0.0;\n        double sum = 0.0;\n        for (const auto& s : students) sum += s.grade;\n        return sum / students.size();\n    }\n};\n\nint main() {\n    StudentDatabase db;\n    db.add(\"Alice\", 101, 95.5);\n    db.add(\"Bob\", 102, 87.0);\n    db.add(\"Charlie\", 103, 91.2);\n    std::cout << \"Students:\\n\";\n    db.display();\n    std::cout << \"Average grade: \" << db.average() << std::endl;\n    return 0;\n}\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "md-readme".into(),
            name_key: "project.tpl_md_readme".into(),
            description_key: "project.tpl_md_readme_desc".into(),
            language: ProjectLanguage::Markdown,
            category: "personal".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 10,
            files: vec![
                TemplateFile {
                    path: "README.md".into(),
                    content: "# Project Title\n\n## Description\nA brief description of your project.\n\n## Features\n- Feature 1\n- Feature 2\n- Feature 3\n\n## Installation\n```bash\ngit clone https://github.com/username/project\ncd project\n```\n\n## Usage\n```bash\n./run.sh\n```\n\n## License\nThis project is licensed under the MIT License.\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "md-docs".into(),
            name_key: "project.tpl_md_docs".into(),
            description_key: "project.tpl_md_docs_desc".into(),
            language: ProjectLanguage::Markdown,
            category: "school".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 15,
            files: vec![
                TemplateFile {
                    path: "docs/index.md".into(),
                    content: "# Project Documentation\n\n## Getting Started\nGuide for getting started with the project.\n\n## API Reference\n### Endpoints\n- `GET /api/items` - List all items\n- `POST /api/items` - Create an item\n\n## Examples\n```python\nprint(\"Hello, World!\")\n```\n\n## FAQ\nFrequently asked questions.\n".into(),
                    is_main: true,
                },
                TemplateFile {
                    path: "docs/setup.md".into(),
                    content: "# Setup Guide\n\n## Prerequisites\n- Python 3.8+\n- Node.js 16+\n\n## Installation Steps\n1. Clone the repository\n2. Install dependencies\n3. Configure environment\n".into(),
                    is_main: false,
                },
            ],
        },
        ProjectTemplate {
            id: "sh-backup".into(),
            name_key: "project.tpl_sh_backup".into(),
            description_key: "project.tpl_sh_backup_desc".into(),
            language: ProjectLanguage::Bash,
            category: "personal".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 15,
            files: vec![
                TemplateFile {
                    path: "backup.sh".into(),
                    content: "#!/bin/bash\n# Backup Script\n\nSOURCE_DIR=\"${1:-./data}\"\nBACKUP_DIR=\"${2:-./backups}\"\nTIMESTAMP=$(date +%Y%m%d_%H%M%S)\nBACKUP_FILE=\"${BACKUP_DIR}/backup_${TIMESTAMP}.tar.gz\"\n\nif [ ! -d \"$SOURCE_DIR\" ]; then\n    echo \"Error: Source directory $SOURCE_DIR not found\"\n    exit 1\nfi\n\nmkdir -p \"$BACKUP_DIR\"\ntar -czf \"$BACKUP_FILE\" -C \"$(dirname \"$SOURCE_DIR\")\" \"$(basename \"$SOURCE_DIR\")\"\n\necho \"Backup created: $BACKUP_FILE\"\necho \"Size: $(du -h \"$BACKUP_FILE\" | cut -f1)\"\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "sh-sysinfo".into(),
            name_key: "project.tpl_sh_sysinfo".into(),
            description_key: "project.tpl_sh_sysinfo_desc".into(),
            language: ProjectLanguage::Bash,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 10,
            files: vec![
                TemplateFile {
                    path: "sysinfo.sh".into(),
                    content: "#!/bin/bash\n# System Info Script\n\necho \"=== System Information ===\"\necho \"Hostname: $(hostname)\"\necho \"Kernel: $(uname -r)\"\necho \"Architecture: $(uname -m)\"\necho \"\"\necho \"=== CPU ===\"\ngrep 'model name' /proc/cpuinfo | head -1 | cut -d: -f2 | sed 's/^ //'\necho \"Cores: $(nproc)\"\necho \"\"\necho \"=== Memory ===\"\nfree -h | grep Mem | awk '{print \"Total: \" $2 \" | Used: \" $3 \" | Free: \" $4}'\necho \"\"\necho \"=== Disk ===\"\ndf -h / | tail -1 | awk '{print \"Total: \" $2 \" | Used: \" $3 \" | Available: \" $4}'\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "arduino-blink".into(),
            name_key: "project.tpl_arduino_blink".into(),
            description_key: "project.tpl_arduino_blink_desc".into(),
            language: ProjectLanguage::Arduino,
            category: "tutorial".into(),
            difficulty: "beginner".into(),
            estimated_minutes: 10,
            files: vec![
                TemplateFile {
                    path: "blink.ino".into(),
                    content: "// Blink LED\nconst int LED_PIN = 13;\n\nvoid setup() {\n  pinMode(LED_PIN, OUTPUT);\n}\n\nvoid loop() {\n  digitalWrite(LED_PIN, HIGH);\n  delay(1000);\n  digitalWrite(LED_PIN, LOW);\n  delay(1000);\n}\n".into(),
                    is_main: true,
                },
            ],
        },
        ProjectTemplate {
            id: "arduino-temp".into(),
            name_key: "project.tpl_arduino_temp".into(),
            description_key: "project.tpl_arduino_temp_desc".into(),
            language: ProjectLanguage::Arduino,
            category: "school".into(),
            difficulty: "intermediate".into(),
            estimated_minutes: 20,
            files: vec![
                TemplateFile {
                    path: "temperature.ino".into(),
                    content: "// Temperature Sensor (LM35)\nconst int SENSOR_PIN = A0;\n\nvoid setup() {\n  Serial.begin(9600);\n}\n\nvoid loop() {\n  int reading = analogRead(SENSOR_PIN);\n  float voltage = reading * (5.0 / 1024.0);\n  float temperature = voltage * 100.0;\n  \n  Serial.print(\"Temperature: \");\n  Serial.print(temperature);\n  Serial.println(\" C\");\n  \n  delay(2000);\n}\n".into(),
                    is_main: true,
                },
            ],
        },
    ]
}

fn default_templates() -> Vec<ProjectTemplate> {
    default_templates_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn setup_hub() -> (ProjectHub, PathBuf) {
        let dir = std::env::temp_dir().join(format!("edushell-test-{}", generate_id()));
        std::fs::create_dir_all(&dir).unwrap();
        let hub = ProjectHub::new(Some(dir.clone()));
        (hub, dir)
    }

    fn teardown(dir: &Path) {
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn test_hub_creation() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (hub, dir) = setup_hub();
        assert!(hub.all_projects().is_empty());
        assert!(hub.recent_projects.is_empty());
        assert!(hub.favorites.is_empty());
        assert_eq!(hub.templates().len(), 26);
        teardown(&dir);
    }

    #[test]
    fn test_create_project_blank() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Test Project", ProjectLanguage::Python, None).unwrap();
        assert_eq!(project.title, "Test Project");
        assert_eq!(project.language, ProjectLanguage::Python);
        assert_eq!(project.status, ProjectStatus::Draft);
        assert!(project.template_id.is_none());
        assert!(!project.id.is_empty());
        assert_eq!(hub.all_projects().len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_create_project_from_template() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Template Project", ProjectLanguage::Html, Some("html-basic")).unwrap();
        assert_eq!(project.title, "Template Project");
        assert_eq!(project.template_id, Some("html-basic".to_string()));
        assert!(project.path.join("index.html").exists());
        assert!(project.path.join("project.json").exists());
        teardown(&dir);
    }

    #[test]
    fn test_templates_list() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (hub, dir) = setup_hub();
        let templates = hub.templates();
        assert_eq!(templates.len(), 26);
        let ids: Vec<&str> = templates.iter().map(|t| t.id.as_str()).collect();
        assert!(ids.contains(&"html-basic"));
        assert!(ids.contains(&"py-hello"));
        assert!(ids.contains(&"rs-hello"));
        teardown(&dir);
    }

    #[test]
    fn test_templates_filter_by_language() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (hub, dir) = setup_hub();
        let py_templates = hub.templates_for_language(ProjectLanguage::Python);
        assert_eq!(py_templates.len(), 4);
        for t in &py_templates {
            assert_eq!(t.language, ProjectLanguage::Python);
        }
        let rs_templates = hub.templates_for_language(ProjectLanguage::Rust);
        assert_eq!(rs_templates.len(), 2);
        teardown(&dir);
    }

    #[test]
    fn test_project_retrieval() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Find Me", ProjectLanguage::JavaScript, None).unwrap();
        let found = hub.open_project(&project.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Find Me");
        teardown(&dir);
    }

    #[test]
    fn test_project_retrieval_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (hub, dir) = setup_hub();
        let found = hub.open_project("nonexistent-id");
        assert!(found.is_none());
        teardown(&dir);
    }

    #[test]
    fn test_project_deletion() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Delete Me", ProjectLanguage::Rust, None).unwrap();
        assert_eq!(hub.all_projects().len(), 1);
        hub.delete_project(&project.id).unwrap();
        assert_eq!(hub.all_projects().len(), 0);
        assert!(hub.open_project(&project.id).is_none());
        teardown(&dir);
    }

    #[test]
    fn test_project_deletion_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let result = hub.delete_project("nonexistent");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_project_duplication() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let original = hub.create_project("Original", ProjectLanguage::Python, None).unwrap();
        let orig_id = original.id.clone();
        let dup = hub.duplicate_project(&orig_id).unwrap();
        assert_eq!(hub.all_projects().len(), 2);
        assert!(dup.title.contains("Original"));
        assert_eq!(dup.language, ProjectLanguage::Python);
        assert_eq!(dup.status, ProjectStatus::Draft);
        assert_ne!(dup.id, orig_id);
        teardown(&dir);
    }

    #[test]
    fn test_project_renaming() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Old Name", ProjectLanguage::C, None).unwrap();
        hub.rename_project(&project.id, "New Name").unwrap();
        let renamed = hub.open_project(&project.id).unwrap();
        assert_eq!(renamed.title, "New Name");
        teardown(&dir);
    }

    #[test]
    fn test_status_update() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Status", ProjectLanguage::Java, None).unwrap();
        assert_eq!(project.status, ProjectStatus::Draft);
        hub.update_status(&project.id, ProjectStatus::InProgress);
        assert_eq!(hub.open_project(&project.id).unwrap().status, ProjectStatus::InProgress);
        hub.update_status(&project.id, ProjectStatus::Completed);
        assert_eq!(hub.open_project(&project.id).unwrap().status, ProjectStatus::Completed);
        hub.update_status(&project.id, ProjectStatus::Archived);
        assert_eq!(hub.open_project(&project.id).unwrap().status, ProjectStatus::Archived);
        hub.update_status(&project.id, ProjectStatus::Submitted);
        assert_eq!(hub.open_project(&project.id).unwrap().status, ProjectStatus::Submitted);
        teardown(&dir);
    }

    #[test]
    fn test_tag_add_remove() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Tags", ProjectLanguage::Bash, None).unwrap();
        hub.add_tag(&project.id, "linux");
        hub.add_tag(&project.id, "script");
        assert_eq!(hub.open_project(&project.id).unwrap().tags.len(), 2);
        hub.add_tag(&project.id, "linux");
        assert_eq!(hub.open_project(&project.id).unwrap().tags.len(), 2);
        hub.remove_tag(&project.id, "linux");
        assert_eq!(hub.open_project(&project.id).unwrap().tags.len(), 1);
        assert_eq!(hub.open_project(&project.id).unwrap().tags[0], "script");
        teardown(&dir);
    }

    #[test]
    fn test_favorite_toggle() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Fav", ProjectLanguage::Markdown, None).unwrap();
        assert!(!hub.is_favorite(&project.id));
        hub.toggle_favorite(&project.id);
        assert!(hub.is_favorite(&project.id));
        assert_eq!(hub.favorite_projects().len(), 1);
        hub.toggle_favorite(&project.id);
        assert!(!hub.is_favorite(&project.id));
        assert_eq!(hub.favorite_projects().len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_projects_by_language() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("Py1", ProjectLanguage::Python, None).unwrap();
        hub.create_project("Py2", ProjectLanguage::Python, None).unwrap();
        hub.create_project("Rs1", ProjectLanguage::Rust, None).unwrap();
        let py_projects = hub.projects_by_language(ProjectLanguage::Python);
        assert_eq!(py_projects.len(), 2);
        let rs_projects = hub.projects_by_language(ProjectLanguage::Rust);
        assert_eq!(rs_projects.len(), 1);
        let js_projects = hub.projects_by_language(ProjectLanguage::JavaScript);
        assert_eq!(js_projects.len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_projects_by_status() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("Draft", ProjectLanguage::Python, None).unwrap();
        let p2 = hub.create_project("In Progress", ProjectLanguage::Python, None).unwrap();
        hub.update_status(&p2.id, ProjectStatus::InProgress);
        let p3 = hub.create_project("Completed", ProjectLanguage::Python, None).unwrap();
        hub.update_status(&p3.id, ProjectStatus::Completed);
        assert_eq!(hub.projects_by_status(ProjectStatus::Draft).len(), 1);
        assert_eq!(hub.projects_by_status(ProjectStatus::InProgress).len(), 1);
        assert_eq!(hub.projects_by_status(ProjectStatus::Completed).len(), 1);
        assert_eq!(hub.projects_by_status(ProjectStatus::Archived).len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_projects_by_category() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("School", ProjectLanguage::Python, Some("py-data-analysis")).unwrap();
        hub.create_project("Personal", ProjectLanguage::Html, Some("html-portfolio")).unwrap();
        hub.create_project("Tutorial", ProjectLanguage::Html, Some("html-basic")).unwrap();
        assert_eq!(hub.projects_by_category("school").len(), 1);
        assert_eq!(hub.projects_by_category("personal").len(), 1);
        assert_eq!(hub.projects_by_category("tutorial").len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_projects_by_tag() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p1 = hub.create_project("A", ProjectLanguage::Python, None).unwrap();
        let _p2 = hub.create_project("B", ProjectLanguage::Python, None).unwrap();
        hub.add_tag(&p1.id, "math");
        hub.add_tag(&p1.id, "science");
        hub.add_tag(&_p2.id, "math");
        assert_eq!(hub.projects_by_tag("math").len(), 2);
        assert_eq!(hub.projects_by_tag("science").len(), 1);
        assert_eq!(hub.projects_by_tag("history").len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_search() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("Calculator App", ProjectLanguage::JavaScript, None).unwrap();
        hub.create_project("Data Analysis", ProjectLanguage::Python, None).unwrap();
        hub.create_project("Web Server", ProjectLanguage::Rust, None).unwrap();
        let results = hub.search("calc");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Calculator App");
        let results = hub.search("data");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Data Analysis");
        let results = hub.search("nonexistent");
        assert_eq!(results.len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_search_with_tags() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p = hub.create_project("My Project", ProjectLanguage::Python, None).unwrap();
        hub.add_tag(&p.id, "machine-learning");
        let results = hub.search("machine");
        assert_eq!(results.len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_recent_projects() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("First", ProjectLanguage::Python, None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        hub.create_project("Second", ProjectLanguage::Python, None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        hub.create_project("Third", ProjectLanguage::Python, None).unwrap();
        let recent = hub.recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].title, "Third");
        assert_eq!(recent[1].title, "Second");
        let recent_all = hub.recent(10);
        assert_eq!(recent_all.len(), 3);
        teardown(&dir);
    }

    #[test]
    fn test_stats_calculation() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p1 = hub.create_project("A", ProjectLanguage::Python, None).unwrap();
        let p2 = hub.create_project("B", ProjectLanguage::Rust, None).unwrap();
        let p3 = hub.create_project("C", ProjectLanguage::Python, None).unwrap();
        hub.update_status(&p1.id, ProjectStatus::Completed);
        hub.update_status(&p2.id, ProjectStatus::InProgress);
        hub.toggle_favorite(&p3.id);
        let stats = hub.stats();
        assert_eq!(stats.total_projects, 3);
        assert_eq!(stats.completed_projects, 1);
        assert_eq!(stats.in_progress, 1);
        assert_eq!(stats.drafts, 1);
        assert_eq!(stats.archived, 0);
        assert_eq!(stats.favorite_count, 1);
        assert_eq!(*stats.by_language.get("Python").unwrap_or(&0), 2);
        assert_eq!(*stats.by_language.get("Rust").unwrap_or(&0), 1);
        teardown(&dir);
    }

    #[test]
    fn test_language_name() {
        let (hub, dir) = setup_hub();
        let name = hub.language_name(ProjectLanguage::Python);
        assert!(!name.is_empty());
        let name = hub.language_name(ProjectLanguage::Other("CustomLang".to_string()));
        assert_eq!(name, "CustomLang");
        teardown(&dir);
    }

    #[test]
    fn test_language_extension() {
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Html), "html");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Python), "py");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Rust), "rs");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::JavaScript), "js");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::C), "c");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Cpp), "cpp");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Java), "java");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Bash), "sh");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Markdown), "md");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Arduino), "ino");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Css), "css");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Scratch), "sb3");
        assert_eq!(ProjectHub::language_extension(ProjectLanguage::Other("x".into())), "txt");
    }

    #[test]
    fn test_language_icon() {
        assert_eq!(ProjectHub::language_icon(ProjectLanguage::Python), "application-python");
        assert_eq!(ProjectHub::language_icon(ProjectLanguage::Html), "text-html");
        assert_eq!(ProjectHub::language_icon(ProjectLanguage::Other("x".into())), "application-octet-stream");
    }

    #[test]
    fn test_supported_languages() {
        let langs = ProjectHub::supported_languages();
        assert!(langs.contains(&ProjectLanguage::Html));
        assert!(langs.contains(&ProjectLanguage::Css));
        assert!(langs.contains(&ProjectLanguage::JavaScript));
        assert!(langs.contains(&ProjectLanguage::Python));
        assert!(langs.contains(&ProjectLanguage::Rust));
        assert!(langs.contains(&ProjectLanguage::C));
        assert!(langs.contains(&ProjectLanguage::Cpp));
        assert!(langs.contains(&ProjectLanguage::Java));
        assert!(langs.contains(&ProjectLanguage::Scratch));
        assert!(langs.contains(&ProjectLanguage::Arduino));
        assert!(langs.contains(&ProjectLanguage::Markdown));
        assert!(langs.contains(&ProjectLanguage::Bash));
        assert_eq!(langs.len(), 12);
    }

    #[test]
    fn test_locale_switching() {
        let (mut hub, dir) = setup_hub();
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
        hub.set_locale("en-US");
        assert_eq!(hub.localization.current_language(), Lang::English);
        hub.set_locale("id-ID");
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
        teardown(&dir);
    }

    #[test]
    fn test_all_templates_accessible() {
        let (hub, dir) = setup_hub();
        for t in hub.templates() {
            let found = hub.template(&t.id);
            assert!(found.is_some(), "template {} should be accessible", t.id);
        }
        assert!(hub.template("nonexistent").is_none());
        teardown(&dir);
    }

    #[test]
    fn test_template_file_retrieval() {
        let (hub, dir) = setup_hub();
        let files = hub.get_template_files("html-basic");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "index.html");
        assert!(files[0].is_main);
        let files = hub.get_template_files("html-portfolio");
        assert_eq!(files.len(), 2);
        let files = hub.get_template_files("nonexistent");
        assert!(files.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_grade_and_feedback() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Grade", ProjectLanguage::Python, None).unwrap();
        assert!(hub.open_project(&project.id).unwrap().grade.is_none());
        assert!(hub.open_project(&project.id).unwrap().feedback.is_none());
        hub.set_grade(&project.id, 95);
        assert_eq!(hub.open_project(&project.id).unwrap().grade, Some(95));
        hub.set_grade(&project.id, 150);
        assert_eq!(hub.open_project(&project.id).unwrap().grade, Some(100));
        hub.set_feedback(&project.id, "Excellent work!");
        assert_eq!(hub.open_project(&project.id).unwrap().feedback.as_deref(), Some("Excellent work!"));
        teardown(&dir);
    }

    #[test]
    fn test_edge_case_empty_title() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("", ProjectLanguage::Python, None).unwrap();
        assert_eq!(project.title, "");
        assert_eq!(hub.all_projects().len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_edge_case_duplicate_tags() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p = hub.create_project("Test", ProjectLanguage::Python, None).unwrap();
        hub.add_tag(&p.id, "tag1");
        hub.add_tag(&p.id, "tag1");
        assert_eq!(hub.open_project(&p.id).unwrap().tags.len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_edge_case_update_status_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.update_status("nonexistent", ProjectStatus::Completed);
        assert_eq!(hub.all_projects().len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_create_project_with_multiple_files() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Multi", ProjectLanguage::Rust, Some("rs-hello")).unwrap();
        assert_eq!(project.file_count, 2);
        assert!(project.path.join("src/main.rs").exists());
        assert!(project.path.join("Cargo.toml").exists());
        teardown(&dir);
    }

    #[test]
    fn test_stats_empty() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (hub, dir) = setup_hub();
        let stats = hub.stats();
        assert_eq!(stats.total_projects, 0);
        assert_eq!(stats.completed_projects, 0);
        assert_eq!(stats.in_progress, 0);
        assert_eq!(stats.archived, 0);
        assert_eq!(stats.drafts, 0);
        assert_eq!(stats.favorite_count, 0);
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_size_kb, 0);
        assert!(stats.by_language.is_empty());
        assert!(stats.by_category.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_export_import_dir() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Export", ProjectLanguage::Python, None).unwrap();
        let export_path = hub.export_project(&project.id, "dir").unwrap();
        assert!(export_path.exists());
        assert!(export_path.join("project.json").exists());
        let (mut hub2, dir2) = setup_hub();
        let imported = hub2.import_project(&export_path).unwrap();
        assert_eq!(imported.title, "Export");
        assert_eq!(imported.language, ProjectLanguage::Python);
        assert_eq!(hub2.all_projects().len(), 1);
        let _ = std::fs::remove_dir_all(&export_path);
        teardown(&dir);
        teardown(&dir2);
    }

    #[test]
    fn test_duplicate_template_project() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let original = hub.create_project("Original", ProjectLanguage::Cpp, Some("cpp-hello")).unwrap();
        let orig_id = original.id.clone();
        let dup = hub.duplicate_project(&orig_id).unwrap();
        assert!(dup.title.contains("(copy)"));
        assert_eq!(hub.all_projects().len(), 2);
        teardown(&dir);
    }

    #[test]
    fn test_open_project_mut_modify() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Mut", ProjectLanguage::Rust, None).unwrap();
        {
            let p = hub.open_project_mut(&project.id).unwrap();
            p.description = "Modified description".to_string();
        }
        assert_eq!(hub.open_project(&project.id).unwrap().description, "Modified description");
        teardown(&dir);
    }

    #[test]
    fn test_rename_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let result = hub.rename_project("nope", "New Name");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_duplicate_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let result = hub.duplicate_project("nope");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_export_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (hub, dir) = setup_hub();
        let result = hub.export_project("nope", "dir");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_export_invalid_format() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Format", ProjectLanguage::Python, None).unwrap();
        let result = hub.export_project(&project.id, "pdf");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_templates_for_language_none() {
        let (hub, dir) = setup_hub();
        let templates = hub.templates_for_language(ProjectLanguage::Java);
        assert!(templates.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_search_case_insensitive() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("My Python Project", ProjectLanguage::Python, None).unwrap();
        let results = hub.search("PYTHON");
        assert_eq!(results.len(), 1);
        let results = hub.search("python");
        assert_eq!(results.len(), 1);
        let results = hub.search("project");
        assert_eq!(results.len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_favorite_projects_list() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p1 = hub.create_project("A", ProjectLanguage::Python, None).unwrap();
        let _p2 = hub.create_project("B", ProjectLanguage::Python, None).unwrap();
        let p3 = hub.create_project("C", ProjectLanguage::Python, None).unwrap();
        hub.toggle_favorite(&p1.id);
        hub.toggle_favorite(&p3.id);
        let favs = hub.favorite_projects();
        assert_eq!(favs.len(), 2);
        assert!(favs.iter().any(|p| p.id == p1.id));
        assert!(favs.iter().any(|p| p.id == p3.id));
        teardown(&dir);
    }

    #[test]
    fn test_all_projects_mut() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("A", ProjectLanguage::Python, None).unwrap();
        hub.create_project("B", ProjectLanguage::Python, None).unwrap();
        let projects = hub.all_projects_mut();
        assert_eq!(projects.len(), 2);
        projects.clear();
        assert_eq!(hub.all_projects().len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_remove_tag_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p = hub.create_project("Test", ProjectLanguage::Python, None).unwrap();
        hub.remove_tag(&p.id, "nonexistent");
        assert!(hub.open_project(&p.id).unwrap().tags.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_project_created_with_template_category() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p = hub.create_project("School Proj", ProjectLanguage::Python, Some("py-data-analysis")).unwrap();
        assert_eq!(p.category, "school");
        teardown(&dir);
    }

    #[test]
    fn test_project_no_template_default_category() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p = hub.create_project("Personal", ProjectLanguage::Python, None).unwrap();
        assert_eq!(p.category, "personal");
        teardown(&dir);
    }

    #[test]
    fn test_project_serialization_roundtrip() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Serialization", ProjectLanguage::Rust, Some("rs-hello")).unwrap();
        let json = serde_json::to_string_pretty(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, project.id);
        assert_eq!(deserialized.title, project.title);
        assert_eq!(deserialized.language, project.language);
        assert_eq!(deserialized.status, project.status);
        assert_eq!(deserialized.template_id, project.template_id);
        teardown(&dir);
    }

    #[test]
    fn test_stats_by_category() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.create_project("A", ProjectLanguage::Python, Some("py-data-analysis")).unwrap();
        hub.create_project("B", ProjectLanguage::Python, Some("py-data-analysis")).unwrap();
        hub.create_project("C", ProjectLanguage::Html, Some("html-portfolio")).unwrap();
        let stats = hub.stats();
        assert_eq!(*stats.by_category.get("school").unwrap_or(&0), 2);
        assert_eq!(*stats.by_category.get("personal").unwrap_or(&0), 1);
        teardown(&dir);
    }

    #[test]
    fn test_language_other_extension() {
        assert_eq!(ProjectLanguage::Other("custom".into()).extension(), "txt");
    }

    #[test]
    fn test_language_other_icon() {
        assert_eq!(ProjectLanguage::Other("custom".into()).icon(), "application-octet-stream");
    }

    #[test]
    fn test_create_project_persists_files() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("Persist", ProjectLanguage::Html, Some("html-form")).unwrap();
        let form_content = std::fs::read_to_string(project.path.join("form.html")).unwrap();
        assert!(form_content.contains("Contact Form"));
        teardown(&dir);
    }

    #[test]
    fn test_delete_removes_fs_dir() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let project = hub.create_project("FS", ProjectLanguage::Python, None).unwrap();
        let project_path = project.path.clone();
        assert!(project_path.exists());
        hub.delete_project(&project.id).unwrap();
        assert!(!project_path.exists());
        teardown(&dir);
    }

    #[test]
    fn test_recent_empty_when_no_projects() {
        let (hub, dir) = setup_hub();
        let recent = hub.recent(5);
        assert!(recent.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_favorite_toggle_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        hub.toggle_favorite("nope");
        assert!(!hub.is_favorite("nope"));
        teardown(&dir);
    }

    #[test]
    fn test_set_grade_clamps() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (mut hub, dir) = setup_hub();
        let p = hub.create_project("Grade Clamp", ProjectLanguage::Python, None).unwrap();
        hub.set_grade(&p.id, 200);
        assert_eq!(hub.open_project(&p.id).unwrap().grade, Some(100));
        teardown(&dir);
    }
}
