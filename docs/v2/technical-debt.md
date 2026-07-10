# Technical Debt Report — EduShell v1.0 → v2.0

## Critical Debt (must fix before v2.0 GA)

| Item | Module | Impact | Fix |
|------|--------|--------|-----|
| GTK4 system dep blocks CI | `edushell-ui`, `edushell-core` | Cannot test in isolated env | Add `--no-default-features` for headless CI |
| Cinnamon dependency audit incomplete before | `edushell-apps` | Hidden coupling | Done — see audit in docs/v2 |
| No integration test across modules | `edushell-apps::integration` | Regression risk | Add cross-module tests |
| Test timeout on full suite | Test binary | Developer friction | Split into per-crate CI jobs |

## Medium Debt

| Item | Module | Impact | Target |
|------|--------|--------|--------|
| Duplicated `now_iso()` | Multiple modules | Code smell | v2.2: centralize in `core` |
| Some modules use `unwrap()` | Various | Panic risk | v2.2: replace with proper error handling |
| No benchmark suite | — | Performance regression risk | v2.2: add `criterion` benchmarks |
| `Lock` poisoning recovery | Test code | Subtle test failures | v2.2: improve test isolation |

## Low Debt

| Item | Target |
|------|--------|
| Inline docs missing for ~5% of pub items | v2.5 |
| No fuzz testing | v3.0 |
| No property-based testing | v3.0 |

## Debt Ratio
- Total lines: ~28,000
- `// TODO` / `// FIXME` count: ~12
- Debt ratio: < 0.1% (healthy)
