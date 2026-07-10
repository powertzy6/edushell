# Prerequisites & Risk Mitigation — EduShell v1

---

## 1. Development Prerequisites

### System Dependencies

```bash
# Ubuntu 24.04 LTS / Linux Mint 22 / Debian 12
# Required for building EduShell v1

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup component add rustfmt clippy llvm-tools-preview

# GTK4 & system libraries
sudo apt-get install -y \
    libgtk-4-dev \
    libgdk-pixbuf-2.0-dev \
    libglib2.0-dev \
    libwayland-dev \
    libadwaita-1-dev \
    gettext \
    libpango1.0-dev \
    libcairo2-dev \
    libgraphene-1.0-dev \
    meson \
    ninja-build \
    pkg-config

# Wayland development
sudo apt-get install -y \
    libwayland-dev \
    wayland-protocols \
    libwlroots-dev

# Testing & CI tools
sudo apt-get install -y \
    pre-commit \
    shellcheck \
    lcov

# Optional: Performance profiling
sudo apt-get install -y \
    linux-tools-common \
    linux-tools-generic \
    valgrind \
    heaptrack
```

### Rust Toolchain

```bash
# Verify installation
rustc --version        # 1.80+ required
cargo --version        # 1.80+ required
rustup show            # Verify targets

# Add Linux target
rustup target add x86_64-unknown-linux-gnu

# Development tools
cargo install cargo-watch       # File watcher for development
cargo install cargo-llvm-cov    # Coverage
cargo install cargo-audit       # Security audit
cargo install cargo-deny        # License check
cargo install cargo-outdated    # Dependency update check
cargo install cargo-geiger      # Unsafe code audit
cargo install cargo-xtask       # Custom tasks (if not workspace)
```

### Development Flow

```bash
# 1. Clone repository
git clone https://github.com/edushell/edushell.git
cd edushell

# 2. Setup development environment
./scripts/setup-dev.sh

# 3. Install pre-commit hooks
pre-commit install

# 4. Build
cargo build

# 5. Test
cargo test

# 6. Lint
cargo clippy --all-targets

# 7. Run (requires Cinnamon session)
cargo run -p edushell-panel
```

---

## 2. Development Readiness Checklist

### Before Starting Part 3 (Implementation)

```markdown
# ✅ = Ready | ❌ = Not Ready | 📋 = Planned

## Architecture Foundation
- [x] Vision & Mission defined
- [x] Philosophy documented
- [x] Design Principles documented
- [x] Engineering Principles documented
- [x] User Personas defined (4 personas)
- [x] Functional Requirements documented (60+ requirements)
- [x] Non-Functional Requirements documented (35+ NFRs)
- [x] Technical Constraints documented
- [x] Technology Stack finalized (Rust + GTK4-rs)
- [x] Repository Structure designed
- [x] Module System designed
- [x] Service Architecture designed
- [x] IPC Architecture designed (D-Bus + channels)
- [x] Configuration System designed (TOML + serde)
- [x] Theme Engine designed
- [x] Localization Architecture designed
- [x] Plugin Architecture prepared (v3+)
- [x] Logging Architecture designed
- [x] Crash Recovery designed
- [x] Startup Flow designed
- [x] Session Lifecycle designed
- [x] Memory Strategy defined
- [x] Performance Strategy defined
- [x] Security Strategy defined
- [x] Testing Strategy defined
- [x] CI/CD Pipeline designed
- [x] Coding Standards defined
- [x] Branching Strategy defined
- [x] Release Strategy defined
- [x] Versioning Strategy defined (SemVer)
- [x] Dependency Management defined

## Technical Prerequisites
- [ ] Rust 1.80+ installed
- [ ] GTK4 4.12+ installed
- [ ] System dependencies installed
- [ ] Editor configured (rust-analyzer)
- [ ] Pre-commit hooks installed
- [ ] CI can run on developer workstation

## Repository Setup
- [ ] GitHub repository created
- [ ] Branch protection rules configured
- [ ] GitHub Actions enabled
- [ ] Code owners defined
- [ ] Issue templates configured
- [ ] PR template configured
- [ ] Dependabot / Renovate configured

## Part 3 Ready?
- [ ] All architecture documents complete
- [ ] First component implementation can begin
- [ ] Build system tested (cargo build succeeds with template)
- [ ] Test framework verified (cargo test works)
```

### Implementation Order (Part 3+)

