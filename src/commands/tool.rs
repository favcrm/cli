//! Generic escape hatch — invoke any registered MCP tool by name.
//!
//! Useful for tools we haven't wrapped as a first-class subcommand yet.
//! Args are passed as a single JSON object string.

use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use serde_json::Value;

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(ClapArgs)]
pub struct Args {
    /// Tool name (see https://api.favcrm.io/mcp tools/list).
    pub name: String,
    /// JSON object of arguments. Defaults to `{}`.
    #[arg(default_value = "{}")]
    pub args: String,
}

pub async fn run(client: &McpClient, a: Args, json: bool) -> Result<()> {
    let parsed: Value = serde_json::from_str(&a.args)
        .with_context(|| format!("invalid JSON for tool args: {}", a.args))?;
    let result = client.call_tool(&a.name, parsed).await?;
    if json { print_json(&result) } else { print_table(&result) }
}
