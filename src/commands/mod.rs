//! Subcommand dispatch table.

use anyhow::Result;
use clap::Subcommand;

use crate::client::McpClient;
use crate::output::{print_json, print_table};

mod bookings;
mod dashboard;
mod doctor;
mod invoices;
mod login;
mod members;
mod orgs;
mod plan;
mod signup;
mod team;
mod tool;
mod whatsapp;
mod whoami;

#[derive(Subcommand)]
pub enum Command {
    /// Save API key + URL to ~/.config/favcrm/config.toml.
    Login(login::Args),
    /// Register a new FavCRM workspace without an existing API key.
    #[command(subcommand)]
    Signup(signup::SignupCmd),
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
    /// Plan, quota, and billing operations.
    #[command(subcommand)]
    Plan(plan::PlanCmd),
    /// Team invites and agentic invite acceptance.
    #[command(subcommand)]
    Team(team::TeamCmd),
    /// WhatsApp Business connection setup.
    #[command(subcommand)]
    Whatsapp(whatsapp::WhatsappCmd),
    /// Diagnose MCP endpoint, auth, plan, and channel setup.
    Doctor,
    /// Universal verb for any registered MCP tool. Subcommands:
    /// `list` (catalog), `describe <name>` (input schema), `call <name> [json]`,
    /// or shortcut `<name> [json]`.
    Tool(tool::Args),
}

pub fn requires_auth(cmd: &Command) -> bool {
    match cmd {
        Command::Login(_) | Command::Signup(_) | Command::Doctor => false,
        Command::Team(c) => c.requires_auth(),
        _ => true,
    }
}

pub fn prefers_optional_auth(cmd: &Command) -> bool {
    matches!(cmd, Command::Doctor)
}

pub async fn dispatch(
    client: &McpClient,
    cmd: Command,
    json: bool,
    url: &str,
    authenticated: bool,
) -> Result<()> {
    match cmd {
        Command::Login(a) => login::run(a),
        Command::Signup(c) => signup::run(client, c, json, url).await,
        Command::Whoami => render(client, json, whoami::call(client).await?),
        Command::Orgs(c) => orgs::run(client, c, json).await,
        Command::Dashboard => render(client, json, dashboard::call(client).await?),
        Command::Members(c) => members::run(client, c, json).await,
        Command::Bookings(c) => bookings::run(client, c, json).await,
        Command::Invoices(c) => invoices::run(client, c, json).await,
        Command::Plan(c) => plan::run(client, c, json).await,
        Command::Team(c) => team::run(client, c, json, url).await,
        Command::Whatsapp(c) => whatsapp::run(client, c, json).await,
        Command::Doctor => doctor::run(client, json, url, authenticated).await,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_commands_do_not_require_existing_auth() {
        assert!(!requires_auth(&Command::Signup(
            signup::SignupCmd::Request {
                email: "owner@example.com".to_string(),
                organisation_name: "Example".to_string(),
                industry: None,
                country: None,
                timezone: None,
            }
        )));
        assert!(!requires_auth(&Command::Team(team::TeamCmd::Invite {
            cmd: team::InviteCmd::AcceptRequest {
                token: "invite".to_string(),
            },
        })));
        assert!(!requires_auth(&Command::Doctor));
    }

    #[test]
    fn normal_crm_and_invite_create_commands_require_auth() {
        assert!(requires_auth(&Command::Whoami));
        assert!(requires_auth(&Command::Team(team::TeamCmd::Invite {
            cmd: team::InviteCmd::Create {
                email: "member@example.com".to_string(),
                name: None,
                role: "staff".to_string(),
                expires_in_hours: None,
            },
        })));
    }
}
