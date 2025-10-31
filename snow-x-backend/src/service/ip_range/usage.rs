use std::collections::HashMap;

use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseConnection, FromQueryResult, Statement, Value,
};
use uuid::Uuid;

use crate::service::error::errors::{Errors, ServiceResult};

#[derive(Debug, Clone, Default)]
pub struct RangeUsageStats {
    pub available: i64,
    pub allocated: i64,
    pub reserved: i64,
    pub unavailable: i64,
    pub expired: i64,
    pub other: i64,
}

impl RangeUsageStats {
    pub fn total(&self) -> i64 {
        self.available
            + self.allocated
            + self.reserved
            + self.unavailable
            + self.expired
            + self.other
    }

    pub fn used(&self) -> i64 {
        self.total() - self.available
    }
}

#[derive(FromQueryResult)]
struct UsageRow {
    ip_range_id: Uuid,
    status: String,
    count: i64,
}

pub async fn fetch_ip_range_usage(
    conn: &DatabaseConnection,
    range_ids: &[Uuid],
) -> ServiceResult<HashMap<Uuid, RangeUsageStats>> {
    let mut usage_map = HashMap::new();

    if range_ids.is_empty() {
        return Ok(usage_map);
    }

    let placeholders: Vec<String> = (1..=range_ids.len())
        .map(|idx| format!("${}", idx))
        .collect();
    let sql = format!(
        r#"
        SELECT ip_range_id, status, COUNT(*) as count
        FROM ip_addresses
        WHERE is_active = true
          AND ip_range_id IN ({})
        GROUP BY ip_range_id, status
        "#,
        placeholders.join(", ")
    );

    let values: Vec<Value> = range_ids.iter().map(|id| Value::from(*id)).collect();

    let rows = conn
        .query_all(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            &sql,
            values,
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    for row in rows {
        let record = UsageRow::from_query_result(&row, "")
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        let entry = usage_map.entry(record.ip_range_id).or_default();
        match record.status.to_lowercase().as_str() {
            "available" => entry.available += record.count,
            "allocated" => entry.allocated += record.count,
            "reserved" => entry.reserved += record.count,
            "unavailable" => entry.unavailable += record.count,
            "expired" => entry.expired += record.count,
            _ => entry.other += record.count,
        }
    }

    Ok(usage_map)
}
