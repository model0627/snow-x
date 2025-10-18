use crate::entity::racks::{Entity as Rack, Model as RackModel};
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, ColumnTrait};

pub async fn repository_get_racks<C>(
    conn: &C,
    page: u64,
    limit: u64,
) -> Result<(Vec<RackModel>, u64), Errors>
where
    C: ConnectionTrait,
{
    let query = Rack::find().filter(crate::entity::racks::Column::IsActive.eq(true));

    let total = query
        .clone()
        .count(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let racks = query
        .paginate(conn, limit)
        .fetch_page(page - 1)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok((racks, total))
}