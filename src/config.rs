use std::{
    fs,
    path::{Path, PathBuf},
};






use serde::{Deserialize, Serialize};

/// Uses CARGO_MANIFEST_DIR environment variable to know where is project root at runtime
pub fn get_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
