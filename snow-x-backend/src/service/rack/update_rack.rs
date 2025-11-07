use super::mapper::build_rack_response;
use crate::dto::rack::request::update_rack::UpdateRackRequest;
use crate::dto::rack::response::rack_info::RackInfoResponse;
use crate::entity::{racks, users};
use crate::service::error::errors::Errors;
use crate::service::notification::{self, CreateNotificationParams};
use chrono::Utc;
use sea_orm::prelude::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use tracing::warn;
use uuid::Uuid;

pub async fn service_update_rack(
    conn: &DatabaseConnection,
    rack_id: Uuid,
    request: UpdateRackRequest,
    updated_by: Uuid,
) -> Result<RackInfoResponse, Errors> {
    let existing = racks::Entity::find_by_id(rack_id)
        .filter(racks::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?
        .ok_or_else(|| Errors::NotFound("Rack not found".to_string()))?;

    let mut model: racks::ActiveModel = existing.clone().into();
    let mut has_change = false;

    if let Some(name) = request.name {
        model.name = Set(name);
        has_change = true;
    }
    if let Some(description) = request.description {
        model.description = Set(Some(description));
        has_change = true;
    }
    if let Some(height) = request.rack_height {
        model.rack_height = Set(height);
        has_change = true;
    }
    if let Some(power) = request.power_capacity {
        model.power_capacity = Set(Some(power));
        has_change = true;
    }
    if let Some(cooling) = request.cooling_type {
        model.cooling_type = Set(Some(cooling));
        has_change = true;
    }
    if let Some(x) = request.location_x {
        let decimal = Decimal::try_from(x).map_err(|_| {
            Errors::BadRequestError("location_x must be a finite number".to_string())
        })?;
        model.location_x = Set(Some(decimal));
        has_change = true;
    }
    if let Some(y) = request.location_y {
        let decimal = Decimal::try_from(y).map_err(|_| {
            Errors::BadRequestError("location_y must be a finite number".to_string())
        })?;
        model.location_y = Set(Some(decimal));
        has_change = true;
    }

    if !has_change {
        return Err(Errors::BadRequestError(
            "No fields were provided to update".to_string(),
        ));
    }

    model.updated_at = Set(Utc::now().into());

    let updated = model
        .update(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    if let Err(err) = enqueue_rack_update_notification(conn, &existing, &updated, updated_by).await
    {
        warn!(
            rack_id = %updated.id,
            "failed to enqueue rack-update notification: {err:?}"
        );
    }

    build_rack_response(conn, updated).await
}

async fn enqueue_rack_update_notification(
    conn: &DatabaseConnection,
    before: &racks::Model,
    after: &racks::Model,
    updated_by: Uuid,
) -> Result<(), Errors> {
    let mut diff = Vec::new();

    push_diff(
        &mut diff,
        "name",
        Some(before.name.clone()),
        Some(after.name.clone()),
    );
    push_diff(
        &mut diff,
        "description",
        before.description.clone(),
        after.description.clone(),
    );
    push_diff(
        &mut diff,
        "rack_height",
        Some(before.rack_height.to_string()),
        Some(after.rack_height.to_string()),
    );
    push_diff(
        &mut diff,
        "power_capacity",
        to_string_opt(before.power_capacity),
        to_string_opt(after.power_capacity),
    );
    push_diff(
        &mut diff,
        "cooling_type",
        before.cooling_type.clone(),
        after.cooling_type.clone(),
    );
    push_diff(
        &mut diff,
        "location_x",
        decimal_to_string(before.location_x),
        decimal_to_string(after.location_x),
    );
    push_diff(
        &mut diff,
        "location_y",
        decimal_to_string(before.location_y),
        decimal_to_string(after.location_y),
    );

    if diff.is_empty() {
        return Ok(());
    }

    let actor = users::Entity::find_by_id(updated_by)
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    let actor_name = actor
        .as_ref()
        .map(|u| u.name.clone())
        .unwrap_or_else(|| "시스템".to_string());
    let actor_handle = actor.as_ref().map(|u| u.handle.clone());
    let actor_email = actor.as_ref().map(|u| u.email.clone());

    let payload = json!({
        "rack_id": after.id,
        "server_room_id": after.server_room_id,
        "diff": diff,
        "actor_id": updated_by,
        "actor_name": actor_name,
        "actor_handle": actor_handle,
        "actor_email": actor_email,
        "link": format!("/ipam/racks/{}", after.id),
        "resource_name": after.name
    });

    notification::service_create_notification(
        conn,
        CreateNotificationParams {
            tenant_id: None,
            channel: "web".to_string(),
            category: Some("rack_updated".to_string()),
            title: Some(format!("랙 수정: {}", after.name)),
            message: Some("랙 정보가 업데이트되었습니다.".to_string()),
            payload: Some(payload),
            scheduled_at: None,
            max_retries: Some(0),
        },
    )
    .await
    .map(|_| ())
}

fn push_diff(
    acc: &mut Vec<serde_json::Value>,
    field: &str,
    before: Option<String>,
    after: Option<String>,
) {
    if before == after {
        return;
    }

    acc.push(json!({
        "field": field,
        "before": before,
        "after": after
    }));
}

fn to_string_opt(value: Option<i32>) -> Option<String> {
    value.map(|v| v.to_string())
}

fn decimal_to_string(value: Option<Decimal>) -> Option<String> {
    value.map(|d| d.to_string())
}
