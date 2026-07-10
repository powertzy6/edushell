# Non-Functional Requirements — EduShell v1

## NFR1. Performance

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR1.1 | Memory usage idle | 500–650 MB RSS | `ps_mem` / `smem` |
| NFR1.2 | CPU usage idle | < 2% average | `htop` / `pidstat` over 60s |
| NFR1.3 | Shell startup time | < 3s (from session start) | systemd-analyze |
| NFR1.4 | Launcher animation | < 200ms to first paint | Custom benchmark |
| NFR1.5 | Search index update | < 5s after file change | Tracker/miner latency |
| NFR1.6 | Notification latency | < 100ms from event to display | Custom benchmark |
| NFR1.7 | Panel resize/repaint | < 16ms (60fps) | GTK inspector |
| NFR1.8 | Minimal background processes | 0 idle daemon by EduShell | `ps aux` |

## NFR2. Reliability

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR2.1 | Uptime without crash | 30 days continuous | Crash reporting |
| NFR2.2 | Crash recovery | Panel auto-restart < 1s | Systemd/autostart |
| NFR2.3 | Settings save/load reliability | 100% | Unit test |
| NFR2.4 | Graceful degradation | Core features work if Cinnamon API fails | Integration test |

## NFR3. Security

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR3.1 | Process isolation | Semua proses EduShell di user space | `ps -u` audit |
| NFR3.2 | Configuration file permission | 600 (user-only) | `stat` |
| NFR3.3 | IPC security | No world-readable sockets | `ss -l` audit |
| NFR3.4 | No credential storage | Zero | Code audit |

## NFR4. Maintainability

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR4.1 | Code coverage (domain layer) | ≥ 70% | Coverage report |
| NFR4.2 | Documentation coverage | 100% public API | Doxygen/valadoc |
| NFR4.3 | Code duplication | < 5% | `dupl` or `sonar` scan |
| NFR4.4 | Cyclomatic complexity per function | ≤ 10 | `lizard` or `radon` |
| NFR4.5 | Lines per file | ≤ 500 | cloc |
| NFR4.6 | Warning-free compilation | 0 warnings | Compiler flags |

## NFR5. Compatibility

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR5.1 | Ubuntu LTS | 24.04, 26.04 | Installation test |
| NFR5.2 | Linux Mint | 22.x, 23.x | Installation test |
| NFR5.3 | Debian Stable | 12, 13 | Installation test |
| NFR5.4 | Wayland | Sway/Wlroots-based session | Session test |
| NFR5.5 | X11 | Fallback compatibility | Session test |
| NFR5.6 | Display scaling | 100%–200% fractional | Visual test |

## NFR6. Scalability

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR6.1 | Number of workspace | 1–32 | Functional test |
| NFR6.2 | Number of pinned apps | Unlimited | Load test |
| NFR6.3 | Number of notifications in history | Up to 100 | Load test |

## NFR7. Usability

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR7.1 | Task completion rate (new user) | > 90% first try | User testing |
| NFR7.2 | Time to find app via launcher | < 5s | User testing |
| NFR7.3 | Settings discoverability | > 80% find setting in < 10s | User testing |
| NFR7.4 | Error message clarity | Non-technical, actionable | Review |

## NFR8. Portability

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR8.1 | CPU architecture | x86_64, aarch64 | CI build test |
| NFR8.2 | GPU | Intel, AMD, NVIDIA (nouveau) | Driver test |
| NFR8.3 | Input method | All standard keyboards, touchpad | Device test |
