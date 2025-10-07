use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateContactRequest {
    pub name: String,
    pub title: Option<String>,
    pub department: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub office_location: Option<String>,
    pub responsibilities: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateContactRequest {
    pub name: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub office_location: Option<String>,
    pub responsibilities: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactQueryParams {
    pub search: Option<String>,
    pub department: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignResourceRequest {
    pub contact_id: Uuid,
    pub resource_type: String,  // office, server_room, rack, device, ip_range
    pub resource_id: Uuid,
    pub role: Option<String>,  // manager, backup, technical, etc.
}
