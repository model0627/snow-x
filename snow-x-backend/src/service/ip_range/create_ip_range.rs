use crate::entity::ip_ranges::{self, Entity as IpRange};
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter};
use uuid::Uuid;

pub async fn service_create_ip_range(
    conn: &DatabaseConnection,
    name: &str,
    description: Option<&str>,
    network_address: &str,
    subnet_mask: i32,
    gateway: Option<&str>,
    dns_servers: Option<Vec<String>>,
    vlan_id: Option<i32>,
    ip_version: i32,
    created_by: &Uuid,
) -> ServiceResult<ip_ranges::Model> {
    // Validate subnet mask
    if subnet_mask < 8 || subnet_mask > 32 {
        return Err(Errors::BadRequestError(
            "Subnet mask must be between 8 and 32".to_string(),
        ));
    }

    // Validate IP version
    if ip_version != 4 && ip_version != 6 {
        return Err(Errors::BadRequestError(
            "IP version must be 4 or 6".to_string(),
        ));
    }

    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    use sea_orm::ConnectionTrait;
    use sea_orm::Statement;

    // Convert DNS servers to JSON for storage in JSONB column
    let dns_json = dns_servers
        .as_ref()
        .map(|dns| serde_json::to_string(dns).unwrap());

    let sql = r#"
        INSERT INTO ip_ranges (
            id, name, description, network_address, subnet_mask,
            gateway, dns_servers, vlan_id, ip_version, created_by, created_at, updated_at, is_active
        ) VALUES (
            $1, $2, $3, $4::inet, $5,
            $6::inet, $7::jsonb, $8, $9, $10, $11, $12, $13
        )
        RETURNING
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
    "#;

    let gateway_str = gateway.map(|s| s.to_string());

    let result = conn
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![
                id.into(),
                name.into(),
                description.map(|s| s.to_string()).into(),
                network_address.into(),
                subnet_mask.into(),
                gateway_str.into(),
                dns_json.into(),
                vlan_id.into(),
                ip_version.into(),
                (*created_by).into(),
                now.into(),
                now.into(),
                true.into(),
            ],
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    if let Some(row) = result {
        let model = ip_ranges::Model::from_query_result(&row, "")
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        Ok(model)
    } else {
        Err(Errors::DatabaseError(
            "Failed to insert IP range".to_string(),
        ))
    }
}
