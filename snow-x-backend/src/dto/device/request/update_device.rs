use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateDeviceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_end: Option<NaiveDate>,
}
