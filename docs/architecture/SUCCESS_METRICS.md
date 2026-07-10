# Success Metrics — EduShell v1

## Technical Metrics

### Performance
| Metric | Target | Measurement |
|--------|--------|-------------|
| Idle memory usage | ≤ 650 MB RSS | `smem -t -P edushell` |
| CPU idle | < 2% average | `pidstat 60 1` |
| Shell startup time | < 3s | `time` in session script |
| Launcher search latency | < 200ms | Custom benchmark |
| First boot to desktop | < 10s | Stopwatch (from login to usable) |
| Uptime without crash | ≥ 30 days | Crash reporting |

### Code Quality
| Metric | Target | Tool |
|--------|--------|------|
| Code coverage (domain) | ≥ 70% | `gcov` / `lcov` |
| Code coverage (total) | ≥ 50% | `gcov` / `lcov` |
| Cyclomatic complexity | ≤ 10 per function | `lizard` |
| Lines per file | ≤ 500 | `cloc` |
| Duplication | < 5% | `dupl` / `sonar` |
| Warning-free build | 0 warnings | `meson compile` with -Werror |
| Test pass rate | 100% | `meson test` |

### Accessibility
| Metric | Target | Tool |
|--------|--------|------|
| Keyboard navigation coverage | 100% of functions | Manual audit |
| High contrast mode | Passes WCAG AA | `contrast-checker` |
| Screen reader support | Orca reads all elements | Manual test |
| `prefers-reduced-motion` | Respected | CSS audit |

### Compatibility
| Metric | Target | Tool |
|--------|--------|------|
| Ubuntu LTS install test | Pass | CI on Ubuntu 24.04 |
| Linux Mint install test | Pass | CI on Mint 22 |
| Debian install test | Pass | CI on Debian 12 |
| Wayland session | Functional | Session test |
| X11 session | Functional | Session test |

## User Experience Metrics

### Adoption
| Metric | Target (v1, 6 months post-launch) | Source |
|--------|--------------------------------------|--------|
| Total installs | ≥ 1000 | Package repository stats |
| Active daily users | ≥ 100 | Optional telemetry (opt-in) |
| GitHub stars | ≥ 100 | GitHub |
| Translation contributions | ≥ 3 contributors | Weblate/Crowdin |

### Satisfaction
| Metric | Target | Source |
|--------|--------|--------|
| User satisfaction score | ≥ 4.0 / 5.0 | Optional survey |
| Bug report → fix time | ≤ 14 days (critical) | GitHub issues |
| Feature request → release | ≤ 90 days | GitHub milestones |

### Community
| Metric | Target | Source |
|--------|--------|--------|
| Contributors | ≥ 5 (non-author) | GitHub contributors |
| Forum members (Indonesia) | ≥ 50 | Community forum |
| Documentation translations | ≥ 2 languages | Weblate |
| Schools using EduShell | ≥ 3 | Voluntary report |

## Process Metrics

| Metric | Target | Source |
|--------|--------|--------|
| CI green percentage | ≥ 95% | CI dashboard |
| PR merge time (median) | ≤ 48 hours | GitHub |
| Release cadence | Every 2 months | Milestone tracking |
| Security issues open | 0 critical | GitHub security tab |
