use crate::entity::device_ip_mappings;
use sea_orm::*;
use uuid::Uuid;

pub async fn service_assign_ip_address(
    conn: &DatabaseConnection,
    device_id: Uuid,
    ip_address_id: Uuid,
) -> Result<(), DbErr> {
    // 이미 할당되어 있는지 확인
    let existing = device_ip_mappings::Entity::find()
        .filter(device_ip_mappings::Column::DeviceId.eq(device_id))
        .filter(device_ip_mappings::Column::IpAddressId.eq(ip_address_id))
        .one(conn)
        .await?;

    if existing.is_some() {
        // 이미 할당되어 있으면 에러 반환
        return Err(DbErr::Custom(
            "IP address is already assigned to this device".to_string(),
        ));
    }

    // 트랜잭션 시작
    let txn = conn.begin().await?;

    // IP 주소 상태를 'assigned'로 업데이트 (Raw SQL 사용)
    let update_result = txn
        .execute(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
                UPDATE ip_addresses
                SET status = $1, updated_at = $2
                WHERE id = $3
            "#,
            vec![
                "assigned".into(),
                chrono::Utc::now().into(),
                ip_address_id.into(),
            ],
        ))
        .await?;

    if update_result.rows_affected() == 0 {
        return Err(DbErr::RecordNotFound("IP address not found".to_string()));
    }

    // 새로운 매핑 생성
    let new_mapping = device_ip_mappings::ActiveModel {
        id: Set(Uuid::new_v4()),
        device_id: Set(device_id),
        ip_address_id: Set(ip_address_id),
        interface_name: Set(None),
        is_primary: Set(false),
        created_at: Set(chrono::Utc::now().into()),
    };

    new_mapping.insert(&txn).await?;

    txn.commit().await?;

    Ok(())
}
