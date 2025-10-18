pub mod assign_ip_address;
pub mod create_device;
pub mod delete_device;
pub mod get_device;
pub mod get_device_ip_addresses;
pub mod get_devices;
pub mod unassign_ip_address;
pub mod update_device;

pub use assign_ip_address::service_assign_ip_address;
pub use create_device::service_create_device;
pub use delete_device::service_delete_device;
pub use get_device::service_get_device_by_id;
pub use get_device_ip_addresses::service_get_device_ip_addresses;
pub use get_devices::service_get_devices;
pub use unassign_ip_address::service_unassign_ip_address;
pub use update_device::service_update_device;
