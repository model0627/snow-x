use crate::entity::{ip_ranges, users};
use crate::service::error::errors::{Errors, ServiceResult};
use crate::service::notification::{self, CreateNotificationParams};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use tracing::warn;
use uuid::Uuid;

pub async fn service_update_ip_range(
    conn: &DatabaseConnection,
    id: &Uuid,
    updated_by: Uuid,
    name: Option<String>,
    description: Option<String>,
    network_address: Option<String>,
    subnet_mask: Option<i32>,
    gateway: Option<String>,
    dns_servers: Option<Vec<String>>,
    vlan_id: Option<i32>,
    ip_version: Option<i32>,
) -> ServiceResult<ip_ranges::Model> {
    // Validate subnet mask if provided
    if let Some(mask) = subnet_mask {
        if mask < 8 || mask > 32 {
            return Err(Errors::BadRequestError(
                "Subnet mask must be between 8 and 32".to_string(),
            ));
        }
    }

    // Validate IP version if provided
    if let Some(version) = ip_version {
        if version != 4 && version != 6 {
            return Err(Errors::BadRequestError(
                "IP version must be 4 or 6".to_string(),
            ));
        }
    }

    let existing = ip_ranges::Entity::find_by_id(*id)
        .filter(ip_ranges::Column::IsActive.eq(true))
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?
        .ok_or_else(|| Errors::NotFound("IP range not found".to_string()))?;

    let mut model: ip_ranges::ActiveModel = existing.clone().into();
    let mut has_change = false;

    if let Some(n) = name {
        model.name = Set(n);
        has_change = true;
    }
    if let Some(d) = description {
        model.description = Set(Some(d));
        has_change = true;
    }
    if let Some(na) = network_address {
        model.network_address = Set(na);
        has_change = true;
    }
    if let Some(sm) = subnet_mask {
        model.subnet_mask = Set(sm);
        has_change = true;
    }
    if let Some(g) = gateway {
        model.gateway = Set(Some(g));
        has_change = true;
    }
    if let Some(dns) = dns_servers {
        model.dns_servers = Set(Some(dns));
        has_change = true;
    }
    if let Some(vlan) = vlan_id {
        model.vlan_id = Set(Some(vlan));
        has_change = true;
    }
    if let Some(version) = ip_version {
        model.ip_version = Set(version);
        has_change = true;
    }

    if !has_change {
        return Err(Errors::BadRequestError("No fields to update".to_string()));
    }

    model.updated_at = Set(Utc::now().into());

    let updated = model
        .update(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    enqueue_ip_range_diff_notification(conn, &existing, &updated, updated_by)
        .await
        .unwrap_or_else(|err| {
            warn!(
                ip_range_id = %updated.id,
                "failed to enqueue ip range update notification: {err:?}"
            );
        });

    Ok(updated)
}

async fn enqueue_ip_range_diff_notification(
    conn: &DatabaseConnection,
    before: &ip_ranges::Model,
    after: &ip_ranges::Model,
    updated_by: Uuid,
) -> ServiceResult<()> {
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
        "network_address",
        Some(before.network_address.clone()),
        Some(after.network_address.clone()),
    );
    push_diff(
        &mut diff,
        "subnet_mask",
        Some(before.subnet_mask.to_string()),
        Some(after.subnet_mask.to_string()),
    );
    push_diff(
        &mut diff,
        "gateway",
        before.gateway.clone(),
        after.gateway.clone(),
    );
    push_diff(
        &mut diff,
        "dns_servers",
        option_vec_to_string(&before.dns_servers),
        option_vec_to_string(&after.dns_servers),
    );
    push_diff(
        &mut diff,
        "vlan_id",
        before.vlan_id.map(|v| v.to_string()),
        after.vlan_id.map(|v| v.to_string()),
    );
    push_diff(
        &mut diff,
        "ip_version",
        Some(before.ip_version.to_string()),
        Some(after.ip_version.to_string()),
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
        "ip_range_id": after.id,
        "diff": diff,
        "actor_id": updated_by,
        "actor_name": actor_name,
        "actor_handle": actor_handle,
        "actor_email": actor_email,
        "link": format!("/ipam/ip-range/{}", after.id),
        "resource_name": after.name,
    });

    notification::service_create_notification(
        conn,
        CreateNotificationParams {
            tenant_id: Some(after.tenant_id),
            channel: "web".to_string(),
            category: Some("ip_range_updated".to_string()),
            title: Some(format!("IP 대역 수정: {}", after.name)),
            message: Some("IP 대역 정보가 수정되었습니다.".to_string()),
            payload: Some(payload),
            scheduled_at: None,
            max_retries: Some(0),
        },
    )
    .await?;

    Ok(())
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

fn option_vec_to_string(values: &Option<Vec<String>>) -> Option<String> {
    values.as_ref().map(|items| items.join(", "))
}
