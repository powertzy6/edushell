// SPDX-License-Identifier: GPL-3.0-or-later

//! # Configuration Engine
//!
//! Manages application configuration with TOML files,
//! schema versioning, automatic migration, file watching,
//! and factory reset.

mod schema;
mod manager;
mod migration;

pub use schema::*;
pub use manager::*;
pub use migration::*;
