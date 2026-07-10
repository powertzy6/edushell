use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::localization::*;

#[derive(Debug, Clone)]
pub enum SoftwareError {
    NotFound(String),
    AlreadyInstalled(String),
    NotInstalled(String),
    InstallFailed(String),
    UninstallFailed(String),
    UpdateFailed(String),
    NoUpdatesAvailable,
}

impl std::fmt::Display for SoftwareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoftwareError::NotFound(id) => write!(f, "app not found: {}", id),
            SoftwareError::AlreadyInstalled(id) => write!(f, "already installed: {}", id),
            SoftwareError::NotInstalled(id) => write!(f, "not installed: {}", id),
            SoftwareError::InstallFailed(id) => write!(f, "install failed: {}", id),
            SoftwareError::UninstallFailed(id) => write!(f, "uninstall failed: {}", id),
            SoftwareError::UpdateFailed(id) => write!(f, "update failed: {}", id),
            SoftwareError::NoUpdatesAvailable => write!(f, "no updates available"),
        }
    }
}

impl std::error::Error for SoftwareError {}

pub type Result<T> = std::result::Result<T, SoftwareError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackageSource {
    System,
    Flatpak,
    Snap,
    AppImage,
    EduShell,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageStatus {
    Installed,
    UpdateAvailable,
    NotInstalled,
    Installing,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub homepage: String,
    pub categories: Vec<String>,
    pub icon: String,
    pub source: PackageSource,
    pub status: PackageStatus,
    pub size_kb: u64,
    pub installed_version: Option<String>,
    pub is_educational: bool,
    pub is_essential: bool,
    pub rating: f32,
    pub install_count: u32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub app_id: String,
    pub app_name: String,
    pub current_version: String,
    pub new_version: String,
    pub size_kb: u64,
    pub importance: String,
    pub changelog: Option<String>,
    pub released_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SoftwareCenterStats {
    pub total_installed: u32,
    pub total_educational: u32,
    pub total_essential: u32,
    pub updates_available: u32,
    pub security_updates: u32,
    pub total_size_kb: u64,
    pub by_source: HashMap<String, u32>,
    pub by_category: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub source: PackageSource,
    pub last_updated: Option<String>,
    pub app_count: u32,
}

pub struct SoftwareCenter {
    installed_apps: Vec<AppEntry>,
    available_apps: Vec<AppEntry>,
    updates: Vec<UpdateInfo>,
    localization: LocalizationManager,
    repos: Vec<RepositoryInfo>,
    #[allow(dead_code)]
    pub use_dummy_data: bool,
}

impl SoftwareCenter {
    pub fn new(use_dummy: bool) -> Self {
        let localization = LocalizationManager::new();
        if use_dummy {
            let apps = Self::dummy_apps();
            let installed: Vec<AppEntry> = apps
                .iter()
                .filter(|a| a.status == PackageStatus::Installed || a.status == PackageStatus::UpdateAvailable)
                .cloned()
                .collect();
            let updates = Self::dummy_updates();
            let repos = Self::dummy_repos();
            Self {
                installed_apps: installed,
                available_apps: apps,
                updates,
                localization,
                repos,
                use_dummy_data: true,
            }
        } else {
            Self {
                installed_apps: Vec::new(),
                available_apps: Vec::new(),
                updates: Vec::new(),
                localization,
                repos: Vec::new(),
                use_dummy_data: false,
            }
        }
    }

    pub fn installed_apps(&self) -> &[AppEntry] {
        &self.installed_apps
    }

    pub fn available_apps(&self) -> &[AppEntry] {
        &self.available_apps
    }

    pub fn all_apps(&self) -> Vec<&AppEntry> {
        let mut seen = std::collections::HashSet::new();
        let mut result: Vec<&AppEntry> = Vec::new();
        for app in self.available_apps.iter() {
            if seen.insert(app.id.clone()) {
                result.push(app);
            }
        }
        for app in self.installed_apps.iter() {
            if seen.insert(app.id.clone()) {
                result.push(app);
            }
        }
        result
    }

    pub fn get_app(&self, id: &str) -> Option<&AppEntry> {
        self.available_apps.iter().find(|a| a.id == id)
            .or_else(|| self.installed_apps.iter().find(|a| a.id == id))
    }

    pub fn get_app_mut(&mut self, id: &str) -> Option<&mut AppEntry> {
        if let Some(app) = self.available_apps.iter_mut().find(|a| a.id == id) {
            Some(app)
        } else {
            self.installed_apps.iter_mut().find(|a| a.id == id)
        }
    }

    pub fn search(&self, query: &str) -> Vec<&AppEntry> {
        let q = query.to_lowercase();
        let mut results: Vec<&AppEntry> = self
            .available_apps
            .iter()
            .chain(self.installed_apps.iter())
            .filter(|a| {
                a.name.to_lowercase().contains(&q)
                    || a.description.to_lowercase().contains(&q)
                    || a.categories.iter().any(|c| c.to_lowercase().contains(&q))
            })
            .collect();
        results.dedup_by_key(|a| a.id.clone());
        results
    }

    pub fn search_installed(&self, query: &str) -> Vec<&AppEntry> {
        let q = query.to_lowercase();
        self.installed_apps
            .iter()
            .filter(|a| {
                a.name.to_lowercase().contains(&q)
                    || a.description.to_lowercase().contains(&q)
                    || a.categories.iter().any(|c| c.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn by_category(&self, cat: &str) -> Vec<&AppEntry> {
        let c = cat.to_lowercase();
        self.available_apps
            .iter()
            .chain(self.installed_apps.iter())
            .filter(|a| a.categories.iter().any(|ac| ac.to_lowercase() == c))
            .collect()
    }

    pub fn by_source(&self, source: PackageSource) -> Vec<&AppEntry> {
        self.available_apps
            .iter()
            .chain(self.installed_apps.iter())
            .filter(|a| a.source == source)
            .collect()
    }

    pub fn educational_apps(&self) -> Vec<&AppEntry> {
        self.available_apps
            .iter()
            .chain(self.installed_apps.iter())
            .filter(|a| a.is_educational)
            .collect()
    }

    pub fn essential_apps(&self) -> Vec<&AppEntry> {
        self.available_apps
            .iter()
            .chain(self.installed_apps.iter())
            .filter(|a| a.is_essential)
            .collect()
    }

    pub fn updates_available(&self) -> &[UpdateInfo] {
        &self.updates
    }

    pub fn updates_count(&self) -> u32 {
        self.updates.len() as u32
    }

    pub fn security_updates(&self) -> Vec<&UpdateInfo> {
        self.updates
            .iter()
            .filter(|u| u.importance == "security")
            .collect()
    }

    pub fn check_for_updates(&mut self) -> Result<()> {
        self.updates.clear();
        for app in &self.available_apps {
            if let Some(ref inst_ver) = app.installed_version {
                if inst_ver != &app.version {
                    self.updates.push(UpdateInfo {
                        app_id: app.id.clone(),
                        app_name: app.name.clone(),
                        current_version: inst_ver.clone(),
                        new_version: app.version.clone(),
                        size_kb: app.size_kb / 10,
                        importance: if app.is_essential {
                            "security".to_string()
                        } else {
                            "recommended".to_string()
                        },
                        changelog: Some(format!("Update from {} to {}", inst_ver, app.version)),
                        released_at: app.updated_at.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn install(&mut self, id: &str) -> Result<()> {
        let app = self
            .available_apps
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or_else(|| SoftwareError::NotFound(id.to_string()))?;

        if app.status == PackageStatus::Installed {
            return Err(SoftwareError::AlreadyInstalled(id.to_string()));
        }

        app.status = PackageStatus::Installing;

        app.status = PackageStatus::Installed;
        app.installed_version = Some(app.version.clone());

        let installed_app = app.clone();
        if !self.installed_apps.iter().any(|a| a.id == id) {
            self.installed_apps.push(installed_app);
        }

        Ok(())
    }

    pub fn uninstall(&mut self, id: &str) -> Result<()> {
        let app = self
            .available_apps
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or_else(|| SoftwareError::NotFound(id.to_string()))?;

        if app.status != PackageStatus::Installed && app.status != PackageStatus::UpdateAvailable {
            return Err(SoftwareError::NotInstalled(id.to_string()));
        }

        app.status = PackageStatus::NotInstalled;
        app.installed_version = None;
        self.installed_apps.retain(|a| a.id != id);
        self.updates.retain(|u| u.app_id != id);

        Ok(())
    }

    pub fn update_app(&mut self, id: &str) -> Result<()> {
        let update_idx = self
            .updates
            .iter()
            .position(|u| u.app_id == id)
            .ok_or_else(|| SoftwareError::UpdateFailed(id.to_string()))?;

        let new_version = self.updates[update_idx].new_version.clone();
        self.updates.remove(update_idx);

        if let Some(app) = self.available_apps.iter_mut().find(|a| a.id == id) {
            app.status = PackageStatus::Installed;
            app.installed_version = Some(new_version.clone());
        }

        if let Some(app) = self.installed_apps.iter_mut().find(|a| a.id == id) {
            app.status = PackageStatus::Installed;
            app.installed_version = Some(new_version);
        }

        Ok(())
    }

    pub fn update_all(&mut self) -> Result<()> {
        let ids: Vec<String> = self.updates.iter().map(|u| u.app_id.clone()).collect();
        for id in ids {
            self.update_app(&id)?;
        }
        Ok(())
    }

    pub fn categories(&self) -> Vec<String> {
        let mut cats = std::collections::HashSet::new();
        for app in self.available_apps.iter().chain(self.installed_apps.iter()) {
            for c in &app.categories {
                cats.insert(c.clone());
            }
        }
        let mut result: Vec<String> = cats.into_iter().collect();
        result.sort();
        result
    }

    pub fn stats(&self) -> SoftwareCenterStats {
        let mut stats = SoftwareCenterStats::default();
        for app in &self.installed_apps {
            stats.total_installed += 1;
            stats.total_size_kb += app.size_kb;
            if app.is_educational {
                stats.total_educational += 1;
            }
            if app.is_essential {
                stats.total_essential += 1;
            }
            *stats
                .by_source
                .entry(format!("{:?}", app.source))
                .or_insert(0) += 1;
            for c in &app.categories {
                *stats.by_category.entry(c.clone()).or_insert(0) += 1;
            }
        }
        stats.updates_available = self.updates.len() as u32;
        stats.security_updates = self
            .updates
            .iter()
            .filter(|u| u.importance == "security")
            .count() as u32;
        stats
    }

    pub fn source_name(&self, source: PackageSource) -> String {
        let key = match source {
            PackageSource::System => "software.system",
            PackageSource::Flatpak => "software.flatpak",
            PackageSource::Snap => "software.snap",
            PackageSource::AppImage => "software.appimage",
            PackageSource::EduShell => "software.source",
            PackageSource::Unknown => "software.source",
        };
        self.localization.get(key).to_string()
    }

    pub fn set_locale(&mut self, locale: &str) {
        match locale {
            "id-ID" => self.localization.set_language(Lang::Indonesian),
            "en-US" => self.localization.set_language(Lang::English),
            _ => {}
        }
    }

    pub fn repos(&self) -> &[RepositoryInfo] {
        &self.repos
    }

    fn dummy_apps() -> Vec<AppEntry> {
        vec![
            AppEntry {
                id: "learning-hub".to_string(),
                name: "Learning Hub".to_string(),
                description: "Central learning platform for EduShell with courses, quizzes, and progress tracking".to_string(),
                version: "2.1.0".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id/learning".to_string(),
                categories: vec!["education".to_string()],
                icon: "learning-hub".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 15360,
                installed_version: Some("2.1.0".to_string()),
                is_educational: true,
                is_essential: true,
                rating: 4.8,
                install_count: 50000,
                created_at: "2024-01-15".to_string(),
                updated_at: "2025-06-01".to_string(),
            },
            AppEntry {
                id: "edu-terminal".to_string(),
                name: "Edu Terminal".to_string(),
                description: "Educational terminal with built-in tutorials and command hints".to_string(),
                version: "1.5.2".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id/terminal".to_string(),
                categories: vec!["education".to_string(), "development".to_string()],
                icon: "edu-terminal".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 5120,
                installed_version: Some("1.5.2".to_string()),
                is_educational: true,
                is_essential: true,
                rating: 4.7,
                install_count: 45000,
                created_at: "2024-01-15".to_string(),
                updated_at: "2025-05-10".to_string(),
            },
            AppEntry {
                id: "project-hub".to_string(),
                name: "Project Hub".to_string(),
                description: "Manage coding projects with templates and build tools".to_string(),
                version: "1.3.0".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id/projects".to_string(),
                categories: vec!["education".to_string(), "development".to_string()],
                icon: "project-hub".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 8192,
                installed_version: Some("1.3.0".to_string()),
                is_educational: true,
                is_essential: false,
                rating: 4.5,
                install_count: 30000,
                created_at: "2024-02-10".to_string(),
                updated_at: "2025-04-20".to_string(),
            },
            AppEntry {
                id: "office-hub".to_string(),
                name: "Office Hub".to_string(),
                description: "Integrated office suite launcher for document, spreadsheet, and presentation work".to_string(),
                version: "1.2.1".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id/office".to_string(),
                categories: vec!["office".to_string(), "education".to_string()],
                icon: "office-hub".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 6144,
                installed_version: Some("1.2.1".to_string()),
                is_educational: true,
                is_essential: true,
                rating: 4.6,
                install_count: 40000,
                created_at: "2024-03-01".to_string(),
                updated_at: "2025-05-15".to_string(),
            },
            AppEntry {
                id: "browser-hub".to_string(),
                name: "Browser Hub".to_string(),
                description: "Web browser hub with educational content filtering and safe browsing".to_string(),
                version: "1.1.0".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id/browser".to_string(),
                categories: vec!["internet".to_string(), "education".to_string()],
                icon: "browser-hub".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 4096,
                installed_version: Some("1.1.0".to_string()),
                is_educational: true,
                is_essential: false,
                rating: 4.3,
                install_count: 25000,
                created_at: "2024-03-15".to_string(),
                updated_at: "2025-03-10".to_string(),
            },
            AppEntry {
                id: "welcome-app".to_string(),
                name: "Welcome App".to_string(),
                description: "First-run wizard to configure EduShell for new users".to_string(),
                version: "1.0.0".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id".to_string(),
                categories: vec!["system".to_string()],
                icon: "welcome-app".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 2048,
                installed_version: Some("1.0.0".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.2,
                install_count: 60000,
                created_at: "2024-01-10".to_string(),
                updated_at: "2024-12-01".to_string(),
            },
            AppEntry {
                id: "vscode".to_string(),
                name: "Visual Studio Code".to_string(),
                description: "Lightweight source code editor with debugging and extensions support".to_string(),
                version: "1.92.0".to_string(),
                author: "Microsoft".to_string(),
                license: "MIT".to_string(),
                homepage: "https://code.visualstudio.com".to_string(),
                categories: vec!["development".to_string()],
                icon: "vscode".to_string(),
                source: PackageSource::Flatpak,
                status: PackageStatus::Installed,
                size_kb: 350000,
                installed_version: Some("1.91.0".to_string()),
                is_educational: false,
                is_essential: false,
                rating: 4.7,
                install_count: 200000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-07-01".to_string(),
            },
            AppEntry {
                id: "git".to_string(),
                name: "Git".to_string(),
                description: "Distributed version control system for tracking code changes".to_string(),
                version: "2.45.0".to_string(),
                author: "Linus Torvalds".to_string(),
                license: "GPL-2.0".to_string(),
                homepage: "https://git-scm.com".to_string(),
                categories: vec!["development".to_string()],
                icon: "git".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 12000,
                installed_version: Some("2.45.0".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.9,
                install_count: 300000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-15".to_string(),
            },
            AppEntry {
                id: "nodejs".to_string(),
                name: "Node.js".to_string(),
                description: "JavaScript runtime built on Chrome V8 engine for server-side development".to_string(),
                version: "20.15.0".to_string(),
                author: "OpenJS Foundation".to_string(),
                license: "MIT".to_string(),
                homepage: "https://nodejs.org".to_string(),
                categories: vec!["development".to_string()],
                icon: "nodejs".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 45000,
                installed_version: Some("20.15.0".to_string()),
                is_educational: false,
                is_essential: false,
                rating: 4.6,
                install_count: 180000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-20".to_string(),
            },
            AppEntry {
                id: "python3".to_string(),
                name: "Python 3".to_string(),
                description: "Interpreted programming language for scripting, automation, and data science".to_string(),
                version: "3.12.4".to_string(),
                author: "Python Software Foundation".to_string(),
                license: "PSF".to_string(),
                homepage: "https://www.python.org".to_string(),
                categories: vec!["development".to_string(), "education".to_string()],
                icon: "python3".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 35000,
                installed_version: Some("3.12.4".to_string()),
                is_educational: true,
                is_essential: true,
                rating: 4.8,
                install_count: 250000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-05-01".to_string(),
            },
            AppEntry {
                id: "gcc".to_string(),
                name: "GCC".to_string(),
                description: "GNU Compiler Collection for C, C++, and other languages".to_string(),
                version: "14.1.0".to_string(),
                author: "GNU Project".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://gcc.gnu.org".to_string(),
                categories: vec!["development".to_string()],
                icon: "gcc".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 80000,
                installed_version: Some("14.1.0".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.5,
                install_count: 200000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-04-10".to_string(),
            },
            AppEntry {
                id: "rust".to_string(),
                name: "Rust".to_string(),
                description: "Systems programming language focused on safety, speed, and concurrency".to_string(),
                version: "1.79.0".to_string(),
                author: "Mozilla Research".to_string(),
                license: "MIT/Apache-2.0".to_string(),
                homepage: "https://www.rust-lang.org".to_string(),
                categories: vec!["development".to_string()],
                icon: "rust".to_string(),
                source: PackageSource::System,
                status: PackageStatus::NotInstalled,
                size_kb: 60000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 4.9,
                install_count: 120000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-13".to_string(),
            },
            AppEntry {
                id: "neovim".to_string(),
                name: "Neovim".to_string(),
                description: "Hyperextensible Vim-based text editor with modern features".to_string(),
                version: "0.10.0".to_string(),
                author: "Neovim Community".to_string(),
                license: "Apache-2.0".to_string(),
                homepage: "https://neovim.io".to_string(),
                categories: vec!["development".to_string()],
                icon: "neovim".to_string(),
                source: PackageSource::System,
                status: PackageStatus::NotInstalled,
                size_kb: 8000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 4.7,
                install_count: 90000,
                created_at: "2024-02-01".to_string(),
                updated_at: "2025-05-20".to_string(),
            },
            AppEntry {
                id: "vim".to_string(),
                name: "Vim".to_string(),
                description: "Highly configurable modal text editor built for efficiency".to_string(),
                version: "9.1.0".to_string(),
                author: "Bram Moolenaar".to_string(),
                license: "Vim License".to_string(),
                homepage: "https://www.vim.org".to_string(),
                categories: vec!["development".to_string()],
                icon: "vim".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 5000,
                installed_version: Some("9.1.0".to_string()),
                is_educational: false,
                is_essential: false,
                rating: 4.6,
                install_count: 150000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-03-01".to_string(),
            },
            AppEntry {
                id: "docker".to_string(),
                name: "Docker".to_string(),
                description: "Container platform for building, shipping, and running applications".to_string(),
                version: "27.0.0".to_string(),
                author: "Docker Inc.".to_string(),
                license: "Apache-2.0".to_string(),
                homepage: "https://www.docker.com".to_string(),
                categories: vec!["development".to_string()],
                icon: "docker".to_string(),
                source: PackageSource::System,
                status: PackageStatus::NotInstalled,
                size_kb: 200000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 4.5,
                install_count: 160000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-25".to_string(),
            },
            AppEntry {
                id: "libreoffice-writer".to_string(),
                name: "LibreOffice Writer".to_string(),
                description: "Full-featured word processor for creating documents and reports".to_string(),
                version: "24.2.5".to_string(),
                author: "The Document Foundation".to_string(),
                license: "MPL-2.0".to_string(),
                homepage: "https://www.libreoffice.org".to_string(),
                categories: vec!["office".to_string()],
                icon: "libreoffice-writer".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 120000,
                installed_version: Some("24.2.5".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.3,
                install_count: 300000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-10".to_string(),
            },
            AppEntry {
                id: "libreoffice-calc".to_string(),
                name: "LibreOffice Calc".to_string(),
                description: "Spreadsheet application for data analysis and calculations".to_string(),
                version: "24.2.5".to_string(),
                author: "The Document Foundation".to_string(),
                license: "MPL-2.0".to_string(),
                homepage: "https://www.libreoffice.org".to_string(),
                categories: vec!["office".to_string()],
                icon: "libreoffice-calc".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 110000,
                installed_version: Some("24.2.5".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.2,
                install_count: 280000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-10".to_string(),
            },
            AppEntry {
                id: "libreoffice-impress".to_string(),
                name: "LibreOffice Impress".to_string(),
                description: "Presentation software for creating slideshows and lectures".to_string(),
                version: "24.2.5".to_string(),
                author: "The Document Foundation".to_string(),
                license: "MPL-2.0".to_string(),
                homepage: "https://www.libreoffice.org".to_string(),
                categories: vec!["office".to_string()],
                icon: "libreoffice-impress".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 100000,
                installed_version: Some("24.2.5".to_string()),
                is_educational: false,
                is_essential: false,
                rating: 4.1,
                install_count: 200000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-10".to_string(),
            },
            AppEntry {
                id: "libreoffice-draw".to_string(),
                name: "LibreOffice Draw".to_string(),
                description: "Vector graphics editor for diagrams and illustrations".to_string(),
                version: "24.2.5".to_string(),
                author: "The Document Foundation".to_string(),
                license: "MPL-2.0".to_string(),
                homepage: "https://www.libreoffice.org".to_string(),
                categories: vec!["office".to_string(), "graphics".to_string()],
                icon: "libreoffice-draw".to_string(),
                source: PackageSource::System,
                status: PackageStatus::NotInstalled,
                size_kb: 95000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 3.9,
                install_count: 150000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-10".to_string(),
            },
            AppEntry {
                id: "gimp".to_string(),
                name: "GIMP".to_string(),
                description: "GNU Image Manipulation Program for photo retouching and composition".to_string(),
                version: "2.10.38".to_string(),
                author: "GIMP Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://www.gimp.org".to_string(),
                categories: vec!["graphics".to_string()],
                icon: "gimp".to_string(),
                source: PackageSource::Flatpak,
                status: PackageStatus::Installed,
                size_kb: 200000,
                installed_version: Some("2.10.38".to_string()),
                is_educational: false,
                is_essential: false,
                rating: 4.4,
                install_count: 180000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-05-01".to_string(),
            },
            AppEntry {
                id: "inkscape".to_string(),
                name: "Inkscape".to_string(),
                description: "Professional vector graphics editor for SVG creation and editing".to_string(),
                version: "1.4".to_string(),
                author: "Inkscape Project".to_string(),
                license: "GPL-2.0".to_string(),
                homepage: "https://inkscape.org".to_string(),
                categories: vec!["graphics".to_string()],
                icon: "inkscape".to_string(),
                source: PackageSource::Flatpak,
                status: PackageStatus::NotInstalled,
                size_kb: 150000,
                installed_version: None,
                is_educational: true,
                is_essential: false,
                rating: 4.5,
                install_count: 140000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-04-15".to_string(),
            },
            AppEntry {
                id: "blender".to_string(),
                name: "Blender".to_string(),
                description: "3D creation suite for modeling, animation, and rendering".to_string(),
                version: "4.1.0".to_string(),
                author: "Blender Foundation".to_string(),
                license: "GPL-2.0".to_string(),
                homepage: "https://www.blender.org".to_string(),
                categories: vec!["graphics".to_string(), "multimedia".to_string()],
                icon: "blender".to_string(),
                source: PackageSource::Flatpak,
                status: PackageStatus::Installed,
                size_kb: 500000,
                installed_version: Some("4.1.0".to_string()),
                is_educational: true,
                is_essential: false,
                rating: 4.8,
                install_count: 160000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-01".to_string(),
            },
            AppEntry {
                id: "vlc".to_string(),
                name: "VLC".to_string(),
                description: "Multimedia player that plays most audio and video formats".to_string(),
                version: "3.0.21".to_string(),
                author: "VideoLAN".to_string(),
                license: "GPL-2.0".to_string(),
                homepage: "https://www.videolan.org".to_string(),
                categories: vec!["multimedia".to_string()],
                icon: "vlc".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 80000,
                installed_version: Some("3.0.21".to_string()),
                is_educational: false,
                is_essential: false,
                rating: 4.6,
                install_count: 250000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-03-01".to_string(),
            },
            AppEntry {
                id: "audacity".to_string(),
                name: "Audacity".to_string(),
                description: "Cross-platform audio editor for recording and editing sound".to_string(),
                version: "3.5.1".to_string(),
                author: "Audacity Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://www.audacityteam.org".to_string(),
                categories: vec!["multimedia".to_string()],
                icon: "audacity".to_string(),
                source: PackageSource::Flatpak,
                status: PackageStatus::NotInstalled,
                size_kb: 40000,
                installed_version: None,
                is_educational: true,
                is_essential: false,
                rating: 4.3,
                install_count: 120000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-04-01".to_string(),
            },
            AppEntry {
                id: "obs-studio".to_string(),
                name: "OBS Studio".to_string(),
                description: "Video recording and live streaming software".to_string(),
                version: "30.2.0".to_string(),
                author: "OBS Project".to_string(),
                license: "GPL-2.0".to_string(),
                homepage: "https://obsproject.com".to_string(),
                categories: vec!["multimedia".to_string()],
                icon: "obs-studio".to_string(),
                source: PackageSource::Flatpak,
                status: PackageStatus::NotInstalled,
                size_kb: 120000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 4.7,
                install_count: 130000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-01".to_string(),
            },
            AppEntry {
                id: "firefox".to_string(),
                name: "Firefox".to_string(),
                description: "Privacy-focused web browser from Mozilla".to_string(),
                version: "128.0".to_string(),
                author: "Mozilla Foundation".to_string(),
                license: "MPL-2.0".to_string(),
                homepage: "https://www.mozilla.org/firefox".to_string(),
                categories: vec!["internet".to_string()],
                icon: "firefox".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 250000,
                installed_version: Some("128.0".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.5,
                install_count: 400000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-07-01".to_string(),
            },
            AppEntry {
                id: "chromium".to_string(),
                name: "Chromium".to_string(),
                description: "Open-source web browser project behind Google Chrome".to_string(),
                version: "127.0".to_string(),
                author: "The Chromium Projects".to_string(),
                license: "BSD-3-Clause".to_string(),
                homepage: "https://www.chromium.org".to_string(),
                categories: vec!["internet".to_string()],
                icon: "chromium".to_string(),
                source: PackageSource::Snap,
                status: PackageStatus::NotInstalled,
                size_kb: 200000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 4.3,
                install_count: 180000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-20".to_string(),
            },
            AppEntry {
                id: "thunderbird".to_string(),
                name: "Thunderbird".to_string(),
                description: "Free email client with calendar, address book, and chat".to_string(),
                version: "128.0".to_string(),
                author: "Mozilla Foundation".to_string(),
                license: "MPL-2.0".to_string(),
                homepage: "https://www.thunderbird.net".to_string(),
                categories: vec!["internet".to_string(), "office".to_string()],
                icon: "thunderbird".to_string(),
                source: PackageSource::System,
                status: PackageStatus::NotInstalled,
                size_kb: 90000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 4.1,
                install_count: 100000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-06-15".to_string(),
            },
            AppEntry {
                id: "nemo".to_string(),
                name: "Nemo".to_string(),
                description: "File manager for Cinnamon desktop with dual-pane support".to_string(),
                version: "6.2.0".to_string(),
                author: "Linux Mint".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://github.com/linuxmint/nemo".to_string(),
                categories: vec!["system".to_string()],
                icon: "nemo".to_string(),
                source: PackageSource::System,
                status: PackageStatus::Installed,
                size_kb: 25000,
                installed_version: Some("6.2.0".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.4,
                install_count: 200000,
                created_at: "2024-01-01".to_string(),
                updated_at: "2025-04-01".to_string(),
            },
            AppEntry {
                id: "edushell-desktop".to_string(),
                name: "EduShell Desktop".to_string(),
                description: "Custom desktop shell with Indonesian education workflow integration".to_string(),
                version: "1.0.0".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id".to_string(),
                categories: vec!["system".to_string(), "education".to_string()],
                icon: "edushell-desktop".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 50000,
                installed_version: Some("1.0.0".to_string()),
                is_educational: true,
                is_essential: true,
                rating: 4.7,
                install_count: 60000,
                created_at: "2024-01-10".to_string(),
                updated_at: "2025-06-01".to_string(),
            },
            AppEntry {
                id: "edushell-core".to_string(),
                name: "EduShell Core".to_string(),
                description: "Core services and configuration for the EduShell ecosystem".to_string(),
                version: "1.0.0".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id".to_string(),
                categories: vec!["system".to_string()],
                icon: "edushell-core".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 30000,
                installed_version: Some("1.0.0".to_string()),
                is_educational: false,
                is_essential: true,
                rating: 4.6,
                install_count: 60000,
                created_at: "2024-01-10".to_string(),
                updated_at: "2025-05-01".to_string(),
            },
            AppEntry {
                id: "edushell-apps".to_string(),
                name: "EduShell Apps".to_string(),
                description: "Bundle of educational applications for the EduShell platform".to_string(),
                version: "1.0.0".to_string(),
                author: "EduShell Team".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://edushell.id".to_string(),
                categories: vec!["system".to_string(), "education".to_string()],
                icon: "edushell-apps".to_string(),
                source: PackageSource::EduShell,
                status: PackageStatus::Installed,
                size_kb: 40000,
                installed_version: Some("1.0.0".to_string()),
                is_educational: true,
                is_essential: true,
                rating: 4.5,
                install_count: 55000,
                created_at: "2024-01-10".to_string(),
                updated_at: "2025-06-15".to_string(),
            },
            AppEntry {
                id: "nemo-appimage".to_string(),
                name: "Nemo AppImage".to_string(),
                description: "Portable AppImage version of Nemo file manager".to_string(),
                version: "6.2.0".to_string(),
                author: "Community".to_string(),
                license: "GPL-3.0".to_string(),
                homepage: "https://appimage.github.io".to_string(),
                categories: vec!["system".to_string()],
                icon: "nemo-appimage".to_string(),
                source: PackageSource::AppImage,
                status: PackageStatus::NotInstalled,
                size_kb: 45000,
                installed_version: None,
                is_educational: false,
                is_essential: false,
                rating: 3.8,
                install_count: 10000,
                created_at: "2024-06-01".to_string(),
                updated_at: "2025-05-01".to_string(),
            },
        ]
    }

    fn dummy_updates() -> Vec<UpdateInfo> {
        vec![
            UpdateInfo {
                app_id: "vscode".to_string(),
                app_name: "Visual Studio Code".to_string(),
                current_version: "1.91.0".to_string(),
                new_version: "1.92.0".to_string(),
                size_kb: 80000,
                importance: "recommended".to_string(),
                changelog: Some("Performance improvements and new extensions API".to_string()),
                released_at: "2025-07-01".to_string(),
            },
        ]
    }

    fn dummy_repos() -> Vec<RepositoryInfo> {
        vec![
            RepositoryInfo {
                id: "edushell-main".to_string(),
                name: "EduShell Main".to_string(),
                url: "https://repo.edushell.id/main".to_string(),
                enabled: true,
                source: PackageSource::EduShell,
                last_updated: Some("2025-07-01".to_string()),
                app_count: 6,
            },
            RepositoryInfo {
                id: "flatpak-flathub".to_string(),
                name: "Flathub".to_string(),
                url: "https://flathub.org/repo".to_string(),
                enabled: true,
                source: PackageSource::Flatpak,
                last_updated: Some("2025-07-05".to_string()),
                app_count: 5,
            },
            RepositoryInfo {
                id: "snap-store".to_string(),
                name: "Snap Store".to_string(),
                url: "https://snapcraft.io".to_string(),
                enabled: true,
                source: PackageSource::Snap,
                last_updated: Some("2025-06-28".to_string()),
                app_count: 1,
            },
        ]
    }
}

impl Default for SoftwareCenter {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_center() -> SoftwareCenter {
        SoftwareCenter::new(true)
    }

    fn create_empty_center() -> SoftwareCenter {
        SoftwareCenter::new(false)
    }

    #[test]
    fn test_creation_with_dummy_data() {
        let center = create_center();
        assert!(center.use_dummy_data);
        assert!(!center.available_apps.is_empty());
        assert!(!center.installed_apps.is_empty());
    }

    #[test]
    fn test_creation_without_dummy_data() {
        let center = create_empty_center();
        assert!(!center.use_dummy_data);
        assert!(center.available_apps.is_empty());
        assert!(center.installed_apps.is_empty());
    }

    #[test]
    fn test_dummy_data_has_thirty_plus_apps() {
        let center = create_center();
        assert!(
            center.available_apps.len() >= 30,
            "expected 30+ apps, got {}",
            center.available_apps.len()
        );
    }

    #[test]
    fn test_installed_apps_returns_correct_slice() {
        let center = create_center();
        let installed = center.installed_apps();
        for app in installed {
            assert!(
                app.status == PackageStatus::Installed || app.status == PackageStatus::UpdateAvailable
            );
        }
    }

    #[test]
    fn test_available_apps_returns_correct_slice() {
        let center = create_center();
        assert!(!center.available_apps().is_empty());
    }

    #[test]
    fn test_all_apps_deduplication() {
        let center = create_center();
        let all = center.all_apps();
        let mut ids: Vec<&str> = all.iter().map(|a| a.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), all.len(), "all_apps should have no duplicates");
    }

    #[test]
    fn test_get_app_existing() {
        let center = create_center();
        let app = center.get_app("git");
        assert!(app.is_some());
        assert_eq!(app.unwrap().name, "Git");
    }

    #[test]
    fn test_get_app_nonexistent() {
        let center = create_center();
        assert!(center.get_app("totally-fake-app").is_none());
    }

    #[test]
    fn test_get_app_mut_existing() {
        let mut center = create_center();
        {
            let app = center.get_app_mut("git");
            assert!(app.is_some());
            app.unwrap().rating = 5.0;
        }
        assert_eq!(center.get_app("git").unwrap().rating, 5.0);
    }

    #[test]
    fn test_get_app_mut_nonexistent() {
        let mut center = create_center();
        assert!(center.get_app_mut("nonexistent").is_none());
    }

    #[test]
    fn test_search_by_name() {
        let center = create_center();
        let results = center.search("Git");
        assert!(!results.is_empty());
        assert!(results.iter().any(|a| a.id == "git"));
    }

    #[test]
    fn test_search_by_description() {
        let center = create_center();
        let results = center.search("container");
        assert!(!results.is_empty());
        assert!(results.iter().any(|a| a.id == "docker"));
    }

    #[test]
    fn test_search_by_category() {
        let center = create_center();
        let results = center.search("multimedia");
        assert!(!results.is_empty());
        assert!(results.iter().any(|a| a.id == "vlc"));
    }

    #[test]
    fn test_search_case_insensitive() {
        let center = create_center();
        let results_lower = center.search("python");
        let results_upper = center.search("PYTHON");
        assert_eq!(results_lower.len(), results_upper.len());
    }

    #[test]
    fn test_search_no_results() {
        let center = create_center();
        let results = center.search("zzz-nonexistent-app-zzz");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_installed() {
        let center = create_center();
        let results = center.search_installed("Git");
        assert!(!results.is_empty());
        assert!(results.iter().all(|a|
            a.status == PackageStatus::Installed || a.status == PackageStatus::UpdateAvailable
        ));
    }

    #[test]
    fn test_search_installed_no_results() {
        let center = create_center();
        let results = center.search_installed("zzz-nonexistent-zzz");
        assert!(results.is_empty());
    }

    #[test]
    fn test_by_category() {
        let center = create_center();
        let dev_apps = center.by_category("development");
        assert!(!dev_apps.is_empty());
        assert!(dev_apps.iter().any(|a| a.id == "git"));
        assert!(dev_apps.iter().any(|a| a.id == "gcc"));
    }

    #[test]
    fn test_by_category_case_insensitive() {
        let center = create_center();
        let lower = center.by_category("development");
        let upper = center.by_category("DEVELOPMENT");
        assert_eq!(lower.len(), upper.len());
    }

    #[test]
    fn test_by_category_no_results() {
        let center = create_center();
        let results = center.by_category("nonexistent-category");
        assert!(results.is_empty());
    }

    #[test]
    fn test_by_source() {
        let center = create_center();
        let edushell = center.by_source(PackageSource::EduShell);
        assert!(!edushell.is_empty());
        assert!(edushell.iter().all(|a| a.source == PackageSource::EduShell));
    }

    #[test]
    fn test_by_source_flatpak() {
        let center = create_center();
        let flatpak = center.by_source(PackageSource::Flatpak);
        assert!(!flatpak.is_empty());
        assert!(flatpak.iter().all(|a| a.source == PackageSource::Flatpak));
    }

    #[test]
    fn test_by_source_snap() {
        let center = create_center();
        let snap = center.by_source(PackageSource::Snap);
        assert!(!snap.is_empty());
        assert!(snap.iter().all(|a| a.source == PackageSource::Snap));
    }

    #[test]
    fn test_by_source_appimage() {
        let center = create_center();
        let appimage = center.by_source(PackageSource::AppImage);
        assert!(!appimage.is_empty());
    }

    #[test]
    fn test_educational_apps() {
        let center = create_center();
        let edu = center.educational_apps();
        assert!(!edu.is_empty());
        assert!(edu.iter().all(|a| a.is_educational));
        assert!(edu.iter().any(|a| a.id == "learning-hub"));
        assert!(edu.iter().any(|a| a.id == "python3"));
    }

    #[test]
    fn test_essential_apps() {
        let center = create_center();
        let essential = center.essential_apps();
        assert!(!essential.is_empty());
        assert!(essential.iter().all(|a| a.is_essential));
        assert!(essential.iter().any(|a| a.id == "git"));
        assert!(essential.iter().any(|a| a.id == "edushell-desktop"));
    }

    #[test]
    fn test_updates_available() {
        let center = create_center();
        let updates = center.updates_available();
        assert!(!updates.is_empty());
    }

    #[test]
    fn test_updates_count() {
        let center = create_center();
        assert_eq!(center.updates_count(), center.updates.len() as u32);
    }

    #[test]
    fn test_security_updates() {
        let center = create_center();
        let security = center.security_updates();
        for u in &security {
            assert_eq!(u.importance, "security");
        }
    }

    #[test]
    fn test_check_for_updates() {
        let mut center = create_center();
        center.updates.clear();
        center.check_for_updates().unwrap();
        assert!(!center.updates.is_empty());
        let vscode_update = center.updates.iter().find(|u| u.app_id == "vscode");
        assert!(vscode_update.is_some());
    }

    #[test]
    fn test_install_lifecycle() {
        let mut center = create_center();
        assert_eq!(
            center.get_app("rust").unwrap().status,
            PackageStatus::NotInstalled
        );
        center.install("rust").unwrap();
        let app = center.get_app("rust").unwrap();
        assert_eq!(app.status, PackageStatus::Installed);
        assert!(app.installed_version.is_some());
    }

    #[test]
    fn test_install_already_installed() {
        let mut center = create_center();
        let result = center.install("git");
        assert!(result.is_err());
        match result.unwrap_err() {
            SoftwareError::AlreadyInstalled(id) => assert_eq!(id, "git"),
            _ => panic!("expected AlreadyInstalled error"),
        }
    }

    #[test]
    fn test_install_nonexistent() {
        let mut center = create_center();
        let result = center.install("nonexistent");
        assert!(result.is_err());
        match result.unwrap_err() {
            SoftwareError::NotFound(id) => assert_eq!(id, "nonexistent"),
            _ => panic!("expected NotFound error"),
        }
    }

    #[test]
    fn test_uninstall_lifecycle() {
        let mut center = create_center();
        assert!(center.installed_apps.iter().any(|a| a.id == "gcc"));
        center.uninstall("gcc").unwrap();
        let app = center.get_app("gcc").unwrap();
        assert_eq!(app.status, PackageStatus::NotInstalled);
        assert!(app.installed_version.is_none());
        assert!(!center.installed_apps.iter().any(|a| a.id == "gcc"));
    }

    #[test]
    fn test_uninstall_not_installed() {
        let mut center = create_center();
        let result = center.uninstall("rust");
        assert!(result.is_err());
        match result.unwrap_err() {
            SoftwareError::NotInstalled(id) => assert_eq!(id, "rust"),
            _ => panic!("expected NotInstalled error"),
        }
    }

    #[test]
    fn test_uninstall_nonexistent() {
        let mut center = create_center();
        let result = center.uninstall("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_app() {
        let mut center = create_center();
        let old_ver = center.get_app("vscode").unwrap().installed_version.clone().unwrap();
        center.update_app("vscode").unwrap();
        let app = center.get_app("vscode").unwrap();
        assert_eq!(app.status, PackageStatus::Installed);
        assert_ne!(app.installed_version.as_ref().unwrap(), &old_ver);
        assert!(center.updates.iter().all(|u| u.app_id != "vscode"));
    }

    #[test]
    fn test_update_app_no_update_available() {
        let mut center = create_center();
        let result = center.update_app("git");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_all() {
        let mut center = create_center();
        let count = center.updates_count();
        assert!(count > 0);
        center.update_all().unwrap();
        assert_eq!(center.updates_count(), 0);
    }

    #[test]
    fn test_categories() {
        let center = create_center();
        let cats = center.categories();
        assert!(!cats.is_empty());
        assert!(cats.contains(&"development".to_string()));
        assert!(cats.contains(&"education".to_string()));
        assert!(cats.contains(&"office".to_string()));
        assert!(cats.contains(&"multimedia".to_string()));
        assert!(cats.contains(&"graphics".to_string()));
        assert!(cats.contains(&"internet".to_string()));
        assert!(cats.contains(&"system".to_string()));
        let mut sorted = cats.clone();
        sorted.sort();
        assert_eq!(cats, sorted, "categories should be sorted");
    }

    #[test]
    fn test_stats() {
        let center = create_center();
        let stats = center.stats();
        assert!(stats.total_installed > 0);
        assert!(stats.total_educational > 0);
        assert!(stats.total_essential > 0);
        assert!(stats.updates_available > 0);
        assert!(stats.total_size_kb > 0);
        assert!(!stats.by_source.is_empty());
        assert!(!stats.by_category.is_empty());
    }

    #[test]
    fn test_stats_total_installed_matches() {
        let center = create_center();
        let stats = center.stats();
        assert_eq!(stats.total_installed, center.installed_apps().len() as u32);
    }

    #[test]
    fn test_stats_sum_size_matches() {
        let center = create_center();
        let stats = center.stats();
        let sum: u64 = center.installed_apps().iter().map(|a| a.size_kb).sum();
        assert_eq!(stats.total_size_kb, sum);
    }

    #[test]
    fn test_source_name_indonesian() {
        let mut center = create_center();
        center.set_locale("id-ID");
        let name = center.source_name(PackageSource::System);
        assert_eq!(name, "Sistem");
    }

    #[test]
    fn test_source_name_english() {
        let mut center = create_center();
        center.set_locale("en-US");
        let name = center.source_name(PackageSource::System);
        assert_eq!(name, "System");
    }

    #[test]
    fn test_source_name_flatpak_english() {
        let mut center = create_center();
        center.set_locale("en-US");
        assert_eq!(center.source_name(PackageSource::Flatpak), "Flatpak");
    }

    #[test]
    fn test_set_locale_switches_language() {
        let mut center = create_center();
        center.set_locale("en-US");
        assert_eq!(center.localization.current_language(), Lang::English);
        center.set_locale("id-ID");
        assert_eq!(center.localization.current_language(), Lang::Indonesian);
    }

    #[test]
    fn test_set_locale_invalid_falls_through() {
        let mut center = create_center();
        let before = center.localization.current_language();
        center.set_locale("fr-FR");
        assert_eq!(center.localization.current_language(), before);
    }

    #[test]
    fn test_repos() {
        let center = create_center();
        let repos = center.repos();
        assert!(!repos.is_empty());
        assert!(repos.iter().any(|r| r.id == "edushell-main"));
        assert!(repos.iter().all(|r| r.enabled));
    }

    #[test]
    fn test_repos_includes_sources() {
        let center = create_center();
        let repos = center.repos();
        let sources: Vec<PackageSource> = repos.iter().map(|r| r.source).collect();
        assert!(sources.contains(&PackageSource::EduShell));
        assert!(sources.contains(&PackageSource::Flatpak));
        assert!(sources.contains(&PackageSource::Snap));
    }

    #[test]
    fn test_empty_center_has_nothing() {
        let center = create_empty_center();
        assert!(center.installed_apps().is_empty());
        assert!(center.available_apps().is_empty());
        assert!(center.all_apps().is_empty());
        assert!(center.updates_available().is_empty());
        assert_eq!(center.updates_count(), 0);
        assert!(center.categories().is_empty());
        assert!(center.repos().is_empty());
    }

    #[test]
    fn test_essential_app_presence() {
        let center = create_center();
        let essential_ids = vec![
            "learning-hub", "edu-terminal", "office-hub", "git", "python3",
            "gcc", "libreoffice-writer", "libreoffice-calc", "firefox",
            "nemo", "welcome-app", "edushell-desktop", "edushell-core", "edushell-apps",
        ];
        for id in essential_ids {
            let app = center.get_app(id);
            assert!(app.is_some(), "{} should exist", id);
            assert!(app.unwrap().is_essential, "{} should be essential", id);
        }
    }

    #[test]
    fn test_app_entry_fields_populated() {
        let center = create_center();
        for app in &center.available_apps {
            assert!(!app.id.is_empty());
            assert!(!app.name.is_empty());
            assert!(!app.description.is_empty());
            assert!(!app.version.is_empty());
            assert!(!app.author.is_empty());
            assert!(!app.license.is_empty());
            assert!(!app.homepage.is_empty());
            assert!(!app.categories.is_empty());
            assert!(!app.icon.is_empty());
            assert!(app.rating >= 0.0 && app.rating <= 5.0);
            assert!(!app.created_at.is_empty());
            assert!(!app.updated_at.is_empty());
        }
    }

    #[test]
    fn test_install_adds_to_installed_list() {
        let mut center = create_center();
        let before = center.installed_apps.len();
        center.install("rust").unwrap();
        assert_eq!(center.installed_apps.len(), before + 1);
    }

    #[test]
    fn test_uninstall_removes_from_installed_list() {
        let mut center = create_center();
        let before = center.installed_apps.len();
        center.uninstall("gcc").unwrap();
        assert_eq!(center.installed_apps.len(), before - 1);
    }

    #[test]
    fn test_uninstall_removes_update_if_present() {
        let mut center = create_center();
        assert!(center.updates.iter().any(|u| u.app_id == "vscode"));
        center.uninstall("vscode").unwrap();
        assert!(!center.updates.iter().any(|u| u.app_id == "vscode"));
    }

    #[test]
    fn test_default_is_dummy() {
        let center = SoftwareCenter::default();
        assert!(center.use_dummy_data);
        assert!(!center.available_apps.is_empty());
    }

    #[test]
    fn test_package_source_variants() {
        let sources = vec![
            PackageSource::System,
            PackageSource::Flatpak,
            PackageSource::Snap,
            PackageSource::AppImage,
            PackageSource::EduShell,
            PackageSource::Unknown,
        ];
        for s in &sources {
            let _ = format!("{:?}", s);
        }
        assert_eq!(sources.len(), 6);
    }

    #[test]
    fn test_package_status_variants() {
        let statuses = vec![
            PackageStatus::Installed,
            PackageStatus::UpdateAvailable,
            PackageStatus::NotInstalled,
            PackageStatus::Installing,
            PackageStatus::Failed,
        ];
        assert_eq!(statuses.len(), 5);
    }

    #[test]
    fn test_software_error_display() {
        let err = SoftwareError::NotFound("test".to_string());
        assert!(format!("{}", err).contains("test"));

        let err = SoftwareError::AlreadyInstalled("test".to_string());
        assert!(format!("{}", err).contains("test"));

        let err = SoftwareError::NotInstalled("test".to_string());
        assert!(format!("{}", err).contains("test"));

        let err = SoftwareError::InstallFailed("test".to_string());
        assert!(format!("{}", err).contains("test"));

        let err = SoftwareError::UninstallFailed("test".to_string());
        assert!(format!("{}", err).contains("test"));

        let err = SoftwareError::UpdateFailed("test".to_string());
        assert!(format!("{}", err).contains("test"));

        let err = SoftwareError::NoUpdatesAvailable;
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_software_error_is_error() {
        let err: Box<dyn std::error::Error> = Box::new(SoftwareError::NotFound("x".to_string()));
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn test_all_apps_empty_with_dummy_false() {
        let center = create_empty_center();
        assert!(center.all_apps().is_empty());
    }

    #[test]
    fn test_update_all_no_updates() {
        let mut center = create_center();
        center.update_all().unwrap();
        let result = center.update_all();
        assert!(result.is_ok());
        assert_eq!(center.updates_count(), 0);
    }

    #[test]
    fn test_edushell_source_apps() {
        let center = create_center();
        let edu = center.by_source(PackageSource::EduShell);
        assert!(edu.iter().any(|a| a.id == "learning-hub"));
        assert!(edu.iter().any(|a| a.id == "edushell-desktop"));
        assert!(edu.iter().any(|a| a.id == "edushell-core"));
        assert!(edu.iter().any(|a| a.id == "edushell-apps"));
    }

    #[test]
    fn test_system_source_apps() {
        let center = create_center();
        let system = center.by_source(PackageSource::System);
        assert!(system.iter().any(|a| a.id == "git"));
        assert!(system.iter().any(|a| a.id == "gcc"));
        assert!(system.iter().any(|a| a.id == "firefox"));
    }

    #[test]
    fn test_reinstall_after_uninstall() {
        let mut center = create_center();
        center.uninstall("git").unwrap();
        assert_eq!(center.get_app("git").unwrap().status, PackageStatus::NotInstalled);
        center.install("git").unwrap();
        assert_eq!(center.get_app("git").unwrap().status, PackageStatus::Installed);
        assert!(center.get_app("git").unwrap().installed_version.is_some());
    }

    #[test]
    fn test_search_in_installed_only_shows_installed() {
        let mut center = create_center();
        let results = center.search_installed("rust");
        assert!(results.is_empty());
        center.install("rust").unwrap();
        let results = center.search_installed("rust");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_stats_by_source_keys() {
        let center = create_center();
        let stats = center.stats();
        let keys: Vec<&String> = stats.by_source.keys().collect();
        assert!(keys.iter().any(|k| k.as_str() == "EduShell"));
        assert!(keys.iter().any(|k| k.as_str() == "System"));
    }

    #[test]
    fn test_categories_sorted_unique() {
        let center = create_center();
        let cats = center.categories();
        for window in cats.windows(2) {
            assert!(window[0] <= window[1], "categories should be sorted");
        }
        let mut deduped = cats.clone();
        deduped.dedup();
        assert_eq!(cats.len(), deduped.len(), "categories should have no duplicates");
    }
}
