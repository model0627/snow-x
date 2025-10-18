use crate::dto::device::response::device_info::DeviceInfoResponse;
use crate::repository::device::get_device_by_id::repository_get_device_by_id;
use crate::service::error::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_device_by_id(
    conn: &DatabaseConnection,
    device_id: Uuid,
) -> ServiceResult<Option<DeviceInfoResponse>> {
    let device = repository_get_device_by_id(conn, &device_id).await?;

    Ok(device.map(|d| DeviceInfoResponse {
        id: d.id,
        rack_id: d.rack_id,
        name: d.name,
        description: d.description,
        device_type: d.device_type,
        manufacturer: d.manufacturer,
        model: d.model,
        serial_number: d.serial_number,
        rack_position: d.rack_position,
        rack_size: d.rack_size,
        power_consumption: d.power_consumption,
        status: d.status,
        purchase_date: d.purchase_date,
        warranty_end: d.warranty_end,
        created_by: d.created_by,
        created_at: d.created_at.into(),
        updated_at: d.updated_at.into(),
        is_active: d.is_active,
        source_type: d.source_type,
        external_api_connection_id: d.external_api_connection_id,
    }))
}
