use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Value};

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum OrgsCmd {
    /// List companies the current user belongs to.
    List,
    /// Switch the active company.
    Switch {
        /// Target company ID.
        company_id: String,
    },
}

pub async fn run(client: &McpClient, cmd: OrgsCmd, json: bool) -> Result<()> {
    let value: Value = match cmd {
        OrgsCmd::List => client.call_tool("list_my_companies", json!({})).await?,
        OrgsCmd::Switch { company_id } => {
            client.call_tool("switch_company", json!({ "companyId": company_id })).await?
        }
    };
    if json { print_json(&value) } else { print_table(&value) }
}
