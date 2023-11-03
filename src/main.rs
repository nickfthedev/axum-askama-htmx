pub mod handlers;
pub mod router;
pub mod templates;

use anyhow::Context;
use router::app_router;

use sqlx::postgres::PgPool;
use std::env;

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber;
//
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // For logs
    tracing_subscriber::fmt::init();
    // Env
    dotenv::dotenv().ok();
    // DB Connect & Migrate
    let pool = PgPool::connect(&env::var("DATABASE_URL")?)
    .await?;
    sqlx::migrate!("./migrations/").run(&pool).await?;
    let shared_pool = Arc::new(pool);
    // Setting routes
    let app = app_router(shared_pool);

    // Setting port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);
    // Starting sever
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")

}
