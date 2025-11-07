use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    dto::auth::internal::access_token::AccessTokenClaims,
    dto::notification::{
        request::{CreateNotificationRequest, UpdateNotificationStatusRequest},
        response::{NotificationListResponse, NotificationResponse},
    },
    entity::notifications_outbox,
    service::notification::{
        self, CreateNotificationParams, NotificationListResult, STATUS_DONE, STATUS_FAILED,
        STATUS_PENDING, STATUS_PROCESSING,
    },
    state::AppState,
};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListNotificationsQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub status: Option<String>,
    pub channel: Option<String>,
}

fn map_model(model: notifications_outbox::Model) -> NotificationResponse {
    NotificationResponse {
        id: model.id,
        tenant_id: model.tenant_id,
        channel: model.channel,
        category: model.category,
        title: model.title,
        message: model.message,
        payload: model.payload,
        status: model.status,
        retry_count: model.retry_count,
        max_retries: model.max_retries,
        last_error: model.last_error,
        scheduled_at: model.scheduled_at.with_timezone(&Utc),
        processing_started_at: model.processing_started_at.map(|dt| dt.with_timezone(&Utc)),
        processed_at: model.processed_at.map(|dt| dt.with_timezone(&Utc)),
        created_at: model.created_at.with_timezone(&Utc),
        updated_at: model.updated_at.with_timezone(&Utc),
    }
}

fn map_list_result(result: NotificationListResult) -> NotificationListResponse {
    NotificationListResponse {
        notifications: result.notifications.into_iter().map(map_model).collect(),
        total: result.total,
        page: result.page,
        limit: result.limit,
    }
}

/// 새 알림을 생성하여 알림 큐에 적재합니다.
#[utoipa::path(
    post,
    path = "/v0/notifications",
    tag = "Notifications",
    request_body = CreateNotificationRequest,
    responses(
        (status = 201, description = "알림 생성 성공", body = NotificationResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요")
    ),
    security(("bearer" = []))
)]
pub async fn create_notification(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateNotificationRequest>,
) -> impl IntoResponse {
    let params = CreateNotificationParams {
        tenant_id: request.tenant_id,
        channel: request.channel,
        category: request.category,
        title: request.title,
        message: request.message,
        payload: request.payload,
        scheduled_at: request.scheduled_at,
        max_retries: request.max_retries,
    };

    match notification::service_create_notification(&state.conn, params).await {
        Ok(model) => (StatusCode::CREATED, Json(map_model(model))).into_response(),
        Err(err) => err.into_response(),
    }
}

/// 알림 목록을 조회합니다.
#[utoipa::path(
    get,
    path = "/v0/notifications",
    tag = "Notifications",
    params(
        ListNotificationsQuery,
    ),
    responses(
        (status = 200, description = "알림 목록 조회 성공", body = NotificationListResponse),
        (status = 401, description = "인증 필요")
    ),
    security(("bearer" = []))
)]
pub async fn get_notifications(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Query(query): Query<ListNotificationsQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match notification::service_get_notifications(
        &state.conn,
        page,
        limit,
        query.status,
        query.channel,
    )
    .await
    {
        Ok(result) => Json(map_list_result(result)).into_response(),
        Err(err) => err.into_response(),
    }
}

/// 알림 상태를 업데이트합니다. (예: 읽음 처리, 강제 실패 표시 등)
#[utoipa::path(
    patch,
    path = "/v0/notifications/{id}",
    tag = "Notifications",
    params(
        ("id" = Uuid, Path, description = "알림 ID")
    ),
    request_body = UpdateNotificationStatusRequest,
    responses(
        (status = 200, description = "알림 상태 업데이트 성공", body = NotificationResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요"),
        (status = 404, description = "알림 없음")
    ),
    security(("bearer" = []))
)]
pub async fn update_notification_status(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateNotificationStatusRequest>,
) -> impl IntoResponse {
    match notification::service_update_notification_status(
        &state.conn,
        &id,
        request.status.as_str(),
        request.last_error,
    )
    .await
    {
        Ok(model) => Json(map_model(model)).into_response(),
        Err(err) => err.into_response(),
    }
}

/// 노출 가능한 상태 값 안내용 상수 목록
#[allow(dead_code)]
pub const ALLOWED_NOTIFICATION_STATUSES: [&str; 4] = [
    STATUS_PENDING,
    STATUS_PROCESSING,
    STATUS_DONE,
    STATUS_FAILED,
];
