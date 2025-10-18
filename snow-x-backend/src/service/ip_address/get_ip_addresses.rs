use crate::entity::ip_addresses;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, Statement, ConnectionTrait, FromQueryResult};
use uuid::Uuid;

pub struct IpAddressListResult {
    pub ip_addresses: Vec<ip_addresses::Model>,
    pub total: u64,
}

pub async fn service_get_ip_addresses(
    conn: &DatabaseConnection,
    ip_range_id: Option<&Uuid>,
    status: Option<&str>,
    search: Option<&str>,
    page: u64,
    limit: u64,
) -> ServiceResult<IpAddressListResult> {
    // Build WHERE clause
    let mut where_clauses = vec!["is_active = true".to_string()];
    let mut param_idx = 1;
    let mut params: Vec<sea_orm::Value> = vec![];

    if let Some(range_id) = ip_range_id {
        where_clauses.push(format!("ip_range_id = ${}", param_idx));
        params.push((*range_id).into());
        param_idx += 1;
    }

    if let Some(s) = status {
        where_clauses.push(format!("status = ${}", param_idx));
        params.push(s.to_string().into());
        param_idx += 1;
    }

    if let Some(search_term) = search {
        where_clauses.push(format!(
            "(HOST(ip_address) ILIKE ${} OR mac_address ILIKE ${} OR hostname ILIKE ${} OR description ILIKE ${})",
            param_idx, param_idx, param_idx, param_idx
        ));
        let search_pattern = format!("%{}%", search_term);
        params.push(search_pattern.into());
        param_idx += 1;
    }

    let where_clause = where_clauses.join(" AND ");

    // Get total count
    let count_sql = format!(
        "SELECT COUNT(*) as count FROM ip_addresses WHERE {}",
        where_clause
    );

    #[derive(FromQueryResult)]
    struct CountResult {
        count: i64,
    }

    let count_result = conn
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &count_sql,
            params.clone(),
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?
        .ok_or_else(|| Errors::DatabaseError("Failed to get count".to_string()))?;

    let count = CountResult::from_query_result(&count_result, "")
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let total = count.count as u64;

    // Get paginated results
    let offset = (page.saturating_sub(1)) * limit;
    let sql = format!(
        r#"
        SELECT
            id,
            ip_range_id,
            HOST(ip_address) as ip_address,
            mac_address,
            hostname,
            status,
            description,
            created_by,
            created_at,
            updated_at,
            is_active
        FROM ip_addresses
        WHERE {}
        ORDER BY ip_address
        LIMIT ${} OFFSET ${}
        "#,
        where_clause, param_idx, param_idx + 1
    );

    params.push((limit as i64).into());
    params.push((offset as i64).into());

    let rows = conn
        .query_all(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &sql,
            params,
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let mut ip_addresses_list = Vec::new();
    for row in rows {
        let model = ip_addresses::Model::from_query_result(&row, "")
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        ip_addresses_list.push(model);
    }

    Ok(IpAddressListResult {
        ip_addresses: ip_addresses_list,
        total
    })
}
