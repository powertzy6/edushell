// SPDX-License-Identifier: GPL-3.0-or-later

//! # EduShell Desktop UI
//!
//! Complete desktop shell graphical interface built on GTK4.
//! Provides the panel, dock, launcher, overview, notifications,
//! quick settings, clock, power menu, and all desktop visual
//! components.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                      DesktopShell                          │
//! │  ┌─────────┐ ┌──────────┐ ┌───────────┐ ┌──────────────┐  │
//! │  │  Panel  │ │   Dock   │ │  Desktop  │ │   Overview   │  │
//! │  │ ┌─────┐ │ │ ┌──────┐ │ │ ┌───────┐ │ │ ┌──────────┐ │  │
//! │  │ │ App │ │ │ │Pinned│ │ │ │Wallpap│ │ │ │Workspaces│ │  │
//! │  │ │Menu │ │ │ │Running│ │ │ │Icons  │ │ │ │Windows   │ │  │
//! │  │ │Clock │ │ │ │Indic.│ │ │ │Menu   │ │ │ │Search    │ │  │
//! │  │ │Tray  │ │ │ └──────┘ │ │ └───────┘ │ │ └──────────┘ │  │
//! │  │ │QS    │ │ └──────────┘ └───────────┘ └──────────────┘  │
//! │  │ └─────┘ │                                               │
//! │  └─────────┘                                               │
//! │  ┌────────────┐ ┌───────────┐ ┌───────────┐               │
//! │  │Notif Center│ │ Launcher  │ │ QuickSet  │               │
//! │  └────────────┘ └───────────┘ └───────────┘               │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Feature Flags
//!
//! - `gtk`: Enable GTK4 rendering (required for graphical shell)
//! - `wayland`: Wayland-specific backend support
//! - `x11`: X11-specific backend support
//! - `dbus`: D-Bus integration for system services
//!
//! ## Component Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | `app` | Main application shell orchestrator |
//! | `panel` | Top/bottom panel with app menu, clock, tray |
//! | `dock` | Application dock with pinned & running apps |
//! | `launcher` | Application launcher with search & categories |
//! | `desktop` | Desktop area with wallpaper & icons |
//! | `overview` | Overview mode for workspaces & windows |
//! | `notifications` | Notification center and toast popups |
//! | `quick_settings` | Quick settings panel |
//! | `clock` | Clock widget and calendar popover |
//! | `power` | Power/shutdown menu |
//! | `animation` | Easing, spring, and timeline animation engine |
//! | `accessibility` | Focus, screen reader, and a11y support |
//! | `multi_monitor` | Multi-monitor layout and management |
//! | `theme` | GTK CSS theming and style management |
//! | `localization` | Internationalization (i18n) support |
//! | `settings` | Configuration engine integration |
//! | `testing` | UI testing utilities |

#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![warn(unreachable_pub)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[cfg(feature = "gtk")]
extern crate gtk4 as gtk;
#[cfg(feature = "gtk")]
extern crate libadwaita as adw;

pub mod app;
pub mod panel;
pub mod dock;
pub mod launcher;
pub mod desktop;
pub mod overview;
pub mod notifications;
pub mod quick_settings;
pub mod clock;
pub mod power;
pub mod animation;
pub mod accessibility;
pub mod multi_monitor;
pub mod theme;
pub mod localization;
pub mod settings;
pub mod testing;

pub mod prelude {
    //! Convenience re-exports for common UI types.
    pub use crate::app::*;
    pub use crate::panel::*;
    pub use crate::dock::*;
    pub use crate::launcher::*;
    pub use crate::desktop::*;
    pub use crate::overview::*;
    pub use crate::notifications::*;
    pub use crate::quick_settings::*;
    pub use crate::clock::*;
    pub use crate::power::*;
    pub use crate::animation::*;
    pub use crate::accessibility::*;
    pub use crate::multi_monitor::*;
    pub use crate::theme::*;
    pub use crate::localization::*;
}

/// Version of the UI crate.
pub const UI_VERSION: &str = env!("CARGO_PKG_VERSION");
