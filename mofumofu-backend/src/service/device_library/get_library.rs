use crate::dto::device_library::response::LibraryInfoResponse;
use crate::entity::device_library;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn service_get_library_by_id(
    conn: &DatabaseConnection,
    id: Uuid,
) -> ServiceResult<Option<LibraryInfoResponse>> {
    let library = device_library::Entity::find_by_id(id)
        .filter(device_library::Column::IsActive.eq(true))
        .one(conn)
        .await?;

    Ok(library.map(|library| LibraryInfoResponse {
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
    }))
}
