use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconTheme {
    pub name: String,
    pub author: String,
    pub version: String,
    pub license: String,
}

impl Default for IconTheme {
    fn default() -> Self {
        Self {
            name: "EduShell Icons".to_string(),
            author: "EduShell Team".to_string(),
            version: "1.0.0".to_string(),
            license: "GPL-3.0-or-later".to_string(),
        }
    }
}

pub struct IconPack {
    icons: HashMap<&'static str, &'static str>,
}

impl IconPack {
    fn default_icons() -> Self {
        let mut icons: HashMap<&'static str, &'static str> = HashMap::new();
        for (name, svg) in all_icons() {
            icons.insert(name, svg);
        }
        Self { icons }
    }

    pub fn get(&self, name: &str) -> Option<&'static str> {
        self.icons.get(name).copied()
    }

    pub fn names(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.icons.keys().copied().collect();
        keys.sort();
        keys
    }
}

pub struct IconManager {
    theme: IconTheme,
    icons: IconPack,
}

impl IconManager {
    pub fn new() -> Self {
        Self {
            theme: IconTheme::default(),
            icons: IconPack::default_icons(),
        }
    }

    pub fn icon_path(&self, name: &str, _size: u32) -> Option<String> {
        self.icons.get(name).map(|_| {
            format!(
                "/usr/share/icons/edushell/{}/apps/{}.svg",
                if _size <= 24 { "16x16" } else { " scalable" }.trim(),
                name
            )
        })
    }

    pub fn app_icon(&self, app_id: &str) -> Option<String> {
        self.icon_path(app_id, 48)
    }

    pub fn category_icon(&self, category: &str) -> Option<String> {
        let mapped = match category {
            "linux" | "terminal" | "bash" => "terminal-icon",
            "git" | "github" => "github",
            "html" | "css" | "javascript" => "javascript",
            "python" => "python",
            "c" | "cpp" => "cpp",
            "rust" => "rust",
            "open-source" => "open-source",
            "command-line" => "terminal-icon",
            "package-manager" => "software-center",
            "filesystem" => "file-manager",
            "networking" => "networking",
            "security" => "security",
            "office" => "office-hub",
            "internet" => "browser-hub",
            "ai" => "ai",
            "vscode" => "vscode",
            "libreoffice" => "libreoffice",
            "markdown" => "markdown",
            "license" => "license",
            "contribution" => "contribution",
            _ => "edushell",
        };
        self.icon_path(mapped, 48)
    }

    pub fn available_icons(&self) -> Vec<&str> {
        self.icons.names()
    }

    pub fn theme(&self) -> &IconTheme {
        &self.theme
    }
}

impl Default for IconManager {
    fn default() -> Self {
        Self::new()
    }
}

fn all_icons() -> Vec<(&'static str, &'static str)> {
    vec![
        ("edushell", SVG_EDUSHELL),
        ("edushell-symbolic", SVG_EDUSHELL_SYMBOLIC),
        ("welcome", SVG_WELCOME),
        ("learning-hub", SVG_LEARNING_HUB),
        ("edu-terminal", SVG_EDU_TERMINAL),
        ("project-hub", SVG_PROJECT_HUB),
        ("office-hub", SVG_OFFICE_HUB),
        ("browser-hub", SVG_BROWSER_HUB),
        ("settings-center", SVG_SETTINGS_CENTER),
        ("file-manager", SVG_FILE_MANAGER),
        ("software-center", SVG_SOFTWARE_CENTER),
        ("search", SVG_SEARCH),
        ("global-search", SVG_GLOBAL_SEARCH),
        ("linux", SVG_LINUX),
        ("terminal-icon", SVG_TERMINAL_ICON),
        ("bash", SVG_BASH),
        ("git", SVG_GIT),
        ("html", SVG_HTML),
        ("css", SVG_CSS),
        ("javascript", SVG_JAVASCRIPT),
        ("python", SVG_PYTHON),
        ("c", SVG_C),
        ("cpp", SVG_CPP),
        ("rust", SVG_RUST),
        ("open-source", SVG_OPEN_SOURCE),
        ("command-line", SVG_COMMAND_LINE),
        ("package-manager", SVG_PACKAGE_MANAGER),
        ("filesystem", SVG_FILESYSTEM),
        ("networking", SVG_NETWORKING),
        ("security", SVG_SECURITY),
        ("office", SVG_OFFICE),
        ("internet", SVG_INTERNET),
        ("ai", SVG_AI),
        ("github", SVG_GITHUB),
        ("vscode", SVG_VSCODE),
        ("libreoffice", SVG_LIBREOFFICE),
        ("markdown", SVG_MARKDOWN),
        ("license", SVG_LICENSE),
        ("contribution", SVG_CONTRIBUTION),
        ("folder", SVG_FOLDER),
        ("document", SVG_DOCUMENT),
        ("image", SVG_IMAGE),
        ("video", SVG_VIDEO),
        ("music", SVG_MUSIC),
        ("archive", SVG_ARCHIVE),
        ("download", SVG_DOWNLOAD),
        ("upload", SVG_UPLOAD),
        ("user", SVG_USER),
        ("settings", SVG_SETTINGS),
        ("info", SVG_INFO),
        ("warning", SVG_WARNING),
        ("error", SVG_ERROR),
        ("check", SVG_CHECK),
        ("close", SVG_CLOSE),
        ("menu", SVG_MENU),
        ("back", SVG_BACK),
        ("forward", SVG_FORWARD),
        ("search-icon", SVG_SEARCH_ICON),
        ("home", SVG_HOME),
        ("star", SVG_STAR),
        ("heart", SVG_HEART),
        ("share", SVG_SHARE),
        ("clock", SVG_CLOCK),
        ("calendar", SVG_CALENDAR),
        ("notification", SVG_NOTIFICATION),
        ("power", SVG_POWER),
        ("lock", SVG_LOCK),
    ]
}

