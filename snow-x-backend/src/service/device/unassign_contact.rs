use crate::entity::contact_resource_mappings::{self, ResourceType};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn service_unassign_contact_from_device(
    conn: &DatabaseConnection,
    device_id: Uuid,
    contact_id: Uuid,
) -> Result<(), DbErr> {
    let delete_result = contact_resource_mappings::Entity::delete_many()
        .filter(contact_resource_mappings::Column::ResourceType.eq(ResourceType::Device))
        .filter(contact_resource_mappings::Column::ResourceId.eq(device_id))
        .filter(contact_resource_mappings::Column::ContactId.eq(contact_id))
        .exec(conn)
        .await?;

    if delete_result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(
            "Contact assignment not found".to_string(),
        ));
    }

    Ok(())
}
