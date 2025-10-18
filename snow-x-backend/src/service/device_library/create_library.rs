use crate::dto::device_library::request::CreateLibraryRequest;
use crate::dto::device_library::response::LibraryInfoResponse;
use crate::entity::device_library;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use uuid::Uuid;

pub async fn service_create_library(
    conn: &DatabaseConnection,
    request: CreateLibraryRequest,
    created_by: Uuid,
) -> ServiceResult<LibraryInfoResponse> {
    let library = device_library::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        name: ActiveValue::Set(request.name),
        description: ActiveValue::Set(request.description),
        device_type: ActiveValue::Set(request.device_type),
        manufacturer: ActiveValue::Set(request.manufacturer),
        model: ActiveValue::Set(request.model),
        default_rack_size: ActiveValue::Set(request.default_rack_size),
        default_power_consumption: ActiveValue::Set(request.default_power_consumption),
        default_config: ActiveValue::Set(request.default_config),
        device_id: ActiveValue::Set(request.device_id),
        device_name: ActiveValue::Set(request.device_name),
        created_by: ActiveValue::Set(created_by),
        created_at: ActiveValue::NotSet,
        updated_at: ActiveValue::NotSet,
        is_active: ActiveValue::Set(true),
        source_type: ActiveValue::Set("manual".to_string()),
        external_api_connection_id: ActiveValue::Set(None),
    }
    .insert(conn)
    .await?;

    Ok(LibraryInfoResponse {
        id: library.id,
        name: library.name,
        description: library.description,
        device_type: library.device_type,
        manufacturer: library.manufacturer,
        model: library.model,
        default_rack_size: library.default_rack_size,
        default_power_consumption: library.default_power_consumption,
        default_config: library.default_config,
        device_id: library.device_id,
        device_name: library.device_name,
        created_by: library.created_by,
        created_at: library.created_at.to_string(),
        updated_at: library.updated_at.to_string(),
        is_active: library.is_active,
        source_type: library.source_type,
        external_api_connection_id: library.external_api_connection_id,
    })
}