const SVG_EDUSHELL: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><text x="64" y="80" font-size="64" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">E</text></svg>"##;

const SVG_EDUSHELL_SYMBOLIC: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><text x="64" y="80" font-size="64" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">e</text></svg>"##;

const SVG_WELCOME: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#33d17a"/><path d="M40 96V64l24-16 24 16v32H72V80H56v16z" fill="white"/></svg>"##;

const SVG_LEARNING_HUB: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#ffa348"/><path d="M64 32L32 48v32l32 16 32-16V48zM40 60l24 12 24-12" fill="none" stroke="white" stroke-width="4" stroke-linejoin="round"/></svg>"##;

const SVG_EDU_TERMINAL: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#1a1a2e"/><path d="M36 48l16 16-16 16" fill="none" stroke="#33d17a" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M60 80h28" fill="none" stroke="#33d17a" stroke-width="4" stroke-linecap="round"/></svg>"##;

const SVG_PROJECT_HUB: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#c061cb"/><path d="M36 40h56v8H36zm0 16h40v8H36zm0 16h48v8H36zm0 16h32v8H36z" fill="white"/></svg>"##;

const SVG_OFFICE_HUB: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><path d="M40 32h48v64H40zm8 8v48h32V40z" fill="white"/><path d="M52 52h24v4H52zm0 12h24v4H52zm0 12h16v4H52z" fill="#e66100"/></svg>"##;

const SVG_BROWSER_HUB: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><circle cx="64" cy="64" r="28" fill="none" stroke="white" stroke-width="4"/><path d="M40 64h48M64 36c-8 0-16 12-16 28s8 28 16 28 16-12 16-28-8-28-16-28z" fill="none" stroke="white" stroke-width="3"/><path d="M36 64c0-16 12-28 28-28" fill="none" stroke="white" stroke-width="3"/></svg>"##;

const SVG_SETTINGS_CENTER: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><circle cx="64" cy="64" r="12" fill="none" stroke="white" stroke-width="4"/><path d="M64 34v8m0 44v8M44 44l6 6m28 28l6 6M34 64h8m44 0h8M44 84l6-6m28-28l6-6" stroke="white" stroke-width="3" stroke-linecap="round"/></svg>"##;

const SVG_FILE_MANAGER: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#f5c211"/><path d="M32 96V40h24l8 8h32v48z" fill="white"/><path d="M36 44h20l8 8h28" fill="none" stroke="#f5c211" stroke-width="2"/></svg>"##;

const SVG_SOFTWARE_CENTER: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#33d17a"/><rect x="36" y="36" width="56" height="56" rx="8" fill="white"/><rect x="44" y="44" width="16" height="16" rx="4" fill="#33d17a"/><rect x="68" y="44" width="16" height="16" rx="4" fill="#33d17a"/><rect x="44" y="68" width="16" height="16" rx="4" fill="#33d17a"/><rect x="68" y="68" width="16" height="16" rx="4" fill="#33d17a"/></svg>"##;

