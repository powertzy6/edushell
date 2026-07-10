# Security Evolution Plan — EduShell v2.0+

## Principles
- Defense in depth
- Least privilege
- Secure by default
- No plaintext secrets
- Sandbox all untrusted code

## v2.0 (Current)
- [x] Plugin permission model defined (`ExtensionPermission`)
- [x] API version checking (manifest requires `api_version` match)
- [x] Crash isolation (extension crashes don't take down shell)
- [x] SPDX license headers on all source files

## v2.2
- [ ] Plugin sandboxing via Linux user namespaces
- [ ] Seccomp filters for plugin processes
- [ ] Configuration file encryption (keyring-backed)
- [ ] Secure IPC between shell and plugins (Cap'n Proto / socket with auth)

## v2.5
- [ ] Signed package verification (GPG)
- [ ] Plugin marketplace with automated security scanning
- [ ] Permission revocation UI
- [ ] Audit logging for sensitive operations

## v3.0
- [ ] Full Wayland security model (no X11 fallback)
- [ ] Screen casting permission dialog
- [ ] Input capture permission
- [ ] Sandboxed file picker (portal API)

## v4.0+
- [ ] Mandatory Access Control (AppArmor profiles shipped with package)
- [ ] Fuzzing infrastructure in CI
- [ ] Regular third-party security audit
- [ ] Bug bounty program
