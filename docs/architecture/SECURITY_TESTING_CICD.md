# Security, Testing & CI/CD Strategy — EduShell

---

## 1. Security Strategy

### Design Principles

```
┌─────────────────────────────────────────────────────────────┐
│                    Security Principles                       │
│                                                              │
│  P1. Least Privilege      — Every process runs as user       │
│  P2. Defense in Depth     — Multiple layers of protection   │
│  P3. Fail Secure          — Failure = restricted, not open   │
│  P4. No Secrets in Code   — No passwords, tokens, keys       │
│  P5. Validate All Inputs  — Sanitize at every boundary       │
│  P6. Principle of Least   — Only expose what's needed        │
│      Astonishment                                            │
│  P7. Secure by Default    — Default config is most secure    │
│  P8. Report, Don't        — Crash closed, not leaking data   │
│      Leak                                                    │
└─────────────────────────────────────────────────────────────┘
```

### Security Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Security Layers                          │
│                                                              │
│  Layer 1: OS Security                                        │
│  ├── User namespace (no root)                                │
│  ├── AppArmor/SELinux profile (optional)                     │
│  └── Kernel hardening (default in Ubuntu/Debian)             │
│                                                              │
│  Layer 2: Process Security                                   │
│  ├── Separate processes (no shared memory)                   │
│  ├── No setuid binaries                                      │
│  └── OOM score adjustment (shell protected)                  │
│                                                              │
│  Layer 3: IPC Security                                       │
│  ├── D-Bus session bus (user only)                           │
│  ├── Input validation at service boundaries                  │
│  └── No world-readable IPC endpoints                         │
│                                                              │
│  Layer 4: Storage Security                                   │
│  ├── XDG Base Directory compliance                           │
│  ├── Config files: 600 permission                            │
│  └── No credential storage                                   │
│                                                              │
│  Layer 5: Network Security                                   │
│  ├── No listening sockets (v1)                               │
│  ├── HTTPS-only for Learning Hub content                     │
│  └── No telemetry without explicit consent (v2+)             │
└─────────────────────────────────────────────────────────────┘
```

### Secure Storage

```rust
// v1: No credentials stored. Simple file permissions.
// v2+: Use libsecret (Secret Service API) if credential storage needed

/// File permission helper
pub fn set_secure_permissions(path: &Path) -> Result<(), std::io::Error> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o600); // Owner read/write only
    std::fs::set_permissions(path, perms)
}

/// Validate that a path is within allowed directories
pub fn validate_safe_path(path: &Path) -> Result<(), SecurityError> {
    let allowed = vec![
        dirs::config_dir().unwrap().join("edushell"),
        dirs::data_dir().unwrap().join("edushell"),
        dirs::cache_dir().unwrap().join("edushell"),
    ];

    let canonical = path.canonicalize()
        .map_err(|_| SecurityError::InvalidPath(path.to_path_buf()))?;

    if !allowed.iter().any(|a| canonical.starts_with(a)) {
        return Err(SecurityError::PathOutsideAllowed(canonical));
    }

    Ok(())
}
```

### Input Validation

```rust
// Validate at every IPC boundary
impl EduShellService {
    /// Validate D-Bus method arguments
    fn validate_panel_position(&self, pos: &str) -> Result<PanelPosition, SecurityError> {
        match pos {
            "Bottom" | "Top" | "Left" | "Right" => {
                Ok(serde_json::from_str(&format!("\"{}\"", pos)).unwrap())
            }
            _ => Err(SecurityError::InvalidArgument(
                "panel_position".into(),
                pos.into(),
                "Must be one of: Bottom, Top, Left, Right".into(),
            )),
        }
    }

    /// Sanitize file path from user input
    fn sanitize_path_input(&self, input: &str) -> Result<PathBuf, SecurityError> {
        // Reject path traversal
        if input.contains("..") || input.contains('\0') {
            return Err(SecurityError::PathTraversalDetected(input.into()));
        }

        let path = PathBuf::from(input);

        // Only allow home directory access
        if !path.starts_with(dirs::home_dir().unwrap()) {
            return Err(SecurityError::PathOutsideAllowed(path));
        }

        Ok(path)
    }
}
```

### Security Audit Checklist

| Check | Tool | Frequency |
|-------|------|-----------|
| Dependency vulnerabilities | `cargo audit` | Every build |
| License compliance | `cargo deny check` | Every build |
| Code vulnerabilities | `cargo clippy -- -W clippy::pedantic` | Every build |
| Unsafe code audit | `cargo geiger` | Every release |
| File permission check | Manual script | Pre-release |
| D-Bus interface audit | Manual review | Pre-release |
| No credentials in code | `git secrets` scan | Every commit |

### Security Incident Response

```rust
/// Security event logging (non-sensitive)
pub enum SecurityEvent {
    InvalidInput { source: String, detail: String },
    PathTraversalAttempt { source: String, path: String },
    PermissionDenied { component: String, resource: String },
    RateLimitExceeded { source: String, endpoint: String },
}

