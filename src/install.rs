//! Installation logic for hassha plugin.
//!
//! Handles installing and uninstalling the hassha plugin for Claude Code and OpenCode.
//! - Claude Code: Uses binary-based hooks with bundled assets
//! - OpenCode: Uses TypeScript plugin system

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

// Bundle all assets at compile time for Claude Code
const PLUGIN_JSON: &str = include_str!("../.claude-plugin/plugin.json");
const MARKETPLACE_JSON: &str = include_str!("../.claude-plugin/marketplace.json");
const HOOKS_JSON: &str = include_str!("../hooks/hooks.json");

// Bundle OpenCode plugin
const OPENCODE_PLUGIN_TS: &str = include_str!("../.opencode/plugins/hassha.ts");
const OPENCODE_PACKAGE_JSON: &str = include_str!("../.opencode/package.json");

/// Target application for installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallTarget {
    ClaudeCode,
    OpenCode,
}

impl InstallTarget {
    /// Get the plugin directory for this target (used for Claude Code)
    pub fn plugin_dir(&self) -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;

        let dir = match self {
            InstallTarget::ClaudeCode => home.join(".claude").join("plugins").join("hassha"),
            InstallTarget::OpenCode => home.join(".config").join("opencode"),
        };

        Ok(dir)
    }
}

/// Install the hassha plugin
///
/// For Claude Code: Creates a self-contained plugin installation with all assets
/// extracted from the bundled binary.
///
/// For OpenCode: Installs the TypeScript plugin and binary.
pub fn install(target: InstallTarget) -> Result<()> {
    match target {
        InstallTarget::ClaudeCode => install_claude_code(),
        InstallTarget::OpenCode => install_opencode(),
    }
}

/// Install for Claude Code
fn install_claude_code() -> Result<()> {
    let plugin_dir = InstallTarget::ClaudeCode.plugin_dir()?;

    println!("Installing hassha for Claude Code...");
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
    println!("  1. Add the marketplace and plugin to Claude Code:");
    println!("     /plugin marketplace add {}", plugin_dir.display());
    println!("     /plugin install hassha@hassha");
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

/// Install for OpenCode
fn install_opencode() -> Result<()> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let config_dir = home.join(".config").join("opencode");
    let plugins_dir = config_dir.join("plugins");

    println!("Installing hassha for OpenCode...");
    println!("  Target: {}", plugins_dir.display());
    println!();

    // Create plugin directory
    fs::create_dir_all(&plugins_dir).with_context(|| {
        format!(
            "Failed to create plugins directory: {}",
            plugins_dir.display()
        )
    })?;

    // Write the TypeScript plugin
    let plugin_path = plugins_dir.join("hassha.ts");
    fs::write(&plugin_path, OPENCODE_PLUGIN_TS)?;
    println!("  ✓ Created plugins/hassha.ts");

    // Write package.json if it doesn't exist or merge dependencies
    let package_json_path = config_dir.join("package.json");
    if !package_json_path.exists() {
        fs::write(&package_json_path, OPENCODE_PACKAGE_JSON)?;
        println!("  ✓ Created package.json");
    } else {
        println!("  ℹ package.json already exists, skipping");
    }

    // Copy the binary to a known location
    let bin_dir = config_dir.join("bin");
    fs::create_dir_all(&bin_dir)?;

    let exe_path = std::env::current_exe()?;
    let dst_bin = bin_dir.join("hassha");
    fs::copy(&exe_path, &dst_bin)
        .with_context(|| format!("Failed to copy binary to {}", dst_bin.display()))?;

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
    println!("Note: The plugin expects the hassha binary at:");
    println!("  {}", dst_bin.display());
    println!();
    println!("You may need to update the HASSHA_BINARY path in the plugin if");
    println!("you move the binary to a different location.");
    println!();
    println!("Next steps:");
    println!();
    println!("  1. Create a config file in your project:");
    println!("     mkdir -p .hassha");
    println!("     cat > .hassha/config.toml << 'EOF'");
    println!("     [hooks.Stop]");
    println!("     melody = \"JY-Shibuya\"");
    println!("     EOF");
    println!();
    println!("  2. Test it works:");
    println!("     {} play JY-Shibuya", dst_bin.display());

    Ok(())
}

/// Uninstall the hassha plugin
pub fn uninstall(target: InstallTarget) -> Result<()> {
    match target {
        InstallTarget::ClaudeCode => uninstall_claude_code(),
        InstallTarget::OpenCode => uninstall_opencode(),
    }
}

/// Uninstall from Claude Code
fn uninstall_claude_code() -> Result<()> {
    let plugin_dir = InstallTarget::ClaudeCode.plugin_dir()?;

    println!("Uninstalling hassha from Claude Code...");

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
    println!("Note: Remember to remove the plugin from your Claude Code settings.");

    Ok(())
}

/// Uninstall from OpenCode
fn uninstall_opencode() -> Result<()> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let config_dir = home.join(".config").join("opencode");
    let plugin_path = config_dir.join("plugins").join("hassha.ts");
    let bin_path = config_dir.join("bin").join("hassha");

    println!("Uninstalling hassha from OpenCode...");

    let mut removed = false;

    if plugin_path.exists() {
        fs::remove_file(&plugin_path)?;
        println!("  ✓ Removed {}", plugin_path.display());
        removed = true;
    }

    if bin_path.exists() {
        fs::remove_file(&bin_path)?;
        println!("  ✓ Removed {}", bin_path.display());
        removed = true;
    }

    if !removed {
        println!("  Plugin not installed");
        return Ok(());
    }

    println!();
    println!("Uninstallation complete!");
    println!();
    println!("Note: package.json was not modified. Remove @opencode-ai/plugin");
    println!("dependency manually if no longer needed.");

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
