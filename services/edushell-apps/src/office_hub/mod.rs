use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::localization::*;

pub type Result<T> = std::result::Result<T, OfficeHubError>;

#[derive(Debug, thiserror::Error)]
pub enum OfficeHubError {
    #[error("document not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("template not found: {0}")]
    TemplateNotFound(String),
    #[error("LibreOffice not found")]
    LibreOfficeNotFound,
    #[error("invalid document type: {0}")]
    InvalidDocumentType(String),
    #[error("duplicate error: {0}")]
    DuplicateError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfficeApp {
    Writer,
    Calc,
    Impress,
    Draw,
    Math,
    Base,
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    Document,
    Spreadsheet,
    Presentation,
    Drawing,
    Formula,
    Database,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficeDocument {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
    pub doc_type: DocumentType,
    pub app: OfficeApp,
    pub created_at: String,
    pub modified_at: String,
    pub size_kb: u64,
    pub favorite: bool,
    pub tags: Vec<String>,
    pub template_id: Option<String>,
    pub last_opened: Option<String>,
    pub open_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficeTemplate {
    pub id: String,
    pub name_key: String,
    pub description_key: String,
    pub doc_type: DocumentType,
    pub app: OfficeApp,
    pub category: String,
    pub thumbnail: Option<String>,
    pub estimated_pages: u32,
    pub difficulty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OfficeHubStats {
    pub total_documents: u32,
    pub total_templates: u32,
    pub by_type: HashMap<String, u32>,
    pub favorite_count: u32,
    pub recent_count: u32,
    pub total_size_kb: u64,
    pub most_used_app: Option<String>,
}

fn now_iso() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string()
}

fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn document_extension_inner(doc_type: DocumentType) -> &'static str {
    match doc_type {
        DocumentType::Document => "odt",
        DocumentType::Spreadsheet => "ods",
        DocumentType::Presentation => "odp",
        DocumentType::Drawing => "odg",
        DocumentType::Formula => "odf",
        DocumentType::Database => "odb",
    }
}

fn app_executable_inner(app: &OfficeApp) -> &'static str {
    match app {
        OfficeApp::Writer => "libreoffice --writer",
        OfficeApp::Calc => "libreoffice --calc",
        OfficeApp::Impress => "libreoffice --impress",
        OfficeApp::Draw => "libreoffice --draw",
        OfficeApp::Math => "libreoffice --math",
        OfficeApp::Base => "libreoffice --base",
        OfficeApp::Unknown(_) => "libreoffice",
    }
}

fn app_icon_inner(app: &OfficeApp) -> &'static str {
    match app {
        OfficeApp::Writer => "libreoffice-writer",
        OfficeApp::Calc => "libreoffice-calc",
        OfficeApp::Impress => "libreoffice-impress",
        OfficeApp::Draw => "libreoffice-draw",
        OfficeApp::Math => "libreoffice-math",
        OfficeApp::Base => "libreoffice-base",
        OfficeApp::Unknown(_) => "libreoffice",
    }
}

fn doc_type_to_app(doc_type: DocumentType) -> OfficeApp {
    match doc_type {
        DocumentType::Document => OfficeApp::Writer,
        DocumentType::Spreadsheet => OfficeApp::Calc,
        DocumentType::Presentation => OfficeApp::Impress,
        DocumentType::Drawing => OfficeApp::Draw,
        DocumentType::Formula => OfficeApp::Math,
        DocumentType::Database => OfficeApp::Base,
    }
}

pub struct OfficeHub {
    documents: Vec<OfficeDocument>,
    templates: Vec<OfficeTemplate>,
    recent_documents: Vec<String>,
    favorites: Vec<String>,
    localization: LocalizationManager,
    documents_dir: PathBuf,
}