const SVG_SEARCH: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><circle cx="52" cy="52" r="20" fill="none" stroke="white" stroke-width="5"/><path d="M68 68l20 20" stroke="white" stroke-width="5" stroke-linecap="round"/></svg>"##;

const SVG_GLOBAL_SEARCH: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#c061cb"/><circle cx="52" cy="52" r="20" fill="none" stroke="white" stroke-width="5"/><path d="M68 68l20 20" stroke="white" stroke-width="5" stroke-linecap="round"/><path d="M24 24l80 80" stroke="white" stroke-width="3" stroke-linecap="round" opacity="0.5"/></svg>"##;

const SVG_LINUX: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#f5c211"/><circle cx="64" cy="40" r="12" fill="white"/><path d="M48 80c0-12 8-24 16-24s16 12 16 24v16H48z" fill="white"/></svg>"##;

const SVG_TERMINAL_ICON: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#1a1a2e"/><path d="M38 48l16 16-16 16" fill="none" stroke="#33d17a" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M62 80h28" fill="none" stroke="#33d17a" stroke-width="4" stroke-linecap="round"/></svg>"##;

const SVG_BASH: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#2d2d2d"/><text x="64" y="76" font-size="44" text-anchor="middle" fill="#33d17a" font-family="monospace" font-weight="bold">$</text></svg>"##;

const SVG_GIT: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e94e31"/><path d="M64 24L24 64l40 40 40-40z" fill="none" stroke="white" stroke-width="5"/><circle cx="64" cy="64" r="8" fill="white"/></svg>"##;

const SVG_HTML: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e44d26"/><text x="64" y="76" font-size="44" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">&lt;/&gt;</text></svg>"##;

const SVG_CSS: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#264de4"/><text x="64" y="76" font-size="44" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">CSS</text></svg>"##;

const SVG_JAVASCRIPT: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#f7df1e"/><text x="64" y="76" font-size="40" text-anchor="middle" fill="#2d2d2d" font-family="sans-serif" font-weight="bold">JS</text></svg>"##;

const SVG_PYTHON: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#306998"/><path d="M48 44h32v40H48z" fill="#ffd43b"/><path d="M44 52h40v24H44z" fill="#306998"/></svg>"##;

const SVG_C: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#555555"/><text x="64" y="76" font-size="44" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">C</text></svg>"##;

const SVG_CPP: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#00599d"/><text x="64" y="76" font-size="36" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">C++</text></svg>"##;

const SVG_RUST: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#dea584"/><text x="64" y="76" font-size="40" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">Rust</text></svg>"##;

const SVG_OPEN_SOURCE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3da639"/><circle cx="64" cy="64" r="24" fill="none" stroke="white" stroke-width="4"/><path d="M52 80l12-32 12 32" fill="none" stroke="white" stroke-width="3" stroke-linecap="round"/></svg>"##;

const SVG_COMMAND_LINE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#1a1a2e"/><path d="M34 48l16 16-16 16" fill="none" stroke="white" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M60 80h34" fill="none" stroke="white" stroke-width="4" stroke-linecap="round"/></svg>"##;

const SVG_PACKAGE_MANAGER: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><rect x="36" y="40" width="56" height="48" rx="6" fill="white"/><rect x="44" y="48" width="40" height="6" rx="2" fill="#e66100"/><rect x="44" y="60" width="40" height="6" rx="2" fill="#e66100"/><rect x="44" y="72" width="24" height="6" rx="2" fill="#e66100"/></svg>"##;

const SVG_FILESYSTEM: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><rect x="32" y="52" width="64" height="40" rx="4" fill="white"/><rect x="36" y="56" width="56" height="6" fill="#3584e4"/><rect x="36" y="66" width="40" height="6" fill="#3584e4"/><rect x="36" y="76" width="28" height="6" fill="#3584e4"/></svg>"##;

const SVG_NETWORKING: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><circle cx="64" cy="64" r="8" fill="white"/><path d="M44 44c12-12 28-12 40 0M34 34c16-16 28-16 44 0" fill="none" stroke="white" stroke-width="3" stroke-linecap="round"/><path d="M44 84c12 12 28 12 40 0M34 94c16 16 28 16 44 0" fill="none" stroke="white" stroke-width="3" stroke-linecap="round"/></svg>"##;

const SVG_SECURITY: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#33d17a"/><path d="M64 28L36 40v24c0 20 12 32 28 36 16-4 28-16 28-36V40z" fill="none" stroke="white" stroke-width="5"/><path d="M54 64l8 8 12-16" fill="none" stroke="white" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>"##;

