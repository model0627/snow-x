use crate::repository::server_room::delete_server_room::repository_delete_server_room;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_delete_server_room(
    conn: &DatabaseConnection,
    server_room_id: &Uuid,
) -> ServiceResult<()> {
    let deleted = repository_delete_server_room(conn, server_room_id).await?;

    if deleted {
        Ok(())
    } else {
        Err(Errors::ServerRoomNotFound)
    }
}
