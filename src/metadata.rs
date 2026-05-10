//! Optional sidecar metadata probe.
//!
//! Inside a FavCRM merchant runtime (favcrm-openclaw-runtime), the sidecar
//! exposes a loopback-only metadata endpoint that returns a non-stale MCP
//! token plus the canonical MCP URL. We probe that endpoint at CLI startup;
//! when it answers, we set FAVCRM_API_KEY and FAVCRM_MCP_URL so the rest of
//! the resolve chain (env > config file > prod default) just works.
//!
//! Outside a runtime container, the connect attempt fails fast (connection
//! refused on 127.0.0.1:3002) and we fall through silently — laptop usage
//! continues to rely on `favcrm login` / FAVCRM_API_KEY env.
//!
//! Mirrors AWS IMDSv2 / Vault Agent / kubelet projected-token patterns:
//! consumer reads a fresh credential on demand from a local trusted source
//! rather than caching long-lived secrets to disk.

use serde::Deserialize;
use std::{env, time::Duration};

const DEFAULT_PORT: &str = "3002";
const PROBE_TIMEOUT: Duration = Duration::from_millis(800);

#[derive(Deserialize)]
struct CliToken {
    token: String,
    url: Option<String>,
}

pub async fn populate_env() {
    if env::var("FAVCRM_API_KEY").is_ok() {
        return;
    }
    let port =
        env::var("FAVCRM_SIDECAR_INTERNAL_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
    let url = format!("http://127.0.0.1:{}/v1/cli-token", port);

    let client = match reqwest::Client::builder().timeout(PROBE_TIMEOUT).build() {
        Ok(c) => c,
        Err(_) => return,
    };
    let resp = match client.get(&url).send().await {
        Ok(r) if r.status().is_success() => r,
        _ => return,
    };
    let body = match resp.json::<CliToken>().await {
        Ok(b) => b,
        Err(_) => return,
    };

    env::set_var("FAVCRM_API_KEY", &body.token);
    if env::var("FAVCRM_MCP_URL").is_err() {
        if let Some(u) = body.url {
            env::set_var("FAVCRM_MCP_URL", u);
        }
    }
}
