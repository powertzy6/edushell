// SPDX-License-Identifier: GPL-3.0-or-later
//! EduShell SDK — official SDK for EduShell third-party developers.
//!
//! ## Modules
//! - `plugin` — Plugin API v1: traits, manifest, lifecycle
//! - `theme` — Theme SDK: create themes with dark/light/accent support
//! - `widget` — Widget SDK: desktop widgets
//! - `search_provider` — Search Provider SDK: add custom search sources
//! - `api` — Public API bindings for the EduShell shell
//! - `docs` — Documentation constants and helpers

pub mod api;
pub mod docs;
pub mod plugin;
pub mod search_provider;
pub mod theme;
pub mod widget;
