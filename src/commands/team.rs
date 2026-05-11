use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use super::signup::{mask_api_key, save_key_if_requested};
use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum TeamCmd {
    /// Create an invite for a teammate.
    Invite {
        #[command(subcommand)]
        cmd: InviteCmd,
    },
}

#[derive(Subcommand)]
pub enum InviteCmd {
    /// Create a one-time team invite token.
    Create {
        /// Invitee email address.
        #[arg(long)]
        email: String,
        /// Optional display name.
        #[arg(long)]
        name: Option<String>,
        /// Role to grant on acceptance.
        #[arg(long, default_value = "staff", value_parser = ["staff", "manager"])]
        role: String,
        /// Invite lifetime in hours.
        #[arg(long)]
        expires_in_hours: Option<u32>,
    },
    /// Request a verification code for an invite token.
    AcceptRequest {
        /// Invite token from the invite link or create response.
        #[arg(long)]
        token: String,
    },
    /// Verify an invite code and receive an MCP API key.
    AcceptVerify {
        /// Invite token from the invite link or create response.
        #[arg(long)]
        token: String,
        /// 6-digit code from email.
        #[arg(long)]
        code: String,
        /// Optional display name when creating a new user.
        #[arg(long)]
        name: Option<String>,
        /// Do not save the returned API key to the CLI config.
        #[arg(long)]
        no_save: bool,
        /// Print the full API key in human output.
        #[arg(long)]
        show_key: bool,
    },
}

impl TeamCmd {
    pub fn requires_auth(&self) -> bool {
        match self {
            TeamCmd::Invite { cmd } => matches!(cmd, InviteCmd::Create { .. }),
        }
    }
}

pub async fn run(client: &McpClient, cmd: TeamCmd, json_out: bool, url: &str) -> Result<()> {
    let value = match cmd {
        TeamCmd::Invite { cmd } => run_invite(client, cmd, json_out, url).await?,
    };
    if json_out {
        print_json(&value)
    } else {
        print_table(&value)
    }
}

async fn run_invite(
    client: &McpClient,
    cmd: InviteCmd,
    json_out: bool,
    url: &str,
) -> Result<Value> {
    match cmd {
        InviteCmd::Create {
            email,
            name,
            role,
            expires_in_hours,
        } => {
            let mut args = Map::new();
            args.insert("email".into(), Value::String(email));
            args.insert("role".into(), Value::String(role));
            if let Some(name) = name {
                args.insert("name".into(), Value::String(name));
            }
            if let Some(hours) = expires_in_hours {
                args.insert("expiresInHours".into(), Value::Number(hours.into()));
            }
            client
                .call_tool("create_team_member_invite", Value::Object(args))
                .await
        }
        InviteCmd::AcceptRequest { token } => {
            client
                .call_tool("accept_team_invite_request", json!({ "token": token }))
                .await
        }
        InviteCmd::AcceptVerify {
            token,
            code,
            name,
            no_save,
            show_key,
        } => {
            let mut args = Map::new();
            args.insert("token".into(), Value::String(token));
            args.insert("code".into(), Value::String(code));
            if let Some(name) = name {
                args.insert("name".into(), Value::String(name));
            }
            let mut result = client
                .call_tool("accept_team_invite_verify", Value::Object(args))
                .await?;
            save_key_if_requested(&mut result, url, no_save)?;
            if !json_out && !show_key {
                mask_api_key(&mut result);
            }
            Ok(result)
        }
    }
}
