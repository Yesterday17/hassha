#!/bin/bash
set -e

# hassha installation script
# Usage: ./install.sh [install_dir]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="${1:-$HOME/.local/bin}"
PLUGIN_DIR="${2:-$HOME/.claude/plugins/hassha}"

echo "==================================="
echo "  hassha - Claude Code Audio Hooks"
echo "==================================="
echo ""

# Check for Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is required but not installed."
    echo "Install it from https://rustup.rs/"
    exit 1
fi

# Build the project
echo "Building hassha..."
cd "$SCRIPT_DIR"
cargo build --release

# Create install directory
mkdir -p "$INSTALL_DIR"
mkdir -p "$PLUGIN_DIR"

# Copy binary
echo "Installing binary to $INSTALL_DIR/hassha..."
cp "$SCRIPT_DIR/target/release/hassha" "$INSTALL_DIR/hassha"
chmod +x "$INSTALL_DIR/hassha"

# Copy plugin files
echo "Installing plugin to $PLUGIN_DIR..."
cp -r "$SCRIPT_DIR/.claude-plugin" "$PLUGIN_DIR/"
cp -r "$SCRIPT_DIR/hooks" "$PLUGIN_DIR/"
mkdir -p "$PLUGIN_DIR/target/release"
cp "$SCRIPT_DIR/target/release/hassha" "$PLUGIN_DIR/target/release/hassha"

echo ""
echo "Installation complete!"
echo ""
echo "Next steps:"
echo ""
echo "1. Add hassha to your PATH (if not already):"
echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
echo ""
echo "2. Add the plugin to Claude Code:"
echo "   claude --plugin-dir $PLUGIN_DIR"
echo ""
echo "   Or add to ~/.claude/settings.json:"
echo "   {"
echo "     \"plugins\": [\"$PLUGIN_DIR\"]"
echo "   }"
echo ""
echo "3. Create a config file in your project:"
echo "   mkdir -p .hassha"
echo "   cp $PLUGIN_DIR/.claude-plugin/config.example.toml .hassha/config.toml"
echo ""
echo "4. Test it works:"
echo "   hassha list"
echo "   hassha play JY-Shibuya"
echo ""
