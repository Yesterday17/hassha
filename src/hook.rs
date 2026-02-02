//! Hook event handler for hassha.
//!
//! Parses stdin JSON from Claude Code hooks and plays the appropriate melody.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::cache::resolve_melody_path;
use crate::config::{get_hook_config, load_config};
use crate::player::play_audio;

/// Input received from Claude Code hooks via stdin
#[derive(Debug, Deserialize)]
pub struct HookInput {
    /// The current working directory
    pub cwd: PathBuf,

    /// The hook event name
    #[allow(dead_code)]
    pub hook_event_name: String,

    /// Session ID
    #[serde(default)]
    #[allow(dead_code)]
    pub session_id: Option<String>,

    /// Tool name (for PostToolUse events)
    #[serde(default)]
    pub tool_name: Option<String>,

    // Other fields we don't need but might be present
    #[serde(flatten)]
    pub _extra: serde_json::Value,
}

/// Read and parse hook input from stdin
pub fn read_hook_input() -> Result<HookInput> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    // Handle empty stdin gracefully
    if buffer.trim().is_empty() {
        anyhow::bail!("No input received on stdin");
    }

    let input: HookInput =
        serde_json::from_str(&buffer).context("Failed to parse hook input JSON")?;

    Ok(input)
}

/// Handle a hook event
pub fn handle_hook(event_name: &str) -> Result<()> {
    // Read input from stdin
    let input = read_hook_input()?;

    // Load configuration from the project directory
    let config = match load_config(&input.cwd)? {
        Some(config) => config,
        None => {
            // No configuration found, silently exit
            return Ok(());
        }
    };

    // Get hook configuration for this event
    let hook_config = match get_hook_config(&config, event_name) {
        Some(config) => config,
        None => {
            // No configuration for this event, silently exit
            return Ok(());
        }
    };

    // Check matcher for PostToolUse events
    if event_name == "PostToolUse" {
        if let Some(matcher) = &hook_config.matcher {
            let tool_name = input.tool_name.as_deref().unwrap_or("");

            // Simple pattern matching (supports exact match or regex-like patterns)
            if !matches_pattern(matcher, tool_name) {
                return Ok(());
            }
        }
    }

    // Resolve melody to a local file path
    let audio_path = resolve_melody_path(&hook_config.melody)?;

    // Play the audio
    play_audio(&audio_path, hook_config.volume)?;

    Ok(())
}

/// Simple pattern matching for tool names
/// Supports:
/// - Exact match: "Bash" matches "Bash"
/// - Pipe-separated alternatives: "Bash|Write" matches "Bash" or "Write"
fn matches_pattern(pattern: &str, value: &str) -> bool {
    if pattern.contains('|') {
        pattern.split('|').any(|p| p.trim() == value)
    } else {
        pattern == value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_pattern() {
        assert!(matches_pattern("Bash", "Bash"));
        assert!(!matches_pattern("Bash", "Write"));
        assert!(matches_pattern("Bash|Write", "Bash"));
        assert!(matches_pattern("Bash|Write", "Write"));
        assert!(!matches_pattern("Bash|Write", "Read"));
    }

    #[test]
    fn test_parse_hook_input() {
        let json = r#"{
            "cwd": "/home/user/project",
            "hook_event_name": "Stop",
            "session_id": "abc123"
        }"#;

        let input: HookInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.cwd, PathBuf::from("/home/user/project"));
        assert_eq!(input.hook_event_name, "Stop");
    }
}
