use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateRackRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooling_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_y: Option<f64>,
}