//! hassha - Audio hook effects for Claude Code / OpenCode
//!
//! Play JR East departure melodies or custom sounds on various events.
//! Configure different melodies per project using `.hassha/config.toml`.

mod cache;
mod cli;
mod config;
mod hook;
mod install;
mod melodies;
mod player;

use anyhow::Result;
use clap::Parser;

use cli::{CacheCommands, Cli, Commands};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hook { event } => {
            hook::handle_hook(&event)?;
        }

        Commands::Play { melody, volume } => {
            let path = cache::resolve_melody_path(&melody)?;
            player::play_audio(&path, volume)?;
            println!("Played: {}", melody);
        }

        Commands::List => {
            println!("Available melodies (JR East Lines):\n");
            println!(
                "Lines: JY=Yamanote, JK=Keihin-Tohoku, JB=Sobu, JA=Saikyo, JU=Ueno-Tokyo, NEX=Narita Express\n"
            );
            println!(
                "{:<22} {:<15} {:<18} {:<10} {}",
                "ID", "Line", "Station", "Japanese", "Melody"
            );
            println!("{}", "-".repeat(85));

            let mut current_line = "";
            for melody in melodies::MELODIES {
                // Add separator between lines
                if melody.line != current_line {
                    if !current_line.is_empty() {
                        println!();
                    }
                    current_line = melody.line;
                }
                println!(
                    "{:<22} {:<15} {:<18} {:<10} {}",
                    melody.id,
                    melody.line_name,
                    melody.station,
                    melody.station_jp,
                    melody.melody_name
                );
            }

            println!("\nUsage in .hassha/config.toml:");
            println!("  [hooks.Stop]");
            println!("  melody = \"JY-Shibuya\"");
            println!("  # or: melody = \"JK-Akihabara\"");
            println!("  # or: melody = \"NEX-Shinjuku\"");
        }

        Commands::Cache { command } => match command {
            CacheCommands::Info => {
                let stats = cache::cache_info()?;
                println!("Cache location: {}", stats.location.display());
                println!("Files cached:   {}", stats.file_count);
                println!(
                    "Total size:     {:.2} MB",
                    stats.total_size as f64 / 1_000_000.0
                );
            }

            CacheCommands::Clear => {
                let cleared = cache::cache_clear()?;
                println!("Cleared {} cached files", cleared);
            }

            CacheCommands::Prefetch => {
                println!("Downloading all predefined melodies...\n");
                let results = cache::prefetch_all()?;

                let mut success = 0;
                let mut failed = 0;

                for (id, result) in results {
                    match result {
                        Ok(path) => {
                            println!("  ✓ {} -> {}", id, path.display());
                            success += 1;
                        }
                        Err(e) => {
                            println!("  ✗ {} - {}", id, e);
                            failed += 1;
                        }
                    }
                }

                println!("\nDownloaded: {}, Failed: {}", success, failed);
            }
        },

        Commands::Install { target } => {
            let install_target = if target.claude_code {
                install::InstallTarget::ClaudeCode
            } else {
                install::InstallTarget::OpenCode
            };
            install::install(install_target)?;
        }

        Commands::Uninstall { target } => {
            let install_target = if target.claude_code {
                install::InstallTarget::ClaudeCode
            } else {
                install::InstallTarget::OpenCode
            };
            install::uninstall(install_target)?;
        }
    }

    Ok(())
}
