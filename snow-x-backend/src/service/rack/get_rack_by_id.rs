use crate::dto::rack::response::rack_info::RackInfoResponse;
use crate::repository::rack::repository_get_rack_by_id;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;
use std::str::FromStr;
use uuid::Uuid;

pub async fn service_get_rack_by_id<C>(
    conn: &C,
    rack_id: Uuid,
) -> Result<Option<RackInfoResponse>, Errors>
where
    C: ConnectionTrait,
{
    let rack = repository_get_rack_by_id(conn, &rack_id).await?;

    if let Some(rack) = rack {
        Ok(Some(RackInfoResponse {
            id: rack.id,
            server_room_id: rack.server_room_id,
            name: rack.name,
            description: rack.description,
            rack_height: rack.rack_height,
            power_capacity: rack.power_capacity,
            cooling_type: rack.cooling_type,
            location_x: rack.location_x.and_then(|d| d.to_string().parse().ok()),
            location_y: rack.location_y.and_then(|d| d.to_string().parse().ok()),
            created_at: rack.created_at.into(),
            updated_at: rack.updated_at.into(),
        }))
    } else {
        Ok(None)
    }
}
