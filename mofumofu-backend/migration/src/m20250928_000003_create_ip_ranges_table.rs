use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpRanges::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpRanges::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(IpRanges::TenantId).uuid().not_null())
                    .col(ColumnDef::new(IpRanges::Name).string().not_null())
                    .col(ColumnDef::new(IpRanges::Description).text())
                    .col(ColumnDef::new(IpRanges::NetworkAddress).custom("INET").not_null())
                    .col(ColumnDef::new(IpRanges::SubnetMask).integer().not_null())
                    .col(ColumnDef::new(IpRanges::Gateway).custom("INET"))
                    .col(ColumnDef::new(IpRanges::DnsServers).text())
                    .col(ColumnDef::new(IpRanges::VlanId).integer())
                    .col(
                        ColumnDef::new(IpRanges::IpVersion)
                            .integer()
                            .not_null()
                            .default(4),
                    )
                    .col(ColumnDef::new(IpRanges::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(IpRanges::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(IpRanges::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(IpRanges::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_ranges_tenant_id")
                            .from(IpRanges::Table, IpRanges::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_ranges_created_by")
                            .from(IpRanges::Table, IpRanges::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Note: Check constraints for subnet_mask and ip_version are handled at application level

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_ip_ranges_tenant_id")
                    .table(IpRanges::Table)
                    .col(IpRanges::TenantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ip_ranges_name")
                    .table(IpRanges::Table)
                    .col(IpRanges::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ip_ranges_is_active")
                    .table(IpRanges::Table)
                    .col(IpRanges::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpRanges::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpRanges {
    Table,
    Id,
    TenantId,
    Name,
    Description,
    NetworkAddress,
    SubnetMask,
    Gateway,
    DnsServers,
    VlanId,
    IpVersion,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}