//! Theme directory management and custom theme discovery

use std::fs;
use std::path::PathBuf;

use crate::error::{Error, Result};

/// Get the user's custom themes directory
pub fn get_themes_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| Error::Config("Could not determine config directory".to_string()))?;

    let themes_dir = config_dir.join("pentest-connector").join("themes");

    // Create directory if it doesn't exist
    if !themes_dir.exists() {
        fs::create_dir_all(&themes_dir).map_err(|e| {
            Error::Config(format!("Failed to create themes directory: {}", e))
        })?;
    }

    Ok(themes_dir)
}

/// Discovered custom theme file
#[derive(Debug, Clone)]
pub struct DiscoveredTheme {
    pub file_name: String,
    pub file_path: PathBuf,
}

/// Scan the themes directory for .css files
pub fn discover_custom_themes() -> Result<Vec<DiscoveredTheme>> {
    let themes_dir = get_themes_dir()?;

    let mut themes = Vec::new();

    if !themes_dir.exists() {
        return Ok(themes);
    }

    let entries = fs::read_dir(&themes_dir).map_err(|e| {
        Error::Config(format!("Failed to read themes directory: {}", e))
    })?;

    for entry in entries.flatten() {
        let path = entry.path();

        // Only include .css files
        if path.extension().and_then(|s| s.to_str()) == Some("css") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                themes.push(DiscoveredTheme {
                    file_name: file_name.to_string(),
                    file_path: path,
                });
            }
        }
    }

    // Sort by file name for consistent ordering
    themes.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    Ok(themes)
}

/// Load a custom theme file's content
pub fn load_theme_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).map_err(|e| {
        Error::Config(format!("Failed to read theme file: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_themes_dir() {
        let dir = get_themes_dir().unwrap();
        assert!(dir.ends_with("pentest-connector/themes"));
    }

    #[test]
    fn test_discover_custom_themes() {
        // Should not error even if directory is empty
        let result = discover_custom_themes();
        assert!(result.is_ok());
    }
}
