# hassha Plugin Installation

## Quick Start

### Option 1: Use with Claude Code directly

```bash
# Clone the repository
git clone https://github.com/yesterday17/hassha.git
cd hassha

# Build the plugin
cargo build --release

# Run Claude Code with the plugin
claude --plugin-dir /path/to/hassha
```

### Option 2: Add to Claude Code settings

Add the plugin path to your Claude Code settings file (`~/.claude/settings.json`):

```json
{
  "plugins": [
    "/path/to/hassha"
  ]
}
```

## Build Requirements

- Rust 1.70 or later
- Cargo

## Post-Installation

1. Create a `.hassha/config.toml` file in your project:

```bash
mkdir -p .hassha
cp /path/to/hassha/.claude-plugin/config.example.toml .hassha/config.toml
```

2. Edit the configuration to your preferences

3. (Optional) Prefetch all melodies for offline use:

```bash
/path/to/hassha/target/release/hassha cache prefetch
```

## Verify Installation

Test that the plugin works:

```bash
# List available melodies
/path/to/hassha/target/release/hassha list

# Play a test melody
/path/to/hassha/target/release/hassha play JY-Shibuya
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
chmod +x /path/to/hassha/target/release/hassha
```
