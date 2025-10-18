use crate::entity::devices::{Column, Entity as DeviceEntity, Model as DeviceModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_device_by_id<C>(
    conn: &C,
    device_id: &Uuid,
) -> Result<Option<DeviceModel>, Errors>
where
    C: ConnectionTrait,
{
    let device = DeviceEntity::find()
        .filter(Column::Id.eq(*device_id))
        .filter(Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(device)
}
