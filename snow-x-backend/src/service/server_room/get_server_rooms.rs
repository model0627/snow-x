use crate::dto::server_room::response::server_room_info::ServerRoomInfoResponse;
use crate::dto::server_room::response::server_room_list::ServerRoomListResponse;
use crate::repository::server_room::get_server_rooms::{
    repository_count_server_rooms, repository_get_server_rooms,
};
use crate::service::error::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_server_rooms(
    conn: &DatabaseConnection,
    office_id: &Uuid,
    page: u64,
    limit: u64,
) -> ServiceResult<ServerRoomListResponse> {
    let page = if page == 0 { 1 } else { page };
    let limit = if limit == 0 { 10 } else { limit };

    let server_rooms = repository_get_server_rooms(conn, office_id, page, limit).await?;
    let total_count = repository_count_server_rooms(conn, office_id).await?;
    let total_pages = (total_count + limit - 1) / limit;

    let server_room_responses: Vec<ServerRoomInfoResponse> = server_rooms
        .into_iter()
        .map(|server_room| ServerRoomInfoResponse {
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
        .collect();

    Ok(ServerRoomListResponse {
        server_rooms: server_room_responses,
        total_count,
        page,
        limit,
        total_pages,
    })
}
