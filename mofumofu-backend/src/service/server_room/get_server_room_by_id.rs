use crate::dto::server_room::response::server_room_info::ServerRoomInfoResponse;
use crate::repository::server_room::get_server_room_by_id::repository_get_server_room_by_id;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_server_room_by_id(
    conn: &DatabaseConnection,
    server_room_id: &Uuid,
) -> ServiceResult<ServerRoomInfoResponse> {
    let server_room = repository_get_server_room_by_id(conn, server_room_id)
        .await?
        .ok_or(Errors::ServerRoomNotFound)?;

    Ok(ServerRoomInfoResponse {
        id: server_room.id,
        office_id: server_room.office_id,
        name: server_room.name,
        description: server_room.description,
        floor_level: server_room.floor_level,
        room_number: server_room.room_number,
        temperature_monitoring: server_room.temperature_monitoring,
        humidity_monitoring: server_room.humidity_monitoring,
        access_control: server_room.access_control,
        created_at: server_room.created_at.into(),
        updated_at: server_room.updated_at.into(),
    })
}