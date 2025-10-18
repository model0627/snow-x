use crate::entity::device_library;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use uuid::Uuid;

pub async fn service_delete_library(
    conn: &DatabaseConnection,
    id: Uuid,
) -> ServiceResult<()> {
    let library = device_library::Entity::find_by_id(id)
        .filter(device_library::Column::IsActive.eq(true))
        .one(conn)
        .await?
        .ok_or_else(|| sea_orm::DbErr::RecordNotFound("Library not found".to_string()))?;

    let mut active_model = library.into_active_model();
    active_model.is_active = ActiveValue::Set(false);
    active_model.update(conn).await?;

    Ok(())
}
