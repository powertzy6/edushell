# Building EduShell

This document covers prerequisites, build commands, feature flags, testing, cross-compilation, and Docker-based reproducible builds for EduShell v1.0.0.

---

## Prerequisites

### Rust Toolchain

Install Rust 1.80 or later via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

Verify:

```bash
rustc --version   # 1.80.0 or newer
cargo --version
```

### System Dependencies

EduShell depends on several system libraries. Install them for your distribution:

#### Ubuntu / Ubuntu Budgie 26.04

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  cmake \
  libwayland-dev \
  wayland-protocols \
  libgtk-4-dev \
  libadwaita-1-dev \
  libglib2.0-dev \
  libpango1.0-dev \
  libcairo2-dev \
  libgdk-pixbuf-2.0-dev \
  libinput-dev \
  libudev-dev \
  libxkbcommon-dev \
  libseat-dev \
  libpixman-1-dev \
  libdrm-dev \
  libgbm-dev \
  libegl-dev \
  libvulkan-dev \
  libdbus-1-dev \
  libssl-dev \
  libsystemd-dev
```

#### Linux Mint 22+

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  cmake \
  libwayland-dev \
  wayland-protocols \
  libgtk-4-dev \
  libadwaita-1-dev \
  libglib2.0-dev \
  libpango1.0-dev \
  libcairo2-dev \
  libgdk-pixbuf-2.0-dev \
  libinput-dev \
  libudev-dev \
  libxkbcommon-dev \
  libseat-dev \
  libpixman-1-dev \
  libdrm-dev \
  libgbm-dev \
  libegl-dev \
  libvulkan-dev \
  libdbus-1-dev \
  libssl-dev
```

#### Debian Stable

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  cmake \
  libwayland-dev \
  wayland-protocols \
  libgtk-4-dev \
  libadwaita-1-dev \
  libglib2.0-dev \
  libpango1.0-dev \
  libcairo2-dev \
  libgdk-pixbuf-2.0-dev \
  libinput-dev \
  libudev-dev \
  libxkbcommon-dev \
  libseat-dev \
  libpixman-1-dev \
  libdrm-dev \
  libgbm-dev \
  libegl-dev \
  libvulkan-dev \
  libdbus-1-dev \
  libssl-dev
```

> **Note:** Debian Stable may ship older GTK4/libadwaita versions. EduShell is tested against the versions in Ubuntu 26.04. Some features may not compile on older releases.

---

## Building

### Using the Dev Script

The fastest way to build EduShell:

```bash
./scripts/dev.sh build
```

This runs a full workspace build with default features.

### Using Cargo Directly

#### Debug Build

```bash
cargo build --workspace
```

#### Release Build (Optimized)

```bash
cargo build --workspace --release
```

Release builds apply full optimizations (`opt-level = 3`, LTO where configured) and are suitable for production deployment.

### Build a Specific Crate

```bash
cargo build -p eduwm
cargo build -p edushell-core
cargo build -p edushell-sdk
```

---

## Build Options and Features

EduShell crates expose optional feature flags. Use `--features` to enable them:

```bash
cargo build --workspace --features "vulkan,experimental-plugins"
```

### Available Features

| Feature                | Crates                    | Description                                     |
| ---------------------- | ------------------------- | ----------------------------------------------- |
| `vulkan`               | eduwm                     | Enable Vulkan rendering backend                 |
| `xwayland`             | eduwm                     | XWayland compatibility for legacy X11 apps      |
| `experimental-plugins` | edushell-sdk              | Enable experimental plugin API surface          |
| `debug-ipc`            | edushell-core             | Verbose IPC logging for development             |
| ` cinnamon-compat`     | edushell-core, edushell-ui| Cinnamon extension compatibility layer          |
| `wayland-debug`        | eduwm                     | Wayland protocol debugging output               |

Check available features for a crate:

```bash
cargo metadata --no-deps --format-version 1 | jq '.packages[].features'
```

---

## Testing

### Full Test Suite

```bash
cargo test --workspace
```

### Per-Crate Tests

```bash
cargo test -p eduwm
cargo test -p edushell-core
cargo test -p edushell-ui
cargo test -p edushell-sdk
cargo test -p edushell-cli
cargo test -p edushell-daemon
```

### Linting

```bash
cargo clippy --workspace -- -D warnings
```

### Formatting

```bash
cargo fmt --all -- --check
```

### Running All Checks

```bash
./scripts/dev.sh check
```

---

## Cross-Compilation

EduShell supports cross-compilation for other Linux targets. Add the target first:

```bash
# Example: ARM64 (aarch64)
rustup target add aarch64-unknown-linux-gnu
```

Install the cross-compilation toolchain and linkers for the target platform, then:

```bash
cargo build --workspace --release --target aarch64-unknown-linux-gnu
```

> **Note:** Cross-compiling GTK4-based crates requires the target's development libraries to be available in a sysroot or via a cross-compilation toolchain. This is most reliably done inside a Docker container with the target's rootfs.

For multi-arch Docker builds, see the Docker section below.

---

## Docker-Based Build

A Dockerfile is provided for reproducible, environment-independent builds:

```bash
docker build -t edushell-build .
```

### Build Inside Container

```bash
docker run --rm -v "$(pwd)":/src -w /src edushell-build cargo build --workspace --release
```

### Extract Release Artifacts

```bash
docker run --rm -v "$(pwd)/target":/src/target edushell-build cp /src/target/release/eduwm /src/target/release/
```

### Using Docker Compose

If a `docker-compose.yml` is present:

```bash
docker compose run --rm build
```

The Docker image includes all required system dependencies and is based on Ubuntu 26.04 for maximum compatibility.

---

## Build Output Locations

| Artifact                  | Debug Path                              | Release Path                              |
| ------------------------- | --------------------------------------- | ----------------------------------------- |
| eduwm binary              | `target/debug/eduwm`                   | `target/release/eduwm`                   |
| edushell-core library     | `target/debug/libedushell_core.so`      | `target/release/libedushell_core.so`      |
| edushell-ui binary        | `target/debug/edushell-ui`             | `target/release/edushell-ui`             |
| edushell-cli binary       | `target/debug/edushell-cli`            | `target/release/edushell-cli`            |
| edushell-daemon binary    | `target/debug/edushell-daemon`         | `target/release/edushell-daemon`         |
| edushell-sdk library      | `target/debug/libedushell_sdk.rlib`     | `target/release/libedushell_sdk.rlib`     |
| Compiled plugins          | `target/debug/plugins/`                 | `target/release/plugins/`                 |
| Debian package            | `target/debian/edushell_1.0.0_amd64.deb` | `target/debian/edushell_1.0.0_amd64.deb` |

---

## Troubleshooting

### Missing GTK4 Libraries

If you see errors like `could not find specification for -lgtk-4`:

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

### Wayland Protocol Errors

Ensure `wayland-protocols` is installed and up to date:

```bash
sudo apt install wayland-protocols
pkg-config --modversion wayland-protocols
```

### Out of Memory During Linking

Release builds with LTO can consume significant memory. Reduce parallelism:

```bash
CARGO_BUILD_JOBS=4 cargo build --workspace --release
```

### Stale Build Artifacts

A clean rebuild can resolve most stale artifact issues:

```bash
cargo clean
cargo build --workspace
```
