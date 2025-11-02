use crate::dto::rack::response::rack_info::{RackDeviceSummary, RackInfoResponse};
use crate::entity::{devices, office, racks::Model as RackModel, server_rooms};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::debug;

pub async fn build_rack_response<C>(conn: &C, rack: RackModel) -> Result<RackInfoResponse, Errors>
where
    C: ConnectionTrait,
{
    debug!(rack_id = %rack.id, "building rack response");

    let server_room = server_rooms::Entity::find()
        .filter(server_rooms::Column::Id.eq(rack.server_room_id))
        .filter(server_rooms::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let (server_room_name, office_name) = if let Some(server_room) = server_room {
        let office = office::Entity::find()
            .filter(office::Column::Id.eq(server_room.office_id))
            .filter(office::Column::IsActive.eq(true))
            .one(conn)
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        let office_name = office.map(|office| office.name);

        (Some(server_room.name), office_name)
    } else {
        (None, None)
    };

    let devices = devices::Entity::find()
        .filter(devices::Column::RackId.eq(Some(rack.id)))
        .filter(devices::Column::IsActive.eq(true))
        .all(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    debug!(
        rack_id = %rack.id,
        server_room_name = ?server_room_name,
        office_name = ?office_name,
        raw_device_count = devices.len(),
        "rack related entities loaded"
    );

    let mut device_summaries = Vec::with_capacity(devices.len());
    let mut used_units = 0_i32;

    for device in devices {
        let sanitized_size = if device.rack_size > 0 {
            device.rack_size
        } else {
            1
        };
        used_units += sanitized_size;

        device_summaries.push(RackDeviceSummary {
            id: device.id,
            name: device.name,
            device_type: Some(device.device_type),
            status: Some(device.status),
            manufacturer: device.manufacturer,
            model: device.model,
            serial_number: device.serial_number,
            rack_position: device.rack_position,
            rack_size: sanitized_size,
        });
    }

    let device_count = device_summaries.len() as i32;

    let usage_percentage = if rack.rack_height > 0 && used_units >= 0 {
        let ratio = used_units as f64 / rack.rack_height as f64;
        Some((ratio * 100.0).clamp(0.0, 100.0))
    } else if used_units > 0 {
        Some(100.0)
    } else {
        Some(0.0)
    };

    Ok(RackInfoResponse {
        id: rack.id,
        server_room_id: rack.server_room_id,
        name: rack.name,
        description: rack.description,
        rack_height: rack.rack_height,
        power_capacity: rack.power_capacity,
        cooling_type: rack.cooling_type,
        location_x: rack.location_x.and_then(|d| d.to_string().parse().ok()),
        location_y: rack.location_y.and_then(|d| d.to_string().parse().ok()),
        created_at: rack.created_at.into(),
        updated_at: rack.updated_at.into(),
        is_active: rack.is_active,
        server_room_name,
        office_name,
        device_count,
        usage_percentage,
        used_units,
        devices: device_summaries,
    })
}