impl OfficeHub {
    pub fn new(documents_dir: Option<PathBuf>) -> Self {
        let localization = LocalizationManager::new();
        let documents_dir = documents_dir.unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".local/share/edushell/office")
        });
        let templates = default_templates();
        let documents = Self::scan_documents(&documents_dir);
        let favorites: Vec<String> = documents.iter().filter(|d| d.favorite).map(|d| d.id.clone()).collect();
        let mut sorted: Vec<&OfficeDocument> = documents.iter().collect();
        sorted.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        let recent_documents: Vec<String> = sorted.iter().take(20).map(|d| d.id.clone()).collect();
        OfficeHub {
            documents,
            templates,
            recent_documents,
            favorites,
            localization,
            documents_dir,
        }
    }

    fn scan_documents(dir: &Path) -> Vec<OfficeDocument> {
        if !dir.exists() {
            return Vec::new();
        }
        let mut docs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let meta_path = path.join("document.json");
                    if let Ok(content) = std::fs::read_to_string(&meta_path) {
                        if let Ok(doc) = serde_json::from_str::<OfficeDocument>(&content) {
                            docs.push(doc);
                        }
                    }
                }
            }
        }
        docs
    }

    fn save_document_meta(doc: &OfficeDocument, docs_dir: &Path) -> Result<()> {
        let doc_dir = docs_dir.join(&doc.id);
        std::fs::create_dir_all(&doc_dir)?;
        let meta_path = doc_dir.join("document.json");
        let content = serde_json::to_string_pretty(doc)?;
        std::fs::write(meta_path, content)?;
        Ok(())
    }

    fn calculate_file_size(path: &Path) -> u64 {
        std::fs::metadata(path).map(|m| (m.len() + 1023) / 1024).unwrap_or(0)
    }

    pub fn create_document(
        &mut self,
        title: &str,
        doc_type: DocumentType,
        template_id: Option<&str>,
    ) -> Result<OfficeDocument> {
        let id = generate_id();
        let now = now_iso();
        let app = doc_type_to_app(doc_type);
        let ext = document_extension_inner(doc_type);
        let doc_dir = self.documents_dir.join(&id);
        std::fs::create_dir_all(&doc_dir)?;
        let file_name = format!("{}.{}", sanitize_filename(title), ext);
        let file_path = doc_dir.join(&file_name);
        std::fs::write(&file_path, " ")?;
        let size_kb = Self::calculate_file_size(&file_path);
        let _category = template_id
            .and_then(|tid| self.templates.iter().find(|t| t.id == tid).map(|t| t.category.clone()))
            .unwrap_or_else(|| "general".to_string());
        let doc = OfficeDocument {
            id: id.clone(),
            title: title.to_string(),
            path: file_path,
            doc_type,
            app,
            created_at: now.clone(),
            modified_at: now,
            size_kb,
            favorite: false,
            tags: Vec::new(),
            template_id: template_id.map(String::from),
            last_opened: None,
            open_count: 0,
        };
        Self::save_document_meta(&doc, &self.documents_dir)?;
        self.recent_documents.insert(0, id.clone());
        self.documents.push(doc.clone());
        Ok(doc)
    }

    pub fn open_application(app: OfficeApp, document: Option<&OfficeDocument>) {
        let mut cmd = std::process::Command::new("libreoffice");
        match &app {
            OfficeApp::Writer => { cmd.arg("--writer"); }
            OfficeApp::Calc => { cmd.arg("--calc"); }
            OfficeApp::Impress => { cmd.arg("--impress"); }
            OfficeApp::Draw => { cmd.arg("--draw"); }
            OfficeApp::Math => { cmd.arg("--math"); }
            OfficeApp::Base => { cmd.arg("--base"); }
            OfficeApp::Unknown(s) => { cmd.arg(s); }
        }
        if let Some(doc) = document {
            cmd.arg(&doc.path);
        }
        let _ = cmd.spawn();
    }

    pub fn open_document(&mut self, id: &str) -> Option<&OfficeDocument> {
        let pos = self.documents.iter().position(|d| d.id == id)?;
        {
            let doc = &mut self.documents[pos];
            doc.open_count += 1;
            doc.last_opened = Some(now_iso());
        }
        self.recent_documents.retain(|x| x != id);
        self.recent_documents.insert(0, id.to_string());
        Some(&self.documents[pos])
    }

    pub fn rename_document(&mut self, id: &str, new_title: &str) -> Result<()> {
        let doc = self.documents.iter_mut().find(|d| d.id == id).ok_or_else(|| OfficeHubError::NotFound(id.to_string()))?;
        let ext = document_extension_inner(doc.doc_type);
        let new_file_name = format!("{}.{}", sanitize_filename(new_title), ext);
        let new_path = doc.path.parent().unwrap_or(&self.documents_dir).join(&new_file_name);
        if new_path != doc.path {
            std::fs::rename(&doc.path, &new_path)?;
            doc.path = new_path;
        }
        doc.title = new_title.to_string();
        doc.modified_at = now_iso();
        Self::save_document_meta(doc, &self.documents_dir)?;
        Ok(())
    }

    pub fn delete_document(&mut self, id: &str) -> Result<()> {
        let pos = self.documents.iter().position(|d| d.id == id).ok_or_else(|| OfficeHubError::NotFound(id.to_string()))?;
        let doc = self.documents.remove(pos);
        if let Some(parent) = doc.path.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }
        self.recent_documents.retain(|x| x != id);
        self.favorites.retain(|x| x != id);
        Ok(())
    }

    pub fn duplicate_document(&mut self, id: &str) -> Result<OfficeDocument> {
        let original = self.documents.iter().find(|d| d.id == id).ok_or_else(|| OfficeHubError::NotFound(id.to_string()))?;
        let new_id = generate_id();
        let now = now_iso();
        let new_doc_dir = self.documents_dir.join(&new_id);
        std::fs::create_dir_all(&new_doc_dir)?;
        let ext = document_extension_inner(original.doc_type);
        let file_name = format!("{} (copy).{}", sanitize_filename(&original.title), ext);
        let new_path = new_doc_dir.join(&file_name);
        if original.path.exists() {
            std::fs::copy(&original.path, &new_path)?;
        } else {
            std::fs::write(&new_path, "")?;
        }
        let size_kb = Self::calculate_file_size(&new_path);
        let dup = OfficeDocument {
            id: new_id.clone(),
            title: format!("{} (copy)", original.title),
            path: new_path,
            doc_type: original.doc_type,
            app: original.app.clone(),
            created_at: now.clone(),
            modified_at: now,
            size_kb,
            favorite: false,
            tags: original.tags.clone(),
            template_id: original.template_id.clone(),
            last_opened: None,
            open_count: 0,
        };
        Self::save_document_meta(&dup, &self.documents_dir)?;
        self.recent_documents.insert(0, dup.id.clone());
        self.documents.push(dup.clone());
        Ok(dup)
    }

    pub fn all_documents(&self) -> &[OfficeDocument] {
        &self.documents
    }

    pub fn documents_by_type(&self, doc_type: DocumentType) -> Vec<&OfficeDocument> {
        self.documents.iter().filter(|d| d.doc_type == doc_type).collect()
    }

    pub fn documents_by_app(&self, app: OfficeApp) -> Vec<&OfficeDocument> {
        self.documents.iter().filter(|d| d.app == app).collect()
    }

    pub fn recent(&self, count: usize) -> Vec<&OfficeDocument> {
        self.recent_documents.iter()
            .take(count)
            .filter_map(|id| self.documents.iter().find(|d| d.id == *id))
            .collect()
    }

    pub fn favorite_documents(&self) -> Vec<&OfficeDocument> {
        self.documents.iter().filter(|d| d.favorite).collect()
    }

    pub fn toggle_favorite(&mut self, id: &str) {
        if let Some(doc) = self.documents.iter_mut().find(|d| d.id == id) {
            doc.favorite = !doc.favorite;
            if doc.favorite {
                self.favorites.push(doc.id.clone());
            } else {
                self.favorites.retain(|x| x != id);
            }
            let _ = Self::save_document_meta(doc, &self.documents_dir);
        }
    }

    pub fn is_favorite(&self, id: &str) -> bool {
        self.documents.iter().any(|d| d.id == id && d.favorite)
    }

    pub fn search(&self, query: &str) -> Vec<&OfficeDocument> {
        let q = query.to_lowercase();
        self.documents
            .iter()
            .filter(|d| {
                d.title.to_lowercase().contains(&q)
                    || d.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn templates(&self) -> &[OfficeTemplate] {
        &self.templates
    }

    pub fn templates_for_type(&self, doc_type: DocumentType) -> Vec<&OfficeTemplate> {
        self.templates.iter().filter(|t| t.doc_type == doc_type).collect()
    }

    pub fn template(&self, id: &str) -> Option<&OfficeTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }

    pub fn quick_create_templates(&self) -> Vec<&OfficeTemplate> {
        self.templates.iter().filter(|t| t.difficulty == "beginner").collect()
    }

    pub fn stats(&self) -> OfficeHubStats {
        let total_documents = self.documents.len() as u32;
        let total_templates = self.templates.len() as u32;
        let mut by_type: HashMap<String, u32> = HashMap::new();
        let mut favorite_count = 0u32;
        let mut total_size_kb = 0u64;
        let mut app_usage: HashMap<String, u32> = HashMap::new();
        for d in &self.documents {
            let type_key = format!("{:?}", d.doc_type);
            *by_type.entry(type_key).or_insert(0) += 1;
            total_size_kb += d.size_kb;
            if d.favorite {
                favorite_count += 1;
            }
            let app_key = format!("{:?}", d.app);
            *app_usage.entry(app_key).or_insert(0) += d.open_count;
        }
        let most_used_app = app_usage.into_iter().max_by_key(|(_, c)| *c).map(|(a, _)| a);
        OfficeHubStats {
            total_documents,
            total_templates,
            by_type,
            favorite_count,
            recent_count: self.recent_documents.len() as u32,
            total_size_kb,
            most_used_app,
        }
    }

    pub fn app_name(&self, app: OfficeApp) -> String {
        let key = match app {
            OfficeApp::Writer => "office.app_writer",
            OfficeApp::Calc => "office.app_calc",
            OfficeApp::Impress => "office.app_impress",
            OfficeApp::Draw => "office.app_draw",
            OfficeApp::Math => "office.app_math",
            OfficeApp::Base => "office.app_base",
            OfficeApp::Unknown(s) => return s,
        };
        self.localization.get(key).to_string()
    }

    pub fn app_executable(app: OfficeApp) -> &'static str {
        app_executable_inner(&app)
    }

    pub fn app_icon(app: OfficeApp) -> &'static str {
        app_icon_inner(&app)
    }

    pub fn document_extension(doc_type: DocumentType) -> &'static str {
        document_extension_inner(doc_type)
    }

    pub fn document_type_name(&self, doc_type: DocumentType) -> String {
        let key = match doc_type {
            DocumentType::Document => "office.type_document",
            DocumentType::Spreadsheet => "office.type_spreadsheet",
            DocumentType::Presentation => "office.type_presentation",
            DocumentType::Drawing => "office.type_drawing",
            DocumentType::Formula => "office.type_formula",
            DocumentType::Database => "office.type_database",
        };
        self.localization.get(key).to_string()
    }

    pub fn app_available(_app: OfficeApp) -> bool {
        Self::libreoffice_path().is_some()
    }

    pub fn libreoffice_path() -> Option<PathBuf> {
        let candidates = &[
            "/usr/bin/libreoffice",
            "/usr/local/bin/libreoffice",
            "/snap/bin/libreoffice",
            "/usr/bin/libreoffice7.6",
            "/usr/bin/libreoffice7.5",
            "/usr/bin/libreoffice7.4",
            "/usr/bin/libreoffice7.3",
            "/usr/bin/libreoffice7.2",
            "/usr/bin/libreoffice7.1",
            "/usr/bin/libreoffice7.0",
            "/usr/lib/libreoffice/program/soffice",
        ];
        for path in candidates {
            if Path::new(path).exists() {
                return Some(PathBuf::from(path));
            }
        }
        if let Ok(output) = std::process::Command::new("which").arg("libreoffice").output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    return Some(PathBuf::from(path_str));
                }
            }
        }
        None
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = match locale {
            "id-ID" | "id" => Lang::Indonesian,
            _ => Lang::English,
        };
        self.localization.set_language(lang);
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' { c } else { '_' })
        .collect::<String>()
        .trim()
        .to_string()
}

