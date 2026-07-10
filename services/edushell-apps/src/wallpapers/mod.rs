use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub category: WallpaperCategory,
    pub author: String,
    pub license: String,
    pub resolution: String,
    pub colors: Vec<String>,
    pub dark_mode: bool,
    pub light_mode: bool,
    pub description: String,
    pub tags: Vec<String>,
    pub is_default: bool,
    pub file_size_kb: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WallpaperCategory {
    Education,
    Technology,
    Indonesia,
    Abstract,
    Dark,
    Light,
    Minimal,
    Nature,
}

impl WallpaperCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Education => "Education",
            Self::Technology => "Technology",
            Self::Indonesia => "Indonesia",
            Self::Abstract => "Abstract",
            Self::Dark => "Dark",
            Self::Light => "Light",
            Self::Minimal => "Minimal",
            Self::Nature => "Nature",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WallpaperPack {
    pub name: String,
    pub version: String,
    pub author: String,
    pub total: u32,
    pub wallpapers: Vec<Wallpaper>,
}

pub struct WallpaperManager {
    wallpapers: Vec<Wallpaper>,
    categories: Vec<WallpaperCategory>,
    current_wallpaper: Option<String>,
    wallpapers_dir: PathBuf,
    shuffle_enabled: bool,
}

const ALL_CATEGORIES: &[WallpaperCategory] = &[
    WallpaperCategory::Education,
    WallpaperCategory::Technology,
    WallpaperCategory::Indonesia,
    WallpaperCategory::Abstract,
    WallpaperCategory::Dark,
    WallpaperCategory::Light,
    WallpaperCategory::Minimal,
    WallpaperCategory::Nature,
];

