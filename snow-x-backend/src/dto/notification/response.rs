use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

/// 단일 알림 응답
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub channel: String,
    pub category: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub payload: Option<Value>,
    pub status: String,
    pub retry_count: i32,
    pub max_retries: i32,
    pub last_error: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub processing_started_at: Option<DateTime<Utc>>,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 알림 목록 응답
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}
