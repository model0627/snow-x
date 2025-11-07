use crate::{
    entity::notifications_outbox,
    service::error::errors::{Errors, ServiceResult},
};
use chrono::{DateTime, Utc};
use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder,
};
use uuid::Uuid;

/// 지원하는 알림 상태 상수
pub const STATUS_PENDING: &str = "pending";
pub const STATUS_PROCESSING: &str = "processing";
pub const STATUS_DONE: &str = "done";
pub const STATUS_FAILED: &str = "failed";

/// 알림 생성시 필요한 데이터
pub struct CreateNotificationParams {
    pub tenant_id: Option<Uuid>,
    pub channel: String,
    pub category: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub max_retries: Option<i32>,
}

/// 알림 목록 조회 결과
pub struct NotificationListResult {
    pub notifications: Vec<notifications_outbox::Model>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

/// 알림을 생성하고 큐에 적재
pub async fn service_create_notification(
    conn: &DatabaseConnection,
    params: CreateNotificationParams,
) -> ServiceResult<notifications_outbox::Model> {
    if params.channel.trim().is_empty() {
        return Err(Errors::BadRequestError(
            "channel must not be empty".to_string(),
        ));
    }

    if let Some(max) = params.max_retries {
        if max < 0 {
            return Err(Errors::BadRequestError(
                "max_retries must be 0 or greater".to_string(),
            ));
        }
    }

    let now = Utc::now();
    let scheduled_at = params.scheduled_at.unwrap_or(now);
    let max_retries = params.max_retries.unwrap_or(3).max(0);

    let active = notifications_outbox::ActiveModel {
        id: Set(Uuid::new_v4()),
        tenant_id: Set(params.tenant_id),
        channel: Set(params.channel),
        category: Set(params.category),
        title: Set(params.title),
        message: Set(params.message),
        payload: Set(params.payload),
        status: Set(STATUS_PENDING.to_string()),
        retry_count: Set(0),
        max_retries: Set(max_retries),
        last_error: Set(None),
        scheduled_at: Set(scheduled_at.into()),
        processing_started_at: Set(None),
        processed_at: Set(None),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };

    active
        .insert(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
}

/// 알림 목록 조회
pub async fn service_get_notifications(
    conn: &DatabaseConnection,
    page: u64,
    limit: u64,
    status: Option<String>,
    channel: Option<String>,
) -> ServiceResult<NotificationListResult> {
    let page = page.max(1);
    let limit = limit.clamp(1, 200);

    let mut query = notifications_outbox::Entity::find();

    if let Some(status) = status {
        query = query.filter(notifications_outbox::Column::Status.eq(status));
    }

    if let Some(channel) = channel {
        query = query.filter(notifications_outbox::Column::Channel.eq(channel));
    }

    let paginator = query
        .order_by_desc(notifications_outbox::Column::CreatedAt)
        .paginate(conn, limit);

    let total = paginator
        .num_items()
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let items = paginator
        .fetch_page(page - 1)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(NotificationListResult {
        notifications: items,
        total,
        page,
        limit,
    })
}

/// 알림 상태 변경 (읽음 처리, 재시도 등)
pub async fn service_update_notification_status(
    conn: &DatabaseConnection,
    id: &Uuid,
    new_status: &str,
    last_error: Option<String>,
) -> ServiceResult<notifications_outbox::Model> {
    validate_status(new_status)?;

    let model = notifications_outbox::Entity::find_by_id(*id)
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?
        .ok_or_else(|| Errors::NotFound(format!("Notification with id {} not found", id)))?;

    let mut active: notifications_outbox::ActiveModel = model.into();
    let now = Utc::now();

    active.status = Set(new_status.to_string());
    active.updated_at = Set(now.into());

    if let Some(err) = last_error {
        active.last_error = Set(Some(err));
    }

    match new_status {
        STATUS_PENDING => {
            active.processing_started_at = Set(None);
            active.processed_at = Set(None);
        }
        STATUS_PROCESSING => {
            active.processing_started_at = Set(Some(now.into()));
        }
        STATUS_DONE | STATUS_FAILED => {
            active.processed_at = Set(Some(now.into()));
        }
        _ => {}
    }

    active
        .update(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
}

fn validate_status(status: &str) -> ServiceResult<()> {
    match status {
        STATUS_PENDING | STATUS_PROCESSING | STATUS_DONE | STATUS_FAILED => Ok(()),
        other => Err(Errors::BadRequestError(format!(
            "Unsupported notification status '{}'",
            other
        ))),
    }
}
