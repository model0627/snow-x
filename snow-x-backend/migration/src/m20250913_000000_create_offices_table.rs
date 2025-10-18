use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Offices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Offices::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Offices::Name).string().not_null())
                    .col(ColumnDef::new(Offices::Description).text())
                    .col(ColumnDef::new(Offices::Address).text().not_null())
                    .col(ColumnDef::new(Offices::ContactPerson).string())
                    .col(ColumnDef::new(Offices::Phone).string())
                    .col(ColumnDef::new(Offices::Email).string())
                    .col(ColumnDef::new(Offices::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Offices::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Offices::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Offices::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_offices_created_by")
                            .from(Offices::Table, Offices::CreatedBy)
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
                    .name("idx_offices_name")
                    .table(Offices::Table)
                    .col(Offices::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_offices_is_active")
                    .table(Offices::Table)
                    .col(Offices::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Offices::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Offices {
    Table,
    Id,
    Name,
    Description,
    Address,
    ContactPerson,
    Phone,
    Email,
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