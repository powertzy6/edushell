# Risk Analysis — EduShell v1

## Risk Assessment Matrix

| ID | Risk | Probability | Impact | Mitigation |
|----|------|-------------|--------|------------|
| R01 | Cinnamon API changes break compatibility | Medium | Critical | Pin Cinnamon version, CI test against target versions |
| R02 | Wayland protocol limitations for shell features | Medium | High | Fallback to X11 for incompatible features, track upstream |
| R03 | Low adoption / no community | Medium | Medium | Focus on user experience, build in Indonesia first |
| R04 | Performance regression on baseline hardware | Medium | Critical | Continuous profiling, resource budget tracked per commit |
| R05 | GTK4 limitations for desktop shell components | Low | High | Use GTK4 Layer Shell; if insufficient, consider GTK3 for specific components |
| R06 | Translation inaccuracies | Low | Medium | Community review, Crowdin/Weblate integration |
| R07 | Developer burnout (solo/small team) | High | Critical | Modular architecture enables parallel contribution; document everything |
| R08 | Vala language limitations / deprecation | Low | Medium | Isolate Vala in shell layer; core logic in C or Rust for future-proofing |
| R09 | Cinnamon Wayland session unstable | Medium | Critical | Test rigorously before release; maintain X11 fallback |
| R10 | Accessibility compliance failure | Medium | High | WCAG audit in CI, test with Orca before each release |
| R11 | Dependency on unmaintained libraries | Low | Medium | Audit dependencies before each release; have replacement candidates |
| R12 | Hardware compatibility issues with older GPUs | Medium | Medium | Test on baseline hardware; minimal GPU feature usage |
| R13 | User confusion between EduShell and Cinnamon | Medium | Medium | Clear branding, separate session entry, documentation |
| R14 | Security vulnerability in IPC | Low | High | No IPC by default; if needed, use authenticated DBus |
| R15 | Regression from Cinnamon upstream update | Medium | High | CI tests against Cinnamon nightly; pin for stable releases |

## Risk Response Plan

### Critical Risks (Immediate Action Required)

**R01: Cinnamon API Changes**
- Lock target Cinnamon version for each EduShell release
- Integration tests run against that specific version in CI
- Maintain a compatibility matrix in `COMPATIBILITY.md`
- If breaking changes occur: patch EduShell or delay upgrade

**R04: Performance Regression**
- Every PR must include resource impact assessment
- CI benchmark suite runs on each merge to `main`
- Performance budget: if idle memory exceeds 650MB, PR is blocked
- Weekly profiling on baseline hardware

**R09: Cinnamon Wayland Session Unstable**
- Target Cinnamon Wayland only when Cinnamon 6.x is stable
- X11 session as default for v1 if Wayland is unstable
- Document known Wayland limitations in release notes
- Monitor upstream Cinnamon Wayland development closely

### High Risks (Active Monitoring)

**R02: Wayland Protocol Limitations**
- Use `wlr-layer-shell` for panel (de-facto standard, supported by Cinnamon)
- If Cinnamon Wayland doesn't support layer-shell: use X11 for v1
- Long-term: develop EduShell compositor in v3-v4

**R05: GTK4 Limitations**
- GTK4 is designed for applications, not shell panels
- Use GTK4 Layer Shell extension (gnome-shell-like approach)
- If GTK4 proves insufficient: evaluate GTK3 or direct Wayland protocol implementation

### Medium Risks (Documented but Tolerated)

**R03: Low Adoption**
- Target specific schools for pilot program
- Create video tutorials in Bahasa Indonesia
- Engage with Linux Indonesia communities
- Partnership with Linux Mint / Ubuntu Indonesia

**R13: User Confusion**
- Distinctive branding (colors, logo, naming)
- Session selector shows "EduShell" clearly in login manager
- First-run tour explaining what EduShell is
- "About EduShell" in Settings
