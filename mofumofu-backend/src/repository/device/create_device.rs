use crate::entity::devices::{ActiveModel, Model as DeviceModel};
use crate::service::error::errors::Errors;
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub async fn repository_create_device<C>(
    conn: &C,
    rack_id: Option<&Uuid>,
    name: &str,
    description: Option<&str>,
    device_type: &str,
    manufacturer: Option<&str>,
    model: Option<&str>,
    serial_number: Option<&str>,
    rack_position: Option<i32>,
    rack_size: i32,
    power_consumption: Option<i32>,
    status: &str,
    purchase_date: Option<NaiveDate>,
    warranty_end: Option<NaiveDate>,
    created_by: &Uuid,
) -> Result<DeviceModel, Errors>
where
    C: ConnectionTrait,
{
    let new_device = ActiveModel {
        id: Set(Uuid::new_v4()),
        rack_id: Set(rack_id.copied()),
        name: Set(name.to_string()),
        description: Set(description.map(|s| s.to_string())),
        device_type: Set(device_type.to_string()),
        manufacturer: Set(manufacturer.map(|s| s.to_string())),
        model: Set(model.map(|s| s.to_string())),
        serial_number: Set(serial_number.map(|s| s.to_string())),
        rack_position: Set(rack_position),
        rack_size: Set(rack_size),
        power_consumption: Set(power_consumption),
        status: Set(status.to_string()),
        purchase_date: Set(purchase_date),
        warranty_end: Set(warranty_end),
        created_by: Set(*created_by),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        is_active: Set(true),
    };

    let device = new_device
        .insert(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(device)
}
