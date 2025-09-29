use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::dto::user::request::create::CreateUserRequest;
use crate::service::auth::service_sign_up;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use crate::utils::extract_ip_address::extract_ip_address;
use crate::utils::extract_user_agent::extract_user_agent;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use std::net::SocketAddr;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/auth/sign_up",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created successfully", body = AuthJWTResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Handle or email already exists"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn sign_up(
    user_agent: Option<TypedHeader<UserAgent>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<AuthJWTResponse, Errors> {
    info!("Received POST request to sign up: {:?}", payload);

    let ip_str = extract_ip_address(&headers, addr);
    let ua_str = extract_user_agent(user_agent);

    let response = service_sign_up(&state, Some(ua_str), Some(ip_str), payload).await?;

    Ok(response)
}
