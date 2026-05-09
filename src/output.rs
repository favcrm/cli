//! JSON / table output helpers.

use anyhow::Result;
use serde_json::Value;
use tabled::{builder::Builder, settings::Style};

pub fn print_json(value: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

/// Render an array of objects as a table. Picks columns from the first row.
/// If the value is not a homogeneous array of objects, falls back to JSON.
pub fn print_table(value: &Value) -> Result<()> {
    let rows_value = pick_array(value);
    let Some(rows) = rows_value.as_array() else {
        return print_json(value);
    };
    if rows.is_empty() {
        println!("(no rows)");
        return Ok(());
    }

    let Some(first_obj) = rows[0].as_object() else {
        return print_json(value);
    };
    let columns: Vec<String> = first_obj.keys().cloned().collect();

    let mut builder = Builder::default();
    builder.push_record(columns.clone());
    for row in rows {
        let obj = match row.as_object() {
            Some(o) => o,
            None => continue,
        };
        let cells: Vec<String> = columns
            .iter()
            .map(|c| obj.get(c).map(stringify_cell).unwrap_or_default())
            .collect();
        builder.push_record(cells);
    }
    let mut table = builder.build();
    table.with(Style::rounded());
    println!("{}", table);
    Ok(())
}

/// Tools commonly return `{ items: [...], total: N }`. Unwrap the first array
/// we find at the top level so the user sees the rows instead of a wrapper.
fn pick_array(value: &Value) -> Value {
    if value.is_array() {
        return value.clone();
    }
    if let Some(obj) = value.as_object() {
        for key in ["items", "results", "rows", "data"] {
            if let Some(v) = obj.get(key) {
                if v.is_array() {
                    return v.clone();
                }
            }
        }
    }
    value.clone()
}

fn stringify_cell(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}
