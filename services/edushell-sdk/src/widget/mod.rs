//! Widget SDK — desktop widgets for EduShell.
use serde::{Deserialize, Serialize};

/// Unique widget identifier.
pub type WidgetId = String;

/// Widget size constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConstraints {
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub default_width: u32,
    pub default_height: u32,
    pub resizable: bool,
}

/// Widget position on screen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Widget metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetMeta {
    pub id: WidgetId,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub category: WidgetCategory,
    pub constraints: WidgetConstraints,
    pub refresh_interval_ms: Option<u64>,
}

/// Widget category.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WidgetCategory {
    Clock,
    Calendar,
    Weather,
    SystemMonitor,
    LearningProgress,
    ProjectProgress,
    Notes,
    Custom(String),
}

/// Widget trait — implement to create a custom widget.
pub trait EduShellWidget: Send + Sync {
    fn meta(&self) -> &WidgetMeta;
    fn render(&self) -> String;
    fn on_click(&mut self);
    fn on_resize(&mut self, width: u32, height: u32);
    fn on_refresh(&mut self);
}

pub struct ClockWidget {
    meta: WidgetMeta,
    format_24h: bool,
    #[allow(dead_code)]
    show_seconds: bool,
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockWidget {
    pub fn new() -> Self {
        Self {
            meta: WidgetMeta {
                id: "clock".into(),
                name: "Clock".into(),
                author: "EduShell".into(),
                version: "1.0.0".into(),
                description: "Display current time".into(),
                category: WidgetCategory::Clock,
                constraints: WidgetConstraints {
                    min_width: 100,
                    min_height: 50,
                    max_width: 400,
                    max_height: 200,
                    default_width: 200,
                    default_height: 100,
                    resizable: true,
                },
                refresh_interval_ms: Some(1000),
            },
            format_24h: true,
            show_seconds: false,
        }
    }
}

impl EduShellWidget for ClockWidget {
    fn meta(&self) -> &WidgetMeta {
        &self.meta
    }
    fn render(&self) -> String {
        "<div>Clock</div>".into()
    }
    fn on_click(&mut self) {
        self.format_24h = !self.format_24h;
    }
    fn on_resize(&mut self, _w: u32, _h: u32) {}
    fn on_refresh(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_widget() {
        let w = ClockWidget::new();
        assert_eq!(w.meta().id, "clock");
        assert_eq!(w.meta().category, WidgetCategory::Clock);
    }

    #[test]
    fn test_clock_render() {
        let w = ClockWidget::new();
        assert_eq!(w.render(), "<div>Clock</div>");
    }

    #[test]
    fn test_clock_on_click() {
        let mut w = ClockWidget::new();
        assert!(w.format_24h);
        w.on_click();
        assert!(!w.format_24h);
    }

    #[test]
    fn test_widget_constraints() {
        let w = ClockWidget::new();
        assert!(w.meta().constraints.resizable);
        assert!(w.meta().refresh_interval_ms.is_some());
    }

    #[test]
    fn test_widget_category() {
        assert_eq!(format!("{:?}", WidgetCategory::Calendar), "Calendar");
    }

    #[test]
    fn test_widget_serde() {
        let pos = WidgetPosition {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
        };
        let json = serde_json::to_string(&pos).unwrap();
        let deserialized: WidgetPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.width, 200);
    }
}
