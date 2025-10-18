use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "offices")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub address: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub contact_person: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(50))", nullable)]
    pub phone: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub email: Option<String>,
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
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}