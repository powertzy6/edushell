use std::collections::HashMap;

pub struct PackageInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub maintainer: &'static str,
    pub depends: &'static [&'static str],
    pub architecture: &'static str,
    pub section: &'static str,
    pub priority: &'static str,
    pub desktop_entry: Option<DesktopEntry>,
}

pub struct DesktopEntry {
    pub name: &'static str,
    pub comment: &'static str,
    pub exec: &'static str,
    pub icon: &'static str,
    pub categories: &'static str,
    pub startup_notify: bool,
    pub no_display: bool,
}

pub struct DebPackage {
    pub control: HashMap<String, String>,
    pub desktop: Option<String>,
    pub install_scripts: HashMap<String, String>,
}

impl DebPackage {
    pub fn for_edushell() -> Self {
        Self {
            control: HashMap::from([
                ("Package".into(), "edushell".into()),
                ("Version".into(), "1.0.0".into()),
                ("Section".into(), "x11".into()),
                ("Priority".into(), "optional".into()),
                ("Architecture".into(), "amd64".into()),
                ("Maintainer".into(), "EduShell Team <dev@edushell.id>".into()),
                ("Description".into(), "Educational Desktop Shell for Indonesian Students".into()),
                ("Depends".into(), "libgtk-4-1 (>= 4.0), libcairo2, libpango-1.0-0, libgdk-pixbuf-2.0-0, python3, nemo, libreoffice-writer, libreoffice-calc".into()),
                ("Homepage".into(), "https://edushell.id".into()),
            ]),
            desktop: Some(desktop_entry_content()),
            install_scripts: HashMap::from([
                ("postinst".into(), postinst_content()),
                ("prerm".into(), prerm_content()),
            ]),
        }
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let required = ["Package", "Version", "Architecture", "Maintainer", "Description"];
        for key in &required {
            if !self.control.contains_key(*key) {
                errors.push(format!("missing control field: {}", key));
            }
        }
        errors
    }
}

fn desktop_entry_content() -> String {
    r#"[Desktop Entry]
Type=Application
Name=EduShell
Comment=Educational Desktop Shell
Exec=edushell-session
Icon=edushell
Categories=Education;Utility;Desktop;GTK;
Terminal=false
StartupNotify=true
NoDisplay=false
"#.to_string()
}

fn postinst_content() -> String {
    r#"#!/bin/bash
set -e
echo "EduShell v1.0 installed successfully."
echo "Please log out and select EduShell from your display manager."
"#.to_string()
}

fn prerm_content() -> String {
    r#"#!/bin/bash
set -e
echo "Removing EduShell..."
"#.to_string()
}

pub fn package_info() -> PackageInfo {
    PackageInfo {
        name: "edushell",
        version: "1.0.0",
        description: "Educational Desktop Shell for Indonesian Students",
        maintainer: "EduShell Team <dev@edushell.id>",
        depends: &["libgtk-4-1", "python3", "nemo", "libreoffice-writer", "libreoffice-calc"],
        architecture: "amd64",
        section: "x11",
        priority: "optional",
        desktop_entry: Some(DesktopEntry {
            name: "EduShell",
            comment: "Educational Desktop Shell",
            exec: "edushell-session",
            icon: "edushell",
            categories: "Education;Utility;Desktop;GTK;",
            startup_notify: true,
            no_display: false,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deb_package_creation() {
        let pkg = DebPackage::for_edushell();
        assert!(pkg.desktop.is_some());
        assert!(pkg.control.contains_key("Package"));
        assert_eq!(pkg.control.get("Package").unwrap(), "edushell");
    }

    #[test]
    fn test_deb_package_validate() {
        let pkg = DebPackage::for_edushell();
        let errors = pkg.validate();
        assert!(errors.is_empty(), "validation errors: {:?}", errors);
    }

    #[test]
    fn test_package_info() {
        let info = package_info();
        assert_eq!(info.name, "edushell");
        assert_eq!(info.version, "1.0.0");
        assert!(info.desktop_entry.is_some());
    }

    #[test]
    fn test_desktop_entry_content() {
        let content = desktop_entry_content();
        assert!(content.contains("EduShell"));
        assert!(content.contains("edushell-session"));
    }

    #[test]
    fn test_postinst_content() {
        let content = postinst_content();
        assert!(content.contains("EduShell v1.0"));
    }
}
