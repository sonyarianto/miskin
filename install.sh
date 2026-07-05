#!/usr/bin/env bash
set -e

echo "=== Miskin Installer ==="
echo ""

DEFAULT_INSTALL_DIR="$HOME/.local/bin"
INSTALL_DIR="${MISKIN_INSTALL_DIR:-$DEFAULT_INSTALL_DIR}"

if ! command -v cargo &>/dev/null; then
    echo "Error: cargo not found. Install Rust first: https://rustup.rs"
    exit 1
fi

echo "Building miskin..."
cargo install --path . --root "$HOME/.local" --force

BIN_PATH="$INSTALL_DIR/bin/miskin"
echo ""
echo "Installed: $BIN_PATH"
echo ""

if ! echo "$PATH" | grep -q "$INSTALL_DIR/bin"; then
    echo "Add to your shell config:"
    echo "  export PATH=\"$INSTALL_DIR/bin:\$PATH\""
    echo ""
fi

echo "Run 'miskin init' to install hooks for your AI tools."
echo "  miskin init              # Claude Code (default)"
echo "  miskin init --agent cursor"
echo "  miskin init --agent copilot"
echo "  miskin init --agent gemini"
echo "  miskin init --agent codex"
echo "  miskin init --agent opencode"
