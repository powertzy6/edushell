# Standards, Release & Dependency Management — EduShell

---

## 1. Coding Standards (Rust)

### Formatting

```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4
edition = "2024"
use_small_heuristics = "Default"
```

### Linting

```toml
# clippy.toml
cognitive-complexity = 10
cyclomatic-complexity = 10
msrv = "1.80"
```

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Crates | `edushell-<name>` | `edushell-core`, `edushell-panel` |
| Modules | `snake_case` | `config::manager` |
| Structs | `PascalCase` | `ConfigManager`, `ThemeEngine` |
| Traits | `PascalCase` | `Plugin`, `ServiceModule` |
| Enums | `PascalCase` | `PanelPosition`, `ThemeMode` |
| Functions | `snake_case` | `load_config()` |
| Variables | `snake_case` | `panel_position` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_WORKSPACE_COUNT` |
| Type params | `PascalCase` (single letter) | `T`, `E`, `F` |
| Lifetimes | `'a`, `'b` | `'ctx`, `'cfg` |

### File Structure

```rust
// Every Rust source file follows this structure:

// 1. License header (GPL-3.0)
// 2. Module documentation (optional)
// 3. Imports (std → external → crate)
// 4. Public re-exports
// 5. Constants
// 6. Structs & Enums
// 7. Trait implementations
// 8. Private functions
// 9. Tests module

// Example:
// SPDX-License-Identifier: GPL-3.0-or-later

//! Configuration manager for EduShell shell settings.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::error::ConfigError;

pub use schema::EduConfig;

const BACKUP_DIR: &str = "backups";

pub struct ConfigManager { ... }

#[cfg(test)]
mod tests { ... }
```

### Documentation Standards

```rust
/// Every public API item MUST have documentation.
///
/// # Arguments
///
/// * `config` - The configuration to validate
///
/// # Returns
///
/// `Ok(())` if valid, `Err(ConfigError)` with description
///
/// # Errors
///
/// Returns `ConfigError::Validate` if any field has invalid value
///
/// # Examples
///
/// ```rust
/// let config = EduConfig::default();
/// validate_config(&config).expect("Default config should be valid");
/// ```
pub fn validate_config(config: &EduConfig) -> Result<(), ConfigError> { ... }
```

### Forbidden Patterns

| Pattern | Reason | Alternative |
|---------|--------|-------------|
| `unwrap()` | Panics on error | `?` operator, `.context()`, `.unwrap_or_default()` |
| `expect()` | Only in tests | Proper error handling |
| `panic!()` | Only in unreachable | Error return |
| `todo!()` | Never committed | Implement or remove |
| `unreachable!()` | Only with justification | Match exhaustively |
| `unsafe` | Must be justified | Safe abstractions |
| `as` casting | Type unsafe | `From`/`TryFrom` traits |
| `anyhow` / `thiserror` | Dependency bloat | Custom `EduError` type |

---

## 2. Branching Strategy

### Git Flow (Simplified)

```
main (stable)
  ▲
  │  merge --no-ff
  │
develop (integration)
  ▲         ▲         ▲
  │         │         │
  │  merge  │  merge  │  merge
  │         │         │
feat/A    feat/B    fix/C
```

### Branch Types

| Branch | From | Into | Lifetime |
|--------|------|------|----------|
| `main` | — | — | Permanent. Latest stable release |
| `develop` | `main` | `main` | Permanent. Integration branch |
| `feat/<name>` | `develop` | `develop` | Short-lived. Feature development |
| `fix/<name>` | `develop` | `develop` | Short-lived. Bug fix |
| `hotfix/<name>` | `main` | `main` + `develop` | Emergency fix for current release |
| `release/v*` | `develop` | `main` + `develop` | Release preparation |

### Branch Rules

- `main` is **protected**: No direct pushes. Only PR merges.
- `develop` is **protected**: No direct pushes. Only PR merges.
- Feature branches: Squash merge to `develop`.
- Release branches: Merge commit to `main` (preserves history).
- Hotfix branches: Cherry-pick to `main`, merge back to `develop`.

---

## 3. Release Strategy

### Release Types

| Type | Version | Frequency | Description |
|------|---------|-----------|-------------|
| Alpha | `v1.0.0-alpha.N` | Weekly | Internal testing, features WIP |
| Beta | `v1.0.0-beta.N` | Bi-weekly | Feature-complete, bug hunting |
| RC | `v1.0.0-rc.N` | As needed | Release candidate |
| Stable | `v1.0.0` | Every 2 months | Production release |
| Patch | `v1.0.1` | As needed | Bug fixes |
| Minor | `v1.1.0` | Every 2 months | New features (backward compatible) |
| Major | `v2.0.0` | Roadmap-based | Architectural changes |

### Release Checklist

```markdown
# Release Checklist v1.0.0

