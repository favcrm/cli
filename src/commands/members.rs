use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum MembersCmd {
    /// Search and filter members.
    Search {
        /// Free-text query (name / email / phone).
        query: Option<String>,
        /// Members with no activity in the last N days.
        #[arg(long)]
        inactive_days: Option<u32>,
        /// Filter by membership tier ID.
        #[arg(long)]
        tier_id: Option<String>,
        /// Max rows (default 20).
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Fetch one member's full profile.
    Get {
        /// Account/member ID.
        account_id: String,
    },
}

pub async fn run(client: &McpClient, cmd: MembersCmd, json: bool) -> Result<()> {
    let value = match cmd {
        MembersCmd::Search { query, inactive_days, tier_id, limit } => {
            let mut args = Map::new();
            if let Some(q) = query { args.insert("search".into(), Value::String(q)); }
            if let Some(d) = inactive_days { args.insert("inactiveDays".into(), Value::Number(d.into())); }
            if let Some(t) = tier_id { args.insert("tierId".into(), Value::String(t)); }
            if let Some(l) = limit { args.insert("limit".into(), Value::Number(l.into())); }
            client.call_tool("search_members", Value::Object(args)).await?
        }
        MembersCmd::Get { account_id } => {
            client.call_tool("get_member_profile", json!({ "accountId": account_id })).await?
        }
    };
    if json { print_json(&value) } else { print_table(&value) }
}
