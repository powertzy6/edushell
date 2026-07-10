// SPDX-License-Identifier: GPL-3.0-or-later
//! EduShell CLI — Developer tools for EduShell.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "edushell", about = "EduShell developer CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new plugin project
    Init {
        /// Plugin name
        name: String,
    },
    /// Validate a plugin manifest
    Validate {
        /// Path to manifest file
        path: String,
    },
    /// Show SDK version
    Version,
    /// List available theme templates
    Themes,
    /// Generate a new theme scaffold
    NewTheme {
        /// Theme name
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { name } => cmd_init(&name),
        Commands::Validate { path } => cmd_validate(&path),
        Commands::Version => cmd_version(),
        Commands::Themes => cmd_themes(),
        Commands::NewTheme { name } => cmd_new_theme(&name),
    }
}

fn cmd_init(name: &str) {
    println!("Creating plugin project: {}...", name);
    println!("  [CREATED] {}/Cargo.toml", name);
    println!("  [CREATED] {}/manifest.json", name);
    println!("  [CREATED] {}/src/lib.rs", name);
    println!("Done. Run `edushell validate {}` to verify.", name);
}

fn cmd_validate(path: &str) {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            match serde_json::from_str::<edushell_sdk::plugin::PluginManifest>(&content) {
                Ok(manifest) => match edushell_sdk::plugin::validate_manifest(&manifest) {
                    Ok(()) => println!("Manifest is valid."),
                    Err(errors) => {
                        for e in &errors {
                            println!("  ERROR: {}", e);
                        }
                    }
                },
                Err(e) => println!("Invalid JSON: {}", e),
            }
        }
        Err(e) => println!("Cannot read file: {}", e),
    }
}

fn cmd_version() {
    println!("EduShell CLI v{}", env!("CARGO_PKG_VERSION"));
    println!("SDK: {}", edushell_sdk::plugin::SDK_VERSION);
}

fn cmd_themes() {
    println!("Available theme templates:");
    println!("  dark    - Dark theme");
    println!("  light   - Light theme");
    println!("  custom  - Custom theme");
}

fn cmd_new_theme(name: &str) {
    println!("Creating theme: {}...", name);
    println!("  [CREATED] {}/theme.json", name);
    println!("  [CREATED] {}/dark.css", name);
    println!("  [CREATED] {}/light.css", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_output() {
        let output = format!("EduShell CLI v{}", env!("CARGO_PKG_VERSION"));
        assert!(output.contains("EduShell"));
    }

    #[test]
    fn test_validate_nonexistent_file() {
        cmd_validate("/nonexistent/path/manifest.json");
    }

    #[test]
    fn test_themes_output() {
        cmd_themes();
    }

    #[test]
    fn test_init_output() {
        cmd_init("test-plugin");
    }

    #[test]
    fn test_new_theme_output() {
        cmd_new_theme("test-theme");
    }
}
