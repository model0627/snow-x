use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

/// 알림 생성 요청
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateNotificationRequest {
    /// 어떤 채널로 전송할지 (예: web, slack_webhook 등)
    pub channel: String,
    /// 알림 분류(선택)
    pub category: Option<String>,
    /// 사용자에게 보여줄 제목(선택)
    pub title: Option<String>,
    /// 간단한 메시지(선택)
    pub message: Option<String>,
    /// 추가 메타데이터(JSON)
    pub payload: Option<Value>,
    /// 멀티 테넌시 사용 시 대상 테넌트
    pub tenant_id: Option<Uuid>,
    /// 예약 발송 시각(기본값: 지금)
    pub scheduled_at: Option<DateTime<Utc>>,
    /// 최대 재시도 횟수(기본값: 3)
    pub max_retries: Option<i32>,
}

/// 알림 상태 업데이트 요청 (예: 읽음 처리, 강제 실패 처리 등)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateNotificationStatusRequest {
    /// 변경할 상태 (pending, processing, done, failed)
    pub status: String,
    /// 실패 사유 등 추가 정보(선택)
    pub last_error: Option<String>,
}
