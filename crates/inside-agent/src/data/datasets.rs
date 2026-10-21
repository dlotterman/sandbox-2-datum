//! The `data.catalog` and `data.query` skills, over the `platforms` reference table.
//!
//! Queries are built from templates, never from free-form SQL: column names come only from an allowlist
//! and every value is a bound parameter, so nothing the caller sends can change the shape of the statement.
use serde_json::{json, Map, Value};
use tokio_postgres::types::ToSql;

use super::{files, postgres};
use crate::{
    app::App,
    policy,
    skill::{SkillError, SkillOutput, SkillResult},
};

/// The only table that can be queried, and the only columns of it that can be selected or filtered on.
const DATASET: &str = "platforms";
const COLUMNS: [&str; 5] = ["name", "kind", "signals", "regions", "notes"];

const MAX_ROWS: u64 = 50;
const MAX_FILTER_VALUE_CHARS: usize = 100;

/// `data.catalog`: what datasets and files this gateway can serve.
pub async fn catalog(app: &App) -> SkillResult {
    let mut datasets = Vec::new();
    if app.postgres.is_configured() {
        // An unreachable database just means the catalog lists no datasets (yet).
        if let Ok(client) = postgres::connect(app).await {
            let rows: i64 =
                client.query_one("SELECT count(*) FROM platforms", &[]).await.map(|row| row.get(0)).unwrap_or(0);
            datasets.push(json!({
                "name": DATASET,
                "description": "Hosting platforms with typical runtime signals, regions and notes (reference data)",
                "columns": COLUMNS,
                "rows": rows,
            }));
        }
    }

    let files: Vec<Value> = if app.rclone.is_configured() {
        let listed = files::list_files(app).await.map_err(SkillError::UpstreamFailed)?;
        listed.into_iter().map(|file| json!({"name": file.name, "bytes": file.size})).collect()
    } else {
        Vec::new()
    };
    Ok(SkillOutput::new("catalog", json!({"datasets": datasets, "files": files})))
}

/// `data.query`: read rows from the reference table. Rules first, then the LLM review, then the query.
pub async fn query(app: &App, args: &Value) -> SkillResult {
    if args["dataset"] != DATASET {
        return Err(rule("unknown dataset (see data.catalog)"));
    }
    let columns = requested_columns(args)?;
    let filters = requested_filters(args)?;
    let limit = args["limit"].as_u64().unwrap_or(MAX_ROWS).clamp(1, MAX_ROWS);

    let purpose = policy::require_purpose(args)?;
    policy::review(app, "data.query", &purpose, args).await?;

    let (sql, values) = build_select(&columns, &filters, limit);
    let parameters: Vec<&(dyn ToSql + Sync)> = values.iter().map(|value| value as &(dyn ToSql + Sync)).collect();
    let client = postgres::connect(app).await.map_err(SkillError::UpstreamFailed)?;
    let rows = client.query(&sql, &parameters).await.map_err(|e| SkillError::UpstreamFailed(e.to_string()))?;

    let rows: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut object = Map::new();
            for (position, column) in columns.iter().enumerate() {
                object.insert(column.to_string(), json!(row.get::<_, Option<String>>(position)));
            }
            Value::Object(object)
        })
        .collect();
    Ok(SkillOutput::new(format!("{} row(s) from {DATASET}", rows.len()), json!({"dataset": DATASET, "rows": rows})))
}

/// The columns the caller wants (all of them if it did not say), each checked against the allowlist.
fn requested_columns(args: &Value) -> Result<Vec<&'static str>, SkillError> {
    let Some(requested) = args["columns"].as_array() else {
        return Ok(COLUMNS.to_vec());
    };
    let mut columns = Vec::new();
    for entry in requested {
        let name = entry.as_str().unwrap_or("");
        let allowed = COLUMNS.iter().find(|column| **column == name);
        columns.push(*allowed.ok_or_else(|| rule(format!("column {name:?} not allowed")))?);
    }
    if columns.is_empty() {
        return Err(rule("columns must not be empty"));
    }
    Ok(columns)
}

/// Equality filters as `(column, value)` pairs; the column must be allowlisted and the value a short string.
fn requested_filters(args: &Value) -> Result<Vec<(&'static str, String)>, SkillError> {
    let Some(requested) = args["filters"].as_object() else {
        return Ok(Vec::new());
    };
    let mut filters = Vec::new();
    for (name, value) in requested {
        let column = COLUMNS
            .iter()
            .find(|column| **column == name.as_str())
            .ok_or_else(|| rule(format!("filter column {name:?} not allowed")))?;
        let value = value.as_str().filter(|text| text.len() <= MAX_FILTER_VALUE_CHARS).ok_or_else(|| {
            rule(format!("filter values must be strings of at most {MAX_FILTER_VALUE_CHARS} characters"))
        })?;
        filters.push((*column, value.to_string()));
    }
    Ok(filters)
}

/// The SQL for a validated request, plus the values to bind to its `$1, $2, ...` placeholders.
/// Column names are spliced into the text (they came from the allowlist); values never are.
fn build_select(columns: &[&str], filters: &[(&str, String)], limit: u64) -> (String, Vec<String>) {
    let conditions: Vec<String> =
        filters.iter().enumerate().map(|(position, (column, _))| format!("{column} = ${}", position + 1)).collect();
    let where_clause =
        if conditions.is_empty() { String::new() } else { format!(" WHERE {}", conditions.join(" AND ")) };
    let sql = format!("SELECT {} FROM {DATASET}{where_clause} ORDER BY name LIMIT {limit}", columns.join(", "));
    let values = filters.iter().map(|(_, value)| value.clone()).collect();
    (sql, values)
}

fn rule(message: impl Into<String>) -> SkillError {
    SkillError::RuleDenied(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::app;

    #[tokio::test]
    async fn queries_only_reach_allowlisted_datasets_columns_and_filters() {
        let app = app("http://127.0.0.1:1", "k");
        let cases = [
            ("unknown dataset", json!({"dataset": "agent_audit"})),
            ("column outside the allowlist", json!({"dataset": "platforms", "columns": ["name", "password"]})),
            ("sql in a column name", json!({"dataset": "platforms", "columns": ["name; drop table platforms"]})),
            ("filter column outside the allowlist", json!({"dataset": "platforms", "filters": {"id": "1"}})),
            ("non-string filter value", json!({"dataset": "platforms", "filters": {"name": {"$ne": ""}}})),
            ("no purpose", json!({"dataset": "platforms", "columns": ["name"]})),
        ];
        for (name, args) in cases {
            let result = query(&app, &args).await;
            assert!(matches!(result, Err(SkillError::RuleDenied(_))), "{name}: expected a rule denial, got {result:?}");
        }
    }

    #[test]
    fn filter_values_are_bound_parameters_and_never_part_of_the_sql_text() {
        let hostile = "x'; DROP TABLE platforms; --".to_string();
        let (sql, values) = build_select(&["name", "kind"], &[("kind", hostile.clone())], 5);
        assert_eq!(sql, "SELECT name, kind FROM platforms WHERE kind = $1 ORDER BY name LIMIT 5");
        assert_eq!(values, vec![hostile]);
    }
}
