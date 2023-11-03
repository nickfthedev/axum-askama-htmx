use crate::handlers::root::root_handler;

use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

// The app router is the router for the app
pub fn app_router() -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    // you can convert handler function to service
    let service = handle_404.into_service();

    let serve_dir = ServeDir::new("public").not_found_service(service);

    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .route("/clickme", post(|| async { "Hi HTMX" }))
        .route("/", get(root_handler))
        .fallback_service(serve_dir)
}

