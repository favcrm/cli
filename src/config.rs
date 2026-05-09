//! Resolve API key + URL: CLI flag > env > `~/.config/favcrm/config.toml`.

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
struct FileCfg {
    api_key: Option<String>,
    url: Option<String>,
}

pub struct Resolved {
    pub api_key: String,
    pub url: String,
}

pub fn config_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("io", "FavCRM", "favcrm")
        .ok_or_else(|| anyhow!("could not resolve config directory"))?;
    Ok(dirs.config_dir().join("config.toml"))
}

fn load_file() -> Result<FileCfg> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(FileCfg::default());
    }
    let body = fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;
    toml::from_str(&body).with_context(|| format!("parse {}", path.display()))
}

pub fn save(api_key: &str, url: &str) -> Result<PathBuf> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let cfg = FileCfg { api_key: Some(api_key.to_string()), url: Some(url.to_string()) };
    fs::write(&path, toml::to_string_pretty(&cfg)?)?;
    Ok(path)
}

pub fn resolve(cli_key: &Option<String>, cli_url: &str) -> Result<Resolved> {
    let file = load_file().unwrap_or_default();
    let api_key = cli_key
        .clone()
        .or(file.api_key)
        .ok_or_else(|| anyhow!("no API key. Set FAVCRM_API_KEY env or run `favcrm login <KEY>`"))?;
    let url = if cli_url == "https://api.favcrm.io/mcp" {
        // Default not overridden on CLI — let file value win if present.
        file.url.unwrap_or_else(|| cli_url.to_string())
    } else {
        cli_url.to_string()
    };
    Ok(Resolved { api_key, url })
}
