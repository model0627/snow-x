use crate::entity::server_rooms::{ActiveModel, Model as ServerRoomModel};
use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_server_room<C>(
    conn: &C,
    office_id: &Uuid,
    name: &str,
    description: Option<&str>,
    floor_level: Option<&str>,
    room_number: Option<&str>,
    temperature_monitoring: bool,
    humidity_monitoring: bool,
    access_control: bool,
    created_by: &Uuid,
) -> Result<ServerRoomModel, Errors>
where
    C: ConnectionTrait,
{
    let new_server_room = ActiveModel {
        id: Set(Uuid::new_v4()),
        office_id: Set(*office_id),
        name: Set(name.to_string()),
        description: Set(description.map(|s| s.to_string())),
        floor_level: Set(floor_level.map(|s| s.to_string())),
        room_number: Set(room_number.map(|s| s.to_string())),
        temperature_monitoring: Set(temperature_monitoring),
        humidity_monitoring: Set(humidity_monitoring),
        access_control: Set(access_control),
        created_by: Set(*created_by),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        is_active: Set(true),
    };

    let server_room = new_server_room
        .insert(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(server_room)
}