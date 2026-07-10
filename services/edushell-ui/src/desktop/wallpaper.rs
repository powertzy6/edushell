// SPDX-License-Identifier: GPL-3.0-or-later

//! # Wallpaper Engine
//!
//! Manages desktop wallpaper rendering with support for
//! images, solid colors, gradients, and slideshows.
//!
//! ## Modes
//!
//! - **Zoom**: Scale to fill keeping aspect ratio (may crop)
//! - **Fit**: Scale to fit keeping aspect ratio (may letterbox)
//! - **Stretch**: Scale to fill ignoring aspect ratio
//! - **Center**: Place at original size, centered
//! - **Tile**: Repeat pattern
//! - **Span**: Stretch across all monitors
//! - **Color**: Solid color only

/// Wallpaper fill mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WallpaperMode {
    /// Scale to fill keeping aspect ratio (may crop).
    Zoom,
    /// Scale to fit keeping aspect ratio (may letterbox).
    Fit,
    /// Scale to fill ignoring aspect ratio.
    Stretch,
    /// Place at original size, centered.
    Center,
    /// Repeat pattern.
    Tile,
    /// Stretch across all monitors.
    Span,
    /// Solid color only.
    Color,
}

impl WallpaperMode {
    /// All available wallpaper modes.
    pub fn all() -> &'static [Self] {
        &[
            Self::Zoom,
            Self::Fit,
            Self::Stretch,
            Self::Center,
            Self::Tile,
            Self::Span,
            Self::Color,
        ]
    }

    /// Human-readable label for this mode.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Zoom => "Zoom",
            Self::Fit => "Fit",
            Self::Stretch => "Stretch",
            Self::Center => "Center",
            Self::Tile => "Tile",
            Self::Span => "Span",
            Self::Color => "Color",
        }
    }
}

/// Wallpaper configuration.
#[derive(Debug, Clone)]
pub struct Wallpaper {
    /// Path to the wallpaper image, if any.
    pub path: Option<String>,
    /// CSS color string for solid-color wallpapers.
    pub color: String,
    /// Fill mode for the wallpaper image.
    pub mode: WallpaperMode,
    /// Interval in seconds between slideshow transitions.
    pub slideshow_interval_secs: u64,
}

impl Wallpaper {
    /// Create a new wallpaper configuration.
    pub fn new() -> Self {
        Self {
            path: None,
            color: "#2e3436".to_string(),
            mode: WallpaperMode::Zoom,
            slideshow_interval_secs: 300,
        }
    }

    /// Create a wallpaper from an image path.
    pub fn from_path(path: &str) -> Self {
        Self {
            path: Some(path.to_string()),
            color: "#2e3436".to_string(),
            mode: WallpaperMode::Zoom,
            slideshow_interval_secs: 300,
        }
    }

    /// Create a solid-color wallpaper.
    pub fn from_color(color: &str) -> Self {
        Self {
            path: None,
            color: color.to_string(),
            mode: WallpaperMode::Color,
            slideshow_interval_secs: 300,
        }
    }
}

impl Default for Wallpaper {
    fn default() -> Self {
        Self::new()
    }
}

/// Engine that manages the current wallpaper and optional slideshow.
pub struct WallpaperEngine {
    current: Wallpaper,
    slideshow_images: Vec<String>,
    slideshow_index: usize,
    slideshow_active: bool,
    gradient_from: Option<String>,
    gradient_to: Option<String>,
    gradient_angle: f64,
}

impl WallpaperEngine {
    /// Create a new wallpaper engine with default settings.
    pub fn new() -> Self {
        Self {
            current: Wallpaper::new(),
            slideshow_images: Vec::new(),
            slideshow_index: 0,
            slideshow_active: false,
            gradient_from: None,
            gradient_to: None,
            gradient_angle: 0.0,
        }
    }

    /// Set the wallpaper to a specific image path.
    pub fn set_wallpaper(&mut self, path: &str) {
        self.current.path = Some(path.to_string());
        self.slideshow_active = false;
    }

    /// Set the wallpaper to a solid color.
    pub fn set_color(&mut self, color: &str) {
        self.current.color = color.to_string();
        self.current.mode = WallpaperMode::Color;
        self.current.path = None;
        self.slideshow_active = false;
    }

    /// Set the wallpaper fill mode.
    pub fn set_mode(&mut self, mode: WallpaperMode) {
        self.current.mode = mode;
    }

    /// Start a slideshow from a list of image paths.
    pub fn set_slideshow(&mut self, images: Vec<String>, interval_secs: u64) {
        if images.is_empty() {
            self.slideshow_active = false;
            return;
        }
        self.slideshow_images = images;
        self.slideshow_index = 0;
        self.slideshow_active = true;
        self.current.slideshow_interval_secs = interval_secs;
        if let Some(path) = self.slideshow_images.first() {
            self.current.path = Some(path.clone());
        }
    }

    /// Advance to the next slideshow image.
    pub fn next_slideshow(&mut self) {
        if !self.slideshow_active || self.slideshow_images.is_empty() {
            return;
        }
        self.slideshow_index = (self.slideshow_index + 1) % self.slideshow_images.len();
        self.current.path = Some(self.slideshow_images[self.slideshow_index].clone());
    }

    /// Get a reference to the current wallpaper configuration.
    pub fn current(&self) -> &Wallpaper {
        &self.current
    }

    /// Set a gradient overlay on the wallpaper.
    pub fn set_gradient(&mut self, from: &str, to: &str, angle: f64) {
        self.gradient_from = Some(from.to_string());
        self.gradient_to = Some(to.to_string());
        self.gradient_angle = angle;
    }

