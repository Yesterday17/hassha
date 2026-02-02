//! Configuration parsing and hierarchical lookup for hassha.
//!
//! Configuration is stored in `.hassha/config.toml` files.
//! The config resolver walks up the directory tree to find the nearest config file.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main configuration structure for hassha
#[derive(Debug, Clone, Deserialize, Default)]
pub struct HasshaConfig {
    /// Hook-specific configurations
    #[serde(default)]
    pub hooks: HashMap<String, HookConfig>,
}

/// Configuration for a specific hook event
#[derive(Debug, Clone, Deserialize)]
pub struct HookConfig {
    /// The melody to play. Can be:
    /// - A predefined melody ID (e.g., "JY-Shibuya")
    /// - A URL (e.g., "https://example.com/sound.mp3")
    /// - A local file path (e.g., "/path/to/sound.mp3")
    pub melody: String,

    /// Optional volume level (0.0 - 1.0). Default is 1.0.
    #[serde(default = "default_volume")]
    pub volume: f32,

    /// Optional matcher pattern for filtering (e.g., for PostToolUse)
    #[serde(default)]
    pub matcher: Option<String>,
}

fn default_volume() -> f32 {
    1.0
}

/// The config file name
pub const CONFIG_FILE: &str = ".hassha/config.toml";

/// Find the config file by walking up the directory tree
pub fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();

    loop {
        let config_path = current.join(CONFIG_FILE);
        if config_path.exists() {
            return Some(config_path);
        }

        // Move to parent directory
        if !current.pop() {
            break;
        }
    }

    None
}

/// Load configuration from a directory (walks up tree to find config)
pub fn load_config(start_dir: &Path) -> Result<Option<HasshaConfig>> {
    let config_path = match find_config_file(start_dir) {
        Some(path) => path,
        None => return Ok(None),
    };

    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    let config: HasshaConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

    Ok(Some(config))
}

/// Get the hook configuration for a specific event
pub fn get_hook_config<'a>(config: &'a HasshaConfig, event: &str) -> Option<&'a HookConfig> {
    config.hooks.get(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml_str = r#"
[hooks.Stop]
melody = "JY-Shibuya"
volume = 0.8

[hooks.Notification]
melody = "https://example.com/custom.mp3"

[hooks.PostToolUse]
melody = "JY-Tokyo"
matcher = "Bash"
"#;

        let config: HasshaConfig = toml::from_str(toml_str).unwrap();

        assert!(config.hooks.contains_key("Stop"));
        assert_eq!(config.hooks["Stop"].melody, "JY-Shibuya");
        assert_eq!(config.hooks["Stop"].volume, 0.8);

        assert!(config.hooks.contains_key("Notification"));
        assert_eq!(
            config.hooks["Notification"].melody,
            "https://example.com/custom.mp3"
        );
        assert_eq!(config.hooks["Notification"].volume, 1.0); // default

        assert!(config.hooks.contains_key("PostToolUse"));
        assert_eq!(
            config.hooks["PostToolUse"].matcher,
            Some("Bash".to_string())
        );
    }
}
