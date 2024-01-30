use crate::Category;
use axum::{response::IntoResponse, Extension, Json};
use sqlx::PgPool;

pub async fn get_categories(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let rows: Vec<Category> = sqlx::query_as("SELECT id, name FROM categories ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}
