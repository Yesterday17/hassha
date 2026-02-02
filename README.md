# hassha

**hassha** (発車, Japanese for "departure") is a plugin for Claude Code and OpenCode that plays audio effects at various hook events. Configure different melodies for different projects using JR East departure melodies or custom sounds.

## Features

- **Multi-project support**: Configure different melodies per project using `.hassha/config.toml`
- **Hierarchical configuration**: Config files are resolved by walking up the directory tree
- **64 predefined melodies**: From 6 JR East lines (Yamanote, Keihin-Tohoku, Sobu, Saikyo, Ueno-Tokyo, Narita Express)
- **Custom audio**: Support for URLs and local file paths
- **Smart caching**: Audio files are downloaded once and cached in `~/.hassha/audio/`
- **History tracking**: View the last 10 played melodies with `hassha history`
- **Easy installation**: Built-in install command for Claude Code and OpenCode

## Installation

The `hassha` binary is fully self-contained with all assets bundled. You only need the single binary to install.

### Build from source

```bash
# Clone the repository
git clone https://github.com/yesterday17/hassha.git
cd hassha

# Build in release mode
cargo build --release

# The binary is now at ./target/release/hassha
```

### Install for Claude Code

```bash
# The install command extracts all bundled assets
./target/release/hassha install --claude-code
```

This creates the plugin at `~/.claude/plugins/hassha/` with all required files. Follow the hints printed in your terminal for further steps.

### Install for OpenCode

```bash
./target/release/hassha install --open-code
```

This installs the TypeScript plugin to `~/.config/opencode/plugins/` and the binary to `~/.config/opencode/bin/`.

### Uninstall

```bash
# Uninstall from Claude Code
hassha uninstall --claude-code

# Uninstall from OpenCode
hassha uninstall --open-code
```

## Configuration

Create a `.hassha/config.toml` file in your project directory:

```toml
# Play Shibuya melody when Claude finishes responding
[hooks.Stop]
melody = "JY-Shibuya"
volume = 0.8

# Play Akihabara melody on notifications (Keihin-Tohoku Line version)
[hooks.Notification]
melody = "JK-Akihabara"
matcher = "permission_prompt"  # Only for permission prompts

# Play custom sound on session start (only for new sessions)
[hooks.SessionStart]
melody = "https://example.com/startup.mp3"
matcher = "startup"  # Only on new sessions, not resume

# Play sound after Bash commands
[hooks.PostToolUse]
melody = "NEX-Shinjuku"
matcher = "Bash"  # Only for Bash tool

# Play sound when subagent starts
[hooks.SubagentStart]
melody = "JY-Tokyo"
matcher = "Explore"  # Only for Explore agent
```

### Supported Hook Events

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

### Matcher Patterns

The `matcher` field supports:
- **Exact match**: `"Bash"` matches only `"Bash"`
- **Alternatives**: `"Bash|Write"` matches `"Bash"` or `"Write"`
- **Wildcard**: `"*"` matches anything
- **Prefix match**: `"mcp__.*"` matches any MCP tool

### Configuration Resolution

hassha walks up the directory tree to find the nearest `.hassha/config.toml`:

```
/home/user/project/src/components/  <- cwd
/home/user/project/src/             <- not found
/home/user/project/                 <- .hassha/config.toml found!
```

This enables:
- **Monorepo support**: Different subdirectories can have different configs
- **Workspace inheritance**: Put config at workspace root for all packages

## Available Lines & Melodies

Melodies use the format `{LINE}-{Station}`:

### Supported Lines

| Code  | Line Name                         | Stations    |
| ----- | --------------------------------- | ----------- |
| `JY`  | Yamanote Line (山手線)            | 30 stations |
| `JK`  | Keihin-Tohoku Line (京浜東北線)   | 15 stations |
| `JB`  | Sobu Line (総武線)                | 5 stations  |
| `JA`  | Saikyo Line (埼京線)              | 5 stations  |
| `JU`  | Ueno-Tokyo Line (上野東京ライン)  | 4 stations  |
| `NEX` | Narita Express (成田エクスプレス) | 4 stations  |

### Sample Melodies

| ID                   | Line           | Station          | Melody            |
| -------------------- | -------------- | ---------------- | ----------------- |
| `JY-Shibuya`         | Yamanote       | Shibuya          | Hana no Horokobi  |
| `JY-Takadanobaba`    | Yamanote       | Takadanobaba     | Astro Boy         |
| `JY-Akihabara`       | Yamanote       | Akihabara        | Ogawa V1          |
| `JK-Akihabara`       | Keihin-Tohoku  | Akihabara        | Beyond the Line   |
| `JK-TakanawaGateway` | Keihin-Tohoku  | Takanawa Gateway | Flower Shop       |
| `JB-Suidobashi`      | Sobu           | Suidobashi       | Fighting Spirit A |
| `JA-Osaki`           | Saikyo         | Osaki            | Twinkling Skyline |
| `JU-Shimbashi`       | Ueno-Tokyo     | Shimbashi        | Sunlight          |
| `NEX-Shinjuku`       | Narita Express | Shinjuku         | Beautiful Hill    |

Run `hassha list` for the complete list of 64 melodies.

## CLI Usage

```bash
# Install the plugin
hassha install --claude-code    # For Claude Code
hassha install --open-code      # For OpenCode

# Uninstall the plugin
hassha uninstall --claude-code
hassha uninstall --open-code

# List all available melodies
hassha list

# Play a melody directly
hassha play JY-Shibuya
hassha play JK-Akihabara --volume 0.5
hassha play NEX-Shinjuku

# Play from URL
hassha play https://example.com/sound.mp3

# Play from local file
hassha play /path/to/sound.mp3

# View melody history (last 10 played)
hassha history

# Clear melody history
hassha history clear

# Cache management
hassha cache info      # Show cache location and size
hassha cache clear     # Clear all cached audio
hassha cache prefetch  # Download all predefined melodies
```

## Melody Sources

The `melody` field in config can be:

1. **Predefined melody ID**: `"JY-Shibuya"`, `"JK-Akihabara"`, `"NEX-Shinjuku"` - Downloads from yamanot.es
2. **URL**: `"https://example.com/sound.mp3"` - Downloads and caches
3. **Local file path**: `"/path/to/sound.mp3"` - Uses directly

## Platform Support

| Platform | Status    | Audio Backend    |
| -------- | --------- | ---------------- |
| macOS    | Supported | `afplay`         |
| Linux    | Planned   | `paplay`/`aplay` |
| Windows  | Planned   | PowerShell       |

## Credits

- Departure melodies from [Yamanotes](https://yamanot.es/) by Morgan Sleeper
- Inspired by the JR East departure melodies in Tokyo, Japan

## License

MIT
