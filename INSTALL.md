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

This installs:
- TypeScript plugin to `~/.config/opencode/plugins/hassha.ts`
- Binary to `~/.config/opencode/bin/hassha`

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

## Supported Hook Events

### Claude Code Events

All 12 Claude Code hook events are supported:

| Event                | Description                      | Matcher Values                                                           |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------ |
| `SessionStart`       | When a session begins or resumes | `startup`, `resume`, `clear`, `compact`                                  |
| `UserPromptSubmit`   | When you submit a prompt         | (no matcher support)                                                     |
| `PreToolUse`         | Before a tool call executes      | Tool names: `Bash`, `Edit`, `Write`, `Read`, `mcp__.*`, etc.             |
| `PermissionRequest`  | When a permission dialog appears | Tool names                                                               |
| `PostToolUse`        | After a tool call succeeds       | Tool names                                                               |
| `PostToolUseFailure` | After a tool call fails          | Tool names                                                               |
| `Notification`       | When Claude needs your attention | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog` |
| `SubagentStart`      | When a subagent is spawned       | `Bash`, `Explore`, `Plan`, or custom agent names                         |
| `SubagentStop`       | When a subagent finishes         | Agent names                                                              |
| `Stop`               | When Claude finishes responding  | (no matcher support)                                                     |
| `PreCompact`         | Before context compaction        | `manual`, `auto`                                                         |
| `SessionEnd`         | When a session terminates        | `clear`, `logout`, `prompt_input_exit`, `other`                          |

### OpenCode Events

OpenCode uses a different event system. The hassha plugin maps OpenCode events to Claude Code equivalents:

| OpenCode Event        | Claude Code Equivalent | Description                             |
| --------------------- | ---------------------- | --------------------------------------- |
| `session.created`     | `SessionStart`         | New session started                     |
| `session.deleted`     | `SessionEnd`           | Session terminated                      |
| `session.idle`        | `Stop`                 | Session finished responding             |
| `session.error`       | `Notification`         | An error occurred                       |
| `session.compacted`   | `PreCompact`           | Session was compacted                   |
| `permission.asked`    | `PermissionRequest`    | Permission dialog appeared              |
| `tool.execute.before` | `PreToolUse`           | Before a tool executes                  |
| `tool.execute.after`  | `PostToolUse`          | After a tool succeeds                   |
| `tool.execute.after`  | `PostToolUseFailure`   | After a tool fails (detected by output) |

**Note**: OpenCode doesn't have direct equivalents for `UserPromptSubmit`, `SubagentStart`, or `SubagentStop`.

## Verify Installation

Test that the plugin works:

```bash
# List available melodies
hassha list

# Play a test melody
hassha play JY-Shibuya

# View melody history
hassha history
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
4. Check melody history to see if hooks are firing: `hassha history`

### Permission issues

Ensure the hassha binary is executable:

```bash
# For Claude Code
chmod +x ~/.claude/plugins/hassha/target/release/hassha

# For OpenCode
chmod +x ~/.config/opencode/bin/hassha
```

### OpenCode plugin not loading

1. Ensure dependencies are installed: `cd ~/.config/opencode && bun install`
2. Check that the plugin file exists at `~/.config/opencode/plugins/hassha.ts`
3. Restart OpenCode to reload plugins
