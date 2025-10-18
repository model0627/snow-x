use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;
use sea_orm_migration::sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create resource_type enum
        manager
            .create_type(
                Type::create()
                    .as_enum(ResourceType::Enum)
                    .values([
                        ResourceType::Office,
                        ResourceType::ServerRoom,
                        ResourceType::Rack,
                        ResourceType::Device,
                        ResourceType::IpRange,
                    ])
                    .to_owned(),
            )
            .await?;

        // Create contact_resource_mappings table
        manager
            .create_table(
                Table::create()
                    .table(ContactResourceMappings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactResourceMappings::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(ContactResourceMappings::ContactId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactResourceMappings::ResourceType)
                            .custom(ResourceType::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactResourceMappings::ResourceId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactResourceMappings::Role)
                            .string()
                            .default("manager"),
                    )
                    .col(
                        ColumnDef::new(ContactResourceMappings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contact_resource_mappings_contact_id")
                            .from(
                                ContactResourceMappings::Table,
                                ContactResourceMappings::ContactId,
                            )
                            .to(Contacts::Table, Contacts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_resource_unique")
                    .table(ContactResourceMappings::Table)
                    .col(ContactResourceMappings::ContactId)
                    .col(ContactResourceMappings::ResourceType)
                    .col(ContactResourceMappings::ResourceId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create index on contact_id
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_resource_contact_id")
                    .table(ContactResourceMappings::Table)
                    .col(ContactResourceMappings::ContactId)
                    .to_owned(),
            )
            .await?;

        // Create index on resource
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_resource_resource")
                    .table(ContactResourceMappings::Table)
                    .col(ContactResourceMappings::ResourceType)
                    .col(ContactResourceMappings::ResourceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ContactResourceMappings::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(ResourceType::Enum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ContactResourceMappings {
    Table,
    Id,
    ContactId,
    ResourceType,
    ResourceId,
    Role,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ResourceType {
    #[sea_orm(iden = "resource_type")]
    Enum,
    #[sea_orm(iden = "office")]
    Office,
    #[sea_orm(iden = "server_room")]
    ServerRoom,
    #[sea_orm(iden = "rack")]
    Rack,
    #[sea_orm(iden = "device")]
    Device,
    #[sea_orm(iden = "ip_range")]
    IpRange,
}
