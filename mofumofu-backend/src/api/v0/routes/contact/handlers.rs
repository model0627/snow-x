use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::contact::request::{CreateContactRequest, UpdateContactRequest};
use crate::dto::contact::response::{ContactInfoResponse, ContactListResponse};
use crate::service::contact::{
    service_create_contact, service_delete_contact, service_get_contact_by_id,
    service_get_contacts, service_update_contact,
};
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_contact,
        get_contacts,
        get_contact_by_id,
        update_contact,
        delete_contact
    ),
    components(schemas(
        CreateContactRequest,
        UpdateContactRequest,
        ContactInfoResponse,
        ContactListResponse
    )),
    tags(
        (name = "Contact", description = "연락처 관리 API")
    )
)]
pub struct ContactApiDoc;

#[derive(Deserialize)]
pub struct ContactQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
    pub department: Option<String>,
    pub is_active: Option<bool>,
}

#[utoipa::path(
    post,
    path = "/v0/ipam/contact",
    tags = ["Contact"],
    summary = "연락처 생성",
    description = "새로운 연락처를 생성합니다",
    request_body = CreateContactRequest,
    responses(
        (status = 201, description = "연락처 생성 성공", body = ContactInfoResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn create_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateContactRequest>,
) -> Result<(StatusCode, Json<ContactInfoResponse>), (StatusCode, Json<serde_json::Value>)> {
    match service_create_contact(&state.conn, request, claims.sub).await {
        Ok(contact) => Ok((StatusCode::CREATED, Json(contact))),
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
    path = "/v0/ipam/contact",
    tags = ["Contact"],
    summary = "연락처 목록 조회",
    description = "연락처 목록을 페이지네이션과 검색 조건으로 조회합니다",
    params(
        ("page" = Option<u64>, Query, description = "페이지 번호 (기본값: 1)"),
        ("limit" = Option<u64>, Query, description = "페이지 크기 (기본값: 20)"),
        ("search" = Option<String>, Query, description = "검색어 (이름, 이메일, 전화번호)"),
        ("department" = Option<String>, Query, description = "부서 필터"),
        ("is_active" = Option<bool>, Query, description = "활성 상태 필터 (기본값: true)")
    ),
    responses(
        (status = 200, description = "연락처 목록 조회 성공", body = ContactListResponse),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_contacts(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Query(params): Query<ContactQueryParams>,
) -> Result<Json<ContactListResponse>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);

    match service_get_contacts(
        &state.conn,
        page,
        limit,
        params.search,
        params.department,
        params.is_active,
    )
    .await
    {
        Ok(contacts) => Ok(Json(contacts)),
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
    path = "/v0/ipam/contact/{id}",
    tags = ["Contact"],
    summary = "연락처 상세 조회",
    description = "특정 연락처의 상세 정보를 조회합니다",
    params(
        ("id" = Uuid, Path, description = "연락처 ID")
    ),
    responses(
        (status = 200, description = "연락처 조회 성공", body = ContactInfoResponse),
        (status = 404, description = "연락처를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn get_contact_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ContactInfoResponse>, (StatusCode, Json<serde_json::Value>)> {
    match service_get_contact_by_id(&state.conn, id).await {
        Ok(Some(contact)) => Ok(Json(contact)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "연락처를 찾을 수 없습니다"
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
    path = "/v0/ipam/contact/{id}",
    tags = ["Contact"],
    summary = "연락처 수정",
    description = "특정 연락처의 정보를 수정합니다",
    params(
        ("id" = Uuid, Path, description = "연락처 ID")
    ),
    request_body = UpdateContactRequest,
    responses(
        (status = 200, description = "연락처 수정 성공", body = ContactInfoResponse),
        (status = 404, description = "연락처를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn update_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateContactRequest>,
) -> Result<Json<ContactInfoResponse>, (StatusCode, Json<serde_json::Value>)> {
    match service_update_contact(&state.conn, id, request).await {
        Ok(contact) => Ok(Json(contact)),
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
    path = "/v0/ipam/contact/{id}",
    tags = ["Contact"],
    summary = "연락처 삭제",
    description = "특정 연락처를 삭제합니다 (소프트 삭제)",
    params(
        ("id" = Uuid, Path, description = "연락처 ID")
    ),
    responses(
        (status = 204, description = "연락처 삭제 성공"),
        (status = 404, description = "연락처를 찾을 수 없음"),
        (status = 401, description = "인증 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(("Bearer" = []))
)]
pub async fn delete_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    match service_delete_contact(&state.conn, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            })),
        )),
    }
}
