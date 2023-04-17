use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete as delete_, get, post};
use axum::{Json, Router};
use model::user;
use thiserror::Error;

use crate::ctx::Ctx;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("cannot create user")]
    UserCreateError(#[from] user::UserCreateError),

    #[error("cannot list user")]
    UserListError(#[from] user::UserListError),

    #[error("cannot read user")]
    UserReadError(#[from] user::UserReadError),

    #[error("cannot delete user")]
    UserDeleteError(#[from] user::UserDeleteError),
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RESP");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}

#[derive(Clone, FromRef)]
struct AppState {
    user_controller: user::UserController,
}

pub fn routes(user_controller: user::UserController) -> Router {
    let app_state = AppState { user_controller }; // user_controller can be used in with_state, but here to introduce sub
                                                  // app state
    Router::new()
        .route("/:id", get(read))
        .route("/", post(create))
        .route("/:id", delete_(delete))
        .with_state(app_state)
}

async fn create(
    ctx: Ctx,
    State(user_controller): State<user::UserController>,
    Json(user_for_create): Json<user::UserForCreate>,
) -> Result<Json<user::User>, UserError> {
    println!("->> {:<12} - user::create", "HANDLER");
    let user = user_controller.create(user_for_create).await?;
    Ok(Json(user))
}

async fn read(
    ctx: Ctx,
    State(user_controller): State<user::UserController>,
    Path(id): Path<String>,
) -> Result<Json<user::User>, UserError> {
    println!("->> {:<12} - user::create", "HANDLER");
    let user = user_controller.read(id).await?;
    Ok(Json(user))
}

async fn delete(
    State(user_controller): State<user::UserController>,
    _ctx: Ctx,
    Path(id): Path<String>,
) -> Result<Json<user::User>, UserError> {
    println!("->> {:<12} - user::delete", "HANDLER");
    let user = user_controller.delete(id).await?;
    Ok(Json(user))
}
