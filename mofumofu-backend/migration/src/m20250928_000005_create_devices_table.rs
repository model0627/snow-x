use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Devices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Devices::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Devices::RackId).uuid())
                    .col(ColumnDef::new(Devices::Name).string().not_null())
                    .col(ColumnDef::new(Devices::Description).text())
                    .col(ColumnDef::new(Devices::DeviceType).string().not_null())
                    .col(ColumnDef::new(Devices::Manufacturer).string())
                    .col(ColumnDef::new(Devices::Model).string())
                    .col(ColumnDef::new(Devices::SerialNumber).string())
                    .col(ColumnDef::new(Devices::RackPosition).integer())
                    .col(
                        ColumnDef::new(Devices::RackSize)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(ColumnDef::new(Devices::PowerConsumption).integer())
                    .col(
                        ColumnDef::new(Devices::Status)
                            .string()
                            .not_null()
                            .default("active"),
                    )
                    .col(ColumnDef::new(Devices::PurchaseDate).date())
                    .col(ColumnDef::new(Devices::WarrantyEnd).date())
                    .col(ColumnDef::new(Devices::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Devices::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Devices::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Devices::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_devices_rack_id")
                            .from(Devices::Table, Devices::RackId)
                            .to(Racks::Table, Racks::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_devices_created_by")
                            .from(Devices::Table, Devices::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Note: Check constraint for status is handled at application level

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_devices_rack_id")
                    .table(Devices::Table)
                    .col(Devices::RackId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_devices_name")
                    .table(Devices::Table)
                    .col(Devices::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_devices_device_type")
                    .table(Devices::Table)
                    .col(Devices::DeviceType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_devices_status")
                    .table(Devices::Table)
                    .col(Devices::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_devices_serial_number")
                    .table(Devices::Table)
                    .col(Devices::SerialNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_devices_is_active")
                    .table(Devices::Table)
                    .col(Devices::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Devices::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Devices {
    Table,
    Id,
    RackId,
    Name,
    Description,
    DeviceType,
    Manufacturer,
    Model,
    SerialNumber,
    RackPosition,
    RackSize,
    PowerConsumption,
    Status,
    PurchaseDate,
    WarrantyEnd,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum Racks {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}