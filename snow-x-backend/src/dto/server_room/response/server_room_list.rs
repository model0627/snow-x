use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::server_room_info::ServerRoomInfoResponse;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ServerRoomListResponse {
    pub server_rooms: Vec<ServerRoomInfoResponse>,
    pub total_count: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl IntoResponse for ServerRoomListResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
