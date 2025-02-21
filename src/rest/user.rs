use crate::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/get/user/{pubkey}", get(load_user))
        .route("/api/login", post(login))
        .route("/api/register", post(register))
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub pubkey: String,
}

#[derive(serde::Serialize)]
pub struct LoginReponse {
    pub known: bool,
}

async fn login(
    State(shared_state): State<Arc<AppState>>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginReponse>, StatusCode> {
    let pubkey = input.pubkey.clone();

    match crate::features::user::exists(shared_state.pool.clone(), &pubkey).await {
        Ok(known) => Ok(Json(LoginReponse { known })),
        Err(e) => {
            tracing::error!(pubkey, ?e, "error creating user");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterInput {
    pub pubkey: String,
    pub nickname: String,
}

async fn register(
    State(shared_state): State<Arc<AppState>>,
    Json(input): Json<RegisterInput>,
) -> StatusCode {
    let pubkey = input.pubkey.clone();
    let nickname = input.nickname.clone();

    match crate::features::user::create(shared_state.pool.clone(), &pubkey, &nickname).await {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            tracing::error!(pubkey, ?e, "error creating user");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub nickname: String,
}

async fn load_user(
    State(shared_state): State<Arc<AppState>>,
    Path(pubkey): Path<String>,
) -> Result<Json<UserResponse>, StatusCode> {
    println!("pubkey: {}", pubkey);

    match crate::features::user::find(shared_state.pool.clone(), &pubkey).await {
        Ok(Some(nickname)) => Ok(Json(UserResponse { nickname })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!(pubkey, ?e, "error fetching user");
            Err(StatusCode::NOT_FOUND)
        }
    }
}
