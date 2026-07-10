// SPDX-License-Identifier: GPL-3.0-or-later
//! EduShell Core v2 — platform foundation for EduShell v2.0+.
//!
//! Zero Cinnamon dependency. All new components build atop this core.
//!
//! ## Architecture
//! - `core` — Base types, errors, identifiers, version
//! - `runtime` — Process lifecycle, thread pool, event loop
//! - `window_api` — Window management abstraction
//! - `desktop_api` — Desktop environment integration
//! - `session_api` — Session lifecycle, user management
//! - `config_engine` — Configuration persistence, schema
//! - `theme_engine` — Theme loading, CSS generation
//! - `workspace_engine` — Virtual desktop management
//! - `search_engine` — Search index, fuzzy matching
//! - `ui_kit` — Native Rust UI component library
//! - `design_system` — Design tokens, spacing, color
//! - `resource_engine` — Theme, font, icon, asset management
//! - `extension_framework` — Plugin sandbox, lifecycle
//! - `application_framework` — App templates, scaffolding
//! - `service_framework` — Background service management
//! - `package_api` — Package metadata, versioning
//! - `learning_engine` — Educational content engine
//! - `migration_engine` — Config/theme/plugin migration
//! - `compatibility_layer` — Cinnamon backward compat

pub mod application_framework;
pub mod compatibility_layer;
pub mod config_engine;
pub mod core;
pub mod design_system;
pub mod desktop_api;
pub mod extension_framework;
pub mod learning_engine;
pub mod migration_engine;
pub mod package_api;
pub mod resource_engine;
pub mod runtime;
pub mod search_engine;
pub mod service_framework;
pub mod session_api;
pub mod theme_engine;
pub mod ui_kit;
pub mod window_api;
pub mod workspace_engine;
