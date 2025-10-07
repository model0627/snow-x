use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add device_id and device_name to device_library table
        manager
            .alter_table(
                Table::alter()
                    .table(DeviceLibrary::Table)
                    .add_column(
                        ColumnDef::new(DeviceLibrary::DeviceId)
                            .uuid()
                            .null()
                    )
                    .add_column(
                        ColumnDef::new(DeviceLibrary::DeviceName)
                            .string_len(255)
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        // Add foreign key for device_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_device_library_device_id")
                    .from(DeviceLibrary::Table, DeviceLibrary::DeviceId)
                    .to(Devices::Table, Devices::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // Create index on device_id
        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_device_id")
                    .table(DeviceLibrary::Table)
                    .col(DeviceLibrary::DeviceId)
                    .to_owned(),
            )
            .await?;

        // Create device_library_mappings table for many-to-many relationships
        manager
            .create_table(
                Table::create()
                    .table(DeviceLibraryMappings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::DeviceId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::LibraryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::InstalledAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::InstalledBy)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::InstallPath)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::Configuration)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::IsPrimary)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::IsActive)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceLibraryMappings::LastChecked)
                            .timestamp()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add foreign keys for device_library_mappings
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_device_library_mappings_device_id")
                    .from(DeviceLibraryMappings::Table, DeviceLibraryMappings::DeviceId)
                    .to(Devices::Table, Devices::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_device_library_mappings_library_id")
                    .from(DeviceLibraryMappings::Table, DeviceLibraryMappings::LibraryId)
                    .to(DeviceLibrary::Table, DeviceLibrary::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_device_library_mappings_installed_by")
                    .from(DeviceLibraryMappings::Table, DeviceLibraryMappings::InstalledBy)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // Create unique constraint on device_id and library_id
        manager
            .create_index(
                Index::create()
                    .name("unique_device_library_mapping")
                    .table(DeviceLibraryMappings::Table)
                    .col(DeviceLibraryMappings::DeviceId)
                    .col(DeviceLibraryMappings::LibraryId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create indexes for device_library_mappings
        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_mappings_device_id")
                    .table(DeviceLibraryMappings::Table)
                    .col(DeviceLibraryMappings::DeviceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_mappings_library_id")
                    .table(DeviceLibraryMappings::Table)
                    .col(DeviceLibraryMappings::LibraryId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_mappings_is_active")
                    .table(DeviceLibraryMappings::Table)
                    .col(DeviceLibraryMappings::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop device_library_mappings table
        manager
            .drop_table(Table::drop().table(DeviceLibraryMappings::Table).to_owned())
            .await?;

        // Drop device_library foreign key and index
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_device_library_device_id")
                    .table(DeviceLibrary::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_device_library_device_id")
                    .table(DeviceLibrary::Table)
                    .to_owned(),
            )
            .await?;

        // Drop device_id and device_name columns from device_library
        manager
            .alter_table(
                Table::alter()
                    .table(DeviceLibrary::Table)
                    .drop_column(DeviceLibrary::DeviceId)
                    .drop_column(DeviceLibrary::DeviceName)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum DeviceLibrary {
    Table,
    Id,
    DeviceId,
    DeviceName,
}

#[derive(DeriveIden)]
enum DeviceLibraryMappings {
    Table,
    Id,
    DeviceId,
    LibraryId,
    InstalledAt,
    InstalledBy,
    InstallPath,
    Configuration,
    IsPrimary,
    IsActive,
    LastChecked,
}

#[derive(DeriveIden)]
enum Devices {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
