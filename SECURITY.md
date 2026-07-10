# Security Policy

This document describes the security practices, vulnerability reporting process, and security features of EduShell.

---

## Supported Versions

| Version       | Supported          | End of Life       |
| ------------- | ------------------ | ----------------- |
| 1.0.x         | Yes                | TBD               |
| < 1.0         | No                 | N/A               |

Security fixes are applied to the latest minor release within a major version. Users are encouraged to always run the latest stable release.

---

## Reporting Vulnerabilities

If you discover a security vulnerability in EduShell, please report it responsibly using the process described below.

### How to Report

1. **Do not** open a public GitHub issue for security vulnerabilities
2. Send an email to the security contact with the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Affected version(s)
   - Potential impact assessment
   - Any suggested fixes (if applicable)
3. You will receive an acknowledgement within **48 hours**
4. A fix will be developed and released as soon as practical
5. You will be credited in the release notes (unless you prefer anonymity)

### What to Include

- **Type of vulnerability:** Buffer overflow, privilege escalation, code injection, denial of service, information disclosure, etc.
- **Attack vector:** Local, network, physical access
- **Affected component:** eduWM, edushell-core, edushell-ui, edushell-daemon, edushell-sdk, plugins
- **Reproduction steps:** Detailed steps to trigger the vulnerability
- **Environment:** Distribution, version, GPU driver, display server configuration

### Response Timeline

| Stage               | Target Timeframe    |
| ------------------- | ------------------- |
| Acknowledgement     | 48 hours            |
| Triage and assessment | 5 business days   |
| Fix development     | 14 business days (critical), 30 business days (other) |
| Public disclosure   | After fix is released |

---

## Security Features in EduShell

### Plugin Sandboxing

EduShell plugins run within the edushell-core process and are subject to the following restrictions:

- **No direct network access** — The plugin SDK does not expose socket or HTTP APIs
- **No subprocess execution** — `std::process::Command` is not available to plugins
- **File system restrictions** — Plugins can only read/write to their designated data directory and read user configuration
- **Permission declarations** — Plugin manifests must declare required permissions; undeclared permissions are denied at load time
- **Memory isolation** — Plugins are loaded as dynamic libraries; memory corruption in a plugin may affect the host process (see Limitations below)

### IPC Security

- **Unix domain sockets** — All inter-component communication uses Unix domain sockets with `0600` permissions (user-only access)
- **Process authentication** — IPC messages are validated against the sender's PID and UID
- **Message validation** — All IPC messages are deserialized and validated before processing; malformed messages are rejected

### Wayland Surface Isolation

- **Shell surface privilege** — The EduShell panel and launcher run on a privileged Wayland layer surface
- **Application isolation** — Application windows cannot capture shell input or read shell state through the Wayland protocol
- **No X11 legacy exposure** — The shell UI is never exposed to XWayland; only legacy applications use XWayland

### Configuration Security

- **System configuration** — Read-only for unprivileged users; managed by the system administrator
- **User configuration** — Stored in `~/.config/edushell/` with restrictive file permissions (`0600`)
- **Validation** — Configuration files are validated on load; invalid configuration is rejected with safe defaults
- **No secrets in config** — The configuration system does not store passwords or API keys

### Session Security

- **Lock screen** — Activated manually (`Super + L`) or automatically after configurable idle timeout
- **Session persistence** — Window layouts are saved to disk with user-only permissions
- **Login integration** — EduShell integrates with systemd-logind for proper session tracking and privilege separation

### Daemon Sandboxing

The `edushell-daemon` process runs under systemd user service sandboxing:

- `PrivateTmp=true` — Isolated `/tmp` namespace
- `ProtectSystem=strict` — Read-only access to `/usr` and `/boot`
- `NoNewPrivileges=true` — Cannot gain additional privileges
- `RestrictNamespaces=true` — Cannot create new namespaces
- `RestrictSUIDSGID=true` — Cannot set SUID/SGID bits
- `MemoryDenyWriteExecute=true` — Cannot create writable-executable memory mappings (where supported)

### Systemd Service Hardening

The systemd service files for edushell-daemon include additional hardening options:

```ini
[Service]
PrivateTmp=true
ProtectSystem=strict
NoNewPrivileges=true
RestrictNamespaces=true
RestrictSUIDSGID=true
MemoryDenyWriteExecute=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
SystemCallFilter=@system-service
SystemCallArchitectures=native
```

---

## Responsible Disclosure Policy

EduShell follows a responsible disclosure policy:

1. **Report privately** — Use the private reporting channel (email) described above
2. **Allow time for a fix** — Do not publicly disclose the vulnerability until a fix is available and released
3. **Coordinate disclosure** — The EduShell team will coordinate with the reporter on the timing of public disclosure
4. **Credit** — Reporters are credited in the release notes unless they request anonymity
5. **No legal action** — The EduShell project will not take legal action against researchers who follow this responsible disclosure process

### Scope

The following are considered in-scope for security reports:

- Vulnerabilities in EduShell's own code (eduWM, edushell-core, edushell-ui, edushell-daemon, edushell-sdk, edushell-cli)
- Security issues in the plugin loading and sandboxing system
- IPC security bypasses
- Wayland surface isolation bypasses
- Configuration file handling vulnerabilities
- Privilege escalation through EduShell components

The following are **out of scope**:

- Vulnerabilities in third-party dependencies (report these to the upstream project)
- Social engineering attacks
- Physical access attacks
- Denial of service through resource exhaustion (unless it causes a security-relevant failure)

---

## Security Auditing

EduShell undergoes the following security practices:

- **Static analysis** — Clippy lints and `cargo audit` are run as part of the CI pipeline
- **Dependency auditing** — `cargo audit` checks for known vulnerabilities in dependencies
- **Code review** — All changes to security-critical code require review
- **Fuzzing** — IPC message parsing and configuration file parsing are fuzz-tested

### Running Security Audits Locally

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit

# Run Clippy with security-relevant lints
cargo clippy --workspace -- -W clippy::all -D clippy::security
```

---

## Contact

For security inquiries, contact the EduShell security team via email. For general questions and non-security bugs, use the public issue tracker.

---

<p align="center"><em>Security is a shared responsibility. If you see something, say something.</em></p>
