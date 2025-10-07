use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateDeviceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_id: Option<Uuid>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub device_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_position: Option<i32>,
    #[serde(default = "default_rack_size")]
    pub rack_size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption: Option<i32>,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_end: Option<NaiveDate>,
}

fn default_rack_size() -> i32 {
    1
}

fn default_status() -> String {
    "active".to_string()
}
