use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeviceIpMappings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeviceIpMappings::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(DeviceIpMappings::DeviceId).uuid().not_null())
                    .col(ColumnDef::new(DeviceIpMappings::IpAddressId).uuid().not_null())
                    .col(ColumnDef::new(DeviceIpMappings::InterfaceName).string())
                    .col(
                        ColumnDef::new(DeviceIpMappings::IsPrimary)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(DeviceIpMappings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_device_ip_mappings_device_id")
                            .from(DeviceIpMappings::Table, DeviceIpMappings::DeviceId)
                            .to(Devices::Table, Devices::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_device_ip_mappings_ip_address_id")
                            .from(DeviceIpMappings::Table, DeviceIpMappings::IpAddressId)
                            .to(IpAddresses::Table, IpAddresses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint for device_id and ip_address_id combination
        manager
            .create_index(
                Index::create()
                    .name("uniq_device_ip_mappings_device_ip")
                    .table(DeviceIpMappings::Table)
                    .col(DeviceIpMappings::DeviceId)
                    .col(DeviceIpMappings::IpAddressId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_device_ip_mappings_device_id")
                    .table(DeviceIpMappings::Table)
                    .col(DeviceIpMappings::DeviceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_ip_mappings_ip_address_id")
                    .table(DeviceIpMappings::Table)
                    .col(DeviceIpMappings::IpAddressId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_ip_mappings_is_primary")
                    .table(DeviceIpMappings::Table)
                    .col(DeviceIpMappings::IsPrimary)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DeviceIpMappings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DeviceIpMappings {
    Table,
    Id,
    DeviceId,
    IpAddressId,
    InterfaceName,
    IsPrimary,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Devices {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum IpAddresses {
    Table,
    Id,
}