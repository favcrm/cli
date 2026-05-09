use anyhow::Result;
use serde_json::{json, Value};

use crate::client::McpClient;

pub async fn call(client: &McpClient) -> Result<Value> {
    client.call_tool("list_my_companies", json!({})).await
}
