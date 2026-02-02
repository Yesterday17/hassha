# hassha

**hassha** (発車, Japanese for "departure") is a Claude Code plugin that plays audio effects at various hook events. Configure different melodies for different projects using JR East departure melodies or custom sounds.

## Features

- **Multi-project support**: Configure different melodies per project using `.hassha/config.toml`
- **Hierarchical configuration**: Config files are resolved by walking up the directory tree
- **64 predefined melodies**: From 6 JR East lines (Yamanote, Keihin-Tohoku, Sobu, Saikyo, Ueno-Tokyo, Narita Express)
- **Custom audio**: Support for URLs and local file paths
- **Smart caching**: Audio files are downloaded once and cached in `~/.hassha/audio/`

## Installation

### Build from source

```bash
# Clone the repository
git clone https://github.com/yesterday17/hassha.git
cd hassha

# Build in release mode
cargo build --release

# The binary is at ./target/release/hassha
```

### Use with Claude Code

Add the plugin to your Claude Code configuration:

```bash
claude --plugin-dir /path/to/hassha
```

Or add it to your settings:

```json
{
  "plugins": ["/path/to/hassha"]
}
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

# Play custom sound on session start
[hooks.SessionStart]
melody = "https://example.com/startup.mp3"

# Play sound after Bash commands
[hooks.PostToolUse]
melody = "NEX-Shinjuku"
matcher = "Bash"  # Only for Bash tool
```

### Supported Hook Events

| Event | Description |
|-------|-------------|
| `Stop` | When Claude finishes responding |
| `Notification` | When Claude needs your attention |
| `SessionStart` | When a Claude session begins |
| `SessionEnd` | When a Claude session ends |
| `PostToolUse` | After a tool executes successfully |

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

| Code | Line Name | Stations |
|------|-----------|----------|
| `JY` | Yamanote Line (山手線) | 30 stations |
| `JK` | Keihin-Tohoku Line (京浜東北線) | 15 stations |
| `JB` | Sobu Line (総武線) | 5 stations |
| `JA` | Saikyo Line (埼京線) | 5 stations |
| `JU` | Ueno-Tokyo Line (上野東京ライン) | 4 stations |
| `NEX` | Narita Express (成田エクスプレス) | 4 stations |

### Sample Melodies

| ID | Line | Station | Melody |
|----|------|---------|--------|
| `JY-Shibuya` | Yamanote | Shibuya | Hana no Horokobi |
| `JY-Takadanobaba` | Yamanote | Takadanobaba | Astro Boy |
| `JY-Akihabara` | Yamanote | Akihabara | Ogawa V1 |
| `JK-Akihabara` | Keihin-Tohoku | Akihabara | Beyond the Line |
| `JK-TakanawaGateway` | Keihin-Tohoku | Takanawa Gateway | Flower Shop |
| `JB-Suidobashi` | Sobu | Suidobashi | Fighting Spirit A |
| `JA-Osaki` | Saikyo | Osaki | Twinkling Skyline |
| `JU-Shimbashi` | Ueno-Tokyo | Shimbashi | Sunlight |
| `NEX-Shinjuku` | Narita Express | Shinjuku | Beautiful Hill |

Run `hassha list` for the complete list of 64 melodies.

## CLI Usage

```bash
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

| Platform | Status | Audio Backend |
|----------|--------|---------------|
| macOS | Supported | `afplay` |
| Linux | Planned | `paplay`/`aplay` |
| Windows | Planned | PowerShell |

## Credits

- Departure melodies from [Yamanotes](https://yamanot.es/) by Morgan Sleeper
- Inspired by the JR East departure melodies in Tokyo, Japan

## License

MIT
