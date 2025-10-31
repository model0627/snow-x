use crate::dto::contact::response::{ResourceMappingListResponse, ResourceMappingResponse};
use crate::entity::{
    contact_resource_mappings::{self, ResourceType},
    contacts,
};
use sea_orm::{ActiveEnum, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn service_get_device_contacts(
    conn: &DatabaseConnection,
    device_id: Uuid,
) -> Result<ResourceMappingListResponse, DbErr> {
    let mappings = contact_resource_mappings::Entity::find()
        .filter(contact_resource_mappings::Column::ResourceType.eq(ResourceType::Device))
        .filter(contact_resource_mappings::Column::ResourceId.eq(device_id))
        .find_also_related(contacts::Entity)
        .all(conn)
        .await?;

    let mut responses = Vec::with_capacity(mappings.len());

    for (mapping, contact) in mappings {
        let contact_name = contact
            .as_ref()
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "알 수 없음".to_string());

        responses.push(ResourceMappingResponse {
            id: mapping.id,
            contact_id: mapping.contact_id,
            contact_name,
            resource_type: mapping.resource_type.clone().to_value(),
            resource_id: mapping.resource_id,
            role: mapping.role.clone(),
            created_at: mapping.created_at.to_string(),
        });
    }

    Ok(ResourceMappingListResponse {
        total: responses.len() as u64,
        mappings: responses,
    })
}
