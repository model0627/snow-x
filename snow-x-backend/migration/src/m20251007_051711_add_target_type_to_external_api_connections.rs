use sea_orm_migration::prelude::*;

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
                        ColumnDef::new(ExternalApiConnections::TargetType)
                            .string_len(50)
                            .not_null()
                            .default("device")
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
                    .drop_column(ExternalApiConnections::TargetType)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ExternalApiConnections {
    Table,
    TargetType,
}
