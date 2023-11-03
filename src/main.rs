pub mod handlers;
pub mod router;
pub mod templates;

use router::app_router;

use std::net::SocketAddr;
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
