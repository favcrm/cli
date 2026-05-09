//! MCP JSON-RPC client over Streamable HTTP.
//!
//! Speaks just enough JSON-RPC to call `tools/call`. Each call returns the
//! tool's `structuredContent.data` field (set by `registerMcpTools` in
//! `v2/api/src/tools/registry.ts`).

use anyhow::{anyhow, Context, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct McpClient {
    http: reqwest::Client,
    url: String,
    token: String,
    id: AtomicU64,
}

#[derive(Deserialize)]
struct RpcEnvelope {
    #[serde(default)]
    result: Option<Value>,
    #[serde(default)]
    error: Option<RpcError>,
}

#[derive(Deserialize, Debug)]
struct RpcError {
    code: i64,
    message: String,
}

impl McpClient {
    pub fn new(url: String, token: String) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(concat!("favcrm-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("reqwest client build");
        Self { http, url, token, id: AtomicU64::new(1) }
    }

    pub async fn call_tool(&self, name: &str, args: Value) -> Result<Value> {
        self.rpc("tools/call", json!({ "name": name, "arguments": args }))
            .await
            .and_then(extract_data)
    }

    /// Fetch the full MCP tool catalog. Returns the raw `tools` array — each
    /// entry has at least `{name, description, inputSchema}`.
    pub async fn list_tools(&self) -> Result<Value> {
        let result = self.rpc("tools/list", json!({})).await?;
        Ok(result.get("tools").cloned().unwrap_or(Value::Array(vec![])))
    }

    async fn rpc(&self, method: &str, params: Value) -> Result<Value> {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let body = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let res = self
            .http
            .post(&self.url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json, text/event-stream")
            .json(&body)
            .send()
            .await
            .with_context(|| format!("POST {}", self.url))?;

        let status = res.status();
        let text = res.text().await?;
        if !status.is_success() {
            return Err(anyhow!("HTTP {} from {}: {}", status, self.url, text));
        }

        let envelope: RpcEnvelope = parse_envelope(&text)
            .with_context(|| format!("parse MCP response: {}", truncate(&text, 200)))?;

        if let Some(err) = envelope.error {
            return Err(anyhow!("MCP error {}: {}", err.code, err.message));
        }
        envelope.result.ok_or_else(|| anyhow!("MCP response missing result"))
    }
}

/// Streamable HTTP transport may answer with raw JSON or a single-event SSE
/// frame (`data: {...}\n\n`). Handle both.
fn parse_envelope(body: &str) -> Result<RpcEnvelope> {
    if body.trim_start().starts_with('{') || body.trim_start().starts_with('[') {
        return Ok(serde_json::from_str(body)?);
    }
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("data:") {
            let payload = rest.trim();
            if !payload.is_empty() && payload != "[DONE]" {
                return Ok(serde_json::from_str(payload)?);
            }
        }
    }
    Err(anyhow!("no JSON or SSE payload in response"))
}

/// Pull the tool's primary payload out of the MCP `tools/call` result. We
/// prefer `structuredContent.data` (set by FavCRM's tool wrapper); fall back
/// to the raw `content[0].text` parsed as JSON, then to the whole result.
fn extract_data(result: Value) -> Result<Value> {
    if let Some(structured) = result.get("structuredContent") {
        if let Some(data) = structured.get("data") {
            return Ok(data.clone());
        }
        return Ok(structured.clone());
    }
    if let Some(content) = result.get("content").and_then(|c| c.as_array()) {
        if let Some(first) = content.first() {
            if let Some(text) = first.get("text").and_then(|t| t.as_str()) {
                if let Ok(parsed) = serde_json::from_str::<Value>(text) {
                    return Ok(parsed);
                }
                return Ok(Value::String(text.to_string()));
            }
        }
    }
    Ok(result)
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n { s.to_string() } else { format!("{}…", &s[..n]) }
}
