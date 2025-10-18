use crate::entity::devices::{ActiveModel, Column, Entity as DeviceEntity};
use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_delete_device<C>(conn: &C, device_id: &Uuid) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    let device = DeviceEntity::find()
        .filter(Column::Id.eq(*device_id))
        .filter(Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?
        .ok_or_else(|| Errors::NotFound("Device not found".to_string()))?;

    let mut device: ActiveModel = device.into();
    device.is_active = Set(false);
    device.updated_at = Set(chrono::Utc::now().into());

    device
        .update(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(())
}
