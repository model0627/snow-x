use crate::api::v0::routes::ip_address::handlers::IpAddressResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::*;
use uuid::Uuid;

pub async fn service_get_device_ip_addresses(
    conn: &DatabaseConnection,
    device_id: Uuid,
) -> Result<Vec<IpAddressResponse>, DbErr> {
    use sea_orm::FromQueryResult;

    // Raw SQL로 INET와 MACADDR을 TEXT로 캐스팅해서 조회
    #[derive(FromQueryResult)]
    struct IpAddressRaw {
        id: Uuid,
        ip_range_id: Uuid,
        ip_address: String,
        status: String,
        hostname: Option<String>,
        description: Option<String>,
        mac_address: Option<String>,
        created_by: Uuid,
        created_at: DateTimeWithTimeZone,
        updated_at: DateTimeWithTimeZone,
        is_active: bool,
    }

    let sql = r#"
        SELECT
            ip.id,
            ip.ip_range_id,
            ip.ip_address::text as ip_address,
            ip.status,
            ip.hostname,
            ip.description,
            ip.mac_address::text as mac_address,
            ip.created_by,
            ip.created_at,
            ip.updated_at,
            ip.is_active
        FROM ip_addresses ip
        INNER JOIN device_ip_mappings dm ON ip.id = dm.ip_address_id
        WHERE dm.device_id = $1
    "#;

    let ip_addresses_raw =
        IpAddressRaw::find_by_statement(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![device_id.into()],
        ))
        .all(conn)
        .await?;

    let ip_responses: Vec<IpAddressResponse> = ip_addresses_raw
        .into_iter()
        .map(|ip| IpAddressResponse {
            id: ip.id,
            ip_range_id: ip.ip_range_id,
            ip_address: ip.ip_address,
            mac_address: ip.mac_address,
            hostname: ip.hostname,
            status: ip.status,
            description: ip.description,
            created_by: ip.created_by,
            created_at: ip.created_at.to_string(),
            updated_at: ip.updated_at.to_string(),
            is_active: ip.is_active,
        })
        .collect();

    Ok(ip_responses)
}
