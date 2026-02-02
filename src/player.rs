//! Audio playback for hassha.
//!
//! Currently supports macOS using `afplay`.
//! Future: Add support for Linux (paplay/aplay) and Windows (PowerShell).

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Play an audio file.
///
/// On macOS, uses `afplay` command.
pub fn play_audio(path: &Path, volume: f32) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        play_audio_macos(path, volume)
    }

    #[cfg(target_os = "linux")]
    {
        play_audio_linux(path, volume)
    }

    #[cfg(target_os = "windows")]
    {
        play_audio_windows(path, volume)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        anyhow::bail!("Audio playback not supported on this platform")
    }
}

/// Play audio on macOS using afplay
#[cfg(target_os = "macos")]
fn play_audio_macos(path: &Path, volume: f32) -> Result<()> {
    // afplay volume is 0-255, where 1.0 = 100% = 255
    // But actually afplay -v takes a float where 1.0 is normal volume
    let volume_arg = volume.clamp(0.0, 1.0);

    let status = Command::new("afplay")
        .arg("-v")
        .arg(volume_arg.to_string())
        .arg(path)
        .status()
        .context("Failed to execute afplay")?;

    if !status.success() {
        anyhow::bail!("afplay exited with status: {}", status);
    }

    Ok(())
}

/// Play audio on Linux using paplay or aplay
#[cfg(target_os = "linux")]
fn play_audio_linux(path: &Path, volume: f32) -> Result<()> {
    // Try paplay first (PulseAudio)
    let result = Command::new("paplay")
        .arg("--volume")
        .arg(((volume * 65536.0) as u32).to_string())
        .arg(path)
        .status();

    if let Ok(status) = result {
        if status.success() {
            return Ok(());
        }
    }

    // Fall back to aplay (ALSA) - doesn't support volume directly
    let status = Command::new("aplay")
        .arg("-q")
        .arg(path)
        .status()
        .context("Failed to execute aplay (and paplay not available)")?;

    if !status.success() {
        anyhow::bail!("aplay exited with status: {}", status);
    }

    Ok(())
}

/// Play audio on Windows using PowerShell
#[cfg(target_os = "windows")]
fn play_audio_windows(path: &Path, _volume: f32) -> Result<()> {
    // Windows Media Player COM object via PowerShell
    let path_str = path.to_string_lossy();
    let script = format!(
        "(New-Object Media.SoundPlayer '{}').PlaySync()",
        path_str.replace("'", "''")
    );

    let status = Command::new("powershell")
        .arg("-c")
        .arg(&script)
        .status()
        .context("Failed to execute PowerShell audio playback")?;

    if !status.success() {
        anyhow::bail!("PowerShell exited with status: {}", status);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore] // Requires audio hardware
    fn test_play_audio() {
        // This test would need an actual audio file
    }
}
