//! Desktop environment integration API.
use serde::{Deserialize, Serialize};

/// Desktop capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopCapabilities {
    pub compositor: bool,
    pub wallpaper_support: bool,
    pub notification_support: bool,
    pub system_tray: bool,
    pub workspace_support: bool,
    pub hot_corners: bool,
    pub desktop_icons: bool,
}

/// Desktop info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopInfo {
    pub name: String,
    pub version: String,
    pub backend: String,
    pub capabilities: DesktopCapabilities,
    pub has_cinnamon_compat: bool,
}

/// Desktop API trait.
pub trait DesktopApi: Send + Sync {
    fn info(&self) -> DesktopInfo;
    fn set_wallpaper(&self, path: &str) -> bool;
    fn show_desktop(&self) -> bool;
    fn open_terminal(&self) -> bool;
    fn lock_screen(&self) -> bool;
    fn switch_user(&self) -> bool;
}

/// Stub implementation.
pub struct StubDesktop;

impl DesktopApi for StubDesktop {
    fn info(&self) -> DesktopInfo {
        DesktopInfo {
            name: "EduShell".into(),
            version: "2.0.0".into(),
            backend: "stub".into(),
            capabilities: DesktopCapabilities {
                compositor: false,
                wallpaper_support: true,
                notification_support: true,
                system_tray: false,
                workspace_support: true,
                hot_corners: false,
                desktop_icons: false,
            },
            has_cinnamon_compat: true,
        }
    }
    fn set_wallpaper(&self, _path: &str) -> bool {
        true
    }
    fn show_desktop(&self) -> bool {
        true
    }
    fn open_terminal(&self) -> bool {
        true
    }
    fn lock_screen(&self) -> bool {
        true
    }
    fn switch_user(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_desktop() {
        let d = StubDesktop;
        let info = d.info();
        assert_eq!(info.name, "EduShell");
        assert!(info.has_cinnamon_compat);
    }

    #[test]
    fn test_desktop_capabilities() {
        let caps = DesktopCapabilities {
            compositor: true,
            wallpaper_support: true,
            notification_support: true,
            system_tray: true,
            workspace_support: true,
            hot_corners: false,
            desktop_icons: false,
        };
        assert!(caps.compositor);
        assert!(!caps.hot_corners);
    }

    #[test]
    fn test_desktop_api_methods() {
        let d = StubDesktop;
        assert!(d.set_wallpaper("/tmp/wall.png"));
        assert!(d.show_desktop());
        assert!(d.lock_screen());
    }
}
