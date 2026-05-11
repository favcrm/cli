use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::config;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum SignupCmd {
    /// Request a signup verification code for a new workspace.
    Request {
        /// Owner email address.
        #[arg(long)]
        email: String,
        /// Business or brand name.
        #[arg(long)]
        organisation_name: String,
        /// Industry vertical.
        #[arg(long)]
        industry: Option<String>,
        /// ISO 3166-1 alpha-2 country code.
        #[arg(long)]
        country: Option<String>,
        /// IANA timezone.
        #[arg(long)]
        timezone: Option<String>,
    },
    /// Verify a signup code and receive the first MCP API key.
    Verify {
        /// Request ID returned by signup request.
        #[arg(long)]
        request_id: String,
        /// 6-digit code from email.
        #[arg(long)]
        code: String,
        /// Do not save the returned API key to the CLI config.
        #[arg(long)]
        no_save: bool,
        /// Print the full API key in human output.
        #[arg(long)]
        show_key: bool,
    },
}

pub async fn run(client: &McpClient, cmd: SignupCmd, json_out: bool, url: &str) -> Result<()> {
    let mut value = match cmd {
        SignupCmd::Request {
            email,
            organisation_name,
            industry,
            country,
            timezone,
        } => {
            let mut args = Map::new();
            args.insert("email".into(), Value::String(email));
            args.insert("organisationName".into(), Value::String(organisation_name));
            insert_optional(&mut args, "industry", industry);
            insert_optional(&mut args, "country", country);
            insert_optional(&mut args, "timezone", timezone);
            client
                .call_tool("register_organisation_request", Value::Object(args))
                .await?
        }
        SignupCmd::Verify {
            request_id,
            code,
            no_save,
            show_key,
        } => {
            let mut result = client
                .call_tool(
                    "register_organisation_verify",
                    json!({ "requestId": request_id, "code": code }),
                )
                .await?;
            save_key_if_requested(&mut result, url, no_save)?;
            if !json_out && !show_key {
                mask_api_key(&mut result);
            }
            result
        }
    };

    if json_out {
        print_json(&value)
    } else {
        if let Some(obj) = value.as_object_mut() {
            if obj.get("apiKeySavedTo").is_some() && obj.get("apiKey").is_some() {
                obj.insert(
                    "nextCommand".into(),
                    Value::String("favcrm whoami".to_string()),
                );
            }
        }
        print_table(&value)
    }
}

fn insert_optional(args: &mut Map<String, Value>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        args.insert(key.into(), Value::String(value));
    }
}

pub fn save_key_if_requested(value: &mut Value, url: &str, no_save: bool) -> Result<()> {
    if no_save {
        return Ok(());
    }
    let Some(api_key) = value.get("apiKey").and_then(Value::as_str) else {
        return Ok(());
    };
    let path = config::save(api_key, url)?;
    if let Some(obj) = value.as_object_mut() {
        obj.insert(
            "apiKeySavedTo".into(),
            Value::String(path.display().to_string()),
        );
    }
    Ok(())
}

pub fn mask_api_key(value: &mut Value) {
    if let Some(obj) = value.as_object_mut() {
        if let Some(api_key) = obj.get("apiKey").and_then(Value::as_str) {
            obj.insert("apiKey".into(), Value::String(mask_secret(api_key)));
        }
    }
}

fn mask_secret(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= 12 {
        return "****".to_string();
    }
    let start: String = chars.iter().take(8).collect();
    let end: String = chars
        .iter()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("{start}...{end}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn masks_api_key_for_human_output() {
        let mut value = json!({ "apiKey": "test_key_1234567890abcdef" });
        mask_api_key(&mut value);
        assert_eq!(value["apiKey"], "test_key...cdef");
    }

    #[test]
    fn leaves_value_without_api_key_unchanged() {
        let mut value = json!({ "ok": true });
        mask_api_key(&mut value);
        assert_eq!(value, json!({ "ok": true }));
    }
}
