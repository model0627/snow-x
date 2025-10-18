use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "custodian_executions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub policy_id: Uuid,
    pub status: String,
    pub dry_run: bool,
    pub task_id: Option<String>,
    pub output: Option<String>,
    pub error: Option<String>,
    #[schema(value_type = String)]
    pub started_at: DateTimeWithTimeZone,
    #[schema(value_type = Option<String>)]
    pub completed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::custodian_policies::Entity",
        from = "Column::PolicyId",
        to = "super::custodian_policies::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    CustodianPolicies,
}

impl Related<super::custodian_policies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustodianPolicies.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
