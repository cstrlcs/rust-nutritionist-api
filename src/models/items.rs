use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Item {
    id: i32,
    name: String,
    category: i32,
    kcal: f64,
    protein: f64,
    carbo: f64,
    lipids: f64,
    fibers: f64,
}