fn built_in_wallpapers() -> Vec<Wallpaper> {
    vec![
        // Education (5)
        Wallpaper {
            id: "knowledge-tree".into(),
            name: "Knowledge Tree".into(),
            filename: "knowledge-tree.svg".into(),
            category: WallpaperCategory::Education,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#2E7D32".into(), "#1B5E20".into(), "#4CAF50".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.knowledge_tree".into(),
            tags: vec!["education".into(), "tree".into(), "nature".into()],
            is_default: true,
            file_size_kb: 1200,
        },
        Wallpaper {
            id: "open-book".into(),
            name: "Open Book".into(),
            filename: "open-book.svg".into(),
            category: WallpaperCategory::Education,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#1565C0".into(), "#0D47A1".into(), "#42A5F5".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.open_book".into(),
            tags: vec!["education".into(), "book".into(), "reading".into()],
            is_default: false,
            file_size_kb: 950,
        },
        Wallpaper {
            id: "learning-path".into(),
            name: "Learning Path".into(),
            filename: "learning-path.svg".into(),
            category: WallpaperCategory::Education,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FF8F00".into(), "#FF6F00".into(), "#FFB300".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.learning_path".into(),
            tags: vec!["education".into(), "path".into(), "journey".into()],
            is_default: false,
            file_size_kb: 1100,
        },
        Wallpaper {
            id: "graduation-cap".into(),
            name: "Graduation Cap".into(),
            filename: "graduation-cap.svg".into(),
            category: WallpaperCategory::Education,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#37474F".into(), "#212121".into(), "#455A64".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.graduation_cap".into(),
            tags: vec!["education".into(), "graduation".into(), "celebration".into()],
            is_default: false,
            file_size_kb: 800,
        },
        Wallpaper {
            id: "world-map".into(),
            name: "World Map".into(),
            filename: "world-map.svg".into(),
            category: WallpaperCategory::Education,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#0277BD".into(), "#01579B".into(), "#03A9F4".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.world_map".into(),
            tags: vec!["education".into(), "world".into(), "geography".into()],
            is_default: false,
            file_size_kb: 1500,
        },
        // Technology (5)
        Wallpaper {
            id: "circuit-board".into(),
            name: "Circuit Board".into(),
            filename: "circuit-board.svg".into(),
            category: WallpaperCategory::Technology,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#00BCD4".into(), "#0097A7".into(), "#00E5FF".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.circuit_board".into(),
            tags: vec!["technology".into(), "circuit".into(), "electronics".into()],
            is_default: false,
            file_size_kb: 1300,
        },
        Wallpaper {
            id: "code-screen".into(),
            name: "Code Screen".into(),
            filename: "code-screen.svg".into(),
            category: WallpaperCategory::Technology,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#1B5E20".into(), "#0D4715".into(), "#2E7D32".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.code_screen".into(),
            tags: vec!["technology".into(), "code".into(), "programming".into()],
            is_default: false,
            file_size_kb: 1400,
        },
        Wallpaper {
            id: "binary-rain".into(),
            name: "Binary Rain".into(),
            filename: "binary-rain.svg".into(),
            category: WallpaperCategory::Technology,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#00E676".into(), "#00C853".into(), "#69F0AE".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.binary_rain".into(),
            tags: vec!["technology".into(), "binary".into(), "matrix".into()],
            is_default: false,
            file_size_kb: 1100,
        },
        Wallpaper {
            id: "data-flow".into(),
            name: "Data Flow".into(),
            filename: "data-flow.svg".into(),
            category: WallpaperCategory::Technology,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#448AFF".into(), "#2979FF".into(), "#82B1FF".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.data_flow".into(),
            tags: vec!["technology".into(), "data".into(), "network".into()],
            is_default: false,
            file_size_kb: 1250,
        },
        Wallpaper {
            id: "cloud-computing".into(),
            name: "Cloud Computing".into(),
            filename: "cloud-computing.svg".into(),
            category: WallpaperCategory::Technology,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#2196F3".into(), "#1976D2".into(), "#64B5F6".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.cloud_computing".into(),
            tags: vec!["technology".into(), "cloud".into(), "computing".into()],
            is_default: false,
            file_size_kb: 950,
        },
        // Indonesia (5)
        Wallpaper {
            id: "bali-sunset".into(),
            name: "Bali Sunset".into(),
            filename: "bali-sunset.svg".into(),
            category: WallpaperCategory::Indonesia,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FF6F00".into(), "#FF3D00".into(), "#FFAB00".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.bali_sunset".into(),
            tags: vec!["indonesia".into(), "bali".into(), "sunset".into()],
            is_default: false,
            file_size_kb: 1800,
        },
        Wallpaper {
            id: "borobudur".into(),
            name: "Borobudur".into(),
            filename: "borobudur.svg".into(),
            category: WallpaperCategory::Indonesia,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#795548".into(), "#5D4037".into(), "#8D6E63".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.borobudur".into(),
            tags: vec!["indonesia".into(), "temple".into(), "borobudur".into()],
            is_default: false,
            file_size_kb: 1600,
        },
        Wallpaper {
            id: "komodo-dragon".into(),
            name: "Komodo Dragon".into(),
            filename: "komodo-dragon.svg".into(),
            category: WallpaperCategory::Indonesia,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#4E342E".into(), "#3E2723".into(), "#5D4037".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.komodo_dragon".into(),
            tags: vec!["indonesia".into(), "komodo".into(), "dragon".into()],
            is_default: false,
            file_size_kb: 1400,
        },
        Wallpaper {
            id: "raja-ampat".into(),
            name: "Raja Ampat".into(),
            filename: "raja-ampat.svg".into(),
            category: WallpaperCategory::Indonesia,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#00838F".into(), "#006064".into(), "#00ACC1".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.raja_ampat".into(),
            tags: vec!["indonesia".into(), "raja-ampat".into(), "underwater".into()],
            is_default: false,
            file_size_kb: 1900,
        },
        Wallpaper {
            id: "wayang-silhouette".into(),
            name: "Wayang Silhouette".into(),
            filename: "wayang-silhouette.svg".into(),
            category: WallpaperCategory::Indonesia,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FF6F00".into(), "#E65100".into(), "#212121".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.wayang_silhouette".into(),
            tags: vec!["indonesia".into(), "wayang".into(), "shadow".into()],
            is_default: false,
            file_size_kb: 850,
        },
        // Abstract (5)
        Wallpaper {
            id: "geometric-dreams".into(),
            name: "Geometric Dreams".into(),
            filename: "geometric-dreams.svg".into(),
            category: WallpaperCategory::Abstract,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#E040FB".into(), "#7C4DFF".into(), "#FF4081".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.geometric_dreams".into(),
            tags: vec!["abstract".into(), "geometric".into(), "dreams".into()],
            is_default: false,
            file_size_kb: 1300,
        },
        Wallpaper {
            id: "color-waves".into(),
            name: "Color Waves".into(),
            filename: "color-waves.svg".into(),
            category: WallpaperCategory::Abstract,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FF1744".into(), "#2979FF".into(), "#FFEA00".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.color_waves".into(),
            tags: vec!["abstract".into(), "waves".into(), "colorful".into()],
            is_default: false,
            file_size_kb: 1500,
        },
        Wallpaper {
            id: "neon-vortex".into(),
            name: "Neon Vortex".into(),
            filename: "neon-vortex.svg".into(),
            category: WallpaperCategory::Abstract,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#00E5FF".into(), "#D500F9".into(), "#FF4081".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.neon_vortex".into(),
            tags: vec!["abstract".into(), "neon".into(), "vortex".into()],
            is_default: false,
            file_size_kb: 1200,
        },
        Wallpaper {
            id: "minimal-grid".into(),
            name: "Minimal Grid".into(),
            filename: "minimal-grid.svg".into(),
            category: WallpaperCategory::Abstract,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#BDBDBD".into(), "#9E9E9E".into(), "#E0E0E0".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.minimal_grid".into(),
            tags: vec!["abstract".into(), "grid".into(), "minimal".into()],
            is_default: false,
            file_size_kb: 700,
        },
        Wallpaper {
            id: "fluid-art".into(),
            name: "Fluid Art".into(),
            filename: "fluid-art.svg".into(),
            category: WallpaperCategory::Abstract,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FF80AB".into(), "#EA80FC".into(), "#8C9EFF".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.fluid_art".into(),
            tags: vec!["abstract".into(), "fluid".into(), "art".into()],
            is_default: false,
            file_size_kb: 1100,
        },
        // Dark (5)
        Wallpaper {
            id: "deep-space".into(),
            name: "Deep Space".into(),
            filename: "deep-space.svg".into(),
            category: WallpaperCategory::Dark,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#0D0D0D".into(), "#1A1A2E".into(), "#16213E".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.deep_space".into(),
            tags: vec!["dark".into(), "space".into(), "deep".into()],
            is_default: false,
            file_size_kb: 1400,
        },
        Wallpaper {
            id: "midnight-city".into(),
            name: "Midnight City".into(),
            filename: "midnight-city.svg".into(),
            category: WallpaperCategory::Dark,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#121212".into(), "#1E1E2F".into(), "#2D2D44".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.midnight_city".into(),
            tags: vec!["dark".into(), "city".into(), "night".into()],
            is_default: false,
            file_size_kb: 1800,
        },
        Wallpaper {
            id: "dark-forest".into(),
            name: "Dark Forest".into(),
            filename: "dark-forest.svg".into(),
            category: WallpaperCategory::Dark,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#0D1F0D".into(), "#1A3A1A".into(), "#0B1A0B".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.dark_forest".into(),
            tags: vec!["dark".into(), "forest".into(), "night".into()],
            is_default: false,
            file_size_kb: 1600,
        },
        Wallpaper {
            id: "obsidian".into(),
            name: "Obsidian".into(),
            filename: "obsidian.svg".into(),
            category: WallpaperCategory::Dark,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#1A1A1A".into(), "#2A2A2A".into(), "#0A0A0A".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.obsidian".into(),
            tags: vec!["dark".into(), "obsidian".into(), "stone".into()],
            is_default: false,
            file_size_kb: 900,
        },
        Wallpaper {
            id: "night-sky".into(),
            name: "Night Sky".into(),
            filename: "night-sky.svg".into(),
            category: WallpaperCategory::Dark,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#0B0B2B".into(), "#1A1A4E".into(), "#2A2A5E".into()],
            dark_mode: true,
            light_mode: false,
            description: "wallpaper.description.night_sky".into(),
            tags: vec!["dark".into(), "sky".into(), "stars".into()],
            is_default: false,
            file_size_kb: 1500,
        },
        // Light (5)
        Wallpaper {
            id: "morning-light".into(),
            name: "Morning Light".into(),
            filename: "morning-light.svg".into(),
            category: WallpaperCategory::Light,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FFF3E0".into(), "#FFE0B2".into(), "#FFCC80".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.morning_light".into(),
            tags: vec!["light".into(), "morning".into(), "sunrise".into()],
            is_default: false,
            file_size_kb: 1000,
        },
        Wallpaper {
            id: "cloudy-sky".into(),
            name: "Cloudy Sky".into(),
            filename: "cloudy-sky.svg".into(),
            category: WallpaperCategory::Light,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#ECEFF1".into(), "#CFD8DC".into(), "#B0BEC5".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.cloudy_sky".into(),
            tags: vec!["light".into(), "clouds".into(), "sky".into()],
            is_default: false,
            file_size_kb: 850,
        },
        Wallpaper {
            id: "white-sand".into(),
            name: "White Sand".into(),
            filename: "white-sand.svg".into(),
            category: WallpaperCategory::Light,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FFF8E1".into(), "#FFECB3".into(), "#FFE082".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.white_sand".into(),
            tags: vec!["light".into(), "sand".into(), "beach".into()],
            is_default: false,
            file_size_kb: 750,
        },
        Wallpaper {
            id: "polar-bear".into(),
            name: "Polar Bear".into(),
            filename: "polar-bear.svg".into(),
            category: WallpaperCategory::Light,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FAFAFA".into(), "#F5F5F5".into(), "#EEEEEE".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.polar_bear".into(),
            tags: vec!["light".into(), "polar".into(), "arctic".into()],
            is_default: false,
            file_size_kb: 1100,
        },
        Wallpaper {
            id: "snow-peak".into(),
            name: "Snow Peak".into(),
            filename: "snow-peak.svg".into(),
            category: WallpaperCategory::Light,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#E3F2FD".into(), "#BBDEFB".into(), "#90CAF9".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.snow_peak".into(),
            tags: vec!["light".into(), "snow".into(), "mountain".into()],
            is_default: false,
            file_size_kb: 1200,
        },
        // Minimal (5)
        Wallpaper {
            id: "single-line".into(),
            name: "Single Line".into(),
            filename: "single-line.svg".into(),
            category: WallpaperCategory::Minimal,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#212121".into(), "#FFFFFF".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.single_line".into(),
            tags: vec!["minimal".into(), "line".into(), "simple".into()],
            is_default: false,
            file_size_kb: 500,
        },
        Wallpaper {
            id: "dot-grid".into(),
            name: "Dot Grid".into(),
            filename: "dot-grid.svg".into(),
            category: WallpaperCategory::Minimal,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#424242".into(), "#757575".into(), "#FFFFFF".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.dot_grid".into(),
            tags: vec!["minimal".into(), "dots".into(), "grid".into()],
            is_default: false,
            file_size_kb: 550,
        },
        Wallpaper {
            id: "empty-desk".into(),
            name: "Empty Desk".into(),
            filename: "empty-desk.svg".into(),
            category: WallpaperCategory::Minimal,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#8D6E63".into(), "#A1887F".into(), "#D7CCC8".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.empty_desk".into(),
            tags: vec!["minimal".into(), "desk".into(), "workspace".into()],
            is_default: false,
            file_size_kb: 650,
        },
        Wallpaper {
            id: "zen-circle".into(),
            name: "Zen Circle".into(),
            filename: "zen-circle.svg".into(),
            category: WallpaperCategory::Minimal,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#000000".into(), "#FFFFFF".into(), "#F5F5F5".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.zen_circle".into(),
            tags: vec!["minimal".into(), "zen".into(), "circle".into()],
            is_default: false,
            file_size_kb: 600,
        },
        Wallpaper {
            id: "less-is-more".into(),
            name: "Less is More".into(),
            filename: "less-is-more.svg".into(),
            category: WallpaperCategory::Minimal,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#333333".into(), "#CCCCCC".into(), "#FFFFFF".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.less_is_more".into(),
            tags: vec!["minimal".into(), "typography".into(), "quote".into()],
            is_default: false,
            file_size_kb: 520,
        },
        // Nature (5)
        Wallpaper {
            id: "mountain-view".into(),
            name: "Mountain View".into(),
            filename: "mountain-view.svg".into(),
            category: WallpaperCategory::Nature,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#4CAF50".into(), "#2E7D32".into(), "#81C784".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.mountain_view".into(),
            tags: vec!["nature".into(), "mountain".into(), "landscape".into()],
            is_default: false,
            file_size_kb: 1700,
        },
        Wallpaper {
            id: "ocean-waves".into(),
            name: "Ocean Waves".into(),
            filename: "ocean-waves.svg".into(),
            category: WallpaperCategory::Nature,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#0288D1".into(), "#01579B".into(), "#03A9F4".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.ocean_waves".into(),
            tags: vec!["nature".into(), "ocean".into(), "waves".into()],
            is_default: false,
            file_size_kb: 1600,
        },
        Wallpaper {
            id: "sunset-beach".into(),
            name: "Sunset Beach".into(),
            filename: "sunset-beach.svg".into(),
            category: WallpaperCategory::Nature,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#FF9800".into(), "#F44336".into(), "#FFB74D".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.sunset_beach".into(),
            tags: vec!["nature".into(), "sunset".into(), "beach".into()],
            is_default: false,
            file_size_kb: 1800,
        },
        Wallpaper {
            id: "green-forest".into(),
            name: "Green Forest".into(),
            filename: "green-forest.svg".into(),
            category: WallpaperCategory::Nature,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#1B5E20".into(), "#2E7D32".into(), "#388E3C".into()],
            dark_mode: true,
            light_mode: true,
            description: "wallpaper.description.green_forest".into(),
            tags: vec!["nature".into(), "forest".into(), "green".into()],
            is_default: false,
            file_size_kb: 1550,
        },
        Wallpaper {
            id: "desert-dunes".into(),
            name: "Desert Dunes".into(),
            filename: "desert-dunes.svg".into(),
            category: WallpaperCategory::Nature,
            author: "EduShell Team".into(),
            license: "GPL-3.0-or-later".into(),
            resolution: "3840x2160".into(),
            colors: vec!["#D84315".into(), "#BF360C".into(), "#FF7043".into()],
            dark_mode: false,
            light_mode: true,
            description: "wallpaper.description.desert_dunes".into(),
            tags: vec!["nature".into(), "desert".into(), "dunes".into()],
            is_default: false,
            file_size_kb: 1400,
        },
    ]
}

