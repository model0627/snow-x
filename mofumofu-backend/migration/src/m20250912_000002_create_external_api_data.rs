use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExternalApiData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExternalApiData::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::ConnectionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::ExternalId)
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::DataType)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::RawData)
                            .json()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::ProcessedData)
                            .json()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::Hash)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::Status)
                            .string_len(50)
                            .default("active"),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::LastSyncAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(ExternalApiData::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await?;

        // Foreign key constraints
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_external_api_data_connection_id")
                    .from(ExternalApiData::Table, ExternalApiData::ConnectionId)
                    .to(ExternalApiConnections::Table, ExternalApiConnections::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_data_connection_external")
                    .table(ExternalApiData::Table)
                    .col(ExternalApiData::ConnectionId)
                    .col(ExternalApiData::ExternalId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_data_connection_type")
                    .table(ExternalApiData::Table)
                    .col(ExternalApiData::ConnectionId)
                    .col(ExternalApiData::DataType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_data_hash")
                    .table(ExternalApiData::Table)
                    .col(ExternalApiData::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_data_last_sync")
                    .table(ExternalApiData::Table)
                    .col(ExternalApiData::LastSyncAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExternalApiData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExternalApiData {
    Table,
    Id,
    ConnectionId,
    ExternalId,
    DataType,
    RawData,
    ProcessedData,
    Hash,
    Status,
    LastSyncAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ExternalApiConnections {
    Table,
    Id,
}