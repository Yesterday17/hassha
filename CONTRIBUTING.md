# Contributing to hassha

Thank you for your interest in contributing to hassha!

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yesterday17/hassha.git
   cd hassha
   ```

2. Build in development mode:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

## Project Structure

```
hassha/
├── .claude-plugin/       # Plugin metadata
│   ├── plugin.json       # Plugin manifest
│   ├── config.example.toml
│   └── config.schema.json
├── hooks/
│   └── hooks.json        # Hook definitions
├── src/
│   ├── main.rs           # CLI entry point
│   ├── cli.rs            # Argument parsing
│   ├── config.rs         # TOML config handling
│   ├── cache.rs          # Audio file caching
│   ├── hook.rs           # Hook event handler
│   ├── melodies.rs       # Melody registry
│   └── player.rs         # Audio playback
└── ...
```

## Adding New Features

### Adding a New Hook Event

1. Add the event to `hooks/hooks.json`
2. Update the `HookInput` struct in `src/hook.rs` if needed
3. Add event handling logic in `handle_hook()`
4. Update documentation

### Adding a New Melody

Edit `src/melodies.rs` and add to the `MELODIES` array:

```rust
MelodyInfo {
    id: "XX-StationName",
    station: "Station Name",
    station_jp: "駅名",
    melody_name: "Melody Name",
    filename: "filename.mp3",
},
```

### Adding Platform Support

Edit `src/player.rs`:

1. Add a new `#[cfg(target_os = "...")]` block
2. Implement the `play_audio_*` function
3. Update the main `play_audio()` function

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` to check for issues
- Add tests for new functionality

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Commit with a clear message
7. Push and create a Pull Request

## Reporting Issues

When reporting issues, please include:

- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Error messages if any

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
