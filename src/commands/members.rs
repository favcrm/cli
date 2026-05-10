use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum MembersCmd {
    /// Create a CRM account/customer, optionally enrolling it as a member.
    Create {
        /// Account/customer display name.
        name: String,
        /// Email address.
        #[arg(long)]
        email: Option<String>,
        /// Phone number.
        #[arg(long)]
        phone: Option<String>,
        /// First name for individual accounts.
        #[arg(long)]
        first_name: Option<String>,
        /// Last name for individual accounts.
        #[arg(long)]
        last_name: Option<String>,
        /// Enroll the account as a member immediately.
        #[arg(long)]
        enroll_membership: bool,
        /// Membership tier ID when enrolling.
        #[arg(long)]
        tier_id: Option<String>,
        /// Internal notes.
        #[arg(long)]
        notes: Option<String>,
    },
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
        MembersCmd::Create {
            name,
            email,
            phone,
            first_name,
            last_name,
            enroll_membership,
            tier_id,
            notes,
        } => {
            let mut args = Map::new();
            args.insert("name".into(), Value::String(name));
            if let Some(v) = email {
                args.insert("email".into(), Value::String(v));
            }
            if let Some(v) = phone {
                args.insert("phone".into(), Value::String(v));
            }
            if let Some(v) = first_name {
                args.insert("firstName".into(), Value::String(v));
            }
            if let Some(v) = last_name {
                args.insert("lastName".into(), Value::String(v));
            }
            if enroll_membership {
                args.insert("enrollMembership".into(), Value::Bool(true));
            }
            if let Some(v) = tier_id {
                args.insert("tierId".into(), Value::String(v));
            }
            if let Some(v) = notes {
                args.insert("notes".into(), Value::String(v));
            }
            client
                .call_tool("create_account", Value::Object(args))
                .await?
        }
        MembersCmd::Search {
            query,
            inactive_days,
            tier_id,
            limit,
        } => {
            let mut args = Map::new();
            if let Some(q) = query {
                args.insert("search".into(), Value::String(q));
            }
            if let Some(d) = inactive_days {
                args.insert("inactiveDays".into(), Value::Number(d.into()));
            }
            if let Some(t) = tier_id {
                args.insert("tierId".into(), Value::String(t));
            }
            if let Some(l) = limit {
                args.insert("limit".into(), Value::Number(l.into()));
            }
            client
                .call_tool("search_members", Value::Object(args))
                .await?
        }
        MembersCmd::Get { account_id } => {
            client
                .call_tool("get_member_profile", json!({ "accountId": account_id }))
                .await?
        }
    };
    if json {
        print_json(&value)
    } else {
        print_table(&value)
    }
}
