# Installing EduShell

This guide covers installing EduShell v1.0.0 as a desktop session on supported Linux distributions.

---

## Supported Distributions

| Distribution            | Version   | Status       |
| ----------------------- | --------- | ------------ |
| Ubuntu Budgie           | 26.04     | Fully tested |
| Ubuntu                  | 26.04     | Fully tested |
| Linux Mint              | 22+       | Fully tested |
| Debian                  | Stable    | Supported    |

Other distributions with systemd, Wayland, and GTK4/libadwaita support should work but are not officially tested.

---

## Installation Methods

### Method 1: Install Script (Recommended)

The install script handles building, packaging, and system integration in one step.

#### Prerequisites

- Rust 1.80+ (see [BUILD.md](BUILD.md))
- Root privileges (`sudo`)
- System dependencies installed (see [BUILD.md](BUILD.md))

#### Steps

1. Clone or extract the EduShell source:

```bash
cd /home/izza/Documents/DS
```

2. Run the install script:

```bash
sudo ./install.sh
```

The script will:

- Build EduShell in release mode
- Install binaries to `/usr/bin/`
- Install libraries to `/usr/lib/edushell/`
- Install desktop session files to `/usr/share/xsessions/` and `/usr/share/wayland-sessions/`
- Install themes and assets to `/usr/share/edushell/`
- Install the EduWM session entry
- Update the desktop database

3. Log out and select **EduShell** from your display manager's session menu.

#### Custom Install Prefix

To install to a custom prefix (e.g. for local installs):

```bash
sudo ./install.sh --prefix=/opt/edushell
```

#### Unattended Install

For scripted/automated deployments:

```bash
sudo ./install.sh --yes
```

---

### Method 2: Debian Package

A pre-built `.deb` package is provided for Debian-based distributions.

#### Install the Package

```bash
sudo dpkg -i target/debian/edushell_1.0.0_amd64.deb
sudo apt-get install -f   # resolve any missing dependencies
```

#### Verify Installation

```bash
dpkg -l | grep edushell
```

#### Upgrade from Previous Version

```bash
sudo dpkg -i target/debian/edushell_1.0.1_amd64.deb
```

`dpkg` will automatically handle the upgrade, replacing old files.

---

### Method 3: Manual Installation

For developers or custom setups:

```bash
# Build release
cargo build --workspace --release

# Install binaries
sudo install -m 755 target/release/eduwm /usr/bin/
sudo install -m 755 target/release/edushell-ui /usr/bin/
sudo install -m 755 target/release/edushell-cli /usr/bin/
sudo install -m 755 target/release/edushell-daemon /usr/bin/

# Install libraries
sudo install -m 644 target/release/libedushell_core.so /usr/lib/edushell/
sudo install -m 644 target/release/libedushell_sdk.rlib /usr/lib/edushell/

# Install session files
sudo install -m 644 data/edushell.desktop /usr/share/wayland-sessions/
sudo install -m 644 data/eduwm.desktop /usr/share/wayland-sessions/

# Install assets
sudo cp -r themes /usr/share/edushell/themes
sudo cp -r plugins /usr/share/edushell/plugins

# Update desktop database
sudo update-desktop-database /usr/share/wayland-sessions/
```

---

## Selecting EduShell at Login

After installation, EduShell appears as a session option in your display manager.

### GDM (GNOME Display Manager)

1. At the login screen, click your username
2. Click the gear icon in the bottom-right corner
3. Select **EduShell** from the session list
4. Enter your password and log in

### LightDM

1. At the login screen, click the session dropdown (top bar)
2. Select **EduShell**
3. Log in normally

### SDDM

1. At the login screen, click the session selector (bottom-left)
2. Select **EduShell**
3. Log in

### Command-Line Session Start

You can also start EduShell from a TTY or existing session:

```bash
# From a TTY (replaces current session)
exec eduwm

# From an existing Wayland session
eduwm --nested
```

---

## Post-Install Configuration

### Verify Installation

```bash
edushell-cli --version
eduwm --version
```

### First Launch Setup

On first launch, EduShell will:

1. Create configuration directories in `~/.config/edushell/`
2. Generate default settings
3. Apply the default theme (Adwaita Dark)
4. Set up workspace defaults (4 workspaces)

### Theme Configuration

Default themes are installed to `/usr/share/edushell/themes/`. User themes can be placed in `~/.config/edushell/themes/`.

### Panel Position

Right-click the panel or use Settings to change:
- Position: top, bottom, left, right
- Size: small, medium, large
- Auto-hide: on/off

### Keyboard Shortcuts

All keyboard shortcuts can be customized via Settings > Keyboard > Shortcuts. See [USER_GUIDE.md](USER_GUIDE.md) for the default shortcut reference.

### Multi-Monitor

EduShell detects connected monitors on launch. Configure per-monitor settings in Settings > Display.

---

## Uninstallation

### Using the Install Script

```bash
sudo ./install.sh --uninstall
```

### Using the Debian Package

```bash
sudo dpkg -r edushell
```

### Manual Uninstallation

```bash
# Remove binaries
sudo rm /usr/bin/eduwm
sudo rm /usr/bin/edushell-ui
sudo rm /usr/bin/edushell-cli
sudo rm /usr/bin/edushell-daemon

# Remove libraries
sudo rm -rf /usr/lib/edushell/

# Remove session files
sudo rm /usr/share/wayland-sessions/edushell.desktop
sudo rm /usr/share/wayland-sessions/eduwm.desktop

# Remove assets
sudo rm -rf /usr/share/edushell/

# Remove user configuration
rm -rf ~/.config/edushell/
rm -rf ~/.cache/edushell/

# Update desktop database
sudo update-desktop-database /usr/share/wayland-sessions/
```

---

## Troubleshooting

### EduShell Does Not Appear in Session List

- Verify session files exist: `ls /usr/share/wayland-sessions/`
- Run `sudo update-desktop-database /usr/share/wayland-sessions/`
- Log out completely and back in (not just lock/unlock)

### Black Screen on Launch

- Ensure Wayland is supported by your GPU driver
- Check logs: `journalctl --user -u edushell-daemon`
- Try launching from TTY: `exec eduwm`
- Verify EduWM binary exists: `which eduwm`

### Panel or Launcher Missing

- Check that the edushell-daemon is running: `ps aux | grep edushell-daemon`
- Restart the daemon: `systemctl --user restart edushell-daemon`
- Reinstall assets: `sudo ./install.sh`

### Missing GTK4 / libadwaita Libraries

EduShell requires GTK4 and libadwaita at runtime. Install them:

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

### Permission Denied on Launch

Ensure the EduWM binary is executable:

```bash
sudo chmod +x /usr/bin/eduwm
```

### Display Manager Not Showing EduShell

Some display managers cache session lists. Restart your display manager:

```bash
sudo systemctl restart gdm    # or lightdm, sddm
```

### GPU Driver Issues

EduShell requires a working Wayland-capable GPU driver. Common drivers:

- **NVIDIA:** Ensure `nvidia-driver-560+` with Wayland support enabled
- **Intel:** `mesa-vulkan-drivers` and `intel-media-driver`
- **AMD:** `mesa-vulkan-drivers` and `mesa-va-drivers`

Enable NVIDIA Wayland mode:

```bash
sudo nano /etc/gdm3/custom.conf
# Uncomment: WaylandEnable=true
```

### Logs and Debugging

Enable verbose logging:

```bash
EDUSHELL_LOG=debug eduwm
```

View daemon logs:

```bash
journalctl --user -f -u edushell-daemon
```

View system-wide logs:

```bash
sudo journalctl -f -u edushell-system
```