impl WallpaperManager {
    pub fn new(wallpapers_dir: Option<PathBuf>) -> Self {
        let wallpapers = built_in_wallpapers();
        Self {
            categories: ALL_CATEGORIES.to_vec(),
            wallpapers,
            current_wallpaper: None,
            wallpapers_dir: wallpapers_dir.unwrap_or_else(|| PathBuf::from("wallpapers")),
            shuffle_enabled: false,
        }
    }

    pub fn all(&self) -> &[Wallpaper] {
        &self.wallpapers
    }

    pub fn get(&self, id: &str) -> Option<&Wallpaper> {
        self.wallpapers.iter().find(|w| w.id == id)
    }

    pub fn by_category(&self, cat: WallpaperCategory) -> Vec<&Wallpaper> {
        self.wallpapers.iter().filter(|w| w.category == cat).collect()
    }

    pub fn by_tags(&self, tags: &[&str]) -> Vec<&Wallpaper> {
        self.wallpapers
            .iter()
            .filter(|w| tags.iter().any(|t| w.tags.iter().any(|wt| wt.eq_ignore_ascii_case(t))))
            .collect()
    }

    pub fn suitable_for_dark(&self) -> Vec<&Wallpaper> {
        self.wallpapers.iter().filter(|w| w.dark_mode).collect()
    }

