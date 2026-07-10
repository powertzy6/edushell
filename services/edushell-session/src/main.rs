// SPDX-License-Identifier: GPL-3.0-or-later
//! EduShell Session — starts the complete EduShell desktop environment.

use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    eprintln!("EduShell Session v{} starting...", env!("CARGO_PKG_VERSION"));

    std::env::set_var("XDG_CURRENT_DESKTOP", "EduShell");
    std::env::set_var("XDG_SESSION_DESKTOP", "EduShell");

    // Extend PATH for display manager environments
    if let Ok(path) = std::env::var("PATH") {
        let extended = format!("/usr/local/bin:/usr/bin:/bin:{}", path);
        std::env::set_var("PATH", extended);
    } else {
        std::env::set_var("PATH", "/usr/local/bin:/usr/bin:/bin");
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    std::thread::spawn(move || {
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);
        r.store(false, Ordering::Relaxed);
    });

    let mut children: Vec<Child> = Vec::new();

    // Try daemon
    match Command::new("edushell-daemon").spawn() {
        Ok(c) => {
            eprintln!("  [OK] edushell-daemon started");
            children.push(c);
        }
        Err(e) => eprintln!("  [--] edushell-daemon: {}", e),
    }

    // Try window managers in order (eduwm is a lib, not a bin yet)
    let wms = &["openbox", "muffin", "marco", "fluxbox", "twm", "x-window-manager"];
    let mut wm_started = false;
    for wm in wms {
        match Command::new(wm).spawn() {
            Ok(c) => {
                eprintln!("  [OK] {} started", wm);
                children.push(c);
                wm_started = true;
                break;
            }
            Err(_) => continue,
        }
    }

    // If no WM started, try xterm as X client (keeps X server alive)
    if !wm_started {
        eprintln!("  [--] No window manager found, starting xterm fallback");
        for cmd in &["xterm", "xfce4-terminal", "gnome-terminal", "xclock"] {
            if let Ok(c) = Command::new(cmd).spawn() {
                eprintln!("  [OK] {} started as fallback", cmd);
                children.push(c);
                break;
            }
        }
    }

    eprintln!("EduShell Desktop ready. (pid={})", std::process::id());

    while running.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    eprintln!("Shutting down...");
    for mut child in children {
        let _ = child.kill();
        let _ = child.wait();
    }
    eprintln!("Session ended.");
}
