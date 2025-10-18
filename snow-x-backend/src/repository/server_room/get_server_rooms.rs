use crate::entity::server_rooms::{Entity as ServerRoomEntity, Model as ServerRoomModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect, QueryOrder, PaginatorTrait};
use uuid::Uuid;

pub async fn repository_get_server_rooms<C>(
    conn: &C,
    office_id: &Uuid,
    page: u64,
    limit: u64,
) -> Result<Vec<ServerRoomModel>, Errors>
where
    C: ConnectionTrait,
{
    let offset = (page.saturating_sub(1)) * limit;

    let server_rooms = ServerRoomEntity::find()
        .filter(crate::entity::server_rooms::Column::OfficeId.eq(*office_id))
        .filter(crate::entity::server_rooms::Column::IsActive.eq(true))
        .order_by_asc(crate::entity::server_rooms::Column::Name)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(server_rooms)
}

pub async fn repository_count_server_rooms<C>(
    conn: &C,
    office_id: &Uuid,
) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let count = ServerRoomEntity::find()
        .filter(crate::entity::server_rooms::Column::OfficeId.eq(*office_id))
        .filter(crate::entity::server_rooms::Column::IsActive.eq(true))
        .count(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(count)
}