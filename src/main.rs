mod middlewares;
mod models;
mod services;

use axum::{middleware, routing::get, serve, Extension, Router};
use dotenv::dotenv;
use middlewares::auth::auth;
use models::{categories::Category, items::Item};
use services::{categories::get_categories, items::get_items};
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/categories", get(get_categories))
        .route("/items", get(get_items))
        .layer(middleware::from_fn(auth))
        .layer(Extension(pool));

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
