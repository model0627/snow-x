use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateLibraryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub device_type: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub default_rack_size: Option<i32>,
    pub default_power_consumption: Option<i32>,
    pub default_config: Option<serde_json::Value>,
    pub device_id: Option<Uuid>,
    pub device_name: Option<String>,
    pub is_active: Option<bool>,
    #[serde(default)]
    pub remove_device_link: Option<bool>,
}
