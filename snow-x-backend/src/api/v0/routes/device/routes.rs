use crate::api::v0::routes::device::handlers::{
    assign_contact_to_device, assign_ip_to_device, create_device, delete_device, get_device_by_id,
    get_device_contacts, get_device_ip_addresses, get_devices, unassign_contact_from_device,
    unassign_ip_from_device, update_device,
};
use crate::middleware::auth::access_jwt_auth;
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn create_device_routes() -> Router<crate::AppState> {
    Router::new()
        .route(
            "/",
            get(get_devices)
                .post(create_device)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}",
            get(get_device_by_id)
                .put(update_device)
                .delete(delete_device)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}/ip-addresses",
            get(get_device_ip_addresses).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}/ip-address",
            post(assign_ip_to_device).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}/ip-address/{ip_address_id}",
            delete(unassign_ip_from_device).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}/contacts",
            get(get_device_contacts).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}/contact",
            post(assign_contact_to_device).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}/contact/{contact_id}",
            delete(unassign_contact_from_device)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
}