## Pre-release
- [ ] Feature freeze (2 weeks before)
- [ ] All tests pass on CI
- [ ] Code coverage ≥ targets
- [ ] Changelog updated
- [ ] Version bumped in Cargo.toml files
- [ ] Translation coverage ≥ 90%
- [ ] Documentation updated
- [ ] Performance benchmarks green
- [ ] Accessibility checklist passed
- [ ] Security audit passed

## Release
- [ ] `release/v1.0.0` branch created
- [ ] Final `cargo build --release`
- [ ] Generate checksums
- [ ] Build .deb package
- [ ] Create GitHub release with tag
- [ ] Upload artifacts
- [ ] Deploy documentation

## Post-release
- [ ] Merge release branch to main
- [ ] Merge main back to develop
- [ ] Announce on community channels
- [ ] Monitor bug reports for 48h
```

### Version Bumping

```bash
# On release branch
# 1. Update version in all Cargo.toml files
# 2. Update version in meson.build (if used)
# 3. Update version in documentation

# cargo xtask bump {major|minor|patch}
cargo xtask bump minor
```

---

## 4. Versioning Strategy (SemVer 2.0)

### Format

```
v<MAJOR>.<MINOR>.<PATCH>[-PRE_RELEASE][+BUILD_METADATA]

Example: v1.0.0-alpha.1
Example: v1.2.3
Example: v2.0.0
```

### When to Increment

| Component | MAJOR | MINOR | PATCH |
|-----------|-------|-------|-------|
| Core library API | Breaking change | New feature | Bug fix |
| Shell components | WM replacement | New widget | UI fix |
| Configuration schema | Incompatible change | New key | Fix validation |
| D-Bus interface | Breaking change | New method | Fix behavior |
| Plugin API | Breaking change | New hook | Bug fix |

### Pre-release Tags

| Tag | Meaning |
|-----|---------|
| `-alpha.N` | Features incomplete, breaking changes expected |
| `-beta.N` | Feature complete, bugs expected |
| `-rc.N` | Release candidate, final testing |

---

## 5. Dependency Management Strategy

### Principles

1. **Minimal dependencies**: Every dependency must justify its existence
2. **Pin all versions**: `Cargo.lock` committed to repository
3. **Prefer pure Rust**: Avoid C FFI dependencies when possible
4. **License compatibility**: All deps must be GPL-3.0 compatible
5. **Regular audit**: `cargo audit` in CI
6. **Update policy**: Minor updates within 30 days, major within 90 days

### Dependency Categories

```toml
# Core: Required for all components
edushell-core = { path = "core/edushell-core" }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
tracing = "0.1"
tokio = { version = "1", features = ["full"] }
zbus = "4"

# GUI: Required for shell/apps
gtk4 = { version = "0.8", features = ["v4_12"] }
gtk4-layer-shell = "0.4"
glib = "0.20"

# Optional / platform-specific
# (behind feature flags)
[target.'cfg(target_os = "linux")'.dependencies]
procfs = "0.16"  # Memory reporting (optional)
```

### Dependency Update Policy

```yaml
# renovate.json (Renovate Bot config)
{
  "$schema": "https://docs.renovatebot.com/renovate-schema.json",
  "extends": ["config:base"],
  "schedule": ["before 9am on Monday"],
  "labels": ["dependencies"],
  "packageRules": [
    {
      "matchUpdateTypes": ["patch"],
      "autoMerge": true
    },
    {
      "matchUpdateTypes": ["minor"],
      "groupName": "minor updates",
      "autoMerge": false
    },
    {
      "matchDepTypes": ["devDependencies"],
      "autoMerge": true
    }
  ]
}
```

---

## 6. Developer Environment

### Dev Container

```jsonc
// .devcontainer/devcontainer.json
{
  "name": "EduShell Development",
  "image": "mcr.microsoft.com/devcontainers/rust:latest",
  "features": {
    "ghcr.io/devcontainers/features/desktop-lite:1": {}
  },
  "customizations": {
    "vscode": {
      "extensions": [
        "rust-lang.rust-analyzer",
        "tamasfe.even-better-toml",
        "serayuzgur.crates",
        "vadimcn.vscode-lldb"
      ]
    }
  },
  "postCreateCommand": "sudo apt-get update && sudo apt-get install -y libgtk-4-dev libgdk-pixbuf-2.0-dev libglib2.0-dev libwayland-dev gettext libadwaita-1-dev && cargo build"
}
```

### VS Code Settings

```jsonc
// .vscode/settings.json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "editor.rulers": [100],
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### Debug Profiles

