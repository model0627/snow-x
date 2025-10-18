use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use super::rack_info::RackInfoResponse;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RackListResponse {
    pub racks: Vec<RackInfoResponse>,
    pub total: i64,
    pub page: u64,
    pub limit: u64,
}

impl IntoResponse for RackListResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}