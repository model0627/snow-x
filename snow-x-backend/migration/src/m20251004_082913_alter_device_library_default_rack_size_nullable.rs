use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Make default_rack_size nullable and remove default value
        manager
            .alter_table(
                Table::alter()
                    .table(DeviceLibrary::Table)
                    .modify_column(
                        ColumnDef::new(DeviceLibrary::DefaultRackSize)
                            .integer()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert: make default_rack_size not null with default value 1
        manager
            .alter_table(
                Table::alter()
                    .table(DeviceLibrary::Table)
                    .modify_column(
                        ColumnDef::new(DeviceLibrary::DefaultRackSize)
                            .integer()
                            .not_null()
                            .default(1)
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum DeviceLibrary {
    Table,
    DefaultRackSize,
}