    /// Remove the gradient overlay.
    pub fn clear_gradient(&mut self) {
        self.gradient_from = None;
        self.gradient_to = None;
        self.gradient_angle = 0.0;
    }

    /// Check if a gradient overlay is active.
    pub fn has_gradient(&self) -> bool {
        self.gradient_from.is_some()
    }

    /// Get the gradient from color, if any.
    pub fn gradient_from(&self) -> Option<&str> {
        self.gradient_from.as_deref()
    }

    /// Get the gradient to color, if any.
    pub fn gradient_to(&self) -> Option<&str> {
        self.gradient_to.as_deref()
    }

    /// Get the gradient angle in degrees.
    pub fn gradient_angle(&self) -> f64 {
        self.gradient_angle
    }

    /// Check if slideshow is active.
    pub fn slideshow_active(&self) -> bool {
        self.slideshow_active
    }

    /// Get the current slideshow index.
    pub fn slideshow_index(&self) -> usize {
        self.slideshow_index
    }

    /// Get the list of slideshow images.
    pub fn slideshow_images(&self) -> &[String] {
        &self.slideshow_images
    }
}

impl Default for WallpaperEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WallpaperEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WallpaperEngine")
            .field("current", &self.current)
            .field("slideshow_active", &self.slideshow_active)
            .field("slideshow_index", &self.slideshow_index)
            .field("slideshow_images", &self.slideshow_images.len())
            .field("has_gradient", &self.has_gradient())
            .finish()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallpaper_default() {
        let w = Wallpaper::new();
        assert!(w.path.is_none());
        assert_eq!(w.color, "#2e3436");
        assert_eq!(w.mode, WallpaperMode::Zoom);
    }

    #[test]
    fn test_wallpaper_from_path() {
        let w = Wallpaper::from_path("/tmp/test.jpg");
        assert_eq!(w.path.as_deref(), Some("/tmp/test.jpg"));
        assert_eq!(w.mode, WallpaperMode::Zoom);
    }

    #[test]
    fn test_wallpaper_from_color() {
        let w = Wallpaper::from_color("#ff0000");
        assert!(w.path.is_none());
        assert_eq!(w.color, "#ff0000");
        assert_eq!(w.mode, WallpaperMode::Color);
    }

    #[test]
    fn test_engine_set_wallpaper() {
        let mut e = WallpaperEngine::new();
        e.set_wallpaper("/path/to/wall.jpg");
        assert_eq!(e.current().path.as_deref(), Some("/path/to/wall.jpg"));
        assert!(!e.slideshow_active());
    }

    #[test]
    fn test_engine_set_color() {
        let mut e = WallpaperEngine::new();
        e.set_color("#000000");
        assert_eq!(e.current().color, "#000000");
        assert_eq!(e.current().mode, WallpaperMode::Color);
        assert!(e.current().path.is_none());
    }

    #[test]
    fn test_engine_set_mode() {
        let mut e = WallpaperEngine::new();
        e.set_mode(WallpaperMode::Tile);
        assert_eq!(e.current().mode, WallpaperMode::Tile);
    }

    #[test]
    fn test_engine_slideshow() {
        let mut e = WallpaperEngine::new();
        let images = vec!["a.jpg".into(), "b.jpg".into(), "c.jpg".into()];
        e.set_slideshow(images.clone(), 60);
        assert!(e.slideshow_active());
        assert_eq!(e.current().path.as_deref(), Some("a.jpg"));
        assert_eq!(e.current().slideshow_interval_secs, 60);
    }

    #[test]
    fn test_engine_slideshow_next() {
        let mut e = WallpaperEngine::new();
        let images = vec!["a.jpg".into(), "b.jpg".into()];
        e.set_slideshow(images, 10);
        assert_eq!(e.current().path.as_deref(), Some("a.jpg"));
        e.next_slideshow();
        assert_eq!(e.current().path.as_deref(), Some("b.jpg"));
        e.next_slideshow();
        assert_eq!(e.current().path.as_deref(), Some("a.jpg"));
    }

    #[test]
    fn test_engine_empty_slideshow() {
        let mut e = WallpaperEngine::new();
        e.set_slideshow(Vec::new(), 60);
        assert!(!e.slideshow_active());
    }

    #[test]
    fn test_engine_gradient() {
        let mut e = WallpaperEngine::new();
        assert!(!e.has_gradient());
        e.set_gradient("#ff0000", "#0000ff", 45.0);
        assert!(e.has_gradient());
        assert_eq!(e.gradient_from(), Some("#ff0000"));
        assert_eq!(e.gradient_to(), Some("#0000ff"));
        assert_eq!(e.gradient_angle(), 45.0);
    }

    #[test]
    fn test_engine_clear_gradient() {
        let mut e = WallpaperEngine::new();
        e.set_gradient("#fff", "#000", 90.0);
        assert!(e.has_gradient());
        e.clear_gradient();
        assert!(!e.has_gradient());
    }

    #[test]
    fn test_wallpaper_mode_all() {
        let modes = WallpaperMode::all();
        assert_eq!(modes.len(), 7);
        assert!(modes.contains(&WallpaperMode::Zoom));
        assert!(modes.contains(&WallpaperMode::Color));
    }

    #[test]
    fn test_wallpaper_mode_labels() {
        assert_eq!(WallpaperMode::Zoom.label(), "Zoom");
        assert_eq!(WallpaperMode::Color.label(), "Color");
        assert_eq!(WallpaperMode::Tile.label(), "Tile");
    }
}
