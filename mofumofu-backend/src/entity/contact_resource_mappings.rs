use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "resource_type")]
pub enum ResourceType {
    #[sea_orm(string_value = "office")]
    Office,
    #[sea_orm(string_value = "server_room")]
    ServerRoom,
    #[sea_orm(string_value = "rack")]
    Rack,
    #[sea_orm(string_value = "device")]
    Device,
    #[sea_orm(string_value = "ip_range")]
    IpRange,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "contact_resource_mappings")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub contact_id: Uuid,
    pub resource_type: ResourceType,
    #[sea_orm(column_type = "Uuid")]
    pub resource_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub role: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contacts::Entity",
        from = "Column::ContactId",
        to = "super::contacts::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Contact,
}

impl Related<super::contacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
