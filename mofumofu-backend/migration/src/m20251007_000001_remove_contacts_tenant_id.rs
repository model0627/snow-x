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
                    .name("fk_contacts_tenant_id")
                    .table(Contacts::Table)
                    .to_owned(),
            )
            .await?;

        // Drop index
        manager
            .drop_index(
                Index::drop()
                    .name("idx_contacts_tenant_id")
                    .table(Contacts::Table)
                    .to_owned(),
            )
            .await?;

        // Drop column
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Contacts::TenantId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add tenant_id column back
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(ColumnDef::new(Contacts::TenantId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        // Re-create index
        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_tenant_id")
                    .table(Contacts::Table)
                    .col(Contacts::TenantId)
                    .to_owned(),
            )
            .await?;

        // Re-create foreign key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_contacts_tenant_id")
                    .from(Contacts::Table, Contacts::TenantId)
                    .to(Tenants::Table, Tenants::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    TenantId,
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
}
