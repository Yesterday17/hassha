//! Installation logic for hassha plugin.
//!
//! Handles installing and uninstalling the hassha plugin for Claude Code and OpenCode.
//! All assets are bundled directly into the binary for self-contained distribution.

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

// Bundle all assets at compile time
const PLUGIN_JSON: &str = include_str!("../.claude-plugin/plugin.json");
const HOOKS_JSON: &str = include_str!("../hooks/hooks.json");
const MARKETPLACE_JSON: &str = include_str!("../.claude-plugin/marketplace.json");

/// Target application for installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallTarget {
    ClaudeCode,
    OpenCode,
}

impl InstallTarget {
    /// Get the plugin directory for this target
    pub fn plugin_dir(&self) -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;

        let dir = match self {
            InstallTarget::ClaudeCode => home.join(".claude").join("plugins").join("hassha"),
            InstallTarget::OpenCode => home.join(".opencode").join("plugins").join("hassha"),
        };

        Ok(dir)
    }

    /// Get the display name for this target
    pub fn name(&self) -> &'static str {
        match self {
            InstallTarget::ClaudeCode => "Claude Code",
            InstallTarget::OpenCode => "OpenCode",
        }
    }
}

/// Install the hassha plugin
///
/// This creates a self-contained plugin installation with all assets
/// extracted from the bundled binary.
pub fn install(target: InstallTarget) -> Result<()> {
    let plugin_dir = target.plugin_dir()?;

    println!("Installing hassha for {}...", target.name());
    println!("  Target: {}", plugin_dir.display());
    println!();

    // Create plugin directory structure
    fs::create_dir_all(&plugin_dir).with_context(|| {
        format!(
            "Failed to create plugin directory: {}",
            plugin_dir.display()
        )
    })?;

    let claude_plugin_dir = plugin_dir.join(".claude-plugin");
    fs::create_dir_all(&claude_plugin_dir)?;

    let hooks_dir = plugin_dir.join("hooks");
    fs::create_dir_all(&hooks_dir)?;

    let bin_dir = plugin_dir.join("target").join("release");
    fs::create_dir_all(&bin_dir)?;

    // Write bundled assets
    fs::write(claude_plugin_dir.join("plugin.json"), PLUGIN_JSON)?;
    println!("  ✓ Created .claude-plugin/plugin.json");

    fs::write(claude_plugin_dir.join("marketplace.json"), MARKETPLACE_JSON)?;
    println!("  ✓ Created .claude-plugin/marketplace.json");

    fs::write(hooks_dir.join("hooks.json"), HOOKS_JSON)?;
    println!("  ✓ Created hooks/hooks.json");

    // Copy the binary
    let exe_path = std::env::current_exe()?;
    let dst_bin = bin_dir.join("hassha");
    fs::copy(&exe_path, &dst_bin)
        .with_context(|| format!("Failed to copy binary to {}", dst_bin.display()))?;

    // Make binary executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dst_bin)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dst_bin, perms)?;
    }
    println!("  ✓ Installed hassha binary");

    println!();
    println!("Installation complete!");
    println!();
    println!("Next steps:");
    println!();

    match target {
        InstallTarget::ClaudeCode => {
            println!("  1. Add the marketplace and plugin to Claude Code:");
            println!("     /plugin marketplace add {}", plugin_dir.display());
            println!("     /plugin /plugin install hassha@hassha");
        }
        InstallTarget::OpenCode => {
            println!("  1. Add the plugin to OpenCode settings:");
            println!("     opencode --plugin-dir {}", plugin_dir.display());
        }
    }

    println!();
    println!("  2. Create a config file in your project:");
    println!("     mkdir -p .hassha");
    println!("     cat > .hassha/config.toml << 'EOF'");
    println!("     [hooks.Stop]");
    println!("     melody = \"JY-Shibuya\"");
    println!("     EOF");
    println!();
    println!("  3. Test it works:");
    println!("     {} play JY-Shibuya", dst_bin.display());

    Ok(())
}

/// Uninstall the hassha plugin
pub fn uninstall(target: InstallTarget) -> Result<()> {
    let plugin_dir = target.plugin_dir()?;

    println!("Uninstalling hassha from {}...", target.name());

    if !plugin_dir.exists() {
        println!("  Plugin not installed at {}", plugin_dir.display());
        return Ok(());
    }

    fs::remove_dir_all(&plugin_dir).with_context(|| {
        format!(
            "Failed to remove plugin directory: {}",
            plugin_dir.display()
        )
    })?;

    println!("  ✓ Removed {}", plugin_dir.display());
    println!();
    println!("Uninstallation complete!");
    println!();
    println!(
        "Note: Remember to remove the plugin from your {} settings.",
        target.name()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_dir() {
        let claude_dir = InstallTarget::ClaudeCode.plugin_dir().unwrap();
        assert!(claude_dir.ends_with("hassha"));
        assert!(claude_dir.to_string_lossy().contains(".claude"));

        let opencode_dir = InstallTarget::OpenCode.plugin_dir().unwrap();
        assert!(opencode_dir.ends_with("hassha"));
        assert!(opencode_dir.to_string_lossy().contains(".opencode"));
    }

    #[test]
    fn test_bundled_assets() {
        // Verify assets are properly bundled
        assert!(PLUGIN_JSON.contains("hassha"));
        assert!(HOOKS_JSON.contains("Stop"));
    }
}
