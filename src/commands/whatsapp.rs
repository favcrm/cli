use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum WhatsappCmd {
    /// Show WhatsApp Business connection status.
    Status,
    /// Create a browser link for Meta Embedded Signup.
    Connect {
        /// Onboarding mode: cloud-api or coexistence.
        #[arg(long, default_value = "cloud-api", value_parser = ["cloud-api", "coexistence"])]
        mode: String,
        /// Connect link lifetime in minutes.
        #[arg(long)]
        ttl_minutes: Option<u32>,
    },
}

pub async fn run(client: &McpClient, cmd: WhatsappCmd, json: bool) -> Result<()> {
    let value = match cmd {
        WhatsappCmd::Status => {
            client
                .call_tool("get_whatsapp_connection_status", json!({}))
                .await?
        }
        WhatsappCmd::Connect { mode, ttl_minutes } => {
            let mut args = Map::new();
            args.insert(
                "onboardingMode".into(),
                Value::String(normalize_mode(&mode)),
            );
            if let Some(ttl) = ttl_minutes {
                args.insert("ttlMinutes".into(), Value::Number(ttl.into()));
            }
            client
                .call_tool("create_whatsapp_connect_link", Value::Object(args))
                .await?
        }
    };

    if json {
        print_json(&value)
    } else {
        print_table(&value)
    }
}

fn normalize_mode(mode: &str) -> String {
    match mode {
        "cloud-api" => "cloud_api".to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_mode;

    #[test]
    fn normalizes_cli_mode_to_tool_enum() {
        assert_eq!(normalize_mode("cloud-api"), "cloud_api");
        assert_eq!(normalize_mode("coexistence"), "coexistence");
    }
}
