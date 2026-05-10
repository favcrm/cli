//! FavCRM CLI entry point.
//!
//! Architecture: thin client over the FavCRM MCP server (default
//! `https://api.favcrm.io/mcp`). Each subcommand maps to one MCP `tools/call`
//! invocation. Auth = `fav_mcp_*` API key from env or config file.

use anyhow::Result;
use clap::Parser;

mod client;
mod commands;
mod config;
mod metadata;
mod output;

#[derive(Parser)]
#[command(
    name = "favcrm",
    version,
    about = "FavCRM CLI — talk to your CRM from the terminal.",
    long_about = "Calls the FavCRM MCP server over HTTPS. Set FAVCRM_API_KEY env or run `favcrm login`."
)]
struct Cli {
    /// MCP endpoint base URL.
    #[arg(
        long,
        env = "FAVCRM_MCP_URL",
        default_value = "https://api.favcrm.io/mcp",
        global = true
    )]
    url: String,

    /// API key (fav_mcp_*). Falls back to config file or FAVCRM_API_KEY env.
    #[arg(long, env = "FAVCRM_API_KEY", global = true)]
    api_key: Option<String>,

    /// Print raw JSON instead of formatted tables.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    cmd: commands::Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Populate env from sidecar metadata when running inside a merchant
    // runtime; no-op (and fast-fail) on a laptop. Must run before clap parses
    // its `env = "FAVCRM_*"` inputs.
    metadata::populate_env().await;

    let cli = Cli::parse();
    let cfg = config::resolve(&cli.api_key, &cli.url)?;
    let client = client::McpClient::new(cfg.url.clone(), cfg.api_key.clone());
    commands::dispatch(&client, cli.cmd, cli.json).await
}
