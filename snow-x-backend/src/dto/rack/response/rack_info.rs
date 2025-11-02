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
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_room_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_name: Option<String>,
    pub device_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_percentage: Option<f64>,
    pub used_units: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<RackDeviceSummary>,
}

impl IntoResponse for RackInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RackDeviceSummary {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_position: Option<i32>,
    pub rack_size: i32,
}
