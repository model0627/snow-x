use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::service::ip_address::{
    service_create_bulk_ip_addresses, service_get_ip_addresses, IpAddressListResult,
};
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct IpAddressResponse {
    pub id: Uuid,
    pub ip_range_id: Uuid,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
}

#[derive(Serialize, ToSchema)]
pub struct IpAddressListResponse {
    pub ip_addresses: Vec<IpAddressResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateBulkIpAddressesRequest {
    pub ip_range_id: Uuid,
    pub start_ip: String,
    pub end_ip: String,
    pub status: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetIpAddressesQuery {
    pub ip_range_id: Option<Uuid>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

/// Create bulk IP addresses
#[utoipa::path(
    post,
    path = "/v0/ipam/ip-address/bulk",
    tag = "IP Address",
    request_body = CreateBulkIpAddressesRequest,
    responses(
        (status = 200, description = "IP addresses created successfully", body = Vec<IpAddressResponse>),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn create_bulk_ip_addresses(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateBulkIpAddressesRequest>,
) -> impl IntoResponse {
    match service_create_bulk_ip_addresses(
        &state.conn,
        &request.ip_range_id,
        &request.start_ip,
        &request.end_ip,
        &request.status,
        request.description.as_deref(),
        &claims.sub,
    )
    .await
    {
        Ok(ip_addresses) => {
            let responses: Vec<IpAddressResponse> = ip_addresses
                .into_iter()
                .map(|addr| IpAddressResponse {
                    id: addr.id,
                    ip_range_id: addr.ip_range_id,
                    ip_address: addr.ip_address,
                    mac_address: addr.mac_address,
                    hostname: addr.hostname,
                    status: addr.status,
                    description: addr.description,
                    created_by: addr.created_by,
                    created_at: addr.created_at.to_rfc3339(),
                    updated_at: addr.updated_at.to_rfc3339(),
                    is_active: addr.is_active,
                })
                .collect();

            Ok((StatusCode::OK, Json(responses)))
        }
        Err(err) => Err(err.into_response()),
    }
}

/// Get IP addresses
#[utoipa::path(
    get,
    path = "/v0/ipam/ip-address",
    tag = "IP Address",
    params(GetIpAddressesQuery),
    responses(
        (status = 200, description = "IP addresses retrieved successfully", body = IpAddressListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn get_ip_addresses(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Query(query): Query<GetIpAddressesQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);

    match service_get_ip_addresses(
        &state.conn,
        query.ip_range_id.as_ref(),
        query.status.as_deref(),
        query.search.as_deref(),
        page,
        limit,
    )
    .await
    {
        Ok(IpAddressListResult {
            ip_addresses,
            total,
        }) => {
            let responses: Vec<IpAddressResponse> = ip_addresses
                .into_iter()
                .map(|addr| IpAddressResponse {
                    id: addr.id,
                    ip_range_id: addr.ip_range_id,
                    ip_address: addr.ip_address,
                    mac_address: addr.mac_address,
                    hostname: addr.hostname,
                    status: addr.status,
                    description: addr.description,
                    created_by: addr.created_by,
                    created_at: addr.created_at.to_rfc3339(),
                    updated_at: addr.updated_at.to_rfc3339(),
                    is_active: addr.is_active,
                })
                .collect();

            Ok((
                StatusCode::OK,
                Json(IpAddressListResponse {
                    ip_addresses: responses,
                    total,
                    page,
                    limit,
                }),
            ))
        }
        Err(err) => Err(err.into_response()),
    }
}