fn default_templates() -> Vec<OfficeTemplate> {
    vec![
        OfficeTemplate {
            id: "writer-blank".into(),
            name_key: "office.tpl_writer_blank".into(),
            description_key: "office.tpl_writer_blank_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "general".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "writer-letter".into(),
            name_key: "office.tpl_writer_letter".into(),
            description_key: "office.tpl_writer_letter_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "correspondence".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "writer-resume".into(),
            name_key: "office.tpl_writer_resume".into(),
            description_key: "office.tpl_writer_resume_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "career".into(),
            thumbnail: None,
            estimated_pages: 2,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "writer-report".into(),
            name_key: "office.tpl_writer_report".into(),
            description_key: "office.tpl_writer_report_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "academic".into(),
            thumbnail: None,
            estimated_pages: 5,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "writer-essay".into(),
            name_key: "office.tpl_writer_essay".into(),
            description_key: "office.tpl_writer_essay_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "academic".into(),
            thumbnail: None,
            estimated_pages: 3,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "writer-certificate".into(),
            name_key: "office.tpl_writer_certificate".into(),
            description_key: "office.tpl_writer_certificate_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "formal".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "writer-lesson-plan".into(),
            name_key: "office.tpl_writer_lesson_plan".into(),
            description_key: "office.tpl_writer_lesson_plan_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "education".into(),
            thumbnail: None,
            estimated_pages: 2,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "writer-notes".into(),
            name_key: "office.tpl_writer_notes".into(),
            description_key: "office.tpl_writer_notes_desc".into(),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            category: "personal".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "calc-blank".into(),
            name_key: "office.tpl_calc_blank".into(),
            description_key: "office.tpl_calc_blank_desc".into(),
            doc_type: DocumentType::Spreadsheet,
            app: OfficeApp::Calc,
            category: "general".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "calc-budget".into(),
            name_key: "office.tpl_calc_budget".into(),
            description_key: "office.tpl_calc_budget_desc".into(),
            doc_type: DocumentType::Spreadsheet,
            app: OfficeApp::Calc,
            category: "finance".into(),
            thumbnail: None,
            estimated_pages: 3,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "calc-grade-book".into(),
            name_key: "office.tpl_calc_grade_book".into(),
            description_key: "office.tpl_calc_grade_book_desc".into(),
            doc_type: DocumentType::Spreadsheet,
            app: OfficeApp::Calc,
            category: "education".into(),
            thumbnail: None,
            estimated_pages: 2,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "calc-attendance".into(),
            name_key: "office.tpl_calc_attendance".into(),
            description_key: "office.tpl_calc_attendance_desc".into(),
            doc_type: DocumentType::Spreadsheet,
            app: OfficeApp::Calc,
            category: "education".into(),
            thumbnail: None,
            estimated_pages: 2,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "calc-schedule".into(),
            name_key: "office.tpl_calc_schedule".into(),
            description_key: "office.tpl_calc_schedule_desc".into(),
            doc_type: DocumentType::Spreadsheet,
            app: OfficeApp::Calc,
            category: "personal".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "calc-invoice".into(),
            name_key: "office.tpl_calc_invoice".into(),
            description_key: "office.tpl_calc_invoice_desc".into(),
            doc_type: DocumentType::Spreadsheet,
            app: OfficeApp::Calc,
            category: "business".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "impress-blank".into(),
            name_key: "office.tpl_impress_blank".into(),
            description_key: "office.tpl_impress_blank_desc".into(),
            doc_type: DocumentType::Presentation,
            app: OfficeApp::Impress,
            category: "general".into(),
            thumbnail: None,
            estimated_pages: 5,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "impress-class".into(),
            name_key: "office.tpl_impress_class".into(),
            description_key: "office.tpl_impress_class_desc".into(),
            doc_type: DocumentType::Presentation,
            app: OfficeApp::Impress,
            category: "education".into(),
            thumbnail: None,
            estimated_pages: 10,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "impress-proposal".into(),
            name_key: "office.tpl_impress_proposal".into(),
            description_key: "office.tpl_impress_proposal_desc".into(),
            doc_type: DocumentType::Presentation,
            app: OfficeApp::Impress,
            category: "business".into(),
            thumbnail: None,
            estimated_pages: 8,
            difficulty: "advanced".into(),
        },
        OfficeTemplate {
            id: "impress-portfolio".into(),
            name_key: "office.tpl_impress_portfolio".into(),
            description_key: "office.tpl_impress_portfolio_desc".into(),
            doc_type: DocumentType::Presentation,
            app: OfficeApp::Impress,
            category: "career".into(),
            thumbnail: None,
            estimated_pages: 6,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "draw-blank".into(),
            name_key: "office.tpl_draw_blank".into(),
            description_key: "office.tpl_draw_blank_desc".into(),
            doc_type: DocumentType::Drawing,
            app: OfficeApp::Draw,
            category: "general".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "draw-flowchart".into(),
            name_key: "office.tpl_draw_flowchart".into(),
            description_key: "office.tpl_draw_flowchart_desc".into(),
            doc_type: DocumentType::Drawing,
            app: OfficeApp::Draw,
            category: "education".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "draw-mindmap".into(),
            name_key: "office.tpl_draw_mindmap".into(),
            description_key: "office.tpl_draw_mindmap_desc".into(),
            doc_type: DocumentType::Drawing,
            app: OfficeApp::Draw,
            category: "education".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "intermediate".into(),
        },
        OfficeTemplate {
            id: "draw-poster".into(),
            name_key: "office.tpl_draw_poster".into(),
            description_key: "office.tpl_draw_poster_desc".into(),
            doc_type: DocumentType::Drawing,
            app: OfficeApp::Draw,
            category: "design".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "advanced".into(),
        },
        OfficeTemplate {
            id: "math-formula".into(),
            name_key: "office.tpl_math_formula".into(),
            description_key: "office.tpl_math_formula_desc".into(),
            doc_type: DocumentType::Formula,
            app: OfficeApp::Math,
            category: "education".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "beginner".into(),
        },
        OfficeTemplate {
            id: "base-blank".into(),
            name_key: "office.tpl_base_blank".into(),
            description_key: "office.tpl_base_blank_desc".into(),
            doc_type: DocumentType::Database,
            app: OfficeApp::Base,
            category: "general".into(),
            thumbnail: None,
            estimated_pages: 1,
            difficulty: "advanced".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn setup_hub() -> (OfficeHub, PathBuf) {
        let dir = std::env::temp_dir().join(format!("edushell-office-test-{}", generate_id()));
        std::fs::create_dir_all(&dir).unwrap();
        let hub = OfficeHub::new(Some(dir.clone()));
        (hub, dir)
    }

    fn teardown(dir: &Path) {
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn test_hub_creation() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (hub, dir) = setup_hub();
        assert!(hub.all_documents().is_empty());
        assert!(hub.recent_documents.is_empty());
        assert!(hub.favorites.is_empty());
        assert_eq!(hub.templates().len(), 24);
        teardown(&dir);
    }

    #[test]
    fn test_create_blank_document() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Test Doc", DocumentType::Document, None).unwrap();
        assert_eq!(doc.title, "Test Doc");
        assert_eq!(doc.doc_type, DocumentType::Document);
        assert_eq!(doc.app, OfficeApp::Writer);
        assert!(doc.template_id.is_none());
        assert!(!doc.id.is_empty());
        assert_eq!(doc.open_count, 0);
        assert!(doc.last_opened.is_none());
        assert_eq!(hub.all_documents().len(), 1);
        assert!(doc.path.exists());
        teardown(&dir);
    }

    #[test]
    fn test_create_document_from_template() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Template Doc", DocumentType::Presentation, Some("impress-blank")).unwrap();
        assert_eq!(doc.title, "Template Doc");
        assert_eq!(doc.template_id, Some("impress-blank".to_string()));
        assert_eq!(doc.doc_type, DocumentType::Presentation);
        assert_eq!(doc.app, OfficeApp::Impress);
        assert!(doc.path.exists());
        teardown(&dir);
    }

    #[test]
    fn test_create_spreadsheet() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Budget", DocumentType::Spreadsheet, Some("calc-budget")).unwrap();
        assert_eq!(doc.doc_type, DocumentType::Spreadsheet);
        assert_eq!(doc.app, OfficeApp::Calc);
        let ext = doc.path.extension().unwrap().to_str().unwrap();
        assert_eq!(ext, "ods");
        teardown(&dir);
    }

    #[test]
    fn test_create_database() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("DB", DocumentType::Database, None).unwrap();
        assert_eq!(doc.doc_type, DocumentType::Database);
        assert_eq!(doc.app, OfficeApp::Base);
        let ext = doc.path.extension().unwrap().to_str().unwrap();
        assert_eq!(ext, "odb");
        teardown(&dir);
    }

    #[test]
    fn test_create_math_formula() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Formula", DocumentType::Formula, None).unwrap();
        assert_eq!(doc.doc_type, DocumentType::Formula);
        assert_eq!(doc.app, OfficeApp::Math);
        let ext = doc.path.extension().unwrap().to_str().unwrap();
        assert_eq!(ext, "odf");
        teardown(&dir);
    }

    #[test]
    fn test_create_drawing() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Drawing", DocumentType::Drawing, None).unwrap();
        assert_eq!(doc.doc_type, DocumentType::Drawing);
        assert_eq!(doc.app, OfficeApp::Draw);
        let ext = doc.path.extension().unwrap().to_str().unwrap();
        assert_eq!(ext, "odg");
        teardown(&dir);
    }

    #[test]
    fn test_open_document_updates_stats() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Stats", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        let opened = hub.open_document(&id);
        assert!(opened.is_some());
        assert_eq!(opened.unwrap().open_count, 1);
        assert!(opened.unwrap().last_opened.is_some());
        let opened_again = hub.open_document(&id);
        assert_eq!(opened_again.unwrap().open_count, 2);
        teardown(&dir);
    }

    #[test]
    fn test_open_document_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let result = hub.open_document("nonexistent");
        assert!(result.is_none());
        teardown(&dir);
    }

    #[test]
    fn test_rename_document() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Old Name", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        hub.rename_document(&id, "New Name").unwrap();
        let renamed = hub.all_documents().iter().find(|d| d.id == id).unwrap();
        assert_eq!(renamed.title, "New Name");
        assert!(renamed.path.exists());
        assert!(renamed.path.to_string_lossy().contains("New Name"));
        teardown(&dir);
    }

    #[test]
    fn test_rename_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let result = hub.rename_document("nope", "New Name");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_delete_document() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Delete Me", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        let path = doc.path.clone();
        assert!(path.exists());
        assert_eq!(hub.all_documents().len(), 1);
        hub.delete_document(&id).unwrap();
        assert_eq!(hub.all_documents().len(), 0);
        assert!(!path.parent().unwrap().exists());
        teardown(&dir);
    }

    #[test]
    fn test_delete_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let result = hub.delete_document("nope");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_duplicate_document() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let original = hub.create_document("Original", DocumentType::Spreadsheet, None).unwrap();
        let orig_id = original.id.clone();
        let dup = hub.duplicate_document(&orig_id).unwrap();
        assert_eq!(hub.all_documents().len(), 2);
        assert!(dup.title.contains("Original"));
        assert!(dup.title.contains("copy"));
        assert_eq!(dup.doc_type, DocumentType::Spreadsheet);
        assert_eq!(dup.app, OfficeApp::Calc);
        assert_ne!(dup.id, orig_id);
        assert!(!dup.favorite);
        assert_eq!(dup.open_count, 0);
        teardown(&dir);
    }

    #[test]
    fn test_duplicate_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let result = hub.duplicate_document("nope");
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_recent_documents_tracking() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let a = hub.create_document("A", DocumentType::Document, None).unwrap();
        let _b = hub.create_document("B", DocumentType::Document, None).unwrap();
        let _c = hub.create_document("C", DocumentType::Document, None).unwrap();
        let recent = hub.recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].title, "C");
        assert_eq!(recent[1].title, "B");
        let recent_all = hub.recent(10);
        assert_eq!(recent_all.len(), 3);
        hub.open_document(&a.id);
        let recent_after_open = hub.recent(3);
        assert_eq!(recent_after_open[0].title, "A");
        teardown(&dir);
    }

    #[test]
    fn test_recent_empty_when_no_documents() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (hub, dir) = setup_hub();
        let recent = hub.recent(5);
        assert!(recent.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_favorite_toggle() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Fav", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        assert!(!hub.is_favorite(&id));
        hub.toggle_favorite(&id);
        assert!(hub.is_favorite(&id));
        assert_eq!(hub.favorite_documents().len(), 1);
        hub.toggle_favorite(&id);
        assert!(!hub.is_favorite(&id));
        assert_eq!(hub.favorite_documents().len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_favorite_toggle_nonexistent() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        hub.toggle_favorite("nope");
        assert!(!hub.is_favorite("nope"));
        teardown(&dir);
    }

    #[test]
    fn test_favorite_documents_list() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let d1 = hub.create_document("A", DocumentType::Document, None).unwrap();
        let _d2 = hub.create_document("B", DocumentType::Document, None).unwrap();
        let d3 = hub.create_document("C", DocumentType::Document, None).unwrap();
        hub.toggle_favorite(&d1.id);
        hub.toggle_favorite(&d3.id);
        let favs = hub.favorite_documents();
        assert_eq!(favs.len(), 2);
        assert!(favs.iter().any(|d| d.id == d1.id));
        assert!(favs.iter().any(|d| d.id == d3.id));
        teardown(&dir);
    }

    #[test]
    fn test_search() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        hub.create_document("Annual Report", DocumentType::Document, None).unwrap();
        hub.create_document("Budget Planning", DocumentType::Spreadsheet, None).unwrap();
        hub.create_document("Class Presentation", DocumentType::Presentation, None).unwrap();
        let results = hub.search("report");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Annual Report");
        let results = hub.search("budget");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Budget Planning");
        let results = hub.search("nonexistent");
        assert_eq!(results.len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_search_case_insensitive() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        hub.create_document("My Document", DocumentType::Document, None).unwrap();
        let results = hub.search("DOCUMENT");
        assert_eq!(results.len(), 1);
        let results = hub.search("document");
        assert_eq!(results.len(), 1);
        let results = hub.search("MY");
        assert_eq!(results.len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_search_with_tags() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let mut doc = hub.create_document("Project Plan", DocumentType::Document, None).unwrap();
        doc.tags.push("important".to_string());
        doc.tags.push("urgent".to_string());
        let pos = hub.documents.iter().position(|d| d.id == doc.id).unwrap();
        hub.documents[pos].tags = doc.tags;
        let results = hub.search("important");
        assert_eq!(results.len(), 1);
        let results = hub.search("urgent");
        assert_eq!(results.len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_stats_calculation() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        hub.create_document("Doc1", DocumentType::Document, None).unwrap();
        hub.create_document("Sheet1", DocumentType::Spreadsheet, None).unwrap();
        hub.create_document("Doc2", DocumentType::Document, None).unwrap();
        let docs = hub.all_documents().to_vec();
        hub.toggle_favorite(&docs[0].id);
        hub.open_document(&docs[0].id);
        hub.open_document(&docs[0].id);
        hub.open_document(&docs[1].id);
        let stats = hub.stats();
        assert_eq!(stats.total_documents, 3);
        assert_eq!(stats.total_templates, 24);
        assert_eq!(stats.favorite_count, 1);
        assert_eq!(*stats.by_type.get("Document").unwrap_or(&0), 2);
        assert_eq!(*stats.by_type.get("Spreadsheet").unwrap_or(&0), 1);
        assert!(stats.total_size_kb > 0);
        assert_eq!(stats.most_used_app, Some("Writer".to_string()));
        teardown(&dir);
    }

    #[test]
    fn test_stats_empty() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (hub, dir) = setup_hub();
        let stats = hub.stats();
        assert_eq!(stats.total_documents, 0);
        assert_eq!(stats.total_templates, 24);
        assert_eq!(stats.favorite_count, 0);
        assert_eq!(stats.recent_count, 0);
        assert_eq!(stats.total_size_kb, 0);
        assert!(stats.by_type.is_empty());
        assert!(stats.most_used_app.is_none());
        teardown(&dir);
    }

    #[test]
    fn test_templates_list() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (hub, dir) = setup_hub();
        let templates = hub.templates();
        assert_eq!(templates.len(), 24);
        let ids: Vec<&str> = templates.iter().map(|t| t.id.as_str()).collect();
        assert!(ids.contains(&"writer-blank"));
        assert!(ids.contains(&"calc-blank"));
        assert!(ids.contains(&"impress-blank"));
        assert!(ids.contains(&"draw-blank"));
        assert!(ids.contains(&"math-formula"));
        assert!(ids.contains(&"base-blank"));
        teardown(&dir);
    }

    #[test]
    fn test_templates_filter_by_type() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (hub, dir) = setup_hub();
        let writer_templates = hub.templates_for_type(DocumentType::Document);
        assert_eq!(writer_templates.len(), 8);
        for t in &writer_templates {
            assert_eq!(t.doc_type, DocumentType::Document);
        }
        let calc_templates = hub.templates_for_type(DocumentType::Spreadsheet);
        assert_eq!(calc_templates.len(), 6);
        let impress_templates = hub.templates_for_type(DocumentType::Presentation);
        assert_eq!(impress_templates.len(), 4);
        let draw_templates = hub.templates_for_type(DocumentType::Drawing);
        assert_eq!(draw_templates.len(), 4);
        let math_templates = hub.templates_for_type(DocumentType::Formula);
        assert_eq!(math_templates.len(), 1);
        let base_templates = hub.templates_for_type(DocumentType::Database);
        assert_eq!(base_templates.len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_all_templates_accessible() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (hub, dir) = setup_hub();
        for t in hub.templates() {
            let found = hub.template(&t.id);
            assert!(found.is_some(), "template {} should be accessible", t.id);
        }
        assert!(hub.template("nonexistent").is_none());
        teardown(&dir);
    }

    #[test]
    fn test_quick_create_templates() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (hub, dir) = setup_hub();
        let quick = hub.quick_create_templates();
        assert!(!quick.is_empty());
        for t in &quick {
            assert_eq!(t.difficulty, "beginner");
        }
        teardown(&dir);
    }

    #[test]
    fn test_app_name() {
        let (hub, dir) = setup_hub();
        let name = hub.app_name(OfficeApp::Writer);
        assert!(!name.is_empty());
        let name = hub.app_name(OfficeApp::Calc);
        assert!(!name.is_empty());
        let name = hub.app_name(OfficeApp::Unknown("Custom".to_string()));
        assert_eq!(name, "Custom");
        teardown(&dir);
    }

    #[test]
    fn test_app_executable() {
        assert_eq!(OfficeHub::app_executable(OfficeApp::Writer), "libreoffice --writer");
        assert_eq!(OfficeHub::app_executable(OfficeApp::Calc), "libreoffice --calc");
        assert_eq!(OfficeHub::app_executable(OfficeApp::Impress), "libreoffice --impress");
        assert_eq!(OfficeHub::app_executable(OfficeApp::Draw), "libreoffice --draw");
        assert_eq!(OfficeHub::app_executable(OfficeApp::Math), "libreoffice --math");
        assert_eq!(OfficeHub::app_executable(OfficeApp::Base), "libreoffice --base");
        assert_eq!(OfficeHub::app_executable(OfficeApp::Unknown("x".into())), "libreoffice");
    }

    #[test]
    fn test_app_icon() {
        assert_eq!(OfficeHub::app_icon(OfficeApp::Writer), "libreoffice-writer");
        assert_eq!(OfficeHub::app_icon(OfficeApp::Calc), "libreoffice-calc");
        assert_eq!(OfficeHub::app_icon(OfficeApp::Impress), "libreoffice-impress");
        assert_eq!(OfficeHub::app_icon(OfficeApp::Draw), "libreoffice-draw");
        assert_eq!(OfficeHub::app_icon(OfficeApp::Math), "libreoffice-math");
        assert_eq!(OfficeHub::app_icon(OfficeApp::Base), "libreoffice-base");
        assert_eq!(OfficeHub::app_icon(OfficeApp::Unknown("x".into())), "libreoffice");
    }

    #[test]
    fn test_document_extension() {
        assert_eq!(OfficeHub::document_extension(DocumentType::Document), "odt");
        assert_eq!(OfficeHub::document_extension(DocumentType::Spreadsheet), "ods");
        assert_eq!(OfficeHub::document_extension(DocumentType::Presentation), "odp");
        assert_eq!(OfficeHub::document_extension(DocumentType::Drawing), "odg");
        assert_eq!(OfficeHub::document_extension(DocumentType::Formula), "odf");
        assert_eq!(OfficeHub::document_extension(DocumentType::Database), "odb");
    }

    #[test]
    fn test_document_type_name() {
        let (hub, dir) = setup_hub();
        let name = hub.document_type_name(DocumentType::Document);
        assert!(!name.is_empty());
        let name = hub.document_type_name(DocumentType::Spreadsheet);
        assert!(!name.is_empty());
        let name = hub.document_type_name(DocumentType::Presentation);
        assert!(!name.is_empty());
        let name = hub.document_type_name(DocumentType::Drawing);
        assert!(!name.is_empty());
        let name = hub.document_type_name(DocumentType::Formula);
        assert!(!name.is_empty());
        let name = hub.document_type_name(DocumentType::Database);
        assert!(!name.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_documents_by_type() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        hub.create_document("Doc", DocumentType::Document, None).unwrap();
        hub.create_document("Sheet", DocumentType::Spreadsheet, None).unwrap();
        hub.create_document("Pres", DocumentType::Presentation, None).unwrap();
        let docs = hub.documents_by_type(DocumentType::Document);
        assert_eq!(docs.len(), 1);
        let sheets = hub.documents_by_type(DocumentType::Spreadsheet);
        assert_eq!(sheets.len(), 1);
        let pres = hub.documents_by_type(DocumentType::Presentation);
        assert_eq!(pres.len(), 1);
        let drawings = hub.documents_by_type(DocumentType::Drawing);
        assert_eq!(drawings.len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_documents_by_app() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        hub.create_document("Doc", DocumentType::Document, None).unwrap();
        hub.create_document("Sheet", DocumentType::Spreadsheet, None).unwrap();
        hub.create_document("Pres", DocumentType::Presentation, None).unwrap();
        let writer_docs = hub.documents_by_app(OfficeApp::Writer);
        assert_eq!(writer_docs.len(), 1);
        let calc_docs = hub.documents_by_app(OfficeApp::Calc);
        assert_eq!(calc_docs.len(), 1);
        let impress_docs = hub.documents_by_app(OfficeApp::Impress);
        assert_eq!(impress_docs.len(), 1);
        let draw_docs = hub.documents_by_app(OfficeApp::Draw);
        assert_eq!(draw_docs.len(), 0);
        teardown(&dir);
    }

    #[test]
    fn test_libreoffice_path_detection() {
        let path = OfficeHub::libreoffice_path();
        if path.is_some() {
            assert!(path.unwrap().exists());
        }
    }

    #[test]
    fn test_app_available() {
        let available = OfficeHub::app_available(OfficeApp::Writer);
        assert_eq!(available, OfficeHub::libreoffice_path().is_some());
    }

    #[test]
    fn test_locale_switching() {
        let (mut hub, dir) = setup_hub();
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
        hub.set_locale("en-US");
        assert_eq!(hub.localization.current_language(), Lang::English);
        hub.set_locale("id-ID");
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
        hub.set_locale("fr-FR");
        assert_eq!(hub.localization.current_language(), Lang::English);
        teardown(&dir);
    }

    #[test]
    fn test_multiple_doc_types_management() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        hub.create_document("Doc", DocumentType::Document, None).unwrap();
        hub.create_document("Sheet", DocumentType::Spreadsheet, None).unwrap();
        hub.create_document("Pres", DocumentType::Presentation, None).unwrap();
        hub.create_document("Draw", DocumentType::Drawing, None).unwrap();
        hub.create_document("Math", DocumentType::Formula, None).unwrap();
        hub.create_document("DB", DocumentType::Database, None).unwrap();
        assert_eq!(hub.all_documents().len(), 6);
        assert_eq!(hub.documents_by_type(DocumentType::Document).len(), 1);
        assert_eq!(hub.documents_by_type(DocumentType::Spreadsheet).len(), 1);
        assert_eq!(hub.documents_by_type(DocumentType::Presentation).len(), 1);
        assert_eq!(hub.documents_by_type(DocumentType::Drawing).len(), 1);
        assert_eq!(hub.documents_by_type(DocumentType::Formula).len(), 1);
        assert_eq!(hub.documents_by_type(DocumentType::Database).len(), 1);
        teardown(&dir);
    }

    #[test]
    fn test_edge_case_empty_title() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("", DocumentType::Document, None).unwrap();
        assert_eq!(doc.title, "");
        assert_eq!(hub.all_documents().len(), 1);
        assert!(doc.path.exists());
        teardown(&dir);
    }

    #[test]
    fn test_edge_case_duplicate_delete() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Test", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        hub.delete_document(&id).unwrap();
        let result = hub.delete_document(&id);
        assert!(result.is_err());
        teardown(&dir);
    }

    #[test]
    fn test_edge_case_duplicate_twice() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Original", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        let _dup1 = hub.duplicate_document(&id).unwrap();
        let _dup2 = hub.duplicate_document(&id).unwrap();
        assert_eq!(hub.all_documents().len(), 3);
        teardown(&dir);
    }

    #[test]
    fn test_document_serialization_roundtrip() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Serialize", DocumentType::Presentation, Some("impress-class")).unwrap();
        let json = serde_json::to_string_pretty(&doc).unwrap();
        let deserialized: OfficeDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, doc.id);
        assert_eq!(deserialized.title, doc.title);
        assert_eq!(deserialized.doc_type, doc.doc_type);
        assert_eq!(deserialized.app, doc.app);
        assert_eq!(deserialized.template_id, doc.template_id);
        teardown(&dir);
    }

    #[test]
    fn test_template_thumbnail_optional() {
        let (hub, dir) = setup_hub();
        let t = hub.template("writer-blank").unwrap();
        assert!(t.thumbnail.is_none());
        teardown(&dir);
    }

    #[test]
    fn test_toggle_favorite_removes_from_favorites_on_delete() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Fav Delete", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        hub.toggle_favorite(&id);
        assert!(hub.is_favorite(&id));
        hub.delete_document(&id).unwrap();
        assert!(!hub.is_favorite(&id));
        assert!(hub.favorite_documents().is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_recent_does_not_include_deleted() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Temp", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        assert_eq!(hub.recent(5).len(), 1);
        hub.delete_document(&id).unwrap();
        assert!(hub.recent(5).is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_create_document_persists_meta() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("Persist", DocumentType::Spreadsheet, None).unwrap();
        let meta_path = dir.join(&doc.id).join("document.json");
        assert!(meta_path.exists());
        let content = std::fs::read_to_string(&meta_path).unwrap();
        assert!(content.contains("Persist"));
        teardown(&dir);
    }

    #[test]
    fn test_rename_document_updates_file_name() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let doc = hub.create_document("First Name", DocumentType::Document, None).unwrap();
        let old_path = doc.path.clone();
        assert!(old_path.to_string_lossy().contains("First Name"));
        hub.rename_document(&doc.id, "Second Name").unwrap();
        let renamed = hub.all_documents().iter().find(|d| d.id == doc.id).unwrap();
        assert!(renamed.path.to_string_lossy().contains("Second Name"));
        assert!(!old_path.exists());
        teardown(&dir);
    }

    #[test]
    fn test_app_name_all_variants() {
        let (hub, dir) = setup_hub();
        let apps = vec![
            OfficeApp::Writer,
            OfficeApp::Calc,
            OfficeApp::Impress,
            OfficeApp::Draw,
            OfficeApp::Math,
            OfficeApp::Base,
        ];
        for app in apps {
            let name = hub.app_name(app);
            assert!(!name.is_empty(), "app_name should not be empty");
        }
        teardown(&dir);
    }

    #[test]
    fn test_document_type_name_all_variants() {
        let (hub, dir) = setup_hub();
        let types = vec![
            DocumentType::Document,
            DocumentType::Spreadsheet,
            DocumentType::Presentation,
            DocumentType::Drawing,
            DocumentType::Formula,
            DocumentType::Database,
        ];
        for dt in types {
            let name = hub.document_type_name(dt);
            assert!(!name.is_empty(), "document_type_name should not be empty");
        }
        teardown(&dir);
    }

    #[test]
    fn test_open_document_recent_order() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let a = hub.create_document("A", DocumentType::Document, None).unwrap();
        let b = hub.create_document("B", DocumentType::Document, None).unwrap();
        let _c = hub.create_document("C", DocumentType::Document, None).unwrap();
        assert_eq!(hub.recent(3).iter().map(|d| d.title.as_str()).collect::<Vec<_>>(), vec!["C", "B", "A"]);
        hub.open_document(&a.id);
        assert_eq!(hub.recent(3).iter().map(|d| d.title.as_str()).collect::<Vec<_>>(), vec!["A", "C", "B"]);
        hub.open_document(&b.id);
        assert_eq!(hub.recent(3).iter().map(|d| d.title.as_str()).collect::<Vec<_>>(), vec!["B", "A", "C"]);
        teardown(&dir);
    }

    #[test]
    fn test_template_difficulty_levels() {
        let (hub, dir) = setup_hub();
        let mut beginner = 0u32;
        let mut intermediate = 0u32;
        let mut advanced = 0u32;
        for t in hub.templates() {
            match t.difficulty.as_str() {
                "beginner" => beginner += 1,
                "intermediate" => intermediate += 1,
                "advanced" => advanced += 1,
                other => panic!("unexpected difficulty: {}", other),
            }
        }
        assert!(beginner > 0);
        assert!(intermediate > 0);
        assert!(advanced > 0);
        assert_eq!(beginner + intermediate + advanced, hub.templates().len() as u32);
        teardown(&dir);
    }

    #[test]
    fn test_template_categories() {
        let (hub, dir) = setup_hub();
        for t in hub.templates() {
            assert!(!t.category.is_empty(), "template {} has empty category", t.id);
            assert!(!t.name_key.is_empty(), "template {} has empty name_key", t.id);
            assert!(!t.description_key.is_empty(), "template {} has empty description_key", t.id);
            assert!(t.estimated_pages > 0, "template {} has zero pages", t.id);
        }
        teardown(&dir);
    }

    #[test]
    fn test_delete_removes_from_favorites() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let a = hub.create_document("A", DocumentType::Document, None).unwrap();
        let b = hub.create_document("B", DocumentType::Document, None).unwrap();
        hub.toggle_favorite(&a.id);
        hub.toggle_favorite(&b.id);
        assert_eq!(hub.favorites.len(), 2);
        hub.delete_document(&a.id).unwrap();
        assert_eq!(hub.favorites.len(), 1);
        assert!(!hub.favorites.contains(&a.id));
        assert!(hub.favorites.contains(&b.id));
        teardown(&dir);
    }

    #[test]
    fn test_duplicate_preserves_template_id() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (mut hub, dir) = setup_hub();
        let original = hub.create_document("From Template", DocumentType::Presentation, Some("impress-blank")).unwrap();
        let dup = hub.duplicate_document(&original.id).unwrap();
        assert_eq!(dup.template_id, original.template_id);
        teardown(&dir);
    }

    #[test]
    fn test_stats_by_type_empty_when_no_docs() {
        let (hub, dir) = setup_hub();
        let stats = hub.stats();
        assert!(stats.by_type.is_empty());
        teardown(&dir);
    }

    #[test]
    fn test_open_application_does_not_crash() {
        let doc = OfficeDocument {
            id: "test-id".into(),
            title: "Test".into(),
            path: PathBuf::from("/tmp/test.odt"),
            doc_type: DocumentType::Document,
            app: OfficeApp::Writer,
            created_at: now_iso(),
            modified_at: now_iso(),
            size_kb: 1,
            favorite: false,
            tags: vec![],
            template_id: None,
            last_opened: None,
            open_count: 0,
        };
        let mut cmd = std::process::Command::new("libreoffice");
        cmd.arg("--writer").arg(&doc.path);
        let child = cmd.spawn();
        assert!(child.is_ok(), "should spawn libreoffice without error");
        if let Ok(mut c) = child {
            let _ = c.kill();
            let _ = c.wait();
        }
    }

    #[test]
    fn test_scan_documents_loads_existing() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let dir = std::env::temp_dir().join(format!("edushell-office-scan-{}", generate_id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut hub = OfficeHub::new(Some(dir.clone()));
        let doc = hub.create_document("Scan Test", DocumentType::Document, None).unwrap();
        let id = doc.id.clone();
        drop(hub);
        let hub2 = OfficeHub::new(Some(dir.clone()));
        assert_eq!(hub2.all_documents().len(), 1);
        assert_eq!(hub2.all_documents()[0].id, id);
        teardown(&dir);
    }

    #[test]
    fn test_set_locale_from_code() {
        let (mut hub, dir) = setup_hub();
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
        hub.set_locale("en-US");
        assert_eq!(hub.localization.current_language(), Lang::English);
        hub.set_locale("id-ID");
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
        hub.set_locale("id");
        assert_eq!(hub.localization.current_language(), Lang::Indonesian);
        hub.set_locale("en");
        assert_eq!(hub.localization.current_language(), Lang::English);
        teardown(&dir);
    }
}
