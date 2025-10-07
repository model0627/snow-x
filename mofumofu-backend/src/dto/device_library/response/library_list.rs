use super::library_info::LibraryInfoResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LibraryListResponse {
    pub libraries: Vec<LibraryInfoResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}
