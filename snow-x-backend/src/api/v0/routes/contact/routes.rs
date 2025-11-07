use crate::api::v0::routes::contact::handlers::{
    create_contact, delete_contact, get_contact_by_id, get_contacts, update_contact,
};
use crate::middleware::auth::access_jwt_auth;
use axum::{Router, routing::get};

pub fn create_contact_routes() -> Router<crate::AppState> {
    Router::new()
        .route(
            "/",
            get(get_contacts)
                .post(create_contact)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}",
            get(get_contact_by_id)
                .put(update_contact)
                .delete(delete_contact)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
}
