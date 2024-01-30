use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Category {
    id: i32,
    name: String,
}
