use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpAddresses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpAddresses::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(IpAddresses::IpRangeId).uuid().not_null())
                    .col(ColumnDef::new(IpAddresses::IpAddress).custom("INET").not_null())
                    .col(
                        ColumnDef::new(IpAddresses::Status)
                            .string()
                            .not_null()
                            .default("available"),
                    )
                    .col(ColumnDef::new(IpAddresses::Hostname).string())
                    .col(ColumnDef::new(IpAddresses::Description).text())
                    .col(ColumnDef::new(IpAddresses::MacAddress).custom("MACADDR"))
                    .col(ColumnDef::new(IpAddresses::LeaseStart).timestamp_with_time_zone())
                    .col(ColumnDef::new(IpAddresses::LeaseEnd).timestamp_with_time_zone())
                    .col(ColumnDef::new(IpAddresses::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(IpAddresses::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(IpAddresses::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(IpAddresses::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_addresses_ip_range_id")
                            .from(IpAddresses::Table, IpAddresses::IpRangeId)
                            .to(IpRanges::Table, IpRanges::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_addresses_created_by")
                            .from(IpAddresses::Table, IpAddresses::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Note: Check constraint for status is handled at application level

        // Create unique constraint for ip_address and ip_range_id combination
        manager
            .create_index(
                Index::create()
                    .name("uniq_ip_addresses_ip_address_range")
                    .table(IpAddresses::Table)
                    .col(IpAddresses::IpAddress)
                    .col(IpAddresses::IpRangeId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_ip_addresses_ip_range_id")
                    .table(IpAddresses::Table)
                    .col(IpAddresses::IpRangeId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ip_addresses_status")
                    .table(IpAddresses::Table)
                    .col(IpAddresses::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ip_addresses_hostname")
                    .table(IpAddresses::Table)
                    .col(IpAddresses::Hostname)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ip_addresses_is_active")
                    .table(IpAddresses::Table)
                    .col(IpAddresses::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpAddresses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpAddresses {
    Table,
    Id,
    IpRangeId,
    IpAddress,
    Status,
    Hostname,
    Description,
    MacAddress,
    LeaseStart,
    LeaseEnd,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum IpRanges {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}