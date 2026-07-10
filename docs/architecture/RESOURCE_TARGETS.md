# Resource Targets — EduShell v1

## Memory Targets (RSS)

| State | Target | Hard Limit | Notes |
|-------|--------|------------|-------|
| Idle (fresh boot, no apps) | 500 MB | 650 MB | Setelah login, panel + shell loaded |
| Normal usage (browser + office) | 900 MB | 1200 MB | EduShell + user apps |
| Heavy usage (many apps) | 1500 MB | 2000 MB | EduShell portion should stay < 700 MB |

## CPU Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Idle (average over 60s) | < 1% | On baseline CPU |
| Launcher open/close | < 5% spike | < 200ms duration |
| Launcher search (real-time) | < 10% | While typing |
| Notification popup | < 3% spike | < 500ms duration |
| Workspace switch | < 8% spike | < 300ms duration |
| Panel update (clock, tray) | < 0.5% | Background |

## Storage Targets

| Component | Size Target | Notes |
|-----------|-------------|-------|
| EduShell core binaries | < 5 MB | Compiled Vala |
| EduShell themes | < 3 MB | CSS + assets |
| EduShell config defaults | < 100 KB | |
| Learning Hub content | < 2 MB | Static HTML/markdown |
| Log files (7 days) | < 5 MB | Rotated |
| Translation files | < 500 KB | .mo files |

## Startup Time Targets

| Phase | Target | Notes |
|-------|--------|-------|
| Login Manager → Desktop | < 10 seconds | Total from SDDM/LightDM to usable desktop |
| Shell process start | < 1 second | From session start to panel visible |
| First launcher open | < 500ms | Including search index warmup |

## Network Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Learning Hub load (local) | < 100ms | From storage |
| Learning Hub load (online) | < 3 seconds | If content needs update |
| Check for updates | < 5 seconds | Background, non-blocking |

## Thermal & Power Targets

| Condition | Target | Notes |
|-----------|--------|-------|
| CPU temperature at idle | < 50°C | On baseline hardware |
| Battery impact | < 0.5W | EduShell portion only |
| Fan activation | Never at idle | On passive-cooled devices |
