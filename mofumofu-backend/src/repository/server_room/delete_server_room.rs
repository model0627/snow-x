use crate::entity::server_rooms::{ActiveModel, Entity as ServerRoomEntity};
use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_delete_server_room<C>(
    conn: &C,
    server_room_id: &Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    // First check if server room exists
    let existing_server_room = ServerRoomEntity::find()
        .filter(crate::entity::server_rooms::Column::Id.eq(*server_room_id))
        .filter(crate::entity::server_rooms::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    if let Some(server_room) = existing_server_room {
        let mut server_room_to_delete: ActiveModel = server_room.into();
        server_room_to_delete.is_active = Set(false);
        server_room_to_delete.updated_at = Set(chrono::Utc::now().into());

        server_room_to_delete
            .update(conn)
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        Ok(true)
    } else {
        Ok(false)
    }
}