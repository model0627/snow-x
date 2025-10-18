use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contacts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contacts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Contacts::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Contacts::Name).string().not_null())
                    .col(ColumnDef::new(Contacts::Title).string())
                    .col(ColumnDef::new(Contacts::Department).string())
                    .col(ColumnDef::new(Contacts::Phone).string())
                    .col(ColumnDef::new(Contacts::Mobile).string())
                    .col(ColumnDef::new(Contacts::Email).string())
                    .col(ColumnDef::new(Contacts::OfficeLocation).string())
                    .col(ColumnDef::new(Contacts::Responsibilities).text())
                    .col(ColumnDef::new(Contacts::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Contacts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Contacts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Contacts::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contacts_tenant_id")
                            .from(Contacts::Table, Contacts::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contacts_created_by")
                            .from(Contacts::Table, Contacts::CreatedBy)
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
                    .name("idx_contacts_tenant_id")
                    .table(Contacts::Table)
                    .col(Contacts::TenantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_name")
                    .table(Contacts::Table)
                    .col(Contacts::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_email")
                    .table(Contacts::Table)
                    .col(Contacts::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_department")
                    .table(Contacts::Table)
                    .col(Contacts::Department)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_contacts_is_active")
                    .table(Contacts::Table)
                    .col(Contacts::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contacts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    Id,
    TenantId,
    Name,
    Title,
    Department,
    Phone,
    Mobile,
    Email,
    OfficeLocation,
    Responsibilities,
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