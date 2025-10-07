use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "contacts")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::None)")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub title: Option<String>,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub department: Option<String>,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub phone: Option<String>,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub mobile: Option<String>,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub email: Option<String>,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub office_location: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub responsibilities: Option<String>,
    #[sea_orm(column_type = "Uuid")]
    pub created_by: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub is_active: bool,
    #[sea_orm(column_type = "String(StringLen::None)")]
    pub source_type: String,
    #[sea_orm(nullable)]
    pub external_api_connection_id: Option<i32>,
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
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