const SVG_OFFICE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><rect x="36" y="36" width="56" height="56" rx="4" fill="white"/><line x1="36" y1="52" x2="92" y2="52" stroke="#e66100" stroke-width="2"/><line x1="36" y1="64" x2="92" y2="64" stroke="#e66100" stroke-width="2"/><line x1="36" y1="76" x2="76" y2="76" stroke="#e66100" stroke-width="2"/></svg>"##;

const SVG_INTERNET: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><circle cx="64" cy="64" r="28" fill="none" stroke="white" stroke-width="4"/><path d="M40 64h48M64 36c-8 0-16 12-16 28s8 28 16 28 16-12 16-28-8-28-16-28z" fill="none" stroke="white" stroke-width="3"/></svg>"##;

const SVG_AI: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#c061cb"/><circle cx="64" cy="48" r="12" fill="white"/><path d="M44 72c0-12 8-24 20-24s20 12 20 24v8H44z" fill="white"/><path d="M36 96c0-8 6-16 14-16h28c8 0 14 8 14 16" fill="none" stroke="white" stroke-width="3"/></svg>"##;

const SVG_GITHUB: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#1a1a2e"/><circle cx="64" cy="56" r="20" fill="white"/><path d="M56 76v16c0 4 8 4 8 0V76" fill="none" stroke="#1a1a2e" stroke-width="2"/><path d="M72 76v12" fill="none" stroke="#1a1a2e" stroke-width="2"/><path d="M56 86c-8 2-12-2-16-4" fill="none" stroke="#1a1a2e" stroke-width="2"/></svg>"##;

const SVG_VSCODE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#007acc"/><rect x="36" y="36" width="56" height="56" rx="6" fill="white"/><path d="M44 48l12 16-12 16" fill="none" stroke="#007acc" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/><path d="M60 80h20" fill="none" stroke="#007acc" stroke-width="3" stroke-linecap="round"/></svg>"##;

const SVG_LIBREOFFICE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#18a303"/><text x="64" y="76" font-size="36" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">Libre</text></svg>"##;

const SVG_MARKDOWN: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#2d2d2d"/><text x="64" y="76" font-size="40" text-anchor="middle" fill="white" font-family="sans-serif" font-weight="bold">M↓</text></svg>"##;

const SVG_LICENSE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><rect x="40" y="32" width="48" height="64" rx="4" fill="white"/><rect x="44" y="36" width="40" height="6" rx="2" fill="#5e5c64"/><rect x="44" y="48" width="32" height="4" rx="2" fill="#5e5c64"/><rect x="44" y="58" width="36" height="4" rx="2" fill="#5e5c64"/><rect x="44" y="68" width="20" height="4" rx="2" fill="#5e5c64"/></svg>"##;

const SVG_CONTRIBUTION: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#33d17a"/><path d="M36 80h8l8-24 8 32 8-40 8 32 8-16 8 16h8" fill="none" stroke="white" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>"##;

const SVG_FOLDER: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#f5c211"/><path d="M32 96V40h24l8 8h32v48z" fill="white"/></svg>"##;

const SVG_DOCUMENT: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><path d="M40 32h32l16 16v48H40z" fill="white"/><path d="M72 32v16h16" fill="none" stroke="#3584e4" stroke-width="2"/><path d="M48 64h32v2H48zm0 12h24v2H48zm0 12h16v2H48z" fill="#3584e4"/></svg>"##;

const SVG_IMAGE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#ffa348"/><rect x="32" y="40" width="64" height="48" rx="4" fill="white"/><circle cx="48" cy="56" r="6" fill="#ffa348"/><path d="M36 88l20-24 16 16 12-8 20 16" fill="none" stroke="#ffa348" stroke-width="3"/></svg>"##;

const SVG_VIDEO: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><rect x="28" y="44" width="48" height="40" rx="4" fill="white"/><polygon points="76,52 92,64 76,76" fill="white"/></svg>"##;

const SVG_MUSIC: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#c061cb"/><circle cx="48" cy="80" r="12" fill="white"/><path d="M60 80V40l32-4v36" fill="none" stroke="white" stroke-width="5"/><circle cx="76" cy="76" r="12" fill="white"/><path d="M60 40l32-4" fill="none" stroke="white" stroke-width="3"/></svg>"##;

const SVG_ARCHIVE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><rect x="32" y="36" width="64" height="56" rx="4" fill="white"/><rect x="44" y="44" width="40" height="8" rx="2" fill="#5e5c64"/><path d="M54 60h20v8H54z" fill="#5e5c64"/><rect x="50" y="72" width="28" height="6" rx="2" fill="#5e5c64"/></svg>"##;

