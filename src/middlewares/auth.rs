use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers.get("authorization")?.to_str().ok()
}

pub async fn auth(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = env::var("API_KEY").unwrap();

    match get_token(&headers) {
        Some(token) if token == api_key => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
