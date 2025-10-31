use crate::entity::{
    contact_resource_mappings::{self, ResourceType},
    contacts,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

pub async fn service_assign_contact_to_device(
    conn: &DatabaseConnection,
    device_id: Uuid,
    contact_id: Uuid,
    role: Option<String>,
) -> Result<(), DbErr> {
    // Ensure contact exists
    let contact_exists = contacts::Entity::find_by_id(contact_id).one(conn).await?;
    if contact_exists.is_none() {
        return Err(DbErr::RecordNotFound("Contact not found".into()));
    }

    // Check if mapping already exists
    let existing = contact_resource_mappings::Entity::find()
        .filter(contact_resource_mappings::Column::ResourceType.eq(ResourceType::Device))
        .filter(contact_resource_mappings::Column::ResourceId.eq(device_id))
        .filter(contact_resource_mappings::Column::ContactId.eq(contact_id))
        .one(conn)
        .await?;

    if existing.is_some() {
        return Err(DbErr::Custom(
            "Contact is already assigned to this device".to_string(),
        ));
    }

    let mapping = contact_resource_mappings::ActiveModel {
        id: Set(Uuid::new_v4()),
        contact_id: Set(contact_id),
        resource_type: Set(ResourceType::Device),
        resource_id: Set(device_id),
        role: Set(role),
        created_at: Set(Utc::now().into()),
    };

    mapping.insert(conn).await?;

    Ok(())
}
