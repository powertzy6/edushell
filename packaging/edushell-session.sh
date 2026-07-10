#!/bin/bash
# EduShell Session — v1.0.0 Full Desktop
# EduShell Desktop Environment session launcher

export XDG_SESSION_TYPE=x11
export XDG_CURRENT_DESKTOP=EduShell
export XDG_SESSION_DESKTOP=EduShell
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$HOME/.local/share"

export PATH="$PATH:/usr/bin:/usr/local/bin:/bin"

[ -f /etc/profile ] && . /etc/profile
[ -f "$HOME/.profile" ] && . "$HOME/.profile"
[ -f "$HOME/.xprofile" ] && . "$HOME/.xprofile"

# Start dbus if not running
if ! pgrep -x dbus-daemon >/dev/null; then
    dbus-launch --exit-with-session &
fi

# Wallpaper
feh --bg-fill "$HOME/.local/share/edushell/wallpaper.svg" 2>/dev/null || true

# Compositor (transparency & effects)
if command -v picom &>/dev/null; then
    picom --backend glx --vsync --fade-in-step=0.03 --fade-out-step=0.03 &
fi

# Panel
if command -v tint2 &>/dev/null; then
    tint2 &
fi

# Network manager tray
if command -v nm-applet &>/dev/null; then
    nm-applet &
fi

# Power manager
if command -v xfce4-power-manager &>/dev/null; then
    xfce4-power-manager &
fi

# Window Manager — EduShell branded Openbox session
exec openbox --startup "$@"
