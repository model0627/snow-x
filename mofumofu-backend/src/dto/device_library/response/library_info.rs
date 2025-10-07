use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LibraryInfoResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub device_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub default_rack_size: Option<i32>,
    pub default_power_consumption: Option<i32>,
    pub default_config: Option<serde_json::Value>,
    pub device_id: Option<Uuid>,
    pub device_name: Option<String>,
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
}
