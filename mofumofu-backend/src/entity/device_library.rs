use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "device_library")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub device_type: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub manufacturer: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub model: Option<String>,
    #[sea_orm(nullable)]
    pub default_rack_size: Option<i32>,
    #[sea_orm(nullable)]
    pub default_power_consumption: Option<i32>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub default_config: Option<serde_json::Value>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub device_id: Option<Uuid>,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub device_name: Option<String>,
    #[sea_orm(column_type = "Uuid")]
    pub created_by: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CreatedBy",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::devices::Entity",
        from = "Column::DeviceId",
        to = "super::devices::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Device,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::devices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Device.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
