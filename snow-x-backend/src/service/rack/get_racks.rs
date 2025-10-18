use crate::dto::rack::response::rack_info::RackInfoResponse;
use crate::dto::rack::response::rack_list::RackListResponse;
use crate::repository::rack::repository_get_racks;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;
use std::str::FromStr;

pub async fn service_get_racks<C>(
    conn: &C,
    page: u64,
    limit: u64,
) -> Result<RackListResponse, Errors>
where
    C: ConnectionTrait,
{
    let (racks, total) = repository_get_racks(conn, page, limit).await?;

    let rack_responses: Vec<RackInfoResponse> = racks
        .into_iter()
        .map(|rack| RackInfoResponse {
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
        })
        .collect();

    Ok(RackListResponse {
        racks: rack_responses,
        total: total as i64,
        page,
        limit,
    })
}