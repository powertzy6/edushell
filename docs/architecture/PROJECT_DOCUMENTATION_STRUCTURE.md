# Project Documentation Structure — EduShell

## Documentation Tree

```
docs/
├── README.md                          # Documentation index & navigation
│
├── architecture/                      # Architectural documentation
│   ├── VISION.md                      # Vision document
│   ├── MISSION.md                     # Mission document
│   ├── PHILOSOPHY.md                  # Philosophy
│   ├── DESIGN_PRINCIPLES.md           # Design principles
│   ├── ENGINEERING_PRINCIPLES.md      # Engineering principles
│   ├── USER_PERSONAS.md               # User personas
│   ├── FUNCTIONAL_REQUIREMENTS.md     # Functional requirements
│   ├── NON_FUNCTIONAL_REQUIREMENTS.md # Non-functional requirements
│   ├── TECHNICAL_CONSTRAINTS.md       # Technical constraints
│   ├── HARDWARE_TARGETS.md            # Hardware targets
│   ├── RESOURCE_TARGETS.md            # Resource targets
│   ├── COMPATIBILITY_TARGETS.md       # Compatibility targets
│   ├── ROADMAP.md                     # v1-v5 roadmap
│   ├── RISK_ANALYSIS.md              # Risk analysis
│   ├── SUCCESS_METRICS.md             # Success metrics
│   ├── ARCHITECTURE_OVERVIEW.md       # Architecture overview
│   ├── COMPONENT_CATALOG.md           # Full component catalog
│   ├── CINNAMON_COMPATIBILITY_MATRIX.md # Cinnamon dependency mapping
│   ├── PROJECT_DOCUMENTATION_STRUCTURE.md  # This file
│   └── REPOSITORY_STRUCTURE.md        # Repository structure
│
├── adr/                               # Architecture Decision Records
│   ├── INDEX.md                       # ADR index
│   ├── ADR-001-vala-gtk4.md          # Language & toolkit choice
│   ├── ADR-002-cinnamon-base.md       # Cinnamon as base
│   ├── ADR-003-wayland-x11.md         # Display server strategy
│   ├── ADR-004-meson-build.md         # Build system
│   ├── ADR-005-gsettings-config.md    # Configuration management
│   ├── ADR-006-session-lifecycle.md   # Session management
│   ├── ADR-007-translation-system.md  # Translation system
│   ├── ADR-008-plugin-architecture.md # Plugin system (v3+)
│   └── ADR-009-component-replacement-strategy.md  # Replacement roadmap
│
├── guides/                            # User and developer guides
│   ├── user/
│   │   ├── getting-started.md         # First-time user guide
│   │   ├── panel-guide.md             # Panel usage
│   │   ├── launcher-guide.md          # Launcher usage
│   │   ├── settings-guide.md          # Settings guide
│   │   ├── keyboard-shortcuts.md      # Shortcut reference
│   │   └── troubleshooting.md         # Common issues
│   │
│   ├── developer/
│   │   ├── getting-started.md         # Development setup
│   │   ├── build-guide.md             # Build instructions
│   │   ├── architecture.md            # Developer architecture overview
│   │   ├── component-guide.md         # How to create a component
│   │   ├── translation-guide.md       # How to translate
│   │   ├── theme-guide.md             # How to create a theme
│   │   ├── testing-guide.md           # Testing guidelines
│   │   └── contributing.md            # Contribution guide
│   │
│   └── packaging/
│       ├── debian-packaging.md        # .deb packaging guide
│       └── ci-cd-guide.md             # CI/CD pipeline guide
│
├── specs/                             # Technical specifications
│   ├── shell-panel-spec.md            # EduPanel specification
│   ├── launcher-spec.md               # EduLauncher specification
│   ├── settings-spec.md               # EduSettings specification
│   ├── notification-spec.md           # Notification system spec
│   ├── theme-spec.md                  # Theme specification
│   ├── keyboard-nav-spec.md           # Keyboard navigation spec
│   └── api-spec.md                    # Internal API specification
│
└── standards/                         # Coding standards
    └── STANDARDS.md                   # Naming, branching, commit, version standards
```

## Documentation Principles

1. **Bahasa Indonesia** untuk dokumentasi pengguna
2. **English** untuk dokumentasi teknis dan pengembang
3. **Architecture decisions** dalam Bahasa Indonesia dan English (bilingual)
4. **Setiap komponen** memiliki spesifikasi teknis di `docs/specs/`
5. **Setiap ADR** ditulis sebelum implementasi dimulai

## Documentation Quality Requirements

| Standard | Requirement |
|----------|-------------|
| Spelling | Zero errors (id + en) |
| Diagrams | Mermaid.js for architecture diagrams |
| Code examples | Syntax highlighted, tested |
| Links | No dead links; CI checks |
| Version | Documentation version matches software version |
