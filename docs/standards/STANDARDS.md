# Coding & Project Standards — EduShell

## 1. File Naming Convention

### Source Files
| Language | Convention | Example |
|----------|-----------|---------|
| Vala | `kebab-case.vala` | `edu-panel.vala`, `edu-launcher-search.vala` |
| C | `snake_case.c` / `snake_case.h` | `cinnamon_adapter.c` |
| Python | `snake_case.py` | `translation_helper.py` |
| Rust | `snake_case.rs` | `search_index.rs` |
| CSS/SCSS | `kebab-case.css` | `edu-panel-style.css` |
| Meson | `meson.build` (exact) | `meson.build` |

### Test Files
| Type | Convention | Example |
|------|-----------|---------|
| Unit test | `test-<component>.vala` | `test-panel.vala` |
| Integration test | `test-<feature>.vala` | `test-launcher-search.vala` |

### Data Files
| Type | Convention | Example |
|------|-----------|---------|
| GSettings schema | `org.edushell.<domain>.gschema.xml` | `org.edushell.shell.gschema.xml` |
| Translation template | `<project>.pot` | `edushell.pot` |
| Translation file | `<lang>.po` | `id.po`, `en.po` |
| Icon | `kebab-case.svg` | `edushell-panel.svg` |
| Wallpaper | `edushell-<variant>.png` | `edushell-default-light.png` |

### Documentation Files
| Type | Convention | Example |
|------|-----------|---------|
| Architecture docs | `UPPER_CASE.md` | `VISION.md`, `ROADMAP.md` |
| Guides | `kebab-case.md` | `getting-started.md` |
| Specs | `kebab-case-spec.md` | `panel-spec.md` |
| ADR | `ADR-<number>-<topic>.md` | `ADR-001-vala-gtk4.md` |

---

## 2. Folder Naming Convention

| Pattern | Example |
|---------|---------|
| Source directories: `kebab-case` | `src/shell/panel/`, `src/apps/edu-settings/` |
| Test directories: match source | `tests/unit/`, `tests/integration/` |
| Data directories: plural nouns | `data/icons/`, `data/wallpapers/`, `data/sounds/` |
| Documentation directories: plural | `docs/guides/`, `docs/specs/` |

---

## 3. Branch Naming Convention

| Branch Type | Pattern | Example |
|-------------|---------|---------|
| Main | `main` | `main` |
| Develop | `develop` | `develop` |
| Feature | `feat/<short-description>` | `feat/panel-autohide` |
| Bugfix | `fix/<short-description>` | `fix/launcher-search-crash` |
| Hotfix | `hotfix/<short-description>` | `hotfix/memory-leak-panel` |
| Release | `release/v<version>` | `release/v1.0.0` |
| Chore | `chore/<short-description>` | `chore/update-deps` |
| Documentation | `docs/<short-description>` | `docs/translation-guide` |
| Refactor | `refactor/<short-description>` | `refactor/panel-layout` |

Rules:
- Use `/` as separator
- Use `kebab-case` after prefix
- Maximum 50 characters for branch name
- No trailing slashes or dots

---

## 4. Commit Message Convention

Follow **Conventional Commits** specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types
| Type | Usage |
|------|-------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Code style (formatting, semicolons) |
| `refactor` | Code change that neither fixes nor adds |
| `perf` | Performance improvement |
| `test` | Adding or fixing tests |
| `chore` | Build process, dependencies, tooling |
| `ci` | CI/CD changes |
| `i18n` | Translation changes |

### Scope Values
| Scope | Component |
|-------|-----------|
| `panel` | EduPanel |
| `launcher` | EduLauncher |
| `workspace` | EduWorkspace |
| `notifications` | EduNotifications |
| `tray` | EduTray |
| `quick-settings` | EduQuickSettings |
| `user-menu` | EduUserMenu |
| `osd` | EduOSD |
| `settings` | EduSettings app |
| `learning-hub` | LearningHub |
| `core` | libedushell-core |
| `adapter` | libcinnamon-adapter |
| `theme` | Theme engine |
| `i18n` | Translation |
| `docs` | Documentation |
| `ci` | CI/CD |
| `build` | Build system |
| `*` | Multiple components |

