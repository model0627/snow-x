use crate::dto::device_library::response::{LibraryInfoResponse, LibraryListResponse};
use crate::entity::device_library;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

pub async fn service_get_libraries(
    conn: &DatabaseConnection,
    page: u64,
    limit: u64,
    search: Option<String>,
) -> ServiceResult<LibraryListResponse> {
    let mut query = device_library::Entity::find()
        .filter(device_library::Column::IsActive.eq(true));

    if let Some(search_term) = search {
        query = query.filter(
            device_library::Column::Name
                .contains(&search_term)
                .or(device_library::Column::DeviceType.contains(&search_term)),
        );
    }

    let paginator = query
        .order_by_desc(device_library::Column::CreatedAt)
        .paginate(conn, limit);

    let total = paginator.num_items().await?;
    let libraries = paginator.fetch_page(page - 1).await?;

    let library_responses: Vec<LibraryInfoResponse> = libraries
        .into_iter()
        .map(|library| LibraryInfoResponse {
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
        })
        .collect();

    Ok(LibraryListResponse {
        libraries: library_responses,
        total,
        page,
        limit,
    })
}
