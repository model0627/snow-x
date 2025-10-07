use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "device_ip_mappings")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub device_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub ip_address_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub interface_name: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTimeWithTimeZone,
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
        belongs_to = "super::ip_addresses::Entity",
        from = "Column::IpAddressId",
        to = "super::ip_addresses::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    IpAddress,
}

impl Related<super::devices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Device.def()
    }
}

impl Related<super::ip_addresses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpAddress.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
