use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocketBaseConfig {
    pub server_url: String,
    pub enabled: bool,
}

impl Default for PocketBaseConfig {
    fn default() -> Self {
        Self {
            server_url: "http://127.0.0.1:8090".to_string(),
            enabled: false,
        }
    }
}

pub fn get_config_file_path() -> anyhow::Result<String> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    let config_path = home.join(".latios").join("config.json");
    Ok(config_path.to_string_lossy().to_string())
}

pub fn load_config() -> anyhow::Result<PocketBaseConfig> {
    let path = get_config_file_path()?;

    if !Path::new(&path).exists() {
        let default_config = PocketBaseConfig::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }

    let contents = fs::read_to_string(&path)?;
    let config: PocketBaseConfig = serde_json::from_str(&contents)?;
    Ok(config)
}

pub fn save_config(config: &PocketBaseConfig) -> anyhow::Result<()> {
    let path = get_config_file_path()?;

    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}
