// SPDX-License-Identifier: GPL-3.0-or-later
//! EduShell Session — starts the complete EduShell desktop environment.

use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    eprintln!("EduShell Session v{}", env!("CARGO_PKG_VERSION"));

    std::env::set_var("XDG_CURRENT_DESKTOP", "EduShell");
    std::env::set_var("XDG_SESSION_DESKTOP", "EduShell");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Handle Ctrl+C
    std::thread::spawn(move || {
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);
        r.store(false, Ordering::Relaxed);
    });

    let mut children: Vec<Child> = Vec::new();

    // Start daemon
    if let Ok(child) = Command::new("edushell-daemon").spawn() {
        children.push(child);
    }

    // Start window manager
    let wm = find_wm();
    if let Some(ref wm_name) = wm {
        if let Ok(child) = Command::new(wm_name).spawn() {
            children.push(child);
        }
    }

    eprintln!("EduShell Desktop ready. Press Ctrl+C to end.");

    while running.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    for mut child in children {
        let _ = child.kill();
        let _ = child.wait();
    }
}

fn find_wm() -> Option<&'static str> {
    for wm in &["eduwm", "muffin", "openbox", "marco", "fluxbox"] {
        if Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", wm))
            .output()
            .is_ok_and(|o| o.status.success())
        {
            return Some(wm);
        }
    }
    None
}
