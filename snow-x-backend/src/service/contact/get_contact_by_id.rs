use crate::dto::contact::response::ContactInfoResponse;
use crate::entity::contacts;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn service_get_contact_by_id(
    conn: &DatabaseConnection,
    id: Uuid,
) -> ServiceResult<Option<ContactInfoResponse>> {
    let contact = contacts::Entity::find_by_id(id)
        .filter(contacts::Column::IsActive.eq(true))
        .one(conn)
        .await?;

    Ok(contact.map(|contact| ContactInfoResponse {
        id: contact.id,
        name: contact.name,
        title: contact.title,
        department: contact.department,
        phone: contact.phone,
        mobile: contact.mobile,
        email: contact.email,
        office_location: contact.office_location,
        responsibilities: contact.responsibilities,
        created_by: contact.created_by,
        created_at: contact.created_at.to_string(),
        updated_at: contact.updated_at.to_string(),
        is_active: contact.is_active,
        source_type: contact.source_type,
        external_api_connection_id: contact.external_api_connection_id,
    }))
}
