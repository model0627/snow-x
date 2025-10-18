use crate::entity::server_rooms::{ActiveModel, Entity as ServerRoomEntity, Model as ServerRoomModel};
use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_update_server_room<C>(
    conn: &C,
    server_room_id: &Uuid,
    name: Option<&str>,
    description: Option<Option<&str>>,
    floor_level: Option<Option<&str>>,
    room_number: Option<Option<&str>>,
    temperature_monitoring: Option<bool>,
    humidity_monitoring: Option<bool>,
    access_control: Option<bool>,
) -> Result<Option<ServerRoomModel>, Errors>
where
    C: ConnectionTrait,
{
    // First check if server room exists
    let existing_server_room = ServerRoomEntity::find()
        .filter(crate::entity::server_rooms::Column::Id.eq(*server_room_id))
        .filter(crate::entity::server_rooms::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    if existing_server_room.is_none() {
        return Ok(None);
    }

    let mut server_room_to_update: ActiveModel = existing_server_room.unwrap().into();

    // Update fields if provided
    if let Some(name) = name {
        server_room_to_update.name = Set(name.to_string());
    }
    if let Some(description) = description {
        server_room_to_update.description = Set(description.map(|s| s.to_string()));
    }
    if let Some(floor_level) = floor_level {
        server_room_to_update.floor_level = Set(floor_level.map(|s| s.to_string()));
    }
    if let Some(room_number) = room_number {
        server_room_to_update.room_number = Set(room_number.map(|s| s.to_string()));
    }
    if let Some(temperature_monitoring) = temperature_monitoring {
        server_room_to_update.temperature_monitoring = Set(temperature_monitoring);
    }
    if let Some(humidity_monitoring) = humidity_monitoring {
        server_room_to_update.humidity_monitoring = Set(humidity_monitoring);
    }
    if let Some(access_control) = access_control {
        server_room_to_update.access_control = Set(access_control);
    }

    server_room_to_update.updated_at = Set(chrono::Utc::now().into());

    let updated_server_room = server_room_to_update
        .update(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(Some(updated_server_room))
}