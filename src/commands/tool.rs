//! `favcrm tool` — universal verb for any registered MCP tool.
//!
//! Three modes:
//!   - `favcrm tool list` — discovery: print every available tool with a
//!     one-line description. Source of truth for what the agent can do.
//!   - `favcrm tool describe <name>` — print one tool's input schema for
//!     when the agent needs to know the exact argument shape.
//!   - `favcrm tool call <name> '{...}'` (or shortcut: `favcrm tool <name> '{...}'`)
//!     — invoke the tool. JSON args default to `{}`.

use anyhow::{anyhow, Context, Result};
use clap::Args as ClapArgs;
use serde_json::Value;

use crate::client::McpClient;
use crate::output::{print_json, print_table};

const META_VERBS: &[&str] = &["list", "describe", "call", "help"];

#[derive(ClapArgs)]
pub struct Args {
    /// One of: `list`, `describe <name>`, `call <name> [json]`, or directly
    /// the tool name (shortcut for `call <name>`).
    pub verb: String,
    /// Tool name when `verb` is `describe` or `call`. Ignored for `list`.
    pub name: Option<String>,
    /// JSON object of arguments for `call`. Defaults to `{}`.
    #[arg(default_value = "{}")]
    pub args: String,
}

pub async fn run(client: &McpClient, a: Args, json: bool) -> Result<()> {
    match a.verb.as_str() {
        "list" => list(client, json).await,
        "describe" => {
            let name = a.name.ok_or_else(|| anyhow!("`favcrm tool describe <name>` — name is required"))?;
            describe(client, &name, json).await
        }
        "call" => {
            let name = a.name.ok_or_else(|| anyhow!("`favcrm tool call <name> [json]` — name is required"))?;
            call(client, &name, &a.args, json).await
        }
        other => {
            // Shortcut: `favcrm tool <name> '{...}'` — `verb` is the tool name,
            // `name` (if Some) is actually the JSON args. Backward compatible
            // with v0.1.0–0.1.2 ergonomics.
            let payload = a.name.unwrap_or_else(|| a.args.clone());
            let payload = if payload.is_empty() { "{}".to_string() } else { payload };
            call(client, other, &payload, json).await
        }
    }
}

async fn list(client: &McpClient, json: bool) -> Result<()> {
    let tools = client.list_tools().await?;
    if json {
        return print_json(&tools);
    }
    let items = tools.as_array().ok_or_else(|| anyhow!("tools/list returned non-array"))?;
    println!("{} tool(s) available:\n", items.len());
    for t in items {
        let name = t.get("name").and_then(|v| v.as_str()).unwrap_or("?");
        let desc = t.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let summary = first_line(desc, 100);
        println!("  {:<32}  {}", name, summary);
    }
    println!("\nUse `favcrm tool describe <name>` for the input schema, then\n`favcrm tool call <name> '{{...}}'` (or `favcrm tool <name> '{{...}}'`) to invoke.");
    Ok(())
}

async fn describe(client: &McpClient, name: &str, json: bool) -> Result<()> {
    let tools = client.list_tools().await?;
    let items = tools.as_array().ok_or_else(|| anyhow!("tools/list returned non-array"))?;
    let found = items
        .iter()
        .find(|t| t.get("name").and_then(|v| v.as_str()) == Some(name))
        .ok_or_else(|| anyhow!("tool not found: {}", name))?;

    if json {
        return print_json(found);
    }
    let desc = found.get("description").and_then(|v| v.as_str()).unwrap_or("");
    println!("# {}\n\n{}\n", name, desc);
    if let Some(schema) = found.get("inputSchema") {
        println!("## Input schema\n");
        println!("{}", serde_json::to_string_pretty(schema)?);
    }
    Ok(())
}

async fn call(client: &McpClient, name: &str, raw_args: &str, json: bool) -> Result<()> {
    let parsed: Value = serde_json::from_str(raw_args)
        .with_context(|| format!("invalid JSON for tool args: {}", raw_args))?;
    let result = client.call_tool(name, parsed).await?;
    if json { print_json(&result) } else { print_table(&result) }
}

fn first_line(s: &str, max: usize) -> String {
    let line = s.lines().next().unwrap_or("");
    if line.chars().count() <= max {
        line.to_string()
    } else {
        let truncated: String = line.chars().take(max).collect();
        format!("{truncated}…")
    }
}

/// Exposed so the dispatcher in `commands::mod` can warn when someone passes
/// a meta verb as a tool name (e.g. typo `favcrm tool lis`).
#[allow(dead_code)]
pub fn is_meta_verb(s: &str) -> bool {
    META_VERBS.contains(&s)
}
