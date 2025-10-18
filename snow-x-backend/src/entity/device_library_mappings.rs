use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "device_library_mappings")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub device_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub library_id: Uuid,
    pub installed_at: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub installed_by: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub install_path: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub configuration: Option<serde_json::Value>,
    pub is_primary: bool,
    pub is_active: bool,
    #[sea_orm(nullable)]
    pub last_checked: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::devices::Entity",
        from = "Column::DeviceId",
        to = "super::devices::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Device,
    #[sea_orm(
        belongs_to = "super::device_library::Entity",
        from = "Column::LibraryId",
        to = "super::device_library::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Library,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::InstalledBy",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    User,
}

impl Related<super::devices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Device.def()
    }
}

impl Related<super::device_library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Library.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
