use crate::dto::contact::request::CreateContactRequest;
use crate::dto::contact::response::ContactInfoResponse;
use crate::entity::contacts;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use uuid::Uuid;

pub async fn service_create_contact(
    conn: &DatabaseConnection,
    request: CreateContactRequest,
    created_by: Uuid,
) -> ServiceResult<ContactInfoResponse> {
    let contact = contacts::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        name: ActiveValue::Set(request.name),
        title: ActiveValue::Set(request.title),
        department: ActiveValue::Set(request.department),
        phone: ActiveValue::Set(request.phone),
        mobile: ActiveValue::Set(request.mobile),
        email: ActiveValue::Set(request.email),
        office_location: ActiveValue::Set(request.office_location),
        responsibilities: ActiveValue::Set(request.responsibilities),
        created_by: ActiveValue::Set(created_by),
        created_at: ActiveValue::NotSet,
        updated_at: ActiveValue::NotSet,
        is_active: ActiveValue::Set(true),
        source_type: ActiveValue::Set("manual".to_string()),
        external_api_connection_id: ActiveValue::Set(None),
    }
    .insert(conn)
    .await?;

    Ok(ContactInfoResponse {
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
    })
}
