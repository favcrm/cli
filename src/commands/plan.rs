use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum PlanCmd {
    /// Show current plan, payment state, modules, and quota usage.
    Status,
    /// Check whether a planned operation is allowed.
    Check {
        /// MCP tool name to preflight, e.g. create_account.
        #[arg(long)]
        tool: Option<String>,
        /// Quota code to check, e.g. contacts.
        #[arg(long)]
        quota: Option<String>,
        /// Module code to check, e.g. whatsapp.
        #[arg(long)]
        module: Option<String>,
        /// Planned usage increment.
        #[arg(long, default_value_t = 1)]
        delta: u32,
    },
    /// List active upgradeable plans.
    Options,
    /// Create a Stripe-hosted upgrade or billing portal link.
    Upgrade {
        /// Target plan code, e.g. favcrm-lite.
        #[arg(long)]
        plan_code: Option<String>,
        /// Target plan ID.
        #[arg(long)]
        plan_id: Option<String>,
        /// Billing cycle.
        #[arg(long, default_value = "monthly")]
        billing_cycle: String,
        /// Return URL for Stripe checkout or portal.
        #[arg(long)]
        return_url: Option<String>,
        /// Actually create the Stripe link.
        #[arg(long)]
        confirm: bool,
    },
    /// Create a billing portal link for the current plan.
    Portal {
        /// Return URL for Stripe portal.
        #[arg(long)]
        return_url: Option<String>,
        /// Actually create the Stripe link.
        #[arg(long)]
        confirm: bool,
    },
}

pub async fn run(client: &McpClient, cmd: PlanCmd, json_out: bool) -> Result<()> {
    let value = match cmd {
        PlanCmd::Status => client.call_tool("get_plan_status", json!({})).await?,
        PlanCmd::Check {
            tool,
            quota,
            module,
            delta,
        } => {
            client
                .call_tool(
                    "check_plan_operation",
                    Value::Object(check_args(tool, quota, module, delta)),
                )
                .await?
        }
        PlanCmd::Options => client.call_tool("list_plan_options", json!({})).await?,
        PlanCmd::Upgrade {
            plan_code,
            plan_id,
            billing_cycle,
            return_url,
            confirm,
        } => {
            client
                .call_tool(
                    "create_plan_upgrade_link",
                    Value::Object(upgrade_args(
                        plan_code,
                        plan_id,
                        billing_cycle,
                        return_url,
                        confirm,
                    )),
                )
                .await?
        }
        PlanCmd::Portal {
            return_url,
            confirm,
        } => {
            client
                .call_tool(
                    "create_plan_upgrade_link",
                    Value::Object(portal_args(return_url, confirm)),
                )
                .await?
        }
    };

    if json_out {
        print_json(&value)
    } else {
        print_table(&value)
    }
}

fn insert_optional(args: &mut Map<String, Value>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        args.insert(key.into(), Value::String(value));
    }
}

fn check_args(
    tool: Option<String>,
    quota: Option<String>,
    module: Option<String>,
    delta: u32,
) -> Map<String, Value> {
    let mut args = Map::new();
    insert_optional(&mut args, "toolName", tool);
    insert_optional(&mut args, "quotaCode", quota);
    insert_optional(&mut args, "moduleCode", module);
    args.insert("delta".into(), Value::Number(delta.into()));
    args
}

fn upgrade_args(
    plan_code: Option<String>,
    plan_id: Option<String>,
    billing_cycle: String,
    return_url: Option<String>,
    confirm: bool,
) -> Map<String, Value> {
    let mut args = Map::new();
    insert_optional(&mut args, "planCode", plan_code);
    insert_optional(&mut args, "planId", plan_id);
    insert_optional(&mut args, "returnUrl", return_url);
    args.insert("billingCycle".into(), Value::String(billing_cycle));
    args.insert("confirm".into(), Value::Bool(confirm));
    args
}

fn portal_args(return_url: Option<String>, confirm: bool) -> Map<String, Value> {
    let mut args = Map::new();
    args.insert("planCode".into(), Value::String("favcrm-lite".into()));
    insert_optional(&mut args, "returnUrl", return_url);
    args.insert("confirm".into(), Value::Bool(confirm));
    args
}
