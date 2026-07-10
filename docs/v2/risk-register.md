# Risk Register — EduShell v2.0 Decoupling

| # | Risk | Probability | Impact | Mitigation | Owner |
|---|------|-----------|--------|------------|-------|
| R1 | Muffin fork diverges and becomes unmaintainable | Medium | Critical | Regular upstream rebase; modular architecture limits blast radius | WM team |
| R2 | Cinnamon Settings fork misses upstream security fixes | Medium | High | Automated CVE monitoring; merge bot | Security |
| R3 | Plugin authors don't migrate from JS to Rust | High | Medium | Compatibility layer keeps old plugins running indefinitely | DevRel |
| R4 | Performance regression during decoupling | Medium | High | Perf benchmarks in CI; alert on >10% regression | QA |
| R5 | Migration Engine fails on edge-case configs | Low | High | Fuzz testing on real user configs; rollback always available | QA |
| R6 | Wayland-only future breaks X11 users | Medium | High | X11 fallback via XWayland; support both until v4.0 | WM team |
| R7 | Community fragmentation (fork vs. mainline) | Low | Medium | Clear governance; open decision-making via ADRs | OSPO |
| R8 | Key developer burnout | Medium | High | Sustainable pace; rotating responsibilities; mentor new contributors | Maintainers |

## Risk Response Plan
- **R1, R2**: Weekly sync with upstream. Automated merge bot.
- **R3**: Documentation, examples, migration guide, CLI scaffolding.
- **R4**: `criterion` benchmarks in CI. Threshold: ±10% = alert, ±20% = block.
- **R5**: Rollback always possible via MigrationEngine. Test with 1000+ synthetic configs.
- **R6**: X11 support documented as deprecated from v2.5. Removal in v4.0.
- **R7**: All major decisions recorded as ADRs. Public discussion period.
- **R8**: `CONTRIBUTING.md` defines bus factor mitigation. No single point of failure.
