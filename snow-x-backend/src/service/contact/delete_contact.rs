use crate::entity::contacts;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use uuid::Uuid;

pub async fn service_delete_contact(
    conn: &DatabaseConnection,
    id: Uuid,
) -> ServiceResult<()> {
    let contact = contacts::Entity::find_by_id(id)
        .filter(contacts::Column::IsActive.eq(true))
        .one(conn)
        .await?
        .ok_or_else(|| sea_orm::DbErr::RecordNotFound("Contact not found".to_string()))?;

    let mut active_model = contact.into_active_model();
    active_model.is_active = ActiveValue::Set(false);
    active_model.update(conn).await?;

    Ok(())
}
