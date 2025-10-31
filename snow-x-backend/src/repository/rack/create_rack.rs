use crate::entity::racks::{ActiveModel, Model as RackModel};
use crate::service::error::errors::Errors;
use sea_orm::prelude::Decimal;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_rack<C>(
    conn: &C,
    server_room_id: &Uuid,
    name: &str,
    description: Option<&str>,
    rack_height: i32,
    power_capacity: Option<i32>,
    cooling_type: Option<&str>,
    location_x: Option<f64>,
    location_y: Option<f64>,
    created_by: &Uuid,
) -> Result<RackModel, Errors>
where
    C: ConnectionTrait,
{
    let new_rack = ActiveModel {
        id: Set(Uuid::new_v4()),
        server_room_id: Set(*server_room_id),
        name: Set(name.to_string()),
        description: Set(description.map(|s| s.to_string())),
        rack_height: Set(rack_height),
        power_capacity: Set(power_capacity),
        cooling_type: Set(cooling_type.map(|s| s.to_string())),
        location_x: Set(location_x.and_then(|x| Decimal::try_from(x).ok())),
        location_y: Set(location_y.and_then(|y| Decimal::try_from(y).ok())),
        created_by: Set(*created_by),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        is_active: Set(true),
    };

    let rack = new_rack
        .insert(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(rack)
}
