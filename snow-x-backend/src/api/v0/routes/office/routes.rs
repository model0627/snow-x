use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::{Router, middleware, routing::get};

use super::handlers::{
    create_office, create_server_room, delete_office, delete_server_room_by_id, get_office,
    get_offices, get_server_room_by_id, get_server_rooms, update_office, update_server_room_by_id,
};

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
        .route("/office", get(get_offices).post(create_office))
        .route(
            "/office/{id}",
            get(get_office).put(update_office).delete(delete_office),
        )
        // Server Room routes
        .route(
            "/office/{office_id}/server-room",
            get(get_server_rooms).post(create_server_room),
        )
        .route(
            "/office/{office_id}/server-room/{id}",
            get(get_server_room_by_id)
                .put(update_server_room_by_id)
                .delete(delete_server_room_by_id),
        )
        .route_layer(middleware::from_fn(access_jwt_auth));

    let router = simple_routes.merge(protected_routes);

    println!(
        "DEBUG: office router created with routes: /test, /simple, /office, /office/{{id}}, /office/{{office_id}}/server-room, /office/{{office_id}}/server-room/{{id}}"
    );
    router
}