```jsonc
// .vscode/launch.json
{
  "configurations": [
    {
      "name": "Debug Panel",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/edushell-panel",
      "args": [],
      "cargo": {
        "args": ["build", "-p", "edushell-panel"]
      }
    },
    {
      "name": "Debug Settings",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/edushell-settings",
      "args": [],
      "cargo": {
        "args": ["build", "-p", "edushell-settings"]
      }
    }
  ]
}
```

---

## 7. Required Documentation

### Pre-Implementation Documentation

| Document | File | Status |
|----------|------|--------|
| Architecture Foundation | `docs/architecture/ARCHITECTURE_FOUNDATION.md` | ✅ This document |
| Technology Stack | `docs/architecture/TECHNOLOGY_STACK.md` | ✅ |
| Project Structure | `docs/architecture/PROJECT_STRUCTURE.md` | ✅ |
| Module & Service Architecture | `docs/architecture/MODULE_SERVICE_ARCHITECTURE.md` | ✅ |
| Configuration Design | `docs/architecture/CONFIGURATION_THEME_LOCALIZATION.md` | ✅ (partial) |
| Theme Engine Design | `docs/architecture/CONFIGURATION_THEME_LOCALIZATION.md` | ✅ (partial) |
| Localization Architecture | `docs/architecture/CONFIGURATION_THEME_LOCALIZATION.md` | ✅ (partial) |
| Plugin Architecture | `docs/architecture/PLUGIN_LOGGING_CRASH.md` | ✅ (partial) |
| Logging Architecture | `docs/architecture/PLUGIN_LOGGING_CRASH.md` | ✅ (partial) |
| Crash Recovery Design | `docs/architecture/PLUGIN_LOGGING_CRASH.md` | ✅ (partial) |
| Startup Flow | `docs/architecture/STARTUP_SESSION_PERFORMANCE.md` | ✅ |
| Session Lifecycle | `docs/architecture/STARTUP_SESSION_PERFORMANCE.md` | ✅ |
| Memory Strategy | `docs/architecture/STARTUP_SESSION_PERFORMANCE.md` | ✅ |
| Performance Strategy | `docs/architecture/STARTUP_SESSION_PERFORMANCE.md` | ✅ |
| Security Strategy | `docs/architecture/SECURITY_TESTING_CICD.md` | ✅ |
| Testing Strategy | `docs/architecture/SECURITY_TESTING_CICD.md` | ✅ |
| CI/CD Pipeline | `docs/architecture/SECURITY_TESTING_CICD.md` | ✅ |
| Coding Standards | `docs/architecture/STANDARDS_RELEASE.md` | ✅ |
| Branching Strategy | `docs/architecture/STANDARDS_RELEASE.md` | ✅ |
| Release Strategy | `docs/architecture/STANDARDS_RELEASE.md` | ✅ |
| Versioning Strategy | `docs/architecture/STANDARDS_RELEASE.md` | ✅ |
| Dependency Management | `docs/architecture/STANDARDS_RELEASE.md` | ✅ |
| Developer Guide | `docs/guides/developer/getting-started.md` | 📋 Part 3 |
| Build Guide | `docs/guides/developer/build-guide.md` | 📋 Part 3 |
| Contribution Guide | `CONTRIBUTING.md` | 📋 Part 3 |
| User Guide | `docs/guides/user/getting-started.md` | 📋 Part 3 |
| API Reference | Generated via `cargo doc` | 📋 Part 3 |
| FAQ | `docs/guides/user/troubleshooting.md` | 📋 Part 3 |
