use crate::entity::ip_ranges;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, DatabaseConnection, FromQueryResult, Statement};
use uuid::Uuid;

pub async fn service_get_ip_range_by_id(
    conn: &DatabaseConnection,
    id: &Uuid,
) -> ServiceResult<ip_ranges::Model> {
    let sql = r#"
        SELECT
            id,
            name,
            description,
            HOST(network_address) as network_address,
            subnet_mask,
            CASE WHEN gateway IS NOT NULL THEN HOST(gateway) ELSE NULL END as gateway,
            dns_servers,
            vlan_id,
            ip_version,
            created_by,
            created_at,
            updated_at,
            is_active
        FROM ip_ranges
        WHERE id = $1 AND is_active = true
    "#;

    let result = conn
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![(*id).into()],
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    if let Some(row) = result {
        ip_ranges::Model::from_query_result(&row, "")
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    } else {
        Err(Errors::NotFound("IP range not found".to_string()))
    }
}
