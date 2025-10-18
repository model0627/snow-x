use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateServerRoomRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_level: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_number: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_monitoring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity_monitoring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<bool>,
}