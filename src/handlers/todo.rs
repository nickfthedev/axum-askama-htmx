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
pub async fn todo_add(pool: Arc<PgPool>, form: Form<TodoForm>) -> String {
    info!("Adding Todo");
    let description = &form.description;
    sqlx::query!("INSERT INTO todos (description, done) VALUES ($1, $2)", description, false)
        .execute(&*pool)
        .await;
   return "Hallo".to_string();
}