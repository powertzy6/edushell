# Compatibility Targets — EduShell v1

## Distribution Compatibility

| Distribution | Version | Status | Notes |
|-------------|---------|--------|-------|
| **Ubuntu** | 24.04 LTS (Noble) | Target | Primary target, Wayland ready |
| **Ubuntu** | 26.04 LTS | Target | Future LTS |
| **Linux Mint** | 22.x (Wilma) | Target | Cinnamon native, easiest integration |
| **Linux Mint** | 23.x | Target | Future |
| **Debian** | 12 (Bookworm) | Target | Needs backports for GTK4|
| **Debian** | 13 (Trixie) | Target | Better GTK4 support |
| **Pop!_OS** | 24.04 | Compatible | Via Ubuntu base |
| **elementary OS** | 8.x | Compatible | Test recommended |

## Display Server Compatibility

| Server | Status | Limitations |
|--------|--------|-------------|
| **Wayland** | Primary | Full features via wlr-layer-shell protocol |
| **X11 (Xorg)** | Fallback | Terbatas, tanpa fitur Wayland-only |

### Wayland Protocol Requirements
| Protocol | Purpose | Status |
|----------|---------|--------|
| wlr-layer-shell | Panel positioning | Required |
| wlr-foreign-toplevel | Window management | Required |
| zwlr-screencopy | Screenshot (future) | v2+ |
| xdg-shell | Client windows | Required via GTK4 |

## Graphics Compatibility

| Vendor | Driver | Status | Notes |
|--------|--------|--------|-------|
| Intel | i915 (in-kernel) | Full | Best support |
| AMD | amdgpu (in-kernel) | Full | |
| NVIDIA | nouveau (open) | Basic | No Wayland on legacy |
| NVIDIA | nvidia (proprietary) | Not supported v1 | Re-evaluate v3+ |
| VirtIO | virtio-gpu | Full | VM support |

## Input Device Compatibility

| Device | Protocol | Status |
|--------|----------|--------|
| Standard USB keyboard | evdev | Full |
| Bluetooth keyboard | BlueZ + evdev | Full |
| Touchpad (Synaptics, ELAN) | libinput | Full |
| Touchpad (multitouch) | libinput gestures | Full |
| Touchscreen | libinput | Full |
| Stylus / Pen | libinput + Wacom | Basic v1 |
| Gamepad | Not supported v1 | v2+ |

## Accessibility Software Compatibility

| Software | Purpose | Status |
|----------|---------|--------|
| Orca | Screen reader | Compatible |
| GNOME Accessibility Toolkit | AT-SPI2 | Compatible |
| eSpeak-NG | TTS engine | Compatible |
| Onboard | On-screen keyboard | Basic compatibility v1 |
