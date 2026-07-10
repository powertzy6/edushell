//! Documentation constants and helpers for EduShell SDK.

/// Official documentation URL.
pub const DOCS_URL: &str = "https://docs.edushell.id";

/// SDK reference URL.
pub const SDK_REF_URL: &str = "https://docs.edushell.id/sdk";

/// API reference URL.
pub const API_REF_URL: &str = "https://docs.edushell.id/api";

/// Plugin development guide URL.
pub const PLUGIN_GUIDE_URL: &str = "https://docs.edushell.id/plugins";

/// Theme development guide URL.
pub const THEME_GUIDE_URL: &str = "https://docs.edushell.id/themes";

/// Return the current SDK version string.
pub fn sdk_version() -> &'static str {
    crate::plugin::SDK_VERSION
}

/// Return the minimum shell version string.
pub fn min_shell_version() -> &'static str {
    crate::plugin::MIN_SHELL_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_url() {
        assert!(DOCS_URL.starts_with("https://"));
    }

    #[test]
    fn test_sdk_version_fn() {
        assert_eq!(sdk_version(), "1.0.0");
    }

    #[test]
    fn test_min_shell_version_fn() {
        assert_eq!(min_shell_version(), "1.0.0");
    }
}
