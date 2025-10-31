use crate::api::v0::routes::ip_address::handlers::{create_bulk_ip_addresses, get_ip_addresses};
use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::{Router, middleware, routing::get};

pub fn ip_address_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/v0/ipam/ip-address",
            get(get_ip_addresses).route_layer(middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/v0/ipam/ip-address/bulk",
            axum::routing::post(create_bulk_ip_addresses)
                .route_layer(middleware::from_fn(access_jwt_auth)),
        )
}
