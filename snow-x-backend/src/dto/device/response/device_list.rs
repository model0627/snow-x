use crate::dto::device::response::device_info::DeviceInfoResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DeviceListResponse {
    pub devices: Vec<DeviceInfoResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}
