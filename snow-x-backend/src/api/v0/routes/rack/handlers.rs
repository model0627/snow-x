use crate::AppState;
use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::rack::request::create_rack::CreateRackRequest;
use crate::dto::rack::request::update_rack::UpdateRackRequest;
use crate::dto::rack::response::rack_info::RackInfoResponse;
use crate::dto::rack::response::rack_list::RackListResponse;
use crate::service::rack::{
    service_create_rack, service_delete_rack, service_get_rack_by_id, service_get_racks,
    service_update_rack,
};
use axum::{
    Extension,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_rack,
        update_rack,
        create_rack_direct,
        get_racks,
        get_rack_by_id,
        delete_rack
    ),
    components(schemas(
        CreateRackRequest,
        UpdateRackRequest,
        RackInfoResponse,
        RackListResponse
    )),
    tags(
        (name = "Rack", description = "랙 관리 API")
    )
)]
pub struct RackApiDoc;

#[derive(Deserialize)]
pub struct RackQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[utoipa::path(
    post,
    path = "/v0/ipam/racks",
    tags = ["Rack"],
    summary = "랙 생성 (간단)",
    description = "새로운 랙을 생성합니다 (서버룸 ID는 요청 본문에 포함)",
    request_body = CreateRackRequest,
    responses(
        (status = 201, description = "랙 생성 성공", body = RackInfoResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn create_rack_direct(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateRackRequest>,
) -> Result<(StatusCode, Json<RackInfoResponse>), (StatusCode, Json<serde_json::Value>)> {
    // 실제 서버룸 ID 사용
    let server_room_id = request
        .server_room_id
        .unwrap_or_else(|| Uuid::parse_str("e0d147a1-8790-4112-923a-f790c1c1b326").unwrap());

    match service_create_rack(&state.conn, request, server_room_id, claims.sub).await {
        Ok(rack) => Ok((StatusCode::CREATED, Json(rack))),
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
    path = "/v0/ipam/server-rooms/{server_room_id}/racks",
    tags = ["Rack"],
    summary = "랙 생성",
    description = "새로운 랙을 생성합니다",
    params(
        ("server_room_id" = Uuid, Path, description = "서버실 ID")
    ),
    request_body = CreateRackRequest,
    responses(
        (status = 201, description = "랙 생성 성공", body = RackInfoResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn create_rack(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Path(server_room_id): Path<Uuid>,
    Json(request): Json<CreateRackRequest>,
) -> Result<(StatusCode, Json<RackInfoResponse>), (StatusCode, Json<serde_json::Value>)> {
    match service_create_rack(&state.conn, request, server_room_id, claims.sub).await {
        Ok(rack) => Ok((StatusCode::CREATED, Json(rack))),
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
    path = "/v0/ipam/racks",
    tags = ["Rack"],
    summary = "랙 목록 조회",
    description = "랙 목록을 페이지네이션으로 조회합니다",
    params(
        ("page" = Option<u64>, Query, description = "페이지 번호 (기본값: 1)"),
        ("limit" = Option<u64>, Query, description = "페이지 크기 (기본값: 20)")
    ),
    responses(
        (status = 200, description = "랙 목록 조회 성공", body = RackListResponse),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_racks(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Query(params): Query<RackQueryParams>,
) -> Result<Json<RackListResponse>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);

    match service_get_racks(&state.conn, page, limit).await {
        Ok(racks) => Ok(Json(racks)),
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
    path = "/v0/ipam/racks/{rack_id}",
    tags = ["Rack"],
    summary = "랙 상세 조회",
    description = "특정 랙의 상세 정보를 조회합니다",
    params(
        ("rack_id" = Uuid, Path, description = "랙 ID")
    ),
    responses(
        (status = 200, description = "랙 조회 성공", body = RackInfoResponse),
        (status = 404, description = "랙을 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_rack_by_id(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(rack_id): Path<Uuid>,
) -> Result<Json<RackInfoResponse>, (StatusCode, Json<serde_json::Value>)> {
    match service_get_rack_by_id(&state.conn, rack_id).await {
        Ok(Some(rack)) => Ok(Json(rack)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "랙을 찾을 수 없습니다"
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
    path = "/v0/ipam/racks/{rack_id}",
    tags = ["Rack"],
    summary = "랙 수정",
    description = "특정 랙의 속성을 수정합니다 (부분 업데이트)",
    params(
        ("rack_id" = Uuid, Path, description = "랙 ID")
    ),
    request_body = UpdateRackRequest,
    responses(
        (status = 200, description = "수정된 랙 정보", body = RackInfoResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요"),
        (status = 404, description = "랙을 찾을 수 없음"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn update_rack(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Path(rack_id): Path<Uuid>,
    Json(request): Json<UpdateRackRequest>,
) -> Result<(StatusCode, Json<RackInfoResponse>), (StatusCode, Json<serde_json::Value>)> {
    match service_update_rack(&state.conn, rack_id, request, claims.sub).await {
        Ok(rack) => Ok((StatusCode::OK, Json(rack))),
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
    path = "/v0/ipam/racks/{rack_id}",
    tags = ["Rack"],
    summary = "랙 삭제",
    description = "특정 랙을 삭제합니다 (소프트 삭제)",
    params(
        ("rack_id" = Uuid, Path, description = "랙 ID")
    ),
    responses(
        (status = 204, description = "랙 삭제 성공"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn delete_rack(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(rack_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    match service_delete_rack(&state.conn, rack_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}
