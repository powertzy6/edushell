# EduShell Desktop Fix — Full Install
# Copy & paste semua baris satu per satu

# 1. Install komponen desktop
sudo apt update
sudo apt install -y tint2 picom feh network-manager-gnome pavucontrol xdg-utils

# 2. Buat folder config
mkdir -p ~/.config/openbox
mkdir -p ~/.config/tint2
mkdir -p ~/.config/edushell
mkdir -p ~/.local/share/applications

# 3. Konfigurasi Openbox
cat > ~/.config/openbox/rc.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<openbox_config xmlns="http://openbox.org/3.4/rc">
  <resistance>
    <strength>10</strength>
    <corner>20</corner>
  </resistance>
  <menu>
    <file>~/.config/openbox/menu.xml</file>
    <hideDelay>200</hideDelay>
    <middle>yes</middle>
  </menu>
  <keyboard>
    <keybind key="W-space">
      <action name="Execute"><command>xfce4-terminal</command></action>
    </keybind>
    <keybind key="W-d">
      <action name="ToggleShowDesktop"/>
    </keybind>
    <keybind key="W-e">
      <action name="Execute"><command>thunar || pcmanfm || xdg-open .</command></action>
    </keybind>
    <keybind key="W-r">
      <action name="Execute"><command>xfce4-appfinder || rofi || dmenu_run</command></action>
    </keybind>
    <keybind key="W-Tab">
      <action name="NextWindow"/>
    </keybind>
    <keybind key="A-F4">
      <action name="Close"/>
    </keybind>
    <keybind key="A-Tab">
      <action name="NextWindow"/>
    </keybind>
    <keybind key="W-1">
      <action name="GoToDesktop"><to>1</to></action>
    </keybind>
    <keybind key="W-2">
      <action name="GoToDesktop"><to>2</to></action>
    </keybind>
    <keybind key="W-3">
      <action name="GoToDesktop"><to>3</to></action>
    </keybind>
    <keybind key="W-4">
      <action name="GoToDesktop"><to>4</to></action>
    </keybind>
  </keyboard>
  <mouse>
    <context name="Root">
      <mousebind button="Right" action="Press">
        <action name="ShowMenu"><menu>root-menu</menu></action>
      </mousebind>
    </context>
  </mouse>
  <desktops>
    <number>4</number>
    <names><name>1</name><name>2</name><name>3</name><name>4</name></names>
  </desktops>
  <resize><popupShow>Never</popupShow></resize>
  <theme>
    <name>Onyx</name>
    <titleLayout>NLIMC</titleLayout>
    <keepBorder>yes</keepBorder>
    <animateIconify>no</animateIconify>
    <font place="ActiveWindow">
      <name>sans</name><size>10</size><weight>bold</weight>
    </font>
  </theme>
</openbox_config>
EOF

# 4. Menu Openbox
cat > ~/.config/openbox/menu.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<openbox_menu xmlns="http://openbox.org/3.4/menu">
  <menu id="root-menu" label="EduShell">
    <item label="Terminal">
      <action name="Execute"><command>xfce4-terminal</command></action>
    </item>
    <item label="File Manager">
      <action name="Execute"><command>thunar || pcmanfm || xdg-open .</command></action>
    </item>
    <item label="Web Browser">
      <action name="Execute"><command>xdg-open https://google.com</command></action>
    </item>
    <separator/>
    <item label="Settings">
      <action name="Execute"><command>xfce4-settings-manager || gnome-control-center</command></action>
    </item>
    <separator/>
    <item label="Lock Screen">
      <action name="Execute"><command>xdg-screensaver lock || gnome-screensaver-command -l</command></action>
    </item>
    <item label="Log Out">
      <action name="Exit"/>
    </item>
    <item label="Reboot">
      <action name="Execute"><command>systemctl reboot</command></action>
    </item>
    <item label="Shutdown">
      <action name="Execute"><command>systemctl poweroff</command></action>
    </item>
  </menu>
</openbox_menu>
EOF

# 5. Konfigurasi tint2 panel
cat > ~/.config/tint2/tint2rc << 'EOF'
# EduShell Panel — tint2 config
panel_items = LTSBC
panel_size = 100% 40
panel_margin = 0 0
panel_padding = 4 4 4
panel_background_id = 1
wm_menu = 0

# Taskbar
taskbar_mode = single_desktop
taskbar_padding = 4 4 4
task_background_id = 0

# Clock
time1_format = %H:%M
time1_font = sans 10
clock_padding = 4 4
clock_background_id = 0

# System tray
systray_padding = 4 4 4
systray_background_id = 0

# Backgrounds
rounded = 0
border_width = 0
background_color = #222222 85

# Launcher
launcher_item_app = ~/.local/share/applications/edushell-terminal.desktop
launcher_item_app = ~/.local/share/applications/edushell-browser.desktop

# Battery
battery_tooltip = 1
battery_hide = 98

# Executor
execp = new
execp_command = date +%a\ %d\ %b
execp_interval = 60
execp_font = sans 9
EOF

# 6. Launcher entries
cat > ~/.local/share/applications/edushell-terminal.desktop << 'EOF'
[Desktop Entry]
Type=Application
Name=Terminal
Exec=xfce4-terminal
Icon=utilities-terminal
EOF

cat > ~/.local/share/applications/edushell-browser.desktop << 'EOF'
[Desktop Entry]
Type=Application
Name=Browser
Exec=xdg-open https://google.com
Icon=web-browser
EOF

# 7. Wallpaper default (warna biru solid)
mkdir -p ~/.local/share/edushell
cat > ~/.local/share/edushell/wallpaper.svg << 'WALL'
<svg xmlns="http://www.w3.org/2000/svg" width="1920" height="1080">
  <defs>
    <linearGradient id="g" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#1a1a2e"/>
      <stop offset="50%" style="stop-color:#16213e"/>
      <stop offset="100%" style="stop-color:#0f3460"/>
    </linearGradient>
  </defs>
  <rect width="1920" height="1080" fill="url(#g)"/>
  <text x="960" y="540" font-family="sans" font-size="48" fill="#ffffff" opacity="0.3" text-anchor="middle" dominant-baseline="middle">EduShell</text>
</svg>
WALL

echo "=== KONFIGURASI SELESAI ==="
echo "Sekarang jalankan: sudo cp ~/Documents/DS/packaging/edushell-session.sh /usr/local/bin/edushell-session"
echo "Lalu logout dan pilih EduShell"
