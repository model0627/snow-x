use axum::{Router, middleware, routing::get};

use crate::{middleware::auth::access_jwt_auth, state::AppState};

use super::handlers::{
    create_ip_range, delete_ip_range, get_ip_range_by_id, get_ip_ranges, update_ip_range,
};

pub fn ip_range_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/v0/ipam/ip-range",
            axum::routing::post(create_ip_range)
                .get(get_ip_ranges)
                .route_layer(middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/v0/ipam/ip-range/{id}",
            get(get_ip_range_by_id)
                .put(update_ip_range)
                .delete(delete_ip_range)
                .route_layer(middleware::from_fn(access_jwt_auth)),
        )
}
