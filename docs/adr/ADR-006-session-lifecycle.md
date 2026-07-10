# ADR-006: Session Lifecycle melalui cinnamon-session

**Status**: Proposed
**Version**: v1
**Date**: 2026-07

## Context
EduShell membutuhkan session manager untuk:
1. Startup sequence (shell → panel → components)
2. Shutdown sequence (graceful component termination)
3. Crash recovery (restart panel if crash)
4. Suspend/resume handling
5. User switching

## Decision
Gunakan **cinnamon-session** sebagai session manager untuk v1.

### Integration
EduShell akan:
1. Register sebagai komponen cinnamon-session via `.desktop` file autostart
2. Gunakan `org.cinnamon.SessionManager` DBus interface untuk lifecycle events
3. EduShell Panel akan menjadi komponen utama dengan `--replace` capability

### Startup Sequence
```
cinnamon-session start
  │
  ├── muffin (window manager)
  │
  ├── edushell-panel (main shell)
  ├── edushell-settings-daemon
  ├── edushell-daemon (background services)
  │
  ├── cinnamon-killer-daemon (crash recovery)
  │
  └── user applications (autostart)
```

### Shutdown Sequence
```
User clicks "Shutdown" in EduUserMenu
  │
  ├── edushell-panel → notify all components → save state
  ├── cinnamon-session → QueryRunning → confirm
  ├── Terminate applications
  └── System poweroff (via logind)
```

### Crash Recovery
- `cinnamon-killer-daemon` akan memonitor `edushell-panel`
- Jika panel crash: restart otomatis dalam < 1 detik
- Jika panel crash berulang (> 3 kali dalam 60 detik): fallback ke Cinnamon panel

### Consequences
**Positive**:
- Tidak perlu build session manager dari awal
- Stabilitas terjamin (cinnamon-session sudah mature)
- Crash recovery built-in

**Negative**:
- Terikat pada cinnamon-session API
- Perlu adaptasi jika cinnamon-session berubah

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| **systemd --user** | Terlalu low-level. Tidak ada session lifecycle management. |
| **gnome-session** | Tidak kompatibel dengan Cinnamon ecosystem. |
| **Custom session manager** | Terlalu banyak kerja untuk v1. Akan dibangun di v3. |
