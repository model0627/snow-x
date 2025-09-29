use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateServerRoomRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_id: Option<Uuid>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_number: Option<String>,
    #[serde(default)]
    pub temperature_monitoring: bool,
    #[serde(default)]
    pub humidity_monitoring: bool,
    #[serde(default)]
    pub access_control: bool,
}