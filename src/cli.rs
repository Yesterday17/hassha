//! CLI argument parsing for hassha.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hassha")]
#[command(about = "Audio hook effects for Claude Code - Play Yamanote Line departure melodies")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Handle a hook event from Claude Code (reads stdin)
    Hook {
        /// The hook event name (e.g., Stop, Notification, SessionStart)
        event: String,
    },

    /// Play a melody directly
    Play {
        /// Melody ID (e.g., JY-Shibuya), URL, or file path
        melody: String,

        /// Volume level (0.0 - 1.0)
        #[arg(short, long, default_value = "1.0")]
        volume: f32,
    },

    /// List available predefined melodies
    List,

    /// Cache management
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },
}

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Show cache information
    Info,

    /// Clear all cached audio files
    Clear,

    /// Download all predefined melodies
    Prefetch,
}
