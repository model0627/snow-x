use crate::dto::device::request::update_device::UpdateDeviceRequest;
use crate::dto::device::response::device_info::DeviceInfoResponse;
use crate::repository::device::update_device::repository_update_device;
use crate::service::error::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_update_device(
    conn: &DatabaseConnection,
    device_id: Uuid,
    request: UpdateDeviceRequest,
) -> ServiceResult<DeviceInfoResponse> {
    let device = repository_update_device(
        conn,
        &device_id,
        request.rack_id.map(Some),
        request.name.as_deref(),
        Some(request.description.as_deref()),
        request.device_type.as_deref(),
        Some(request.manufacturer.as_deref()),
        Some(request.model.as_deref()),
        Some(request.serial_number.as_deref()),
        request.rack_position.map(Some),
        request.rack_size,
        request.power_consumption.map(Some),
        request.status.as_deref(),
        request.purchase_date.map(Some),
        request.warranty_end.map(Some),
    )
    .await?;

    Ok(DeviceInfoResponse {
        id: device.id,
        rack_id: device.rack_id,
        name: device.name,
        description: device.description,
        device_type: device.device_type,
        manufacturer: device.manufacturer,
        model: device.model,
        serial_number: device.serial_number,
        rack_position: device.rack_position,
        rack_size: device.rack_size,
        power_consumption: device.power_consumption,
        status: device.status,
        purchase_date: device.purchase_date,
        warranty_end: device.warranty_end,
        created_by: device.created_by,
        created_at: device.created_at.into(),
        updated_at: device.updated_at.into(),
        is_active: device.is_active,
    })
}
