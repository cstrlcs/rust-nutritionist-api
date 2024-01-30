use crate::Item;
use axum::{extract::Query, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Deserialize)]
pub struct GetItemsQueryParams {
    category_id: Option<i32>,
    search: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
}

pub async fn get_items(
    Extension(pool): Extension<PgPool>,
    params: Query<GetItemsQueryParams>,
) -> impl IntoResponse {
    let params = params.0;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(100);
    let offset = (page - 1) * per_page;

    let mut query = QueryBuilder::<Postgres>::new(
        "SELECT id, name, category, kcal, protein, carbo, lipids, fibers FROM items",
    );

    if let Some(category_id) = params.category_id {
        query.push(" WHERE category = ").push_bind(category_id);
    } else if let Some(search) = params.search {
        query
            .push(" WHERE LOWER(name) LIKE ")
            .push_bind(format!("%{}%", search.to_lowercase()));
    }

    query.push(" LIMIT ").push_bind(per_page);
    query.push(" OFFSET ").push_bind(offset);

    let rows: Vec<Item> = query.build_query_as().fetch_all(&pool).await.unwrap();
    Json(rows)
}
