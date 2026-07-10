// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::schema::EduConfig;
use crate::error::EduResult;

/// Migrate configuration from one schema version to another.
///
/// This is a placeholder that handles the initial v1.0.0 setup.
/// As the schema evolves, migration functions are added here.
pub fn migrate_config(
    config: &mut EduConfig,
    from_version: &str,
    to_version: &str,
) -> EduResult<()> {
    if from_version == to_version {
        return Ok(());
    }

    tracing::info!(
        target: "edushell::config::migration",
        from = from_version,
        to = to_version,
        "Running configuration migration"
    );

    // Parse versions for comparison
    let from = semver::Version::parse(from_version).unwrap_or_else(|_| {
        semver::Version::new(1, 0, 0)
    });

    let to = semver::Version::parse(to_version).unwrap_or_else(|_| {
        semver::Version::new(1, 0, 0)
    });

    // Run each migration step in order
    // v1.0.0 → v1.1.0
    if from < semver::Version::new(1, 1, 0) && to >= semver::Version::new(1, 1, 0) {
        migrate_v1_0_to_v1_1(config)?;
    }

    // Future migrations:
    // v1.1.0 → v1.2.0
    // if from < semver::Version::new(1, 2, 0) && to >= semver::Version::new(1, 2, 0) {
    //     migrate_v1_1_to_v1_2(config)?;
    // }

    // Update version
    config.version = to_version.to_string();

    tracing::info!(
        target: "edushell::config::migration",
        to = to_version,
        "Configuration migration complete"
    );

    Ok(())
}

/// Migrate from v1.0.0 to v1.1.0 schema.
fn migrate_v1_0_to_v1_1(_config: &mut EduConfig) -> EduResult<()> {
    // Example migration: ensure new fields have defaults
    // In real use, this would handle field renames, type changes, etc.
    tracing::debug!(
        target: "edushell::config::migration",
        "Running v1.0.0 → v1.1.0 migration"
    );
    Ok(())
}

/// Register a custom migration function for future schema changes.
///
/// # Example
///
/// ```ignore
/// register_migration!("1.0.0", "1.1.0", |config| {
///     // Transform config fields
///     Ok(())
/// });
/// ```
#[macro_export]
macro_rules! register_migration {
    ($from:expr, $to:expr, $func:expr) => {
        // Registration logic for extensible migration system
    };
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_same_version() {
        let mut config = EduConfig::default();
        config.version = "1.0.0".into();
        let result = migrate_config(&mut config, "1.0.0", "1.0.0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_migration_updates_version() {
        let mut config = EduConfig::default();
        config.version = "1.0.0".into();
        migrate_config(&mut config, "1.0.0", "1.1.0").unwrap();
        assert_eq!(config.version, "1.1.0");
    }
}
