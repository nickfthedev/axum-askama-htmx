use crate::handlers::root::root_handler;
use crate::handlers::todo::{todo_add, todo_main};

use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sqlx::{PgPool, pool};
use tower_http::services::ServeDir;
use std::sync::Arc;
use axum::response::Response;

// The app router  for the app
pub fn app_router(pool: Arc<PgPool>) -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    // you can convert handler function to service
    let service = handle_404.into_service();

    let serve_dir = ServeDir::new("public").not_found_service(service);

    Router::new()
        // TEST ROUTES
        .route("/", get(root_handler))
        .route("/health", get(|| async { "Health OK" }))
        .route("/clickme", post(|| async { "Hi HTMX" }))
        // todoapp routes
        .route("/todo", get(todo_main)) 
        .route("/todo", post(move || todo_add(pool.clone())))
        .fallback_service(serve_dir)
}
