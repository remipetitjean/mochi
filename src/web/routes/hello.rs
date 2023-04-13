use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello2/:name", get(hello2))
}

// `/hello` or `/hello?name=rem`
async fn hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello <strong>{name}</strong>"))
}

// `/hello` or `/hello?name=rem`
async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}
