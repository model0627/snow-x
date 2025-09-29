use crate::entity::racks::{ActiveModel, Entity as Rack};
use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_delete_rack<C>(
    conn: &C,
    rack_id: &Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    // 소프트 삭제 (is_active = false로 설정)
    let rack = Rack::find()
        .filter(crate::entity::racks::Column::Id.eq(*rack_id))
        .filter(crate::entity::racks::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    if let Some(rack) = rack {
        let mut active_rack: ActiveModel = rack.into();
        active_rack.is_active = Set(false);
        active_rack.updated_at = Set(chrono::Utc::now().into());

        active_rack
            .update(conn)
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
    }

    Ok(())
}