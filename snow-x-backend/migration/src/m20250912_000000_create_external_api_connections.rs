use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExternalApiConnections::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExternalApiConnections::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::BaseUrl)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::Headers)
                            .json()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::AuthConfig)
                            .json()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::SyncInterval)
                            .integer()
                            .default(3600),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::IsActive)
                            .boolean()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::AutoSync)
                            .boolean()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::LastSyncAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::NextSyncAt)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::SyncCount)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::LastErrorMessage)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(ExternalApiConnections::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await?;

        // Indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_connections_name")
                    .table(ExternalApiConnections::Table)
                    .col(ExternalApiConnections::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_external_api_connections_active_sync")
                    .table(ExternalApiConnections::Table)
                    .col(ExternalApiConnections::IsActive)
                    .col(ExternalApiConnections::NextSyncAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExternalApiConnections::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExternalApiConnections {
    Table,
    Id,
    Name,
    BaseUrl,
    Description,
    Headers,
    AuthConfig,
    SyncInterval,
    IsActive,
    AutoSync,
    LastSyncAt,
    NextSyncAt,
    SyncCount,
    LastErrorMessage,
    CreatedAt,
    UpdatedAt,
}
