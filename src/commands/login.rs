use anyhow::Result;
use clap::Args as ClapArgs;

use crate::config;

#[derive(ClapArgs)]
pub struct Args {
    /// Your fav_mcp_* API key.
    pub api_key: String,
    /// Override MCP endpoint base URL.
    #[arg(long, default_value = "https://api.favcrm.io/mcp")]
    pub url: String,
}

pub fn run(args: Args) -> Result<()> {
    let path = config::save(&args.api_key, &args.url)?;
    println!("Saved {} → {}", args.url, path.display());
    Ok(())
}