### Examples
```
feat(panel): add autohide functionality

Implement panel autohide on mouse leave with configurable delay.
User can enable/disable via EduSettings.

Closes #42
```

```
fix(launcher): fix crash when search index is empty

Add null check for search results before rendering grid.

Fixes #87
```

```
docs(arch): add ADR-009 for component replacement strategy
```

```
i18n(po): update Indonesian translation for EduSettings
```

### Rules
- **Maximum**: 72 characters for subject, 72 per line for body
- **Language**: English for subject and body
- **Imperative**: "feat: add" not "feat: added"
- **No period** at end of subject line
- **Footer**: `Closes #N`, `Fixes #N`, `Refs #N`
- **One commit per logical change**

---

## 5. Release Version Convention

Follow **Semantic Versioning 2.0.0**:

```
v<MAJOR>.<MINOR>.<PATCH>[-<pre-release>]
```

### Rules
| Increment | When |
|-----------|------|
| **MAJOR** | Incompatible API changes (v1→v2) |
| **MINOR** | New functionality (backward compatible) |
| **PATCH** | Bug fixes (backward compatible) |

### Pre-release Tags
| Tag | Usage |
|-----|-------|
| `-alpha.N` | Internal testing |
| `-beta.N` | Public testing |
| `-rc.N` | Release candidate |

### Examples
```
v1.0.0-alpha.1
v1.0.0-beta.1
v1.0.0-rc.1
v1.0.0          # Stable release
v1.1.0          # Minor feature release
v1.1.1          # Patch release
v2.0.0          # Major architectural release
```

### Git Tags
- All releases MUST be tagged: `git tag -a v1.0.0 -m "EduShell v1.0.0"`
- Tags MUST be signed: `git tag -s v1.0.0 -m "EduShell v1.0.0"`
- Pre-releases: `v1.0.0-alpha.1`, `v1.0.0-beta.1`

---

## 6. Coding Standards

### Vala
| Standard | Rule |
|----------|------|
| Indentation | 4 spaces (no tabs) |
| Line length | 100 characters max |
| Braces | Same line (K&R style) |
| Naming (classes) | `PascalCase` |
| Naming (methods) | `snake_case` |
| Naming (variables) | `snake_case` |
| Naming (constants) | `UPPER_SNAKE_CASE` |
| Naming (signals) | `snake_case` |
| Naming (properties) | `snake_case` |
| Visibility | Explicit: `public`, `private`, `protected` |
| File header | License header in every file |
| Null safety | Use nullable types explicitly (`string?`) |

### Python
| Standard | Rule |
|----------|------|
| Style | PEP 8 |
| Indentation | 4 spaces |
| Line length | 88 characters (Black default) |
| Type hints | Required for all public functions |
| Docstrings | Google style |
| Formatting | Black + isort |

### CSS / SCSS
| Standard | Rule |
|----------|------|
| Indentation | 2 spaces |
| Selector naming | BEM-like: `.edu-panel__button--active` |
| Colors | Use CSS variables for theme |
| Units | `rem` for fonts, `px` for borders, `%` for widths |
| Nesting | Max 3 levels deep |

---

## 7. Code Quality Gates

| Gate | Requirement | Tool |
|------|-------------|------|
| Build | Pass with 0 warnings | `meson compile` with -Werror |
| Lint | 0 errors, 0 warnings | `valac --lint`, `black --check` |
| Test | 100% pass rate | `meson test` |
| Coverage | Domain ≥ 70% | `gcov` / `lcov` |
| Complexity | ≤ 10 per function | `lizard` |
| Duplication | < 5% | `dupl` |
| File length | ≤ 500 lines | `cloc` |
