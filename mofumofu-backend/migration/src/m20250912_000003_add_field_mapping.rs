use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ExternalApiConnections::Table)
                    .add_column(
                        ColumnDef::new(ExternalApiConnections::FieldMapping)
                            .json()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ExternalApiConnections::Table)
                    .drop_column(ExternalApiConnections::FieldMapping)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ExternalApiConnections {
    Table,
    FieldMapping,
}