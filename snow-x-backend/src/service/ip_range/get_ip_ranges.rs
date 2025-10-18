use crate::entity::ip_ranges;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, Statement, ConnectionTrait, FromQueryResult};
use uuid::Uuid;

pub struct IpRangeListResult {
    pub ip_ranges: Vec<ip_ranges::Model>,
    pub total: u64,
}

pub async fn service_get_ip_ranges(
    conn: &DatabaseConnection,
    page: u64,
    limit: u64,
) -> ServiceResult<IpRangeListResult> {
    // Get total count
    let count_sql = r#"
        SELECT COUNT(*) as count
        FROM ip_ranges
        WHERE is_active = true
    "#;

    #[derive(FromQueryResult)]
    struct CountResult {
        count: i64,
    }

    let count_result = conn
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            count_sql,
            vec![],
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?
        .ok_or_else(|| Errors::DatabaseError("Failed to get count".to_string()))?;

    let count = CountResult::from_query_result(&count_result, "")
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let total = count.count as u64;

    // Get paginated results with INET cast to TEXT
    let offset = (page.saturating_sub(1)) * limit;
    let sql = r#"
        SELECT
            id,
            name,
            description,
            HOST(network_address) as network_address,
            subnet_mask,
            CASE WHEN gateway IS NOT NULL THEN HOST(gateway) ELSE NULL END as gateway,
            ARRAY_TO_JSON(dns_servers)::TEXT as dns_servers,
            vlan_id,
            ip_version,
            created_by,
            created_at,
            updated_at,
            is_active
        FROM ip_ranges
        WHERE is_active = true
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    "#;

    let rows = conn
        .query_all(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![(limit as i64).into(), (offset as i64).into()],
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let mut ip_ranges = Vec::new();
    for row in rows {
        let model = ip_ranges::Model::from_query_result(&row, "")
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        ip_ranges.push(model);
    }

    Ok(IpRangeListResult { ip_ranges, total })
}
