use crate::dto::device::request::create_device::CreateDeviceRequest;
use crate::dto::device::response::device_info::DeviceInfoResponse;
use crate::repository::device::create_device::repository_create_device;
use crate::service::error::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_create_device(
    conn: &DatabaseConnection,
    request: CreateDeviceRequest,
    created_by: Uuid,
) -> ServiceResult<DeviceInfoResponse> {
    let device = repository_create_device(
        conn,
        request.rack_id.as_ref(),
        &request.name,
        request.description.as_deref(),
        &request.device_type,
        request.manufacturer.as_deref(),
        request.model.as_deref(),
        request.serial_number.as_deref(),
        request.rack_position,
        request.rack_size,
        request.power_consumption,
        &request.status,
        request.purchase_date,
        request.warranty_end,
        &created_by,
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
