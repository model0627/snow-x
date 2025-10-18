use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tenants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tenants::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Tenants::Name).string().not_null())
                    .col(ColumnDef::new(Tenants::Description).text())
                    .col(ColumnDef::new(Tenants::Settings).json())
                    .col(ColumnDef::new(Tenants::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Tenants::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tenants::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tenants::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tenants_created_by")
                            .from(Tenants::Table, Tenants::CreatedBy)
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
                    .name("idx_tenants_name")
                    .table(Tenants::Table)
                    .col(Tenants::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tenants_is_active")
                    .table(Tenants::Table)
                    .col(Tenants::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tenants::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
    Name,
    Description,
    Settings,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}