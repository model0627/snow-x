use crate::entity::ip_addresses;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use uuid::Uuid;

pub async fn service_create_bulk_ip_addresses(
    conn: &DatabaseConnection,
    ip_range_id: &Uuid,
    start_ip: &str,
    end_ip: &str,
    status: &str,
    description: Option<&str>,
    created_by: &Uuid,
) -> ServiceResult<Vec<ip_addresses::Model>> {
    // Validate status
    if ![
        "available",
        "reserved",
        "unavailable",
        "allocated",
        "expired",
    ]
    .contains(&status)
    {
        return Err(Errors::BadRequestError("Invalid status value".to_string()));
    }

    // Convert IP addresses to numbers for range calculation
    let start_num = ip_to_number(start_ip)?;
    let end_num = ip_to_number(end_ip)?;

    if start_num > end_num {
        return Err(Errors::BadRequestError(
            "Start IP must be less than or equal to end IP".to_string(),
        ));
    }

    let ip_count = end_num - start_num + 1;
    if ip_count > 1000 {
        return Err(Errors::BadRequestError(
            "Cannot create more than 1000 IP addresses at once".to_string(),
        ));
    }

    // Generate all IP addresses in the range
    let mut ip_addresses_list = Vec::new();
    for i in 0..ip_count {
        let ip_num = start_num + i;
        let ip_str = number_to_ip(ip_num);
        ip_addresses_list.push(ip_str);
    }

    // Bulk insert
    let now = chrono::Utc::now();
    let values: Vec<String> = ip_addresses_list
        .iter()
        .map(|ip| {
            let id = Uuid::new_v4();
            let desc_str = description
                .map(|d| format!("'{}'", d.replace("'", "''")))
                .unwrap_or_else(|| "NULL".to_string());
            format!(
                "('{}', '{}', '{}'::inet, '{}', {}, '{}', '{}', '{}', true)",
                id,
                ip_range_id,
                ip,
                status,
                desc_str,
                created_by,
                now.to_rfc3339(),
                now.to_rfc3339()
            )
        })
        .collect();

    let sql = format!(
        r#"
        INSERT INTO ip_addresses (
            id, ip_range_id, ip_address, status, description, created_by, created_at, updated_at, is_active
        ) VALUES {}
        ON CONFLICT (ip_address, ip_range_id) DO NOTHING
        RETURNING
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
        "#,
        values.join(", ")
    );

    let rows = conn
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            sql,
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let mut result = Vec::new();
    for row in rows {
        let model = ip_addresses::Model::from_query_result(&row, "")
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        result.push(model);
    }

    Ok(result)
}

fn ip_to_number(ip: &str) -> ServiceResult<u32> {
    let parts: Result<Vec<u32>, _> = ip.split('.').map(|s| s.parse::<u32>()).collect();

    match parts {
        Ok(p) if p.len() == 4 && p.iter().all(|&x| x <= 255) => {
            Ok((p[0] << 24) + (p[1] << 16) + (p[2] << 8) + p[3])
        }
        _ => Err(Errors::BadRequestError(format!(
            "Invalid IP address: {}",
            ip
        ))),
    }
}

fn number_to_ip(num: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (num >> 24) & 255,
        (num >> 16) & 255,
        (num >> 8) & 255,
        num & 255
    )
}

use sea_orm::FromQueryResult;
