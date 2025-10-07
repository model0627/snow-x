use crate::api::v0::routes::device_library::handlers::{
    create_library, delete_library, get_libraries, get_library_by_id, update_library,
};
use crate::middleware::auth::access_jwt_auth;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_device_library_routes() -> Router<crate::AppState> {
    Router::new()
        .route(
            "/",
            get(get_libraries)
                .post(create_library)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/{id}",
            get(get_library_by_id)
                .put(update_library)
                .delete(delete_library)
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
}
