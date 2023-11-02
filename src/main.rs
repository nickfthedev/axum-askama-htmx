mod templates;

use axum::{routing::get, Router};
use std::net::SocketAddr;

use templates::HelloTemplate;

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


pub async fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}