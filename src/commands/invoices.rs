use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum InvoicesCmd {
    /// List invoices.
    List {
        /// Filter by status: draft|sent|paid|void|overdue.
        #[arg(long)]
        status: Option<String>,
        /// Max rows.
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Fetch one invoice by ID.
    Get { invoice_id: String },
    /// Mark an invoice as sent (publishes + emails).
    Send { invoice_id: String },
    /// Void an invoice (reverses without deletion).
    Void { invoice_id: String },
    /// Aggregate invoice stats.
    Stats,
}

pub async fn run(client: &McpClient, cmd: InvoicesCmd, json: bool) -> Result<()> {
    let value = match cmd {
        InvoicesCmd::List { status, limit } => {
            let mut args = Map::new();
            if let Some(v) = status { args.insert("status".into(), Value::String(v)); }
            if let Some(v) = limit { args.insert("limit".into(), Value::Number(v.into())); }
            client.call_tool("list_invoices", Value::Object(args)).await?
        }
        InvoicesCmd::Get { invoice_id } => {
            client.call_tool("get_invoice", json!({ "invoiceId": invoice_id })).await?
        }
        InvoicesCmd::Send { invoice_id } => {
            client.call_tool("mark_invoice_sent", json!({ "invoiceId": invoice_id })).await?
        }
        InvoicesCmd::Void { invoice_id } => {
            client.call_tool("void_invoice", json!({ "invoiceId": invoice_id })).await?
        }
        InvoicesCmd::Stats => client.call_tool("get_invoice_stats", json!({})).await?,
    };
    if json { print_json(&value) } else { print_table(&value) }
}
