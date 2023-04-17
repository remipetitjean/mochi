use axum::extract::{FromRef, Path, State};
use axum::routing::{delete as delete_, get, post};
use axum::{Json, Router};

use crate::ctx::Ctx;
use crate::model_old::{Bot, BotForCreate, ModelController};
use crate::Result;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc }; // mc can be used in with_state, but here to introduce sub
                                     // app state
    Router::new()
        .route("/", post(create))
        .route("/", get(list))
        .route("/:id", delete_(delete))
        .with_state(app_state)
}

async fn create(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(bot_for_create): Json<BotForCreate>,
) -> Result<Json<Bot>> {
    println!("->> {:<12} - bot::create", "HANDLER");
    let bot = mc.create(ctx, bot_for_create).await?;
    Ok(Json(bot))
}

async fn list(State(mc): State<ModelController>, ctx: Option<Ctx>) -> Result<Json<Vec<Bot>>> {
    println!("->> {:<12} - bot::list", "HANDLER");
    let bots = mc.list(ctx).await?;
    Ok(Json(bots))
}

async fn delete(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Bot>> {
    println!("->> {:<12} - bot::delete", "HANDLER");
    let bot = mc.delete(ctx, id).await?;
    Ok(Json(bot))
}
