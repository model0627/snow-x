use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeviceLibrary::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeviceLibrary::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(DeviceLibrary::TenantId).uuid().not_null())
                    .col(ColumnDef::new(DeviceLibrary::Name).string().not_null())
                    .col(ColumnDef::new(DeviceLibrary::Description).text())
                    .col(ColumnDef::new(DeviceLibrary::DeviceType).string().not_null())
                    .col(ColumnDef::new(DeviceLibrary::Manufacturer).string())
                    .col(ColumnDef::new(DeviceLibrary::Model).string())
                    .col(
                        ColumnDef::new(DeviceLibrary::DefaultRackSize)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(ColumnDef::new(DeviceLibrary::DefaultPowerConsumption).integer())
                    .col(ColumnDef::new(DeviceLibrary::DefaultConfig).json())
                    .col(ColumnDef::new(DeviceLibrary::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(DeviceLibrary::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DeviceLibrary::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DeviceLibrary::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_device_library_tenant_id")
                            .from(DeviceLibrary::Table, DeviceLibrary::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_device_library_created_by")
                            .from(DeviceLibrary::Table, DeviceLibrary::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_tenant_id")
                    .table(DeviceLibrary::Table)
                    .col(DeviceLibrary::TenantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_name")
                    .table(DeviceLibrary::Table)
                    .col(DeviceLibrary::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_device_type")
                    .table(DeviceLibrary::Table)
                    .col(DeviceLibrary::DeviceType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_manufacturer")
                    .table(DeviceLibrary::Table)
                    .col(DeviceLibrary::Manufacturer)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_is_active")
                    .table(DeviceLibrary::Table)
                    .col(DeviceLibrary::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DeviceLibrary::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DeviceLibrary {
    Table,
    Id,
    TenantId,
    Name,
    Description,
    DeviceType,
    Manufacturer,
    Model,
    DefaultRackSize,
    DefaultPowerConsumption,
    DefaultConfig,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}