use crate::AppState;
use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::device_library::request::{CreateLibraryRequest, UpdateLibraryRequest};
use crate::dto::device_library::response::{LibraryInfoResponse, LibraryListResponse};
use crate::service::device_library::{
    service_create_library, service_delete_library, service_get_libraries,
    service_get_library_by_id, service_update_library,
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
        create_library,
        get_libraries,
        get_library_by_id,
        update_library,
        delete_library
    ),
    components(schemas(
        CreateLibraryRequest,
        UpdateLibraryRequest,
        LibraryInfoResponse,
        LibraryListResponse
    )),
    tags(
        (name = "Device Library", description = "장비 라이브러리 관리 API")
    )
)]
pub struct DeviceLibraryApiDoc;

#[derive(Deserialize)]
pub struct LibraryQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
    pub device_type: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v0/ipam/device-library",
    tags = ["Device Library"],
    summary = "장비 라이브러리 생성",
    description = "새로운 장비 라이브러리(템플릿)를 생성합니다",
    request_body = CreateLibraryRequest,
    responses(
        (status = 201, description = "라이브러리 생성 성공", body = LibraryInfoResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn create_library(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateLibraryRequest>,
) -> Result<(StatusCode, Json<LibraryInfoResponse>), (StatusCode, Json<serde_json::Value>)> {
    match service_create_library(&state.conn, request, claims.sub).await {
        Ok(library) => Ok((StatusCode::CREATED, Json(library))),
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
    path = "/v0/ipam/device-library",
    tags = ["Device Library"],
    summary = "장비 라이브러리 목록 조회",
    description = "장비 라이브러리 목록을 페이지네이션과 검색 조건으로 조회합니다",
    params(
        ("page" = Option<u64>, Query, description = "페이지 번호 (기본값: 1)"),
        ("limit" = Option<u64>, Query, description = "페이지 크기 (기본값: 20)"),
        ("search" = Option<String>, Query, description = "검색어 (라이브러리명)"),
        ("device_type" = Option<String>, Query, description = "장비 유형 필터")
    ),
    responses(
        (status = 200, description = "라이브러리 목록 조회 성공", body = LibraryListResponse),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_libraries(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Query(params): Query<LibraryQueryParams>,
) -> Result<Json<LibraryListResponse>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);

    match service_get_libraries(&state.conn, page, limit, params.search).await {
        Ok(libraries) => Ok(Json(libraries)),
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
    path = "/v0/ipam/device-library/{id}",
    tags = ["Device Library"],
    summary = "장비 라이브러리 상세 조회",
    description = "특정 장비 라이브러리의 상세 정보를 조회합니다",
    params(
        ("id" = Uuid, Path, description = "라이브러리 ID")
    ),
    responses(
        (status = 200, description = "라이브러리 조회 성공", body = LibraryInfoResponse),
        (status = 404, description = "라이브러리를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_library_by_id(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> Result<Json<LibraryInfoResponse>, (StatusCode, Json<serde_json::Value>)> {
    match service_get_library_by_id(&state.conn, id).await {
        Ok(Some(library)) => Ok(Json(library)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "라이브러리를 찾을 수 없습니다"
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
    path = "/v0/ipam/device-library/{id}",
    tags = ["Device Library"],
    summary = "장비 라이브러리 수정",
    description = "특정 장비 라이브러리의 정보를 수정합니다",
    params(
        ("id" = Uuid, Path, description = "라이브러리 ID")
    ),
    request_body = UpdateLibraryRequest,
    responses(
        (status = 200, description = "라이브러리 수정 성공", body = LibraryInfoResponse),
        (status = 404, description = "라이브러리를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn update_library(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateLibraryRequest>,
) -> Result<Json<LibraryInfoResponse>, (StatusCode, Json<serde_json::Value>)> {
    match service_update_library(&state.conn, id, request).await {
        Ok(library) => Ok(Json(library)),
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
    path = "/v0/ipam/device-library/{id}",
    tags = ["Device Library"],
    summary = "장비 라이브러리 삭제",
    description = "특정 장비 라이브러리를 삭제합니다 (소프트 삭제)",
    params(
        ("id" = Uuid, Path, description = "라이브러리 ID")
    ),
    responses(
        (status = 204, description = "라이브러리 삭제 성공"),
        (status = 404, description = "라이브러리를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn delete_library(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    match service_delete_library(&state.conn, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}
