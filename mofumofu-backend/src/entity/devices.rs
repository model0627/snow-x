use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub rack_id: Option<Uuid>,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub device_type: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub manufacturer: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub model: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub serial_number: Option<String>,
    pub rack_position: Option<i32>,
    pub rack_size: i32,
    pub power_consumption: Option<i32>,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub status: String,
    pub purchase_date: Option<Date>,
    pub warranty_end: Option<Date>,
    #[sea_orm(column_type = "Uuid")]
    pub created_by: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::racks::Entity",
        from = "Column::RackId",
        to = "super::racks::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Rack,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CreatedBy",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::racks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rack.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
