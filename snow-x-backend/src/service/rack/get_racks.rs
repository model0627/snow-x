use super::mapper::build_rack_response;
use crate::dto::rack::response::rack_info::RackInfoResponse;
use crate::dto::rack::response::rack_list::RackListResponse;
use crate::repository::rack::repository_get_racks;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;

pub async fn service_get_racks<C>(
    conn: &C,
    page: u64,
    limit: u64,
) -> Result<RackListResponse, Errors>
where
    C: ConnectionTrait,
{
    let (racks, total) = repository_get_racks(conn, page, limit).await?;

    let mut rack_responses: Vec<RackInfoResponse> = Vec::with_capacity(racks.len());
    for rack in racks {
        let response = build_rack_response(conn, rack).await?;
        rack_responses.push(response);
    }

    Ok(RackListResponse {
        racks: rack_responses,
        total: total as i64,
        page,
        limit,
    })
}
