use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExternalApiSyncLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::ConnectionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::Status)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::RequestUrl)
                            .string_len(1000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::RequestMethod)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::RequestHeaders)
                            .json()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::RequestBody)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::ResponseStatus)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::ResponseHeaders)
                            .json()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::ResponseBody)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::RecordsProcessed)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::ErrorMessage)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::Duration)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::CompletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(ExternalApiSyncLogs::UpdatedAt)
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
                    .name("fk_external_api_sync_logs_connection_id")
                    .from(ExternalApiSyncLogs::Table, ExternalApiSyncLogs::ConnectionId)
                    .to(ExternalApiConnections::Table, ExternalApiConnections::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_sync_logs_connection_started")
                    .table(ExternalApiSyncLogs::Table)
                    .col(ExternalApiSyncLogs::ConnectionId)
                    .col(ExternalApiSyncLogs::StartedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_sync_logs_status")
                    .table(ExternalApiSyncLogs::Table)
                    .col(ExternalApiSyncLogs::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExternalApiSyncLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExternalApiSyncLogs {
    Table,
    Id,
    ConnectionId,
    Status,
    RequestUrl,
    RequestMethod,
    RequestHeaders,
    RequestBody,
    ResponseStatus,
    ResponseHeaders,
    ResponseBody,
    RecordsProcessed,
    ErrorMessage,
    Duration,
    StartedAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ExternalApiConnections {
    Table,
    Id,
}