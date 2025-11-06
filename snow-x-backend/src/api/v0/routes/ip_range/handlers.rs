use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    dto::auth::internal::access_token::AccessTokenClaims,
    entity::ip_ranges,
    service::ip_range::{
        RangeUsageStats, create_ip_range::service_create_ip_range,
        delete_ip_range::service_delete_ip_range, fetch_ip_range_usage,
        get_ip_range_by_id::service_get_ip_range_by_id, get_ip_ranges::service_get_ip_ranges,
        update_ip_range::service_update_ip_range,
    },
    state::AppState,
};

#[derive(Deserialize, ToSchema)]
pub struct CreateIpRangeRequest {
    pub name: String,
    pub description: Option<String>,
    pub network_address: String,
    pub subnet_mask: i32,
    pub gateway: Option<String>,
    pub dns_servers: Option<Vec<String>>,
    pub vlan_id: Option<i32>,
    #[serde(default = "default_ip_version")]
    pub ip_version: i32,
    pub tenant_id: Uuid,
}

fn default_ip_version() -> i32 {
    4
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateIpRangeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub network_address: Option<String>,
    pub subnet_mask: Option<i32>,
    pub gateway: Option<String>,
    pub dns_servers: Option<Vec<String>>,
    pub vlan_id: Option<i32>,
    pub ip_version: Option<i32>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct ListIpRangesQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct IpRangeResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub network_address: String,
    pub subnet_mask: i32,
    pub gateway: Option<String>,
    pub dns_servers: Option<Vec<String>>,
    pub vlan_id: Option<i32>,
    pub ip_version: i32,
    pub total_ips: i64,
    pub used_ips: i64,
    pub available_ips: i64,
    pub usage_percentage: f64,
    pub allocated_ips: i64,
    pub reserved_ips: i64,
    pub unavailable_ips: i64,
    pub expired_ips: i64,
    pub other_ips: i64,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub is_active: bool,
}

