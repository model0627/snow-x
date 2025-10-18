use crate::dto::device::response::device_info::DeviceInfoResponse;
use crate::dto::device::response::device_list::DeviceListResponse;
use crate::repository::device::get_devices::repository_get_devices;
use crate::service::error::errors::ServiceResult;
use sea_orm::DatabaseConnection;

pub async fn service_get_devices(
    conn: &DatabaseConnection,
    page: u64,
    limit: u64,
    search: Option<String>,
    device_type: Option<String>,
    status: Option<String>,
) -> ServiceResult<DeviceListResponse> {
    let (devices, total) = repository_get_devices(
        conn,
        page,
        limit,
        search.as_deref(),
        device_type.as_deref(),
        status.as_deref(),
    )
    .await?;

    let device_responses: Vec<DeviceInfoResponse> = devices
        .into_iter()
        .map(|device| DeviceInfoResponse {
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
            source_type: device.source_type,
            external_api_connection_id: device.external_api_connection_id,
        })
        .collect();

    Ok(DeviceListResponse {
        devices: device_responses,
        total,
        page,
        limit,
    })
}
