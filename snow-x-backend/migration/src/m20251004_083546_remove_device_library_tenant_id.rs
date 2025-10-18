use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop foreign key constraint first
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_device_library_tenant_id")
                    .table(DeviceLibrary::Table)
                    .to_owned(),
            )
            .await?;

        // Drop index
        manager
            .drop_index(
                Index::drop()
                    .name("idx_device_library_tenant_id")
                    .table(DeviceLibrary::Table)
                    .to_owned(),
            )
            .await?;

        // Drop column
        manager
            .alter_table(
                Table::alter()
                    .table(DeviceLibrary::Table)
                    .drop_column(DeviceLibrary::TenantId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add column back
        manager
            .alter_table(
                Table::alter()
                    .table(DeviceLibrary::Table)
                    .add_column(ColumnDef::new(DeviceLibrary::TenantId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        // Recreate index
        manager
            .create_index(
                Index::create()
                    .name("idx_device_library_tenant_id")
                    .table(DeviceLibrary::Table)
                    .col(DeviceLibrary::TenantId)
                    .to_owned(),
            )
            .await?;

        // Recreate foreign key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_device_library_tenant_id")
                    .from(DeviceLibrary::Table, DeviceLibrary::TenantId)
                    .to(Tenants::Table, Tenants::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum DeviceLibrary {
    Table,
    TenantId,
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
}
