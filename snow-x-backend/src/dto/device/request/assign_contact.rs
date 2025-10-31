use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignContactRequest {
    pub contact_id: Uuid,
    pub role: Option<String>,
}
