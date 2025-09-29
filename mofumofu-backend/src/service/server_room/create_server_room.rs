use crate::dto::server_room::request::create_server_room::CreateServerRoomRequest;
use crate::dto::server_room::response::server_room_info::ServerRoomInfoResponse;
use crate::repository::server_room::create_server_room::repository_create_server_room;
use crate::service::error::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_create_server_room(
    conn: &DatabaseConnection,
    request: CreateServerRoomRequest,
    created_by: &Uuid,
) -> ServiceResult<ServerRoomInfoResponse> {
    let office_id = request.office_id.expect("office_id should be set by handler");
    let server_room = repository_create_server_room(
        conn,
        &office_id,
        &request.name,
        request.description.as_deref(),
        request.floor_level.as_deref(),
        request.room_number.as_deref(),
        request.temperature_monitoring,
        request.humidity_monitoring,
        request.access_control,
        created_by,
    )
    .await?;

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