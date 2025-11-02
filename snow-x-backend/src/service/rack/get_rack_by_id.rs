use super::mapper::build_rack_response;
use crate::dto::rack::response::rack_info::RackInfoResponse;
use crate::repository::rack::repository_get_rack_by_id;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;
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
        let response = build_rack_response(conn, rack).await?;
        Ok(Some(response))
    } else {
        Ok(None)
    }
}
