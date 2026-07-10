#!/bin/sh
# EduShell Desktop Session
export XDG_CURRENT_DESKTOP=EduShell
export XDG_SESSION_DESKTOP=EduShell
export PATH="/usr/local/bin:/usr/bin:/bin:$PATH"

# Load user profile
[ -f "$HOME/.xprofile" ] && . "$HOME/.xprofile"

# Start session with dbus + window manager
exec dbus-launch --exit-with-session openbox-session
