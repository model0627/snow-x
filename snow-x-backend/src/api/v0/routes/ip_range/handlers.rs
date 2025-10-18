use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    dto::auth::internal::access_token::AccessTokenClaims,
    service::ip_range::{
        create_ip_range::service_create_ip_range,
        delete_ip_range::service_delete_ip_range,
        get_ip_range_by_id::service_get_ip_range_by_id,
        get_ip_ranges::service_get_ip_ranges,
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
    pub name: String,
    pub description: Option<String>,
    pub network_address: String,
    pub subnet_mask: i32,
    pub gateway: Option<String>,
    pub dns_servers: Option<Vec<String>>,
    pub vlan_id: Option<i32>,
    pub ip_version: i32,
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
            let response = IpRangeResponse {
                id: ip_range.id,
                name: ip_range.name,
                description: ip_range.description,
                network_address: ip_range.network_address,
                subnet_mask: ip_range.subnet_mask,
                gateway: ip_range.gateway,
                dns_servers: ip_range.dns_servers,
                vlan_id: ip_range.vlan_id,
                ip_version: ip_range.ip_version,
                created_by: ip_range.created_by,
                created_at: ip_range.created_at,
                updated_at: ip_range.updated_at,
                is_active: ip_range.is_active,
            };
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
            let response = IpRangeListResponse {
                ip_ranges: result
                    .ip_ranges
                    .into_iter()
                    .map(|ip| IpRangeResponse {
                        id: ip.id,
                        name: ip.name,
                        description: ip.description,
                        network_address: ip.network_address,
                        subnet_mask: ip.subnet_mask,
                        gateway: ip.gateway,
                        dns_servers: ip.dns_servers,
                        vlan_id: ip.vlan_id,
                        ip_version: ip.ip_version,
                        created_by: ip.created_by,
                        created_at: ip.created_at,
                        updated_at: ip.updated_at,
                        is_active: ip.is_active,
                    })
                    .collect(),
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
            let response = IpRangeResponse {
                id: ip_range.id,
                name: ip_range.name,
                description: ip_range.description,
                network_address: ip_range.network_address,
                subnet_mask: ip_range.subnet_mask,
                gateway: ip_range.gateway,
                dns_servers: ip_range.dns_servers,
                vlan_id: ip_range.vlan_id,
                ip_version: ip_range.ip_version,
                created_by: ip_range.created_by,
                created_at: ip_range.created_at,
                updated_at: ip_range.updated_at,
                is_active: ip_range.is_active,
            };
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
            let response = IpRangeResponse {
                id: ip_range.id,
                name: ip_range.name,
                description: ip_range.description,
                network_address: ip_range.network_address,
                subnet_mask: ip_range.subnet_mask,
                gateway: ip_range.gateway,
                dns_servers: ip_range.dns_servers,
                vlan_id: ip_range.vlan_id,
                ip_version: ip_range.ip_version,
                created_by: ip_range.created_by,
                created_at: ip_range.created_at,
                updated_at: ip_range.updated_at,
                is_active: ip_range.is_active,
            };
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
