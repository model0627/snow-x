use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add source_type column to contacts table
        // Values: 'manual' (default), 'api_sync'
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(
                        ColumnDef::new(Contacts::SourceType)
                            .string()
                            .not_null()
                            .default("manual")
                    )
                    .to_owned(),
            )
            .await?;

        // Add external_api_connection_id column to track which connection synced this data
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(
                        ColumnDef::new(Contacts::ExternalApiConnectionId)
                            .integer()
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Contacts::SourceType)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Contacts::ExternalApiConnectionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    SourceType,
    ExternalApiConnectionId,
}
