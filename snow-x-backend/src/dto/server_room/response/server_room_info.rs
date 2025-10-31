use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ServerRoomInfoResponse {
    pub id: Uuid,
    pub office_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_number: Option<String>,
    pub temperature_monitoring: bool,
    pub humidity_monitoring: bool,
    pub access_control: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for ServerRoomInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
