use crate::api::v0::routes::custodian::handlers::{
    create_policy, delete_policy, execute_policy, get_execution, get_policies, get_policy,
    update_policy, validate_yaml,
};
use crate::middleware::auth::access_jwt_auth;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_custodian_routes() -> Router<crate::AppState> {
    Router::new()
        .route(
            "/policies",
            get(get_policies)
                .post(create_policy)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/policies/{id}",
            get(get_policy)
                .put(update_policy)
                .delete(delete_policy)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/execute",
            post(execute_policy).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/executions/{id}",
            get(get_execution).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/validate",
            post(validate_yaml).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
}
