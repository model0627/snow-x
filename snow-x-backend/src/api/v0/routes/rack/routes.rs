use crate::api::v0::routes::rack::handlers::{create_rack, create_rack_direct, delete_rack, get_rack_by_id, get_racks};
use crate::middleware::auth::access_jwt_auth;
use axum::{routing::{get, post}, Router};

pub fn create_rack_routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(get_racks).post(create_rack_direct).route_layer(axum::middleware::from_fn(access_jwt_auth)))
        .route("/{rack_id}", get(get_rack_by_id).delete(delete_rack).route_layer(axum::middleware::from_fn(access_jwt_auth)))
        .nest(
            "/server-rooms/{server_room_id}/racks",
            Router::new().route("/", axum::routing::post(create_rack).route_layer(axum::middleware::from_fn(access_jwt_auth))),
        )
}