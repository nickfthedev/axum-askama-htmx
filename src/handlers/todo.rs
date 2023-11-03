use anyhow::Ok;
use askama_axum::IntoResponse;
use sqlx::{PgPool, pool};
use tracing::info;
use std::sync::Arc;
use crate::templates::todo::TodoTemplate;

///
///  Renders main template
/// 
pub async fn todo_main() -> TodoTemplate<'static> {
    TodoTemplate { name: "master" }
}

use axum::extract::Form;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TodoForm {
    description: String,
}

///
/// Function for adding a task
/// 
pub async fn todo_add(pool: Arc<PgPool>) -> String {
    info!("Adding Todo");
    sqlx::query!("INSERT INTO todos (description, done) VALUES ($1, $2)", "Test", false)
        .execute(&*pool)
        .await;
   return "Hallo".to_string();
}