use crate::entity::devices::{Column, Entity as DeviceEntity, Model as DeviceModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

pub async fn repository_get_devices<C>(
    conn: &C,
    page: u64,
    limit: u64,
    search: Option<&str>,
    device_type: Option<&str>,
    status: Option<&str>,
) -> Result<(Vec<DeviceModel>, u64), Errors>
where
    C: ConnectionTrait,
{
    let mut query = DeviceEntity::find().filter(Column::IsActive.eq(true));

    if let Some(search_term) = search {
        query = query.filter(Column::Name.contains(search_term));
    }

    if let Some(dtype) = device_type {
        query = query.filter(Column::DeviceType.eq(dtype));
    }

    if let Some(stat) = status {
        query = query.filter(Column::Status.eq(stat));
    }

    let total = query
        .clone()
        .count(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let devices = query
        .order_by_desc(Column::CreatedAt)
        .paginate(conn, limit)
        .fetch_page(page - 1)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok((devices, total))
}
