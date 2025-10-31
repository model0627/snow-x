use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateRackRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_room_id: Option<Uuid>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "default_rack_height")]
    pub rack_height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooling_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_y: Option<f64>,
}

fn default_rack_height() -> i32 {
    42
}
