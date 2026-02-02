//! Hook event handler for hassha.
//!
//! Parses stdin JSON from Claude Code hooks and plays the appropriate melody.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::cache::resolve_melody_path;
use crate::config::{get_hook_config, load_config};
use crate::history::add_history_entry;
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

    /// Tool name (for tool-related events: PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest)
    #[serde(default)]
    pub tool_name: Option<String>,

    /// Source (for SessionStart: startup, resume, clear, compact)
    #[serde(default)]
    pub source: Option<String>,

    /// Notification type (for Notification events)
    #[serde(default)]
    pub notification_type: Option<String>,

    /// Agent type (for SubagentStart, SubagentStop)
    #[serde(default)]
    pub agent_type: Option<String>,

    /// Reason (for SessionEnd)
    #[serde(default)]
    pub reason: Option<String>,

    /// Trigger (for PreCompact: manual, auto)
    #[serde(default)]
    pub trigger: Option<String>,

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

/// Get the matcher value from the hook input based on event type
fn get_matcher_value(event_name: &str, input: &HookInput) -> Option<String> {
    match event_name {
        // Tool-based events match on tool_name
        "PreToolUse" | "PostToolUse" | "PostToolUseFailure" | "PermissionRequest" => {
            input.tool_name.clone()
        }
        // SessionStart matches on source (startup, resume, clear, compact)
        "SessionStart" => input.source.clone(),
        // SessionEnd matches on reason
        "SessionEnd" => input.reason.clone(),
        // Notification matches on notification_type
        "Notification" => input.notification_type.clone(),
        // Subagent events match on agent_type
        "SubagentStart" | "SubagentStop" => input.agent_type.clone(),
        // PreCompact matches on trigger (manual, auto)
        "PreCompact" => input.trigger.clone(),
        // UserPromptSubmit and Stop don't support matchers
        _ => None,
    }
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

    // Check matcher if configured
    if let Some(matcher) = &hook_config.matcher {
        let matcher_value = get_matcher_value(event_name, &input);
        let value = matcher_value.as_deref().unwrap_or("");

        if !matches_pattern(matcher, value) {
            return Ok(());
        }
    }

    // Resolve melody to a local file path
    let audio_path = resolve_melody_path(&hook_config.melody)?;

    // Play the audio
    play_audio(&audio_path, hook_config.volume)?;

    // Log to history (ignore errors - history is non-critical)
    let _ = add_history_entry(
        event_name,
        &hook_config.melody,
        &input.cwd.to_string_lossy(),
        input.tool_name.as_deref(),
        hook_config.matcher.as_deref(),
    );

    Ok(())
}

/// Simple pattern matching for matcher values
/// Supports:
/// - Exact match: "Bash" matches "Bash"
/// - Pipe-separated alternatives: "Bash|Write" matches "Bash" or "Write"
/// - Wildcard: "*" matches anything
/// - Regex-like prefix: "mcp__.*" matches "mcp__memory__create"
fn matches_pattern(pattern: &str, value: &str) -> bool {
    // Wildcard matches everything
    if pattern == "*" {
        return true;
    }

    // Pipe-separated alternatives
    if pattern.contains('|') {
        return pattern
            .split('|')
            .any(|p| matches_single_pattern(p.trim(), value));
    }

    matches_single_pattern(pattern, value)
}

/// Match a single pattern (no pipes)
fn matches_single_pattern(pattern: &str, value: &str) -> bool {
    // Simple regex-like suffix matching for ".*"
    if pattern.ends_with(".*") {
        let prefix = &pattern[..pattern.len() - 2];
        return value.starts_with(prefix);
    }

    // Exact match
    pattern == value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_pattern() {
        // Exact match
        assert!(matches_pattern("Bash", "Bash"));
        assert!(!matches_pattern("Bash", "Write"));

        // Pipe-separated
        assert!(matches_pattern("Bash|Write", "Bash"));
        assert!(matches_pattern("Bash|Write", "Write"));
        assert!(!matches_pattern("Bash|Write", "Read"));

        // Wildcard
        assert!(matches_pattern("*", "Bash"));
        assert!(matches_pattern("*", "anything"));

        // Prefix with .*
        assert!(matches_pattern("mcp__.*", "mcp__memory__create"));
        assert!(matches_pattern("mcp__.*", "mcp__github__search"));
        assert!(!matches_pattern("mcp__.*", "Bash"));

        // Combined
        assert!(matches_pattern("Bash|mcp__.*", "Bash"));
        assert!(matches_pattern("Bash|mcp__.*", "mcp__test__tool"));
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

    #[test]
    fn test_parse_tool_hook_input() {
        let json = r#"{
            "cwd": "/home/user/project",
            "hook_event_name": "PostToolUse",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test"}
        }"#;

        let input: HookInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.tool_name, Some("Bash".to_string()));
    }

    #[test]
    fn test_parse_session_start_input() {
        let json = r#"{
            "cwd": "/home/user/project",
            "hook_event_name": "SessionStart",
            "source": "startup"
        }"#;

        let input: HookInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.source, Some("startup".to_string()));
    }
}
