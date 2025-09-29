use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServerRooms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServerRooms::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(ServerRooms::OfficeId).uuid().not_null())
                    .col(ColumnDef::new(ServerRooms::Name).string().not_null())
                    .col(ColumnDef::new(ServerRooms::Description).text())
                    .col(ColumnDef::new(ServerRooms::FloorLevel).string())
                    .col(ColumnDef::new(ServerRooms::RoomNumber).string())
                    .col(
                        ColumnDef::new(ServerRooms::TemperatureMonitoring)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ServerRooms::HumidityMonitoring)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ServerRooms::AccessControl)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(ServerRooms::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(ServerRooms::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ServerRooms::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ServerRooms::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_server_rooms_office_id")
                            .from(ServerRooms::Table, ServerRooms::OfficeId)
                            .to(Offices::Table, Offices::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_server_rooms_created_by")
                            .from(ServerRooms::Table, ServerRooms::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_server_rooms_office_id")
                    .table(ServerRooms::Table)
                    .col(ServerRooms::OfficeId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_server_rooms_name")
                    .table(ServerRooms::Table)
                    .col(ServerRooms::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_server_rooms_is_active")
                    .table(ServerRooms::Table)
                    .col(ServerRooms::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServerRooms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ServerRooms {
    Table,
    Id,
    OfficeId,
    Name,
    Description,
    FloorLevel,
    RoomNumber,
    TemperatureMonitoring,
    HumidityMonitoring,
    AccessControl,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum Offices {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}