```markdown
## Recommended Implementation Sequence

### Sprint 1: Foundation Libraries
1. Core library: error types, logging, config
2. Core library: IPC, runtime, async bridge
3. Core library: crash handler
4. Theme engine (basic)
5. Localization (basic)

### Sprint 2: Core Shell
6. Cinnamon adapter (session, background)
7. Panel window (empty shell with layer-shell)
8. Panel widgets (clock, menu button, task list)
9. System tray (basic: clock only)

### Sprint 3: Launcher & Search
10. Launcher window (basic search)
11. App listing from desktop files
12. Category filtering
13. Favorites & recent apps

### Sprint 4: Notifications & OSD
14. Notification daemon (D-Bus service)
15. Notification popup widget
16. Notification center
17. OSD (volume, brightness)

### Sprint 5: Quick Settings & User Menu
18. Quick settings panel
19. Network/WiFi toggle
20. Audio slider
21. Dark mode toggle
22. User menu (power options)

### Sprint 6: Settings Application
23. Settings app skeleton
24. Panel settings page
25. Launcher settings page
26. Theme settings page
27. Language settings page
28. Accessibility settings page
29. Shortcuts settings page

### Sprint 7: Learning Hub
30. Learning Hub (WebView + static content)
31. Getting started content (id + en)
32. Tips & tricks content

### Sprint 8: Polish & Release
33. Theme (light + dark + high contrast)
34. Wallpaper set
35. Keyboard navigation audit
36. Accessibility audit
37. Performance optimization
38. .deb packaging
39. Documentation finalization
40. v1.0.0 release
```

---

## 3. Technical Risks & Mitigations

### Risk Register

```yaml
risks:
  - id: R01
    title: GTK4-rs binding gaps
    probability: medium
    impact: high
    description: >
      GTK4-rs may not expose all GTK4 C API features needed for
      shell-specific widgets (layer-shell integration, custom
      window types).
    mitigation: >
      1. Use C FFI directly for missing bindings (minimal surface)
      2. Contribute missing bindings upstream
      3. Fallback to GTK3-rs for specific components if necessary
    contingency: >
      If layer-shell binding is missing, use wlr-layer-shell C library
      directly via FFI (thin wrapper).

  - id: R02
    title: Wayland protocol limitations
    probability: medium
    impact: critical
    description: >
      wlr-layer-shell is not a standardized Wayland protocol.
      Cinnamon Wayland may not support it, or may change support.
    mitigation: >
      1. Use gtk4-layer-shell crate which handles protocol negotiation
      2. Maintain X11 fallback throughout v1
      3. Monitor wayland-protocols for layer-shell standardization
    contingency: >
      If layer-shell unavailable: use X11 for v1 release.
      Work with Cinnamon team to add support.

  - id: R03
    title: Solo developer productivity bottleneck
    probability: high
    impact: high
    description: >
      All architecture and implementation done by single developer.
      Risk of burnout, slow progress, or quality compromise.
    mitigation: >
      1. Modular architecture enables parallel contribution
      2. Comprehensive documentation reduces onboarding time
      3. CI/CD automates repetitive tasks
      4. Realistic sprint planning (2-week cycles)
      5. Community building starting from v1.0-alpha
    contingency: >
      Cut scope: defer non-critical features to v1.x.
      Focus on core shell (panel, launcher, settings) for v1.0.

  - id: R04
    title: Cinnamon Wayland session instability
    probability: medium
    impact: critical
    description: >
      Cinnamon Wayland session is new in Cinnamon 6.x. May have
      stability issues during EduShell v1 development.
    mitigation: >
      1. Test against Cinnamon X11 (stable) as primary target
      2. Run CI tests on both Wayland and X11
      3. Pin tested Cinnamon version in documentation
      4. Track Cinnamon Wayland bug tracker
    contingency: >
      Release v1.0 on X11 only if Wayland is unstable.
      Add Wayland support in v1.1 when Cinnamon Wayland stabilizes.

  - id: R05
    title: GTK4 memory usage exceeds target
    probability: low
    impact: medium
    description: >
      GTK4's minimum memory footprint (~30MB for shared libraries)
      may make it harder to hit 500-650MB idle target.
    mitigation: >
      1. Profile memory usage from first build
      2. Use jemalloc for better memory management
      3. Lazy-load GTK components
      4. Consider GTK3 for shell if GTK4 proves too heavy
    contingency: >
      If memory exceeds target by >100MB: strip GTK features,
      use static linking, drop animations.

  - id: R06
    title: Translation system complexity
    probability: low
    impact: medium
    description: >
      gettext integration in Rust requires C FFI. May add complexity
      to build system and cross-compilation.
    mitigation: >
      1. Use gettext-rs crate (mature binding)
      2. Wrap translation in thin abstraction layer
      3. Fallback to plain Rust string matching if gettext fails
    contingency: >
      If gettext-rs proves problematic: use custom TOML-based
      translation system as fallback.

  - id: R07
    title: Dependency on unmaintained crates
    probability: low
    impact: high
    description: >
      Some Rust crates in the ecosystem may become unmaintained
      during the project lifetime.
    mitigation: >
      1. Choose crates with active maintenance history
      2. Pin specific versions (Cargo.lock)
      3. Have replacement candidates documented
      4. Contribute to critical crates if needed
    contingency: >
      If critical crate abandoned: fork it, maintain minimal fork,
      or replace with alternative.

  - id: R08
    title: Hardware testing limitations
    probability: medium
    impact: medium
    description: >
      Developer may not have access to all target hardware
      (Celeron N3060, AMD, ARM64).
    mitigation: >
      1. Test on nearest available hardware
      2. Use QEMU with limited resources for baseline testing
      3. Community testing program during beta
      4. Document hardware requirements clearly
    contingency: >
      Release beta with known hardware limitations.
      Expand support based on community testing.

  - id: R09
    title: Accessibility compliance failure
    probability: medium
    impact: high
    description: >
      WCAG AA compliance is complex. GTK4 provides good foundations
      but shell-specific widgets may have gaps.
    mitigation: >
      1. Test with Orca screen reader from Sprint 1
      2. Use GTK4 accessibility inspector regularly
      3. Document accessibility requirements per component
      4. Keyboard navigation tested with every build
    contingency: >
      If full WCAG AA not achievable in v1.0:
      document known gaps, fix in v1.1. Ship with high-contrast
      and keyboard nav as priority.

  - id: R10
    title: Community adoption resistance
    probability: medium
    impact: medium
    description: >
      Indonesian students may resist switching from Windows.
      Linux community may criticize "another shell" on Cinnamon.
    mitigation: >
      1. Focus on user experience, not technical superiority
      2. Provide clear migration guide from Windows
      3. Position EduShell as Cinnamon enhancement, not replacement
      4. Engage with Linux Mint community early
      5. Make it trivially easy to try (Live USB?)
    contingency: >
      If adoption is low: focus on documentation and user guides.
      Target schools directly for pilot programs.
```

