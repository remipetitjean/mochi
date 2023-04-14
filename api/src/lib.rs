use axum::http::{Method, Uri};
use axum::middleware;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

pub use self::error::{Error, Result};
use crate::ctx::Ctx;
use crate::logs::log_request;
use crate::model::ModelController;
use crate::mw_auth::{mw_ctx_resolver, mw_require_auth};

mod ctx;
pub mod error;
mod logs;
mod model;
mod mw_auth;
mod routes;


pub const AUTH_TOKEN: &str = "auth-token";


pub async fn main() -> Result<()> {
    // initialize microcontroller
    let mc = ModelController::new().await?;

    let auth_routes = routes::bot::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_require_auth));

    let routes = Router::new()
        .merge(routes::hello::routes())
        .merge(routes::login::routes())
        .nest("/bot", auth_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new());

    // start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    req_method: Method,
    uri: Uri,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // get the potential error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // if client error, build new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "req_uuid": uuid.to_string(),
                "error": {
                    "type": client_error.as_ref(),
                }
            });
            println!("  ->> client_error_body: {client_error_body}");

            // build new response
            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = match client_status_error {
        Some(t) => Some(t.1),
        _ => None,
    };
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
    println!();
    error_response.unwrap_or(res)
}
