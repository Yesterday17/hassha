//! History tracking for hassha melody playback.
//!
//! Maintains a log of the last N played melodies and triggered events.
//! History is stored in `~/.hassha/history.json`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Maximum number of history entries to keep
const MAX_HISTORY_ENTRIES: usize = 10;

/// A single history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Timestamp when the melody was played (ISO 8601 format)
    pub timestamp: String,

    /// The hook event that triggered playback
    pub event: String,

    /// The melody that was played (ID, URL, or path)
    pub melody: String,

    /// The project directory where the event occurred
    pub project_dir: String,

    /// Optional: tool name for tool-related events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    /// Optional: matcher that was used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
}

/// The history log structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

/// Get the path to the history file
pub fn history_file_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".hassha").join("history.json"))
}

/// Load the history from disk
pub fn load_history() -> Result<History> {
    let path = history_file_path()?;

    if !path.exists() {
        return Ok(History::default());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read history file: {}", path.display()))?;

    let history: History = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse history file: {}", path.display()))?;

    Ok(history)
}

/// Save the history to disk
pub fn save_history(history: &History) -> Result<()> {
    let path = history_file_path()?;

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(history)?;
    fs::write(&path, content)?;

    Ok(())
}

/// Add a new entry to the history
pub fn add_history_entry(
    event: &str,
    melody: &str,
    project_dir: &str,
    tool_name: Option<&str>,
    matcher: Option<&str>,
) -> Result<()> {
    let mut history = load_history().unwrap_or_default();

    // Get current timestamp in ISO 8601 format
    let timestamp = chrono_lite_timestamp();

    let entry = HistoryEntry {
        timestamp,
        event: event.to_string(),
        melody: melody.to_string(),
        project_dir: project_dir.to_string(),
        tool_name: tool_name.map(|s| s.to_string()),
        matcher: matcher.map(|s| s.to_string()),
    };

    // Add to the front
    history.entries.insert(0, entry);

    // Trim to max entries
    history.entries.truncate(MAX_HISTORY_ENTRIES);

    save_history(&history)?;

    Ok(())
}

/// Get a simple timestamp without external dependencies
fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = duration.as_secs();

    // Convert to rough datetime (not timezone aware, but good enough for logging)
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;

    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Approximate date calculation (not accounting for leap years perfectly)
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let (month, day) = days_to_month_day(remaining_days as u32, is_leap_year(year));

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_to_month_day(mut day_of_year: u32, leap: bool) -> (u32, u32) {
    let days_in_months: [u32; 12] = if leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    for (i, &days) in days_in_months.iter().enumerate() {
        if day_of_year < days {
            return (i as u32 + 1, day_of_year + 1);
        }
        day_of_year -= days;
    }

    (12, 31) // fallback
}

/// Format history for display
pub fn format_history(history: &History) -> String {
    if history.entries.is_empty() {
        return "No melody history yet.".to_string();
    }

    let mut output = String::new();
    output.push_str("Recent melody history (last 10):\n\n");

    for (i, entry) in history.entries.iter().enumerate() {
        output.push_str(&format!(
            "{}. [{}] {}\n",
            i + 1,
            entry.timestamp,
            entry.event
        ));
        output.push_str(&format!("   Melody: {}\n", entry.melody));
        output.push_str(&format!("   Project: {}\n", entry.project_dir));

        if let Some(tool) = &entry.tool_name {
            output.push_str(&format!("   Tool: {}\n", tool));
        }
        if let Some(matcher) = &entry.matcher {
            output.push_str(&format!("   Matcher: {}\n", matcher));
        }

        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry_serialization() {
        let entry = HistoryEntry {
            timestamp: "2024-01-15T10:30:00Z".to_string(),
            event: "Stop".to_string(),
            melody: "JY-Shibuya".to_string(),
            project_dir: "/home/user/project".to_string(),
            tool_name: None,
            matcher: None,
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("Stop"));
        assert!(json.contains("JY-Shibuya"));
    }

    #[test]
    fn test_history_truncation() {
        let mut history = History::default();

        // Add more than MAX entries
        for i in 0..15 {
            history.entries.insert(
                0,
                HistoryEntry {
                    timestamp: format!("2024-01-{:02}T10:00:00Z", i + 1),
                    event: "Stop".to_string(),
                    melody: format!("melody-{}", i),
                    project_dir: "/test".to_string(),
                    tool_name: None,
                    matcher: None,
                },
            );
        }

        history.entries.truncate(MAX_HISTORY_ENTRIES);

        assert_eq!(history.entries.len(), MAX_HISTORY_ENTRIES);
    }
}
