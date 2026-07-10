// SPDX-License-Identifier: GPL-3.0-or-later
//! EduShell Session — launches the complete desktop environment

use std::process::{exit, Command};

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    eprintln!("EduShell Session v{version}");

    // Find the right session startup script
    let session = match find_session() {
        Some(s) => s,
        None => {
            eprintln!("No window manager or desktop components found.");
            exit(1);
        }
    };

    // Launch it via dbus for proper session lifecycle
    let status = Command::new("dbus-launch")
        .arg("--exit-with-session")
        .arg(&session)
        .status()
        .expect("failed to start dbus-launch");

    exit(status.code().unwrap_or(1));
}

fn find_session() -> Option<String> {
    // Primary: openbox-session (provides proper X session management)
    if has_cmd("openbox-session") {
        return Some("openbox-session".into());
    }
    // Secondary: plain window managers
    for wm in &["openbox", "fluxbox", "xfwm4", "metacity", "marco", "muffin"] {
        if has_cmd(wm) {
            return Some(wm.to_string());
        }
    }
    // Ternary: any X client at all
    if has_cmd("xterm") {
        return Some("xterm".into());
    }
    None
}

fn has_cmd(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {name}"))
        .output()
        .is_ok_and(|o| o.status.success())
}
