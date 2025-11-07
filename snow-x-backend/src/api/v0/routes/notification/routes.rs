use axum::{
    Router, middleware,
    routing::{get, patch},
};

use crate::middleware::auth::access_jwt_auth;

use super::handlers::{create_notification, get_notifications, update_notification_status};

pub fn notification_routes() -> Router<crate::AppState> {
    Router::new()
        .route(
            "/notifications",
            get(get_notifications)
                .post(create_notification)
                .route_layer(middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/notifications/{id}",
            patch(update_notification_status).route_layer(middleware::from_fn(access_jwt_auth)),
        )
}
