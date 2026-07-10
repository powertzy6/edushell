#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "=== EduShell Development Script ==="
echo ""

case "${1:-help}" in
    build)
        cargo build --workspace "$@"
        ;;
    test)
        cargo test -p edushell-core2 -p edushell-sdk -p edushell-cli -p eduwm "$@"
        ;;
    check)
        cargo check -p edushell-core2 -p edushell-sdk -p edushell-cli -p edushell-apps -p eduwm "$@"
        ;;
    lint)
        cargo fmt --all --check
        cargo clippy -p edushell-core2 -p edushell-sdk -p edushell-cli -p edushell-apps -p eduwm -- -D warnings
        ;;
    docs)
        cargo doc --no-deps -p edushell-core2 -p edushell-sdk -p edushell-cli -p eduwm "$@"
        echo "Docs built at target/doc/"
        ;;
    clean)
        cargo clean
        ;;
    release)
        cargo build --release -p edushell-core2 -p edushell-sdk -p edushell-cli -p edushell-apps -p eduwm
        ;;
    install)
        sudo bash "${SCRIPT_DIR}/install.sh"
        ;;
    help|*)
        echo "Usage: $0 <command>"
        echo ""
        echo "Commands:"
        echo "  build    Build all workspace crates"
        echo "  test     Run tests (non-GTK crates)"
        echo "  check    Check compilation"
        echo "  lint     Run rustfmt and clippy"
        echo "  docs     Build documentation"
        echo "  clean    Clean build artifacts"
        echo "  release  Build release binaries"
        echo "  install  Install EduShell system-wide"
        echo "  help     Show this help"
        ;;
esac
