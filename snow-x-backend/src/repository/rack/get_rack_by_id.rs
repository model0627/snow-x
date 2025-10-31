use crate::entity::racks::{Entity as Rack, Model as RackModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_rack_by_id<C>(
    conn: &C,
    rack_id: &Uuid,
) -> Result<Option<RackModel>, Errors>
where
    C: ConnectionTrait,
{
    let rack = Rack::find()
        .filter(crate::entity::racks::Column::Id.eq(*rack_id))
        .filter(crate::entity::racks::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(rack)
}