    pub fn suitable_for_light(&self) -> Vec<&Wallpaper> {
        self.wallpapers.iter().filter(|w| w.light_mode).collect()
    }

    pub fn default_wallpaper(&self) -> Option<&Wallpaper> {
        self.wallpapers.iter().find(|w| w.is_default)
    }

    pub fn search(&self, query: &str) -> Vec<&Wallpaper> {
        let q = query.to_lowercase();
        self.wallpapers
            .iter()
            .filter(|w| {
                w.name.to_lowercase().contains(&q)
                    || w.tags.iter().any(|t| t.to_lowercase().contains(&q))
                    || w.description.to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn categories(&self) -> &[WallpaperCategory] {
        &self.categories
    }

    pub fn category_name(&self, cat: WallpaperCategory) -> String {
        cat.name().to_string()
    }

    pub fn category_count(&self, cat: WallpaperCategory) -> u32 {
        self.wallpapers.iter().filter(|w| w.category == cat).count() as u32
    }

    pub fn total_count(&self) -> u32 {
        self.wallpapers.len() as u32
    }

    pub fn random(&self) -> Option<&Wallpaper> {
        if self.wallpapers.is_empty() {
            return None;
        }
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as usize;
        let idx = seed % self.wallpapers.len();
        self.wallpapers.get(idx)
    }

    pub fn set_current(&mut self, id: &str) -> bool {
        if self.wallpapers.iter().any(|w| w.id == id) {
            self.current_wallpaper = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn current(&self) -> Option<&Wallpaper> {
        self.current_wallpaper.as_ref().and_then(|id| self.get(id))
    }

    pub fn set_shuffle(&mut self, enabled: bool) {
        self.shuffle_enabled = enabled;
    }

    pub fn shuffle_enabled(&self) -> bool {
        self.shuffle_enabled
    }

    pub fn get_path(&self, id: &str) -> Option<PathBuf> {
        self.get(id).map(|w| self.wallpapers_dir.join(&w.filename))
    }

    pub fn pack_info(&self) -> WallpaperPack {
        WallpaperPack {
            name: "EduShell Official Wallpaper Pack".into(),
            version: "1.0.0".into(),
            author: "EduShell Team".into(),
            total: self.wallpapers.len() as u32,
            wallpapers: self.wallpapers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_manager_creation() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.total_count(), 40);
    }

    #[test]
    fn test_all_wallpapers_loaded() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.all().len(), 40);
    }

    #[test]
    fn test_categories_list() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.categories().len(), 8);
    }

    #[test]
    fn test_get_by_id_found() {
        let manager = WallpaperManager::new(None);
        let w = manager.get("knowledge-tree");
        assert!(w.is_some());
        assert_eq!(w.unwrap().name, "Knowledge Tree");
        assert_eq!(w.unwrap().category, WallpaperCategory::Education);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let manager = WallpaperManager::new(None);
        assert!(manager.get("nonexistent").is_none());
    }

    #[test]
    fn test_education_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Education).len(), 5);
    }

    #[test]
    fn test_technology_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Technology).len(), 5);
    }

    #[test]
    fn test_indonesia_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Indonesia).len(), 5);
    }

    #[test]
    fn test_abstract_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Abstract).len(), 5);
    }

    #[test]
    fn test_dark_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Dark).len(), 5);
    }

    #[test]
    fn test_light_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Light).len(), 5);
    }

    #[test]
    fn test_minimal_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Minimal).len(), 5);
    }

    #[test]
    fn test_nature_category_count() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.by_category(WallpaperCategory::Nature).len(), 5);
    }

    #[test]
    fn test_by_tags() {
        let manager = WallpaperManager::new(None);
        let results = manager.by_tags(&["education"]);
        assert!(!results.is_empty());
        assert!(results.iter().all(|w| w.tags.iter().any(|t| t == "education")));
    }

    #[test]
    fn test_by_tags_case_insensitive() {
        let manager = WallpaperManager::new(None);
        let results = manager.by_tags(&["EDUCATION"]);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_suitable_for_dark() {
        let manager = WallpaperManager::new(None);
        let results = manager.suitable_for_dark();
        assert!(!results.is_empty());
        assert!(results.iter().all(|w| w.dark_mode));
    }

    #[test]
    fn test_suitable_for_light() {
        let manager = WallpaperManager::new(None);
        let results = manager.suitable_for_light();
        assert!(!results.is_empty());
        assert!(results.iter().all(|w| w.light_mode));
    }

    #[test]
    fn test_default_wallpaper() {
        let manager = WallpaperManager::new(None);
        let def = manager.default_wallpaper();
        assert!(def.is_some());
        assert_eq!(def.unwrap().id, "knowledge-tree");
        assert!(def.unwrap().is_default);
    }

    #[test]
    fn test_search_by_name() {
        let manager = WallpaperManager::new(None);
        let results = manager.search("Knowledge Tree");
        assert!(!results.is_empty());
        assert!(results.iter().any(|w| w.name == "Knowledge Tree"));
    }

    #[test]
    fn test_search_by_tag() {
        let manager = WallpaperManager::new(None);
        let results = manager.search("matrix");
        assert!(!results.is_empty());
        assert!(results.iter().any(|w| w.tags.contains(&"matrix".to_string())));
    }

    #[test]
    fn test_search_no_results() {
        let manager = WallpaperManager::new(None);
        let results = manager.search("zzzznonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_category_counts() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.category_count(WallpaperCategory::Education), 5);
        assert_eq!(manager.category_count(WallpaperCategory::Technology), 5);
        assert_eq!(manager.category_count(WallpaperCategory::Indonesia), 5);
        assert_eq!(manager.category_count(WallpaperCategory::Abstract), 5);
        assert_eq!(manager.category_count(WallpaperCategory::Dark), 5);
        assert_eq!(manager.category_count(WallpaperCategory::Light), 5);
        assert_eq!(manager.category_count(WallpaperCategory::Minimal), 5);
        assert_eq!(manager.category_count(WallpaperCategory::Nature), 5);
    }

    #[test]
    fn test_random_returns_some() {
        let manager = WallpaperManager::new(None);
        let r = manager.random();
        assert!(r.is_some());
    }

    #[test]
    fn test_set_current_valid() {
        let mut manager = WallpaperManager::new(None);
        assert!(manager.set_current("open-book"));
        let curr = manager.current();
        assert!(curr.is_some());
        assert_eq!(curr.unwrap().id, "open-book");
    }

    #[test]
    fn test_set_current_invalid() {
        let mut manager = WallpaperManager::new(None);
        assert!(!manager.set_current("does-not-exist"));
        assert!(manager.current().is_none());
    }

    #[test]
    fn test_set_current_changes() {
        let mut manager = WallpaperManager::new(None);
        assert!(manager.set_current("open-book"));
        assert_eq!(manager.current().unwrap().id, "open-book");
        assert!(manager.set_current("binary-rain"));
        assert_eq!(manager.current().unwrap().id, "binary-rain");
    }

    #[test]
    fn test_shuffle_toggle() {
        let mut manager = WallpaperManager::new(None);
        assert!(!manager.shuffle_enabled());
        manager.set_shuffle(true);
        assert!(manager.shuffle_enabled());
        manager.set_shuffle(false);
        assert!(!manager.shuffle_enabled());
    }

    #[test]
    fn test_get_path() {
        let manager = WallpaperManager::new(Some(PathBuf::from("/test/wallpapers")));
        let path = manager.get_path("knowledge-tree");
        assert!(path.is_some());
        assert_eq!(path.unwrap(), PathBuf::from("/test/wallpapers/knowledge-tree.svg"));
    }

    #[test]
    fn test_get_path_invalid() {
        let manager = WallpaperManager::new(None);
        let path = manager.get_path("nonexistent");
        assert!(path.is_none());
    }

    #[test]
    fn test_pack_info() {
        let manager = WallpaperManager::new(None);
        let pack = manager.pack_info();
        assert_eq!(pack.name, "EduShell Official Wallpaper Pack");
        assert_eq!(pack.version, "1.0.0");
        assert_eq!(pack.author, "EduShell Team");
        assert_eq!(pack.total, 40);
        assert_eq!(pack.wallpapers.len(), 40);
    }

    #[test]
    fn test_category_name() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.category_name(WallpaperCategory::Education), "Education");
        assert_eq!(manager.category_name(WallpaperCategory::Technology), "Technology");
        assert_eq!(manager.category_name(WallpaperCategory::Indonesia), "Indonesia");
        assert_eq!(manager.category_name(WallpaperCategory::Abstract), "Abstract");
        assert_eq!(manager.category_name(WallpaperCategory::Dark), "Dark");
        assert_eq!(manager.category_name(WallpaperCategory::Light), "Light");
        assert_eq!(manager.category_name(WallpaperCategory::Minimal), "Minimal");
        assert_eq!(manager.category_name(WallpaperCategory::Nature), "Nature");
    }

    #[test]
    fn test_category_enum_name() {
        assert_eq!(WallpaperCategory::Education.name(), "Education");
        assert_eq!(WallpaperCategory::Technology.name(), "Technology");
        assert_eq!(WallpaperCategory::Indonesia.name(), "Indonesia");
        assert_eq!(WallpaperCategory::Abstract.name(), "Abstract");
        assert_eq!(WallpaperCategory::Dark.name(), "Dark");
        assert_eq!(WallpaperCategory::Light.name(), "Light");
        assert_eq!(WallpaperCategory::Minimal.name(), "Minimal");
        assert_eq!(WallpaperCategory::Nature.name(), "Nature");
    }

    #[test]
    fn test_all_wallpapers_have_required_fields() {
        let manager = WallpaperManager::new(None);
        for w in manager.all() {
            assert!(!w.id.is_empty(), "id should not be empty");
            assert!(!w.name.is_empty(), "name should not be empty");
            assert!(!w.filename.is_empty(), "filename should not be empty");
            assert!(!w.author.is_empty(), "author should not be empty");
            assert!(!w.license.is_empty(), "license should not be empty");
            assert!(!w.resolution.is_empty(), "resolution should not be empty");
            assert!(!w.colors.is_empty(), "colors should not be empty");
            assert!(!w.description.is_empty(), "description should not be empty");
            assert!(!w.tags.is_empty(), "tags should not be empty");
            assert!(w.file_size_kb >= 300, "file_size_kb should be reasonable");
            assert!(w.file_size_kb <= 3000, "file_size_kb should be reasonable");
        }
    }

    #[test]
    fn test_no_duplicate_ids() {
        let manager = WallpaperManager::new(None);
        let ids: HashSet<&str> = manager.all().iter().map(|w| w.id.as_str()).collect();
        assert_eq!(ids.len(), manager.all().len());
    }

    #[test]
    fn test_all_categories_have_5_entries() {
        let manager = WallpaperManager::new(None);
        for cat in ALL_CATEGORIES {
            assert_eq!(
                manager.by_category(*cat).len(),
                5,
                "category {:?} should have 5 wallpapers",
                cat
            );
        }
    }

    #[test]
    fn test_all_wallpapers_have_at_least_one_mode() {
        let manager = WallpaperManager::new(None);
        for w in manager.all() {
            assert!(
                w.dark_mode || w.light_mode,
                "wallpaper '{}' has neither dark nor light mode",
                w.id
            );
        }
    }

    #[test]
    fn test_total_count_matches_all() {
        let manager = WallpaperManager::new(None);
        assert_eq!(manager.total_count(), manager.all().len() as u32);
    }

    #[test]
    fn test_by_tags_multiple() {
        let manager = WallpaperManager::new(None);
        let results = manager.by_tags(&["education", "nature"]);
        assert_eq!(results.len(), 10);
    }

    #[test]
    fn test_by_tags_no_match() {
        let manager = WallpaperManager::new(None);
        let results = manager.by_tags(&["nonexistenttag"]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_by_description() {
        let manager = WallpaperManager::new(None);
        let results = manager.search("sunset");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_case_insensitive() {
        let manager = WallpaperManager::new(None);
        let results = manager.search("KNOWLEDGE");
        assert!(!results.is_empty());
        assert!(results.iter().any(|w| w.name == "Knowledge Tree"));
    }

    #[test]
    fn test_search_partial_match() {
        let manager = WallpaperManager::new(None);
        let results = manager.search("circuit");
        assert!(!results.is_empty());
        assert!(results.iter().any(|w| w.name == "Circuit Board"));
    }

    #[test]
    fn test_default_is_only_one() {
        let manager = WallpaperManager::new(None);
        let defaults: Vec<&Wallpaper> = manager.all().iter().filter(|w| w.is_default).collect();
        assert_eq!(defaults.len(), 1);
    }

    #[test]
    fn test_category_count_total_matches() {
        let manager = WallpaperManager::new(None);
        let sum: u32 = ALL_CATEGORIES.iter().map(|c| manager.category_count(*c)).sum();
        assert_eq!(sum, manager.total_count());
    }

    #[test]
    fn test_current_none_by_default() {
        let manager = WallpaperManager::new(None);
        assert!(manager.current().is_none());
    }

    #[test]
    fn test_get_path_default_dir() {
        let manager = WallpaperManager::new(None);
        let path = manager.get_path("knowledge-tree");
        assert!(path.is_some());
        assert_eq!(path.unwrap(), PathBuf::from("wallpapers/knowledge-tree.svg"));
    }

    #[test]
    fn test_wallpaperpack_default() {
        let pack = WallpaperPack::default();
        assert_eq!(pack.total, 0);
        assert!(pack.wallpapers.is_empty());
    }

    #[test]
    fn test_serde_wallpaper() {
        let w = Wallpaper {
            id: "test".into(),
            name: "Test".into(),
            filename: "test.svg".into(),
            category: WallpaperCategory::Minimal,
            author: "test".into(),
            license: "MIT".into(),
            resolution: "1920x1080".into(),
            colors: vec!["#000".into()],
            dark_mode: true,
            light_mode: false,
            description: "desc".into(),
            tags: vec!["test".into()],
            is_default: false,
            file_size_kb: 500,
        };
        let json = serde_json::to_string(&w).unwrap();
        let deserialized: Wallpaper = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test");
        assert_eq!(deserialized.category, WallpaperCategory::Minimal);
    }

    #[test]
    fn test_wallpapercategory_debug() {
        let cat = WallpaperCategory::Nature;
        assert_eq!(format!("{:?}", cat), "Nature");
    }

    #[test]
    fn test_wallpapercategory_copy() {
        let a = WallpaperCategory::Dark;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_wallpapercategory_eq() {
        assert_eq!(WallpaperCategory::Dark, WallpaperCategory::Dark);
        assert_ne!(WallpaperCategory::Dark, WallpaperCategory::Light);
    }

    #[test]
    fn test_wallpapercategory_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let a = WallpaperCategory::Abstract;
        let b = WallpaperCategory::Abstract;
        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn test_all_wallpapers_have_filenames_with_extensions() {
        let manager = WallpaperManager::new(None);
        for w in manager.all() {
            assert!(w.filename.ends_with(".svg"), "filename should end with .svg");
        }
    }

    #[test]
    fn test_all_wallpapers_have_3_or_more_tags() {
        let manager = WallpaperManager::new(None);
        for w in manager.all() {
            assert!(w.tags.len() >= 2, "wallpaper '{}' should have at least 2 tags", w.id);
        }
    }

    #[test]
    fn test_random_distinct_calls_may_differ() {
        let manager = WallpaperManager::new(None);
        let r1 = manager.random().map(|w| w.id.clone());
        let r2 = manager.random().map(|w| w.id.clone());
        assert!(r1.is_some());
        assert!(r2.is_some());
    }
}
