//! Subcommand dispatch table.

use anyhow::Result;
use clap::Subcommand;

use crate::client::McpClient;
use crate::output::{print_json, print_table};

mod bookings;
mod dashboard;
mod invoices;
mod login;
mod members;
mod orgs;
mod tool;
mod whoami;

#[derive(Subcommand)]
pub enum Command {
    /// Save API key + URL to ~/.config/favcrm/config.toml.
    Login(login::Args),
    /// Show the current user/company.
    Whoami,
    /// List or switch organisations.
    #[command(subcommand)]
    Orgs(orgs::OrgsCmd),
    /// Today's headline stats for the active company.
    Dashboard,
    /// Members / customers.
    #[command(subcommand)]
    Members(members::MembersCmd),
    /// Bookings.
    #[command(subcommand)]
    Bookings(bookings::BookingsCmd),
    /// Invoices.
    #[command(subcommand)]
    Invoices(invoices::InvoicesCmd),
    /// Universal verb for any registered MCP tool. Subcommands:
    /// `list` (catalog), `describe <name>` (input schema), `call <name> [json]`,
    /// or shortcut `<name> [json]`.
    Tool(tool::Args),
}

pub async fn dispatch(client: &McpClient, cmd: Command, json: bool) -> Result<()> {
    match cmd {
        Command::Login(a) => login::run(a),
        Command::Whoami => render(client, json, whoami::call(client).await?),
        Command::Orgs(c) => orgs::run(client, c, json).await,
        Command::Dashboard => render(client, json, dashboard::call(client).await?),
        Command::Members(c) => members::run(client, c, json).await,
        Command::Bookings(c) => bookings::run(client, c, json).await,
        Command::Invoices(c) => invoices::run(client, c, json).await,
        Command::Tool(a) => tool::run(client, a, json).await,
    }
}

fn render(_client: &McpClient, json: bool, value: serde_json::Value) -> Result<()> {
    if json {
        print_json(&value)
    } else {
        print_table(&value)
    }
}