### Risk Matrix Summary

```
Impact ▲
       │
Critical│ R02 R04  R07     R01
       │
  High  │ R03     R09           R05 R06
       │
 Medium │    R08 R10
       │
  Low   │                R05
       │
       └──────────────────────────────► Probability
            Low    Medium   High
```

### Critical Risk Watchlist (Monthly Review)

| ID | Risk | Status | Next Review | Owner |
|----|------|--------|-------------|-------|
| R02 | Wayland protocol limitations | 🟢 No issue yet | 2026-08 | Lead |
| R04 | Cinnamon Wayland instability | 🟡 Monitor upstream | 2026-08 | Lead |
| R01 | GTK4-rs binding gaps | 🟢 No issue yet | 2026-08 | Lead |
| R03 | Solo developer | 🟡 Track velocity | 2026-08 | Lead |
| R09 | Accessibility compliance | 🟢 Planning phase | 2026-09 | Lead |

---

## 4. Summary: Part 2 Deliverables Checklist

| # | Output | Status |
|---|--------|--------|
| 1 | Technology Stack Final | ✅ TECHNOLOGY_STACK.md |
| 2 | Alasan teknis setiap pilihan | ✅ TECHNOLOGY_STACK.md |
| 3 | Perbandingan teknologi | ✅ TECHNOLOGY_STACK.md |
| 4 | Struktur repository lengkap | ✅ PROJECT_STRUCTURE.md |
| 5 | Struktur source code lengkap | ✅ PROJECT_STRUCTURE.md |
| 6 | Dependency Graph | ✅ PROJECT_STRUCTURE.md |
| 7 | Modular Architecture | ✅ MODULE_SERVICE_ARCHITECTURE.md |
| 8 | Service Architecture | ✅ MODULE_SERVICE_ARCHITECTURE.md |
| 9 | IPC Architecture | ✅ MODULE_SERVICE_ARCHITECTURE.md |
| 10 | Configuration System Design | ✅ CONFIGURATION_THEME_LOCALIZATION.md |
| 11 | Theme Engine Design | ✅ CONFIGURATION_THEME_LOCALIZATION.md |
| 12 | Localization Architecture | ✅ CONFIGURATION_THEME_LOCALIZATION.md |
| 13 | Plugin Architecture | ✅ PLUGIN_LOGGING_CRASH.md |
| 14 | Logging Architecture | ✅ PLUGIN_LOGGING_CRASH.md |
| 15 | Crash Recovery Design | ✅ PLUGIN_LOGGING_CRASH.md |
| 16 | Startup Flow Diagram | ✅ STARTUP_SESSION_PERFORMANCE.md |
| 17 | Session Lifecycle | ✅ STARTUP_SESSION_PERFORMANCE.md |
| 18 | Memory Strategy | ✅ STARTUP_SESSION_PERFORMANCE.md |
| 19 | Performance Strategy | ✅ STARTUP_SESSION_PERFORMANCE.md |
| 20 | Security Strategy | ✅ SECURITY_TESTING_CICD.md |
| 21 | Testing Strategy | ✅ SECURITY_TESTING_CICD.md |
| 22 | CI/CD Pipeline | ✅ SECURITY_TESTING_CICD.md |
| 23 | Coding Standard | ✅ STANDARDS_RELEASE.md |
| 24 | Branching Strategy | ✅ STANDARDS_RELEASE.md |
| 25 | Release Strategy | ✅ STANDARDS_RELEASE.md |
| 26 | Versioning Strategy (SemVer) | ✅ STANDARDS_RELEASE.md |
| 27 | Dependency Management | ✅ STANDARDS_RELEASE.md |
| 28 | Dokumentasi lengkap | ✅ STANDARDS_RELEASE.md |
| 29 | Checklist kesiapan | ✅ This document |
| 30 | Daftar risiko teknis | ✅ This document |
