use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Racks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Racks::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Racks::ServerRoomId).uuid().not_null())
                    .col(ColumnDef::new(Racks::Name).string().not_null())
                    .col(ColumnDef::new(Racks::Description).text())
                    .col(
                        ColumnDef::new(Racks::RackHeight)
                            .integer()
                            .not_null()
                            .default(42),
                    )
                    .col(ColumnDef::new(Racks::PowerCapacity).integer())
                    .col(ColumnDef::new(Racks::CoolingType).string())
                    .col(ColumnDef::new(Racks::LocationX).decimal_len(10, 2))
                    .col(ColumnDef::new(Racks::LocationY).decimal_len(10, 2))
                    .col(ColumnDef::new(Racks::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Racks::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Racks::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Racks::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_racks_server_room_id")
                            .from(Racks::Table, Racks::ServerRoomId)
                            .to(ServerRooms::Table, ServerRooms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_racks_created_by")
                            .from(Racks::Table, Racks::CreatedBy)
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
                    .name("idx_racks_server_room_id")
                    .table(Racks::Table)
                    .col(Racks::ServerRoomId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_racks_name")
                    .table(Racks::Table)
                    .col(Racks::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_racks_is_active")
                    .table(Racks::Table)
                    .col(Racks::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Racks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Racks {
    Table,
    Id,
    ServerRoomId,
    Name,
    Description,
    RackHeight,
    PowerCapacity,
    CoolingType,
    LocationX,
    LocationY,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum ServerRooms {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}