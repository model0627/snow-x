use crate::repository::device::delete_device::repository_delete_device;
use crate::service::error::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_delete_device(
    conn: &DatabaseConnection,
    device_id: Uuid,
) -> ServiceResult<()> {
    repository_delete_device(conn, &device_id).await?;
    Ok(())
}
