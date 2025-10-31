use crate::dto::server_room::request::update_server_room::UpdateServerRoomRequest;
use crate::dto::server_room::response::server_room_info::ServerRoomInfoResponse;
use crate::repository::server_room::update_server_room::repository_update_server_room;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_update_server_room(
    conn: &DatabaseConnection,
    server_room_id: &Uuid,
    request: UpdateServerRoomRequest,
) -> ServiceResult<ServerRoomInfoResponse> {
    let server_room = repository_update_server_room(
        conn,
        server_room_id,
        request.name.as_deref(),
        request.description.as_ref().map(|o| o.as_deref()),
        request.floor_level.as_ref().map(|o| o.as_deref()),
        request.room_number.as_ref().map(|o| o.as_deref()),
        request.temperature_monitoring,
        request.humidity_monitoring,
        request.access_control,
    )
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
