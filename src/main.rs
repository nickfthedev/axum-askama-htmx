mod templates;

use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use templates::HelloTemplate;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber;

//
#[tokio::main]
async fn main() {
    // For logs
    tracing_subscriber::fmt::init();

    // Setting routes
    let app = app_router();

    // Setting port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);
    // Starting sever
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// The app router is the router for the app
fn app_router() -> Router {
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

pub async fn root_handler() -> HelloTemplate<'static> {
    HelloTemplate { name: "wanker" }
}
