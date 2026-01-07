use crate::models::AppData;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn load_data(path: &str) -> Result<AppData> {
    if !Path::new(path).exists() {
        // Create default data file
        let default_data = AppData::default();
        save_data(path, &default_data)?;
        return Ok(default_data);
    }

    let contents =
        fs::read_to_string(path).context(format!("Failed to read data file: {}", path))?;

    let data: AppData = serde_json::from_str(&contents).context("Failed to parse JSON data")?;

    Ok(data)
}

pub fn save_data(path: &str, data: &AppData) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(data).context("Failed to serialize data to JSON")?;

    fs::write(path, json).context(format!("Failed to write data file: {}", path))?;

    Ok(())
}
