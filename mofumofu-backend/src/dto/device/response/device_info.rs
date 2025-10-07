use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DeviceInfoResponse {
    pub id: Uuid,
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
    pub rack_size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption: Option<i32>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_end: Option<NaiveDate>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_api_connection_id: Option<i32>,
}

impl IntoResponse for DeviceInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
