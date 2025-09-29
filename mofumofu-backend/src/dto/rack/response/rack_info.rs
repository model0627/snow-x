use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RackInfoResponse {
    pub id: Uuid,
    pub server_room_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub rack_height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooling_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_y: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for RackInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}