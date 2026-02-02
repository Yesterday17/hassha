//! CLI argument parsing for hassha.

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hassha")]
#[command(
    about = "Audio hook effects for Claude Code / OpenCode - Play JR East departure melodies"
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Handle a hook event (reads stdin)
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

    /// Show recent melody history
    History {
        #[command(subcommand)]
        command: Option<HistoryCommands>,
    },

    /// Cache management
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },

    /// Install hassha plugin for Claude Code or OpenCode
    Install {
        #[command(flatten)]
        target: InstallTarget,
    },

    /// Uninstall hassha plugin
    Uninstall {
        #[command(flatten)]
        target: InstallTarget,
    },
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct InstallTarget {
    /// Install for Claude Code
    #[arg(long, group = "target")]
    pub claude_code: bool,

    /// Install for OpenCode
    #[arg(long, group = "target")]
    pub open_code: bool,
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

#[derive(Subcommand)]
pub enum HistoryCommands {
    /// Clear melody history
    Clear,
}
