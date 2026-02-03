# Agent Instructions

This file contains instructions for AI agents working on the hassha codebase.

## Melody Management

When adding or modifying melodies in the codebase:

1. **Update the melody definitions** in `src/melodies.rs`
2. **Update MELODIES.md** to reflect the changes - this file lists all available melodies for users
3. Ensure the melody follows the naming convention: `{LINE}-{Station}` (e.g., `JY-Shibuya`, `JK-Akihabara`)
4. Add appropriate metadata: station name (English and Japanese), line name, and melody name
5. Test the melody with `hassha play {MELODY_ID}`

## Code Style

- Follow Rust idioms and conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Keep documentation up to date

## Testing

- Write tests for new functionality
- Ensure all tests pass with `cargo test`
- Test on macOS (primary platform)
