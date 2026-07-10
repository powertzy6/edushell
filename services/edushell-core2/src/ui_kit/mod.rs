//! UI Kit — native Rust component library for EduShell.
//! Defines types and traits for all UI components.
//! Actual GTK4 rendering lives in edushell-ui; this crate owns
//! the abstract contracts.

use serde::{Deserialize, Serialize};

/// Component identifier.
pub type ComponentId = String;

/// Base UI component trait.
pub trait UiComponent: Send + Sync {
    fn id(&self) -> &ComponentId;
    fn component_type(&self) -> &'static str;
}

/// Button component config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonConfig {
    pub id: ComponentId,
    pub label: String,
    pub icon: Option<String>,
    pub variant: ButtonVariant,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
    Link,
}

/// Card component config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardConfig {
    pub id: ComponentId,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub elevated: bool,
}

/// Dialog config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogConfig {
    pub id: ComponentId,
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: Option<String>,
    pub destructive: bool,
}

/// List item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub id: ComponentId,
    pub primary: String,
    pub secondary: Option<String>,
    pub icon: Option<String>,
    pub selected: bool,
}

/// Navigation item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavItem {
    pub id: ComponentId,
    pub label: String,
    pub icon: Option<String>,
    pub badge: Option<u32>,
    pub active: bool,
}

/// Sidebar config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarConfig {
    pub id: ComponentId,
    pub items: Vec<NavItem>,
    pub collapsed: bool,
}

/// Tab item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabItem {
    pub id: ComponentId,
    pub label: String,
    pub icon: Option<String>,
    pub active: bool,
}

/// Input field config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub id: ComponentId,
    pub label: String,
    pub placeholder: String,
    pub value: String,
    pub input_type: InputType,
    pub required: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Search,
    Number,
}

/// Toggle/switch config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchConfig {
    pub id: ComponentId,
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
}

/// Slider config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderConfig {
    pub id: ComponentId,
    pub label: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
}

/// Progress bar config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressConfig {
    pub id: ComponentId,
    pub value: f64,
    pub max: f64,
    pub indeterminate: bool,
    pub label: Option<String>,
}

/// Toast notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastConfig {
    pub id: ComponentId,
    pub message: String,
    pub variant: ToastVariant,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToastVariant {
    Info,
    Success,
    Warning,
    Error,
}

/// Menu item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: ComponentId,
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub enabled: bool,
    pub separator: bool,
    pub submenu: Option<Vec<MenuItem>>,
}

/// Tooltip config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipConfig {
    pub text: String,
    pub position: TooltipPosition,
    pub delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// Modal dialog config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalConfig {
    pub id: ComponentId,
    pub title: String,
    pub content: String,
    pub closeable: bool,
    pub width: u32,
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_config() {
        let b = ButtonConfig {
            id: "btn-1".into(),
            label: "Save".into(),
            icon: None,
            variant: ButtonVariant::Primary,
            enabled: true,
        };
        assert_eq!(b.label, "Save");
        assert_eq!(b.variant, ButtonVariant::Primary);
    }

    #[test]
    fn test_card_config() {
        let c = CardConfig {
            id: "card-1".into(),
            title: "Title".into(),
            subtitle: Some("Sub".into()),
            icon: None,
            elevated: true,
        };
        assert!(c.elevated);
    }

    #[test]
    fn test_dialog_config() {
        let d = DialogConfig {
            id: "d-1".into(),
            title: "Confirm".into(),
            message: "Sure?".into(),
            confirm_label: "Yes".into(),
            cancel_label: Some("No".into()),
            destructive: false,
        };
        assert_eq!(d.confirm_label, "Yes");
    }

    #[test]
    fn test_switch_config() {
        let s = SwitchConfig {
            id: "s-1".into(),
            label: "Dark mode".into(),
            checked: true,
            disabled: false,
        };
        assert!(s.checked);
    }

    #[test]
    fn test_slider_config() {
        let s = SliderConfig {
            id: "sl-1".into(),
            label: "Volume".into(),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 50.0,
        };
        assert_eq!(s.value, 50.0);
    }

    #[test]
    fn test_toast_config() {
        let t = ToastConfig {
            id: "t-1".into(),
            message: "Saved".into(),
            variant: ToastVariant::Success,
            duration_ms: 3000,
        };
        assert_eq!(t.variant, ToastVariant::Success);
    }

    #[test]
    fn test_menu_item() {
        let m = MenuItem {
            id: "m-1".into(),
            label: "Open".into(),
            icon: Some("folder".into()),
            shortcut: Some("Ctrl+O".into()),
            enabled: true,
            separator: false,
            submenu: None,
        };
        assert_eq!(m.shortcut, Some("Ctrl+O".into()));
    }

    #[test]
    fn test_tooltip_config() {
        let t = TooltipConfig {
            text: "Help".into(),
            position: TooltipPosition::Bottom,
            delay_ms: 500,
        };
        assert_eq!(t.position, TooltipPosition::Bottom);
    }

    #[test]
    fn test_input_type() {
        assert_eq!(format!("{:?}", InputType::Search), "Search");
    }

    #[test]
    fn test_modal_config() {
        let m = ModalConfig {
            id: "modal-1".into(),
            title: "About".into(),
            content: "v2.0".into(),
            closeable: true,
            width: 400,
            height: 300,
        };
        assert_eq!(m.width, 400);
    }

    #[test]
    fn test_nav_item_with_badge() {
        let n = NavItem {
            id: "n-1".into(),
            label: "Inbox".into(),
            icon: None,
            badge: Some(5),
            active: false,
        };
        assert_eq!(n.badge, Some(5));
    }

    #[test]
    fn test_list_item() {
        let l = ListItem {
            id: "l-1".into(),
            primary: "Item".into(),
            secondary: None,
            icon: None,
            selected: true,
        };
        assert!(l.selected);
    }
}
