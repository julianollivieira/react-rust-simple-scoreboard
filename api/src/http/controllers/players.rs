use crate::http::requests::{CreatePlayerRequest, UpdatePlayerRequest};
use crate::http::{error::ApiError, ApiContext};
use crate::models::Player;
use axum::{
    extract::Path,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use uuid::Uuid;

// Create a router for the players endpoint.
pub fn router() -> Router {
    Router::new()
        .route("/players", get(get_players))
        .route("/players", post(create_player))
        .route("/players/:id", patch(update_player))
        .route("/players/:id", delete(delete_player))
}

// Handler for GET '/players', returns a list of all players.
#[axum_macros::debug_handler]
async fn get_players(ctx: Extension<ApiContext>) -> Result<Json<Vec<Player>>, ApiError> {
    let result = Player::get_all(&ctx.pool).await;
    match result {
        Ok(players) => Ok(Json(players)),
        Err(e) => Err(e.into()),
    }
}

// Handler for POST '/players', creates a new player and returns it.
#[axum_macros::debug_handler]
async fn create_player(
    ctx: Extension<ApiContext>,
    Json(req): Json<CreatePlayerRequest>,
) -> Result<Json<Player>, ApiError> {
    let result = Player::new(&ctx.pool, req.name, req.score).await;
    match result {
        Ok(player) => Ok(Json(player)),
        Err(e) => Err(e.into()),
    }
}

// Handler for PATCH '/players/:id', updates a player with the given id and returns it.
#[axum_macros::debug_handler]
async fn update_player(
    Path(id): Path<Uuid>,
    ctx: Extension<ApiContext>,
    Json(req): Json<UpdatePlayerRequest>,
) -> Result<Json<Player>, ApiError> {
    let result = Player::update(&ctx.pool, id, req.name, req.score).await;
    match result {
        Ok(player) => Ok(Json(player)),
        Err(e) => Err(e.into()),
    }
}

// Handler for DELETE '/players/:id', deletes a player with the given id.
#[axum_macros::debug_handler]
async fn delete_player(Path(id): Path<Uuid>, ctx: Extension<ApiContext>) -> Result<(), ApiError> {
    let result = Player::delete(&ctx.pool, id).await;
    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
