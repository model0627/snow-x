use super::openapi::ApiDoc;
use crate::api::v0::routes::admin::routes::admin_routes;
use crate::api::v0::routes::auth::routes::auth_routes;
use crate::api::v0::routes::comment::routes::comment_routes;
use crate::api::v0::routes::contact::routes::create_contact_routes;
use crate::api::v0::routes::custodian::routes::create_custodian_routes;
use crate::api::v0::routes::device::routes::create_device_routes;
use crate::api::v0::routes::device_library::routes::create_device_library_routes;
use crate::api::v0::routes::draft::routes::draft_routes;
use crate::api::v0::routes::follow::routes::follow_routes;
use crate::api::v0::routes::hashtag::routes::hashtag_routes;
use crate::api::v0::routes::ip_address::routes::ip_address_routes;
use crate::api::v0::routes::ip_range::routes::ip_range_routes;
use crate::api::v0::routes::like::routes::like_routes;
use crate::api::v0::routes::office::routes::office_routes;
use crate::api::v0::routes::post::routes::post_routes;
use crate::api::v0::routes::rack::routes::create_rack_routes;
use crate::api::v0::routes::report::routes::report_routes;
use crate::api::v0::routes::user::routes::user_routes;
use crate::service::error::errors::handler_404;
use crate::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// API + Swagger UI 라우터 통합
pub fn api_routes() -> Router<AppState> {
    println!("DEBUG: api_routes() function called - creating API router");

    println!("DEBUG: Creating basic router with SwaggerUI");
    let mut router =
        Router::new().merge(SwaggerUi::new("/docs").url("/swagger.json", ApiDoc::openapi()));

    println!("DEBUG: Adding auth routes");
    router = router.nest("/v0", auth_routes());

    println!("DEBUG: Adding user routes");
    router = router.nest("/v0", user_routes());

    println!("DEBUG: Adding post routes");
    router = router.nest("/v0", post_routes());

    println!("DEBUG: Adding draft routes");
    router = router.nest("/v0", draft_routes());

    println!("DEBUG: Adding comment routes");
    router = router.nest("/v0", comment_routes());

    println!("DEBUG: Adding follow routes");
    router = router.nest("/v0", follow_routes());

    println!("DEBUG: Adding like routes");
    router = router.nest("/v0", like_routes());

    println!("DEBUG: Adding hashtag routes");
    router = router.nest("/v0/hashtag", hashtag_routes());

    println!("DEBUG: Adding report routes");
    router = router.nest("/v0", report_routes());

    println!("DEBUG: About to add office routes");
    router = router.nest("/v0/ipam", office_routes());
    println!("DEBUG: Office routes added successfully");

    println!("DEBUG: Adding IP range routes");
    router = router.merge(ip_range_routes());
    println!("DEBUG: IP range routes added successfully");

    println!("DEBUG: Adding IP address routes");
    router = router.merge(ip_address_routes());
    println!("DEBUG: IP address routes added successfully");

    println!("DEBUG: Adding rack routes");
    router = router.nest("/v0/ipam/racks", create_rack_routes());
    println!("DEBUG: Rack routes added successfully");

    println!("DEBUG: Adding device routes");
    router = router.nest("/v0/ipam/device", create_device_routes());
    println!("DEBUG: Device routes added successfully");

    println!("DEBUG: Adding device library routes");
    router = router.nest("/v0/ipam/device-library", create_device_library_routes());
    println!("DEBUG: Device library routes added successfully");

    println!("DEBUG: Adding contact routes");
    router = router.nest("/v0/ipam/contact", create_contact_routes());
    println!("DEBUG: Contact routes added successfully");

    println!("DEBUG: Adding custodian routes");
    router = router.nest("/v0/custodian", create_custodian_routes());
    println!("DEBUG: Custodian routes added successfully");

    println!("DEBUG: Adding admin routes");
    router = router.nest("/v0/admin", admin_routes());

    println!("DEBUG: Adding fallback handler");
    router = router.fallback(handler_404);

    println!("DEBUG: API router created with all routes including office");
    router
}