#[derive(Serialize, ToSchema)]
pub struct IpRangeListResponse {
    pub ip_ranges: Vec<IpRangeResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

fn calculate_capacity(ip_version: i32, subnet_mask: i32) -> i64 {
    if ip_version != 4 {
        return 0;
    }

    match subnet_mask {
        0..=30 => {
            let host_bits = 32 - subnet_mask;
            let total = 1_i64 << host_bits;
            (total - 2).max(0)
        }
        31 => 2,
        32 => 1,
        _ => 0,
    }
}

fn build_ip_range_response(range: &ip_ranges::Model, usage: RangeUsageStats) -> IpRangeResponse {
    let capacity = calculate_capacity(range.ip_version, range.subnet_mask);
    let recorded_total = usage.total();
    let total_ips = if capacity > 0 {
        capacity
    } else {
        recorded_total
    };

    let mut used_ips = usage.used();
    if total_ips > 0 && used_ips > total_ips {
        used_ips = total_ips;
    }

    let mut available_ips = if recorded_total > 0 {
        usage.available
    } else if total_ips > 0 {
        total_ips - used_ips
    } else {
        0
    };

    if total_ips > 0 {
        let max_available = total_ips - used_ips;
        if available_ips > max_available {
            available_ips = max_available;
        }
        if available_ips < 0 {
            available_ips = 0;
        }
    }

    let usage_percentage = if total_ips > 0 {
        (used_ips as f64 / total_ips as f64) * 100.0
    } else {
        0.0
    };

    IpRangeResponse {
        id: range.id,
        tenant_id: range.tenant_id,
        name: range.name.clone(),
        description: range.description.clone(),
        network_address: range.network_address.clone(),
        subnet_mask: range.subnet_mask,
        gateway: range.gateway.clone(),
        dns_servers: range.dns_servers.clone(),
        vlan_id: range.vlan_id,
        ip_version: range.ip_version,
        total_ips,
        used_ips,
        available_ips,
        usage_percentage,
        allocated_ips: usage.allocated,
        reserved_ips: usage.reserved,
        unavailable_ips: usage.unavailable,
        expired_ips: usage.expired,
        other_ips: usage.other,
        created_by: range.created_by,
        created_at: range.created_at,
        updated_at: range.updated_at,
        is_active: range.is_active,
    }
}

/// Create new IP range
#[utoipa::path(
    post,
    path = "/v0/ipam/ip-range",
    tag = "IP Range",
    request_body = CreateIpRangeRequest,
    responses(
        (status = 201, description = "IP range created successfully", body = IpRangeResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn create_ip_range(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateIpRangeRequest>,
) -> impl IntoResponse {
    match service_create_ip_range(
        &state.conn,
        &request.tenant_id,
        &request.name,
        request.description.as_deref(),
        &request.network_address,
        request.subnet_mask,
        request.gateway.as_deref(),
        request.dns_servers,
        request.vlan_id,
        request.ip_version,
        &claims.sub,
    )
    .await
    {
        Ok(ip_range) => {
            let response = build_ip_range_response(&ip_range, RangeUsageStats::default());
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => Err(err.into_response()),
    }
}

/// Get all IP ranges
#[utoipa::path(
    get,
    path = "/v0/ipam/ip-range",
    tag = "IP Range",
    params(
        ListIpRangesQuery,
    ),
    responses(
        (status = 200, description = "IP ranges retrieved successfully", body = IpRangeListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn get_ip_ranges(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Query(query): Query<ListIpRangesQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service_get_ip_ranges(&state.conn, page, limit).await {
        Ok(result) => {
            let usage_map = result.usage;
            let mut range_responses = Vec::with_capacity(result.ip_ranges.len());

            for ip in result.ip_ranges {
                let usage = usage_map.get(&ip.id).cloned().unwrap_or_default();
                range_responses.push(build_ip_range_response(&ip, usage));
            }

            let response = IpRangeListResponse {
                ip_ranges: range_responses,
                total: result.total,
                page,
                limit,
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err(err.into_response()),
    }
}

/// Get single IP range by ID
#[utoipa::path(
    get,
    path = "/v0/ipam/ip-range/{id}",
    tag = "IP Range",
    params(
        ("id" = Uuid, Path, description = "IP Range ID")
    ),
    responses(
        (status = 200, description = "IP range retrieved successfully", body = IpRangeResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "IP range not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn get_ip_range_by_id(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service_get_ip_range_by_id(&state.conn, &id).await {
        Ok(ip_range) => {
            let usage_map = match fetch_ip_range_usage(&state.conn, &[ip_range.id]).await {
                Ok(map) => map,
                Err(err) => return Err(err.into_response()),
            };

            let usage = usage_map.get(&ip_range.id).cloned().unwrap_or_default();
            let response = build_ip_range_response(&ip_range, usage);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err(err.into_response()),
    }
}

/// Update IP range
#[utoipa::path(
    put,
    path = "/v0/ipam/ip-range/{id}",
    tag = "IP Range",
    params(
        ("id" = Uuid, Path, description = "IP Range ID")
    ),
    request_body = UpdateIpRangeRequest,
    responses(
        (status = 200, description = "IP range updated successfully", body = IpRangeResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "IP range not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn update_ip_range(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateIpRangeRequest>,
) -> impl IntoResponse {
    match service_update_ip_range(
        &state.conn,
        &id,
        request.name,
        request.description,
        request.network_address,
        request.subnet_mask,
        request.gateway,
        request.dns_servers,
        request.vlan_id,
        request.ip_version,
    )
    .await
    {
        Ok(ip_range) => {
            let usage_map = match fetch_ip_range_usage(&state.conn, &[ip_range.id]).await {
                Ok(map) => map,
                Err(err) => return Err(err.into_response()),
            };
            let usage = usage_map.get(&ip_range.id).cloned().unwrap_or_default();
            let response = build_ip_range_response(&ip_range, usage);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err(err.into_response()),
    }
}

/// Delete IP range (soft delete)
#[utoipa::path(
    delete,
    path = "/v0/ipam/ip-range/{id}",
    tag = "IP Range",
    params(
        ("id" = Uuid, Path, description = "IP Range ID")
    ),
    responses(
        (status = 204, description = "IP range deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "IP range not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn delete_ip_range(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service_delete_ip_range(&state.conn, &id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_response()),
    }
}
