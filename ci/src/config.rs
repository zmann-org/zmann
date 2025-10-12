use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

pub type BundlerConfig = HashMap<String, PackageConfig>;

#[derive(Clone, Debug, Deserialize)]
pub struct PackageConfig {
    pub private: Option<bool>,
}

pub fn load_bundler_config() -> Result<Option<BundlerConfig>> {
    let bundler_config_path = Path::new("bundler.toml");
    if !bundler_config_path.exists() {
        return Ok(None);
    }

    let config_str = fs::read_to_string(bundler_config_path)?;
    let result = toml::from_str(&config_str)?;
    Ok(Some(result))
}
