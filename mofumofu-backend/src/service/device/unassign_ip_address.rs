use crate::entity::{device_ip_mappings, ip_addresses};
use sea_orm::*;
use uuid::Uuid;

pub async fn service_unassign_ip_address(
    conn: &DatabaseConnection,
    device_id: Uuid,
    ip_address_id: Uuid,
) -> Result<(), DbErr> {
    println!("DEBUG: Starting unassign - device_id: {}, ip_address_id: {}", device_id, ip_address_id);

    // 매핑을 찾아서 삭제 (Raw SQL로 변경 - INET 타입 문제 회피)
    println!("DEBUG: Finding mapping with raw SQL...");

    #[derive(FromQueryResult)]
    struct MappingResult {
        id: Uuid,
    }

    let mapping_result = MappingResult::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
                SELECT id
                FROM device_ip_mappings
                WHERE device_id = $1 AND ip_address_id = $2
                LIMIT 1
            "#,
            vec![device_id.into(), ip_address_id.into()],
        ),
    )
    .one(conn)
    .await?;

    println!("DEBUG: Mapping found: {:?}", mapping_result.is_some());

    match mapping_result {
        Some(mapping) => {
            println!("DEBUG: Starting transaction...");
            // 트랜잭션 시작
            let txn = conn.begin().await?;

            println!("DEBUG: Deleting mapping with id: {}", mapping.id);
            // 매핑 삭제 (Raw SQL 사용)
            txn.execute(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"
                    DELETE FROM device_ip_mappings
                    WHERE id = $1
                "#,
                vec![mapping.id.into()],
            ))
            .await?;

            println!("DEBUG: Mapping deleted, updating IP status...");
            // IP 주소 상태를 'available'로 업데이트 (Raw SQL 사용)
            let update_result = txn
                .execute(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"
                        UPDATE ip_addresses
                        SET status = $1, updated_at = $2
                        WHERE id = $3
                    "#,
                    vec![
                        "available".into(),
                        chrono::Utc::now().into(),
                        ip_address_id.into(),
                    ],
                ))
                .await?;

            println!("DEBUG: IP status updated, rows affected: {}", update_result.rows_affected());

            if update_result.rows_affected() == 0 {
                return Err(DbErr::RecordNotFound("IP address not found".to_string()));
            }

            println!("DEBUG: Committing transaction...");
            txn.commit().await?;

            println!("DEBUG: Transaction committed successfully");
            Ok(())
        }
        None => {
            println!("DEBUG: Mapping not found");
            Err(DbErr::RecordNotFound(
                "IP address mapping not found".to_string(),
            ))
        }
    }
}
