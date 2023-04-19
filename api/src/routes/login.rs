use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{LoginError, Result, AUTH_TOKEN};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - login", "HANDLER");

    // TODO: implement real auth logic
    if payload.username != "remi" || payload.password != "password" {
        return Err(LoginError::LoginFail);
    }

    // add cookie
    // FIXME: implement real auth-token generation / signature
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // create the success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)
}