const SVG_DOWNLOAD: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#33d17a"/><path d="M64 32v40M44 52l20 20 20-20" fill="none" stroke="white" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/><path d="M32 84h64v12H32z" fill="white"/></svg>"##;

const SVG_UPLOAD: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><path d="M64 72V32M44 52l20-20 20 20" fill="none" stroke="white" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/><path d="M32 84h64v12H32z" fill="white"/></svg>"##;

const SVG_USER: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><circle cx="64" cy="44" r="16" fill="white"/><path d="M36 96c0-16 12-28 28-28s28 12 28 28" fill="none" stroke="white" stroke-width="5"/></svg>"##;

const SVG_SETTINGS: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><circle cx="64" cy="64" r="12" fill="none" stroke="white" stroke-width="4"/><path d="M64 34v8m0 44v8M44 44l6 6m28 28l6 6M34 64h8m44 0h8M44 84l6-6m28-28l6-6" stroke="white" stroke-width="3" stroke-linecap="round"/></svg>"##;

const SVG_INFO: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><circle cx="64" cy="64" r="24" fill="none" stroke="white" stroke-width="5"/><path d="M64 56v24M64 44v4" stroke="white" stroke-width="5" stroke-linecap="round"/></svg>"##;

const SVG_WARNING: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#ffa348"/><path d="M64 28L28 100h72z" fill="none" stroke="white" stroke-width="5" stroke-linejoin="round"/><path d="M64 60v20M64 44v4" stroke="white" stroke-width="5" stroke-linecap="round"/></svg>"##;

const SVG_ERROR: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><circle cx="64" cy="64" r="24" fill="none" stroke="white" stroke-width="5"/><path d="M50 50l28 28M78 50l-28 28" stroke="white" stroke-width="5" stroke-linecap="round"/></svg>"##;

const SVG_CHECK: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#33d17a"/><path d="M36 64l20 20 36-36" fill="none" stroke="white" stroke-width="6" stroke-linecap="round" stroke-linejoin="round"/></svg>"##;

const SVG_CLOSE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><path d="M38 38l52 52M90 38l-52 52" stroke="white" stroke-width="6" stroke-linecap="round"/></svg>"##;

const SVG_MENU: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><path d="M32 44h64M32 64h64M32 84h64" stroke="white" stroke-width="5" stroke-linecap="round"/></svg>"##;

const SVG_BACK: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><path d="M72 36L44 64l28 28" fill="none" stroke="white" stroke-width="6" stroke-linecap="round" stroke-linejoin="round"/></svg>"##;

const SVG_FORWARD: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><path d="M56 36l28 28-28 28" fill="none" stroke="white" stroke-width="6" stroke-linecap="round" stroke-linejoin="round"/></svg>"##;

const SVG_SEARCH_ICON: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><circle cx="52" cy="52" r="20" fill="none" stroke="white" stroke-width="5"/><path d="M68 68l20 20" stroke="white" stroke-width="5" stroke-linecap="round"/></svg>"##;

const SVG_HOME: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#33d17a"/><path d="M64 32L32 60v36h24V76h16v20h24V60z" fill="white"/></svg>"##;

const SVG_STAR: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#f5c211"/><path d="M64 28l12 24 28 4-20 18 6 28-26-14-26 14 6-28-20-18 28-4z" fill="white"/></svg>"##;

const SVG_HEART: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><path d="M64 96C36 72 24 56 24 44c0-12 8-20 20-20 8 0 14 4 20 12 6-8 12-12 20-12 12 0 20 8 20 20 0 12-12 28-40 52z" fill="white"/></svg>"##;

const SVG_SHARE: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><circle cx="88" cy="40" r="12" fill="white"/><circle cx="40" cy="64" r="12" fill="white"/><circle cx="88" cy="88" r="12" fill="white"/><path d="M52 60l24-14M52 68l24 14" fill="none" stroke="white" stroke-width="3"/></svg>"##;

const SVG_CLOCK: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><circle cx="64" cy="64" r="24" fill="none" stroke="white" stroke-width="5"/><path d="M64 48v16l12 12" fill="none" stroke="white" stroke-width="4" stroke-linecap="round"/></svg>"##;

