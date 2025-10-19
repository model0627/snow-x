use crate::api::v0::routes::ip_address::handlers::IpAddressResponse;
use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::device::request::create_device::CreateDeviceRequest;
use crate::dto::device::request::update_device::UpdateDeviceRequest;
use crate::dto::device::response::device_info::DeviceInfoResponse;
use crate::dto::device::response::device_list::DeviceListResponse;
use crate::service::device::{
    service_assign_ip_address, service_create_device, service_delete_device,
    service_get_device_by_id, service_get_device_ip_addresses, service_get_devices,
    service_unassign_ip_address, service_update_device,
};
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_device,
        get_devices,
        get_device_by_id,
        update_device,
        delete_device,
        get_device_ip_addresses,
        assign_ip_to_device,
        unassign_ip_from_device
    ),
    components(schemas(
        CreateDeviceRequest,
        UpdateDeviceRequest,
        DeviceInfoResponse,
        DeviceListResponse,
        IpAddressResponse,
        AssignIpRequest
    )),
    tags(
        (name = "Device", description = "장비 관리 API")
    )
)]
pub struct DeviceApiDoc;

#[derive(Deserialize)]
pub struct DeviceQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
    pub device_type: Option<String>,
    pub status: Option<String>,
    pub rack_id: Option<Uuid>,
}

#[utoipa::path(
    post,
    path = "/v0/ipam/device",
    tags = ["Device"],
    summary = "장비 생성",
    description = "새로운 장비를 생성합니다",
    request_body = CreateDeviceRequest,
    responses(
        (status = 201, description = "장비 생성 성공", body = DeviceInfoResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn create_device(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceInfoResponse>), (StatusCode, Json<serde_json::Value>)> {
    match service_create_device(&state.conn, request, claims.sub).await {
        Ok(device) => Ok((StatusCode::CREATED, Json(device))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/v0/ipam/device",
    tags = ["Device"],
    summary = "장비 목록 조회",
    description = "장비 목록을 페이지네이션과 검색 조건으로 조회합니다",
    params(
        ("page" = Option<u64>, Query, description = "페이지 번호 (기본값: 1)"),
        ("limit" = Option<u64>, Query, description = "페이지 크기 (기본값: 20)"),
        ("search" = Option<String>, Query, description = "검색어 (장비명)"),
        ("device_type" = Option<String>, Query, description = "장비 유형 필터"),
        ("status" = Option<String>, Query, description = "상태 필터")
    ),
    responses(
        (status = 200, description = "장비 목록 조회 성공", body = DeviceListResponse),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_devices(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Query(params): Query<DeviceQueryParams>,
) -> Result<Json<DeviceListResponse>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);

    match service_get_devices(
        &state.conn,
        page,
        limit,
        params.search,
        params.device_type,
        params.status,
        params.rack_id,
    )
    .await
    {
        Ok(devices) => Ok(Json(devices)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/v0/ipam/device/{id}",
    tags = ["Device"],
    summary = "장비 상세 조회",
    description = "특정 장비의 상세 정보를 조회합니다",
    params(
        ("id" = Uuid, Path, description = "장비 ID")
    ),
    responses(
        (status = 200, description = "장비 조회 성공", body = DeviceInfoResponse),
        (status = 404, description = "장비를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_device_by_id(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> Result<Json<DeviceInfoResponse>, (StatusCode, Json<serde_json::Value>)> {
    match service_get_device_by_id(&state.conn, id).await {
        Ok(Some(device)) => Ok(Json(device)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "장비를 찾을 수 없습니다"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(
    put,
    path = "/v0/ipam/device/{id}",
    tags = ["Device"],
    summary = "장비 수정",
    description = "특정 장비의 정보를 수정합니다",
    params(
        ("id" = Uuid, Path, description = "장비 ID")
    ),
    request_body = UpdateDeviceRequest,
    responses(
        (status = 200, description = "장비 수정 성공", body = DeviceInfoResponse),
        (status = 404, description = "장비를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn update_device(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateDeviceRequest>,
) -> Result<Json<DeviceInfoResponse>, (StatusCode, Json<serde_json::Value>)> {
    match service_update_device(&state.conn, id, request).await {
        Ok(device) => Ok(Json(device)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/v0/ipam/device/{id}",
    tags = ["Device"],
    summary = "장비 삭제",
    description = "특정 장비를 삭제합니다 (소프트 삭제)",
    params(
        ("id" = Uuid, Path, description = "장비 ID")
    ),
    responses(
        (status = 204, description = "장비 삭제 성공"),
        (status = 404, description = "장비를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn delete_device(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    match service_delete_device(&state.conn, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AssignIpRequest {
    pub ip_address_id: Uuid,
}

#[utoipa::path(
    get,
    path = "/v0/ipam/device/{id}/ip-addresses",
    tags = ["Device"],
    summary = "장비에 할당된 IP 주소 목록 조회",
    description = "특정 장비에 할당된 모든 IP 주소를 조회합니다",
    params(
        ("id" = Uuid, Path, description = "장비 ID")
    ),
    responses(
        (status = 200, description = "IP 주소 목록 조회 성공", body = Vec<IpAddressResponse>),
        (status = 404, description = "장비를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_device_ip_addresses(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(device_id): Path<Uuid>,
) -> Result<Json<Vec<IpAddressResponse>>, (StatusCode, Json<serde_json::Value>)> {
    match service_get_device_ip_addresses(&state.conn, device_id).await {
        Ok(ip_addresses) => Ok(Json(ip_addresses)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/v0/ipam/device/{id}/ip-address",
    tags = ["Device"],
    summary = "장비에 IP 주소 할당",
    description = "특정 장비에 IP 주소를 할당합니다",
    params(
        ("id" = Uuid, Path, description = "장비 ID")
    ),
    request_body = AssignIpRequest,
    responses(
        (status = 204, description = "IP 주소 할당 성공"),
        (status = 400, description = "잘못된 요청 (이미 할당된 IP)"),
        (status = 404, description = "장비 또는 IP 주소를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn assign_ip_to_device(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(device_id): Path<Uuid>,
    Json(request): Json<AssignIpRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    match service_assign_ip_address(&state.conn, device_id, request.ip_address_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("already assigned") {
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "IP 주소가 이미 이 장비에 할당되어 있습니다"
                    })),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": error_msg
                    })),
                ))
            }
        }
    }
}

#[utoipa::path(
    delete,
    path = "/v0/ipam/device/{id}/ip-address/{ip_address_id}",
    tags = ["Device"],
    summary = "장비에서 IP 주소 할당 해제",
    description = "특정 장비에서 IP 주소 할당을 해제합니다",
    params(
        ("id" = Uuid, Path, description = "장비 ID"),
        ("ip_address_id" = Uuid, Path, description = "IP 주소 ID")
    ),
    responses(
        (status = 204, description = "IP 주소 할당 해제 성공"),
        (status = 404, description = "IP 할당을 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn unassign_ip_from_device(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path((device_id, ip_address_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    match service_unassign_ip_address(&state.conn, device_id, ip_address_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("not found") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": "IP 할당을 찾을 수 없습니다"
                    })),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": error_msg
                    })),
                ))
            }
        }
    }
}
