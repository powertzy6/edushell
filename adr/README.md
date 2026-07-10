# Architecture Decision Records — EduShell v2.0

## ADR-001: Decouple Cinnamon Incrementally
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: EduShell v1.0 depends on Cinnamon components. Full rewrite is too risky.
- **Decision**: Incrementally replace components per the Replacement Matrix. Each release swaps 2–5 components. Compatibility layer ensures plugins/themes work across versions.
- **Consequences**: Slower decoupling but no breaking changes for users. Dual maintenance burden during transition.

## ADR-002: EduShell Core v2 as Platform Foundation
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: Core v1 is tightly coupled to Cinnamon. Need an independent core.
- **Decision**: Create `edushell-core2` as a standalone Rust library with zero Cinnamon dependency. All new components build atop it. Old components bridge via Compatibility Layer.
- **Consequences**: Clean architecture. Can develop and test independently. Gradual migration.

## ADR-003: Native Rust UI Kit (No JS)
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: Cinnamon uses JS for applets/extensions. JS adds complexity, runtime cost, and security surface.
- **Decision**: All new UI components use Rust + GTK4-rs. UI Kit exposes a pure Rust API. No JavaScript.
- **Consequences**: Steeper learning curve for UI developers, but better performance, type safety, and security.

## ADR-004: Plugin API over Applet API
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: Cinnamon's applet/extension API is JS-based and tightly coupled.
- **Decision**: Replace with Plugin API v2 (Rust traits, WASM-ready). Provide a compatibility shim for existing Cinnamon applets.
- **Consequences**: Plugin developers must rewrite in Rust for full features. Old applets work via shim with reduced capability.

## ADR-005: DBus as Primary IPC
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: Cinnamon uses DBus heavily. Full replacement is impractical in v2.
- **Decision**: Keep DBus for system services (NetworkManager, UPower, BlueZ). New internal communication uses Rust channels + shared memory for performance.
- **Consequences**: Dual IPC strategy. DBus for backward compat. Fast IPC for internal use.

## ADR-006: Migration Engine Required
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: Users have existing configs, themes, plugins. Breaking them is unacceptable.
- **Decision**: Every decoupling step includes a Migration Engine phase that imports old configs and provides rollback.
- **Consequences**: Adds ~15% development overhead per component. Zero user disruption.

## ADR-007: Fork Before Rewrite
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: Critical complex components (Muffin, Settings) cannot be rewritten from scratch.
- **Decision**: Fork first, stabilize as independent project, then incrementally rewrite internals.
- **Consequences**: Immediate independence. Long-term convergence. Parallel tracks.

## ADR-008: Cargo Workspace Monorepo
- **Date**: 2026-07-10
- **Status**: Accepted
- **Context**: Multiple crates need coordinated development and releases.
- **Decision**: Keep single Cargo workspace. Each component is a workspace member. Shared version = workspace.
- **Consequences**: Simple CI. Atomic commits. Single `cargo test --workspace` validates all.
