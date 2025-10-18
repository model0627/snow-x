use crate::entity::ip_ranges;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, Statement, ConnectionTrait, FromQueryResult};
use uuid::Uuid;

pub async fn service_update_ip_range(
    conn: &DatabaseConnection,
    id: &Uuid,
    name: Option<String>,
    description: Option<String>,
    network_address: Option<String>,
    subnet_mask: Option<i32>,
    gateway: Option<String>,
    dns_servers: Option<Vec<String>>,
    vlan_id: Option<i32>,
    ip_version: Option<i32>,
) -> ServiceResult<ip_ranges::Model> {
    // Validate subnet mask if provided
    if let Some(mask) = subnet_mask {
        if mask < 8 || mask > 32 {
            return Err(Errors::BadRequestError(
                "Subnet mask must be between 8 and 32".to_string(),
            ));
        }
    }

    // Validate IP version if provided
    if let Some(version) = ip_version {
        if version != 4 && version != 6 {
            return Err(Errors::BadRequestError(
                "IP version must be 4 or 6".to_string(),
            ));
        }
    }

    let now = chrono::Utc::now();
    let mut updates = Vec::new();
    let mut params: Vec<sea_orm::Value> = vec![(*id).into()];
    let mut param_idx = 2;

    if let Some(n) = name {
        updates.push(format!("name = ${}", param_idx));
        params.push(n.into());
        param_idx += 1;
    }
    if let Some(d) = description {
        updates.push(format!("description = ${}", param_idx));
        params.push(d.into());
        param_idx += 1;
    }
    if let Some(na) = network_address {
        updates.push(format!("network_address = ${}::inet", param_idx));
        params.push(na.into());
        param_idx += 1;
    }
    if let Some(sm) = subnet_mask {
        updates.push(format!("subnet_mask = ${}", param_idx));
        params.push(sm.into());
        param_idx += 1;
    }
    if let Some(g) = gateway {
        updates.push(format!("gateway = ${}::inet", param_idx));
        params.push(g.into());
        param_idx += 1;
    }
    if let Some(dns) = dns_servers {
        let dns_array = {
            let items: Vec<String> = dns.iter().map(|s| format!("\"{}\"", s)).collect();
            format!("{{{}}}", items.join(","))
        };
        updates.push(format!("dns_servers = ${}::text[]", param_idx));
        params.push(dns_array.into());
        param_idx += 1;
    }
    if let Some(vlan) = vlan_id {
        updates.push(format!("vlan_id = ${}", param_idx));
        params.push(vlan.into());
        param_idx += 1;
    }
    if let Some(version) = ip_version {
        updates.push(format!("ip_version = ${}", param_idx));
        params.push(version.into());
        param_idx += 1;
    }

    updates.push(format!("updated_at = ${}", param_idx));
    params.push(now.into());

    if updates.is_empty() {
        return Err(Errors::BadRequestError("No fields to update".to_string()));
    }

    let sql = format!(
        r#"
            UPDATE ip_ranges
            SET {}
            WHERE id = $1 AND is_active = true
            RETURNING
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
        "#,
        updates.join(", ")
    );

    let result = conn
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &sql,
            params,
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