impl SecurityEvent {
    pub fn log(&self) {
        match self {
            SecurityEvent::InvalidInput { source, detail } => {
                log::warn!("Security: Invalid input from {}: {}", source, detail);
            }
            SecurityEvent::PathTraversalAttempt { source, path } => {
                log::error!("Security: Path traversal attempt from {}: {}", source, path);
            }
            // ...
        }
    }
}
```

---

## 2. Testing Strategy

### Test Pyramid

```
                    ╱╲
                   ╱  ╲
                  ╱ E2E╲               Manual acceptance tests
                 ╱ Tests╲
                ╱────────╲
               ╱          ╲
              ╱Integration ╲          Cross-component tests
             ╱    Tests     ╲         D-Bus, file system, GTK widget
            ╱────────────────╲
           ╱                  ╲
          ╱   Unit Tests       ╲      Individual functions, pure logic
         ╱                      ╲     Config, error handling, utilities
        ╱────────────────────────╲
       ╱                          ╲
      ╱  Static Analysis           ╲   Clippy, rustfmt, cargo check
     ╱──────────────────────────────╲
```

### Test Categories

```rust
// tests/unit/test_config.rs

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_default_values() {
        let config = EduConfig::default();
        assert_eq!(config.shell.panel_position, PanelPosition::Bottom);
        assert_eq!(config.shell.workspace_count, 4);
        assert!(config.shell.panel_autohide);
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = EduConfig::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialized: EduConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.shell.panel_position, deserialized.shell.panel_position);
    }

    #[test]
    fn test_config_file_io() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        let mut mgr = ConfigManager::with_path(&path);

        mgr.set_panel_position(PanelPosition::Top);
        mgr.save().unwrap();

        let mgr2 = ConfigManager::load_from(&path).unwrap();
        assert_eq!(mgr2.current().shell.panel_position, PanelPosition::Top);
    }

    #[test]
    fn test_config_migration_v1_to_v2() {
        // Test migration from older schema
        let old_config = r#"
            version = "1.0.0"
            [shell]
            panel-position = "Bottom"
        "#;

        let result = migrate_config(old_config, "1.0.0", "2.0.0");
        assert!(result.is_ok());
        // Verify new fields have defaults
        assert_eq!(result.unwrap().shell.panel_opacity, 0.95);
    }
}
```

### Test Coverage Targets

| Layer | Target | Location |
|-------|--------|----------|
| Core library | ≥ 90% | `core/*/src/` |
| Services | ≥ 70% | `services/*/src/` |
| Shell components | ≥ 50% | `shell/*/src/` |
| Apps | ≥ 40% | `apps/*/src/` |
| Integration | Key paths | `tests/integration/` |

### UI Testing Strategy

```rust
// For v1: Manual UI testing with documented checklists
// For v2+: Automated UI testing with gtk4-test or similar

/// Manual test checklist (stored as test file)
// tests/manual/keyboard_navigation.md
//
// # Keyboard Navigation Test
// - [ ] Tab through all panel elements
// - [ ] Super key opens launcher
// - [ ] Arrow keys navigate launcher grid
// - [ ] Esc closes launcher
// - [ ] Super+Tab cycles through windows
// - [ ] Super+Arrow switches workspace
// - [ ] Alt+F4 closes current window
```

### Benchmark Suite

```rust
// tests/benchmarks/launcher_search_bench.rs

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_search_latency(c: &mut Criterion) {
    c.bench_function("launcher_search_1000_results", |b| {
        let search = SearchEngine::new();
        search.index_apps(generate_test_apps(1000));

        b.iter(|| {
            search.search("fire");
        });
    });
}

criterion_group!(benches, bench_search_latency);
criterion_main!(benches);
```

### Test Execution

```bash
# All tests
cargo test --workspace

# Unit tests only
cargo test --lib --workspace

# Integration tests only
cargo test --test '*' --workspace

# Benchmarks
cargo bench --workspace

# With coverage
cargo llvm-cov --workspace --html

