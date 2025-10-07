use crate::dto::device_library::request::UpdateLibraryRequest;
use crate::dto::device_library::response::LibraryInfoResponse;
use crate::entity::device_library;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn service_update_library(
    conn: &DatabaseConnection,
    id: Uuid,
    request: UpdateLibraryRequest,
) -> ServiceResult<LibraryInfoResponse> {
    let library = device_library::Entity::find_by_id(id)
        .filter(device_library::Column::IsActive.eq(true))
        .one(conn)
        .await?
        .ok_or_else(|| sea_orm::DbErr::RecordNotFound("Library not found".to_string()))?;

    let mut active_model: device_library::ActiveModel = library.into();

    if let Some(name) = request.name {
        active_model.name = ActiveValue::Set(name);
    }
    if request.description.is_some() {
        active_model.description = ActiveValue::Set(request.description);
    }
    if let Some(device_type) = request.device_type {
        active_model.device_type = ActiveValue::Set(device_type);
    }
    if request.manufacturer.is_some() {
        active_model.manufacturer = ActiveValue::Set(request.manufacturer);
    }
    if request.model.is_some() {
        active_model.model = ActiveValue::Set(request.model);
    }
    if request.default_rack_size.is_some() {
        active_model.default_rack_size = ActiveValue::Set(request.default_rack_size);
    }
    if request.default_power_consumption.is_some() {
        active_model.default_power_consumption = ActiveValue::Set(request.default_power_consumption);
    }
    if request.default_config.is_some() {
        active_model.default_config = ActiveValue::Set(request.default_config);
    }
    if request.device_id.is_some() {
        active_model.device_id = ActiveValue::Set(request.device_id);
    }
    if request.device_name.is_some() {
        active_model.device_name = ActiveValue::Set(request.device_name);
    }
    if let Some(is_active) = request.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    let updated_library = active_model.update(conn).await?;

    Ok(LibraryInfoResponse {
        id: updated_library.id,
        name: updated_library.name,
        description: updated_library.description,
        device_type: updated_library.device_type,
        manufacturer: updated_library.manufacturer,
        model: updated_library.model,
        default_rack_size: updated_library.default_rack_size,
        default_power_consumption: updated_library.default_power_consumption,
        default_config: updated_library.default_config,
        device_id: updated_library.device_id,
        device_name: updated_library.device_name,
        created_by: updated_library.created_by,
        created_at: updated_library.created_at.to_string(),
        updated_at: updated_library.updated_at.to_string(),
        is_active: updated_library.is_active,
    })
}
