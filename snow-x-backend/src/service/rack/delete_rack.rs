use crate::repository::rack::repository_delete_rack;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_delete_rack<C>(
    conn: &C,
    rack_id: Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    repository_delete_rack(conn, &rack_id).await
}