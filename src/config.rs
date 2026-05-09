//! Resolve API key + URL.
//!
//! Priority order:
//!   key: --api-key > FAVCRM_API_KEY > FAVCRM_MCP_TOKEN > config file
//!   url: --url    > FAVCRM_MCP_URL > FAVCRM_API_BASE+'/mcp' > config file > prod default
//!
//! The merchant runtime image (favcrm-openclaw-runtime) sets FAVCRM_MCP_TOKEN
//! and FAVCRM_API_BASE on the machine. Reading them directly here means the
//! bundled CLI works inside any FavCRM container with no extra wiring.

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

pub const DEFAULT_URL: &str = "https://api.favcrm.io/mcp";

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
        .or_else(|| env::var("FAVCRM_MCP_TOKEN").ok())
        .or(file.api_key)
        .ok_or_else(|| anyhow!("no API key. Set FAVCRM_API_KEY (or FAVCRM_MCP_TOKEN inside a runtime) or run `favcrm login <KEY>`"))?;

    // CLI flag wins outright. Otherwise fall back to merchant-runtime envs,
    // saved config, then the prod default.
    let url = if cli_url != DEFAULT_URL {
        cli_url.to_string()
    } else if let Ok(u) = env::var("FAVCRM_MCP_URL") {
        u
    } else if let Ok(base) = env::var("FAVCRM_API_BASE") {
        format!("{}/mcp", base.trim_end_matches('/'))
    } else {
        file.url.unwrap_or_else(|| DEFAULT_URL.to_string())
    };

    Ok(Resolved { api_key, url })
}
