use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add source_type column to devices table
        // Values: 'manual' (default), 'api_sync'
        manager
            .alter_table(
                Table::alter()
                    .table(Devices::Table)
                    .add_column(
                        ColumnDef::new(Devices::SourceType)
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
                    .table(Devices::Table)
                    .add_column(
                        ColumnDef::new(Devices::ExternalApiConnectionId)
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
                    .table(Devices::Table)
                    .drop_column(Devices::SourceType)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Devices::Table)
                    .drop_column(Devices::ExternalApiConnectionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Devices {
    Table,
    SourceType,
    ExternalApiConnectionId,
}
