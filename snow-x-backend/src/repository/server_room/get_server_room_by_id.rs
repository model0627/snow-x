use crate::entity::server_rooms::{Entity as ServerRoomEntity, Model as ServerRoomModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_server_room_by_id<C>(
    conn: &C,
    server_room_id: &Uuid,
) -> Result<Option<ServerRoomModel>, Errors>
where
    C: ConnectionTrait,
{
    let server_room = ServerRoomEntity::find()
        .filter(crate::entity::server_rooms::Column::Id.eq(*server_room_id))
        .filter(crate::entity::server_rooms::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(server_room)
}