const SVG_CALENDAR: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#3584e4"/><rect x="32" y="40" width="64" height="56" rx="4" fill="white"/><rect x="32" y="40" width="64" height="16" rx="4" fill="#3584e4" opacity="0.2"/><text x="64" y="82" font-size="24" text-anchor="middle" fill="#3584e4" font-weight="bold">17</text><line x1="44" y1="32" x2="44" y2="40" stroke="white" stroke-width="3"/><line x1="84" y1="32" x2="84" y2="40" stroke="white" stroke-width="3"/></svg>"##;

const SVG_NOTIFICATION: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#ffa348"/><path d="M64 28c-12 0-20 10-20 24v16l-8 12h56l-8-12V52c0-14-8-24-20-24z" fill="white"/><circle cx="64" cy="28" r="8" fill="#e66100"/><path d="M52 96c0 6 6 12 12 12s12-6 12-12" fill="none" stroke="white" stroke-width="4"/></svg>"##;

const SVG_POWER: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#e66100"/><path d="M64 28v32M48 44c-8 6-14 16-14 26 0 16 14 30 30 30s30-14 30-30c0-10-6-20-14-26" fill="none" stroke="white" stroke-width="5" stroke-linecap="round"/></svg>"##;

const SVG_LOCK: &str = r##"<svg viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg"><rect width="128" height="128" rx="24" fill="#5e5c64"/><rect x="44" y="60" width="40" height="32" rx="4" fill="white"/><path d="M48 60V48c0-8 8-16 16-16s16 8 16 16v12" fill="none" stroke="white" stroke-width="5"/><circle cx="64" cy="76" r="4" fill="#5e5c64"/></svg>"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_lookup() {
        let mgr = IconManager::new();
        assert!(mgr.icon_path("edushell", 48).is_some());
        assert!(mgr.icon_path("nonexistent", 48).is_none());
    }

    #[test]
    fn test_missing_icon_fallback() {
        let mgr = IconManager::new();
        assert!(mgr.icon_path("does-not-exist", 24).is_none());
    }

    #[test]
    fn test_theme_info() {
        let mgr = IconManager::new();
        let theme = mgr.theme();
        assert_eq!(theme.name, "EduShell Icons");
        assert_eq!(theme.author, "EduShell Team");
        assert_eq!(theme.version, "1.0.0");
        assert_eq!(theme.license, "GPL-3.0-or-later");
    }

    #[test]
    fn test_all_required_icons_exist() {
        let mgr = IconManager::new();
        let required = [
            "edushell",
            "edushell-symbolic",
            "welcome",
            "learning-hub",
            "edu-terminal",
            "project-hub",
            "office-hub",
            "browser-hub",
            "settings-center",
            "file-manager",
            "software-center",
            "search",
            "global-search",
        ];
        for name in &required {
            assert!(
                mgr.icon_path(name, 48).is_some(),
                "required icon '{}' not found",
                name
            );
        }
    }

    #[test]
    fn test_category_mappings() {
        let mgr = IconManager::new();
        assert!(mgr.category_icon("linux").is_some());
        assert!(mgr.category_icon("python").is_some());
        assert!(mgr.category_icon("rust").is_some());
        assert!(mgr.category_icon("unknown-category").is_some());
    }

    #[test]
    fn test_available_icons() {
        let mgr = IconManager::new();
        let icons = mgr.available_icons();
        assert!(!icons.is_empty());
        assert!(icons.contains(&"edushell"));
        assert!(icons.contains(&"folder"));
        assert!(icons.contains(&"terminal-icon"));
    }

    #[test]
    fn test_app_icon() {
        let mgr = IconManager::new();
        assert!(mgr.app_icon("edushell").is_some());
        assert!(mgr.app_icon("welcome").is_some());
        assert!(mgr.app_icon("nonexistent-app").is_none());
    }

    #[test]
    fn test_default_icon_manager() {
        let mgr = IconManager::default();
        assert!(mgr.available_icons().len() > 50);
    }

    #[test]
    fn test_icon_pack_get() {
        let pack = IconPack::default_icons();
        assert!(pack.get("edushell").is_some());
        assert!(pack.get("folder").is_some());
        assert!(pack.get("nonexistent").is_none());
    }

    #[test]
    fn test_category_fallback() {
        let mgr = IconManager::new();
        let cat = mgr.category_icon("completely-random-category");
        assert!(cat.is_some());
    }

    #[test]
    fn test_icon_path_size() {
        let mgr = IconManager::new();
        let small = mgr.icon_path("edushell", 16);
        assert!(small.is_some());
        let large = mgr.icon_path("edushell", 48);
        assert!(large.is_some());
    }
}
