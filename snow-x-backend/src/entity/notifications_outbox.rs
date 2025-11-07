use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "notifications_outbox")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub tenant_id: Option<Uuid>,
    #[sea_orm(column_type = "String(StringLen::None)")]
    pub channel: String,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub category: Option<String>,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub message: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub payload: Option<serde_json::Value>,
    #[sea_orm(column_type = "String(StringLen::None)")]
    pub status: String,
    pub retry_count: i32,
    pub max_retries: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub last_error: Option<String>,
    pub scheduled_at: DateTimeWithTimeZone,
    #[sea_orm(nullable)]
    pub processing_started_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(nullable)]
    pub processed_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
