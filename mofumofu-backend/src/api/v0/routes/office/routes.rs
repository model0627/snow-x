use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};

use super::handlers::{create_office, delete_office, get_office, get_offices, update_office};

// Simple test handler to verify route registration
async fn test_handler() -> &'static str {
    println!("DEBUG: test_handler called!");
    "Office test route working!"
}

pub fn office_routes() -> Router<AppState> {
    println!("DEBUG: office_routes() function called - creating office router");

    // Create a simple test to verify the basic routing is working
    let simple_routes = Router::new()
        .route("/test", get(test_handler))
        .route("/simple", get(|| async { "Simple route working!" }));

    let protected_routes = Router::new()
        .route(
            "/office",
            get(get_offices).post(create_office),
        )
        .route(
            "/office/{id}",
            get(get_office).put(update_office).delete(delete_office),
        )
        .route_layer(middleware::from_fn(access_jwt_auth));

    let router = simple_routes.merge(protected_routes);

    println!("DEBUG: office router created with routes: /test, /simple, /office, /office/{{id}}");
    router
}