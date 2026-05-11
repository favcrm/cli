use anyhow::Result;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::config;
use crate::output::{print_json, print_table};

pub async fn run(client: &McpClient, json_out: bool, url: &str, authenticated: bool) -> Result<()> {
    let mut report = Map::new();
    report.insert("url".into(), Value::String(url.to_string()));
    report.insert("authenticated".into(), Value::Bool(authenticated));
    if authenticated {
        report.insert("authSource".into(), Value::String(auth_source()));
    }

    match client.list_tools().await {
        Ok(tools) => {
            report.insert("reachable".into(), Value::Bool(true));
            report.insert(
                "toolCount".into(),
                Value::Number(tools.as_array().map(|v| v.len()).unwrap_or(0).into()),
            );
        }
        Err(err) => {
            report.insert("reachable".into(), Value::Bool(false));
            report.insert("error".into(), Value::String(err.to_string()));
            return render(Value::Object(report), json_out);
        }
    }

    if authenticated {
        insert_tool_result(
            client,
            &mut report,
            "currentUser",
            "list_my_companies",
            json!({}),
        )
        .await;
        insert_tool_result(client, &mut report, "plan", "get_plan_status", json!({})).await;
        insert_tool_result(
            client,
            &mut report,
            "whatsapp",
            "get_whatsapp_connection_status",
            json!({}),
        )
        .await;
    }

    render(Value::Object(report), json_out)
}

async fn insert_tool_result(
    client: &McpClient,
    report: &mut Map<String, Value>,
    key: &str,
    tool: &str,
    args: Value,
) {
    match client.call_tool(tool, args).await {
        Ok(value) => {
            report.insert(key.into(), value);
        }
        Err(err) => {
            report.insert(format!("{key}Error"), Value::String(err.to_string()));
        }
    }
}

fn render(value: Value, json_out: bool) -> Result<()> {
    if json_out {
        print_json(&value)
    } else {
        print_table(&value)
    }
}

fn auth_source() -> String {
    if std::env::var("FAVCRM_API_KEY").is_ok() {
        "FAVCRM_API_KEY".to_string()
    } else if std::env::var("FAVCRM_MCP_TOKEN").is_ok() {
        "FAVCRM_MCP_TOKEN".to_string()
    } else if config::config_path().map(|p| p.exists()).unwrap_or(false) {
        "config file".to_string()
    } else {
        "cli flag".to_string()
    }
}
