use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use uuid::Uuid;

pub async fn service_delete_ip_range(conn: &DatabaseConnection, id: &Uuid) -> ServiceResult<()> {
    let now = chrono::Utc::now();

    let sql = r#"
        UPDATE ip_ranges
        SET is_active = false, updated_at = $2
        WHERE id = $1 AND is_active = true
    "#;

    let result = conn
        .execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![(*id).into(), now.into()],
        ))
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(Errors::NotFound("IP range not found".to_string()));
    }

    Ok(())
}
