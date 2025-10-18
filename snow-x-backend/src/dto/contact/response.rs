use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactInfoResponse {
    pub id: Uuid,
    pub name: String,
    pub title: Option<String>,
    pub department: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub office_location: Option<String>,
    pub responsibilities: Option<String>,
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
    pub source_type: String,
    pub external_api_connection_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactListResponse {
    pub contacts: Vec<ContactInfoResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceMappingResponse {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub contact_name: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub role: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceMappingListResponse {
    pub mappings: Vec<ResourceMappingResponse>,
    pub total: u64,
}
