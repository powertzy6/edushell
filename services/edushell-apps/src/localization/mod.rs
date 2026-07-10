use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalizedString {
    Id(String),
    En(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    Indonesian,
    English,
}

impl Lang {
    pub fn code(&self) -> &str {
        match self {
            Lang::Indonesian => "id-ID",
            Lang::English => "en-US",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalizationManager {
    current: Lang,
    strings: HashMap<&'static str, (&'static str, &'static str)>,
}

impl LocalizationManager {
    pub fn new() -> Self {
        let lang = Self::detect();
        Self {
            current: lang,
            strings: l10n::all_strings(),
        }
    }

    pub fn detect() -> Lang {
        match std::env::var("LANG") {
            Ok(val) if val.starts_with("id") || val.starts_with("id_ID") => Lang::Indonesian,
            _ => Lang::Indonesian,
        }
    }

    pub fn get<'a>(&self, key: &'a str) -> &'a str {
        match self.strings.get(key) {
            Some((id, en)) => match self.current {
                Lang::Indonesian => id,
                Lang::English => en,
            },
            None => {
                tracing::warn!("localization key not found: {}", key);
                key
            }
        }
    }

    pub fn set_language(&mut self, lang: Lang) {
        self.current = lang;
    }

    pub fn current_language(&self) -> Lang {
        self.current
    }

    pub fn available_languages(&self) -> Vec<Lang> {
        vec![Lang::Indonesian, Lang::English]
    }

    pub fn is_available(&self, code: &str) -> bool {
        matches!(code, "id-ID" | "en-US")
    }
}

impl Default for LocalizationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! tr {
    ($mgr:expr, $key:expr) => {
        $mgr.get($key)
    };
}

pub mod l10n {
    use std::collections::HashMap;

    pub fn all_strings() -> HashMap<&'static str, (&'static str, &'static str)> {
        let mut m = HashMap::new();

        fn insert(
            m: &mut HashMap<&'static str, (&'static str, &'static str)>,
            entries: Vec<(&'static str, (&'static str, &'static str))>,
        ) {
            for (k, v) in entries {
                m.insert(k, v);
            }
        }

        insert(&mut m, welcome());
        insert(&mut m, learning());
        insert(&mut m, terminal());
        insert(&mut m, project());
        insert(&mut m, office());
        insert(&mut m, browser());
        insert(&mut m, settings());
        insert(&mut m, file_manager());
        insert(&mut m, software());
        insert(&mut m, search());
        insert(&mut m, theme());
        insert(&mut m, icons());
        insert(&mut m, shortcuts());
        insert(&mut m, common());

        m
    }

    pub fn welcome() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            (
                "welcome.title",
                ("Selamat Datang di EduShell", "Welcome to EduShell"),
            ),
            (
                "welcome.subtitle",
                (
                    "Shell Desktop Pendidikan Indonesia",
                    "Indonesian Educational Desktop Shell",
                ),
            ),
            ("welcome.get_started", ("Mulai", "Get Started")),
            ("welcome.explore", ("Jelajahi Fitur", "Explore Features")),
            ("welcome.docs", ("Dokumentasi", "Documentation")),
            ("welcome.recent", ("Aktivitas Terbaru", "Recent Activity")),
            ("welcome.tips", ("Tips & Trik", "Tips & Tricks")),
            ("welcome.version", ("Versi", "Version")),
            ("welcome.about", ("Tentang EduShell", "About EduShell")),
            ("welcome.quick_links", ("Tautan Cepat", "Quick Links")),
        ]
    }

    pub fn learning() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("learning.title", ("Pusat Belajar", "Learning Hub")),
            ("learning.courses", ("Kursus", "Courses")),
            ("learning.modules", ("Modul", "Modules")),
            ("learning.quizzes", ("Kuis", "Quizzes")),
            ("learning.progress", ("Kemajuan", "Progress")),
            ("learning.certificate", ("Sertifikat", "Certificate")),
            ("learning.badges", ("Lencana", "Badges")),
            ("learning.recommended", ("Direkomendasikan", "Recommended")),
            ("learning.popular", ("Populer", "Popular")),
            ("learning.new", ("Baru", "New")),
            ("learning.completed", ("Selesai", "Completed")),
            ("learning.in_progress", ("Sedang Berjalan", "In Progress")),
            ("learning.not_started", ("Belum Dimulai", "Not Started")),
            (
                "learning.category_programming",
                ("Pemrograman", "Programming"),
            ),
            ("learning.category_linux", ("Linux", "Linux")),
            ("learning.category_network", ("Jaringan", "Networking")),
            ("learning.category_design", ("Desain", "Design")),
            ("learning.category_office", ("Perkantoran", "Office")),
            ("learning.category_security", ("Keamanan", "Security")),
            ("learning.difficulty_beginner", ("Pemula", "Beginner")),
            (
                "learning.difficulty_intermediate",
                ("Menengah", "Intermediate"),
            ),
            ("learning.difficulty_advanced", ("Lanjutan", "Advanced")),
            (
                "learning.search_placeholder",
                ("Cari kursus...", "Search courses..."),
            ),
            (
                "learning.empty_state",
                ("Belum ada kursus", "No courses yet"),
            ),
        ]
    }

    pub fn terminal() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("terminal.title", ("Terminal Edukasi", "Edu Terminal")),
            ("terminal.new_tab", ("Tab Baru", "New Tab")),
            ("terminal.close_tab", ("Tutup Tab", "Close Tab")),
            ("terminal.copy", ("Salin", "Copy")),
            ("terminal.paste", ("Tempel", "Paste")),
            ("terminal.clear", ("Bersihkan", "Clear")),
            ("terminal.find", ("Cari", "Find")),
            ("terminal.font_size", ("Ukuran Huruf", "Font Size")),
            ("terminal.color_scheme", ("Skema Warna", "Color Scheme")),
            ("terminal.reset", ("Atur Ulang", "Reset")),
            ("terminal.fullscreen", ("Layar Penuh", "Fullscreen")),
            (
                "terminal.command_history",
                ("Riwayat Perintah", "Command History"),
            ),
            ("terminal.run_command", ("Jalankan Perintah", "Run Command")),
            (
                "terminal.enter_command",
                ("Masukkan perintah...", "Enter command..."),
            ),
            ("terminal.output", ("Keluaran", "Output")),
            ("terminal.error", ("Galat", "Error")),
        ]
    }

    pub fn project() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("project.title", ("Pusat Proyek", "Project Hub")),
            ("project.new_project", ("Proyek Baru", "New Project")),
            ("project.open_project", ("Buka Proyek", "Open Project")),
            ("project.save_project", ("Simpan Proyek", "Save Project")),
            ("project.export", ("Ekspor", "Export")),
            ("project.import", ("Impor", "Import")),
            ("project.delete_project", ("Hapus Proyek", "Delete Project")),
            ("project.project_name", ("Nama Proyek", "Project Name")),
            ("project.description", ("Deskripsi", "Description")),
            (
                "project.language",
                ("Bahasa Pemrograman", "Programming Language"),
            ),
            ("project.template", ("Templat", "Template")),
            (
                "project.recent_projects",
                ("Proyek Terbaru", "Recent Projects"),
            ),
            (
                "project.empty_state",
                ("Belum ada proyek", "No projects yet"),
            ),
            (
                "project.file_structure",
                ("Struktur Berkas", "File Structure"),
            ),
            ("project.run", ("Jalankan", "Run")),
            ("project.build", ("Bangun", "Build")),
            ("project.debug", ("Debug", "Debug")),
            (
                "project.templates",
                ("Templat Tersedia", "Available Templates"),
            ),
        ]
    }

    pub fn office() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("office.title", ("Pusat Perkantoran", "Office Hub")),
            ("office.new_document", ("Dokumen Baru", "New Document")),
            (
                "office.new_spreadsheet",
                ("Spreadsheet Baru", "New Spreadsheet"),
            ),
            (
                "office.new_presentation",
                ("Presentasi Baru", "New Presentation"),
            ),
            ("office.open_file", ("Buka Berkas", "Open File")),
            ("office.recent_files", ("Berkas Terbaru", "Recent Files")),
            ("office.templates", ("Templat", "Templates")),
            ("office.format_bold", ("Tebal", "Bold")),
            ("office.format_italic", ("Miring", "Italic")),
            ("office.format_underline", ("Garis Bawah", "Underline")),
            ("office.align_left", ("Rata Kiri", "Align Left")),
            ("office.align_center", ("Rata Tengah", "Align Center")),
            ("office.align_right", ("Rata Kanan", "Align Right")),
            ("office.insert_image", ("Sisipkan Gambar", "Insert Image")),
            ("office.insert_table", ("Sisipkan Tabel", "Insert Table")),
            ("office.print", ("Cetak", "Print")),
            ("office.page_setup", ("Pengaturan Halaman", "Page Setup")),
        ]
    }

    pub fn browser() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("browser.title", ("Pusat Peramban", "Browser Hub")),
            (
                "browser.url_placeholder",
                ("Masukkan URL...", "Enter URL..."),
            ),
            ("browser.search_web", ("Cari di Web", "Search Web")),
            ("browser.bookmarks", ("Markah", "Bookmarks")),
            ("browser.history", ("Riwayat", "History")),
            ("browser.downloads", ("Unduhan", "Downloads")),
            ("browser.new_tab", ("Tab Baru", "New Tab")),
            ("browser.close_tab", ("Tutup Tab", "Close Tab")),
            ("browser.reload", ("Muat Ulang", "Reload")),
            ("browser.back", ("Kembali", "Back")),
            ("browser.forward", ("Maju", "Forward")),
            ("browser.home", ("Beranda", "Home")),
            ("browser.incognito", ("Mode Pribadi", "Incognito Mode")),
            ("browser.zoom_in", ("Perbesar", "Zoom In")),
            ("browser.zoom_out", ("Perkecil", "Zoom Out")),
            ("browser.zoom_reset", ("Setel Ulang Zoom", "Reset Zoom")),
            (
                "browser.developer_tools",
                ("Alat Pengembang", "Developer Tools"),
            ),
            (
                "browser.web_edu_filter",
                ("Filter Edukasi", "Educational Filter"),
            ),
        ]
    }

    pub fn settings() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("settings.title", ("Pusat Pengaturan", "Settings Center")),
            ("settings.general", ("Umum", "General")),
            ("settings.appearance", ("Tampilan", "Appearance")),
            ("settings.theme", ("Tema", "Theme")),
            ("settings.language", ("Bahasa", "Language")),
            ("settings.shortcuts", ("Pintasan", "Shortcuts")),
            ("settings.notifications", ("Notifikasi", "Notifications")),
            ("settings.about", ("Tentang", "About")),
            ("settings.privacy", ("Privasi", "Privacy")),
            ("settings.network", ("Jaringan", "Network")),
            ("settings.audio", ("Audio", "Audio")),
            ("settings.display", ("Tampilan", "Display")),
            ("settings.account", ("Akun", "Account")),
            ("settings.updates", ("Pembaruan", "Updates")),
            (
                "settings.reset",
                ("Atur Ulang Pengaturan", "Reset Settings"),
            ),
            (
                "settings.restore_defaults",
                ("Kembalikan ke Default", "Restore Defaults"),
            ),
        ]
    }

    pub fn file_manager() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("file_manager.title", ("Manajer Berkas", "File Manager")),
            ("file_manager.new_folder", ("Folder Baru", "New Folder")),
            ("file_manager.new_file", ("Berkas Baru", "New File")),
            ("file_manager.open", ("Buka", "Open")),
            ("file_manager.cut", ("Potong", "Cut")),
            ("file_manager.copy", ("Salin", "Copy")),
            ("file_manager.paste", ("Tempel", "Paste")),
            ("file_manager.delete", ("Hapus", "Delete")),
            ("file_manager.rename", ("Ganti Nama", "Rename")),
            ("file_manager.properties", ("Properti", "Properties")),
            ("file_manager.compress", ("Kompres", "Compress")),
            ("file_manager.extract", ("Ekstrak", "Extract")),
            ("file_manager.search", ("Cari Berkas", "Search Files")),
            ("file_manager.select_all", ("Pilih Semua", "Select All")),
            ("file_manager.home", ("Beranda", "Home")),
            ("file_manager.documents", ("Dokumen", "Documents")),
            ("file_manager.downloads", ("Unduhan", "Downloads")),
            ("file_manager.pictures", ("Gambar", "Pictures")),
            ("file_manager.music", ("Musik", "Music")),
            ("file_manager.videos", ("Video", "Videos")),
            ("file_manager.trash", ("Sampah", "Trash")),
            (
                "file_manager.empty_trash",
                ("Kosongkan Sampah", "Empty Trash"),
            ),
            ("file_manager.restore", ("Pulihkan", "Restore")),
            ("file_manager.sort_name", ("Urutkan Nama", "Sort by Name")),
            (
                "file_manager.sort_date",
                ("Urutkan Tanggal", "Sort by Date"),
            ),
            ("file_manager.sort_size", ("Urutkan Ukuran", "Sort by Size")),
            ("file_manager.view_list", ("Tampilan Daftar", "List View")),
            ("file_manager.view_grid", ("Tampilan Grid", "Grid View")),
            ("file_manager.view_icons", ("Tampilan Ikon", "Icon View")),
            (
                "file_manager.path_placeholder",
                ("/home/user", "/home/user"),
            ),
            ("file_manager.learning", ("Pusat Belajar", "Learning")),
            ("file_manager.projects", ("Pusat Proyek", "Projects")),
            ("file_manager.school", ("Sekolah", "School")),
            ("file_manager.favorite", ("Favorit", "Favorite")),
            ("file_manager.workspace", ("Ruang Kerja", "Workspace")),
            ("file_manager.recent", ("Terbaru", "Recent")),
            ("file_manager.desktop", ("Desktop", "Desktop")),
            ("file_manager.templates", ("Templat", "Templates")),
            ("file_manager.public", ("Publik", "Public")),
        ]
    }

    pub fn software() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            (
                "software.title",
                ("Pusat Perangkat Lunak", "Software Center"),
            ),
            ("software.install", ("Pasang", "Install")),
            ("software.uninstall", ("Hapus", "Uninstall")),
            ("software.update", ("Perbarui", "Update")),
            ("software.search", ("Cari Aplikasi", "Search Applications")),
            ("software.installed", ("Terpasang", "Installed")),
            (
                "software.updates_available",
                ("Pembaruan Tersedia", "Updates Available"),
            ),
            ("software.categories", ("Kategori", "Categories")),
            ("software.popular", ("Populer", "Popular")),
            ("software.recommended", ("Direkomendasikan", "Recommended")),
            ("software.education", ("Pendidikan", "Education")),
            ("software.development", ("Pengembangan", "Development")),
            ("software.graphics", ("Grafis", "Graphics")),
            ("software.games", ("Permainan", "Games")),
            ("software.office", ("Perkantoran", "Office")),
            ("software.internet", ("Internet", "Internet")),
            ("software.system", ("Sistem", "System")),
            ("software.accessories", ("Aksesoris", "Accessories")),
            ("software.snap", ("Snap", "Snap")),
            ("software.flatpak", ("Flatpak", "Flatpak")),
            ("software.appimage", ("AppImage", "AppImage")),
            ("software.source", ("Sumber", "Source")),
            ("software.version", ("Versi", "Version")),
            ("software.size", ("Ukuran", "Size")),
            ("software.license", ("Lisensi", "License")),
            (
                "software.installed_date",
                ("Tanggal Pasang", "Install Date"),
            ),
        ]
    }

    pub fn search() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            (
                "search.placeholder",
                ("Cari apa pun...", "Search anything..."),
            ),
            ("search.apps", ("Aplikasi", "Apps")),
            ("search.files", ("Berkas", "Files")),
            ("search.settings", ("Pengaturan", "Settings")),
            ("search.web", ("Web", "Web")),
            ("search.recent", ("Pencarian Terbaru", "Recent Searches")),
            ("search.no_results", ("Tidak ada hasil", "No results found")),
            ("search.results_for", ("Hasil untuk", "Results for")),
            ("search.filter", ("Filter", "Filter")),
            ("search.global", ("Pencarian Global", "Global Search")),
            ("scope.all", ("Semua", "All")),
            ("scope.applications", ("Aplikasi", "Applications")),
            ("scope.settings", ("Pengaturan", "Settings")),
            ("scope.learning", ("Pembelajaran", "Learning")),
            ("scope.projects", ("Proyek", "Projects")),
            ("scope.commands", ("Perintah", "Commands")),
            ("scope.bookmarks", ("Penanda", "Bookmarks")),
            ("scope.office_templates", ("Templat Office", "Office Templates")),
            ("scope.browser_bookmarks", ("Penanda Browser", "Browser Bookmarks")),
            ("scope.file_manager", ("Manajer Berkas", "File Manager")),
            ("scope.welcome", ("Selamat Datang", "Welcome")),
        ]
    }

    pub fn theme() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("theme.title", ("Tema", "Theme")),
            ("theme.light", ("Terang", "Light")),
            ("theme.dark", ("Gelap", "Dark")),
            ("theme.auto", ("Otomatis", "Auto")),
            ("theme.accent_color", ("Warna Aksen", "Accent Color")),
            (
                "theme.rounded_corners",
                ("Sudut Membulat", "Rounded Corners"),
            ),
            ("theme.animations", ("Animasi", "Animations")),
            ("theme.wallpaper", ("Wallpaper", "Wallpaper")),
            ("theme.font_size", ("Ukuran Huruf", "Font Size")),
            ("theme.custom_css", ("CSS Kustom", "Custom CSS")),
            ("theme.preview", ("Pratinjau", "Preview")),
            ("theme.apply", ("Terapkan", "Apply")),
            ("theme.reset", ("Atur Ulang Tema", "Reset Theme")),
        ]
    }

    pub fn icons() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("icons.title", ("Tema Ikon", "Icon Theme")),
            ("icons.default", ("Default", "Default")),
            ("icons.edu", ("EduShell", "EduShell")),
            ("icons.classic", ("Klasik", "Classic")),
            ("icons.modern", ("Modern", "Modern")),
            ("icons.symbolic", ("Simbolik", "Symbolic")),
            ("icons.custom", ("Kustom", "Custom")),
            ("icons.preview", ("Pratinjau Ikon", "Icon Preview")),
            ("icons.size", ("Ukuran Ikon", "Icon Size")),
            ("icons.reset", ("Atur Ulang Ikon", "Reset Icons")),
        ]
    }

    pub fn shortcuts() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            (
                "shortcuts.title",
                ("Pintasan Papan Ketik", "Keyboard Shortcuts"),
            ),
            ("shortcuts.launcher", ("Peluncur", "Launcher")),
            ("shortcuts.desktop", ("Desktop", "Desktop")),
            ("shortcuts.workspace", ("Ruang Kerja", "Workspace")),
            ("shortcuts.screenshot", ("Tangkapan Layar", "Screenshot")),
            ("shortcuts.system", ("Sistem", "System")),
            (
                "shortcuts.accessibility",
                ("Aksesibilitas", "Accessibility"),
            ),
            ("shortcuts.media", ("Media", "Media")),
            ("shortcuts.applications", ("Aplikasi", "Applications")),
            (
                "shortcuts.open_launcher",
                ("Buka Peluncur", "Open Launcher"),
            ),
            ("shortcuts.open_search", ("Buka Pencarian", "Open Search")),
            (
                "shortcuts.open_terminal",
                ("Buka Terminal", "Open Terminal"),
            ),
            (
                "shortcuts.open_learning",
                ("Buka Pusat Belajar", "Open Learning Hub"),
            ),
            (
                "shortcuts.open_project",
                ("Buka Pusat Proyek", "Open Project Hub"),
            ),
            (
                "shortcuts.open_office",
                ("Buka Pusat Perkantoran", "Open Office Hub"),
            ),
            (
                "shortcuts.open_browser",
                ("Buka Pusat Peramban", "Open Browser Hub"),
            ),
            (
                "shortcuts.open_settings",
                ("Buka Pengaturan", "Open Settings"),
            ),
            ("shortcuts.open_files", ("Buka Berkas", "Open Files")),
            (
                "shortcuts.next_workspace",
                ("Ruang Kerja Berikutnya", "Next Workspace"),
            ),
            (
                "shortcuts.prev_workspace",
                ("Ruang Kerja Sebelumnya", "Previous Workspace"),
            ),
            (
                "shortcuts.screenshot_full",
                ("Tangkapan Layar Penuh", "Screenshot Full"),
            ),
            (
                "shortcuts.screenshot_area",
                ("Tangkapan Layar Area", "Screenshot Area"),
            ),
            (
                "shortcuts.screenshot_window",
                ("Tangkapan Layar Jendela", "Screenshot Window"),
            ),
            (
                "shortcuts.toggle_notifications",
                ("Alihkan Notifikasi", "Toggle Notifications"),
            ),
            (
                "shortcuts.toggle_quick_settings",
                ("Alihkan Pengaturan Cepat", "Toggle Quick Settings"),
            ),
            ("shortcuts.lock_screen", ("Kunci Layar", "Lock Screen")),
            ("shortcuts.logout", ("Keluar", "Log Out")),
            ("shortcuts.shutdown", ("Matikan", "Shutdown")),
            ("shortcuts.restart", ("Mulai Ulang", "Restart")),
            (
                "shortcuts.global_search",
                ("Pencarian Global", "Global Search"),
            ),
        ]
    }

    pub fn common() -> Vec<(&'static str, (&'static str, &'static str))> {
        vec![
            ("common.ok", ("OK", "OK")),
            ("common.cancel", ("Batal", "Cancel")),
            ("common.save", ("Simpan", "Save")),
            ("common.delete", ("Hapus", "Delete")),
            ("common.edit", ("Sunting", "Edit")),
            ("common.close", ("Tutup", "Close")),
            ("common.open", ("Buka", "Open")),
            ("common.back", ("Kembali", "Back")),
            ("common.forward", ("Maju", "Forward")),
            ("common.next", ("Berikutnya", "Next")),
            ("common.previous", ("Sebelumnya", "Previous")),
            ("common.finish", ("Selesai", "Finish")),
            ("common.continue", ("Lanjutkan", "Continue")),
            ("common.retry", ("Ulangi", "Retry")),
            ("common.yes", ("Ya", "Yes")),
            ("common.no", ("Tidak", "No")),
            ("common.apply", ("Terapkan", "Apply")),
            ("common.reset", ("Atur Ulang", "Reset")),
            ("common.refresh", ("Segarkan", "Refresh")),
            ("common.search", ("Cari", "Search")),
            ("common.filter", ("Saring", "Filter")),
            ("common.sort", ("Urutkan", "Sort")),
            ("common.loading", ("Memuat...", "Loading...")),
            ("common.error", ("Galat", "Error")),
            ("common.warning", ("Peringatan", "Warning")),
            ("common.info", ("Informasi", "Information")),
            ("common.success", ("Berhasil", "Success")),
            ("common.failed", ("Gagal", "Failed")),
            ("common.name", ("Nama", "Name")),
            ("common.type", ("Tipe", "Type")),
            ("common.size", ("Ukuran", "Size")),
            ("common.date", ("Tanggal", "Date")),
            ("common.time", ("Waktu", "Time")),
            ("common.location", ("Lokasi", "Location")),
            ("common.description", ("Deskripsi", "Description")),
            ("common.details", ("Rincian", "Details")),
            ("common.more", ("Lainnya", "More")),
            ("common.less", ("Lebih Sedikit", "Less")),
            ("common.all", ("Semua", "All")),
            ("common.none", ("Tidak Ada", "None")),
            ("common.custom", ("Kustom", "Custom")),
            ("common.default", ("Default", "Default")),
            ("common.advanced", ("Lanjutan", "Advanced")),
            ("common.basic", ("Dasar", "Basic")),
            ("common.enabled", ("Aktif", "Enabled")),
            ("common.disabled", ("Nonaktif", "Disabled")),
            ("common.on", ("Hidup", "On")),
            ("common.off", ("Mati", "Off")),
            ("common.new", ("Baru", "New")),
            ("common.add", ("Tambah", "Add")),
            ("common.remove", ("Hapus", "Remove")),
            ("common.rename", ("Ganti Nama", "Rename")),
            ("common.duplicate", ("Duplikat", "Duplicate")),
            ("common.move", ("Pindahkan", "Move")),
            ("common.copy", ("Salin", "Copy")),
            ("common.paste", ("Tempel", "Paste")),
            ("common.cut", ("Potong", "Cut")),
            ("common.select", ("Pilih", "Select")),
            ("common.deselect", ("Batalkan Pilihan", "Deselect")),
            ("common.select_all", ("Pilih Semua", "Select All")),
            ("common.undo", ("Urungkan", "Undo")),
            ("common.redo", ("Ulangi", "Redo")),
            ("common.find", ("Temukan", "Find")),
            ("common.replace", ("Ganti", "Replace")),
            ("common.print", ("Cetak", "Print")),
            ("common.export", ("Ekspor", "Export")),
            ("common.import", ("Impor", "Import")),
            ("common.share", ("Bagikan", "Share")),
            ("common.download", ("Unduh", "Download")),
            ("common.upload", ("Unggah", "Upload")),
            ("common.install", ("Pasang", "Install")),
            ("common.uninstall", ("Hapus Pasang", "Uninstall")),
            ("common.update", ("Perbarui", "Update")),
            ("common.upgrade", ("Tingkatkan", "Upgrade")),
            ("common.downgrade", ("Turunkan", "Downgrade")),
            ("common.help", ("Bantuan", "Help")),
            ("common.settings", ("Pengaturan", "Settings")),
            ("common.preferences", ("Preferensi", "Preferences")),
            ("common.about", ("Tentang", "About")),
            ("common.quit", ("Keluar", "Quit")),
            ("common.exit", ("Keluar", "Exit")),
            ("common.fullscreen", ("Layar Penuh", "Fullscreen")),
            ("common.minimize", ("Minimalkan", "Minimize")),
            ("common.maximize", ("Maksimalkan", "Maximize")),
            ("common.restore", ("Pulihkan", "Restore")),
            ("common.pin", ("Sematkan", "Pin")),
            ("common.unpin", ("Lepas Semat", "Unpin")),
            ("common.lock", ("Kunci", "Lock")),
            ("common.unlock", ("Buka Kunci", "Unlock")),
            ("common.hide", ("Sembunyikan", "Hide")),
            ("common.show", ("Tampilkan", "Show")),
            ("common.toggle", ("Alihkan", "Toggle")),
            ("common.expand", ("Perluas", "Expand")),
            ("common.collapse", ("Tutup", "Collapse")),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indonesian() {
        let mgr = LocalizationManager::new();
        assert_eq!(mgr.get("common.ok"), "OK");
        assert_eq!(mgr.get("common.cancel"), "Batal");
        assert_eq!(mgr.get("welcome.title"), "Selamat Datang di EduShell");
    }

    #[test]
    fn test_english() {
        let mut mgr = LocalizationManager::new();
        mgr.set_language(Lang::English);
        assert_eq!(mgr.get("common.ok"), "OK");
        assert_eq!(mgr.get("common.cancel"), "Cancel");
        assert_eq!(mgr.get("welcome.title"), "Welcome to EduShell");
    }

    #[test]
    fn test_fallback_key_not_found() {
        let mgr = LocalizationManager::new();
        let result = mgr.get("nonexistent.key");
        assert_eq!(result, "nonexistent.key");
    }

    #[test]
    fn test_detect_default_indonesian() {
        let lang = LocalizationManager::detect();
        assert_eq!(lang, Lang::Indonesian);
    }

    #[test]
    fn test_language_codes() {
        assert_eq!(Lang::Indonesian.code(), "id-ID");
        assert_eq!(Lang::English.code(), "en-US");
    }

    #[test]
    fn test_set_language() {
        let mut mgr = LocalizationManager::new();
        assert_eq!(mgr.current_language(), Lang::Indonesian);
        mgr.set_language(Lang::English);
        assert_eq!(mgr.current_language(), Lang::English);
    }

    #[test]
    fn test_available_languages() {
        let mgr = LocalizationManager::new();
        let langs = mgr.available_languages();
        assert!(langs.contains(&Lang::Indonesian));
        assert!(langs.contains(&Lang::English));
    }

    #[test]
    fn test_is_available() {
        let mgr = LocalizationManager::new();
        assert!(mgr.is_available("id-ID"));
        assert!(mgr.is_available("en-US"));
        assert!(!mgr.is_available("jp-JP"));
    }

    #[test]
    fn test_tr_macro() {
        let mgr = LocalizationManager::new();
        assert_eq!(tr!(mgr, "common.save"), "Simpan");
    }

    #[test]
    fn test_all_keys_exist() {
        let all = l10n::all_strings();
        assert!(!all.is_empty(), "string map must not be empty");
        for (key, (id_val, en_val)) in &all {
            assert!(!key.is_empty(), "key must not be empty");
            assert!(
                !id_val.is_empty(),
                "id string for key '{}' must not be empty",
                key
            );
            assert!(
                !en_val.is_empty(),
                "en string for key '{}' must not be empty",
                key
            );
        }
    }

    #[test]
    fn test_no_empty_values() {
        let mgr = LocalizationManager::new();
        let all = l10n::all_strings();
        for (key, (id_val, en_val)) in &all {
            assert!(!id_val.is_empty(), "ID value for '{}' is empty", key);
            assert!(!en_val.is_empty(), "EN value for '{}' is empty", key);
        }
        for (key, _) in &all {
            let val = mgr.get(key);
            assert!(!val.is_empty(), "get('{}') returned empty", key);
        }
    }

    #[test]
    fn test_language_switch_consistency() {
        let mut mgr = LocalizationManager::new();
        let all = l10n::all_strings();
        for (key, (id_val, _)) in &all {
            assert_eq!(mgr.get(key), *id_val, "expected ID value for '{}'", key);
        }
        mgr.set_language(Lang::English);
        for (key, (_, en_val)) in &all {
            assert_eq!(mgr.get(key), *en_val, "expected EN value for '{}'", key);
        }
    }

    #[test]
    fn test_all_sections_have_content() {
        assert!(!l10n::welcome().is_empty());
        assert!(!l10n::learning().is_empty());
        assert!(!l10n::terminal().is_empty());
        assert!(!l10n::project().is_empty());
        assert!(!l10n::office().is_empty());
        assert!(!l10n::browser().is_empty());
        assert!(!l10n::settings().is_empty());
        assert!(!l10n::file_manager().is_empty());
        assert!(!l10n::software().is_empty());
        assert!(!l10n::search().is_empty());
        assert!(!l10n::theme().is_empty());
        assert!(!l10n::icons().is_empty());
        assert!(!l10n::shortcuts().is_empty());
        assert!(!l10n::common().is_empty());
    }

    #[test]
    fn test_default_new() {
        let mgr = LocalizationManager::default();
        assert_eq!(mgr.current_language(), Lang::Indonesian);
    }
}
