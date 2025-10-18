use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CustodianPolicies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustodianPolicies::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(CustodianPolicies::Name)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CustodianPolicies::Description).text())
                    .col(
                        ColumnDef::new(CustodianPolicies::Content)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustodianPolicies::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(CustodianPolicies::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on name for faster lookups
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_custodian_policies_name")
                    .table(CustodianPolicies::Table)
                    .col(CustodianPolicies::Name)
                    .to_owned(),
            )
            .await?;

        // Create executions table
        manager
            .create_table(
                Table::create()
                    .table(CustodianExecutions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustodianExecutions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(
                        ColumnDef::new(CustodianExecutions::PolicyId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustodianExecutions::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustodianExecutions::DryRun)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(CustodianExecutions::Output).text())
                    .col(ColumnDef::new(CustodianExecutions::Error).text())
                    .col(
                        ColumnDef::new(CustodianExecutions::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(CustodianExecutions::CompletedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_custodian_executions_policy_id")
                            .from(CustodianExecutions::Table, CustodianExecutions::PolicyId)
                            .to(CustodianPolicies::Table, CustodianPolicies::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on policy_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_custodian_executions_policy_id")
                    .table(CustodianExecutions::Table)
                    .col(CustodianExecutions::PolicyId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CustodianExecutions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CustodianPolicies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CustodianPolicies {
    Table,
    Id,
    Name,
    Description,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CustodianExecutions {
    Table,
    Id,
    PolicyId,
    Status,
    DryRun,
    Output,
    Error,
    StartedAt,
    CompletedAt,
}