# Watch mode (development)
cargo watch -x test
```

---

## 3. CI/CD Pipeline

### Pipeline Architecture

```
                    CI/CD PIPELINE
                    ═══════════════

  Push / PR to main
       │
       ▼
  ┌─────────────────────────────────────────┐
  │  Stage 1: Static Analysis (3min)        │
  │                                         │
  │  ├── cargo fmt --check                  │
  │  ├── cargo clippy --all-targets         │
  │  ├── cargo check                        │
  │  └── cargo deny check                   │
  └──────────────────┬──────────────────────┘
                     │
                     ▼
  ┌─────────────────────────────────────────┐
  │  Stage 2: Build (5min)                  │
  │                                         │
  │  ├── cargo build --release              │
  │  └── cargo build (debug)                │
  └──────────────────┬──────────────────────┘
                     │
                     ▼
  ┌─────────────────────────────────────────┐
  │  Stage 3: Test (8min)                   │
  │                                         │
  │  ├── cargo test --workspace              │
  │  ├── cargo test --test integration       │
  │  ├── cargo bench                         │
  │  └── cargo llvm-cov --html              │
  └──────────────────┬──────────────────────┘
                     │
                     ▼
  ┌─────────────────────────────────────────┐
  │  Stage 4: Security Audit (2min)         │
  │                                         │
  │  ├── cargo audit                        │
  │  └── cargo outdated                     │
  └──────────────────┬──────────────────────┘
                     │
                     ▼
  ┌─────────────────────────────────────────┐
  │  Stage 5: Package (3min)                │
  │                                         │
  │  ├── cargo xtask deb                    │
  │  ├── Generate checksums                 │
  │  └── Upload artifacts                   │
  └──────────────────┬──────────────────────┘
                     │
                     ▼
  ┌─────────────────────────────────────────┐
  │  Stage 6: Documentation (2min)          │
  │                                         │
  │  ├── cargo doc --no-deps                │
  │  └── Deploy to GitHub Pages             │
  └─────────────────────────────────────────┘

  Total: ~23 min (parallelized where possible)
```

### GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  # ── Stage 1: Static Analysis ──
  lint:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          components: rustfmt, clippy
      - run: cargo fmt --check
      - run: cargo clippy --all-targets
      - run: cargo deny check

  # ── Stage 2 & 3: Build & Test ──
  build-and-test:
    runs-on: ubuntu-24.04
    needs: lint
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          components: llvm-tools-preview
      - uses: Swatinem/rust-cache@v2

      - name: Install system dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libgtk-4-dev libgdk-pixbuf-2.0-dev \
            libglib2.0-dev libwayland-dev \
            gettext

      - name: Build
        run: cargo build --release

      - name: Test
        run: cargo test --workspace

      - name: Test (release)
        run: cargo test --workspace --release

      - name: Coverage
        run: cargo llvm-cov --workspace --lcov --output-path lcov.info

      - uses: codecov/codecov-action@v3
        with:
          files: lcov.info

  # ── Stage 4: Security Audit ──
  security-audit:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - uses: actions-rust-lang/audit@v1
      - run: cargo outdated --exit-code 1 || true

  # ── Stage 5: Packaging ──
  package:
    runs-on: ubuntu-24.04
    needs: build-and-test
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo xtask deb
      - uses: actions/upload-artifact@v4
        with:
          name: edushell-deb
          path: target/edushell-*.deb

  # ── Stage 6: Documentation ──
  docs:
    runs-on: ubuntu-24.04
    needs: build-and-test
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo doc --no-deps --workspace
      - uses: actions/upload-pages-artifact@v2
        with:
          path: target/doc
      - uses: actions/deploy-pages@v2
        if: github.ref == 'refs/heads/main'
```

### Release Pipeline

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  release:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo xtask dist

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            target/edushell-*.deb
            target/edushell-*.tar.gz
            target/checksums.txt
          body_path: CHANGELOG.md
          draft: false
          prerelease: ${{ contains(github.ref_name, '-alpha') || contains(github.ref_name, '-beta') }}
```

### Pre-commit Hooks

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.5.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-added-large-files
      - id: check-merge-conflict

  - repo: https://github.com/doublify/pre-commit-rust
    rev: v1.0
    hooks:
      - id: fmt
      - id: clippy
      - id: cargo-check
```

### Build Targets

| Target | Profile | When |
|--------|---------|------|
| x86_64-unknown-linux-gnu | dev | Development |
| x86_64-unknown-linux-gnu | release | Release builds |
| aarch64-unknown-linux-gnu | release | ARM64 (v2+) |
| x86_64-unknown-linux-musl | release | Static binary (optional) |
