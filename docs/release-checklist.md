# EduShell v1.0 — Release Checklist

## Pre-Release
- [ ] All crates compile without errors
- [ ] All crates compile without warnings
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Security audit clean (`cargo audit`)
- [ ] Dependency audit clean (`cargo deny`)
- [ ] License compliance verified (SPDX headers present)
- [ ] Performance meets targets (startup < 2s, search < 100ms)
- [ ] Memory profile acceptable (< 256 MB idle)

## Packaging
- [ ] Debian package builds clean
- [ ] Package installs clean on Ubuntu LTS
- [ ] Package installs clean on Linux Mint
- [ ] Package installs clean on Debian Stable
- [ ] Desktop entry registered
- [ ] Session file registered
- [ ] Icons installed
- [ ] Post-install script runs without error
- [ ] Pre-remove script runs without error
- [ ] Upgrade from previous version works
- [ ] Rollback from new version works
- [ ] Uninstall removes all files

## SDK & API
- [ ] SDK crate compiles
- [ ] CLI tool works (`edushell version`)
- [ ] Plugin API manifest validation works
- [ ] Theme SDK documentation complete
- [ ] Widget SDK documentation complete
- [ ] Search Provider SDK documentation complete

## Documentation
- [ ] Getting Started guide published
- [ ] Installation guide published
- [ ] Architecture overview published
- [ ] API reference published
- [ ] SDK reference published
- [ ] Plugin development guide published
- [ ] Theme development guide published
- [ ] Contributing guide published
- [ ] FAQ published

## Release Artifacts
- [ ] Source tarball created
- [ ] Debian .deb package created
- [ ] Checksums computed (SHA-256)
- [ ] GPG signature created
- [ ] GitHub release tag created
- [ ] Release notes written
- [ ] Changelog updated
