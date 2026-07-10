# Repository Structure — EduShell

## GitHub Repository Layout

```
edushell/
│
├── .github/                           # GitHub-specific configs
│   ├── workflows/
│   │   ├── ci.yml                     # CI pipeline
│   │   ├── release.yml                # Release workflow
│   │   └── lint.yml                   # Lint workflow
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   ├── feature_request.md
│   │   └── config.yml
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── CODEOWNERS                     # Code ownership
│
├── docs/                              # Documentation
│   ├── README.md                      # Documentation index
│   ├── architecture/                  # Architecture docs (22 documents)
│   ├── adr/                           # Architecture Decision Records
│   ├── guides/                        # User + Developer + Packaging guides
│   ├── specs/                         # Technical specifications
│   └── standards/                     # Coding standards
│
├── src/                               # Source code
│   ├── meson.build                    # Root meson build
│   │
│   ├── shell/                         # Shell components (Layer 1)
│   │   ├── meson.build
│   │   ├── panel/
│   │   │   ├── meson.build
│   │   │   ├── edu-panel.vala
│   │   │   ├── edu-panel-widget.vala
│   │   │   └── edu-panel-style.css
│   │   ├── launcher/
│   │   │   ├── meson.build
│   │   │   ├── edu-launcher.vala
│   │   │   ├── edu-launcher-search.vala
│   │   │   └── edu-launcher-grid.vala
│   │   ├── workspace/
│   │   │   ├── meson.build
│   │   │   └── edu-workspace.vala
│   │   ├── notifications/
│   │   │   ├── meson.build
│   │   │   └── edu-notifications.vala
│   │   ├── tray/
│   │   │   ├── meson.build
│   │   │   └── edu-tray.vala
│   │   ├── quick-settings/
│   │   │   ├── meson.build
│   │   │   └── edu-quick-settings.vala
│   │   ├── user-menu/
│   │   │   ├── meson.build
│   │   │   └── edu-user-menu.vala
│   │   └── osd/
│   │       ├── meson.build
│   │       └── edu-osd.vala
│   │
│   ├── apps/                          # Application components (Layer 2)
│   │   ├── meson.build
│   │   ├── edu-settings/
│   │   │   ├── meson.build
│   │   │   ├── edu-settings.vala      # Main window
│   │   │   ├── pages/
│   │   │   │   ├── panel-page.vala
│   │   │   │   ├── launcher-page.vala
│   │   │   │   ├── theme-page.vala
│   │   │   │   ├── language-page.vala
│   │   │   │   ├── accessibility-page.vala
│   │   │   │   ├── shortcuts-page.vala
│   │   │   │   └── about-page.vala
│   │   │   └── edu-settings-style.css
│   │   ├── learning-hub/
│   │   │   ├── meson.build
│   │   │   ├── edu-learning-hub.vala
│   │   │   └── content/               # Static HTML content
│   │   │       ├── index.html
│   │   │       ├── getting-started.html
│   │   │       ├── tips-and-tricks.html
│   │   │       ├── keyboard-shortcuts.html
│   │   │       └── community.html
│   │   └── edu-tour/                  # First-run tour (v1.x)
│   │       ├── meson.build
│   │       └── edu-tour.vala
│   │
│   ├── lib/                           # Library components (Layer 3)
│   │   ├── meson.build
│   │   ├── edushell-core/
│   │   │   ├── meson.build
│   │   │   ├── config.vala
│   │   │   ├── logging.vala
│   │   │   ├── ipc.vala
│   │   │   └── utils.vala
│   │   ├── cinnamon-adapter/
│   │   │   ├── meson.build
│   │   │   ├── session-adapter.vala
│   │   │   ├── background-adapter.vala
│   │   │   └── keybindings-adapter.vala
│   │   ├── settings-backend/
│   │   │   ├── meson.build
│   │   │   └── settings.vala
│   │   ├── translation/
│   │   │   ├── meson.build
│   │   │   └── i18n.vala
│   │   └── theme-engine/
│   │       ├── meson.build
│   │       ├── theme-manager.vala
│   │       └── theme-loader.vala
│   │
│   ├── bridge/                        # Cinnamon bridge (Layer 4)
│   │   ├── meson.build
│   │   ├── session-bridge/
│   │   │   ├── meson.build
│   │   │   └── session-bridge.vala
│   │   ├── applet-bridge/
│   │   │   ├── meson.build
│   │   │   └── applet-bridge.vala
│   │   └── background-bridge/
│   │       ├── meson.build
│   │       └── background-bridge.vala
│   │
│   └── daemon/                        # Background daemon
│       ├── meson.build
│       └── edushell-daemon.vala
│
├── data/                              # Data files
│   ├── meson.build
│   ├── icons/
│   │   ├── edushell-panel.svg
│   │   ├── edushell-launcher.svg
│   │   └── ...
│   ├── sounds/
│   │   ├── startup.ogg
│   │   ├── notification.ogg
│   │   └── ...
│   ├── wallpapers/
│   │   ├── edushell-default-light.png
│   │   └── edushell-default-dark.png
│   └── gsettings/
│       ├── org.edushell.shell.gschema.xml
│       ├── org.edushell.launcher.gschema.xml
│       └── org.edushell.settings.gschema.xml
│
├── po/                                # Translation files
│   ├── POTFILES.in                    # List of source files with translatable strings
│   ├── edushell.pot                   # Template file
│   ├── id.po                          # Indonesian translation
│   └── en.po                          # English translation
│
├── tests/                             # Test files
│   ├── meson.build
│   ├── unit/
│   │   ├── test-config.vala
│   │   ├── test-logging.vala
│   │   └── test-settings.vala
│   ├── integration/
│   │   ├── test-panel-lifecycle.vala
│   │   ├── test-launcher-search.vala
│   │   └── test-cinnamon-adapter.vala
│   └── manual/
│       ├── keyboard-navigation-test.md
│       └── accessibility-checklist.md
│
├── scripts/                           # Utility scripts
│   ├── meson-build.sh                 # Quick build script
│   ├── install-local.sh               # Local install for testing
│   ├── run-tests.sh                   # Test runner
│   ├── lint-check.sh                  # Lint wrapper
│   └── gen-translation.sh            # Translation helper
│
├── config/                            # Configuration templates
│   ├── edushell.conf                  # Default shell config
│   ├── code-style.cfg                 # Vala code style
│   └── lint-rules.xml                 # Lint rules
│
├── assets/                            # Design assets (source)
│   ├── branding/
│   │   ├── logo.svg
│   │   └── logo.png
│   ├── mockups/                       # UI mockups
│   └── presentations/                 # Design presentations
│
├── .editorconfig                      # Editor settings
├── .gitignore                         # Git ignore rules
├── .gitattributes                     # Git attributes
├── meson.build                        # Root meson build file
├── meson_options.txt                  # Meson build options
├── README.md                          # Project README
├── LICENSE                            # GPL-3.0-or-later
├── CONTRIBUTING.md                    # Contribution guide
├── CODE_OF_CONDUCT.md                 # Code of conduct
├── SECURITY.md                        # Security policy
└── AUTHORS.md                         # Authors list
```

## Directory Purpose Summary

| Directory | Purpose | Audience |
|-----------|---------|----------|
| `.github/` | CI/CD, issue templates | Maintainers |
| `docs/` | All documentation | Users + Developers |
| `src/` | All source code | Developers |
| `data/` | Icons, sounds, wallpapers, schemas | System |
| `po/` | Translation files | Translators |
| `tests/` | All test code | Developers |
| `scripts/` | Build & utility scripts | Developers |
| `config/` | Development configuration | Developers |
| `assets/` | Design source files | Designers |
