use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::McpClient;
use crate::output::{print_json, print_table};

#[derive(Subcommand)]
pub enum BookingsCmd {
    /// List bookings.
    List {
        /// Filter by ISO date (start).
        #[arg(long)]
        from: Option<String>,
        /// Filter by ISO date (end).
        #[arg(long)]
        to: Option<String>,
        /// Status: pending|confirmed|completed|cancelled|no_show.
        #[arg(long)]
        status: Option<String>,
        /// Max rows.
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Fetch one booking by ID.
    Get { booking_id: String },
    /// Booking aggregate stats for the active company.
    Stats,
    /// Cancel a booking.
    Cancel { booking_id: String },
    /// Mark a booking complete.
    Complete { booking_id: String },
    /// Confirm a pending booking.
    Confirm { booking_id: String },
}

pub async fn run(client: &McpClient, cmd: BookingsCmd, json: bool) -> Result<()> {
    let value = match cmd {
        BookingsCmd::List {
            from,
            to,
            status,
            limit,
        } => {
            let mut args = Map::new();
            if let Some(v) = from {
                args.insert("from".into(), Value::String(v));
            }
            if let Some(v) = to {
                args.insert("to".into(), Value::String(v));
            }
            if let Some(v) = status {
                args.insert("status".into(), Value::String(v));
            }
            if let Some(v) = limit {
                args.insert("limit".into(), Value::Number(v.into()));
            }
            client
                .call_tool("list_bookings", Value::Object(args))
                .await?
        }
        BookingsCmd::Get { booking_id } => {
            client
                .call_tool("get_booking_detail", json!({ "bookingId": booking_id }))
                .await?
        }
        BookingsCmd::Stats => client.call_tool("get_booking_stats", json!({})).await?,
        BookingsCmd::Cancel { booking_id } => {
            client
                .call_tool("cancel_booking", json!({ "bookingId": booking_id }))
                .await?
        }
        BookingsCmd::Complete { booking_id } => {
            client
                .call_tool("complete_booking", json!({ "bookingId": booking_id }))
                .await?
        }
        BookingsCmd::Confirm { booking_id } => {
            client
                .call_tool("confirm_booking", json!({ "bookingId": booking_id }))
                .await?
        }
    };
    if json {
        print_json(&value)
    } else {
        print_table(&value)
    }
}
