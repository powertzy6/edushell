// SPDX-License-Identifier: GPL-3.0-or-later

//! # EduShell Core Library
//!
//! Foundation library for the EduShell desktop shell.
//! Provides all core infrastructure: error handling, logging,
//! event bus, state management, service lifecycle, configuration,
//! theme engine, resource management, window tracking,
//! application registry, search backend, session management,
//! and settings persistence.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │                   edushell-core                   │
//! │                                                   │
//! │  error │ logging │ event │ state │ service       │
//! │  config │ theme │ resource │ window │ apps       │
//! │  search │ session │ settings │ clock             │
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! ## Feature Flags
//!
//! - `gtk`: Enable GTK4 integration (theme engine CSS provider)

#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![warn(unreachable_pub)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub mod error;
pub mod logging;
pub mod event;
pub mod state;
pub mod service;
pub mod lifecycle;
pub mod config;
pub mod resource;
pub mod theme;
pub mod window;
pub mod apps;
pub mod search;
pub mod session;
pub mod settings;
pub mod clock;

pub mod prelude {
    //! Convenience re-exports for common types.

    pub use crate::error::*;
    pub use crate::event::*;
    pub use crate::state::*;
    pub use crate::service::*;
    pub use crate::lifecycle::*;
    pub use crate::config::*;
    pub use crate::resource::*;
    pub use crate::theme::*;
    pub use crate::window::*;
    pub use crate::apps::*;
    pub use crate::search::*;
    pub use crate::session::*;
    pub use crate::settings::*;
    pub use crate::clock::*;
}

/// Version of the core library.
pub const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
