use crate::entity::devices::{ActiveModel, Column, Entity as DeviceEntity, Model as DeviceModel};
use crate::service::error::errors::Errors;
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub async fn repository_update_device<C>(
    conn: &C,
    device_id: &Uuid,
    rack_id: Option<Option<Uuid>>,
    name: Option<&str>,
    description: Option<Option<&str>>,
    device_type: Option<&str>,
    manufacturer: Option<Option<&str>>,
    model: Option<Option<&str>>,
    serial_number: Option<Option<&str>>,
    rack_position: Option<Option<i32>>,
    rack_size: Option<i32>,
    power_consumption: Option<Option<i32>>,
    status: Option<&str>,
    purchase_date: Option<Option<NaiveDate>>,
    warranty_end: Option<Option<NaiveDate>>,
) -> Result<DeviceModel, Errors>
where
    C: ConnectionTrait,
{
    let device = DeviceEntity::find()
        .filter(Column::Id.eq(*device_id))
        .filter(Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?
        .ok_or_else(|| Errors::NotFound("Device not found".to_string()))?;

    let mut device: ActiveModel = device.into();

    if let Some(rid) = rack_id {
        device.rack_id = Set(rid);
    }
    if let Some(n) = name {
        device.name = Set(n.to_string());
    }
    if let Some(desc) = description {
        device.description = Set(desc.map(|s| s.to_string()));
    }
    if let Some(dtype) = device_type {
        device.device_type = Set(dtype.to_string());
    }
    if let Some(mfr) = manufacturer {
        device.manufacturer = Set(mfr.map(|s| s.to_string()));
    }
    if let Some(mdl) = model {
        device.model = Set(mdl.map(|s| s.to_string()));
    }
    if let Some(sn) = serial_number {
        device.serial_number = Set(sn.map(|s| s.to_string()));
    }
    if let Some(rp) = rack_position {
        device.rack_position = Set(rp);
    }
    if let Some(rs) = rack_size {
        device.rack_size = Set(rs);
    }
    if let Some(pc) = power_consumption {
        device.power_consumption = Set(pc);
    }
    if let Some(stat) = status {
        device.status = Set(stat.to_string());
    }
    if let Some(pd) = purchase_date {
        device.purchase_date = Set(pd);
    }
    if let Some(we) = warranty_end {
        device.warranty_end = Set(we);
    }

    device.updated_at = Set(chrono::Utc::now().into());

    let updated_device = device
        .update(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(updated_device)
}
