mod templates;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber;

use templates::HelloTemplate;

#[tokio::main]
async fn main() {
    // For logs
    tracing_subscriber::fmt::init();

    // Setting routes
    let app = Router::new().route("/", get(root_handler));

    // Setting port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);
    // Starting sever
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn root_handler() -> HelloTemplate<'static> {
    HelloTemplate { name: "wanker" }
}

