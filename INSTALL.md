# hassha Plugin Installation

The `hassha` binary is fully self-contained with all assets bundled. You only need the single binary to install.

## Quick Start

### Build from source

```bash
# Clone the repository
git clone https://github.com/yesterday17/hassha.git
cd hassha

# Build the plugin
cargo build --release

# The binary is at ./target/release/hassha
```

### Install for Claude Code

```bash
./target/release/hassha install --claude-code
```

This creates a complete plugin installation at `~/.claude/plugins/hassha/`.

Then add the plugin to your Claude Code settings (`~/.claude/settings.json`):

```json
{
  "plugins": [
    "~/.claude/plugins/hassha"
  ]
}
```

### Install for OpenCode

```bash
./target/release/hassha install --open-code
```

This creates a complete plugin installation at `~/.opencode/plugins/hassha/`.

## Build Requirements

- Rust 1.70 or later
- Cargo

## Post-Installation

1. Create a `.hassha/config.toml` file in your project:

```bash
mkdir -p .hassha
cat > .hassha/config.toml << 'EOF'
[hooks.Stop]
melody = "JY-Shibuya"
volume = 0.8
EOF
```

2. (Optional) Prefetch all melodies for offline use:

```bash
hassha cache prefetch
```

## Verify Installation

Test that the plugin works:

```bash
# List available melodies
hassha list

# Play a test melody
hassha play JY-Shibuya
```

## Uninstall

```bash
# Uninstall from Claude Code
hassha uninstall --claude-code

# Uninstall from OpenCode
hassha uninstall --open-code
```

## Troubleshooting

### No sound plays

1. Check that your system volume is not muted
2. On macOS, ensure `afplay` is available (it's included by default)
3. Verify the melody was downloaded: `hassha cache info`

### Hook not triggering

1. Ensure `.hassha/config.toml` exists in your project or a parent directory
2. Check the hook event name is correct (case-sensitive)
3. For `PostToolUse`, verify the `matcher` pattern matches the tool name

### Permission issues

Ensure the hassha binary is executable:

```bash
chmod +x ~/.claude/plugins/hassha/target/release/hassha
```
