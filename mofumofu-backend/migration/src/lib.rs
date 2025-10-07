pub use sea_orm_migration::prelude::*;
mod common;
mod m20250717_064915_user_roles;
mod m20250717_064916_users;
mod m20250718_155828_user_refresh_tokens;
mod m20250718_155829_oauth_providers;
mod m20250718_155830_user_oauth_connections;
mod m20250718_162056_hashtags;
mod m20250718_162057_posts;
mod m20250718_162058_drafts;
mod m20250718_162101_post_hashtags;
mod m20250718_162102_comments;
mod m20250719_031841_follows;
mod m20250811_004451_action_types;
mod m20250811_004802_target_types;
mod m20250811_004808_system_events;
mod m20250815_103030_create_like_target_type_enum;
mod m20250815_103031_create_likes_table;
mod m20250820_061016_create_report_target_type_enum;
mod m20250820_061038_create_report_status_enum;
mod m20250820_061054_create_reports_table;
mod m20250912_000000_create_external_api_connections;
mod m20250912_000001_create_external_api_sync_logs;
mod m20250912_000002_create_external_api_data;
mod m20250912_000003_add_field_mapping;
mod m20250913_000000_create_offices_table;
mod m20250928_000000_create_tenants_table;
mod m20250928_000001_create_server_rooms_table;
mod m20250928_000002_create_racks_table;
mod m20250928_000003_create_ip_ranges_table;
mod m20250928_000004_create_ip_addresses_table;
mod m20250928_000005_create_devices_table;
mod m20250928_000006_create_device_ip_mappings_table;
mod m20250928_000007_create_device_library_table;
mod m20250928_000008_create_contacts_table;
mod m20251004_082913_alter_device_library_default_rack_size_nullable;
mod m20251004_083546_remove_device_library_tenant_id;
mod m20251004_084105_add_device_library_connections;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250717_064915_user_roles::Migration),
            Box::new(m20250717_064916_users::Migration),
            Box::new(m20250718_155828_user_refresh_tokens::Migration),
            Box::new(m20250718_155829_oauth_providers::Migration),
            Box::new(m20250718_155830_user_oauth_connections::Migration),
            Box::new(m20250718_162056_hashtags::Migration),
            Box::new(m20250718_162057_posts::Migration),
            Box::new(m20250718_162058_drafts::Migration),
            Box::new(m20250718_162101_post_hashtags::Migration),
            Box::new(m20250718_162102_comments::Migration),
            Box::new(m20250719_031841_follows::Migration),
            Box::new(m20250811_004451_action_types::Migration),
            Box::new(m20250811_004802_target_types::Migration),
            Box::new(m20250811_004808_system_events::Migration),
            Box::new(m20250815_103030_create_like_target_type_enum::Migration),
            Box::new(m20250815_103031_create_likes_table::Migration),
            Box::new(m20250820_061016_create_report_target_type_enum::Migration),
            Box::new(m20250820_061038_create_report_status_enum::Migration),
            Box::new(m20250820_061054_create_reports_table::Migration),
            Box::new(m20250912_000000_create_external_api_connections::Migration),
            Box::new(m20250912_000001_create_external_api_sync_logs::Migration),
            Box::new(m20250912_000002_create_external_api_data::Migration),
            Box::new(m20250912_000003_add_field_mapping::Migration),
            Box::new(m20250913_000000_create_offices_table::Migration),
            Box::new(m20250928_000000_create_tenants_table::Migration),
            Box::new(m20250928_000001_create_server_rooms_table::Migration),
            Box::new(m20250928_000002_create_racks_table::Migration),
            Box::new(m20250928_000003_create_ip_ranges_table::Migration),
            Box::new(m20250928_000004_create_ip_addresses_table::Migration),
            Box::new(m20250928_000005_create_devices_table::Migration),
            Box::new(m20250928_000006_create_device_ip_mappings_table::Migration),
            Box::new(m20250928_000007_create_device_library_table::Migration),
            Box::new(m20250928_000008_create_contacts_table::Migration),
            Box::new(m20251004_082913_alter_device_library_default_rack_size_nullable::Migration),
            Box::new(m20251004_083546_remove_device_library_tenant_id::Migration),
            Box::new(m20251004_084105_add_device_library_connections::Migration),
        ]
    }
}
