//! Audio file caching for hassha.
//!
//! Downloads and caches audio files in `~/.hassha/audio/`.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Get the cache directory path
pub fn cache_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".hassha").join("audio"))
}

/// Ensure the cache directory exists
pub fn ensure_cache_dir() -> Result<PathBuf> {
    let dir = cache_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).context("Failed to create cache directory")?;
    }
    Ok(dir)
}

/// Get the cache path for a given URL
pub fn cache_path_for_url(url: &str) -> Result<PathBuf> {
    let cache = ensure_cache_dir()?;

    // Create a safe filename from the URL
    // Use the last path component or hash the URL
    let filename = url
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty() && s.contains('.'))
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // Hash the URL for a unique filename
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            url.hash(&mut hasher);
            format!("{:x}.mp3", hasher.finish())
        });

    Ok(cache.join(filename))
}

/// Check if a URL is cached and return the path if so
pub fn get_cached(url: &str) -> Result<Option<PathBuf>> {
    let path = cache_path_for_url(url)?;
    if path.exists() {
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

/// Download a file from URL and cache it
pub fn download_and_cache(url: &str) -> Result<PathBuf> {
    // Check if already cached
    if let Some(path) = get_cached(url)? {
        return Ok(path);
    }

    let cache_path = cache_path_for_url(url)?;

    // Download the file
    let response =
        reqwest::blocking::get(url).with_context(|| format!("Failed to download: {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download {}: HTTP {}", url, response.status());
    }

    let bytes = response
        .bytes()
        .with_context(|| format!("Failed to read response body from: {}", url))?;

    // Write to cache
    fs::write(&cache_path, &bytes)
        .with_context(|| format!("Failed to write cache file: {}", cache_path.display()))?;

    Ok(cache_path)
}

/// Resolve a melody source to a local file path.
///
/// The source can be:
/// - A predefined melody ID (e.g., "JY-Shibuya") -> downloads from yamanot.es
/// - A URL (e.g., "https://...") -> downloads and caches
/// - A local file path (e.g., "/path/to/file.mp3") -> returns as-is
pub fn resolve_melody_path(source: &str) -> Result<PathBuf> {
    use crate::melodies::MelodyRegistry;

    // Check if it's a predefined melody
    let registry = MelodyRegistry::new();
    if let Some(melody) = registry.get(source) {
        let url = melody.url();
        return download_and_cache(&url);
    }

    // Check if it's a URL
    if source.starts_with("http://") || source.starts_with("https://") {
        return download_and_cache(source);
    }

    // Treat as local file path
    let path = Path::new(source);
    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        anyhow::bail!("Audio file not found: {}", source)
    }
}

/// Get cache statistics
pub struct CacheStats {
    pub location: PathBuf,
    pub file_count: usize,
    pub total_size: u64,
}

/// Get information about the cache
pub fn cache_info() -> Result<CacheStats> {
    let dir = cache_dir()?;

    if !dir.exists() {
        return Ok(CacheStats {
            location: dir,
            file_count: 0,
            total_size: 0,
        });
    }

    let mut file_count = 0;
    let mut total_size = 0;

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            file_count += 1;
            total_size += entry.metadata()?.len();
        }
    }

    Ok(CacheStats {
        location: dir,
        file_count,
        total_size,
    })
}

/// Clear all cached audio files
pub fn cache_clear() -> Result<usize> {
    let dir = cache_dir()?;

    if !dir.exists() {
        return Ok(0);
    }

    let mut cleared = 0;
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::remove_file(entry.path())?;
            cleared += 1;
        }
    }

    Ok(cleared)
}

/// Download all predefined melodies
pub fn prefetch_all() -> Result<Vec<(String, Result<PathBuf>)>> {
    use crate::melodies::MELODIES;

    let results: Vec<_> = MELODIES
        .iter()
        .map(|melody| {
            let url = melody.url();
            let result = download_and_cache(&url);
            (melody.id.to_string(), result)
        })
        .collect();

    Ok(results)
}
