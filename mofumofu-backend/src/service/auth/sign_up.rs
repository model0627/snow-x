use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::dto::user::request::create::CreateUserRequest;
use crate::entity::user_refresh_tokens::ActiveModel as RefreshTokenActiveModel;
use crate::repository::auth::create_refresh_token::repository_create_refresh_token;
use crate::repository::user::create_user::repository_create_user;
use crate::service::auth::jwt::{create_jwt_access_token, create_jwt_refresh_token};
use crate::service::error::errors::{Errors, ServiceResult};
use crate::state::AppState;
use sea_orm::{Set, TransactionTrait};
use tracing::info;

pub async fn service_sign_up(
    state: &AppState,
    user_agent: Option<String>,
    ip_address: Option<String>,
    payload: CreateUserRequest,
) -> ServiceResult<AuthJWTResponse> {
    let txn = state.conn.begin().await?;

    // 사용자 생성 (is_verified는 true로 설정됨)
    let user = repository_create_user(&txn, payload).await?;

    // JWT 토큰 생성
    let access_token =
        create_jwt_access_token(&user.id).map_err(|e| Errors::TokenCreationError(e.to_string()))?;

    let refresh_token = create_jwt_refresh_token(&user.id)
        .map_err(|e| Errors::TokenCreationError(e.to_string()))?;

    // Refresh 토큰 저장
    let refresh_model = RefreshTokenActiveModel {
        id: Set(refresh_token.jti),
        user_id: Set(user.id.clone()),
        ip_address: Set(ip_address),
        user_agent: Set(user_agent),
        refresh_token: Set(refresh_token.token.clone()),
        expires_at: Set(refresh_token.expires_at),
        created_at: Set(refresh_token.issued_at),
        revoked_at: Default::default(),
    };

    repository_create_refresh_token(&txn, refresh_model).await?;

    txn.commit().await?;

    info!("User created successfully: {}", user.email);

    Ok(AuthJWTResponse {
        access_token,
        cookie_refresh_token: refresh_token.token,
    })
}
