use crate::dto::contact::request::UpdateContactRequest;
use crate::dto::contact::response::ContactInfoResponse;
use crate::entity::contacts;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn service_update_contact(
    conn: &DatabaseConnection,
    id: Uuid,
    request: UpdateContactRequest,
) -> ServiceResult<ContactInfoResponse> {
    let contact = contacts::Entity::find_by_id(id)
        .filter(contacts::Column::IsActive.eq(true))
        .one(conn)
        .await?
        .ok_or_else(|| sea_orm::DbErr::RecordNotFound("Contact not found".to_string()))?;

    let mut active_model: contacts::ActiveModel = contact.into();

    if let Some(name) = request.name {
        active_model.name = ActiveValue::Set(name);
    }
    if request.title.is_some() {
        active_model.title = ActiveValue::Set(request.title);
    }
    if request.department.is_some() {
        active_model.department = ActiveValue::Set(request.department);
    }
    if request.phone.is_some() {
        active_model.phone = ActiveValue::Set(request.phone);
    }
    if request.mobile.is_some() {
        active_model.mobile = ActiveValue::Set(request.mobile);
    }
    if request.email.is_some() {
        active_model.email = ActiveValue::Set(request.email);
    }
    if request.office_location.is_some() {
        active_model.office_location = ActiveValue::Set(request.office_location);
    }
    if request.responsibilities.is_some() {
        active_model.responsibilities = ActiveValue::Set(request.responsibilities);
    }
    if let Some(is_active) = request.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    let updated_contact = active_model.update(conn).await?;

    Ok(ContactInfoResponse {
        id: updated_contact.id,
        name: updated_contact.name,
        title: updated_contact.title,
        department: updated_contact.department,
        phone: updated_contact.phone,
        mobile: updated_contact.mobile,
        email: updated_contact.email,
        office_location: updated_contact.office_location,
        responsibilities: updated_contact.responsibilities,
        created_by: updated_contact.created_by,
        created_at: updated_contact.created_at.to_string(),
        updated_at: updated_contact.updated_at.to_string(),
        is_active: updated_contact.is_active,
    })
}
